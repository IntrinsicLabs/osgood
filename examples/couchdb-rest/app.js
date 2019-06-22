app.interface = '127.0.0.1';
app.port = 8000;
app.host = 'localhost';

// TODO: Need an optional trailing slash
app.get('/users', 'list.js', policy => {
  policy.outboundHttp.allowGet('http://localhost:5984/users/_all_docs');
});

app.get('/users/:id', 'view.js', policy => {
  policy.outboundHttp.allowGet('http://localhost:5984/users/*');
});

app.delete('/users/:id', 'delete.js', policy => {
  policy.outboundHttp.allowGet('http://localhost:5984/users/*');
  policy.outboundHttp.allowDelete('http://localhost:5984/users/*');
});

// TODO: Need an optional trailing slash
app.post('/users', 'create.js', policy => {
  policy.outboundHttp.allowPost('http://localhost:5984/users/');
});

app.put('/users/:id', 'update.js', policy => {
  policy.outboundHttp.allowGet('http://localhost:5984/users/*');
  policy.outboundHttp.allowPut('http://localhost:5984/users/*');
});
