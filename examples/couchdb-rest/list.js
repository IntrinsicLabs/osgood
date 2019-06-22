import { dbRequest, json } from './common.js';

export default async function main(request, context) {
  const payload = await dbRequest('GET', '_all_docs');

  const obj = await payload.json();

  if (obj.error) {
    return json({ error: 'UNABLE_TO_LIST' }, 500);
  }

  const result = [];

  for (let row of obj.rows) {
    result.push(row.id);
  }

  // TODO: should be array of hydrated objects
  return json(result);
}
