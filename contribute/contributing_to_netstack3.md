# Contributing to Netstack3

Netstack3 is a networking stack being developed from scratch in Rust. It is
under active development, and we encourage everyone to contribute! Most of
Netstack3 - roughly 90% by lines of code - is platform-agnostic, meaning you can
build and test on your development machine without needing to build all of
Fuchsia or run it in a virtual machine. You can just use your normal `cargo`
development workflow (`cargo check`, `cargo test`, etc).

<!-- TODO(https://fxbug.dev/70286): Make shortlinks for these Monorail search
URLs -->

You can find a list of good bugs to get started with in our [good first bugs
list][good-first-bugs], and a list of good bugs to tackle next in our [good
second bugs list][good-second-bugs].

This document is designed to get you started with the basics of contributing to
Netstack3. It will get you from zero to building, testing, and contributing to
Netstack3's platform-agnostic core.

If you're interested in contributing and would like to chat with the Netstack3
developers, don't hesitate to reach out! We hang out on the
<connectivity-dev@fuchsia.dev> mailing list.

## Getting set up

* [Get the source code](/docs/get-started/get_fuchsia_source.md)
* [Configure and build Fuchsia](/docs/get-started/build_fuchsia.md)
  * Your `fx set` command will need to enable Netstack3 development using Cargo
    by including the flags `--with //src/connectivity/network/netstack3:bin
    --cargo-toml-gen`. Assuming you're developing on an x86 machine, use `fx set
    core.x64 --with //src/connectivity/network/netstack3:bin --cargo-toml-gen`.
* The source code for Netstack3's platform-agnostic core lives in
  `src/connectivity/network/netstack3/core`.
* Build the Netstack3 core by running `fx build
  src/connectivity/network/netstack3/core:netstack3-core`.
* Generate a `Cargo.toml` file to enable development with `cargo` by running `fx
  build build/rust:cargo_toml_gen` followed by `fx gen-cargo
  //src/connectivity/network/netstack3/core:netstack3-core`.
* Add the following lines to your `~/.cargo/config` file, replacing the absolute
  path to your Fuchsia directory and replacing the target if your local
  development target is different than `x86_64-unknown-linux-gnu`:

  ```toml
  [target.x86_64-unknown-linux-gnu]
  rustflags = ["-L", "absolute_path_to_fuchsia_directory/out/default/host_x64/obj/third_party/boringssl"]
  ```

  If you don't have a config file, you can create one with just these contents.
  If you're not sure what your development target is, run `rustup show`. These
  config lines instruct `cargo` where to find the build artifacts for BoringSSL,
  which we use for some cryptographic operations.
* Run a `cargo` command like `cargo check` or `cargo test` to see it in action!

From now on, you will mostly be able to just use `cargo` for development.
Sometimes, major changes to Netstack3's dependencies may cause your build setup
to break. See the [Troubleshooting](#Troubleshooting) section for advice on
fixing your build if this happens.

### Troubleshooting

* If you can't run `cargo` correctly after pulling the latest changes with `jiri
  update`, try the following:
  * Remove the `Cargo.lock` file, run `cargo clean`, and try again
  * If `cargo` still doesn't work correctly, try a full clean build:
    * `fx clean` - this cleans out the build output, and prepares you to do a
      full clean build
    * Run the same `fx set` command you ran in the beginning (probably `fx set
      core.x64 --with //src/connectivity/network/netstack3:bin
      --cargo-toml-gen`).
    * `fx build src/connectivity/network/netstack3/core:netstack3-core`
    * `fx build build/rust:cargo_toml_gen`

## Contributing changes

Interested in contributing to Netstack3? We'd love to have you! If you're not
sure where to start, try our [good first bugs list][good-first-bugs] or our
[good second bugs list][good-second-bugs]. If you're still a bit lost, don't
hesitate to reach out to us at <connectivity-dev@fuchsia.dev>.

If you have ideas for changes which aren't already tracked on our issue tracker,
that's great! Bug fixes and documentation improvements are especially welcome.
Just reach out to us first to make sure we're on the same page so you don't
spend time doing work that we won't end up accepting.

 Once you're ready to contribute, just follow these simple steps:

* Read the general instructions for [Contributing to Fuchsia](/CONTRIBUTING.md)
* Read our [Fuchsia Networking Contributor Guide](/src/connectivity/network/CONTRIBUTING.md)

## Understanding

If you're interested in diving deeper to understand Netstack3's design and
architecture, check out [our docs](/src/connectivity/network/netstack3/docs).

[good-first-bugs]: https://bugs.fuchsia.dev/p/fuchsia/issues/list?q=%28component%3Anetworkstack%3Enetstack3+OR+component%3AConnectivity%3ELibraries%3Enet_types+OR+component%3AConnectivity%3ELibraries%3Einternet_checksum+OR+component%3AConnectivity%3ELibraries%3Epacket%29+label%3AGoodFirstBug&can=2
[good-second-bugs]: https://bugs.fuchsia.dev/p/fuchsia/issues/list?q=%28component%3Anetworkstack%3Enetstack3+OR+component%3AConnectivity%3ELibraries%3Enet_types+OR+component%3AConnectivity%3ELibraries%3Einternet_checksum+OR+component%3AConnectivity%3ELibraries%3Epacket%29+label%3AGoodSecondBug&can=2
