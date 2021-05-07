# Device driver lifecycle

Device drivers are loaded into driver host processes when it is determined they are
needed. What determines if they are loaded or not is the binding program, which
is a description of what device a driver can bind to. The binding program is
defined using a small domain specific language, which is compiled to bytecode that
is distributed with the driver.


An example binding program from the Intel Ethernet driver:

```
fuchsia.device.protocol == fuchsia.pci.protocol.PCI_DEVICE;
fuchsia.pci.vendor == fuchsia.pci.vendor.INTEL;
accept fuchsia.pci.device {
    0x100E, // Qemu
    0x15A3, // Broadwell
    0x1570, // Skylake
    0x1533, // I210 standalone
    0x15b7, // Skull Canyon NUC
    0x15b8, // I219
    0x15d8, // Kaby Lake NUC
}
```

The bind compiler takes a binding program and outputs a C header file that
defines a macro, `ZIRCON_DRIVER`. The `ZIRCON_DRIVER` macro includes the
necessary compiler directives to put the binding program into an ELF NOTE
section, allowing it to be inspected by the Device Coordinator without needing
to fully load the driver into its process.

The second parameter to `ZIRCON_DRIVER` is a `zx_driver_ops_t` structure pointer
(defined by [`lib/ddk/driver.h`](/src/lib/ddk/include/lib/ddk/driver.h), which
defines the init, bind, create, and release methods.

`init()` is invoked when a driver is loaded into a Driver Host process and allows for
any global initialization. Typically none is required. If the `init()` method is
implemented and fails, the driver load will fail.

`bind()` is invoked to offer the driver a device to bind to. The device is one that
has matched the bind program the driver has published. If the `bind()` method succeeds,
the driver **must** create a new device and add it as a child of the device passed in
to the `bind()` method. See Device Lifecycle for more information.

`create()` is invoked for platform/system bus drivers or proxy drivers. For the
vast majority of drivers, this method is not required.

`release()` is invoked before the driver is unloaded, after all devices it may have
created in `bind()` and elsewhere have been destroyed. Currently this method is
**never** invoked. Drivers, once loaded, remain loaded for the life of a Driver Host
process.

# Device Lifecycle

Within a Driver Host process, devices exist as a tree of `zx_device_t` structures,
which are opaque to the driver. These are created with `device_add()`, which the
driver provides a `zx_protocol_device_t` structure to. The methods defined by the
function pointers in this structure are the "[device ops](device-ops.md)". The
various structures and functions are defined in [`device.h`](/src/lib/ddk/include/lib/ddk/device.h)

The `device_add()` function creates a new device, adding it as a child to the
provided parent device. That parent device **must** be either the device passed
in to the `bind()` method of a device driver, or another device which has been
created by the same device driver.

A side-effect of `device_add()` is that the newly created device will be added
to the global Device Filesystem maintained by the Device Coordinator. If the
device has not implemented an `init()` hook, the device will be immediately
accessible through opening its node in devfs.

The `init()` hook is invoked following `device_add()`. This is useful for
drivers that have to do extended initialization or probing and do not want
to visibly publish their device(s) until that succeeds (and quietly remove
them if that fails). The driver should call `device_init_reply()` once they
have completed initialization. This reply does not necessarily need to be
called from the `init()` hook. The device will remain invisible and is
guaranteed not to be removed until this point.

Devices are reference counted. A reference is acquired when a driver creates
the device with `device_add()` and when the device is opened by a remote process
through the Device Filesystem.

From the moment that `device_init_reply()` is called, or `device_add()` is called
without an implemented `init()` hook, other device ops may be called by the
Driver Host.

When `device_async_remove()` is called on a device, this schedules the removal
of the device and its descendents.

The removal of a device consists of four parts: running the device's `unbind()` hook,
removal of the device from the Device Filesystem, dropping the reference acquired
by `device_add()` and running the device's `release()` hook.

When the `unbind()` method is invoked, this signals to the driver it should start
shutting the device down, and call `device_unbind_reply()` once it has finished unbinding.
Unbind also acts as a hard barrier for FIDL transactions.
The FDF will not permit any new FIDL transactions or connections
to be created when Unbind is called. Drivers are responsible
for closing or replying to any outstanding transactions in their
unbind hook if they handle FIDL messages.
This is an optional hook. If it is not implemented, it is treated as `device_unbind_reply()`
was called immediately. When device_unbind_reply is called,
all FIDL connections will be terminated.

Since a child device may have work in progress when its `unbind()` method is
called, it's possible that the parent device (which already completed
unbinding) could continue to receive device method calls or protocol method
calls on behalf of that child. It is advisable that before completing unbinding,
the parent device should arrange for these methods to return errors, so that
calls from a child before the child removal is completed do not start more
work or cause unexpected interactions.

