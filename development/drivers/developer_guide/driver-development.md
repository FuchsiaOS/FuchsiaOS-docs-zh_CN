# Fuchsia driver development

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1). Also the workflows documented on
this page may only be specific to the Fuchsia source checkout
(`fuchsia.git`) environment.

Fuchsia drivers are shared libraries that are dynamically loaded in driver host
processes in user space. The process of loading a driver is controlled by the
driver manager. See
[Device Model](/development/drivers/concepts/device_driver_model/device-model.md)
for more information on driver hosts, driver manager and the driver and device
lifecycles.

## Directory structure

Drivers may be found throughout the source tree under `driver` subdirectories of
areas as specified in the
[source code layout](/development/source_code/layout.md) document. Most
Fuchsia drivers are found under [//src/devices/](/src/devices). They are grouped
based on the protocols they implement. The driver protocols are defined in
[ddk/include/lib/ddk/protodefs.h](/src/lib/ddk/include/lib/ddk/protodefs.h). For
example, a USB ethernet driver goes in
[//src/connectivity/ethernet/drivers/](/src/connectivity/ethernet/drivers/)
rather than [//src/devices/usb/drivers/](/src/devices/usb/drivers) because it
implements an ethernet protocol. However, drivers that implement the USB stack
are in [//src/devices/usb/drivers/](/src/devices/usb/drivers) because they
implement USB protocols.

In the driver's `BUILD.gn`, there should be a `fuchsia_driver_component` target.
In order to get a driver to show up under `/boot/driver`, it should be included
as under the `board_bootfs_labels` list in the relevant board file(s) under
//boards. In order to get it to show up inside of `/system/driver` it should be
added to the system package with a `driver_package` build target, which should
then be referenced by relevant boardfile(s) under `//boards`. The driver manager
looks first in `/boot/driver`, then `/system/driver/` for loadable drivers.

## Creating a new driver

Creating a new driver can be done automatically by using the
[Create Tool](/tools/create/README.md). Simply run the following command:

```
fx create driver --path <PATH> --lang cpp
```

This will create the directory `<PATH>` containing an empty driver where the
last segment of `<PATH>` is the driver name and GN target name. After this
command is run, the following steps need to be followed:

1.  Include the `fuchsia_driver_component` or `driver_package` build target in
    the correct place to get your driver included into the system.
2.  For packaged drivers the `driver_package` build target should be added to
    the relevant board file in `//boards` or `//vendor/<foo>/boards` to a
    `xx_package_labels` GN argument.
3.  For boot drivers the `fuchsia_driver_component` build target should be added
    to the relevant board file in `//boards` or `//vendor/<foo>/boards` to the
    `board_bootfs_labels` GN argument.
4.  Include the `tests` build target in the `<PATH>:tests` build target to get
    your tests included in CQ.
5.  Add proper bind rules in `<NAME>.bind`.
6.  Add driver information in `<NAME>-info.json`. The file must include a
    `short_description` and `areas` matching at least one of the areas listed at
    `//build/drivers/areas.txt`.
7.  Write the functionality for the driver.

## Declaring a driver

At a minimum, a driver should contain the driver declaration and implement the
`bind()` driver op.

Drivers are loaded and bound to a device when the driver manager successfully
finds a matching driver for a device. A driver declares the devices it is
compatible with through bind rules, which should be placed in a `.bind` file
alongside the driver. The bind compiler compiles those rules and creates a
driver declaration macro containing those rules in a C header file. The
following bind program declares the
[AHCI driver](/src/devices/block/drivers/ahci/):

```
using fuchsia.pci;
using fuchsia.pci.massstorage;

fuchsia.BIND_PROTOCOL == fuchsia.pci.BIND_PROTOCOL.DEVICE;
fuchsia.BIND_PCI_CLASS == fuchsia.pci.BIND_PCI_CLASS.MASS_STORAGE;
fuchsia.BIND_PCI_SUBCLASS == fuchsia.pci.massstorage.BIND_PCI_SUBCLASS_SATA;
fuchsia.BIND_PCI_INTERFACE == 0x01;
fuchsia.BIND_COMPOSITE == 1;
```

These bind rules state that the driver binds to devices with a `BIND_PROTOCOL`
property that matches `DEVICE` from the `pci` namespace and the given PCI
class/subclass/interface. The `pci` namespace is imported from the `fucnsia.pci`
library on the first line. For more details, refer to the [binding
documentation](/development/drivers/concepts/device_driver_model/driver-binding.md).

To generate a driver declaration macro including these bind rules, there should
be a corresponding `bind_rules` build target. This should declare dependencies
corresponding to the "using" statements in the bind file.

```
driver_bind_rules("bind") {
    rules = "ahci.bind"
    header_output = "ahci-bind.h"
    bind_output = "ahci.bindbc"
    deps = [
        "//src/devices/bind/fuchsia.pci",
        "//src/devices/bind/fuchsia.pci.massstorage",
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

The [PCI driver](/src/devices/bus/drivers/pci/kpci.cc) publishes the matching
device with the following properties:

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
[lib/ddk/binding.h](/src/lib/ddk/include/lib/ddk/binding.h). In the near future,
all bind properties will be defined by bind libraries like the `fuchsia.pci`
library imported above. If you are introducing a new device class, you may need
to introduce new bind properties to the binding header as well as the
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
tasks or block in this function, because it is invoked from the driver host's
RPC thread and it will not be able to service other requests in the meantime.
Instead, it should spawn a new thread to perform lengthy tasks.

The driver should make no assumptions about the state of the hardware in
`bind()`. It may need to reset the hardware or otherwise ensure it is in a known
state. Because the system recovers from a driver crash by re-spawning the driver
host, the hardware may be in an unknown state when `bind()` is invoked.

There are generally four outcomes from `bind()`:

1.  The driver determines the device is supported and does not need to do any
    heavy lifting, so publishes a new device with `device_add()` in C or
    `ddk::Device::DdkAdd()` in the
    [DDKTL](/development/drivers/concepts/driver_development/using-ddktl.md)
    C++ wrapper library and returns `ZX_OK.

2.  The driver determines that even though the bind program matched, the device
    cannot be supported (maybe due to checking hw version bits or whatnot) and
    returns an error.

3.  The driver needs to do further initialization before the device is ready or
    it's sure it can support it, so it publishes a invisible device that
    implements the [`init()`](/src/lib/ddk/include/lib/ddk/device.h) hook and
    kicks off a thread to keep working, while returning `ZX_OK` to `bind()`.
    That thread will eventually call
    [`device_init_reply()`](/src/lib/ddk/include/lib/ddk/driver.h) in C or
    `ddk::InitTxn::Reply()` in the
    [DDKTL](/development/drivers/concepts/driver_development/using-ddktl.md)
    C++ wrapper library. The device is guaranteed not to be removed until the
    reply is received. The status indicates `ZX_OK` if it was able to
    successfully initialize the device and it should be made visible, or an
    error to indicate that the device should be removed.

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
with hardware (for example, through MMIO) or by communicating with its parent
device (for example, queueing a USB transaction).

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
[`zx_interrupt_wait()`](/reference/syscalls/interrupt_wait.md) will mask
the interrupt before returning and unmask the interrupt when it is called again
the next time. For edge-triggered interrupts, the interrupt remains unmasked.

The interrupt thread should not perform any long-running tasks. For drivers that
perform lengthy tasks, use a worker thread.

You can signal an interrupt handle with
[`zx_interrupt_trigger()`](/reference/syscalls/interrupt_trigger.md) on
slot `ZX_INTERRUPT_SLOT_USER` to return from `zx_interrupt_wait()`. This is
necessary to shut down the interrupt thread during driver clean up.

## FIDL messages

## Non-driver processes

Messages for each device class are defined in the
[FIDL](/development/languages/fidl/README.md) language. Each device
implements zero or more FIDL protocols, multiplexed over a single channel per
client. The driver is given the opportunity to interpret FIDL messages through
the `message()` hook. These are only accessible to non-driver components by
means of devfs.

## Drivers in other processes

If a driver needs to communicate with a driver in a separate process, rather
than define protocol ops, it must instead host an outgoing directory, similar to
components, which should host all FIDL protocols the child driver would access
on bind.

## Protocol ops vs. FIDL messages

Protocol ops define the in-process API for a device. FIDL messages define the
API for communicating out of process. Define a protocol op if the function is
meant to be called by other drivers in the same process. A driver should call a
protocol op on its parent to make use of those functions.

## Isolate devices

Devices that are added with `DEVICE_ADD_MUST_ISOLATE` spawn a new driver host.
The device must have an accompanying outgoing directory which hosts FIDL
protocols. A driver which is bound to the device will be loaded into the new
driver host and provided the ability to connect FIDL protocols exported in the
outgoing directory provided by the parent driver.

## Driver rights

Although drivers run in user space processes, they have a more restricted set of
rights than normal processes. Drivers are not allowed to access the filesystem,
including devfs. That means a driver cannot interact with arbitrary devices. If
your driver needs to do this, consider writing a service component instead. For
example, the virtual console is implemented by the
[virtcon](/src/bringup/bin/virtcon) component.

Privileged operations such as `zx_vmo_create_contiguous()` and
[`zx_interrupt_create`](/reference/syscalls/interrupt_create.md) require a
root resource handle. This handle is not available to drivers other than the
system driver ([ACPI](/src/devices/board/drivers/x86) on x86 systems and
[platform](/src/devices/bus/drivers/platform) on ARM systems). A device should
request its parent to perform such operations for it. Contact the author of the
parent driver if its protocol does not address this use case.

Similarly, a driver is not allowed to request arbitrary MMIO ranges, interrupts
or GPIOs. Bus drivers such as PCI and platform only return the resources
associated to the child device.

## Advanced Topics and Tips

### Taking a long time to initialize

What if your device takes a long time to initialize? When we discussed the
`null_bind()` function above, we indicated that a successful return told the
driver manager that the driver is now associated with the device. We can't spend
a lot of time in the bind function; we're basically expected to initialize our
device, publish it, and be done.

But your device might need to perform a lengthy initialization operation, such
as:

*   enumerate hardware points
*   load firmware
*   negotiate a protocol

and so on, which might take a long time to do.

You can publish your device as "invisible" by implementing the device `init()`
hook. The `init()` hook is run after the device is added through `device_add()`,
and may be used to safely access the device state and to spawn a worker thread.
The device will remain invisible and is guaranteed not to be removed until
`device_init_reply()` is called, which may be done from any thread. This meets
the requirements for the binding function, but nobody is able to use your device
(because nobody knows about it yet, because it's not visible). Now your device
can perform the long operations with a background thread.

When your device is ready to service client requests, call `device_init_reply()`
which will cause it to appear in the pathname space.

#### Power savings

Two callouts, `suspend()` and `resume()`, are available for your device in order
to support power or other resource saving features.

Both take a device context pointer and a flags argument, but the flags argument
is used only in the suspend case.

Flag                                    | Meaning
--------------------------------------- | -------
`DEVICE_SUSPEND_FLAG_REBOOT`            | The driver should shut itself down in preparation for a reboot or shutdown of the machine
`DEVICE_SUSPEND_FLAG_REBOOT_BOOTLOADER` | ?
`DEVICE_SUSPEND_FLAG_REBOOT_RECOVERY`   | ?
`DEVICE_SUSPEND_FLAG_POWEROFF`          | The driver should shut itself down in preparation for power off
`DEVICE_SUSPEND_FLAG_MEXEC`             | The driver should shut itself down in preparation for a [soft reboot](/reference/syscalls/system_mexec.md)
`DEVICE_SUSPEND_FLAG_SUSPEND_RAM`       | The driver should arrange so that it can be restarted from RAM

<!---
Yeah, I'm just guessing on the flags; they're used so little...

For documentation purposes, what should I write? That they are just hints, or
that you *must* do something because of a given flag, or ... ?
-->

## Reference: Support functions

This section lists support functions that are provided for your driver to use.

### Accessor functions

The context block that's passed as the first argument to your driver's protocol
functions is an opaque data structure. This means that in order to access the
data elements, you need to call an accessor function:

Function            | Purpose
------------------- | --------------------------------
`device_get_name()` | Retrieves the name of the device

### Administrative functions

The following functions are used to administer the device:

Function                | Purpose
----------------------- | ------------------------------------------------------
`device_add()`          | Adds a device to a parent
`device_async_remove()` | Schedules the removal of a device and all its children

<!---
@@@ Notes only @@@

This section is great for things like talking about buffer management,
threading, best practices, advanced options for device_add(), and so on.
I think it can be somewhere between the man page ("printf is used to print a string
and takes the following parameters") and an application note &mdash; I want to see
examples of how to use the functions, what the arguments mean, what the impact of
various design decisions is, that kind of thing.
-->
