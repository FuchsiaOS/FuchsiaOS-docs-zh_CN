# FFX Config

FFX's configuration management allows you to configure your FFX code without
recompiling. From a high level, configurations can be thought of as key-value
map.

The configuration management code searches in several locations for the value
associated with a key. The search is performed in the following order
and returns the first value found:

1. [Runtime configuration (set by the command line)](#runtime-configuration)
2. [Environment variables](#environment-variables)
3. [Configuration files](#configuration-files)
4. [Heuristics (compiled methods that can guess from the environment)](#heuristics)

## Runtime Configuration

Configuration can be set at runtime when ffx is invoked. The top level command
accepts a configuration parameter:

```sh
$fx ffx --config "config=runtime" ...
```

For example, run:

```sh
$fx ffx --config "config-test=runtime" config get --name config-test
```

The runtime parameter takes the format of comma separated key-value pairs
("{key}={value},{key}={value},etc...") because this is not strongly typed, any
configurations set here will be assumed to be strings.

The daemon runs as it's own process and currently the runtime
configuration is not transferred from the CLI to the daemon if the daemon is
started up. It's expected that if you want to configure the the daemon using
the runtime configurations, the daemon command will be run manually:

```sh
$fx ffx --config "config-test=runtime" daemon
```

## Environment variables

Keys can be tied to environment variables as well. This is currently done at
compile time.

Keys can be tied to any number of environment variables and the first
environment variable that resolves to a value is used (in the order that they
are given via the vector parameter).

## Configuration Files

More to come.

## Heuristics

Heuristic configurations use code that is executed at the time of query to
resolve keys to values. This code is set at compile time.

Keys are associated with functions that must match the signature:

```rust
fn(key: &str) -> Option<Value>
```