The `release()` method is only called after the creating driver has completed
unbinding, all open instances of that device have been closed,
and all children of that device have been unbound and released. This
is the last opportunity for the driver to destroy or free any resources associated
with the device. It is not valid to refer to the `zx_device_t` for that device
after `release()` returns. Calling any device methods or protocol methods for
protocols obtained from the parent device past this point is illegal and will
likely result in a crash.

## An Example of the Tear-Down Sequence

To explain how the `unbind()` and `release()` work during the tear-down process,
below is an example of how a USB WLAN driver would usually handle it. In short,
the `unbind()` call sequence is top-down while the `release()` sequence is bottom-up.

Note that this is just an example. This might not match what exactly the real WLAN driver
is doing.

Assume a WLAN device is plugged in as a USB device, and a PHY interface has been
created under the USB device. In addition to the PHY interface, 2 MAC interfaces
have been created under the PHY interface.

```
            +------------+
            | USB Device | .unbind()
            +------------+ .release()
                  |
            +------------+
            |  WLAN PHY  | .unbind()
            +------------+ .release()
              |        |
    +------------+  +------------+
    | WLAN MAC 0 |  | WLAN MAC 1 | .unbind()
    +------------+  +------------+ .release()
```

Now, we unplug this USB WLAN device.

* The USB XHCI detects the removal and calls `device_async_remove(usb_device)`.

* This will lead to the USB device's `unbind()` being called.
  Once it completes unbinding, it would call `device_unbind_reply()`.

```c
    usb_device_unbind(void* ctx) {
        // Stop interrupt or anything to prevent incoming requests.
        ...

        device_unbind_reply(usb_dev);
    }
```

* When the USB device completes unbinding, the WLAN PHY's `unbind()` is called.
  Once it completes unbinding, it would call `device_unbind_reply()`.

```c
    wlan_phy_unbind(void* ctx) {
        // Stop interrupt or anything to prevent incoming requests.
        ...

        device_unbind_reply(wlan_phy);
    }
```

* When wlan_phy completes unbinding, unbind() will be called on all of its children
  (wlan_mac_0, wlan_mac_1).

```c
    wlan_mac_unbind(void* ctx) {
        // Stop accepting new requests, and notify clients that this device is offline (often just
        // by returning an ZX_ERR_IO_NOT_PRESENT to any requests that happen after unbind).
        ...

        device_unbind_reply(iface_mac_X);
    }
```

* Once all the clients of a device have been removed, and that device has no children,
  its refcount will reach zero and its release() method will be called.

* WLAN MAC 0 and 1's `release()` are called.

```c
    wlan_mac_release(void* ctx) {
        // Release sources allocated at creation.
        ...

        // Delete the object here.
        ...
    }
```

* The wlan_phy has no open connections, but still has child devices (wlan_mac_0 and wlan_mac_1).
  Once they have both been released, its refcount finally reaches zero and its release()
  method is invoked.

```c
    wlan_phy_release(void* ctx) {
        // Release sources allocated at creation.
        ...

        // Delete the object here.
        ...
    }
```

* Once the USB device now has no child devices or open connections, its `release()` would be called.
