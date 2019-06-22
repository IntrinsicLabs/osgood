Here's an example of a Hello World application built using Osgood. This
application will only use a single route, `GET /hello`, and so we'll only need
two files. The [Application File](Osgood-Application-File) handles application
routing, configuration, and policies. The [Worker File](Osgood-Worker-Files)
handles the actual application logic for the route.

### Application File: `app.js`

This configuration file is fairly minimal. For a larger overview check out the
[Application File](Osgood-Application-File) page.

Here we configure the app to listen on port `3000`. We also define a route at
`GET /hello`, which will be routed to the worker file `hello.js`. Also, since
the application doesn't need to perform any I/O, the policy configuration
argument is just a noop function. This means the worker _cannot_ perform any
I/O, even if an attacker were able to `eval()` arbitrary code within the
worker.

```javascript
app.port = 3000;
app.get('/hello', 'hello.js', policy => {});
```

### Worker File `hello.js`

Here we describe the application code. The default exported function accepts
two arguments, `request` and `context`. The `request` argument is an instance
of the [Request](https://developer.mozilla.org/en-US/docs/Web/API/Request)
class available in modern browsers. The second argument, `context`, contains
some additional information described in the [Osgood Worker
Files](Osgood-Worker-Files#requests) page.

This function can either return a promise (either directly or by virtue of
being an `async` function), or it can return a value directly. In this example
we're simply returning a string which will then be sent to the client.

```javascript
export default async (request, context) => {
  return "Hello, World!";
};
```

### Command Line

Now let's execute our application. Once you've followed along with the
[install](Installation) instructions you're then ready to execute the code. Run
the following commands in two different terminals to call your application
code:

```shell
$ osgood ./app.js
$ curl http://localhost:3000/hello
```

Once the `curl` call is complete you should see the text `Hello, World!`
displayed in your terminal.
