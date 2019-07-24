export default (_, context) => {
  const type = context.params.type;
  const url = 'http://localhost:9001/';
  // Note: not doing multipart since that's taken care of in multipart.js
  switch (type) {
    case 'stringUrl':
      return fetch(url);
    case 'bodyString':
      return fetch(new Request(url + 'echo', {
        method: 'POST',
        body: 'this is a bodyString test'
      }));
    case 'stream':
      return fetch(new Request(url + 'echo', {
        method: 'POST',
        body: new ReadableStream({
          start(controller) {
            controller.enqueue('this is a stream test');
            controller.close();
          }
        })
      }));
  }
}
