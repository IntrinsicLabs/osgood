Osgood can be built from source by cloning the repository and running the following commands.

## Install Rust

Osgood is built using Rust. Either run the command below or visit
the [Install Rust](https://www.rust-lang.org/tools/install) page for more
details.

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

## Compile

You can compile the project by running `cargo build`. This command compiles V8,
which will likely take ~10-30 minutes on your machine. If you'd like to see
V8's progress, run `cargo build -vv` instead.

To avoid unnecessary work, the build script will leverage any existing
`depot_tools` package the machine already has, if the script can find the
necessary commands in your `PATH`.

The build script can optionally use a custom or precompiled version of V8. To
tell the script to do this, set the `CUSTOM_V8` environment variable to the
path to the V8 folder.

### Ubuntu Users

There are a few packages you'll need before you'll be able to compile:

```sh
sudo apt install libc++-dev clang libclang-dev libssl-dev
```

### Arch Linux Users

Google's `depot_tools` project has known issues when running on Arch Linux due
to assumptions the project makes about the version of Python referenced by the
`python` binary. Fixing this requires manual intervention; consider installing
`depot-tools-git` from the AUR.
