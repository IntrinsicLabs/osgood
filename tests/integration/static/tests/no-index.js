'use strict';
const assert = require('assert');
const {
  test,
  request,
  assertFilterEqual
} = require('./../../common.js');

const PORT = 3003;

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
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'text/html');
  assert.strictEqual(res.headers['content-length'], '57');
  assert.ok(body.toString().includes("This is index.html"));
});

test(async function thingsHtml() {
  const [res, body] = await request(PORT, '/things.html');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'text/html');
  assert.strictEqual(res.headers['content-length'], '58');
  assert.ok(body.toString().includes("This is things.html"));
});


test(async function cleanUrlFileWithHtmlExt() {
  const [res, body] = await request(PORT, '/veggies/');
  assert.strictEqual(res.statusCode, 404);
});
