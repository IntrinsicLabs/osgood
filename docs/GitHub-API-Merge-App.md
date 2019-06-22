This application makes two outbound requests, transforms the data into a
simpler format, and responds with the combined request. This is often referred
to as the API Facade pattern.

## Application File `app.js`

This file only needs a single incoming route, `GET /merge/:username`. It will
make two outgoing HTTP requests and so we whitelist those requests.

```javascript
app.port = 3000;

app.route('GET', '/merge/:username', 'gh-merge.js', policy => {
  policy.outboundHttp.allowGet('https://api.github.com/users/*/gists');
  policy.outboundHttp.allowGet('https://api.github.com/users/*/repos');
});
```

## Command Line

You'll be able to run the application by doing the following:

```shell
$ osgood ./app.js
$ curl http://localhost/merge/tlhunter
```

The `:username` segment in the URL will be extracted and provided to the
application via `context.params.username`.

## Worker File `merge.js`

This application will make two requests to the GitHub API to get a list of
gists and a list of repos for the specified user. It will then sort the entries
by popularity and strip out a bunch of redundant data. Finally it will reply
with the subset of data.

```javascript
const MAX_LIST = 3;

export default async function (request, context) {
  const username = context.params.username;

  const [gists_req, repos_req] = await Promise.all([
    fetch(`https://api.github.com/users/${username}/gists`),
    fetch(`https://api.github.com/users/${username}/repos`),
  ]);

  const [gists, repos] = await Promise.all([
    gists_req.json(),
    repos_req.json(),
  ]);

  return transform(gists, repos);
}

function transform(gists, repos) {
  const owner = gists.length ? gists[0].owner : repos[0].owner;
  const payload = {
    user: {
      username: owner.login,
      avatar:   owner.avatar_url,
      url:      owner.html_url,
    },
    repos: [],
    gists: [],
  };

  repos.sort((a, b) => {
    if (a.watchers_count > b.watchers_count) return -1;
    else if (a.watchers_count < b.watchers_count) return 1;
  });

  gists.sort((a, b) => {
    if (a.comments > b.comments) return -1;
    else if (a.comments < b.comments) return 1;
  });

  let repo_count = 0;
  for (const repo of repos) {
    if (repo.disabled || repo.archived || repo.private) continue;
    if (++repo_count > MAX_LIST) break;
    payload.repos.push({
      name:     repo.full_name,
      url:      repo.html_url,
      desc:     repo.description,
      created:  repo.created_at,
      updated:  repo.updated_at,
      watchers: repo.watchers_count,
      forks:    repo.forks_count,
    });
  }

  let gist_count = 0;
  for (const gist of gists) {
    if (!gist.public) continue;
    if (++gist_count > MAX_LIST) break;
    payload.gists.push({
      url:      gist.html_url,
      desc:     gist.description,
      created:  gist.created_at,
      updated:  gist.updated_at,
      comments: gist.comments,
    });
  }

  return JSON.stringify(payload, null, 4);
}
```
