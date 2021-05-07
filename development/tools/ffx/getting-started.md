# Getting started with ffx

This doc will guide you through some of the features of `ffx`. For an overview
of the design and components of `ffx`, see [the ffx overview](/docs/development/tools/ffx/overview.md).

Warning: **`ffx` is currently in alpha. Its APIs, command-line surface, and
documentation are subject to change.**


## Contacting the ffx team

If you discover possible bugs or have questions or suggestions,
[file a bug](https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=ffx+User+Bug).

## Prerequisites

To follow the examples in this doc, you'll need a Fuchsia device running. If you
don't have a physical device connected, you can use an emulator with networking
enabled (`-N`).

Tip: To start a headless emulator, run `fx emu --headless --software-gpu -N`.

Your device must be running a `core`
[product configuration](/docs/concepts/build_system/boards_and_products.md)
or a product configuration that extends `core` (such as `workstation`).

Optionally, you can run `fx log`, which will provide some additional information
about the interactions between `ffx` and your Fuchsia target device.

## Introduction

After following all the prerequisites, run the following in a terminal:

```sh
fx ffx help
```

This will list all of the available `ffx` subcommands. You'll see something
like:

```
Usage: ffx [-c <config>] [-e <env>] [-t <target>] [<command>] [<args>]

Fuchsia's developer tool

Options:
  -c, --config      override default configuration
  -e, --env         override default environment settings
  -t, --target      apply operations across single or multiple targets
  --help            display usage information

Commands:
  component         Discover and manage components
  config            View and switch default and user configurations
  daemon            Interact with/control the ffx daemon
  diagnostic        Run diagnostic tests on Fuchsia targets
  docs              View suite of docs for ffx and for Fuchsia
  doctor            Run common checks for the ffx tool and host environment
  emulator          Start and manage Fuchsia emulators
  overnet           Interact with the Overnet mesh
  package           Create and publish Fuchsia packages
  sdk               Modify or query the installed SDKs
  target            Interact with a target device or emulator
  vendor            Run partner plugins
  version           Print out ffx tool and daemon versions
```

You can use `fx ffx help <subcommand>` or `fx ffx <subcommand> --help` to see
more about any subcommand.

## Interacting with target devices

In a terminal, run the following:

```sh
fx ffx target list
```

You'll see a list of devices that `ffx` has discovered. For example, with a
single emulator running, output looks like:

```
NAME                    TYPE       STATE      ADDRS/IP                       AGE     CS
fuchsia-5254-0063-5e7a  Unknown    Unknown    [fe80::5054:ff:fe63:5e7a%4]    0m0s    N
```

NOTE: Ignore the `TYPE` and `STATE` columns - they have no values besides
`UNKNOWN` right now.

A couple of columns are worth explanation:

- `AGE`: This is the time since `ffx` was last able to reach the device.
- `RCS`: Indicates whether there is a reachable instance of the
  Remote Control Service (RCS) running on the device.

In order to get `ffx` to automatically connect to a device, you must either have
set the target's nodename to be the default target, or attempt to interact with the
device.

To set the target to be the default, run:

```sh
fx ffx target default set $NODENAME
```

If the default target has been set prior to starting the daemon, waiting a few seconds
should yield a change to the `RCS` status to show `Y`.

If the default target has been set after starting the daemon, attempting to interact
with the target should be sufficient to kick off a connection, like the following

```sh
fx ffx component list
```

