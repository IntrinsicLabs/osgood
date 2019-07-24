export default class FormData {

  // TODO: This could be a Map<name, [[value, filename?]]> for efficient lookups
  // However, there shouldn't be more than a dozen entries
  // Duplicate names are allowed to exist otherwise it could be a simple Map<name, [value, filename?]>
  #data = [];

  constructor(form) {
    if (form) {
      throw new TypeError("Osgood FormData doesn't support a form argument");
    }
  }

  // FormData can have duplicate entries
  append(name, value, filename) {
    if (filename) {
      // TODO: if (!(value instanceof Blob) && !(value instanceof File)) { value = String(value); }
      // TODO: There's some more logic about extracting filename from File arg, and defaulting filename to 'blob'
      throw new TypeError("Osgood currently doesn't support files");
    }

    const d = [name, value];

    // if (typeof filename !== 'undefined') {
    //   d.push(String(filename));
    // }

    this.#data.push(d);
  }

  // destroys all existing entries with same name
  set(name, value, filename) {
    this.delete(name);
    this.append(name, value, filename);
  }

  // destroys all existing entries with same name
  delete(name) {
    const new_data = [];

    for (let entry of this.#data) {
      if (entry[0] !== name) {
        new_data.push(entry);
      }
    }

    this.#data = new_data;
  }

  // get first entry of `name`
  get(name) {
    for (let entry of this.#data) {
      if (entry[0] === name) {
        return entry[1];
      }
    }
  }

  // get array of entries of `name`
  getAll(name) {
    const matches = [];

    for (let entry of this.#data) {
      if (entry[0] === name) {
        matches.push(entry[1]);
      }
    }

    return matches;
  }

  has(name) {
    for (let entry of this.#data) {
      if (entry[0] === name) {
        return true
      }
    }

    return false;
  }

  entries() {
    return this.#data[Symbol.iterator]();
  }

  [Symbol.iterator]() {
    return this.#data[Symbol.iterator]();
  }

  // iterator<key>
  *keys() {
    for (let entry of this.#data) {
      yield entry[0];
    }
  }

  // iterator<value>
  *values() {
    for (let entry of this.#data) {
      yield entry[1];
    }
  }

  // not in the spec but Firefox and Chrome have it
  forEach(fn) {
    for (let entry of this.#data) {
      fn(entry[0], entry[1], this);
    }
  }

  toString() {
    return '[object FormData]';
  }
}
