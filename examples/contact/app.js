#!/usr/bin/env osgood

app.port = 3000;

app.static('/', './static');

app.route('POST', '/contact', 'contact.js', policy => {
  policy.outboundHttp.allowPost('https://api.mailgun.net/v3/samples.mailgun.org/messages');
});
