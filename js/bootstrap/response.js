import { BodyMixin } from 'internal:body_mixin.js';

const { getPrivate } = _bindings;

const statusSym = getPrivate('status');
const statusTextSym = getPrivate('statusText');

const bodyInit = BodyMixin.init;

export default class Response {
  // #status;
  // #statusText;
  constructor(body, init = {}) {
    this[statusSym] = init.status || 200;
    this[statusTextSym] = init.statusText || 'OK';
    bodyInit.call(this, body, init);
  }

  get status() {
    return this[statusSym];
  }

  get statusText() {
    return this[statusTextSym];
  }
}
BodyMixin.mixin(Response);
