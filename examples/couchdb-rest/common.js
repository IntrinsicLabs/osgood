const AUTH = `Basic ${btoa('osgood_admin:hunter12')}`;

export function json(obj, status = 200) {
  const headers = new Headers({
    'Content-Type': 'application/json'
  });

  const body = JSON.stringify(obj);

  const response = new Response(body, { headers, status });

  return response;
}

// Makes a request to CouchDB
export function dbRequest(method = 'GET', path = '', body = '') {
  const options = {
    method,
    headers: {
      Authorization: AUTH
    }
  }

  if (body) {
    options.headers['Content-Type'] = 'application/json';
    options.body = JSON.stringify(body);
  }

  return fetch(`http://localhost:5984/users/${path}`, options);
}
