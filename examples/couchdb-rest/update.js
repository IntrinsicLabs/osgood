import { dbRequest, json } from './common.js';

export default async function main(request, context) {
  const id = context.params.id;

  if (!id) {
    return json({ error: 'INVALID_REQUEST' }, 400);
  }

  try {
    var record = await request.json();
  } catch (e) {
    return json({error: 'CANNOT_PARSE'}, 401);
  }

  if ((record.id && record.id !== id) || (record._id && record._id !== id)) {
    return json({error: 'CANNOT_RENAME'}, 401);
  }

  if (record.created || record.updated) {
    return json({error: 'CANNOT_CHANGE_DATES'}, 401);
  }

  const existing_record = await dbRequest('GET', id);

  const existing_obj = await existing_record.json();

  if (existing_obj.error && existing_obj.error === 'not_found') {
    return json({ error: 'NOT_FOUND' }, 404);
  }

  // WARNING: This isn't atomic

  const rev = existing_obj._rev;

  record._rev = rev;

  // retain existing created time
  record.created = existing_obj.created;
  record.updated = (new Date()).toISOString();

  const update_payload = await dbRequest('PUT', id, record);

  const update_obj = await update_payload.json();

  if (update_obj.error) {
    return json({ error: 'UNABLE_TO_UPDATE' }, 500);
  }

  delete record._rev; // hide implementation detail
  record.id = update_obj.id;

  return json(record);
}
