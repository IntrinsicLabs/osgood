#!/usr/bin/env osgood
// app.interface = '0.0.0.0'; -- default so commented it out
app.port = 3000;
//app.host = 'localhost'; -- default so commented it out

app.get('/hello', 'osgood-hello-world-worker.js');
