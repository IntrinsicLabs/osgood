import { StringReadable, isBufferish } from 'internal:common.js';
import Headers from 'internal:headers.js';

const { getPrivate } = _bindings;

const rawHeadersSym = getPrivate('rawHeaders');
const headersSym = getPrivate('headers');
const bodySym = getPrivate('body');
const _bodyStringSym = getPrivate('_bodyString');
const chunksSym = getPrivate('chunks');
export const writeChunkSym = getPrivate('writeChunk');
const writerSym = getPrivate('writer');

export class BodyMixin {
  // #rawHeaders; // not yet instantiated
  // #headers; // instantiated
  // #body;
  // #_bodyString;

  static init(body, initObj = {}) {
    if (!(initObj.headers instanceof Headers)) {
      this[rawHeadersSym] = initObj.headers;
    } else {
      this[headersSym] = initObj.headers;
    }

    if (body === writeChunkSym) {
      this[chunksSym] = [];
    } else if (body instanceof ReadableStream || body instanceof TransformStream || body instanceof FormData) {
      this[bodySym] = body;
    } else if (typeof body === 'string') {
      this[_bodyStringSym] = body;
    } else if (isBufferish(body)) {
      this[bodySym] = new StringReadable(body);
    }
  }

  get headers() {
    if (!this[headersSym]) {
      this[headersSym] = new Headers(this[rawHeadersSym]);
    }
    return this[headersSym];
  }

  get body() {
    if (!this[bodySym] && this[chunksSym]) {
      let writer;
      const stream = new ReadableStream({
        start(controller) {
          writer = controller;
        }
      });
      this[writerSym] = writer;
      for (const chunk of this[chunksSym]) {
        if (typeof chunk === 'undefined') {
          writer.close();
        } else {
          writer.enqueue(chunk);
        }
      }
      delete this[chunksSym];
      this[bodySym] = stream;
    }
    return this[bodySym];
  }

  get _bodyString() {
    return this[_bodyStringSym];
  }

  async arrayBuffer() {
    let bufs = [];
    const lengths = [];
    let totalLength = 0;
    for await (let buf of this.body) {
      if (typeof buf === 'string') {
        const encoder = new TextEncoder();
        buf = encoder.encode(buf).buffer;
      }
      bufs.push(buf);
      const len = buf.byteLength;
      lengths.push(len);
      totalLength += len;
    }
    const result = new Uint8Array(totalLength);
    let idx = 0;
    for (const [i, buf] of Object.entries(bufs)) {
      result.set(new Uint8Array(buf), idx);
      idx += lengths[i];
    }
    return result.buffer;
  }

  async text() {
    let result = '';
    for await (let chunk of this.body) {
      if (typeof chunk === 'object' && chunk !== null && isBufferish(chunk)) {
        const decoder = new TextDecoder();
        chunk = decoder.decode(chunk);
      }
      if (typeof chunk === 'string') {
        result += chunk;
      } else {
        result += String(chunk);
      }
    }
    return result;
  }

  async json() {
    return JSON.parse(await this.text());
  }

  static mixin(klass) {
    const descs = Object.getOwnPropertyDescriptors(BodyMixin.prototype);
    for (let [key, desc] of Object.entries(descs)) {
      if (key === 'constructor') {
        continue;
      }
      Object.defineProperty(klass.prototype, key, desc);
    }
  }
}


export function writeChunk(chunk) {
  if (this[chunksSym]) {
    this[chunksSym].push(chunk);
  } else {
    const writer = this[writerSym];
    if (typeof chunk === 'undefined') {
      writer.close();
    } else {
      writer.enqueue(chunk);
    }
  }
}
