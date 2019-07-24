export function isBufferish(chunk) {
  if (!chunk) {
    return false;
  }
  return chunk instanceof ArrayBuffer ||
    (chunk.buffer && chunk.buffer instanceof ArrayBuffer);
}

export function unimplemented() {
  throw new Error('Unimplemented!');
}

// FIXME: This currently only implements pair iterators!
// https://heycam.github.io/webidl/#idl-iterable
export class IteratorMixin {
  // https://heycam.github.io/webidl/#es-iterable-entries
  entries() {
    return this[Symbol.iterator]();
  }

  // https://heycam.github.io/webidl/#es-forEach
  forEach() {
    unimplemented();
  }

  // https://heycam.github.io/webidl/#es-iterable-keys
  *keys() {
    for (const [key, value] of this) {
      yield key;
    }
  }

  // https://heycam.github.io/webidl/#es-iterable-values
  *values() {
    for (const [key, value] of this) {
      yield value;
    }
  }

  static mixin(klass) {
    if (!(Symbol.iterator in klass.prototype)) {
      throw new Error('Cannot mixin IteratorMixin because class is not iterable');
    }
    for (const key of Reflect.ownKeys(IteratorMixin.prototype)) {
      if (key === 'constructor') {
        continue;
      }
      if (key in klass.prototype) {
        throw new Error(`Cannot mixin IteratorMixin because key '${key}' already exists`);
      }
      klass.prototype[key] = IteratorMixin.prototype[key];
    }
  }
}

export class StringReadable extends ReadableStream {
  constructor(string) {
    super({
      start(controller) {
        controller.enqueue(string);
        controller.close();
      }
    });
  }
}
