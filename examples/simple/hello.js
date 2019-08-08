console.log('curl http://localhost:8080/hello');

export default (request, context) => {
  console.log('REQUEST', request);
  console.log('CONTEXT', context);

  return "Hello, World!";
};
