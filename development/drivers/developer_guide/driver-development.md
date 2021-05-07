# Fuchsia driver development

Fuchsia drivers are shared libraries that are dynamically loaded in driver host
processes in user space. The process of loading a driver is controlled by the
driver manager. See [Device Model](/docs/concepts/drivers/device_driver_model/device-model.md)
for more information on driver hosts, driver manager and the driver and device lifecycles.

## Directory structure

Drivers may be found throughout the source tree under `driver` subdirectories of
areas as specified in the
[source code layout](/docs/concepts/source_code/layout.md) document. Most
Fuchsia drivers are found under [//src/devices/](/src/devices). They are grouped
based on the protocols they implement. The driver protocols are defined in
[ddk/include/lib/ddk/protodefs.h](/src/lib/ddk/include/lib/ddk/protodefs.h). For
example, a USB ethernet driver goes in
[//src/connectivity/ethernet/drivers/](/src/connectivity/ethernet/drivers/)
rather than [//src/devices/usb/drivers/](/src/devices/usb/drivers) because it
implements an ethernet protocol. However, drivers that implement the USB stack
are in [//src/devices/usb/drivers/](/src/devices/usb/drivers) because they
implement USB protocols.

In the driver's `BUILD.gn`, there should be a `driver_module` target. In order
to get a driver to show up under `/boot/driver`, it should be included as under
the `board_bootfs_labels` list in the relevant board file(s) under //boards. In
order to get it to show up inside of `/system/driver` it should be added to the
system package with a `driver_package` build target, which should then be
referenced by relevant boardfile(s) under `//boards`. The driver manager looks
first in `/boot/driver`, then `/system/driver/` for loadable drivers.

## Creating a new driver

Creating a new driver can be done automatically by using the
[Create Tool](/tools/create/README.md). Simply run the following command:

```
fx create driver <PATH> --lang cpp
```

This will create the directory `<PATH>` containing an empty driver where the
last segment of `<PATH>` is the driver name and GN target name. After this
command is run, the following steps need to be followed:

1) Include the `driver_module` or `driver_package` build target in the correct
place to get your driver included into the system. - For packaged drivers the
`driver_package` build target should be added to the relevant board file in
`//boards` or `//vendor/<foo>/boards` to a `xx_package_labels` GN argument. -
For boot drivers the `driver_module` build target should be added to the
relevant board file in `//boards` or `//vendor/<foo>/boards` to the
`board_bootfs_labels` GN argument. 2) Include the `tests` build target in the
`<PATH>:tests` build target to get your tests included in CQ. 3) Add proper bind
rules in `<NAME>.bind`. 4) Write the functionality for the driver.

## Declaring a driver

At a minimum, a driver should contain the driver declaration and implement the
`bind()` driver op.

Drivers are loaded and bound to a device when the driver manager successfully
finds a matching driver for a device. A driver declares the devices it is
compatible with through bind rules, which should be placed in a `.bind` file
alongside the driver. The bind compiler compiles those rules and creates a
driver declaration macro containing those rules in a C header file. The
following bind program declares the
[AHCI driver](/src/devices/block/drivers/ahci/ahci.h):

```
using deprecated.pci;

deprecated.BIND_PROTOCOL == deprecated.pci.BIND_PROTOCOL.DEVICE;
deprecated.BIND_PCI_CLASS == 0x01;
deprecated.BIND_PCI_SUBCLASS == 0x06;
deprecated.BIND_PCI_INTERFACE == 0x01;
```

These bind rules state that the driver binds to devices with a `BIND_PROTOCOL`
property that matches `DEVICE` from the `pci` namespace and with PCI class 1,
subclass 6, interface 1. The `pci` namespace is imported from the
`deprecated.pci` library on the first line. For more details, refer to the
[binding documentation](/docs/concepts/drivers/device_driver_model/driver-binding.md).

To generate a driver declaration macro including these bind rules, there should
be a corresponding `bind_rules` build target.

```
bind_rules("bind") {
    rules = "ahci.bind"
    output = "ahci-bind.h"
    deps = [
        "//src/devices/bind/deprecated.pci",
    ]
}
```

The driver can now include the generated header and declare itself with the
following macro. `"zircon"` is the vendor id and `"0.1"` is the driver version.

