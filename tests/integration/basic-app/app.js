#!/usr/bin/env osgood

app.port = 3000;

// Testing correct behavior
app.get('/hello', 'hello.js');
app.route('GET', '/return-array', 'return-array.js');
app.get('/relative', 'relative.js');
app.get('/imports', 'imports.js');
app.get('/test/:hello/what', 'url-params.js');
app.get('/return-class-instance', 'return-class-instance.js');
app.get('/complex-good', 'complex-good.js');
app.get('/noreply', 'noreply.js');
app.get('/echo-headers', 'echo-headers.js');
app.get('/string-stream-resp', 'string-stream-resp.js');
app.get('/responses/:type', 'responses.js');

// Routes which talk to external services
app.route('GET', '/urlencode', 'urlencode.js', policy => {
  policy.outboundHttp.allowPost(`http://localhost:9001/form-echo`);
});
app.route('GET', '/multipart', 'multipart.js', policy => {
  policy.outboundHttp.allowPost(`http://localhost:9001/form-echo`);
});
app.route('POST', '/poststream', 'poststream.js', policy => {
  policy.outboundHttp.allowPost(`http://localhost:9001/form-echo`);
});
app.get('/image', 'image.js', policy => {
  policy.outboundHttp.allowGet(`http://localhost:9001/image.png`);
});
app.get('/host-header-foolery', 'host-header-foolery.js', policy => {
  policy.outboundHttp.allowGet(`http://localhost:9001/`);
});
app.route('GET', '/intrinsic', 'intrinsic.js', policy => {
  policy.outboundHttp.allowGet('https://intrinsic.com');
});
app.get('/http-policies', 'http-policies.js', policy => {
  policy.outboundHttp.allowGet('http://localhost:9001/');
});
app.get('/fetches/:type', 'fetches.js', policy => {
  policy.outboundHttp.allowGet('http://localhost:9001/');
  policy.outboundHttp.allowPost('http://localhost:9001/echo');
});

// Testing incorrect/problematic behavior
app.get('/evil', 'evil.js');
app.get('/badstart', 'badstart.js');
app.get('/nohandler', 'nohandler.js');
app.get('/badhandler', 'badhandler.js');
app.get('/syntaxerror', 'syntaxerror.js');
app.get('/complex-bad', 'complex-bad.js');
app.get('/connection-refused', 'connection-refused.js', policy => {
  policy.outboundHttp.allowGet('http://localhost:12345');
});
app.get('/bad-protocol', 'bad-protocol.js', policy => {
  policy.outboundHttp.allowGet('https://localhost:3000');
});
