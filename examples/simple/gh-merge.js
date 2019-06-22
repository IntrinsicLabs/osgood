const MAX_LIST = 3;

export default async function main(request, context) {
  const username = context.params.username;

  const [gists_req, repos_req] = await Promise.all([
    fetch(`https://api.github.com/users/${username}/gists`),
    fetch(`https://api.github.com/users/${username}/repos`),
  ]);

  // TODO: currently osgood doesn't provide the `json` method
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