```c
#include "src/devices/block/drivers/ahci/ahci-bind.h"
...
ZIRCON_DRIVER(ahci, ahci_driver_ops, "zircon", "0.1");
```

The [PCI driver](/src/devices/bus/drivers/pci/kpci.cc) publishes the
matching device with the following properties:

```c
zx_device_prop_t device_props[] = {
    {BIND_PROTOCOL, 0, ZX_PROTOCOL_PCI},
    {BIND_PCI_VID, 0, info.vendor_id},
    {BIND_PCI_DID, 0, info.device_id},
    {BIND_PCI_CLASS, 0, info.base_class},
    {BIND_PCI_SUBCLASS, 0, info.sub_class},
    {BIND_PCI_INTERFACE, 0, info.program_interface},
    {BIND_PCI_REVISION, 0, info.revision_id},
    {BIND_PCI_BDF_ADDR, 0, BIND_PCI_BDF_PACK(info.bus_id, info.dev_id,
                                             info.func_id)},
};
```

For now, binding variables and macros are defined in
[lib/ddk/binding.h](/src/lib/ddk/include/lib/ddk/binding.h). In the near future, all
bind properties will be defined by bind libraries like the `fuchsia.pci` library
imported above. If you are introducing a new device class, you may need to
introduce new bind properties to the binding header as well as the
[bind libraries](/src/devices/bind/).

Bind properties are 32-bit values. If your variable value requires greater than
a 32-bit value, split them into multiple 32-bit variables. An example is ACPI
HID values, which are 8 characters (64-bits) long. It is split into
`BIND_ACPI_HID_0_3` and `BIND_ACPI_HID_4_7`. Once the migration to bind
libraries is complete you will be able to use other data types such as strings,
larger numbers, and booleans.

You may specify `disable_autobind = true` in the `bind_rules` build rule to
disable the automatic binding behaviour. In that case, a driver can be bound to
a device using `fuchsia.device.Controller/Bind` FIDL call.

## Driver binding

A driver's `bind()` function is called when it is matched to a device. Generally
a driver will initialize any data structures needed for the device and
initialize hardware in this function. It should not perform any time-consuming
tasks or block in this function, because it is invoked from the driver host's RPC
thread and it will not be able to service other requests in the meantime.
Instead, it should spawn a new thread to perform lengthy tasks.

The driver should make no assumptions about the state of the hardware in
`bind()`, resetting the hardware or otherwise ensuring it is in a known state.
Because the system recovers from a driver crash by re-spawning the driver host, the
hardware may be in an unknown state when `bind()` is invoked.

A driver is required to publish a `zx_device_t` in `bind()` by calling
`device_add()`. This is necessary for the driver manager to keep track of the
device lifecycle. If the driver is not able to publish a functional device in
`bind()`, for example if it is initializing the full device in a thread, it
should publish an invisible device by implementing the device `init()` hook, and
call `device_init_reply()` once initialization is complete.
`device_init_reply()` does not necessarily need to be called from the `init()`
hook. For example, it may be called from another worker thread. The device is
also guaranteed not to be removed until the reply is received. See `init()` in
[src/lib/ddk/include/lib/ddk/device.h](/src/lib/ddk/include/lib/ddk/device.h) and
`device_init_reply()` in
[src/lib/ddk/include/lib/ddk/driver.h](/src/lib/ddk/include/lib/ddk/driver.h).

There are generally four outcomes from `bind()`:

1.  The driver determines the device is supported and does not need to do any
    heavy lifting, so publishes a new device with `device_add()` and returns
    `ZX_OK`.

2.  The driver determines that even though the bind program matched, the device
    cannot be supported (maybe due to checking hw version bits or whatnot) and
    returns an error.

3.  The driver needs to do further initialization before the device is ready or
    it's sure it can support it, so it publishes a device that implements the
    `init()` hook and kicks off a thread to keep working, while returning
    `ZX_OK` to `bind()`. That thread will eventually call `device_init_reply()`
    with a status indicating whether it was able to successfully initialize the
    device and should be made visible, or that the device should be removed.

