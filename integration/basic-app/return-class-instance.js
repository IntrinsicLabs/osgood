// This class is a valid response because it has a toJSON method
class User {
  constructor(username, password) {
    this.username = username;
    this.password = password;
  }

  toJSON() {
    return {
      username: this.username,
    };
  }
}

export default () => new User('osgood', 'SHOULD NEVER BE READ BY CLIENT');
