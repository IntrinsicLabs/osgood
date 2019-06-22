class MyClass {
  constructor() {
    this.password = 'SHOULD NEVER BE READ BY CLIENT';
    this.username = 'foo';
  }

  toJSON() {
    return {
      username: this.username
    };
  }
}

export default () => {
  return {
    foo: [
      {
        bar: [
          new MyClass()
        ]
      }
    ]
  }
}
