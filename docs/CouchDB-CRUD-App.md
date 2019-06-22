This application will allow us to build an interface in front of CouchDB. We do
this to keep our clients from becoming too tightly coupled with the backend
system.

Full source code is available at
[examples/couchdb-rest](https://github.com/IntrinsicLabs/osgood/tree/master/examples/couchdb-rest).
Here we'll only be covering a single, complex route.

## Application File `app.js`

The full application will use five different routes, which is a pretty common
CRUD pattern. The five routes are to perform five actions: LIST, VIEW, UPDATE,
DELETE, CREATE. However, we'll only show code for the UPDATE route since that's
the most complicated.

```javascript
app.interface = '127.0.0.1';
app.port = 8000;
app.host = 'localhost';

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

app.post('/users', 'create.js', policy => {
  policy.outboundHttp.allowPost('http://localhost:5984/users');
});

app.put('/users/:id', 'update.js', policy => {
  policy.outboundHttp.allowGet('http://localhost:5984/users/*');
  policy.outboundHttp.allowPut('http://localhost:5984/users/*');
});
```

The above syntax for extracting route parameters is inspired by existing tools
such as Express.js. Each route has a policy function which describes its
capabilities. For example, our route for performing updates needs to both GET
data from CouchDB, as well as PUT data to it.

## Common File `common.js`

This file contains some common tools that we'll use within the different
routes.

For example, when building a JSON-speaking HTTP API, it's common to respond
with JSON data. This requires that we also send headers describing the content
as being JSON. We also want to be able to override status codes for conveying
different errors. That's what the `json` function defined below does.

We also have a function called `dbRequest`, which is a simple function for
generating HTTP requests for our CouchDB server. We use this function to pass
along authentication and set the appropriate request headers.

```javascript
const AUTH = `Basic ${btoa('s3w_admin:hunter12')}`;

export function json(obj, status = 200) {
  const headers = new Headers({
    'Content-Type': 'application/json'
  });

  const body = JSON.stringify(obj);

  const response = new Response(body, { headers, status });

  return response;
}

// Makes a request to CouchDB
export function dbRequest(method = 'GET', path = '', body = '') {
  const options = {
    method,
    headers: {
      Authorization: AUTH
    }
  }

  if (body) {
    options.headers['Content-Type'] = 'application/json';
    options.body = JSON.stringify(body);
  }

  return fetch(`http://localhost:5984/users/${path}`, options);
}
```

## Worker File `update.js`

Here is where the code for our UPDATE route lives. Note that we're able to
`import` and `export` from within files. In this case we're importing code from
`common.js`, code which would naturally be shared among all routes.

Since we're using an `async` function we get to prefix asynchronous operations
with the `await` keyword. This means our worker code ends up being pretty
simple. There's also a ton of error handling going on which is why there are so
many early returns.

We are building this application to prevent the client from directly accessing
the CouchDB server. This is done for a few reasons:

- Hide authentication from the client
- Transform data into a common format
- Enforce certain data requirements (non-writable `id`, `created` time, `modified` time)

```javascript
import { dbRequest, json } from './common.js';

export default async function (request, context) {
  const id = context.params.id;

  if (!id) {
    return json({ error: 'INVALID_REQUEST' }, 400);
  }

  try {
    var record = await request.json();
  } catch (e) {
    return json({error: 'CANNOT_PARSE'}, 401);
  }

  if ((record.id && record.id !== id) || (record._id && record._id !== id)) {
    return json({error: 'CANNOT_RENAME'}, 401);
  }

  if (record.created || record.updated) {
    return json({error: 'CANNOT_CHANGE_DATES'}, 401);
  }

  const existing_record = await dbRequest('GET', id);

  const existing_obj = await existing_record.json();

  if (existing_obj.error && existing_obj.error === 'not_found') {
    return json({ error: 'NOT_FOUND' }, 404);
  }

  // WARNING: This isn't atomic

  const rev = existing_obj._rev;

  record._rev = rev;

  // retain existing created time
  record.created = existing_obj.created;
  record.updated = (new Date()).toISOString();

  const update_payload = await dbRequest('PUT', id, record);

  const update_obj = await update_payload.json();

  if (update_obj.error) {
    return json({ error: 'UNABLE_TO_UPDATE' }, 500);
  }

  delete record._rev; // hide implementation detail
  record.id = update_obj.id;

  return json(record);
}
```
