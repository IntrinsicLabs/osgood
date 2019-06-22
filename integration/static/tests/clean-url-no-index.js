'use strict';
const assert = require('assert');
const {
  test,
  request,
  assertFilterEqual
} = require('./../../common.js');

const PORT = 3004;

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
  assert.strictEqual(res.statusCode, 404);
});

test(async function thingsHtml() {
  const [res, body] = await request(PORT, '/things');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'text/html');
  assert.strictEqual(res.headers['content-length'], '58');
  assert.ok(body.toString().includes("This is things.html"));
});

test(async function redirectToTrailingSlash() {
  const [res, body] = await request(PORT, '/veggies');
  assert.strictEqual(res.statusCode, 404);
});

test(async function indexWithinAFolder() {
  const [res, body] = await request(PORT, '/veggies/');
  assert.strictEqual(res.statusCode, 404);
});

test(async function redirectToWithoutTrailingSlash() {
  const [res, body] = await request(PORT, '/fruits/');
  assert.strictEqual(res.statusCode, 301);
  assert.strictEqual(res.headers['content-length'], '43');
  assert.ok(body.toString().includes("301: moved to http://localhost:3004/fruits\n"));
});

test(async function cleanUrlFile() {
  const [res, body] = await request(PORT, '/fruits');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'text/html');
  assert.strictEqual(res.headers['content-length'], '58');
  assert.ok(body.toString().includes("This is fruits.html"));
});

test(async function cleanUrlFileWithHtmlExt() {
  const [res, body] = await request(PORT, '/fruits.html');
  assert.strictEqual(res.statusCode, 404);
});

test(async function cleanUrlFile() {
  const [res, body] = await request(PORT, '/drinks/soft/');
  assert.strictEqual(res.statusCode, 404);
});

test(async function redirectToWithoutTrailingSlash2() {
  const [res, body] = await request(PORT, '/drinks/orange/');
  assert.strictEqual(res.statusCode, 301);
  assert.strictEqual(res.headers['content-length'], '50');
  assert.ok(body.toString().includes("301: moved to http://localhost:3004/drinks/orange\n"));
});

test(async function cleanUrlFile2() {
  const [res, body] = await request(PORT, '/drinks/orange');
  assert.strictEqual(res.statusCode, 200);
  assert.strictEqual(res.headers['content-type'], 'text/html');
  assert.strictEqual(res.headers['content-length'], '65');
  assert.ok(body.toString().includes("This is drinks/orange.html"));
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
