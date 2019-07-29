The most basic usage of Osgood requires that you run the `osgood` binary and
pass in the path to an [Application File](Osgood-Application-File) as the only
argument.

```sh
$ osgood ./app.js
```

## Command Line Flags

Three basic flags are currently provided by Osgood:

```sh
$ osgood --help # displays help message
$ osgood --version # displays version number
$ osgood --v8-help # displays V8 flags
```

Additional flags can be passed to the underlying V8 engine. To get a list of
the possible flags first run the command with the `--v8-help` flag. The listed
flags can be passed in by prefixing them with `--v8-`. For example, the
`--max-old-space-size` flag can be passed in like so:

```sh
$ osgood --v8-max-old-space-size=10 ./app.js
```
