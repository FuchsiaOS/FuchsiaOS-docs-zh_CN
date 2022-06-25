# Using the C++ DDK Template Library

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

In this section, we'll look at the C++ DDK Template Library, or "**DDKTL**" for short.
It's a set of C++ templated classes that simplify the work of writing a driver
by providing mixins that ensure type safety and perform basic functionality.

> If you're not familiar with mixins, you should read the Wikipedia articles on:
> * [mixins] and
> * [CRTPs &mdash; or Curiously Recurring Template Patterns][crtp].

The mixins that we'll be discussing are defined in
[`//src/lib/ddktl/include/ddktl/device.h`](/src/lib/ddktl/include/ddktl/device.h).

The following mixins are provided:

Mixin class            | Function             | Purpose
-----------------------|----------------------|------------------------------
`ddk::GetProtocolable`    | **DdkGetProtocol()** | fetches the protocol
`ddk::Initializable`      | **DdkInit()**        | called after **DdkAdd()**, for completing initialization of a device safely
`ddk::Openable`           | **DdkOpen()**        | client's **open()**
`ddk::Closable`           | **DdkClose()**       | client's **close()**
`ddk::Unbindable`         | **DdkUnbind()**      | called when this device is being removed
`ddk::Suspendable`        | **DdkSuspend()**     | to suspend device
`ddk::Resumable`          | **DdkResume()**      | to resume device
`ddk::PerformanceTunable` | **DdkSetPerformanceState()**   | to transition the performant state
`ddk::AutoSuspendable`    | **DdkConfigureAutoSuspend()**   | to configure whether a driver can auto suspend the device
`ddk::Rxrpcable`          | **DdkRxrpc()**       | remote messages for bus devices
`ddk::MessageableManual`  | **DdkMessage()**     | for FIDL IPC messages

For completeness, the following mixins are also provided, but have been deprecated:

Deprecated Mixin class      | Function             | Purpose
----------------------------|----------------------|------------------------------
`ddk::Readable`             | **DdkRead()**        | client's **read()**
`ddk::Writable`             | **DdkWrite()**       | client's **write()**
`ddk::GetSizable`           | **DdkGetSize()**     | returns size of device
`ddk::UnbindableDeprecated` | **DdkUnbindDeprecated()**   | called when this device is being removed

When defining the class for your device, you specify which functions it will
support by including the appropriate mixins.
For example (line numbers added for documentation purposes only):

```c++
[01] using DeviceType = ddk::Device<MyDevice,
[02]                                ddk::Initializable,   // safely initialize after **DdkAdd()**
[03]                                ddk::Openable,        // we support open()
[04]                                ddk::Closable,        // close()
[05]                                ddk::Readable,        // read()
[06]                                ddk::Unbindable>;     // and the device can be unbound
```

This creates a shortcut to `DeviceType`.
The `ddk::Device` templated class takes one or more arguments, with the
first argument being the base class (here, `MyDevice`).
The additional template arguments are the mixins that define
which FDF device member functions are implemented.

Once defined, we can then declare our device class (`MyDevice`) as inheriting
from `DeviceType`:

```c++
[07] class MyDevice : public DeviceType {
[08]   public:
[09]     explicit MyDevice(zx_device_t* parent)
[10]       : DeviceType(parent) {}
[11]
[12]     zx_status_t Bind() {
[13]         // Any other setup required by MyDevice.
[14]         // The device_add_args_t will be filled out by the base class.
[15]         return DdkAdd("my-device-name");
[16]     }
[17]
[18]     // Methods required by the ddk mixins
[19]     void DdkInit(ddk::InitTxn txn);
[20]     zx_status_t DdkOpen(zx_device_t** dev_out, uint32_t flags);
[21]     zx_status_t DdkClose(uint32_t flags);
[22]     zx_status_t DdkRead(void* buf, size_t count, zx_off_t off, size_t* actual);
[23]     void DdkUnbind(ddk::UnbindTxn txn);
[24]     void DdkRelease();
[25] };
```

Because the `DeviceType` class contains five mixins (lines `[02` .. `06]`: `Initializable`,
`Openable`, `Closable`, `Readable`, and `Unbindable`), we're required to provide
the respective function implementations (lines `[18` .. `23]`)
in our class.

