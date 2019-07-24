import { isBufferish } from 'internal:common.js';

export default class BodyMixin {
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
    for (const key of Reflect.ownKeys(BodyMixin.prototype)) {
      if (key === 'constructor') {
        continue;
      }
      klass.prototype[key] = BodyMixin.prototype[key];
    }
  }
}
