export default async (request, context) => {
  await good_request_1();
  await good_request_2();
  await shouldError(bad_request_1, 'bad url good header "host"');
  await shouldError(bad_request_2, 'bad url good header "Host"');
  return 'all good';
};

async function shouldError(fn, label) {
  let didError = false;
  try {
    await fn();
  } catch(e) {
    didError = true;
  }

  if (!didError) throw new Error(label);
}

function good_request_1() {
  return fetch('http://localhost:9001/');
}

function good_request_2() {
  return fetch('http://localhost:9001/', { headers: new Headers({
    'host': 'localhost'
  })});
}

function bad_request_1() {
  return fetch('http://evil.intrinsic.org/', { headers: new Headers({
    'host': 'localhost'
  })});
}

function bad_request_2() {
  return fetch('http://localhost:9001/', { headers: new Headers({
    'Host': 'evil.intrinsic.org'
  })});
}
