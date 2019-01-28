<!--Multi Device Setup-->
多设备设置
============

This guide will walk you through the process of getting two Fuchsia devices
set up and synchronizing story state using the
[Ledger](https://fuchsia.googlesource.com/peridot/+/master/docs/ledger/).

## Setup

### Devices

Follow the steps at in the [top level docs](../README.md) to:
* Check out the source and build Fuchsia.
* Install it on two devices (or emulators).
* Connect the devices to the network.

### [Googlers only] Private Test Network Setup

Follow netboot instructions.

### Identify Test Machines

Each Fuchsia device has a unique node name based on its MAC address.  It is of
the form `power-nerd-saved-santa`.  You can list the nodes on your network with
the `netls` command.

```
> netls
    device glad-bats-hunk-lady (fe80::f64d:30ff:fe68:2620/6)
    device blurt-chip-sugar-wish (fe80::8eae:4cff:feee:4f40/6)
```

### Running Commands On Test Machines

The `netruncmd <nodename> <command>` command can be used to run commands on
remote machines.  The output of the command is not shown.  If you need to see
the output, use the `loglistener [<nodename>]` command.

### Ledger Setup

Ledger is a distributed storage system for Fuchsia.  Stories use it to
synchronize their state across multiple devices.  Follow the steps in Ledger's
[User Guide](https://fuchsia.googlesource.com/peridot/+/master/docs/ledger/user_guide.md)
to:

* Set up [persistent storage](https://fuchsia.googlesource.com/zircon/+/master/docs/minfs.md). (optional)
* Verify the network is connected.
* Configure a Firebase instance.
* Setup sync on each device using `configure_ledger`.

## Run Stories

### Virtual consoles.

The systems boots up with three virtual consoles.  Alt-F1 through
Alt-F3 can be used to switch between virtual consoles.

### Wipe Data

The format of the Ledger as well as the format of the data each story syncs is
under rapid development and no effort is currently made towards forwards and
backwards compatibility.  Because of this, after updating the Fuchsia code, it
is a good idea to wipe your remote and local data using `cloud_sync clean`.

```
$ netruncmd <nodename> cloud_sync clean
```

### Start A Story On One Device

Use the `basemgr` to start a story on one device:

```
$ netruncmd <first-node-name> "basemgr --session_shell=dev_session_shell \
  --session_shell_args=--root_module=example_todo_story"
```

Using `loglistener <first-node-name>` take note of the story ID from a line the
following:

```
... DevSessionShell Starting story with id: IM7U9hBcCt
```

### Open The Same Story On The Second Device.

The story can be started on the second device either through the system UI or by
specifying the story ID.

#### System UI, aka Session Shell, aka Armadillo
Launch the system UI using `basemgr`:

```
$ netruncmd <second-node-name> "basemgr"
```

Once the system UI starts, you should be able to see the story started in the
step above.  Click on that to open it.

#### By Story ID

With the story ID noted above from launch the story from a shell:

```
$ netruncmd <second-node-name> "basemgr \
  --session_shell=dev_session_shell \
  --session_shell_args=--story_id=<story_id>
```
