# CLI

The Command Line Interface (CLI) provides the UX for FFX. It is responsible for:

- Parsing user parameters (CLI Parameters)
- Communicating with the daemon (starting it if necessary)
- Routing parsed parameters and requested FIDL proxies to the proper code path
  for execution

## Parsing CLI Parameters

FFX uses the Rust crate [Argh](https://github.com/google/argh) to manage CLI
parameter definitions and parsing. As Argh complies with the Google standard,
code is able to be structured into decoupled plugins for FFX. Each subcommand
in the FFX call can be represented by a plugin, so for example:

```sh
$fx ffx component run "fuchsia-pkg://fuchsia.com/hello_world_rust#meta/hello_world_rust.cmx"
```

The part of the command 'component run' is a subcommand that routes the code
execution to the 'component run' plugin. More on this in the
[routing](#routing) section.

## Communicating with the daemon

FFX uses a daemon to run on the host to facilitate long running tasks. By
running these tasks in a daemon in the background, up-to-date data can be
supplied to the CLI as soon as it is requested instead of waiting for target
devices to respond.

When the CLI needs to communicate with the daemon, it first checks to see if
the process is running on the host. If the daemon is not running, the CLI
spawns the daemon process and waits for a connection. So it is possible to see
longer than average run times on the initial run of FFX.

## Routing

The CLI is designed as an extensible architecture. In this architecture,
plugins provide the code execution. Plugins are defined by a mixture of GN
build rules and Rust attributes. Plugins are designed to be as decoupled from
the internal workings of FFX as possible. If you are interested in developing a
plugin for FFX, please visit the [Integrating With FFX](/docs/development/tools/ffx/development/plugins.md) page.

