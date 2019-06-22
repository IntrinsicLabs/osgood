#!/usr/bin/env osgood

// app.interface = '0.0.0.0'; -- default so commented it out
app.port = 3000;
//app.host = 'localhost'; -- default so commented it out

app.get('/hello', 'hello.js');

app.route('GET', '/gh-merge/:username', 'gh-merge.js', policy => {
  policy.outboundHttp.allowGet('https://api.github.com/users/*/gists');
  policy.outboundHttp.allowGet('https://api.github.com/users/*/repos');
});
