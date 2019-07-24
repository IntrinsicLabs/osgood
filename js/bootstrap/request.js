import BodyMixin from 'internal:body_mixin.js';
import { StringReadable, isBufferish } from 'internal:common.js';
import Headers from 'internal:headers.js';
import FormData from 'internal:form_data.js';

export default class Request {
  #rawHeaders; // not yet instantiated
  #headers; // instantiated
  #url;
  #method;
  #body;
  #_bodyString;
  constructor(input, init = {}) {
    // TODO support `input` being a Request
    this.#url = input;

    if (!(init.headers instanceof Headers)) {
      this.#rawHeaders = init.headers;
    } else {
      this.#headers = init.headers;
    }
    this.#method = init.method || 'GET';

    if (init.body instanceof ReadableStream || init.body instanceof FormData) {
      this.#body = init.body;
    } else if (typeof init.body === 'string') {
      this.#_bodyString = init.body;
    } else if (isBufferish(init.body)) {
      this.#body = new StringReadable(init.body);
    }
  }

  get headers() {
    if (!this.#headers) {
      this.#headers = new Headers(this.#rawHeaders);
    }
    return this.#headers;
  }

  get url() {
    return this.#url;
  }

  get method() {
    return this.#method;
  }

  get body() {
    return this.#body;
  }

  get _bodyString() {
    return this.#_bodyString;
  }
}
BodyMixin.mixin(Request);
