<!---

# Device driver lifecycle

Device drivers are loaded into driver host processes when it is determined they are
needed. What determines if they are loaded or not is the binding program, which
is a description of what device a driver can bind to. The binding program is
defined using a small domain specific language, which is compiled to bytecode that
is distributed with the driver.

An example binding program from the Intel Ethernet driver:

--->

# 设备驱动生命周期

当需要时，设备驱动被加载到驱动主机进程中。决定它们是否加载取决于绑定程序，它是一个设备驱动绑定关系的描述。绑定程序使用一个小范围定义语言来定义，然后编译成比特码分发到驱动中。

下述为一个网络以太网驱动的绑定程序示例：

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

<!---

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

--->

绑定编译器获取到绑定程序后以C头文件定义宏`ZIRCON_DRIVER`的形式输出。`ZIRCON_DRIVER`宏中包含必要的编译指令来放置绑定程序到 ELF NOTE 章节中，并允许设备协调程序在不需要完全加载设备到进程中就完成检查。

`ZIRCON_DRIVER`第二个参数是一个 `zx_driver_ops_t` 结构体指针（在 [`lib/ddk/driver.h`](/src/lib/ddk/include/lib/ddk/driver.h)中，并定义了init，bind，create和release的接口）。

`init()` 在驱动加载到驱动主机进程时被调用，允许任意全局初始化。通常情况下不需要。如果`init()`方法执行失败，驱动加载则会失败。

`bind()`在提供设备和驱动绑定时被调用。该设备是一个与驱动程序发布绑定程序相匹配的设备。如果`bind()`方法调用成功，驱动**必须**创建一个新的设备，并将其作为传递给`bind()`方法的设备子设备。详情参见设备生命周期。

`create()` 被平台/系统总线设备或代理驱动调用。对于绝大多数设备，这个方法是没有必要的。

`release()`在驱动卸载之前，`bind()` 和其他地方创建所有的所有设备被销毁之后被调用。现在这个方法**再也不**调用。因为驱动一旦加载，就会在驱动主机进程生命周期中一直保持加载状态。

<!---

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

--->

# 设备生命周期

在驱动主机进程中，设备以 `zx_device_t` 结构体树形式存在，并对驱动不可见。驱动提供 `zx_protocol_device_t`结构体用`device_add()`方法创建设备。该方法在该结构体中以”[device ops](device-ops.md)“函数指针的形式定义。更多结构体和函数定义参见[`device.h`](/src/lib/ddk/include/lib/ddk/device.h)。

`device_add()` 函数创建了一个新的设备，并作为一个子设备添加到提供的父设备中。父设备**必须**是要么通过`bind()` 传递到设备驱动的设备，要么是另一个被同样设备驱动创建的设备。

`device_add()`的副作用则是新创建的设备将被添加到由设备协调程序维护的全局设备文件系统中。如果这个设备没有实现`init()` 钩函数，那么设备将在通过 devfs 打开节点时而被立即访问。

`init()`钩函数在 `device_add()`被调用。对驱动来说，必须经过扩展初始化或者检查，并且不想在设置成功（如果失败则悄悄移除）之前对外部不可见是非常有用的。驱动在它们完成初始化时需要调用`device_init_reply()`。这个回复不一定需要从`init()`钩函数中调用。而设备将保持不可见，并保证在这之前不会被移除。

设备是参考计数的。当驱动使用`device_add()`创建设备和设备被远端进程通过设备文件系统打开时，就会获得一个引用。

当调用`device_init_reply()`， 或者没有实现`init()`钩函数而调用`device_add()` 的那一刻起，其他设备操作就可以被驱动主机调用。

<!---

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

--->

当 `device_async_remove()`在设备上被调用时，这就会规划移除该设备及其子设备。

移除设备包含4个步骤：运行设备`unbind()`钩函数，从设备文件系统移除设备，移除`device_add()`获取的引用和运行设备`release()`钩函数。

当`unbind()`方法被调用时，这标志着驱动应当开始关闭设备，然后在完成解绑时调用`device_unbind_reply()`。

解绑也作为 FIDL 传输的一个硬屏障。

当解绑被调用后， FDF 将不再允许创建任何新的 FIDL 传输或者连接。如果驱动处理 FIDL 消息，则需要负责关闭或回复在unbind函数中的任何未完成的工作。

这是一个可选的钩函数。如果没有实现它的话，将被视为 `device_unbind_reply()`立即调用。当device_unbind_reply 被调用时，所有 FIDL 连接都被中断。

因为子设备可能在`unbind()` 被调用时在进程内运行，所以父设备有可能（已经完全解绑）可以代表它的子设备继续接收设备方法调用或者协议方法调用。建议在完成解绑之前，父设备就应该安排这些方法返回错误，这样，在子设备移除完成之前，来自子设备的调用就不会启动更多的任务或导致意外的交互。

