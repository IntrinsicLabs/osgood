import BodyMixin from 'internal:body_mixin.js';
import { StringReadable, isBufferish } from 'internal:common.js';
import Headers from 'internal:headers.js';

const { getPrivate } = _bindings;

const rawHeadersSym = getPrivate('rawHeaders');
const headersSym = getPrivate('headers');
const statusSym = getPrivate('status');
const statusTextSym = getPrivate('statusText');
const bodySym = getPrivate('body');
const _bodyStringSym = getPrivate('_bodyString');

export default class Response {
  // #rawHeaders; // not yet instantiated
  // #headers; // instantiated
  // #status;
  // #statusText;
  // #body;
  // #_bodyString;
  constructor(body, init = {}) {
    this[statusSym] = init.status || 200;
    this[statusTextSym] = init.statusText || 'OK';
    if (!(init.headers instanceof Headers)) {
      this[rawHeadersSym] = init.headers;
    } else {
      this[headersSym] = init.headers;
    }
    if (body instanceof ReadableStream || body instanceof TransformStream) {
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

  get status() {
    return this[statusSym];
  }

  get statusText() {
    return this[statusTextSym];
  }

  get body() {
    return this[bodySym];
  }

  get _bodyString() {
    return this[_bodyStringSym];
  }

}
BodyMixin.mixin(Response);
