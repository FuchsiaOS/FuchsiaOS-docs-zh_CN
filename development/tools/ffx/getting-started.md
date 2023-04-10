# Getting started with ffx

This doc will guide you through some of the features of `ffx`. For an overview
of the design and components of `ffx`, see [the ffx overview](/development/tools/ffx/overview.md).

## Contacting the ffx team

If you discover possible bugs or have questions or suggestions,
[file a bug](https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=ffx+User+Bug).

## Prerequisites

To follow the examples in this doc, you'll need a Fuchsia device running. If you
don't have a physical device connected, you can use an emulator.

To start an emulator with networking enabled but without graphical user
interface support, run `ffx emu start --headless`.

For more information on configuring the emulator
see, [Start the Fuchsia emulator](/get-started/set_up_femu.md).

Your device must be running a `core`
[product configuration](/development/build/build_system/boards_and_products.md)
or a product configuration that extends `core` (such as `workstation_eng`).

Optionally, you can run `ffx log`, which will provide some additional information
about the interactions between `ffx` and your Fuchsia target device.

## Introduction

After following all the prerequisites, run the following in a terminal:

```posix-terminal
fx ffx help
```

This will list all of the available `ffx` subcommands. You'll see something
like:

```none
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

```posix-terminal
fx ffx target list
```

You'll see a list of devices that `ffx` has discovered. For example, with a
single emulator running, output looks like:

```none
NAME                    SERIAL       TYPE       STATE      ADDRS/IP                       RCS
fuchsia-emulator  <unknown>    Unknown    Product    [fe80::5054:ff:fe63:5e7a%4]    N
```

`RCS`: Indicates whether there is a reachable instance of the Remote Control
Service (RCS) running on the device.

In order to get `ffx` to automatically connect to a device, you must either have
set the target's nodename to be the default target, or attempt to interact with the
device.

To set the target to be the default, run:

```posix-terminal
fx ffx target default set $NODENAME
```

If the default target has been set prior to starting the daemon, waiting a few seconds
should yield a change to the `RCS` status to show `Y`.

If the default target has been set after starting the daemon, attempting to interact
with the target should be sufficient to kick off a connection, like the following

```posix-terminal
fx ffx component list
```

Note: If the default target has been set, and you are unable to run that command
against the target, [reach out](#contacting_the_ffx_team) to the `ffx` team.

Then the next time you list targets you should see that an `RCS` connection
is active.

```none
$ fx ffx target list
NAME                    SERIAL       TYPE       STATE      ADDRS/IP                       RCS
fuchsia-emulator  <unknown>    Unknown    Product    [fe80::5054:ff:fe63:5e7a%4]    Y
```

If a target has been set as default there will be a `*` next to it.

If you had `ffx log` running, you should also see something like the following in
the logs:

```none
[00009.776170][28540][28542][remote-control, remote_control_bin] INFO: published remote control service to overnet
```

NOTE: if the `RCS` column remains `N` for an extended amount of time and you have
already set this target's nodename to `target.default` _before_ initially starting
ffx, [reach out](#contacting_the_ffx_team) to the `ffx` team.

### On Default Targets

Above we covered setting the default target using the command

```posix-terminal
fx ffx target default set
```

It is also possible to set the default target on a per-command basis using the
`--target` flag like so.

```posix-terminal
fx ffx --target $NODENAME component list
```

### Interacting with multiple devices

TODO: fill this out.

### Controlling the state of target devices

You can use the `target off` and `target reboot` subcommands to power-off or
reboot a device, respectively.

## Configuration

See documentation for the [config command](/development/tools/ffx/commands/config.md).

## Interacting with Components

### Monikers

Many `ffx` commands that use components take monikers as a parameter. You can read more about
monikers and their syntax in [component moniker documentation](/reference/components/moniker.md).

### Finding components

The `component list` command will output monikers of all components that currently exist
in the component topology.

```none
$ fx ffx component list
/
/bootstrap
/bootstrap/archivist
/bootstrap/base_resolver
/bootstrap/console
/bootstrap/console-launcher
/bootstrap/cr50_agent
/bootstrap/device_name_provider
/bootstrap/driver_index
/bootstrap/driver_manager
/bootstrap/flashmap
/bootstrap/fshost
/bootstrap/fshost/blobfs
/bootstrap/fshost/blobfs/decompressor
...
```

You can use the `component select capability` command to search for components that use/expose
a capability with a given name.

The following command will display all components that use/expose the `diagnostics` capability:

```none
$ fx ffx component capability diagnostics
Exposed:
  /bootstrap/archivist
  /bootstrap/base_resolver
  /bootstrap/driver_manager
  /bootstrap/fshost
  /bootstrap/fshost/blobfs
  /bootstrap/fshost/blobfs/decompressor
  /bootstrap/fshost/minfs
  /bootstrap/pkg-cache
  /bootstrap/power_manager
  ...
```

### Inspecting a component

You can use the `component show` command to get detailed information about a specific
component.

`component show` allows partial matching on URL, moniker and component instance ID.

The following command will display information about the `/core/network/dhcpd` component:

```none
$ fx ffx component show dhcpd
               Moniker:  /core/network/dhcpd
                   URL:  #meta/dhcpv4_server.cm
           Instance ID:  20b2c7aba6793929c252d4e933b8a1537f7bfe8e208ad228c50a896a18b2c4b5
                  Type:  CML Component
       Component State:  Resolved
 Incoming Capabilities:  /svc/fuchsia.net.name.Lookup
                         /svc/fuchsia.posix.socket.packet.Provider
                         /svc/fuchsia.posix.socket.Provider
                         /svc/fuchsia.stash.SecureStore
                         /svc/fuchsia.logger.LogSink
  Exposed Capabilities:  fuchsia.net.dhcp.Server
           Merkle root:  521109a2059e15acc93bf77cd20546d106dfb625f2d1a1105bb71a5e5ea6b3ca
       Execution State:  Running
          Start reason:  '/core/network/netcfg' requested capability 'fuchsia.net.dhcp.Server'
         Running since:  2022-09-15 16:07:48.469094140 UTC
                Job ID:  28641
            Process ID:  28690
 Outgoing Capabilities:  fuchsia.net.dhcp.Server
