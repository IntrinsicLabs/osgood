const $name = document.getElementById('name');
const $email = document.getElementById('email');
const $message = document.getElementById('message');
const $form = document.getElementById('form');
const $status = document.getElementById('status');

$form.addEventListener('submit', (event) => {
  event.preventDefault();

  (async () => {
    const payload = {
      name: $name.value,
      email: $email.value,
      message: $message.value,
    };

    try {
      var response = await fetch('/contact', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
      });
    } catch(e) {
      $status.innerHTML = `<span class="fail">Failed to send: ${e.message}</span>`;
      return;
    }

    const body = await response.json();

    if (body.error) {
      $status.innerHTML = `<span class="fail">Failed to send: ${body.message}</span>`;
      return;
    }
    $status.innerHTML = `<span class="success">success: ${body.message}</span>`;

  })();

  return false;
});

