Last Refreshed: 2019-09-03

# Measuring Rust Binary Bloat

Instructions for analyzing the binary size and composition of Rust programs on Fuchsia. The examples
use the `//src/diagnostics/archivist` package.

## Prerequisites

* [cargo-bloat](#cargo-bloat)
* [fargo](#fargo)
* [Run `fx build`](#run-a-build)
* [Create Cargo.toml](#create-cargotoml)

### cargo-bloat

These instructions cover the installation and use of [cargo-bloat][bloat], a tool for Rust projects
inspired by [Bloaty McBloatface][google-bloaty].

To install:

```
cargo install cargo-bloat --features regex-filter [--force]
```

If you've installed a previous version or would like to add regex filtering support to an existing
installation, you may need to add `--force`.

### fargo

This example uses cargo subcommands from crates.io, so you must [install fargo][fargo].

### Run `fx build`

Consult the [fx common tools documentation][fx-common-tools] for information about including the
correct binary target in your build args.

Next, run `fx build` within your Fuchsia source directory.

### Create Cargo.toml

Follow the instructions to [generate a `Cargo.toml` for your project][cargo-toml-gen].

## Build with fargo

`cargo-bloat` doesn't currently support passing an arbitrary manifest path, you'll need to `cd` to
the directory with the generated `Cargo.toml`:

```
cd $FUCHSIA_DIR/src/diagnostics/archivist
```

From that directory, ensure you can produce a release build with fargo:

```
fargo build --release
```

Run the release build from plain fargo first, as cargo-bloat will swallow any build errors.

## Measure size contributions with cargo-bloat

Once we're sure that fargo can produce a release binary for our target, run cargo-bloat:

```
fargo cargo bloat -- --release -n 5
Compiling ...
Analyzing .../src/../out/cargo_target/x86_64-fuchsia/release/archivist

 File  .text     Size              Crate Name
 1.7%   4.5%  38.0KiB              regex <regex::exec::ExecNoSync as regex::re_trait::Regular...
 1.0%   2.5%  21.4KiB       regex_syntax regex_syntax::ast::parse::ParserI<P>::parse_with_comments
 0.8%   2.1%  17.7KiB fuchsia_component? <fuchsia_component::server::ServiceFs<ServiceObjTy> as...
 0.4%   0.9%   8.0KiB              regex regex::re_unicode::Regex::shortest_match_at
 0.4%   0.9%   7.8KiB                std _ZN9libunwind10CFI_ParserINS_17LocalAddressSpaceEE17paI...
33.8%  89.0% 751.9KiB                    And 6152 smaller methods. Use -n N to show more.
38.0% 100.0% 844.8KiB                    .text section size, the file size is 2.2MiB
```

It's important to measure binary size in release, as it can be difficult to predict the impact of
changes on debug builds.

The `-n 5` arguments limit output to 5 lines. Run `cargo bloat --help` for all options, and see
below for several commonly used ones.

### Group functions by crate

Use the `--crates` flag to group bloat analysis by an estimate of the source crate:

```
fargo cargo bloat -- --release -n 5 --crates
Compiling ...
Analyzing .../src/../out/cargo_target/x86_64-fuchsia/release/archivist

 File  .text     Size Crate
13.3%  34.9% 294.8KiB std
 6.3%  16.7% 141.0KiB regex
 5.0%  13.2% 111.3KiB regex_syntax
 2.1%   5.5%  46.1KiB fidl
 1.8%   4.7%  39.5KiB json5
 9.6%  25.1% 212.2KiB And 64 more crates. Use -n N to show more.
38.0% 100.0% 844.8KiB .text section size, the file size is 2.2MiB

Note: numbers above are a result of guesswork. They are not 100% correct and never will be.
```

Attributing generic functions to their originating crate is an error-prone heuristic analysis and
you should drill down with filters and granular output to confirm any discoveries from crate-grouped
output.

### Filter functions

To drill down into the sources of bloat in a particular crate, you can filter by source crate name
or regex over the function name (with the feature flag enabled) with the `--filter` flag:

```
fargo cargo bloat -- --release -n 5 --filter regex_syntax
Compiling ...
Analyzing .../src/../out/cargo_target/x86_64-fuchsia/release/archivist

File .text     Size        Crate Name
1.0%  2.5%  21.4KiB regex_syntax regex_syntax::ast::parse::ParserI<P>::parse_with_comments
0.3%  0.8%   6.4KiB regex_syntax regex_syntax::ast::parse::ParserI<P>::parse_escape
0.2%  0.5%   3.8KiB regex_syntax <regex_syntax::hir::translate::TranslatorI as regex_syntax::ast...
0.1%  0.2%   1.8KiB regex_syntax <regex_syntax::hir::translate::TranslatorI as regex_syntax::ast...
0.1%  0.2%   1.7KiB regex_syntax regex_syntax::unicode::class
3.4%  9.0%  76.2KiB              And 698 smaller methods. Use -n N to show more.
5.0% 13.2% 111.2KiB              filtered data size, the file size is 2.2MiB
```


[bloat]: https://github.com/RazrFalcon/cargo-bloat
[fargo]: https://fuchsia.googlesource.com/fargo/#getting-started
[fx-common-tools]: /development/build/fx.md#common-daily-tools
[cargo-toml-gen]: /development/languages/rust/cargo.md#cargo-toml-gen
[google-bloaty]: https://github.com/google/bloaty
