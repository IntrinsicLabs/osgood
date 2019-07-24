import { generateContextObject } from 'internal:context.js';
import { isBufferish } from 'internal:common.js';
import Headers from 'internal:headers.js';
import Response from 'internal:response.js';
import Request from 'internal:request.js';

const {
  sendError,
  startResponse,
  writeResponse,
  stringResponse,
  setFetchHandler,
  setIncomingReqHeadHandler,
  setIncomingReqBodyHandler,
} = self._bindings;

const inFlightInbounds = {};

const writerMap = new WeakMap();

// This function checks to see if the object should serialize into a POJO
// Object, one that is free of class instances. "Double getters" do exist.
// For example, it could first reply wth a string, and later reply a class
// instance. Keep in mind this check is done to prevent a foot gun, not for
// security purposes. If it were for security we'd construct a shadow object
// and copy properties. Double Getter's are explained here:
// https://medium.com/intrinsic/protecting-your-javascript-apis-9ce5b8a0e3b5
function shouldSerializeIntoPOJO(obj) {
  if (obj === null) {
    return true;
  } else if (typeof obj !== 'object') {
    return true;
  }

  if (obj.toJSON) {
    obj = obj.toJSON();
  }

  if (obj === null) {
    return true;
  } else if (typeof obj !== 'object') {
    return true;
  }

  const proto = Object.getPrototypeOf(obj);

  if (proto === Array.prototype) {
    for (let value of obj) {
      if (!shouldSerializeIntoPOJO(value)) {
        return false;
      }
    }
    return true;
  } else if (proto !== Object.prototype && proto !== null) {
    return false;
  } else {
    // intentionally ignore Symbol properties as they're ignored by JSON.stringify
    for (let value of Object.values(obj)) {
      if (!shouldSerializeIntoPOJO(value)) {
        return false;
      }
    }
    return true;
  }
}

function inboundErrorHandler(fn) {
  return async (reqId, ...args) => {
    try {
      await fn(reqId, ...args);
    } catch (e) {
      console.error(e.stack);
      sendError(500, '', reqId);
      delete inFlightInbounds[reqId];
    }
  };
}

async function handleIncomingReqHead(reqId, fn, method, url, headers) {
  let writer;
  const body = new ReadableStream({
    start(controller) {
      writer = controller;
    }
  });
  const request = new Request(url, {
    method,
    headers,
    body
  });
  writerMap.set(request, writer);
  inFlightInbounds[reqId] = request;
  let response;
  if (typeof fn !== 'function') {
    throw new TypeError('Worker did not provide a valid handler');
  }
  response = fn(request, generateContextObject(url));
  if (typeof response === 'object' && response !== null && typeof response.then === 'function') {
    response = await response;
  }

  switch (typeof response) {
    case 'string': {
      // handle it in native code
      break;
    }
    case 'object': {
      if (response === null) {
        throw new TypeError('Response was an invalid object');
      }
      if (response instanceof Response) {
        // we're good!
      } else if (isBufferish(response)) {
        response = new Response(response, {
          headers: new Headers({
            'Content-Type': 'application/octet-stream'
          })
        });
      } else {
        if (shouldSerializeIntoPOJO(response)) {
          const body = JSON.stringify(response);
          response = new Response(body, {
            headers: new Headers({
              'Content-Type': 'application/json'
            })
          });
        } else {
          throw new TypeError('Response object must be a POJO');
        }
      }
      break;
    }
    default:
      throw new TypeError(`Invalid response type "${typeof response}"`);
  }

  if (typeof response === 'string') {
    stringResponse(response, reqId);
  } else if (response.body) {
    startResponse(response, reqId);
    let stream =
      response.body instanceof TransformStream
      ? response.body.readable
      : response.body;
    for await (let chunk of stream) {
      writeResponse(chunkAsArrayBuffer(chunk), reqId);
    }
    writeResponse(null, reqId);
  } else {
    startResponse(response, reqId, response._bodyString);
  }

  delete inFlightInbounds[reqId];
}
setIncomingReqHeadHandler(inboundErrorHandler(handleIncomingReqHead));

async function handleIncomingReqBody(reqId, body) {
  let writer = writerMap.get(inFlightInbounds[reqId]);
  if (!writer) {
    // Response has already been sent, so we don't care about any
    // more incoming data.
    return;
  }
  if (typeof body === 'undefined') {
    await writer.close();
  } else {
    await writer.enqueue(body);
  }
}
setIncomingReqBodyHandler(inboundErrorHandler(handleIncomingReqBody));

function chunkAsArrayBuffer(chunk) {
  if (!(chunk instanceof ArrayBuffer)) {
    if (typeof chunk === 'string') {
      const enc = new TextEncoder();
      chunk = enc.encode(chunk).buffer;
      return chunk;
    }
    if (typeof chunk === 'object') {
      if (chunk.buffer && chunk.buffer instanceof ArrayBuffer) {
        chunk = chunk.buffer;
      } else {
        throw new TypeError(
          'body chunks must be strings, ArrayBuffers, TypedArrays or DataViews'
        );
      }
    }
  }
  return chunk;
}