4.  The driver represents a bus or controller with 0..n children that may
    dynamically appear or disappear. In this case it should publish a device
    immediately representing the bus or controller, and then dynamically publish
    children (that downstream drivers will bind to) representing hardware on
    that bus. Examples: AHCI/SATA, USB, etc.

After a device is added and made visible by the system, it is made available to
client processes and for binding by compatible drivers.

## Banjo protocols

A driver provides a set of device ops and optional protocol ops to a device.
Device ops implement the device lifecycle methods and the external interface to
the device that are called by other user space applications and services.
Protocol ops implement the in-process protocols of the device that are called by
other drivers loaded into the same driver host.

You can pass one set of protocol ops for the device in `device_add_args_t`. If a
device supports multiple protocols, implement the `get_protocol()` device op. A
device can only have one protocol id. The protocol id corresponds to the class
the device is published under in devfs.

## Driver operation

A driver generally operates by servicing client requests from children drivers
or other processes. It fulfills those requests either by communicating directly
with hardware (for example, through MMIO) or by communicating with its parent device
(for example, queueing a USB transaction).

External client requests from processes outside the driver host are fulfilled by
children drivers, generally in the same process, are fulfilled by banjo
protocols corresponding to the device class. Driver-to-driver requests should
use banjo protocols instead of device ops.

A device can get a protocol supported by its parent by calling
`device_get_protocol()` on its parent device.

## Device interrupts

Device interrupts are implemented by interrupt objects, which are a type of
kernel objects. A driver requests a handle to the device interrupt from its
parent device in a device protocol method. The handle returned will be bound to
the appropriate interrupt for the device, as defined by a parent driver. For
example, the PCI protocol implements `map_interrupt()` for PCI children. A
driver should spawn a thread to wait on the interrupt handle.

The kernel will automatically handle masking and unmasking the interrupt as
appropriate, depending on whether the interrupt is edge-triggered or
level-triggered. For level-triggered hardware interrupts,
[zx_interrupt_wait()](/docs/reference/syscalls/interrupt_wait.md) will mask the
interrupt before returning and unmask the interrupt when it is called again the
next time. For edge-triggered interrupts, the interrupt remains unmasked.

The interrupt thread should not perform any long-running tasks. For drivers that
perform lengthy tasks, use a worker thread.

You can signal an interrupt handle with
[zx_interrupt_trigger()](/docs/reference/syscalls/interrupt_trigger.md) on slot
**ZX_INTERRUPT_SLOT_USER** to return from `zx_interrupt_wait()`. This is
necessary to shut down the interrupt thread during driver clean up.

## FIDL Messages

Messages for each device class are defined in the
[FIDL](/docs/development/languages/fidl/README.md) language. Each device
implements zero or more FIDL protocols, multiplexed over a single channel per
client. The driver is given the opportunity to interpret FIDL messages through the
`message()` hook.

## Protocol ops vs. FIDL messages

Protocol ops define the in-process API for a device. FIDL messages define the
external API. Define a protocol op if the function is meant to be called by
other drivers in the same process. A driver should call a protocol op on its
parent to make use of those functions.

## Isolate devices

Devices that are added with `DEVICE_ADD_MUST_ISOLATE` spawn a new driver host
with a proxy device. The device exists in both the parent driver host and as the
root of the new driver host. Devmgr attempts to load **driver**`.proxy.so` into
the new driver host. For example, PCI is supplied by `libpci.so` so devmgr would
look to load `libpci.proxy.so`. The driver is provided a channel in `create()`
when it creates the proxy device (the "bottom half" that runs in the new driver
host). The proxy device should cache this channel for when it needs to
communicate with the top half (e.g. if it needs to call API on the parent
device).

`rxrpc()` is invoked on the top half when this channel is written to by the
bottom half. There is no common wire protocol for this channel. For an example,
refer to the [PCI driver](/src/devices/bus/drivers/pci).

Note: Proxying is currently deprecated in favor of using FIDL between drivers in
different driver hosts. Please talk to the driver framework team before
implementing new proxies.

## Driver rights

Although drivers run in user space processes, they have a more restricted set of
rights than normal processes. Drivers are not allowed to access the filesystem,
including devfs. That means a driver cannot interact with arbitrary devices. If
your driver needs to do this, consider writing a service component instead. For
example,the virtual console is implemented by the
[virtcon](/src/bringup/bin/virtcon) component.

