class MyClass {
  constructor() {
    this.password = 'SHOULD NEVER BE READ BY CLIENT';
    this.username = 'foo';
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
