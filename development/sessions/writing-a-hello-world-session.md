# Writing a "Hello World" session {#writing-a-hello-world-session}

Sessions are regular components that the `session_manager` can launch at
startup. This means that creating a session component follows all of the same
steps as creating any other component. This document discusses creating an
example session that launches at startup and prints "Hello World!" to the system
log.

## Create the directory structure {#create-the-directory-structure}

Components require a specific directory structure. The `fx` tool provides a
generator that creates this structure for you. It takes the name of the
component and the language you want to use as arguments. For example, this
component is called `hello-world-session` and is written in Rust.

Run the following command to create the directory structure for this example:

```posix-terminal
fx create component-v2 hello-world-session --lang rust
```

This command creates the following directory structure with a template for a
component offering a service:

```none
hello-world-session
  |- meta
  |   |- hello-world-session-unittests.cml
  |   |- hello-world-session.cml
  |
  |- src
  |   |- main.rs
  |
  |- BUILD.gn
```

## Create a component manifest {#create-a-component-manifest}

The component manifest file (`hello-world-session.cml` in this case) gives
Fuchsia information about our component. The component manifest file has to have
the same name as the component that it refers to. This session doesn't do much
so you don't need to add anything to the one that was generated.

The following lines of code are from `hello-world-session.cml`:

1. The file starts by including other cml files if needed.

   ```json5
   include: [ "sdk/lib/diagnostics/syslog/client.shard.cml" ],
   ```

   This `include` key lets the session component use the
   `fuchsia.logger.LogSink` capability so that it can print to the system log.

1. Next is the `program` block.

   ```json5
   program: {
       // Use the built-in ELF runner to run native binaries.
       runner: "elf",
       // The binary to run for this component.
       binary: "bin/hello_world_session",
   },
   ```

   The `program` block tells the `component_manager` where the binary for the
   session component can be found. The `runner` key tells the `component_manager`
   that is should run the component binary using the ELF runner.

1. Finally the component manifest describes capabilities that the component
   can `use`, `offer`, or `expose`.

   ```json5
   use: [
       // List your component's dependencies here, ex:
       // { protocol: "fuchsia.net.NameLookup" }
   ],
   ```

## Create a session config {#create-a-session-config}

`session_manager` needs to know to which session component to launch at startup.
To do this create a session config JSON file in the `meta` directory that
contains the URL of the session component.

Component URLs follow the format:

<pre><code>fuchsia-pkg://fuchsia.com/<var>package_name</var>#meta/<var>your_session.cm</var></code></pre>

Notice that the path points to a `.cm` file. `.cm` files are compiled versions
of `.cml` files that are generated when `fx build` is run. So, in this case, the
component URL is:

```none
fuchsia-pkg://fuchsia.com/hello-world-session#meta/hello-world-session.cm
```

The whole session config file looks like this:

```json
{
    "session_url": "fuchsia-pkg://fuchsia.com/hello-world-session#meta/hello-world-session.cm"
}
```

## Writing a session in Rust {#writing-a-session-in-rust}

Now you can write the actual code for the session component. Inside the
`src/main.rs` file that was generated there is a lot of code that isn't needed
for this example. Replace the contents of `src/main.rs` with the following code:

```rust
use {
    anyhow::{Context, Error},
    fuchsia_async as fasync,
    fuchsia_syslog::fx_log_info,
};

/// Creates a simple session that just prints "Hello World" to the syslog.
#[fasync::run_singlethreaded]
async fn main() -> Result<(), Error> {
    fuchsia_syslog::init_with_tags(&["hello_world_session"])
        .context("Failed to initialize logger.")?;

    fx_log_info!("Hello World!");

    Ok(())
}

#[cfg(test)]
mod tests {
    #[fuchsia::test]
    async fn smoke_test() {
        assert!(true);
    }
}
```

This code initializes the system log and then prints "Hello World!".
`fuchsia_syslog::init_with_tags` can cause an error so the main function returns
`Result<(), Error>` as well. `fx_log_info!` is a macro that prints to the
system log with a level of `info`. There are similar marcros for `error` and
`warn`.

## Writing the `BUILD.gn` {#writing-the-build-gn}

The last file to modify is the `BUILD.gn`. This tells the compiler how to build
the the session component.

### Imports {#imports}

The file starts by importing GN templates that are used in this `BUILD.gn`. To
build a session component you must import the `session_config.gni`.

```gn
import("//src/session/build/session_config.gni")
```

### Session config {#session-config}

The added import statement gives the `BUILD.gn` access to the `session_config`
command. This command tells the build where to find the `session_config.json`
for this component. Add the `session_config` to the `BUILD.gn` just below the
import statements:

```gn
session_config("hello-world-session-config.json") {
  config = "meta/hello-world-session-config.json"
}
```

### Rust binary {#rust-binary}

The next section describes the actual Rust binary. It tells the compiler what
the name of the binary should be, that it includes unit tests, what dependencies
it has, and where it's source is located. The only thing that needs to be
changed in this section are the dependencies. Since this session component
doesn't have much functionality there are things in the list that aren't needed.
But, also, `syslog` is missing so the session wouldn't have access to do any
logging. Replace the dependencies with the following code:

```gn
deps = [
  "//src/lib/fuchsia",
  "//src/lib/fuchsia-async",
  "//src/lib/syslog/rust:syslog",
  "//third_party/rust_crates:anyhow",
]
```

Everything else can stay as it is in the generated `BUILD.gn`. The
`fuchsia_component` and `fuchsia_package` commands tell Fuchsia more about the
component including what it is called, where to find the manifest, and what
dependencies the package and component have.

## Building the session {#building-the-session}

To build the session `fx set` must first be used to configure the build so that
`session_manager`, your session component, and the session config are included
in the base image. This is done with the `--with-base` command.

<pre class="prettyprint"><code class="devsite-terminal">fx set <var>product</var>.<var>board</var> --with-base=//src/session,<var>//path/to/your/session</var>,<var>//path/to/your/session:your_session_config</var></code></pre>

This project is in the session/examples directory so the `fx set` for it would
be:

```posix-terminal
fx set core.x64 --with-base=//src/session,//src/session/examples/hello-world-session,//src/session/examples/hello-world-session:hello-world-session-config.json
```

Once that's done and built `session_manager` should automatically start your
session on boot.
