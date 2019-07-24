export default async function main() {
  const form = new FormData();

  // This test mimics the Mailgun API
  form.append('from', 'Intrinsic <hello@intrinsic.com>');
  form.append('to', 'spam@intrinsic.com');
  form.append('subject', 'Osgood Hello World');
  form.append('text', 'Hello from Osgood!');

  console.log(form);

  const result = await fetch('http://localhost:9001/form-echo', {
    method: 'POST',
    headers: new Headers({
      Authorization: 'Basic ' + btoa('api:key-00000000000000000000000000000000')
    }),
    body: form
  });

  const json = await result.json();

  console.log(json);

  return json;
}
