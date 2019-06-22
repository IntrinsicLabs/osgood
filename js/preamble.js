{
  const window = {};
  self.window = window;
  self.gc = () => {};
  require('./vendor/streams/reference-implementation/lib');
  require('fast-text-encoding/text.js');
  const { URL, URLSearchParams } = require('whatwg-url');
  self.ReadableStream = window.ReadableStream;
  self.WritableStream = window.WritableStream;
  self.TransformStream = window.TransformStream;
  self.TextEncoder = window.TextEncoder;
  self.TextDecoder = window.TextDecoder;
  self.URL = URL;
  self.URLSearchParams = URLSearchParams;
  delete self.window;
  delete self.gc;
}