All DDKTL classes must provide a release function (here, line `[24]` provides
**DdkRelease()**), so that's why we didn't specify this in the mixin definition
for `DeviceType`.

> Keep in mind that once you reply to the `InitTxn` (provided in **DdkInit()**)
> you _cannot_ safely use the device instance &mdash; other threads may call
> **DdkUnbind()**, which typically calls **DdkRelease()**, and that frees the driver's
> device context. This would constitute a "use-after-free" violation.
> For devices that do not implement **DdkInit()**, this would apply after you call **DdkAdd()**.

Recall from the preceding sections that your device must register with the driver manager
in order to be usable.
This is accomplished as follows:

```c++
[26] zx_status_t my_bind(zx_device_t* device,
[27]                     void** cookie) {
[28]     auto dev = std::make_unique<MyDevice>(device);
[29]     auto status = dev->Bind();
[30]     if (status == ZX_OK) {
[31]         // driver manager is now in charge of the memory for dev
[32]         dev.release();
[33]     }
[34]     return status;
[35] }
```

Here, **my_bind()** creates an instance of `MyDevice`, calls the **Bind()** routine,
and then returns a status.

**Bind()** (line `[12]` in the `class MyDevice` declaration above), performs whatever
setup it needs to, and then calls **DdkAdd()** with the device name.

Since the device is `Initializable`, the driver manager will then call your implementation
of **DdkInit()** with an `InitTxn`. The device will be invisible and not able to be
unbound until the device replies to the `InitTxn`. This reply can be done from any
thread &mdash; it does not necessarily need to be before returning from **DdkInit()**.

After replying to the `InitTxn`, your device will be visible in the Device filesystem,
and any **open()**, **close()**, and **read()** client calls
will now flow to your implementations of **DdkOpen()**, **DdkClose()**,
and **DdkRead()**, respectively.

As an example, in the directory [`//src/devices/block/drivers/zxcrypt`](/src/devices/block/drivers/zxcrypt)
we have a typical device declaration ([`device.h`](/src/devices/block/drivers/zxcrypt/device.h)):

```c++
[01] class Device;
[02] using DeviceType = ddk::Device<Device,
[03]                                ddk::GetProtocolable,
[04]                                ddk::GetSizable,
[05]                                ddk::Unbindable>;
...
[06] class Device final : public DeviceType,
[07]                      public ddk::BlockImplProtocol<Device, ddk::base_protocol>,
[08]                      public ddk::BlockPartitionProtocol<Device>,
[09]                      public ddk::BlockVolumeProtocol<Device> {
[10] public:
...
[11]     // ddk::Device methods; see ddktl/device.h
[12]     zx_status_t DdkGetProtocol(uint32_t proto_id, void* out);
[13]     zx_off_t DdkGetSize();
[14]     void DdkUnbind(ddk::UnbindTxn txn);
[15]     void DdkRelease();
...
```

Lines `[01` .. `05]` declare the shortcut `DeviceType` with the base class
`Device` and three mixins, `GetProtocolable`, `GetSizable`, and `Unbindable`.

What's interesting here is line `[06]`: we not only inherit from the `DeviceType`,
but also from other classes on lines `[07` .. `09]`.

Lines `[11` .. `15]` provide the prototypes for the three optional mixins and the
mandatory **DdkRelease()** member function.

Here's an example of the `zxcrypt` device's `DdkGetProtocol` implementation (from
[`device.cc`](/src/devices/block/drivers/zxcrypt/device.cc)):

```c++
zx_status_t Device::DdkGetProtocol(uint32_t proto_id, void* out) {
    auto* proto = static_cast<ddk::AnyProtocol*>(out);
    proto->ctx = this;
    switch (proto_id) {
    case ZX_PROTOCOL_BLOCK_IMPL:
        proto->ops = &block_impl_protocol_ops_;
        return ZX_OK;
    case ZX_PROTOCOL_BLOCK_PARTITION:
        proto->ops = &block_partition_protocol_ops_;
        return ZX_OK;
    case ZX_PROTOCOL_BLOCK_VOLUME:
        proto->ops = &block_volume_protocol_ops_;
        return ZX_OK;
    default:
        return ZX_ERR_NOT_SUPPORTED;
    }
}
```

