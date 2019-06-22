export default async () => {
  const result = await fetch('http://localhost:9001/form-echo', {
    method: 'POST',
    headers: {
      'content-type': 'application/x-www-form-urlencoded'
    },
    body: 'foo1=bar1&foo2=bar2&foo2=bar3'
  });

  return result.json();
}
