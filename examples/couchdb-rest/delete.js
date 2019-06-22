import { dbRequest, json } from './common.js';

export default async function main(request, context) {
  const id = context.params.id;

  if (!id) {
    return json({ error: 'INVALID_REQUEST' }, 400);
  }

  const payload = await dbRequest('GET', id);

  const obj = await payload.json();

  if (obj.error && obj.error === 'not_found') {
    return json({ error: 'NOT_FOUND' }, 404);
  }

  if (obj.error) {
    return json({ error: 'UNABLE_TO_DELETE' }, 500);
  }

  // WARNING: This isn't atomic

  const rev = obj._rev;

  const delete_payload = await dbRequest('DELETE', `${id}?rev=${rev}`);

  const delete_obj = await delete_payload.json();

  delete obj._rev; // hide implementation detail
  obj.id = obj._id;
  delete obj._id; // hide implementation detail

  if (!delete_obj.ok) {
    return json({error: 'UNABLE_TO_DELETE'}, 500);
  }

  return json(obj);
}
