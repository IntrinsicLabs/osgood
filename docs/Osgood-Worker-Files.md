Osgood Worker Files are given a different set of globals than the application
file. For example, there is no `app` global available.

Each worker file will run in a separate thread from the others, as well as a
separate thread from the application file. This means that no global state can
be shared between them. This also means that if two different workers `import`
the same file, no instantiated singletons may be shared.

## Requests

A worker file works by exporting a default function. This function will receive
two arguments. An example looks like the following:

```javascript
export default async (request, context) => {
  console.log(request.url); // 'http://localhost:8000/users/tlhunter'
  console.log(request.headers); // instanceof Headers
  console.log(request.method); // 'POST'
  console.log(request.body); // instanceof ReadableStream
  console.log(context.params); // { username: 'tlhunter' }
  console.log(context.query); // instanceof URLSearchParams
}
```

The `request` argument is an instance of
[Request](https://developer.mozilla.org/en-US/docs/Web/API/Request). It
contains only the most basic information about the incoming request. The
`context` argument provides some additional niceties added by Osgood.

### Parsing an Incoming Body

Parsing an incoming body works the same way as it would inside of a Service
Worker in your browser. If you're receiving a JSON request from the client,
such as within a POST request, you can have the content parsed for you by
running the following:

```javascript
const body = await request.json();
```

Keep in mind that if the request contains invalid JSON, the operation will
throw an error.


## Responses

An Osgood Worker function decides what response to provide to the client based
on the return value. If a promise is returned then the resolved value is used
for the response. Otherwise, if a simple object or string is returned, then
that will be used as the response. For the most control an Osgood Worker can
return an instance of
[Response](https://developer.mozilla.org/en-US/docs/Web/API/Response), which
allows setting headers and a status code.

However, there are a few caveats to this approach that you should be aware of.

### Default Values

Osgood will attempt to provide a default `Content-Type` header when a value is
returned which isn't an instance of Response.

#### String

If a string is returned, then the content type will be set to `text/plain`. If
you plan on returning a different value, such as HTML, you'll need to make use
of a Response object and set the headers manually. If you want to return
another primitive value, like a `boolean` or a `number`, then you'll need to
manually convert it into a string first.

#### TypedArray or ArrayBuffer

If an instance of a `TypedArray`—such as `Uint8Array`—or an `ArrayBuffer` is
returned then the content type will be set to `application/octet-stream`.

#### POJO Object

A POJO (Plain Ol' JavaScript Object) is an object with a prototype set to
either `null` or to `Object.prototype`. Specifically, it is a simple object
probably created manually with `{}` brackets, and is not an instance of a
class.

If a value being returned is a POJO, then the value will be converted into a
JSON string representation and the content type header will be set to
`application/json`. This is convenient for spinning up simple API servers.

#### Class Instance

However, if the value is an Object but not a POJO, such as an instance of a
class, then we won't simply convert the object into JSON and reply with it.
This may sound like a pain but it was actually a deliberate decision chosen for
security reasons.

Consider, for example, a `User` class which has a `username` and `displayName`
property. This seems like a likely object to serialize into a string. However,
if deep within the application someone modifies the object to then contain a
`password` field, the application is now accidentally leaking private data.

```javascript
// Anti-Pattern: This will fail
class User { }
const joe = new User();
export default function() {
  return joe;
}
```

For this reason, if a class instance is returned, or an object which at any
point contains a class instance, the request will fail.

The pattern we would like to promote is specifically returning a new POJO
object at the end of a worker. This is convenient because the "contract" of
your application is clearly defined, and deeper changes within an application
don't affect output (which can potentially break consuming code):

```javascript
export default function() {
  const joe = new User();

  return {
    username: joe.username,
    displayName: joe.displayname
  };
}
```

#### Class Instance with `.toJSON()`

As part of our decision to deliberately prevent class instances from being
passed as responses, we did specifically make it acceptable to provide class
instances with a `.toJSON()` method. We chose this because the developer is
then intentionally specifying exactly which properties should be returned in
the response.

```javascript
// This is OK
class User {
  constructor(username, password) {
    this.username = username;
    this.password = password;
  }
  toJSON() {
    return {
      username: this.username
    };
  }
}
const joe = new User('joe', 'hunter12');
export default function() {
  return joe;
}
```

The same rules apply for deeply nested class instances so make sure any object
you return are either not class instances or contain a `toJSON` method.

### Response Object

For more control over the response one can return an instance of the `Response`
object. This allows for setting things like headers and status codes (which are
otherwise set to `200`). Here's an example of how to do this:

```javascript
export default function(request, context) {
  const payload = {
    isCool: context.params.username
  };
  const status = 451;

  const headers = new Headers({
    'Content-Type': 'application/vnd.widgetcorp+json'
  });

  const body = JSON.stringify(payload);

  const response = new Response(body, {
    headers,
    status
  });

  return response;
};
```
