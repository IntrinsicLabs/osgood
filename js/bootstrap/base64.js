// adapted from: https://stackoverflow.com/a/23190164
const tableStr =
  'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
const table = tableStr.split('');

// TODO: We should consider throwing a InvalidCharacterError/DOMException
// This would require creating a global.DOMException property for `instanceof`
// https://html.spec.whatwg.org/multipage/webappapis.html#atob
export function atob(base64) {
  if (/(=[^=]+|={3,})$/.test(base64)) {
    throw new TypeError('String contains an invalid character');
  }

  base64 = base64.replace(/=/g, '');

  const n = base64.length & 3;

  if (n === 1) {
    throw new Error('String contains an invalid character');
  }

  for (var i = 0, j = 0, len = base64.length / 4, bin = []; i < len; ++i) {
    const a = tableStr.indexOf(base64[j++] || 'A');
    const b = tableStr.indexOf(base64[j++] || 'A');
    const c = tableStr.indexOf(base64[j++] || 'A');
    const d = tableStr.indexOf(base64[j++] || 'A');

    if ((a | b | c | d) < 0) {
      throw new TypeError('String contains an invalid character');
    }

    bin[bin.length] = ((a << 2) | (b >> 4)) & 255;
    bin[bin.length] = ((b << 4) | (c >> 2)) & 255;
    bin[bin.length] = ((c << 6) | d) & 255;
  }

  return String.fromCharCode.apply(null, bin).substr(0, bin.length + n - 4);
}

export function btoa(bin) {
  const base64 = [];
  for (let i = 0, j = 0, len = bin.length / 3; i < len; ++i) {
    const a = bin.charCodeAt(j++),
      b = bin.charCodeAt(j++),
      c = bin.charCodeAt(j++);
    if ((a | b | c) > 255) {
      throw new TypeError('String contains an invalid character');
    }

    base64[base64.length] =
      table[a >> 2] +
      table[((a << 4) & 63) | (b >> 4)] +
      (isNaN(b) ? '=' : table[((b << 2) & 63) | (c >> 6)]) +
      (isNaN(b + c) ? '=' : table[c & 63]);
  }

  return base64.join('');
}
