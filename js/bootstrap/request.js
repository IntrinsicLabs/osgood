import BodyMixin from 'internal:body_mixin.js';

const { getPrivate } = _bindings;

const urlSym = getPrivate('url');
const methodSym = getPrivate('method');


const bodyInit = BodyMixin.init;

export default class Request {
  // #url;
  // #method;
  constructor(input, init = {}) {
    // TODO support `input` being a Request
    this[urlSym] = input;
    this[methodSym] = init.method || 'GET';
    bodyInit.call(this, init.body, init);
  }

  get url() {
    return this[urlSym];
  }

  get method() {
    return this[methodSym];
  }
}
BodyMixin.mixin(Request);
