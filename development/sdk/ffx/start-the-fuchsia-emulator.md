# Start the Fuchsia emulator

The [`ffx emu`][ffx-emu] commands launch and manage the Fuchsia emulator on
your host machine.

## Concepts

The Fuchsia emulator (also known as FEMU) provides a mechanism for developing
and testing the Fuchsia platform and products without using physical hardware
devices. The Fuchsia emulator is built on [QEMU][qemu]{:.external},
an open-source machine emulator and virtualizer.

### QEMU and AEMU {:#qemu-and-aemu}

There are two emulation engines available in the Fuchsia emulator: the first is
an extension of the Android Emulator ([AEMU][aemu]{:.external}) and the second
is a build of QEMU (version 6.2.0). The AEMU engine is primarily used for tasks
that involve graphics because AEMU supports the Vulkan graphics libraries.
However, the most recent AEMU (which is built off of a fork of QEMU 2.12) lacks
several versions of upstream fixes, features, and performance enhancements. For
this reason, using the QEMU engine in headless mode is preferred for workflows that
[do not require graphics](#start-the-fuchsia-emulator-without-graphics-support).

Starting an emulator requires you to specify the following information:

- An engine to use as the backend.
- A type of virtual device to emulate.
- A product to run on the virtual device.

By default, the `ffx emu` command selects the AEMU engine, as it's expected that
most tasks will require some level of graphics support. At the moment, all
existing product bundles only have a single virtual device type.

### Product bundles {:#product-bundles}

Both FEMU engines use a set of Fuchsia images and metadata known as a _product
bundle_. With the Fuchsia SDK, you need to retrieve product bundles (using the
[`ffx product-bundle get`](#download-a-product-bundle) command) ahead of time.
A product bundle includes system images for different CPU architectures. And
each system image includes metadata (such as hardware specifications
for running on different virtual devices) and associated system packages.

### Networking {:#networking}

Three networking modes are available between the emulator and its host machine:

- [Tun/Tap network mode](#start-the-fuchsia-emulator-in-tun-tap-networking-mode)
- [User networking mode](#start-the-fuchsia-emulator-in-user-networking-mode)
- [Networking disabled](#start-the-fuchsia-emulator-with-networking-disabled)

These modes are selected using the `--net` flag with the options of `tap`, `user`,
and `none`, respectively. There is also the `--net auto` mode, which is the
default if no flag is specified. In the `auto` mode, the emulator checks to see
if a [Tun/Tap][tun-tap]{:.external} interface is available on the host machine,
and if there is, the emulator selects the `--net tap` mode. However, if there is
not or the Tun/Tap interface is already busy (for instance, it’s attached to
another emulator process), the emulator then selects the `--net user` mode.

## List all available product bundles {:#list-product-bundles}

To view the list of all available product bundles from an online storage (by
default, Google Cloud Storage at `gs://fuchsia`), run the following command:

```posix-terminal
ffx product-bundle list
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx product-bundle list
Getting product metadata.
.
gs://fuchsia/development/8.20220511.2.1/sdk/product_bundles.json#terminal.arm64
gs://fuchsia/development/8.20220511.2.1/sdk/product_bundles.json#terminal.qemu-arm64
gs://fuchsia/development/8.20220511.2.1/sdk/product_bundles.json#terminal.qemu-x64
gs://fuchsia/development/8.20220511.2.1/sdk/product_bundles.json#terminal.x64
gs://fuchsia/development/8.20220511.2.1/sdk/product_bundles.json#workstation.chromebook-x64-dfv2
gs://fuchsia/development/8.20220511.2.1/sdk/product_bundles.json#workstation.qemu-x64*
...
```

## Download a product bundle {:#download-a-product-bundle}

To download a product bundle from an online storage (by default, Google Cloud
Storage at `gs://fuchsia`), run the following command:

```posix-terminal
ffx product-bundle get {{ "<var>" }}PRODUCT_BUNDLE{{ "</var>" }}
```

Replace `PRODUCT_BUNDLE` with the name of a product bundle.

The example command below downloads the `workstation.qemu-x64` product bundle:

```none {:.devsite-disable-click-to-copy}
$ ffx product-bundle get workstation.qemu-x64
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx product-bundle get workstation.qemu-x64
Getting product metadata.
.
Getting product metadata.
.
Getting product data for "workstation.qemu-x64"
...........................................................................................
Getting package data for "workstation.qemu-x64"
.
...........................................................................................
...........................................................................................
........................................Download of product data for "workstation.qemu-x64" is complete.
```

This command may take a few minutes to download the image and product metadata.

## Start the Fuchsia emulator {:#start-the-fuchsia-emulator}

Note: By default, the `ffx emu start` command runs on the
[AEMU engine](#qemu-and-aemu) with graphics support. To disable graphics,
see [Start the Fuchsia emulator without graphics
support](#start-the-fuchsia-emulator-without-graphics-support).

To start the Fuchsia emulator, run the following command:

```posix-terminal
ffx emu start {{ "<var>" }}PRODUCT_BUNDLE{{ "</var>" }} [--name {{ "<var>" }}NAME{{ "</var>" }}]

```

Replace `PRODUCT_BUNDLE` with the name of a product bundle
[downloaded](#download-a-product-bundle) on your host machine.

The example command below uses the `workstation.qemu-x64` product bundle to
start the Fuchsia emulator:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64
Logging to "/home/alice/.local/share/Fuchsia/ffx/emu/instances/fuchsia-emulator/emulator.log"
Waiting for Fuchsia to start (up to 60 seconds).............................
Emulator is ready.
```

Once the emulator is launched, the command exits while the emulator continues to
run in the background. Then a separate window opens and starts displaying the
graphical interface of the Fuchsia system included in the input product bundle.

By default, the Fuchsia emulator is launched with the name `fuchsia-emulator`.
However, to start the Fuchsia emulator with a different name (for instance, when
you need to launch multiple instances), you can use the `--name` flag, for
example:.

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --name my-fuchsia-example-01
```

To see the list of all running emulator instances, you can use the
[`ffx emu list`](#list-running-emulator-instances) command.

## Start the Fuchsia emulator without graphics support {:#start-the-fuchsia-emulator-without-graphics-support}

By default, the `ffx emu start` command launches the Fuchsia emulator and opens a
new window to display the emulator’s graphic interface. However, in some cases,
you may prefer disabling the graphics support on your emulator instance for
tasks that require no visuals, such as automated testing.

To start the Fuchsia emulator without graphics, run the following command:

Note: Using the [QEMU engine](#qemu-and-aemu) in headless mode is preferred
for workflows without graphics support.

```posix-terminal
ffx emu start {{ "<var>" }}PRODUCT_BUNDLE{{ "</var>" }} --headless [--engine qemu]
```

Replace `PRODUCT_BUNDLE` with the name of a product bundle
[downloaded](#download-a-product-bundle) on your host machine.

The example command below uses the `workstation.qemu-x64` product bundle:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --headless --engine qemu
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --headless --engine qemu
Logging to "/home/alice/.local/share/Fuchsia/ffx/emu/instances/fuchsia-emulator/emulator.log"
Waiting for Fuchsia to start (up to 60 seconds).............................
Emulator is ready.
```

This command starts an instance of the Fuchsia emulator in the background as
usual, but its graphical interface is not displayed on your host machines’
screen.

## List all running emulator instances {:#list-running-emulator-instances}

To view the list of running Fuchsia emulator instances on the host machine, run
the following command:

```posix-terminal
ffx emu list
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx emu list
[Inactive]  my-fuchsia-example-01
[Active]    fuchsia-emulator
```

In the output, next to each emulator instance is its status. If the status is
`[Active]`, it means the instance is running and is responsive to other Fuchsia
tools. However if the status is `[Inactive]`, this may mean the instance is
[stopped while preserving its working directory](#stop-an-emulator-instance-with-persist),
or it is still running but is not responsive (for instance, due to a network
failure). An instance that is started with networking disabled is never shown
as `[Active]`, even if it is running and healthy.

## Stop an emulator instance {:#stop-an-emulator-instance}

To stop a running instance of the Fuchsia emulator, run the following command:

```posix-terminal
ffx emu stop {{ "<var>" }}INSTANCE_NAME{{ "</var>" }}
```

Replace `INSTANCE_NAME` with the name of a running Fuchsia emulator instance.

The example command below stops the `my-fuchsia-example-01` instance:

```none {:.devsite-disable-click-to-copy}
$ ffx emu stop my-fuchsia-example-01
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx emu stop my-fuchsia-example-01
Stopping emulator 'my-fuchsia-example-01'...
Terminating running instance 2752927
```

The command terminates the running process for that emulator and removes the
temporary working directory created for that instance on the host machine. (If
you want to preserve this working directory, see
[Stop a running emulator instance while retaining its working directory](#stop-an-emulator-instance-with-persist).

If there is only one running emulator instance, you can omit the `INSTANCE_NAME`
field to stop the instance, for example:

```none {:.devsite-disable-click-to-copy}
$ ffx emu stop
Stopping emulator 'fuchsia-emulator'...
Terminating running instance 2752927
```

If you want to stop all running instances at once, you can use the `--all` flag,
for example:

```none {:.devsite-disable-click-to-copy}
$ ffx emu stop --all
Stopping emulator 'my-fuchsia-example-01'...
Stopping emulator 'fuchsia-emulator'...
Terminating running instance 2749406
```

## Stop an emulator instance while preserving its working directory {:#stop-an-emulator-instance-with-persist}

When stopping a running emulator instance, by default the working directory for
that instance is deleted from the host machine. However, in some cases, you may
want to keep the instance’s working directory so that you can continue examining
the directory’s contents for debugging purposes.

To stop a running instance of the Fuchsia emulator without deleting its working
directory, run the following command:

```posix-terminal
ffx emu stop {{ "<var>" }}INSTANCE_NAME{{ "</var>" }} --persist
```

Replace `INSTANCE_NAME` with the name of a running Fuchsia emulator instance.

The example command below stops the `my-fuchsia-example-01` instance:

```none {:.devsite-disable-click-to-copy}
$ ffx emu stop my-fuchsia-example-01 --persist
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx emu stop my-fuchsia-example-01 --persist
Stopping emulator 'my-fuchsia-example-01'...
Terminating running instance 2752927
```

After this command is run, the instance's working directory remains on the host
machine, and the stopped instance continues to appear on the emulator instances
list ([`ffx emu list`](#list-running-emulator-instances)) with the `[Inactive]` status.

To restart this stopped emulator instance, run the `ffx emu start` command
with the `--reuse` flag, for example:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start my-fuchsia-example-01 --reuse
```

When you no longer need the working directory, you can run the `ffx emu stop`
command again to delete the directory, for example:

```none {:.devsite-disable-click-to-copy}
$ ffx emu stop my-fuchsia-example-01
Stopping emulator 'my-fuchsia-example-01'...
```

## Show the emulator configuration {:#show-the-emulator-configuration}

To view the Fuchsia emulator configuration on the host machine, run the
following command:

```posix-terminal
ffx emu show
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx emu show
EmulatorConfiguration {
    device: DeviceConfig {
        audio: AudioDevice {
            model: Hda,
        },
        cpu: VirtualCpu {
            architecture: X64,
            count: 0,
        },
...
```

## Start the Fuchsia emulator in Tun/Tap networking mode {:#start-the-fuchsia-emulator-in-tun-tap-networking-mode}

In the `--net tap` mode, the host machine's kernel sets up a virtual networking
tunnel device (see [Tun/Tap][tun-tap]{:.external}) and bridges that device with
the host machine's physical network. This setup enables traffic coming through
the virtual device to be routed to the physical network. The emulator then
attaches its instance's virtual network interface to the virtual tunnel
device on the host machine. This has the same effect as the emulator instance
being directly connected to the physical network of the host machine. Therefore,
the `--net tap` mode delivers the fastest performance among all networking options.
However, this mode’s setup requires superuser access (`sudo`) on the host machine,
and it is only supported on Linux hosts (though there exist open-source extensions
that enable this setup on macOS hosts as well).

To start the emulator in the Tun/Tap networking mode,
run the following command:

```posix-terminal
ffx emu start {{ "<var>" }}PRODUCT_BUNDLE{{ "</var>" }} --net tap
```

Replace `PRODUCT_BUNDLE` with the name of a product bundle
[downloaded](#download-a-product-bundle) on your host machine.

The example command below uses the `workstation.qemu-x64` product bundle:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --net tap
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --net tap
Logging to "/home/alice/.local/share/Fuchsia/ffx/emu/instances/fuchsia-emulator/emulator.log"
Waiting for Fuchsia to start (up to 60 seconds)......................
Emulator is ready.
```

With the Tun/Tap networking mode, you have the option to specify an “upscript”
to, for instance, set up network interfaces and firewall exceptions. This script
is executed as part of the start-up for every emulator instance that runs with
the Tun/Tap networking mode.

To add a network upscript to your `ffx` configuration, run the following command
before starting the emulator:

```posix-terminal
ffx config set emu.upscript <PATH_TO_UPSCRIPT>
```

Replace `PATH_TO_UPSCRIPT` with the path of an upscript file.

The example command below adds the `my-upscript.sh` to the `ffx` configuration:

```none {:.devsite-disable-click-to-copy}
$ ffx config set emu.upscript /home/alice/my-fuchsia-project/my-upscript.sh
```

## Start the Fuchsia emulator in user networking mode {:#start-the-fuchsia-emulator-in-user-networking-mode}

If you don't have superuser access (`sudo`) on the host machine or don't want to
adjust your macOS setup to use Tun/Tap (see the
[`--net tap`](#start-the-fuchsia-emulator-in-tun-tap-networking-mode) mode), you can
configure the network using the `--net user` mode. In this mode, the emulator acts
as a firewall (through [SLiRP][slirp]{:.external}). Traffic coming out of an emulator
instance is unrestricted. However, sending traffic to an emulator instance requires
the following setup:

- A virtual device’s specification, which is part of a
  [product bundle](#product-bundles) (for instance, `workstation.qemu-x64`),
  pre-defines the mapping of the ports needed for the device.
- These ports can be mapped to unused ports on the host machine during the
  start-up of an emulator instance.

For example, to set up an SSH connection from the host machine to an emulator instance,
first the product bundle's virtual device specification must allow an instance's SSH
port (which is 22 commonly) to be used by the device. Next, when you start the Fuchsia
emulator, you can use the `ffx emu start` command to map the instance's SSH port to
an unused port on the host machine, let's say 8022. Then once the emulator instance
starts running, you can establish an SSH session to the instance through the host
machine’s port 8022.

In the virtual device specification, the `ports` field provides a list of
service names and their port numbers, for example:

```none {:.devsite-disable-click-to-copy}
ports = {
        ssh = 22
        mdns = 5353
        debug = 2345
}
```

The example above indicates that the emulator must map an emulator instance's three
specified ports (that is, 22, 5353, and 2345) to unused ports on the host machine
during start-up. Using this specification, the emulator uses the names `ssh`,
`mdns`, and `debug` for the three ports, respectively. For each of these ports,
you can use the `--port-map` flag to map the name to an unused port on the host
machine, for example:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --net user --port-map ssh:8022 --port-map debug:12345
```

Any ports in the virtual device specification that are not explicitly mapped using
the `--port-map` flag get arbitrarily assigned to unused ports on the host
machine during start-up. The final port mapping can be seen in the output of
[`ffx emu show`](#show-the-emulator-configuration).

To start the emulator in user networking mode, run the following
command:

```posix-terminal
ffx emu start {{ "<var>" }}PRODUCT_BUNDLE{{ "</var>" }} --net user [--port-map {{ "<var>" }}PORT_NAME{{ "</var>" }}:{{ "<var>" }}PORT_NUMBER{{ "</var>" }}]
```

Replace the following:

- `PRODUCT_BUNDLE` – The name of a product bundle
  [downloaded](#download-a-product-bundle) on your host machine.
- `PORT_NAME` – (Optional) The name of a service on the emulator instance.
- `PORT_NUMBER` – (Optional) The port number on the host machine to which you
  want to map the `PORT_NAME` service.

The example command below starts the emulator in user networking mode:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --net user
```

Without specifying the `--port-map` flags, the emulator maps the services on the
emulator instance to any unused ports on the host machine.

The example command below specifies port maps when starting the emulator
in user networking mode:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --net user --port-map ssh:8022 --port-map debug:12345
```

To see how ports are mapped for an emulator instance in user networking
mode, run [`ffx emu show`](#show-the-emulator-configuration) and examine the
output's `port_map` field, for example:

```none {:.devsite-disable-click-to-copy}
$ ffx emu show
EmulatorConfiguration {
    [...]
    host: HostConfig {
        [...]
        networking: User,
        port_map: {
            "ssh": PortMapping {
                guest: 22,
                host: Some(
                    8022,
                ),
            },
            "mdns": PortMapping {
                guest: 5353,
                host: Some(
                    35801,
                ),
            },
            "debug": PortMapping {
                guest: 2345,
                host: Some(
                    12345,
                ),
            },
        },
    },
    [...]
}
```

## Start the Fuchsia emulator with networking disabled {:#start-the-fuchsia-emulator-with-networking-disabled}

When `--net none` is specified, no networking capabilities are available to the
emulator instance (from the instance's perspective), as if there is no
network card installed on the virtual machine. The only mechanisms available for
communicating with the emulator instance are through [the QEMU monitor and the
emulated serial port](#start-the-fuchsia-emulator-with-console).

To start the emulator with networking disabled, run the following command:

Note: With networking disabled, adding the [`--console`](#emulated-serial-port) flag is
recommended since it provides a way to interact with the emulator instance.

```posix-terminal
ffx emu start {{ "<var>" }}PRODUCT_BUNDLE{{ "</var>" }} --net none [--console]
```

Replace `PRODUCT_BUNDLE` with the name of a product bundle
[downloaded](#download-a-product-bundle) on your host machine.

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --net none
Logging to "/home/alice/.local/share/Fuchsia/ffx/emu/instances/fuchsia-emulator/emulator.log"
Waiting for Fuchsia to start (up to 60 seconds).............................................................
Emulator did not respond to a health check before timing out.
```

When networking is disabled, the `ffx emu start` command cannot complete the “health check”
of the instance, which results in the status of the instance to be `[Inactive]`, which is
expected, for example:

```none {:.devsite-disable-click-to-copy}
$ ffx emu list
[Inactive]    fuchsia-emulator
```

## Start the Fuchsia emulator with a console for debugging {:#start-the-fuchsia-emulator-with-console}

The Fuchsia emulator provides the following built-in consoles that enable developers to interact
with emulator instances:

- [The QEMU monitor](#qemu-monitor)
- [Fuchsia's emulated serial port](#emulated-serial-port)

Both consoles work with both the [QEMU and AEMU](#qemu-and-aemu) engines.

### The QEMU monitor {:#qemu-monitor}

With the QEMU monitor, you can issue commands directly to the emulator process. Some of these
commands allow you to modify emulated devices, query virtual hardware state, and terminate the
emulator process using virtual hardware signals. (For more information on available commands, see
the [QEMU Monitor][qemu-monitor]{:.external} page.)

To start an emulator instance attached to the QEMU monitor, run the following command:

```posix-terminal
ffx emu start {{ "<var>" }}PRODUCT_BUNDLE{{ "</var>" }} --monitor
```

Replace `PRODUCT_BUNDLE` with the name of a product bundle
[downloaded](#download-a-product-bundle) on your host machine.

The example command below uses the `workstation.qemu-x64` product bundle:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --monitor
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --monitor
QEMU 2.12.0 monitor - type 'help' for more information
(qemu)
```

When running an emulator instance with the QEMU monitor attached, the command does not exit to
the terminal immediately. Instead, the command drops you into a
[QEMU monitor prompt][qemu-monitor]{:.external}.

To exit the QEMU monitor prompt, you can issue the `quit` command, which also terminates
the emulator instance, for example:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --monitor
QEMU 2.12.0 monitor - type 'help' for more information
(qemu) {{ '<strong>' }}quit{{ '</strong>' }}

WARNING | Write called without a backing file!
$
```

### Fuchsia's emulated serial port {:#emulated-serial-port}

If an emulator instance is started while attached to the emulated serial port, the Fuchsia platform
treats the serial port as a terminal. During the boot process, this terminal streams kernel logs
over the port, and once the system boots, it displays a Fuchsia command line prompt.

To start an emulator instance attached to the emulated serial port, run the following command:

```posix-terminal
ffx emu start {{ "<var>" }}PRODUCT_BUNDLE{{ "</var>" }} --console
```

Replace `PRODUCT_BUNDLE` with the name of a product bundle
[downloaded](#download-a-product-bundle) on your host machine.

The example command below uses the `workstation.qemu-x64` product bundle:

```none {:.devsite-disable-click-to-copy}
$ ffx emu start workstation.qemu-x64 --console
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
alice@alice:~$ ffx emu start workstation.qemu-x64 --console
INFO    | Android emulator version 31.3.8.0 (build_id 8611574) (CL:N/A)
[... various log entries]
[00021.699] 01095:01158> [component_manager] INFO: Connecting fuchsia.sys2.LifecycleController
$
```

After starting the emulator instance, this command drops you into a Fuchsia command line prompt
where you can issue various on-device command lines directly to the instance.

To exit the serial port console, you can issue the `dm poweroff` command, which also terminates
the emulator instance, for example:

```none {:.devsite-disable-click-to-copy}
$ dm poweroff
[00258.335] 26935:26939> [shutdown-shim]: checking power_manager liveness
[... various log entries]
[00260.447] 03742:03744> [00260.447038][3742][3744][driver_manager.cm] WARNING: [src/devices/bin/driver_manager/v1/suspend_handler.cc(211)] Failed to cause VFS exit ourselves, this is expected during orderly shutdown: FIDL operation failed d
[00260.429] 03341:03344> WARNING | Write called without a backing file!
alice@alice:~$
```

<!-- Reference links -->

[ffx-emu]: https://fuchsia.dev/reference/tools/sdk/ffx#emu
[qemu]: https://www.qemu.org/
[aemu]: https://developer.android.com/studio/run/emulator
[tun-tap]: https://en.wikipedia.org/wiki/TUN/TAP
[slirp]: https://en.wikipedia.org/wiki/Slirp
[qemu-monitor]: https://www.qemu.org/docs/master/system/monitor.html