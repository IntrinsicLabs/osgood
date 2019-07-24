import FormData from 'internal:form_data.js';
import Response from 'internal:response.js';
import Request from 'internal:request.js';

const {
  setFetchHandler,
  _fetch
} = self._bindings;

const fetchCbs = {};
function handleFetch(err, body, meta, fetchId) {
  fetchCbs[fetchId](err, body, meta);
}
setFetchHandler(handleFetch);

function parseUrl(url) {
  const urlObj = new URL(url);
  const newObj = {};
  for (const key of ['protocol', 'hostname', 'port', 'pathname']) {
    newObj[key] = urlObj[key];
  }
  newObj.port = newObj.port || '80';
  if (urlObj.search) {
    newObj.pathname = newObj.pathname + urlObj.search;
  }
  return newObj;
}

// https://tools.ietf.org/html/rfc1867
// https://www.w3.org/Protocols/rfc1341/7_2_Multipart.html#z0
function generateMultipartFormData(formData) {
  const boundary = `--------------OsgoodFormBoundary${Math.floor(Math.random() * 899999999) + 100000000}`;

  let body = '';

  for (let entry of formData) {
    if (entry[2]) {
      throw new TypeError("Osgood currently doesn't support files");
    }
    body += `--${boundary}\r\n`;
    body += `Content-Disposition: form-data; name="${entry[0]}"\r\n`;
    body += `\r\n`;
    body += `${entry[1]}\r\n`;
  }

  body += `--${boundary}--\r\n`;

  return {
    body,
    boundary,
    contentType: `multipart/form-data; boundary=${boundary}`
  };
}

let increasingFetchId = 0;
export default async function fetch(input, init) {
  const fetchId = ++increasingFetchId;
  const p = new Promise((resolve, reject) => {
    let writer = null;
    fetchCbs[fetchId] = (err, data, meta) => {
      if (err) {
        err = new Error(err);
        reject(err);
        console.error('rejected fetch call due to: ' + err);
        return;
      }
      if (meta) {
        const readable = new ReadableStream({
          start(controller) {
            writer = controller;
          }
        });
        resolve(new Response(readable, meta));
      } else if (data === null) {
        writer.close();
        delete fetchCbs[fetchId];
      } else {
        if (data instanceof ArrayBuffer) {
          data = new Uint8Array(data);
        }
        writer.enqueue(data);
      }
    };
  });

  if (typeof input === 'string') {
    input = new Request(input, init);
  }
  const parsedUrl = parseUrl(input.url);

  if (parsedUrl.protocol !== 'http:' && parsedUrl.protocol !== 'https:') {
    throw new TypeError(`Unsupported protocol: "${parsedUrl.protocol}"`);
  }

  if (typeof input._bodyString === 'string') {
    _fetch(
      parsedUrl,
      input.url,
      input.headers,
      input.method.toUpperCase(),
      input._bodyString,
      fetchId,
      'string'
    );
  } else if (input.body instanceof FormData) {
    const { contentType, body } = generateMultipartFormData(input.body);

    input.headers.set('Content-Type', contentType);

    _fetch(
      parsedUrl,
      input.url,
      input.headers,
      input.method.toUpperCase(),
      body,
      fetchId,
      'string'
    );
  } else if (typeof input.body === 'object') {
    _fetch(
      parsedUrl,
      input.url,
      input.headers,
      input.method.toUpperCase(),
      input._bodyString,
      fetchId,
      'stream'
    );
    for await (const chunk of input.body) {
      _fetch(null, null, null, null, chunk, fetchId, 'stream');
    }
    _fetch(null, null, null, null, false, fetchId, 'stream');
  } else {
    _fetch(
      parsedUrl,
      input.url,
      input.headers,
      input.method.toUpperCase(),
      null,
      fetchId,
      'none'
    );
  }

  return p;
}