Privileged operations such as `zx_vmo_create_contiguous()` and
[zx_interrupt_create](/docs/reference/syscalls/interrupt_create.md) require a
root resource handle. This handle is not available to drivers other than the
system driver ([ACPI](/src/devices/board/drivers/x86) on x86 systems and
[platform](/src/devices/bus/drivers/platform) on ARM systems). A device should
request its parent to perform such operations for it. Contact the author of the
parent driver if its protocol does not address this use case.

Similarly, a driver is not allowed to request arbitrary MMIO ranges, interrupts
or GPIOs. Bus drivers such as PCI and platform only return the resources
associated to the child device.

# Advanced Topics and Tips

## Taking a long time to initialize

What if your device takes a long time to initialize?
When we discussed the **null_bind()** function above, we indicated that a successful
return told the driver manager that the driver is now associated with the device.
We can't spend a lot of time in the bind function; we're basically expected to initialize
our device, publish it, and be done.

But your device might need to perform a lengthy initialization operation, such as:

*   enumerate hardware points
*   load firmware
*   negotiate a protocol

and so on, which might take a long time to do.

You can publish your device as "invisible" by implementing the device `init()` hook.
The `init()` hook is run after the device is added through **device_add()**, and may be
used to safely access the device state and to spawn a worker thread. The device will
remain invisible and is guaranteed not to be removed until **device_init_reply()** is called,
which may be done from any thread. This meets the requirements for the binding function,
but nobody is able to use your device (because nobody knows about it yet, because it's
not visible). Now your device can perform the long operations with a background thread.

When your device is ready to service client requests, call
**device_init_reply()**
which will cause it to appear in the pathname space.

### Power savings

Two callouts, **suspend()** and **resume()**, are available for your device in
order to support power or other resource saving features.

Both take a device context pointer and a flags argument, but the flags argument is
used only in the suspend case.

Flag                                | Meaning
------------------------------------|------------------------------------------------------------
`DEVICE_SUSPEND_FLAG_REBOOT`        | The driver should shut itself down in preparation for a reboot or shutdown of the machine
`DEVICE_SUSPEND_FLAG_REBOOT_BOOTLOADER` | ?
`DEVICE_SUSPEND_FLAG_REBOOT_RECOVERY`   | ?
`DEVICE_SUSPEND_FLAG_POWEROFF`      | The driver should shut itself down in preparation for power off
`DEVICE_SUSPEND_FLAG_MEXEC`         | @@@ almost nobody uses this except for a graphics controller, what does it do? @@@
`DEVICE_SUSPEND_FLAG_SUSPEND_RAM`   | The driver should arrange so that it can be restarted from RAM

<!--- Yeah, I'm just guessing on the flags; they're used so little...-->

For documentation purposes, what should I write?
That they are just hints, or that you *must* do something because of a given flag, or ... ?

## Reference: Support functions

This section lists support functions that are provided for your driver to use.

### Accessor functions

The context block that's passed as the first argument to your driver's protocol functions
is an opaque data structure.
This means that in order to access the data elements, you need to call an accessor function:

Function                | Purpose
------------------------|-------------------------------------------
**device_get_name()**        | Retrieves the name of the device
**device_get_parent()**      | Retrieves the parent device of the device

### Administrative functions

The following functions are used to administer the device:

Function                    | Purpose
----------------------------|-------------------------------------------
**device_add()**                 | Adds a device to a parent
**device_async_remove()**        | Schedules the removal of a device and all its children

### Signalling

The following functions are used to set the state of a device:

Function                | Purpose
------------------------|-------------------------------------------
**device_state_set()**       | sets the given signal(s) on the device
**device_state_clr()**       | clears the given signal(s) on the device

We saw these in the `/dev/misc/demo-fifo` handler above.

<!---
@@@ Notes only @@@

This section is great for things like talking about buffer management,
threading, best practices, advanced options for device_add(), and so on.
I think it can be somewhere between the man page ("printf is used to print a string
and takes the following parameters") and an application note &mdash; I want to see
examples of how to use the functions, what the arguments mean, what the impact of
various design decisions is, that kind of thing.
-->
