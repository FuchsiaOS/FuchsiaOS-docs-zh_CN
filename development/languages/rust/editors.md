# Rust Editor Configuration

[TOC]

## Intellij

See instructions on [the Intellij Rust site](https://intellij-rust.github.io/).
Run `fx gen-cargo //garnet/foo/path/to/target:label` for the GN target that you want to work on and
open the corresponding directory.

## VIM

See instructions on [`rust-lang/rust.vim`](https://github.com/rust-lang/rust.vim).

## Visual Studio Code

The VIM plugin uses the RLS (Rust language server) so you'll need to first
[install rustup](https://rustup.rs/) and run
`rustup component add rls-preview rust-analysis rust-src`. Once the RLS is installed,
install [this VSCode plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust).

Next open File -> Preferences -> Settings (or type Ctrl+Comma). Add the setting
`"rust.target": "x86_64-fuchsia"`.

Finally, run `fx gen-cargo //garnet/foo/path/to/target:label` for the GN target that you want to work on and
open the corresponding directory in VSCode.
