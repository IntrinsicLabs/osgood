const { _route } = self._bindings;

const REGEX_CAPTURE_GROUPS = /\:([a-zA-Z0-9_]+)/g;
const REPLACE_CAPTURE_GROUPS = '(?<$1>[^\\/]+)'; // named capture groups

const REGEX_DOUBLE_ASTERISK = /\*\*/g;
const REPLACE_DOUBLE_ASTERISK = '(.+)'; // unnamed capture group

const REGEX_SINGLE_ASTERISK = /\*/g;
const REPLACE_SINGLE_ASTERISK = '([^\\/]+)'; // unnamed capture group

function patternToRegExp(pattern) {
  const matcherString = pattern
    .replace(REGEX_CAPTURE_GROUPS, REPLACE_CAPTURE_GROUPS)
    .replace(REGEX_DOUBLE_ASTERISK, REPLACE_DOUBLE_ASTERISK)
    .replace(REGEX_SINGLE_ASTERISK, REPLACE_SINGLE_ASTERISK)
    .replace(/\//g, '\\\/');
  return new RegExp(`^${matcherString}$`);
}

const routeRegex = patternToRegExp(_route);

function parseParamsFromUrlPath(url) {
  const { pathname } = new URL(url);
  const {groups} = routeRegex.exec(pathname) || {};
  return groups;
}

export function generateContextObject(url) {
  let params;
  let query;
  return {
    get query() {
      return params || (params = new URL(url).searchParams);
    },
    get params() {
      return query || (query = parseParamsFromUrlPath(url));
    }
  };
}
