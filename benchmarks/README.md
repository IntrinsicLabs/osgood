# Benchmarks

When performing benchmarks it's important to make sure the conditions for
running two separate servers are the same. One of these considerations is that
the data being transmitted should be exactly equal. This requires checking the
headers, for example by using `curl -i` or `httpie`.


## Included Services

We've included a `Hello, World!` application for both Osgood, as well as
Node.js. Both files make use of a shebang and can be run directory (assuming
`node` and `osgood` are available in your path).

Both applications will listen on port `3000` and will respond to a request made
to `/hello`.


## Benchmark Command

Here's the command we've been using while developing Osgood:

```sh
siege -c 10 -r 100000 -b http://localhost:3000/hello
```

This command will terminate after a while and provide some results. It's
important to test Osgood in this manner to prevent performance regressions from
being released.
