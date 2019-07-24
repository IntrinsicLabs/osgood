'use strict';

export default async (request, context) => {
  let transform = new TransformStream();

  let stream = transform.writable;
  let writer = stream.getWriter();
  writer.write("Hello, world!\n");
  writer.close();

  return new Response(transform.readable, {
    headers: new Headers({
      "Content-Type": "text/plain"
    })
  });
};
