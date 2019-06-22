export default request => {
  console.log(request.method + ' ' + request.url);
  return fetch('https://localhost:3000');
};
