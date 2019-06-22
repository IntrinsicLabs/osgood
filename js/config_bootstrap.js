{
  const formatLog = args =>
    args.map(x => (typeof x === 'string' ? x : JSON.stringify(x))).join(' ');

  console.log = (...args) => {
    _log(formatLog(args));
  };

  console.error = (...args) => {
    _error(formatLog(args));
  };

  console.warn = (...args) => {
    _error(formatLog(args));
  };

  console.info = (...args) => {
    _log(formatLog(args));
  };

  console.debug = (...args) => {
    _log(formatLog(args));
  };

  console.trace = (...args) => {
    const { stack } = new Error();
    const formattedStack = stack
      .split('\n')
      .slice(2)
      .join('\n');
    _log(`${formatLog(args)}\n${formattedStack}`);
  };

  const httpMethods = [
    'Get',
    'Post',
    'Put',
    'Patch',
    'Delete',
    'Head',
    'Options',
    'Trace',
    'Connect'
  ];

  Object.defineProperty(this, 'app', {
    value: {},
    writable: false,
    enumerable: true,
    configurable: false
  });

  // port defaults to 8080
  let port = 8080;
  Reflect.defineProperty(app, 'port', {
    get: () => port,
    set(p) {
      if (!Number.isInteger(p) || p < 0) {
        throw new Error('port must be a valid port number');
      }
      port = p;
    },
    enumerable: true,
    configurable: false
  });

  // interface defaults to 0.0.0.0
  let interface = '0.0.0.0';
  Reflect.defineProperty(app, 'interface', {
    get: () => interface,
    set(i) {
      if (typeof i !== 'string') {
        throw new Error('interface must be a valid IP address');
      }
      interface = i;
    },
    enumerable: true,
    configurable: false
  });

  // host defaults to localhost
  app.host = 'localhost';

  app.routes = [];
  app.staticRoutes = [];

  app.static = (routePrefix, directory, options = {}) => {
    if (typeof routePrefix !== 'string') {
      throw new TypeError('routePrefix must be a string');
    }

    if (routePrefix.endsWith('/')) {
      routePrefix = routePrefix.substring(0, routePrefix.length - 1);
    }

    if (typeof directory !== 'string') {
      throw new TypeError('directory must be a string');
    }

    if (directory.endsWith('/')) {
      directory = directory.substring(0, directory.length - 1);
    }

    app.staticRoutes.push({ routePrefix, directory, options });
  };

  const formatRoute = route => {
    const formattedRoute = route.replace(/\:([a-zA-Z0-9_]+)/g, '*');
    return formattedRoute;
  };

  // TODO: method should also accept an array
  app.route = (method, route, worker, policyFn = () => {}) => {
    const policyWriter = {
      outboundHttp: {}
    };
    const policies = [];
    for (const method of httpMethods) {
      policyWriter.outboundHttp[`allow${method}`] = pattern => {
        // TODO check if pattern is formatted correctly (no hashes or query params)
        policies.push({ method: method.toUpperCase(), pattern });
      };
    }
    policyFn(policyWriter);
    app.routes.push({
      method,
      pattern: formatRoute(route),
      rawPattern: route,
      file: worker,
      policies
    });
  };

  // Syntax Sugar
  for (const method of httpMethods) {
    app[method.toLowerCase()] = (route, worker, policyFn) => {
      app.route(method.toUpperCase(), route, worker, policyFn);
    };
  }
}
