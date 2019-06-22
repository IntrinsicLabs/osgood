Osgood runs your JavaScript code using the V8 engine and therefore provides
fairly modern features. For example, the following are available:

- `async` functions / `await` keyword
- `class` syntax and private fields
- `BigInt`
- destructuring assignments
- `WeakMap`, `WeakSet`
- `const`, `let` declarations
- arrow functions
- default parameters
- property shorthands
- regular expression named capture groups

JavaScript is interesting because there are many features which aren't
technically part of the core language. Popular functions like `alert()`,
`setTimeout()`, `atob()`, and `fetch()` are examples. They are instead features
added by various implementations (such as browsers).

We specifically chose to provide features beneficial to server side JavaScript
applications—features that are based on existing implementations available in
other JavaScript environments.

We provide many globals which are commonly used in other JavaScript runtimes.
Here's a list of them:

### Generic Features

- `console.{log,error,warn,info,debug,trace}`
- `setTimeout` / `setInterval` / `clearTimeout` / `clearInterval`
- `atob` / `btoa`
- `ReadableStream`, `WritableStream`, `TransformStream`
- `TextEncoder`, `TextDecoder`
- `URL`, `URLSearchParams`

### Fetch API

Instead of building an interface for making requests from scratch we chose to
implement the familiar `fetch()` interface provided by browsers. These classes
and functions all revolve around the [Fetch
API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API).

- `Request`: read more on [Request](https://developer.mozilla.org/en-US/docs/Web/API/Request)
- `Response`: read more on [Response](https://developer.mozilla.org/en-US/docs/Web/API/Response)
- `Headers`: read more on [Headers](https://developer.mozilla.org/en-US/docs/Web/API/Headers)
- `fetch()`: read more on [fetch](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/fetch)
- `FormData`: read more on [FormData](https://developer.mozilla.org/en-US/docs/Web/API/FormData)
