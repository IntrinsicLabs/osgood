import { unimplemented, IteratorMixin } from 'internal:common.js';

export default class Headers {
  constructor(init = {}) {
    // TODO This really should be private and not exposed to user code. It's
    // exposed for now to more easily pass it into native code. In the future,
    // we can just call `entries()` to get the underlying headers.
    this._headers = {};
    if (init instanceof Headers) {
      for (const [name, value] of Headers.prototype.keys.apply(init)) {
        this.append(name, value);
      }
    } else if (typeof init === 'object') {
      if (Symbol.iterator in init) {
        for (const header of init) {
          if (typeof header !== 'object' || !(Symbol.iterator in header)) {
            throw new TypeError('Invalid headers');
          }
          let [name, value, ...extras] = [...header];
          if (extras.length !== 0) {
            throw new TypeError('Invalid headers');
          }
          this.append(name, value);
        }
      } else {
        for (const name in init) {
          if (Object.prototype.hasOwnProperty.call(init, name)) {
            this.append(name, init[name]);
          }
        }
      }
    } else {
      throw new TypeError('Invalid headers');
    }
  }

  set(name, value) {
    name = normalizeHeaderName(name);
    value = normalizeHeaderValue(value);
    this._headers[name] = String(value);
  }

  append(name, value) {
    name = normalizeHeaderName(name);
    value = normalizeHeaderValue(value);
    if (name in this._headers) {
      this._headers[name] += ', ' + value;
    } else {
      this._headers[name] = value;
    }
  }

  get(name) {
    name = normalizeHeaderName(name);
    return this._headers[name];
  }

  has(name) {
    name = normalizeHeaderName(name);
    return name in this._headers;
  }

  delete(name) {
    name = normalizeHeaderName(name);
    delete this._headers[name];
  }

  *[Symbol.iterator]() {
    yield* Object.entries(this._headers);
  }
}
IteratorMixin.mixin(Headers);

// https://tools.ietf.org/html/rfc7230#section-3.2.6
const headerNameRe = /^[\^_`a-zA-Z\-0-9!#$%&'*+.|~]+$/;
function normalizeHeaderName(name) {
  if (!name) {
    throw new TypeError('Invalid header name');
  }
  name = String(name).toLowerCase();
  if (!headerNameRe.test(name)) {
    throw new TypeError('Invalid header name: ' + name);
  }
  return name;
}

// https://tools.ietf.org/html/rfc7230#section-3.2
const invalidHeaderValueRe = /[^\t\x20-\x7e\x80-\xff]/;
function normalizeHeaderValue(value) {
  if (value === undefined) {
    throw new TypeError('Invalid header value');
  }
  value = String(value);
  if (invalidHeaderValueRe.test(value)) {
    throw new TypeError('Invalid header value: ' + value);
  }
  return value;
}
