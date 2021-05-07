# Logging in Rust

Rust programs on Fuchsia generally use the [log crate] and its `info!/warn!/error!` macros. The
[`fuchsia_syslog`] crate is used for initialization.

See the [language agnostic logging docs](/docs/concepts/diagnostics/logs/README.md) for more information
about recording and viewing logs.

## Requirements

### GN dependencies

The necessary crates can be included with two additions to `deps` in `BUILD.gn`:

```gn
deps = [
  "//src/lib/syslog/rust:syslog",   # for initialization
  "//third_party/rust_crates:log",  # for recording messages
]
```

See [Rust: Overview][rust-dev] for more information about building Rust within Fuchsia.

### Component manifest dependency {#manifest}

Ensure that your component has the required capabilities to log by including the
following in your component manifest:

   * {.cmx}

   ```json
   {
     "include": [
       "sdk/lib/diagnostics/syslog/client.shard.cmx"
     ],
     ...
   }
   ```

   * {.cml}

   ```json5
   {
     include: [
       "sdk/lib/diagnostics/syslog/client.shard.cml"
     ],
     ...
   }
   ```

Note: The above is only available for in-tree development.
This is tracked in [fxbug.dev/64207](http://fxbug.dev/64207).
Out of tree developers should copy the snippets shown below instead.

   * {.cmx}

   ```json
   {
     "sandbox": {
       "services": [
         "fuchsia.logger.LogSink"
       ]
     },
     ...
   }
   ```

   * {.cml}

   ```json5
   {
     use: [
       { protocol: "fuchsia.logger.LogSink" },
     ],
   }
   ```

The syslog library will fallback to `stderr` if the `LogSink` connection fails.

## Initialization

The Rust `log` crate must be connected to the Fuchsia backend before it can emit messages. Failure
to initialize the library will result in dropped messages.

### Basic

In your component's `main.rs`:

```rust
use fuchsia_syslog as syslog;

fn main() {
    // configures a single tag with the process name
    syslog::init().unwrap();
}
```

### With tags

By default the process name is used as the tag for all log messages. Tags can be used to further
categorize log messages from the program, and they can also be globally overridden.

```rust
use fuchsia_syslog as syslog;

fn main() {
    // overrides the use of process name for tag
    syslog::init_with_tags(&["my_tags"]).unwrap();

    // additional tag for just this message
    syslog::fx_log_info!(tag: "init", "an update on program initialization");
}
```

### Configure severity

The syslog crate starts at `INFO` severity but can be overridden.

```rust
use fuchsia_syslog as syslog;

fn main() {
    syslog::init().unwrap();

    // suppress INFO and below
    syslog::set_severity(syslog::levels::WARN);
}
```

## Recording messages

Most uses of logging are with the `log` crate's macros:

```rust
trace!("something happened: {}", 5); // maps to TRACE
debug!("something happened: {}", 4); // maps to DEBUG
info!("something happened: {}", 3);  // maps to INFO
warn!("something happened: {}", 2);  // maps to WARN
error!("something happened: {}", 1); // maps to ERROR
```

The `fuchsia_syslog` crate also offers macros like `fx_log_info!`, which allow manually specifying
the tag of a message.

## Standard streams

`println!`, `eprintln!` etc. go to standard out (`stdout`) and standard error (`stderr`).

See [`stdout` & `stderr`] in the language-agnostic logging docs for details on the routing of stdio
streams in the system.

[log crate]: https://fuchsia-docs.firebaseapp.com/rust/log/
[`fuchsia_syslog`]: https://fuchsia-docs.firebaseapp.com/rust/fuchsia_syslog/
[initialized in main]: /docs/development/languages/rust/add-logging.md
[rust-dev]: /docs/development/languages/rust/README.md
[`.cmx` file]: /docs/concepts/components/v1/component_manifests.md
[`stdout` & `stderr`]: /docs/development/diagnostics/logs/recording.md#stdout-stderr
