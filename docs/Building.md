Osgood can be built from source by cloning the repository and running the
following commands. This will require that you first install Rust and Node.js
on your machine. Note that these are _only_ required for building; they are
unnecessary for _running_ Osgood.

## Install Rust

Osgood is built using Rust. Either run the command below or visit
the [Install Rust](https://www.rust-lang.org/tools/install) page for more
details.

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

## Install Node.js and npm

Osgood rebuilds some familiar JavaScript APIs which are commonly provided in
browser environments but aren't part of the core JavaScript language itself.
Some of these APIs we've built ourselves. Others are provided via an npm
package.

Osgood therefore requires that such packages be downloaded during the build
process. Once all necessary packages are present they're essentially
concatenated together (via Webpack) and the resulting file is distributed with
Osgood.

Run the following command to get a modern version of Node.js and npm:

```shell
$ curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.34.0/install.sh | bash
```

## Compile

Run the following command to build Osgood:

```shell
$ cargo build
```

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
sudo apt install build-essential pkg-config libc++-dev libc++abi-dev \
  clang libclang-dev libssl-dev
```

### Arch Linux Users

Google's `depot_tools` project has known issues when running on Arch Linux due
to assumptions the project makes about the version of Python referenced by the
`python` binary. Fixing this requires manual intervention; consider installing
`depot-tools-git` from the AUR.
