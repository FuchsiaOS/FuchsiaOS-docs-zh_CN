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

### `list-files [<monikers...>]`

Lists all files that contain inspect data under the given `paths`.

The files that this command looks for are:

- `fuchsia.inspect.Tree`: A service file. The standard way inspect libraries
  export inspect data.
- `*.inspect`: VMO files with inspect data. The standard way the Dart inspect
  library exports inspect data.
- `fuchsia.inspect.deprecated.Inspect`: A service file. The standard way the Go
  library exports inspect data.

Example usage:

```
$ iquery list-files bootstrap/archivist bootstrap/driver_manager
bootstrap/archivist
  fuchsia.inspect.Tree
bootstrap/driver_manager
  class/display-controller/000.inspect
  class/input-report/000.inspect
  class/input-report/001.inspect
  class/misc/000.inspect
  class/pci-root/000.inspect
  class/pci/000.inspect
  class/sysmem/481.inspect
  driver_manager/driver_host/10171/root.inspect
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

#### `--file`

The filename we are interested in. If this is provided, the output will only
contain data from components which expose Inspect under the given file under
their out/diagnostics directory.


#### `--help`

Prints usage information about `show`.


[Inspect API]: /docs/development/diagnostics/inspect/README.md
