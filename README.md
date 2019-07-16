![Osgood](./osgood.svg)

Osgood is a secure, fast, and simple platform for running JavaScript HTTP
servers. It is written using Rust and V8.

Services written today share a common flaw: Being over-privileged. Osgood is an
attempt to build a platform from the ground up, one which applies the
[_Principle of Least
Privilege_](https://en.wikipedia.org/wiki/Principle_of_least_privilege) at its
very core. Osgood requires that policies be written ahead of time describing
the I/O requirements of an application. If such an operation hasn't been
whitelisted, it will fail. Developers familiar with JavaScript development in
the web browser should feel right at home with the APIs provided in Osgood.


## Documentation

[Osgood Documentation](https://github.com/IntrinsicLabs/osgood/wiki)


## Hello, World!

```javascript
// app.js
app.port = 3000;

app.get('/hello', 'hello-worker.js');
```

```javascript
// hello-worker.js
export default () => 'Hello, World!';
```

```bash
$ osgood app.js
$ curl http://localhost:3000/hello
```


## What is Osgood?

Osgood is a JavaScript runtime purpose-built to run HTTP servers. Its goals are
to provide a secure way to build HTTP servers that are fast and simple. Osgood
handles server routing and configuration for you, allowing you to focus on
application code.

Osgood gives you fine-grained control over your application's privileged
operations. It follows the [_Principle of Least
Privilege_](https://en.wikipedia.org/wiki/Principle_of_least_privilege) by
prohibiting workers from accessing resources you don't explicitly allow.

Here's an example policy:

```javascript
policy.outboundHttp.allowGet('https://intrinsic.com');
```


## Installing Osgood

### Download a Prebuilt Release

All prebuilt releases can be downloaded from the
[Releases](https://github.com/IntrinsicLabs/osgood/releases) page.

### Building Osgood

We have more information on compiling Osgood on our [Building
Osgood](https://github.com/IntrinsicLabs/osgood/wiki/Building) wiki page.


## Osgood Overview

### Application File

An Osgood application file is essentially the entrypoint for the application.
Each application will have a single application file. It is the only necessary
argument for the `osgood` command.

This file has three purposes:

- Configure global settings such as port and interface
- Route incoming requests to the desired Osgood worker
- Configure the security policies for each Osgood worker

More information about Osgood application files are available on the [Osgood
Application
File](https://github.com/IntrinsicLabs/osgood/wiki/Osgood-Application-File)
wiki page.


### Worker File

An Osgood worker file works by exporting a default function. Typically you'll
export an `async` function but it also works fine by returning a promise or a
string value.

Workers are called with information about the incoming request and the returned
value is then used to dictate the response to the client.

More information about Osgood worker files are available on the [Osgood Worker
Files](https://github.com/IntrinsicLabs/osgood/wiki/Osgood-Worker-Files) wiki
page.

## Contributing

Contributions are welcome! Please see [`CONTRIBUTING.md`](./CONTRIBUTING.md).

## License

Osgood uses the MIT License. Please see [`LICENSE.txt`](./LICENSE.txt).
