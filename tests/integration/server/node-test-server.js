#!/usr/bin/env node
'use strict';

// This is a Node.js server for mimicing the complex HTTP interactions that
// Osgood may need to deal with

const formidable = require('formidable');
const http = require('http');

const image = Buffer.from(
  'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAA' +
  'AfFcSJAAAABGdBTUEAALGPC/xhBQAAACBjSFJN' +
  'AAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mA' +
  'AAF3CculE8AAAADUlEQVQIHWPYd/3jfwAIYwOG' +
  'l/DKWAAAAABJRU5ErkJggg==',
  'base64'
);

http.createServer((req, res) => {
  if (req.url === '/') {
    res.writeHead(200, {
      'Content-Type': 'text/plain'
    });
    res.end('ok');
    return;
  }

  if (req.url === '/form-echo') {
    if (req.method !== 'POST') {
      res.writeHead(405, {
        'Content-Type': 'application/json'
      });
      res.end(JSON.stringify({
        ok: false,
        error: `expected POST, got ${req.method}`
      }));
      return;
    }

    const form = new formidable.IncomingForm();

    form.parse(req, (err, fields, files) => {
      if (err) {
        res.writeHead(400, {
          'Content-Type': 'application/json'
        });
        res.end(JSON.stringify({
          ok: false,
          error: err.message
        }));
        return;
      }

      res.writeHead(200, {
        'Content-Type': 'application/json'
      });
      res.end(JSON.stringify({
        ok: true,
        fields,
        files,
        headers: req.headers
      }));
      return;
    });
  }

  if (req.url === '/echo' && req.method === 'POST') {
    req.pipe(res);
    return;
  }

  if (req.url === '/image.png') {
    res.writeHead(200, {
      'Content-Type': 'image/png'
    });
    res.end(image);
    return;
  }

  if (req.url === '/?query=test') {
    res.writeHead(200, {
      'Content-Type': 'text/plain'
    });
    res.end('ok');
    return;
  }


}).listen(9001);

console.log(`integration server listening at localhost:9001`);
