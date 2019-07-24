'use strict';
const assert = require('assert');
const {
  test,
  request,
  assertFilterEqual
} = require('./../../common.js');

const PORT = 3000;

test(async function hello() {
  const [res, body] = await request(PORT, '/hello');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'text/plain');
  assert.strictEqual(res.headers['content-length'], '14');
  assert.strictEqual('Hello, world!\n', body.toString());
});

test(async function stringStreamResp() {
  const [res, body] = await request(PORT, '/string-stream-resp');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'text/plain');
  assert.strictEqual(res.headers['transfer-encoding'], 'chunked');
  assert.strictEqual('Hello, world!\n', body.toString());
});

test(async function poststream() {
  const [res, body] = await request(PORT, '/poststream', { method: 'POST'}, 'foo1=bar1&such=stream');
  assert.strictEqual(res.statusCode, 200);
  assert.ok(res.headers['content-type'].startsWith('application/json'));
  assert.strictEqual(res.headers['transfer-encoding'], 'chunked');
  assert.deepStrictEqual({"foo1":"bar1","such":"stream"}, JSON.parse(body.toString()).fields);
});

test(async function intrinsic() {
  const [res, body] = await request(PORT, '/intrinsic');
  assert.strictEqual(res.statusCode, 200);
  const [iRes, iBody] = await request(PORT, 'https://intrinsic.com/');
  assertFilterEqual(['date', 'connection', 'expires'], res.headers, iRes.headers);
  assert.deepStrictEqual(body, iBody);
});

test(async function image() {
  const [res, body] = await request(PORT, '/image');
  assert.strictEqual(res.statusCode, 200);
  const [iRes, iBody] = await request(PORT, 'http://localhost:9001/image.png');
  assertFilterEqual(['date', 'connection', 'expires'], res.headers, iRes.headers);
  assert.deepStrictEqual(body, iBody);
});

test(async function hostHeaderFoolery() {
  const [res, body] = await request(PORT, '/host-header-foolery');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'text/plain');
  assert.strictEqual(body.toString(), 'all good');
});

test(async function returnArray() {
  const [res, body] = await request(PORT, '/return-array');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'application/json');
  assert.strictEqual(body.toString(), '[1,2,3]');
});

test(async function relative() {
  const [res, body] = await request(PORT, '/relative');
  assert.strictEqual(res.statusCode, 500);
});

test(async function imports() {
  const [res, body] = await request(PORT, '/imports');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'application/json');
  assert.deepStrictEqual(JSON.parse(body.toString()), {
    "imports1": { "imports1": 123, "foo": 8765 },
    "imports2":{ "imports1": { "imports1": 123, "foo": 8765}, "imports2": 456 }
  });
});

test(async function urlParams() {
  const randomStr = Math.random().toString(36).substring(2, 10);
  const [res, body] = await request(PORT, `/test/${randomStr}/what`);
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(body.toString(), `{"hello":"${randomStr}"}`);
});

test(async function returnClassInstance() {
  const [res, body] = await request(PORT, '/return-class-instance');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'application/json');
  assert.strictEqual(body.toString(), '{"username":"osgood"}');
});

test(async function complexGood() {
  const [res, body] = await request(PORT, '/complex-good');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'application/json');
  assert.strictEqual(body.toString(), '{"foo":[{"bar":[{"username":"foo"}]}]}');
});

test(async function noreply() {
  try {
    const [res, body] = await request(PORT, '/noreply', { timeout: 100 });
  } catch (e) {
    assert.strictEqual(e.toString(), 'Error: request timed out');
    return;
  }
  throw new Error('should have timed out');
});

test(async function echoHeaders() {
  const [res, body] = await request(PORT, '/echo-headers', {
    headers: { foo: 'bar' }
  });
  const headers = JSON.parse(body);
  assert.ok(headers.find(header => header[0] === 'foo' && header[1] === 'bar'));
});

test(async function applicationFormurlencode() {
  const [res, body] = await request(PORT, '/urlencode');
  assert.strictEqual(res.statusCode, 200);
  assert.ok(res.headers['content-type'].startsWith('application/json'));
  assert.strictEqual(res.headers['content-length'], '207');
  const json = JSON.parse(body.toString());
  assert.deepStrictEqual({"foo1":"bar1","foo2":["bar2", "bar3"]}, json.fields);
  assert.deepStrictEqual('application/x-www-form-urlencoded', json.headers['content-type']);
});

test(async function multipartFormData() {
  const [res, body] = await request(PORT, '/multipart');
  assert.strictEqual(res.statusCode, 200);
  const json = JSON.parse(body.toString());
  assert.ok(json.ok);
  assert.deepEqual(json.fields, {
    from: 'Intrinsic <hello@intrinsic.com>',
    to: 'spam@intrinsic.com',
    subject: 'Osgood Hello World',
    text: 'Hello from Osgood!'
  });
  assert.deepEqual(json.files, {});
  assert.strictEqual(json.headers.authorization, 'Basic YXBpOmtleS0wMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMA==');
  assert.ok(json.headers['content-type'].startsWith('multipart/form-data; boundary='));
  assert.ok(json.headers['user-agent'].startsWith('osgood/'));
  assert.equal(json.headers['content-length'], 509);
});

test(async function httpPolicies() {
  const [res, body] = await request(PORT, '/http-policies');
  assert.strictEqual(res.statusCode, 200);
});

test(async function fetch_stringUrl() {
  const [res, body] = await request(PORT, '/fetches/stringUrl');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(body.toString(), 'ok');
});

test(async function fetch_bodyString() {
  const [res, body] = await request(PORT, '/fetches/bodyString');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(body.toString(), 'this is a bodyString test');
});

test(async function fetch_stream() {
  const [res, body] = await request(PORT, '/fetches/stream');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(body.toString(), 'this is a stream test');
});

test(async function response_string() {
  const [res, body] = await request(PORT, '/responses/string');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(body.toString(), 'this is a string test');
});

test(async function response_typedArray() {
  const [res, body] = await request(PORT, '/responses/typedArray');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(body.toString(), 'this is a typedarray test');
});

test(async function response_arrayBuffer() {
  const [res, body] = await request(PORT, '/responses/arrayBuffer');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(body.toString(), 'this is an arraybuffer test');
});

test(async function response_dataView() {
  const [res, body] = await request(PORT, '/responses/dataView');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(body.toString(), 'this is a dataview test');
});

test(async function response_objectHeader() {
  const [res] = await request(PORT, '/responses/objectHeader');
  assert.strictEqual(res.statusCode, 500);
  assert.strictEqual(res.headers.testheader, 'test header value');
});

test(async function response_classHeader() {
  const [res] = await request(PORT, '/responses/classHeader');
  assert.strictEqual(res.statusCode, 500);
  assert.strictEqual(res.headers.testheader, 'test header value');
});
