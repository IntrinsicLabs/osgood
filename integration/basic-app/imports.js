import { imports1 } from './imports/import1.js';
import { imports2 } from './imports/import2.js';

console.log(imports1);
console.log(imports2);
imports1.foo = 8765;
console.log(imports2);

export default () => {
  return { imports1, imports2 };
};
