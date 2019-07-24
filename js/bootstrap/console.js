import FormData from 'internal:form_data.js';

const {
  _log,
  _error,
} = self._bindings;

function inspect(obj) {
  const INDENT = 2;
  const seen = new WeakSet();
  let depth = 0;

  function pad(groupTerm) {
    return ' '.repeat(groupTerm ? depth - INDENT : depth);
  }

  function dive(node) {
    depth += INDENT;
    if (node === null) {
      depth -= INDENT;
      return `null`;
    }
    if (Array.isArray(node)) {
      let result = `[\n`;
      for (let item of node) {
        result += `${pad()}${dive(item)}\n`;
      }
      result += `${pad(true)}]`;
      depth -= INDENT;
      return result;
    }
    if (node instanceof URLSearchParams) {
      depth -= INDENT;
      return `URLSearchParams { ${node.toString()} }`;
    }
    if (node instanceof FormData) {
      depth -= INDENT;
      const pairs = [];
      for (const fd of node) {
        // Using stringify to escape quotes and remove ambiguity for human reader
        pairs.push(`${fd[0]}=${JSON.stringify(fd[1])}`);
      }
      return `FormData { ${pairs.join(', ')} }`;
    }

    const type = typeof node;

    switch (type) {
      case 'undefined':
        depth -= INDENT;
        return `undefined`;
      case 'function':
        depth -= INDENT;
        return `${node.name}(${node.length})`;
      case 'bigint':
        depth -= INDENT;
        return `${node}n`;
      case 'number':
      case 'boolean':
      case 'symbol':
        depth -= INDENT;
        return `${String(node)}`;
      case 'string':
        depth -= INDENT;
        return `'${node}'`;
      case 'object':
        if (seen.has(node)) {
          depth -= INDENT;
          return `[CIRCULAR]`;
        }
        seen.add(node);
        const keys = Reflect.ownKeys(node);
        let result = `${
            node.constructor !== Object && node.constructor !== undefined ? node.constructor.name + ' ' : ''
          }{\n`;
        for (let key of keys) {
          result += `${pad()}${String(key)}: ${dive(node[key])}\n`;
        }
        result += `${pad(true)}}`;
        depth -= INDENT;
        return result;
      default:
        throw new Error(`unknown type: ${type}`);
    }
  }

  return dive(obj);
}

const formatLog = args =>
  args.map(x => (typeof x === 'string' ? x : inspect(x))).join(' ');

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
