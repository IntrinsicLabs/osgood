import BodyMixin from 'internal:body_mixin.js';
import { StringReadable, isBufferish } from 'internal:common.js';
import Headers from 'internal:headers.js';
import FormData from 'internal:form_data.js';

const { getPrivate } = _bindings;

const rawHeadersSym = getPrivate('rawHeaders');
const headersSym = getPrivate('headers');
const urlSym = getPrivate('url');
const methodSym = getPrivate('method');
const bodySym = getPrivate('body');
const _bodyStringSym = getPrivate('_bodyString');

export default class Request {
  // #rawHeaders; // not yet instantiated
  // #headers; // instantiated
  // #url;
  // #method;
  // #body;
  // #_bodyString;
  constructor(input, init = {}) {
    // TODO support `input` being a Request
    this[urlSym] = input;

    if (!(init.headers instanceof Headers)) {
      this[rawHeadersSym] = init.headers;
    } else {
      this[headersSym] = init.headers;
    }
    this[methodSym] = init.method || 'GET';

    if (init.body instanceof ReadableStream || init.body instanceof FormData) {
      this[bodySym] = init.body;
    } else if (typeof init.body === 'string') {
      this[_bodyStringSym] = init.body;
    } else if (isBufferish(init.body)) {
      this[bodySym] = new StringReadable(init.body);
    }
  }

  get headers() {
    if (!this[headersSym]) {
      this[headersSym] = new Headers(this[rawHeadersSym]);
    }
    return this[headersSym];
  }

  get url() {
    return this[urlSym];
  }

  get method() {
    return this[methodSym];
  }

  get body() {
    return this[bodySym];
  }

  get _bodyString() {
    return this[_bodyStringSym];
  }
}
BodyMixin.mixin(Request);
