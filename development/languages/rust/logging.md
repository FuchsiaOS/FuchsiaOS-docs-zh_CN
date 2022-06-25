# Logging in Rust

This document explains how to get started with logging in Rust programs on
Fuchsia. For general information about recording and viewing logs, see the
[language-agnostic logging documentation][doc-logging].

## Required capabilities {#capabilities}

Ensure that your component requests the appropriate logging capabilities by
including the following in your component manifest:

   * {.cmx}

   ```json
   {
     "include": [
       "syslog/client.shard.cmx"
     ],
     ...
   }
   ```

   * {.cml}

   ```json5
   {
     include: [
       "syslog/client.shard.cml"
     ],
     ...
   }
   ```

## Initialization {#initialization}

You must initialize logging before you can [record logs](#record) from Rust code.
Initialization is handled by the [`fuchsia`][ref-fuchsia] crate setup macros.

### GN dependencies

Add the following `deps` to your `BUILD.gn` file:

```gn
deps = [
  "//src/lib/fuchsia",
]
```

### Setup

In your Rust source files, logging is enabled by default for any function
initialized using the `fuchsia::main` or `fuchsia::test` macros:

```rust
#[fuchsia::main]
fn main() {
    // ...
}

#[fuchsia::test]
fn example_test() {
    // ...
}
```

You can also pass the `logging` flag to make this explicit:

```rust
#[fuchsia::main(logging = true)]
fn main() {
    // ...
}

#[fuchsia::test(logging = true)]
fn example_test() {
    // ...
}
```

## Add tags

Log messages can include one or more tags to provide additional context.
To enable log tags for a given scope, pass the `logging_tags` parameter during
[initialization](#initialization):

```rust
#[fuchsia::main(logging_tags = ["foo", "bar"])]
fn main() {
    // ...
}

#[fuchsia::test(logging_tags = ["foo", "bar"])]
fn example_test_with_tags() {
    // ...
}
```

## Record logs {#record}

Rust programs on Fuchsia generally use the `tracing` crate macros to record
logs.

### GN dependencies

Add the `tracing` crate to the `deps` entry of your `BUILD.gn` file:

```gn
deps = [
  "//third_party/rust_crates:tracing",
]
```

### Log events

Call the macros provided by the `tracing` crate to record logs at the declared
severity level:

```rust
use tracing;

fn main() {
    tracing::trace!("something happened: {}", 5); // maps to TRACE
    tracing::debug!("something happened: {}", 4); // maps to DEBUG
    tracing::info!("something happened: {}", 3);  // maps to INFO
    tracing::warn!("something happened: {}", 2);  // maps to WARN
    tracing::error!("something happened: {}", 1); // maps to ERROR
}
```

## Standard streams

Rust macros such as `println!`, `eprintln!` etc. map to standard out (`stdout`)
and standard error (`stderr`). Using these streams may require additional setup
work for your program.

For more details, see the [standard streams][std-streams] section in the
language-agnostic logging documentation.

[doc-logging]: concepts/components/diagnostics/logs/README.md
[ref-fuchsia]: https://fuchsia-docs.firebaseapp.com/rust/fuchsia/
[rust-dev]: development/languages/rust/README.md
[std-streams]: development/diagnostics/logs/recording.md#stdout-stderr
