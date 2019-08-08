#!/usr/bin/env osgood

app.get('/hello', 'hello.js');

app.route('GET', '/gh-merge/:username', 'gh-merge.js', policy => {
  policy.outboundHttp.allowGet('https://api.github.com/users/*/gists');
  policy.outboundHttp.allowGet('https://api.github.com/users/*/repos');
});
