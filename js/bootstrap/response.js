import BodyMixin from 'internal:body_mixin.js';
import { StringReadable, isBufferish } from 'internal:common.js';
import Headers from 'internal:headers.js';

export default class Response {
  #rawHeaders; // not yet instantiated
  #headers; // instantiated
  #status;
  #statusText;
  #body;
  #_bodyString;
  constructor(body, init = {}) {
    this.#status = init.status || 200;
    this.#statusText = init.statusText || 'OK';
    if (!(init.headers instanceof Headers)) {
      this.#rawHeaders = init.headers;
    } else {
      this.#headers = init.headers;
    }
    if (body instanceof ReadableStream || body instanceof TransformStream) {
      this.#body = body;
    } else if (typeof body === 'string') {
      this.#_bodyString = body;
    } else if (isBufferish(body)) {
      this.#body = new StringReadable(body);
    }
  }

  get headers() {
    if (!this.#headers) {
      this.#headers = new Headers(this.#rawHeaders);
    }
    return this.#headers;
  }

  get status() {
    return this.#status;
  }

  get statusText() {
    return this.#statusText;
  }

  get body() {
    return this.#body;
  }

  get _bodyString() {
    return this.#_bodyString;
  }

}
BodyMixin.mixin(Response);
