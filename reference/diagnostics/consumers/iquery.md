# iquery

`iquery` - the Fuchsia Inspect API Query Toolkit

## Synopsis

```
iquery [--format <format>] <command> [<args>]
```

## Description

`iquery` is a utility program for inspecting component nodes exposed over the
[Inspect API]. It accepts a set of options and a command with
its respective options.

To prevent hard to debug issues in selectors where your shell is escaping some
character or others, it's recommended to always wrap selectors in single or
double quotes so your shell passes them as raw strings to iquery.

## Options

### `--format`

The format in which the output will be displayed.

Accepted formats:

- `text`: default, good for human reading
- `json`: good for machine reading

### `--help`

Prints usage information.

## Commands

### `list`

Lists all components (relative to the scope where the archivist receives events
from) of components that expose inspect.

For v1: this is the realm path plus the realm name.

For v2: this is the moniker without the instances ids.

Example usage:

```
$ iquery list
archivist.cmx
bootstrap/device_manager
...
```

#### `--manifest`

The name that you specified for the manifest file. If this is specified, the
output only contains monikers for components whose URL contains the specified
name.

#### `--with-url`

Prints both the moniker and the URL with which the component was launched.

#### `--help`

Prints usage information about `list`.

### `list-files [<paths...>]`

Lists all files that contain inspect data under the given `paths`. This will
only list files for v1 components given that v2 components are not mapped to the
filesystem at the moment.

The files that this command looks for are:

- `fuchsia.inspect.Tree`: A service file. The standard way inspect libraries
  export inspect data.
- `*.inspect`: VMO files with inspect data. The standard way the Dart inspect
  library exports inspect data.
- `fuchsia.inspect.deprecated.Inspect`: A service file. The standard way the Go
  library exports inspect data.

Example usage:

```
$ iquery list-files /hub /dev
/dev/diagnostics/driver_manager/dm.inspect
/hub/c/archivist.cmx/21352/out/diagnostics/fuchsia.inspect.Tree
/hub/c/archivist.cmx/21352/system_diagnostics/fuchsia.inspect.Tree
/hub/c/bt-gap.cmx/35231/out/diagnostics/bt-gap.inspect
/hub/c/bt-gap.cmx/35231/system_diagnostics/fuchsia.inspect.Tree
/hub/c/netstack.cmx/26786/out/diagnostics/counters/fuchsia.inspect.deprecated.Inspect
...
```

#### `--help`

Prints usage information about `list-files`.

### `selectors [<selectors...>]`

Lists all available full selectors (component selector + tree selector).

If a component selector is provided, it’ll only print selectors for that component.

If a full selector (component + tree) is provided, it lists all selectors under the given node.

Example usage:

```
$ iquery selectors 'archivist.cmx:root/fuchsia.inspect.Health' 'timekeeper.cmx'
archivist.cmx:root/fuchsia.inspect.Health:start_timestamp_nanos
archivist.cmx:root/fuchsia.inspect.Health:status
timekeeper.cmx:root/current:system_uptime_monotonic_nanos
timekeeper.cmx:root/current:utc_nanos
timekeeper.cmx:root:start_time_monotonic_nanos
```

#### `--manifest`

The name that you specified for the manifest file. If this is specified, the
output only contains monikers for components whose URL contains the specified
name.

#### `--help`

Prints usage information about `selectors`


### `show [<selectors...>]`

Prints the inspect hierarchies that match the given selectors.

Example usage:

```
$ iquery show 'archivist.cmx:root/fuchsia.inspect.Health' 'timekeeper.cmx'
archivist.cmx:
  root:
    fuchsia.inspect.Health:
      start_timestamp_nanos = 30305104656
      status = OK
timekeeper.cmx:
  root:
    start_time_monotonic_nanos = 30347000053
    current:
      system_uptime_monotonic_nanos = 61617527688648
      utc_nanos = 1591119246552989779
```

#### `--manifest`

The name that you specified for the manifest file. If this is specified, the
output only contains monikers for components whose URL contains the specified
name.

#### `--help

Prints usage information about `show`.

### `show-file [<paths...>]`

Given a path, prints the inspect data contained in files at the given paths. At the moment this
command only works for v1 components as we only have a v1 hub.

Example usage:

```
$ fx shell iquery show-file /dev/diagnostics/driver_manager/dm.inspect /hub/c/archivist.cmx/21352/out/diagnostics/fuchsia.inspect.Tree
/dev/diagnostics/driver_manager/dm.inspect:
  root:
    device_count = 126
    ...
/hub/c/archivist.cmx/21352/out/diagnostics/fuchsia.inspect.Tree:
  root:
    all_archive_accessor_node:
      archive_accessor_connections_closed = 15
  ...
```

#### `--help`

Prints usage information about `show-file`.

[Inspect API]: /development/diagnostics/inspect/README.md
