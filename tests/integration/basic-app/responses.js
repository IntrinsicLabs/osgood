function encode(str) {
  const encoder = new TextEncoder();
  return encoder.encode(str);
}

export default (_, context) => {
  // streaming response bodies are tested elsewhere.
  // non-Response-class responses are tested elsewhere.
  switch (context.params.type) {
    case 'string':
      return new Response('this is a string test');
    case 'typedArray':
      return new Response(encode('this is a typedarray test'));
    case 'arrayBuffer':
      return new Response(encode('this is an arraybuffer test'));
    case 'dataView':
      return new Response(
        new DataView(encode('this is a dataview test').buffer)
      );
    case 'objectHeader':
      return new Response('this is a string test', {
        status: 500,
        headers: {
          TestHeader: 'test header value'
        }
      });
    case 'classHeader':
      return new Response('this is a string test', {
        status: 500,
        headers: new Headers({
          TestHeader: 'test header value'
        })
      });
  }
}
