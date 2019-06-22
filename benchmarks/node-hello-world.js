#!/usr/bin/env node

console.log(`Node.js ${process.version}`);

const http = require('http');
// This can be benchmarked against the /hello endpoint
// Both servers send the same number of bytes

const hostname = '127.0.0.1';
const port = 3000;

const server = http.createServer((req, res) => {
  res.statusCode = 200;
  res.setHeader('Content-Type', 'text/plain');
  res.removeHeader('Connection');
  res.end('Hello, world!');
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
