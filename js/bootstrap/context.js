const { _route, getPrivate } = self._bindings;
const urlSym = getPrivate('url');
const querySym = getPrivate('query');
const paramsSym = getPrivate('params');

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

class Context {
  constructor(url) {
    this[urlSym] = url;
  }

  get query() {
    return this[paramsSym] || (this[paramsSym] = new URL(this[urlSym]).searchParams);
  }

  get params() {
    return this[querySym] || (this[querySym] = parseParamsFromUrlPath(this[urlSym]));
  }

}

export function generateContextObject(url) {
  return new Context(url);
}
