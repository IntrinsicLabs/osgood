const {
  setTimerHandler,
  setTimeout: _setTimeout,
  setInterval: _setInterval,
  clearTimer
} = self._bindings;

let timerIdCounter = 0;
const timerMap = new Map();
let timerNestingLevel = 0;

function handleTimer(timerId) {
  timerMap.get(timerId)();
}
setTimerHandler(handleTimer);


function normalizeTimeout(timeout) {
  timeout = Number(timeout);

  return (timeout >= 0 ? timeout : 0);
}

// Implementation is based on the following specification:
// https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#timers
// A few adjustments/assumptions were made:
// - The global scope will behave like a `WorkerGlobalScope`
// - The method `HostEnsureCanCompileStrings` will throw an exception
// - Since the WHATWG spec appears to be ambiguous about valid types for
//   `timeout`, it will first be cast to an ECMAScript Number using the
//   Number constructor, and then values of `NaN` will be treated as `0`
//
// TODO(perf): We can avoid making this function megamorphic by performing
// typechecking in both `setInterval` and `setTimeout`, which is probably
// worth doing if this is a hot path
function setTimer(id, handler, timeout, nestingLevel, args, repeating) {
  timeout = normalizeTimeout(timeout);

  // Throttle timeout values
  if (nestingLevel > 5 && timeout < 4) {
    timeout = 4;
  }

  // Handler can be any type, but we don't currently support string
  // compilation, and all other non-function types will get casted to
  // strings anyway
  if (typeof handler !== 'function') {
    throw new Error('Dynamic string compilation is currently unsupported');
  }

  timerMap.set(id, () => {
    timerNestingLevel = nestingLevel + 1;
    try {
      handler.apply(null, args);
    } catch (err) {
      console.error(err && typeof err === 'object' && err.stack ? err.stack : String(err));
    }

    if (repeating) {
      setTimer(id, handler, timeout, timerNestingLevel, args, repeating);
    }

    timerNestingLevel = nestingLevel;
  });

  if (repeating && nestingLevel > 5) {
    // Micro-optimization to switch to native tokio interval handler after backoff
    repeating = false;
    _setInterval(id, timeout);
  } else {
    _setTimeout(id, timeout);
  }
}

export function setInterval(handler, timeout, ...args) {
  const id = timerIdCounter++;
  setTimer(id, handler, timeout, timerNestingLevel, args, true);
  return id;
}

export function setTimeout(handler, timeout, ...args) {
  const id = timerIdCounter++;
  setTimer(id, handler, timeout, timerNestingLevel, args, false);
  return id;
}

export function clearTimeout(id) {
  timerMap.delete(id);
  clearTimer(id);
}
