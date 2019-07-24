'use strict';
const assert = require('assert');
const {
  test,
  request
} = require('./../../common.js');

const PORT = 3000;

test(async function evil() {
  const [res, body] = await request(PORT, '/evil');
  assert.strictEqual(res.statusCode, 500);
  assert.strictEqual(body.toString(), '');
  // TODO check process output for policy violation
});

test(async function badstart() {
  const [res, body] = await request(PORT, '/badstart');
  assert.strictEqual(res.statusCode, 503);
  assert.strictEqual(body.toString(), 'route not available: /badstart\n');
});

test(async function nohandler() {
  const [res, body] = await request(PORT, '/nohandler');
  // TODO is this correct behaviour????
  assert.strictEqual(res.statusCode, 500);
  assert.strictEqual(body.toString(), '');
});

test(async function badhandler() {
  const [res, body] = await request(PORT, '/badhandler');
  assert.strictEqual(res.statusCode, 500);
  assert.strictEqual(body.toString(), '');
});

test(async function syntaxerror() {
  const [res, body] = await request(PORT, '/syntaxerror');
  assert.strictEqual(res.statusCode, 503);
  assert.strictEqual(body.toString(), 'route not available: /syntaxerror\n');
});

test(async function complexBad() {
  const [res, body] = await request(PORT, '/complex-bad');
  assert.strictEqual(res.statusCode, 500);
  assert.strictEqual(body.toString(), '');
});

test(async function connectionRefused() {
  const [res, body] = await request(PORT, '/connection-refused');
  assert.strictEqual(res.statusCode, 500);
  assert.strictEqual(body.toString(), '');
});

test(async function badProtocol() {
  const [res, body] = await request(PORT, '/bad-protocol');
  assert.strictEqual(res.statusCode, 500);
  assert.strictEqual(body.toString(), '');
});