NOTE: if the default target has been set, and you are unable to run that command
against the target, [reach out](#contacting_the_ffx_team) to the `ffx` team.

Then the next time you list targets you should see that an `RCS` connection
isn't active.

```sh
$ fx ffx target list
NAME                    TYPE       STATE      ADDRS/IP                       AGE     RCS
fuchsia-5254-0063-5e7a  Unknown    Unknown    [fe80::5054:ff:fe63:5e7a%4]    0m6s    Y
```

If a target has been set as default there will be a `*` next to it.

If you had `fx log` running, you should also see something like the following in
the logs:

```
[00009.776170][28540][28542][remote-control, remote_control_bin] INFO: published remote control service to overnet
```

NOTE: if the `RCS` column remains `N` for an extended amount of time and you have
already set this target's nodename to `target.default` _before_ initially starting
ffx, [reach out](#contacting_the_ffx_team) to the `ffx` team.

#### On Default Targets

Above we covered setting the default target using the command

```sh
fx ffx target default set
```

It is also possible to set the default target on a per-command basis using the
`--target` flag like so.

```sh
fx ffx --target $NODENAME component list
```

### Interacting with multiple devices
TODO: fill this out.

### Controlling the state of target devices

You can use the `target off` and `target reboot` subcommands to power-off or
reboot a device, respectively.

## Configuration

See documentation for the [config command](/docs/development/tools/ffx/commands/config.md).

## Interacting with Components

### Selectors

Many `ffx` commands that use components take selectors as a parameter. You can read more about selectors and their syntax
in [component selector documentation](/docs/development/tools/ffx/commands/component-select.md).

### Inspecting the component topology

You can use the `component select` command to
* inspect services in the
[component topology](/docs/concepts/components/v2/topology.md)
* search for components that expose a service.

For example, the following command will display all services offered by
[v1 components](/docs/glossary.md#components-v1):

```sh
$ fx ffx component select moniker 'core/appmgr:out:*'`

core/appmgr
|
--out
   |
   --chromium.cast.ApplicationConfigManager
   --fuchsia.bluetooth.avrcp.PeerManager
   --fuchsia.bluetooth.avrcp.test.PeerManagerExt
   --fuchsia.bluetooth.bredr.Profile
   --fuchsia.bluetooth.control.Control
   --fuchsia.bluetooth.gatt.Server
   --fuchsia.bluetooth.le.Central
   --fuchsia.bluetooth.le.Peripheral
   --fuchsia.bluetooth.snoop.Snoop
   --fuchsia.bluetooth.sys.Access
   --fuchsia.bluetooth.sys.HostWatcher
   --fuchsia.boot.Arguments
   --fuchsia.boot.FactoryItems
   --fuchsia.boot.Items
   --fuchsia.boot.ReadOnlyLog
   --fuchsia.boot.RootJobForInspect
   --fuchsia.boot.RootResource
   [truncated]
```

Note: this command can be slow (~10-15s), especially for selectors that match a
large number of services.

The following command will display all components that expose `diagnostics`:

```sh
$ fx ffx component select capability diagnostics

./bootstrap/archivist
./bootstrap/driver_manager
./bootstrap/fshost
./bootstrap/power_manager
./core/appmgr
./core/detect
./core/last_reboot
./core/log-stats
./core/pkg-cache
./core/sampler
./core/system-update-committer
```

### Verifying a service is up

You can use the `component knock` command to verify that a service starts
successfully: `knock` will open a channel to the service and return success if
and only if the channel isn't closed.

The component framework will start the component that provides the service
on-demand.

Note: the selector you pass to `knock` may contain a wildcard but must match
_exactly one_ service. You cannot `knock` on multiple services at once.

For example:

```
$ fx ffx component knock 'core/appmgr:out:fuchsia.hwinfo.P*'
Success: service is up. Connected to 'core/appmgr:out:fuchsia.hwinfo.Product'.

$ fx ffx component knock 'core/appmgr:out:not.a.real.service'
Failed to connect to service: NoMatchingServices
```

### Running a component

`ffx` can run components on a device given their package URL and arguments.
`stdout` and `stderr` will be streamed to the corresponding descriptor on the
host terminal.

Only v1 components can be `run`: v2 components are only started on-demand by
the framework. Learn more about the component lifecycle
[here](/docs/concepts/components/v2/lifecycle.md).

Note: `fx serve ` must be running in order to `component run` a package that is
not
[in base or cached](/docs/concepts/build_system/boards_and_products.md#dependency_sets).

Here's an example of running the Rust hello-world component. First, you'll need
the hello-world package in your universe:

```
$ fx set <product>.<board> --with //examples/hello_world/rust:hello-world-rust && fx build
...
$ fx ffx component run fuchsia-pkg://fuchsia.com/hello-world-rust#meta/hello-world-rust.cmx
Hello, world!
```


## Resolving connectivity issues

If you're experiencing problems communicating with a target device using `ffx`,
you can use the `doctor` command to diagnose and attempt to resolve them. If you
file a bug that involves a target device, we'll typically ask for the output
from `ffx doctor` to provide information about where the problem is.

`doctor` will attempt to communicate with the ffx daemon, killing
and restarting it if needed. If this is successful, it will attempt to SSH into
a target device and start the Remote Control Service.

If you try running `ffx doctor` under normal circumstances, you should see:

```
$ fx ffx doctor
Checking for a running daemon...none running.
Attempting to kill any zombie daemons...killed at least one daemon.
Starting a new daemon instance...success
Attempting to connect to the daemon. This may take a couple seconds...success
Attempting to communicate with the daemon...success
Attempting to list targets...success
Attempting to get an RCS connection...success
Attempting to communicate with RCS...success


SUCCESS. You should be able to run ffx commands now.
```

If `doctor` fails, it will try to suggest a resolution to the problem. It will
also provide a link to the Monorail component in which you can file a bug if you
persistently have problems. For example, if `doctor` is unable to start the RCS,
you would see the following:

```
$ fx ffx doctor
Checking for a running daemon...found
Attempting to connect to the daemon. This may take a couple seconds...success
Attempting to communicate with the daemon...success
Attempting to list targets...success
Attempting to get an RCS connection...success
Attempting to communicate with RCS...FAILED. Timed out.


Attempt 2 of 3
Attempting to list targets...success
Attempting to get an RCS connection...success
Attempting to communicate with RCS...FAILED. Timed out.


Attempt 3 of 3
Attempting to list targets...success
Attempting to get an RCS connection...success
Attempting to communicate with RCS...FAILED. Timed out.


Connecting to RCS failed after maximum attempts. To resolve this issue, try
rebooting your device. If this persists, please file a bug at the link below
and include 1) all output
above and 2) device syslog if available.Bug link: ...
```

## Next steps

- Please provide feedback on this doc by [reaching out to the ffx team](#contacting_the_ffx_team)!
- Learn how to [extend `ffx`](/docs/development/tools/ffx/development/plugins.md).
