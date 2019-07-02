An Osgood Application file has access to a global object called `app`. By
setting various properties on this object, we're able to configure the behavior
of our application.

## App Configuration

`app.interface`: This is the name of the interface we'll listen on. It defaults
to `0.0.0.0`, which means all interfaces. You can also set it to `127.0.0.1`,
which means only requests from the local machine will work. You can also set it
to the IP address of a hardware interface on your machine.

`app.port`: This is the port which Osgood will listen on. By default, it listens
on `8080`.

```javascript
app.interface = '127.0.0.1';
app.port = 8080;
app.host = 'localhost';
```

## Routing

After the application basics have been configured, we can go ahead and configure
the different routes used in our application. This can be done by calling
methods on the `app` objects. Each of these methods have the same signature:

- `app.get(routePattern, workerFilename, policyFunction)`
- `app.post(...)`
- `app.put(...)`
- `app.patch(...)`
- `app.delete(...)`
- `app.head(...)`
- `app.options(...)`
- `app.trace(...)`
- `app.connect(...)`

### Route Pattern

The route pattern is essentially a
[glob](https://www.npmjs.com/package/glob#glob-primer) with the added ability
to extract URL parameters. It is specifically matched against the path of the
requested URL. Here's a quick explanation of how it works:

- An asterisk (`*`) refers to any character that isn't a forward slash
- A colon (`:`) followed by `[a-zA-Z0-9_]` is similar to an asterisk but is captured for `context.params.paramName`
- A double asterisk (`**`) refers to any character including a forward slash

### Worker Filename

This is the path to the file to be loaded for handling requests.

### Policy Function

Policy Functions are used for configuring the security policies used by Osgood
Workers. They're configured by calling methods available on the `policy`
argument. The only policies currently available in Osgood are available on the
`policy.outboundHttp` object.

The following example will _only_ match requests for `GET http://localhost:8000/users`:

```javascript
app.get('/users', 'foo.js', policy => {
  policy.outboundHttp.allowGet('http://localhost:5984/users/_all_docs');
});
```

### Routing Examples

The following example will match requests for `GET
http://localhost:8000/users/admin` but will not match requests for either `GET
http://localhost:8000/users/admin/xyz` or `POST
http://localhost:8000/users/admin`:

```javascript
app.get('/users/:id', 'view.js', policy => {
  policy.outboundHttp.allowGet('http://localhost:5984/users/*');
});
```

## API Security Policies

Osgood applies the _Principle of Least Privilege_ on a per-worker basis. This
means that by default a worker isn't allowed to talk to third party services.
By writing policies the developer is able to whitelist ahead of time which
outbound services can be communicated with, and how they may be communicated
with.

These policies are based on our existing [Intrinsic for Node.js HTTP
Policies](https://intrinsic.com/docs/latest/policy-outbound-http.html), which
have proved to be a simple and effective approach for securing servers.

A policy function looks like this:

```javascript
(policy) => {
  policy.outboundHttp.allowGet('http://api.local:123/users/*');
  policy.outboundHttp.allowPost('http://api.local:123/widgets/**');
}
```

Policies are configured by using the `policy.outboundHttp` object. This object
has several methods correlating to popular HTTP methods, each with the same
signature:

- `allowGet(urlPattern)`
- `allowPost(...)`
- `allowPut(...)`
- `allowPatch(...)`
- `allowDelete(...)`
- `allowHead(...)`
- `allowOptions(...)`
- `allowTrace(...)`
- `allowConnect(...)`

The `urlPattern` argument is similar in syntax to the incoming HTTP request
pattern, except that there are no parameter capturing.

- An asterisk (`*`) refers to any character that isn't a forward slash
- A double asterisk (`**`) refers to any character including a forward slash

Port numbers should only be supplied if the URL is using using a port which
doesn't match the protocol, for example `http:` with `80` or `https:` with
`443`.

## Static Routes

Static routes can be configured using the `app.static()` method.

```javascript
app.static(routePrefix, path);
```

Unlike the other routes which accept complex patterns, the `routePrefix`
argument here only works as a prefix. For example with a value of `/assets`,
any request falling under `http://localhost:3000/assets` will trigger the
static route.

The `path` argument is a path to a directory to serve content from. With a
value set to `public`, and a `routePrefix` set to `/assets`, a request for
`http://localhost:3000/assets/styles/main.css` will translate to
`./public/styles/main.css`.

The `Content-Type` header is inferred solely based on the file extension. For
example, the file `style.css` will result in `text/css`, whereas a file without
an extension such as `foobar` will fallback to `application/octet-stream`.

### Caveats:

- The `path` argument must point to a directory, not a file
- There is no concept of an `index` file
  - Any request directly to a directory without a filename (i.e. `/assets`) will fail
  - We may allow this to be configurable in the future, e.g., setting to `index.html`
