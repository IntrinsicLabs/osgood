export default async (request, context) => {
  try {
    var incoming = await request.json();
  } catch (e) {
    return json({error: 'CANNOT_PARSE', message: "Invalid JSON Payload Provided"}, 400);
  }

  const email = new FormData();

  if (!('email' in incoming) || !('name' in incoming) || !('message' in incoming)) {
    return json({error: 'MISSING_FIELDS', message: "Invalid JSON Payload Provided"}, 400);
  } else if (typeof incoming.email !== 'string' || typeof incoming.name !== 'string' || typeof incoming.message !== 'string') {
    return json({error: 'INVALID_FIELD_TYPES', message: "Invalid JSON Payload Provided"}, 400);
  } else if (!incoming.name) {
    return json({error: 'EMPTY_VALUE', message: "You forgot to supply a name!", field: 'name'}, 400);
  } else if (!incoming.email) {
    return json({error: 'EMPTY_VALUE', message: "You forgot to supply an email address!", field: 'email'}, 400);
  } else if (!incoming.message) {
    return json({error: 'EMPTY_VALUE', message: "You forgot to supply a message!", field: 'message'}, 400);
  }

  email.append('from', `${incoming.name} <${incoming.email}>`);
  email.append('to', 'spam@intrinsic.com');
  email.append('subject', "Contact form email");
  email.append('text', incoming.message);

  // URL and API Key are samples from the Mailgun docs
  // This URL is _only_ a demo for parsing multipart/form-data fields
  // This will _not_ send a real email
  try {
    await fetch('https://api.mailgun.net/v3/samples.mailgun.org/messages', {
      method: 'POST',
      headers: new Headers({
        Authorization: 'Basic ' + btoa('api:key-3ax6xnjp29jd6fds4gc373sgvjxteol0')
      }),
      body: email
    });
  } catch (e) {
    return json({error: 'CANNOT_SEND', message: "Cannot parse provided JSON"}, 500);
  }

  return json({success: true, message: "Email has been sent"});
}

function json(obj, status = 200) {
  const headers = new Headers({
    'Content-Type': 'application/json'
  });

  const body = JSON.stringify(obj);

  const response = new Response(body, { headers, status });

  return response;
}