# As seen in a driver

Let's take a look at how a driver uses the DDKTL.

We're going to use the USB XHCI driver for this set of code samples; you can find it
[here: `//src/devices/usb/drivers/xhci/usb-xhci.cpp`](/src/devices/usb/drivers/xhci/usb-xhci.cc).

Drivers have a driver declaration (usually at the bottom of the source file), like this:

```c
ZIRCON_DRIVER(driver_name, driver_ops, "zircon", "0.1");
```

The second parameter to the **ZIRCON_DRIVER()** macro is a `zx_driver_ops_t` structure.
In the C++ version we use a lambda function to help with initialization:

```c++
namespace usb_xhci {
...
static zx_driver_ops_t driver_ops = [](){
    zx_driver_ops_t ops = {};
    ops.version = DRIVER_OPS_VERSION;
    ops.bind = UsbXhci::Create;
    return ops;
}();

} // namespace usb_xhci

ZIRCON_DRIVER(usb_xhci, usb_xhci::driver_ops, "zircon", "0.1");
```

This executes the **driver_ops()** lambda, which returns an initialized `zx_driver_ops_t` structure.
Why the lambda? C++ doesn't like partial initialization of structures, so we start with an
empty instance of `ops`, set the fields we're interested in, and then return the structure.

The **UsbXhci::Create()** function is as follows:

```c++
[01] zx_status_t UsbXhci::Create(void* ctx, zx_device_t* parent) {
[02]     fbl::AllocChecker ac;
[03]     auto dev = std::unique_ptr<UsbXhci>(new (&ac) UsbXhci(parent));
[04]     if (!ac.check()) {
[05]         return ZX_ERR_NO_MEMORY;
[06]     }
[07]
[08]     auto status = dev->Init();
[09]     if (status != ZX_OK) {
[10]         return status;
[11]     }
[12]
[13]     // driver manager is now in charge of the device.
[14]     __UNUSED auto* unused = dev.release();
[15]     return ZX_OK;
[16] }
```

First, note the constructor for `dev` (it's the `new ... UsbXhci(parent)` call
on line `[03]`) &mdash; we'll come back to it shortly.

Once `dev` is constructed, line `[08]` calls **dev->Init()**, which serves as
a de-multiplexing point calling one of two initialization functions:

```c++
zx_status_t UsbXhci::Init() {
    if (pci_.is_valid()) {
        return InitPci();
    } else if (pdev_.is_valid()) {
        return InitPdev();
    } else {
        return ZX_ERR_NOT_SUPPORTED;
    }
}
```

## Parent protocol usage

Let's follow the path of the `pci_` member by way of the **InitPci()** function.
We'll see how the device uses the functions from the parent protocol.

In **UsbXhci::Create()** the constructor for `dev` initialized the member `pci_`
from the `parent` argument.
Here are the relevant excerpts from the class definition:

```c++
class UsbXhci: ... {
public:
    explicit UsbXhci(zx_device_t* parent)
        : UsbXhciType(parent), pci_(parent), pdev_(parent) {}
...
prviate:
    ddk::PciProtocolClient pci_;
...
};
```

The first use that **InitPci()** makes of the `pci_` member is to get a
[**BTI** (Bus Transaction Initiator)][bti] object:

```c++
zx_status_t UsbXhci::InitPci() {
...
    zx::bti bti;
    status = pci_.GetBti(0, &bti);
    if (status != ZX_OK) {
        return status;
    }
    ...
```

This usage is typical.

<!-- xref table -->
[bti]: reference/kernel_objects/bus_transaction_initiator.md
[crtp]: https://en.wikipedia.org/wiki/Curiously_recurring_template_pattern
[dev/block/zxcrypt/device.cpp]: /src/devices/block/drivers/zxcrypt/device.cc
[dev/block/zxcrypt/device.h]: /src/devices/block/drivers/zxcrypt/device.h
[dev/block/zxcrypt]: /src/devices/block/drivers/zxcrypt
[include/ddktl/device.h]: /src/lib/ddktl/include/ddktl/device.h
[mixins]: https://en.wikipedia.org/wiki/Mixin
[usb-xhci.cc]: /src/devices/usb/drivers/xhci/usb-xhci.cc
