import FormData from 'internal:form_data.js';
import Headers from 'internal:headers.js';
import Response from 'internal:response.js';
import Request from 'internal:request.js';
import { atob, btoa } from 'internal:base64.js';
import { setInterval, setTimeout, clearTimeout } from 'internal:timers.js';
import fetch from 'internal:fetch.js';
import 'internal:inbound.js';
import 'internal:console.js';

delete self._bindings;

Object.assign(self, {
  FormData,
  Headers,
  Response,
  Request,
  atob,
  btoa,
  setInterval,
  setTimeout,
  clearInterval: clearTimeout,
  clearTimeout,
  fetch,
});
