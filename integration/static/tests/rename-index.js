'use strict';
const assert = require('assert');
const {
  test,
  request,
  assertFilterEqual
} = require('./../../common.js');

const PORT = 3002;

test(async function root() {
  const [res, body] = await request(PORT, '/');
  assert.strictEqual(res.statusCode, 404);
});

test(async function rootWithOutTrailingSlash() {
  const [res, body] = await request(PORT, '');
  assert.strictEqual(res.statusCode, 404);
});

test(async function rootIndexHtml() {
  const [res, body] = await request(PORT, '/index.html');
  assert.strictEqual(res.headers['content-type'], 'text/html');
  assert.strictEqual(res.headers['content-length'], '57');
  assert.ok(body.toString().includes("This is index.html"));
});

test(async function thingsHtml() {
  const [res, body] = await request(PORT, '/things');
  assert.strictEqual(res.statusCode, 404);
});

// TODO: Is this correct behavior? When hyper_staticfile tries to read a file
// and it is a directory, then it returns a 301 to the directory.
test(async function redirectToTrailingSlash() {
  const [res, body] = await request(PORT, '/veggies');
  assert.strictEqual(res.statusCode, 301);
});

test(async function defaultWithinAFolder() {
  const [res, body] = await request(PORT, '/fruits/');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'text/html');
  assert.strictEqual(res.headers['content-length'], '66');
  assert.ok(body.toString().includes("This is fruits/default.html"));
});

test(async function cleanUrlFileWithHtmlExt() {
  const [res, body] = await request(PORT, '/fruits.html');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'text/html');
  assert.strictEqual(res.headers['content-length'], '58');
  assert.ok(body.toString().includes("This is fruits.html"));
});

test(async function servePNG() {
  const [res, body] = await request(PORT, '/images/blue.png');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'image/png');
  assert.strictEqual(res.headers['content-length'], '130');
});

test(async function serveJPG() {
  const [res, body] = await request(PORT, '/images/brown.jpg');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'image/jpeg');
  assert.strictEqual(res.headers['content-length'], '837');
});

test(async function serveSVG() {
  const [res, body] = await request(PORT, '/images/slate.svg');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'image/svg+xml');
  assert.strictEqual(res.headers['content-length'], '227');
});

test(async function servePDF() {
  const [res, body] = await request(PORT, '/images/hello-world.pdf');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'application/pdf');
  assert.strictEqual(res.headers['content-length'], '1313');
});
