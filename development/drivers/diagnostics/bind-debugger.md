# Bind Debugger

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

The debugger can be used to run a bind rules against a particular device. It
outputs a trace of the bind rules's execution, describing why the driver
would or would not bind to the device.

You can run the debugger in the following ways:

 - **[As a host tool.](#running-the-debugger-host)** You provide the bind rules
   source file and a file listing the properties of the device. This is useful
   during bind rules development for testing the outcome of the rules against
   different combinations of node properties.
 - **[On the target device.](#running-the-debugger-target)** You specify the driver
   path and the device path within the system. This is useful for figuring out why
   a driver did or did not bind to a particular device.

Note: The debugger can only be used with bind rules written in the bind language
described in this page.

## Running the debugger as a host tool {#running-the-debugger-host}

You can run the debugger with the `--debug` option in the bind compiler.

```
fx bindc debug \
  --include src/devices/bind/fuchsia.usb/fuchsia.usb.bind \
  --debug tools/bindc/examples/gizmo.dev \
  tools/bindc/examples/gizmo.bind
```

The bind rules source and the library sources are in the formats described in
the [bind rules](/development/drivers/concepts/device_driver_model/driver-binding.md#bind-rules) and
[bind libraries](/development/drivers/concepts/device_driver_model/driver-binding.md#bind-libraries) sections,
respectively. The `--debug` option takes a file containing a specification of
the device to run the bind rules against.

Note: The `--debug` and `--output` options are mutually exclusive, so the C
header file will not be generated when running the compiler in debug mode.

### Device specification file {#device-specification}

The debugger takes a file specifying the device to run against the bind rules.
This specification is simply a list of key-value pairs describing the properties
of the device.

#### Example

This example device specification can be found at
[//tools/bindc/examples/gizmo.dev](/tools/bindc/examples/gizmo.dev).

```
fuchsia.BIND_PROTOCOL = fuchsia.usb.BIND_PROTOCOL.INTERFACE
fuchsia.BIND_USB_VID = fuchsia.usb.BIND_USB_VID.REALTEK
fuchsia.BIND_USB_CLASS = fuchsia.usb.BIND_USB_CLASS.VIDEO
fuchsia.BIND_USB_SUBCLASS = fuchsia.usb.BIND_USB_SUBCLASS.VIDEO_CONTROL
```

#### Grammar

```
device-specification = ( property )* ;

property = compound-identifier , "=" , value ;

compound-identifier = IDENTIFIER ( "." , IDENTIFIER )* ;

value = compound-identifier | STRING-LITERAL | NUMERIC-LITERAL | "true" | "false" ;
```

An identifier matches the regex `[a-zA-Z]([a-zA-Z0-9_]*[a-zA-Z0-9])?`.

A string literal matches the regex `”[^”]*”`, and a numeric literal matches the
regex `[0-9]+` or
`0x[0-9A-F]+`.

The bind compiler will ignore (treat as whitespace) any line prefixed by `//`,
and any multiple
lines delimited by `/*` and `*/`.

## Running the debugger on the target device {#running-the-debugger-target}

The debugger is run using its package URL. For example:

```
ffx driver debug-bind /system/driver/bt-hci-intel.so class/bt-transport/000
```

The command takes the path of the driver to debug and the path of the device to
debug it against.

### Device path

There are two ways to specify the device:

 - Its path within /dev/class, e.g. `class/bt-transport/000`.
 - Its topological path, e.g. `sys/platform/pci/00:14.0/xhci/usb-bus/003/003/ifc-000/bt_transport_usb`.

Both of the paths are relative to /dev/.

The topological path can be determined from the output of `driver dump`. For
example, tracing the path to the node `[bt_transport_usb]` in the output below
gives the topological path
`sys/platform/pci/00:14.0/xhci/usb-bus/003/003/ifc-000/bt_transport_usb`.

```
[root]
   <root> pid=3456
      [null] pid=3456 /boot/driver/builtin.so
      [zero] pid=3456 /boot/driver/builtin.so
   [misc]
      <misc> pid=3525
         [demo-fifo] pid=3525 /boot/driver/demo-fifo.so
         [ktrace] pid=3525 /boot/driver/ktrace.so
   [sys]
      <sys> pid=3369 /boot/driver/platform-bus.so
         [pci] pid=3369 /boot/driver/platform-bus-x86.so
            [00:00.0] pid=3369 /boot/driver/bus-pci.so
            [00:14.0] pid=3369 /boot/driver/bus-pci.so
               <00:14.0> pid=4384 /boot/driver/bus-pci.proxy.so
                  [xhci] pid=4384 /boot/driver/xhci.so
                     [xdc] pid=4384 /boot/driver/xhci.so
                     [usb-bus] pid=4384 /boot/driver/usb-bus.so
                        [001] pid=4384 /boot/driver/usb-bus.so
                           [001] pid=4384 /boot/driver/usb-composite.so
                              [ifc-000] pid=4384 /boot/driver/usb-composite.so
                                 [usb-hid] pid=4384 /boot/driver/usb-hid.so
                                    [hid-device-000] pid=4384 /boot/driver/hid.so
                        [002] pid=4384 /boot/driver/usb-bus.so
                           [002] pid=4384 /boot/driver/usb-composite.so
                              [ifc-000] pid=4384 /boot/driver/usb-composite.so
                                 [usb-hid] pid=4384 /boot/driver/usb-hid.so
                                    [hid-device-000] pid=4384 /boot/driver/hid.so
                        [003] pid=4384 /boot/driver/usb-bus.so
                           [003] pid=4384 /boot/driver/usb-composite.so
                              [ifc-000] pid=4384 /boot/driver/usb-composite.so
                                 [bt_transport_usb] pid=4384 /boot/driver/bt-transport-usb.so
                                    [bt_hci_intel] pid=4384 /system/driver/bt-hci-intel.so
                                       [bt_host] pid=4384 /system/driver/bt-host.so
```

It should be noted that if driver framework version 2 (DFv2) is enabled, the
node's topological path can't be determined from its place in the node graph.

## Debugger output

The output of the debugger is a trace of the bind rules' evaluation. The trace
contains information about whether each statement in the bind rules succeeded,
and why or why not. For example, if a condition statement failed because the
device did not have the required value, the debugger will output what the actual
value of the device was (or the fact that the device had no value for that
property). The trace also includes information about which branches were taken
in if statements.

### Example

The output of the debugger when running the host tool command
[above](#running-the-debugger-host) is:

```
Line 4: Condition statement succeeded: fuchsia.BIND_PROTOCOL == fuchsia.usb.BIND_PROTOCOL.INTERFACE;
Line 6: If statement condition failed: fuchsia.BIND_USB_VID == fuchsia.usb.BIND_USB_VID.INTEL
    Actual value of `fuchsia.BIND_USB_VID` was `fuchsia.usb.BIND_USB_VID.REALTEK` [0xbda].
Line 9: If statement condition succeeded: fuchsia.BIND_USB_VID == fuchsia.usb.BIND_USB_VID.REALTEK
Line 11: Accept statement succeeded.
    Value of `fuchsia.BIND_USB_CLASS` was `fuchsia.usb.BIND_USB_CLASS.VIDEO` [0xe].
Driver binds to device.
```

If you run the debugger on the Fuchsia target device, you will see similar output
information. However, information such as identifiers and source code snippets may
be missing, since the system only stores the bind rules bytecode, not the
source code.

The trace shows the outcome of each statement that was reached while executing
the bind rules:

- The device has the USB device protocol, so the first condition statement is
satisfied.
- The device's vendor ID is REALTEK, so the second branch of the if statement is
taken.
- The device has one of the two accepted classes (video), so the accept
statement is satisfied.

The debugger outputs that the driver would successfully bind to a device with
these properties.
