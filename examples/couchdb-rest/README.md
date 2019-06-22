# setup

Run CouchDB:

```sh
docker run \
  -e COUCHDB_USER=osgood_admin \
  -e COUCHDB_PASSWORD=hunter12 \
  -p 5984:5984 \
  --name osgood-couch \
  -d couchdb
```

Create a database:

```sh
curl \
  -X PUT \
  http://osgood_admin:hunter12@localhost:5984/users
```

Generate POST request to create a document:

```sh
curl \
  -X POST \
  http://localhost:8000/users \
  -d '{"foo": "bar"}' \
  -H "Content-Type: application/json"
```

Or, use the `commands.sh` script to interact with the REST API. This file
requires that both `curl` and [`jq`](https://stedolan.github.io/jq/) are
installed. Both are highly useful CLI tools.
