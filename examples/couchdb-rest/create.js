import { dbRequest, json } from './common.js';

export default async function main(request, context) {
  try {
    var record = await request.json();
  } catch (e) {
    return json({error: 'CANNOT_PARSE'}, 401);
  }

  if (record.id || record._id) {
    return json({error: 'CANNOT_SPECIFY_ID'}, 401);
  }

  if (record.created || record.updated) {
    return json({error: 'CANNOT_CHANGE_DATES'}, 401);
  }

  record.created = (new Date()).toISOString();
  record.updated = null;

  const payload = await dbRequest('POST', '', record);

  const obj = await payload.json();

  if (obj.error) {
    return json({ error: 'UNABLE_TO_CREATE' }, 500);
  }

  record.id = obj.id;

  return json(record);
}