```

### Verifying capability routes

You can use the `component doctor` command to verify that all capabilities
exposed and used by a component are successfully routed.

For example:

```none
$ fx ffx component doctor /bootstrap/archivist
Querying component manager for /bootstrap/archivist
URL: fuchsia-boot:///#meta/archivist.cm
Instance ID: None

      Used Capability                      Error
 [✓]  fuchsia.boot.ReadOnlyLog             N/A
 [✓]  fuchsia.boot.WriteOnlyLog            N/A
 [✓]  fuchsia.component.DetectBinder       N/A
 [✓]  fuchsia.component.KcounterBinder     N/A
 [✓]  fuchsia.component.LogStatsBinder     N/A
 [✓]  fuchsia.component.PersistenceBinder  N/A
 [✓]  fuchsia.component.SamplerBinder      N/A
 [✓]  fuchsia.sys.internal.ComponentEvent  N/A
      Provider
 [✓]  fuchsia.sys.internal.LogConnector    N/A
 [✓]  config-data                          N/A
 [✓]  fuchsia.sys2.EventSource             N/A

      Exposed Capability                   Error
 [✓]  fuchsia.diagnostics.FeedbackArchive  N/A
      Accessor
 [✓]  fuchsia.diagnostics.LegacyMetricsAr  N/A
      chiveAccessor
 [✓]  fuchsia.diagnostics.LoWPANArchiveAc  N/A
      cessor
 [✓]  diagnostics                          N/A
 [✓]  fuchsia.diagnostics.ArchiveAccessor  N/A
 [✓]  fuchsia.diagnostics.LogSettings      N/A
 [✓]  fuchsia.logger.Log                   N/A
 [✓]  fuchsia.logger.LogSink               N/A
```

```none
$ fx ffx component doctor /core/feedback
Querying component manager for /core/feedback
URL: fuchsia-pkg://fuchsia.com/forensics#meta/feedback.cm
Instance ID: eb345fb7dcaa4260ee0c65bb73ef0ec5341b15a4f603f358d6631c4be6bf7080

      Used Capability                      Error
 [✓]  fuchsia.boot.ReadOnlyLog             N/A
 [✓]  fuchsia.boot.WriteOnlyLog            N/A
 [✓]  fuchsia.diagnostics.FeedbackArchive  N/A
      Accessor
 [✓]  fuchsia.hardware.power.statecontrol  N/A
      .RebootMethodsWatcherRegister
 [✓]  fuchsia.hwinfo.Board                 N/A
 [✓]  fuchsia.hwinfo.Product               N/A
 [✓]  fuchsia.metrics.MetricEventLoggerFa  N/A
      ctory
 [✓]  fuchsia.net.http.Loader              N/A
 [✓]  fuchsia.process.Launcher             N/A
 [✓]  fuchsia.sysinfo.SysInfo              N/A
 [✓]  fuchsia.ui.activity.Provider         N/A
 [✗]  fuchsia.feedback.DeviceIdProvider    `/core/feedback` tried to use `fuchsia.feedback.DeviceIdProvider` from its parent,
                                           but the parent does not offer that capability. Note, use clauses in CML default to
                                           using from parent.
 ...
```

### Running a component

The `component run` command can create and launch components in a given isolated collection.

Note: `fx serve` must be running in order to run a package that is not
[in base or cached](/development/build/build_system/boards_and_products.md#dependency_sets).

Here's an example of running the Rust `hello-world` component in the `/core/ffx-laboratory`
collection. First, you'll need the hello-world package in your universe:

```none
$ fx set <product>.<board> --with //examples/hello_world/rust:hello-world-rust && fx build
...
```

Then use the `component run` command to create and launch a component instance from the URL
`fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm` with the moniker
`/core/ffx-laboratory:hello-world-rust`:

```none
$ fx ffx component run /core/ffx-laboratory:hello-world-rust fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm
URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm
Moniker: /core/ffx-laboratory:hello-world-rust
Creating component instance...
...
$ fx ffx component show hello-world-rust
               Moniker: /core/ffx-laboratory:hello-world-rust
                   URL: fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm
                  Type: v2 dynamic component
       Execution State: Running
                Job ID: 50775
            Process ID: 50819
...
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

```none
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

```none
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

## Testing with ffx

The `ffx` command is useful when writing integration tests which need to interact
with the Fuchsia environment. However, since `ffx` is primarily designed for
developers, it inspects the current environment for configuration and also starts
a daemon in the background to coordinate communication with Fuchsia devices. This
makes it more complex to write automated tests that use `ffx` since the configuration
and daemon should be isolated in order to avoid side effects, or interference from
the global environment.

To achieve this isolation, test authors need to use [isolate directories][isolate-dir]
when running tests which use `ffx`.

## Next steps

- Please provide feedback on this doc by [reaching out to the ffx team](#contacting_the_ffx_team)!
- Learn how to [extend `ffx`](/development/tools/ffx/development/plugins.md).


<!-- Reference links -->

[isolate-dir]: development/integration_testing.md