`release()`方法仅在创建驱动完成解绑后调用，所有设备的打开实例都被关闭，并且所有子设备已经解绑和销毁。这是驱动销毁或者释放设备相关资源的最后机会。在`release()`返回后，引用该设备的`zx_device_t` 结构体是无效的。调用任意的设备方法或从父设备获取协议方法传递指针是非法的，这很可能导致崩溃。

<!---

## An Example of the Tear-Down Sequence

To explain how the `unbind()` and `release()` work during the tear-down process,
below is an example of how a USB WLAN driver would usually handle it. In short,
the `unbind()` call sequence is top-down while the `release()` sequence is bottom-up.

Note that this is just an example. This might not match what exactly the real WLAN driver
is doing.

Assume a WLAN device is plugged in as a USB device, and a PHY interface has been
created under the USB device. In addition to the PHY interface, 2 MAC interfaces
have been created under the PHY interface.

--->

## 销毁顺序示例

为了解释说明`unbind()`和`release()`在销毁过程中是怎样工作的，下述是一个在 USB WLAN 驱动中处理的示例。简而言之，`unbind()`是调用顺序的最上层， `release()` 则是顺序的最底层。

注意，这仅仅是一个示例。这与实际 WLAN 驱动怎样工作可能不一致。

假设一个 WLAN 设备作为 USB 设备插入，然后在 USB 设备下已经创建一个 PHY 接口。除了 PHY 接口，2个 MAC 接口也会在 PHY 接口下被创建。

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

<!---

Now, we unplug this USB WLAN device.

* The USB XHCI detects the removal and calls `device_async_remove(usb_device)`.

* This will lead to the USB device's `unbind()` being called.
  Once it completes unbinding, it would call `device_unbind_reply()`.

--->
  现在，我们要拔出这个 USB WLAN 设备

*  USB XHCI 监测到设备移除并调用`device_async_remove(usb_device)`。

*  这将导致USB 设备中`unbind()`函数将被调用。

  一旦完成解绑，它会调用`device_unbind_reply()`。

```c
    usb_device_unbind(void* ctx) {
        // Stop interrupt or anything to prevent incoming requests.
        ...

        device_unbind_reply(usb_dev);
    }
```

<!---
* When the USB device completes unbinding, the WLAN PHY's `unbind()` is called.
  Once it completes unbinding, it would call `device_unbind_reply()`.

--->
* 当 USB 设备完成解绑后， WLAN PHY `unbind()`函数被调用。
  一旦完成解绑，它会调用`device_unbind_reply()`。
```c
    wlan_phy_unbind(void* ctx) {
        // Stop interrupt or anything to prevent incoming requests.
        ...

        device_unbind_reply(wlan_phy);
    }
```
<!---
* When wlan_phy completes unbinding, unbind() will be called on all of its children
  (wlan_mac_0, wlan_mac_1).

--->
*  当 wlan_phy 完成解绑后，unbind() 将在它所有的子设备上被调用。
  (wlan_mac_0, wlan_mac_1).

```c
    wlan_mac_unbind(void* ctx) {
        // Stop accepting new requests, and notify clients that this device is offline (often just
        // by returning an ZX_ERR_IO_NOT_PRESENT to any requests that happen after unbind).
        ...

        device_unbind_reply(iface_mac_X);
    }
```
<!---

* Once all the clients of a device have been removed, and that device has no children,
  its refcount will reach zero and its release() method will be called.

* WLAN MAC 0 and 1's `release()` are called.

--->
*  一旦所有设备的客户端都完成移除后，设备就没有子设备了，
  它的 refcount 将被清零，并且release()方法被调用。
*  WLAN MAC 0和1的`release()`被调用。

```c
    wlan_mac_release(void* ctx) {
        // Release sources allocated at creation.
        ...

        // Delete the object here.
        ...
    }
```
<!---
* The wlan_phy has no open connections, but still has child devices (wlan_mac_0 and wlan_mac_1).
  Once they have both been released, its refcount finally reaches zero and its release()
  method is invoked.

--->

* wlan_phy 没有打开的连接，但是依然有子设备（ wlan_mac_0 和 wlan_mac_1 ）。

  一旦它们都被释放后，它的 refcount 最终被清零，并且 release() 方法被调用。


```c
    wlan_phy_release(void* ctx) {
        // Release sources allocated at creation.
        ...

        // Delete the object here.
        ...
    }
```

<!---
* Once the USB device now has no child devices or open connections, its `release()` would be called.

--->

* 一旦 USB 设备现在已经没有子设备或者打开的连接，它的 `release()` 接口将被调用。
