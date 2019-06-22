'use strict';

const http = require('http');
const https = require('https');
const assert = require('assert');

const request = (port, url, opts = {}, reqBody = null) =>
  new Promise((resolve, reject) => {
    if (!url.startsWith('http:') && !url.startsWith('https:')) {
      url = 'http://localhost:'  + port + url;
    }
    (url.startsWith('https:') ? https : http).request(url, opts)
    .on('response', async res => resolve([res, await consume(res)]))
    .on('timeout', () => reject(new Error('request timed out')))
    .on('error', reject)
    .end(reqBody);
  });

async function consume(strm) {
  const chunks = [];
  for await (const chunk of strm) {
    chunks.push(chunk);
  }
  return Buffer.concat(chunks);
}

const testFns = [];

const test = testFns.push.bind(testFns);

const filter = (terms, obj) => Object.keys(obj).reduce((acc, cur) => {
  if (!terms.includes(cur)) {
    acc[cur] = obj[cur];
  }
  return acc;
}, {});

function assertFilterEqual(terms, a, b) {
  const fa = filter(terms, a);
  const fb = filter(terms, b);
  return assert.deepStrictEqual(fa, fb);
}


const red = process.stdout.isTTY ? str => `\x1b[31m${str}\x1b[0m` : str => str;
const green = process.stdout.isTTY ? str => `\x1b[32m${str}\x1b[0m` : str => str;

function runTests() {
  return Promise.all(testFns.map(fn => (async () => {
    const name = process.stdout.isTTY ? `\x1b[1m${fn.name}\x1b[22m` : fn.name;
    try {
      await fn();
      console.log(green(`[PASS]: ${name}`));
    } catch (e) {
      console.log(red(`[FAIL]: ${name}\n${e.stack}`));
      process.exitCode = 1;
    }
  })()));
}

module.exports = {
  assertFilterEqual,
  test,
  request,
  runTests,
};
