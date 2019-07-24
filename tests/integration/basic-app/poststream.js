export default ({ body }) =>
  fetch('http://localhost:9001/form-echo', {
    method: 'POST',
    headers: {
      'content-type': 'application/x-www-form-urlencoded'
    },
    body
  });
