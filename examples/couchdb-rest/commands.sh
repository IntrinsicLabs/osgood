# require `jq`: https://stedolan.github.io/jq/

echo "\ncreate user"
curl -s -XPOST http://localhost:8000/users --data '{"username": "osgood"}' -H 'Content-Type: application/json' | jq "."

echo "\nlist users"
curl -s http://localhost:8000/users | jq "."

echo "\nget user"
curl -s http://localhost:8000/users/`curl -s http://localhost:8000/users | jq -r ".[0]"` | jq "."

echo "\nupdate user"
curl -s -XPUT http://localhost:8000/users/`curl -s http://localhost:8000/users | jq -r ".[0]"` -d '{"username": "osgood", "is_cool": true}' -H 'Content-Type: application/json' | jq "."

echo "\ndelete user"
curl -s -XDELETE http://localhost:8000/users/`curl -s http://localhost:8000/users | jq -r ".[0]"` | jq "."
