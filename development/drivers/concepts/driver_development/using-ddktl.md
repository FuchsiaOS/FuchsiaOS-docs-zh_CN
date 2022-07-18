<!---

# Using the C++ DDK Template Library

In this section, we'll look at the C++ DDK Template Library, or "**DDKTL**" for short.
It's a set of C++ templated classes that simplify the work of writing a driver
by providing mixins that ensure type safety and perform basic functionality.

> If you're not familiar with mixins, you should read the Wikipedia articles on:
> * [mixins] and
> * [CRTPs &mdash; or Curiously Recurring Template Patterns][crtp].

The mixins that we'll be discussing are defined in
[`//src/lib/ddktl/include/ddktl/device.h`](/src/lib/ddktl/include/ddktl/device.h).

The following mixins are provided:

--->

# C++ DDK 模板库使用指南

在本章中，我们将关注  C++ DDK 模板库，或者简短了解“**DDKTL**”。
它是一系列的 C++ 模板类，通过提供多态来确保类型安全和运行基本功能，用来简化写驱动程序的工作。

> 如果你不熟悉多态的话，你可以阅读相关维基百科文章：
> * [mixins] and
> * [CRTPs &mdash; or Curiously Recurring Template Patterns][crtp].

我们将要讨论的多态在[`//src/lib/ddktl/include/ddktl/device.h`](/src/lib/ddktl/include/ddktl/device.h)中定义。

我们将提供下述的多态：

Mixin class            | Function             | Purpose
-----------------------|----------------------|------------------------------
`ddk::GetProtocolable`    | **DdkGetProtocol()** | fetches the protocol
`ddk::Initializable`      | **DdkInit()**        | called after **DdkAdd()**, for completing initialization of a device safely
`ddk::Openable`           | **DdkOpen()**        | client's **open()**
`ddk::Closable`           | **DdkClose()**       | client's **close()**
`ddk::Unbindable`         | **DdkUnbind()**      | called when this device is being removed
`ddk::Messageable`        | **DdkMessage()**     | for FIDL IPC messages
`ddk::Suspendable`        | **DdkSuspend()**     | to suspend device
`ddk::Resumable`          | **DdkResume()**      | to resume device
`ddk::PerformanceTunable` | **DdkSetPerformanceState()**   | to transition the performant state
`ddk::AutoSuspendable`    | **DdkConfigureAutoSuspend()**   | to configure whether a driver can auto suspend the device
`ddk::Rxrpcable`          | **DdkRxrpc()**       | remote messages for bus devices

<!---

For completeness, the following mixins are also provided, but have been deprecated:

--->

为了完整性期间，下述的多态也同样会提供，但是已经被废弃：

Deprecated Mixin class      | Function             | Purpose
----------------------------|----------------------|------------------------------
`ddk::Readable`             | **DdkRead()**        | client's **read()**
`ddk::Writable`             | **DdkWrite()**       | client's **write()**
`ddk::GetSizable`           | **DdkGetSize()**     | returns size of device
`ddk::UnbindableDeprecated` | **DdkUnbindDeprecated()**   | called when this device is being removed

<!---

These mixins correspond to the functions defined in the
[`zx_protocol_device_t`](/src/lib/ddk/include/lib/ddk/device.h#74) struct
that is used in the [simple, C-based drivers](/docs/development/drivers/developer_guide/simple.md).

When defining the class for your device, you specify which functions it will
support by including the appropriate mixins.
For example (line numbers added for documentation purposes only):

--->

这些多态对应的函数被定义在[`zx_protocol_device_t`](/src/lib/ddk/include/lib/ddk/device.h#74)结构体中，用于 [简单的, 基于C语言的驱动程序](/docs/development/drivers/developer_guide/simple.md)中。

当定义你的设备类时，你需要明确哪个函数将通过包含适合的多态来支持。
例如（添加的行号仅用于文件编制）：

```c++
[01] using DeviceType = ddk::Device<MyDevice,
[02]                                ddk::Initializable,   // safely initialize after **DdkAdd()**
[03]                                ddk::Openable,        // we support open()
[04]                                ddk::Closable,        // close()
[05]                                ddk::Readable,        // read()
[06]                                ddk::Unbindable>;     // and the device can be unbound
```

<!---

This creates a shortcut to `DeviceType`.
The `ddk::Device` templated class takes one or more arguments, with the
first argument being the base class (here, `MyDevice`).
The additional template arguments are the mixins that define
which FDF device member functions are implemented.

Once defined, we can then declare our device class (`MyDevice`) as inheriting
from `DeviceType`:

--->

这将创建一个`DeviceType`的快捷方式。
`ddk::Device`模板类输入一个或多个参数，第一个参数为基础类（这里则为`MyDevice`）。
额外的模板参数则为多态，定义了哪个 FDF 设备成员函数被实现。

一旦被定义后，接下来我们可以从`DeviceType`的继承，声明我们的设备类（`MyDevice`）：

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

<!---

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

--->

因为 `DeviceType` 类包含五个多态（ `[02` .. `06]`行：`Initializable`, `Openable`, `Closable`, `Readable` 和`Unbindable`），我们需要在自己的类中提供各自的函数实现（ `[18` .. `23]`行）。

所有 DDKTL 类必须提供一个 release 函数（这里为`[24]`行中提供的**DdkRelease()**），这也就是为什么我们没有在`DeviceType`内的多态定义中声明的原因。

> 请记住，一旦你回复了`InitTxn`（在**DdkInit()**中提供），你 _不能_ 安全的使用设备实例—因为其他的线程可能调用**DdkUnbind()**，它通常会调用**DdkRelease()**，就会释放驱动的设备上下文。这样会构成“释放后再使用”的违例。对于没有实现**DdkInit()**的设备，在你调用**DdkAdd()**后适用。

从前面的章节中可以看出，你的设备必须在驱动管理器中注册，才能使用。
具体做法如下：

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

<!---

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

--->

在这里的代码中，**my_bind()**创建了一个`MyDevice`的实例，调用**Bind()**例行程序，然后返回一个状态。

**Bind()**（上述`class MyDevice` 声明中`[12]`行），执行它需要的任何设置，然后使用设备名称调用**DdkAdd()**。

因为设备是`Initializable`的，接下来驱动管理器将使用`InitTxn`调用你的**DdkInit()** 实现。在设备回复`InitTxn`前，设备都保持不可见并且不可被绑定状态。这个回复可以在任意的线程中完成—它不一定需要在**DdkInit()**返回之前。

在回复了`InitTxn`之后，你的设备将在设备文件系统中变得可见，并且客户端调用的任何 **open()**, **close()**,和**read()**都将分别运行在你的**DdkOpen()**, **DdkClose()** 和**DdkRead()**的实现中。

正如在目录[`//src/devices/block/drivers/zxcrypt`](/src/devices/block/drivers/zxcrypt)中的示例，我们提供了一个典型的设备声明（[`device.h`](/src/devices/block/drivers/zxcrypt/device.h)）。

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

<!---

Lines `[01` .. `05]` declare the shortcut `DeviceType` with the base class
`Device` and three mixins, `GetProtocolable`, `GetSizable`, and `Unbindable`.

What's interesting here is line `[06]`: we not only inherit from the `DeviceType`,
but also from other classes on lines `[07` .. `09]`.

Lines `[11` .. `15]` provide the prototypes for the three optional mixins and the
mandatory **DdkRelease()** member function.

Here's an example of the `zxcrypt` device's `DdkGetProtocol` implementation (from
[`device.cc`](/src/devices/block/drivers/zxcrypt/device.cc)):

--->

`[01` .. `05]`行中使用基础类`Device`和三个多态 `GetProtocolable`, `GetSizable`和`Unbindable`声明了 `DeviceType`的快捷方式。

有趣的是在`[06]`行中：我们不仅是从`DeviceType`继承，也同样从`[07` .. `09]`行中的其他类继承。

 `[11` .. `15]`行中提供了三个可选多态的原型，和一个必备的**DdkRelease()**成员函数。

下面是一个`zxcrypt`设备的`DdkGetProtocol`实现的示例（从[`device.cc`](/src/devices/block/drivers/zxcrypt/device.cc)节选）：

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

<!---

# As seen in a driver

Let's take a look at how a driver uses the DDKTL.

We're going to use the USB XHCI driver for this set of code samples; you can find it
[here: `//src/devices/usb/drivers/xhci/usb-xhci.cpp`](/src/devices/usb/drivers/xhci/usb-xhci.cc).

Recall that in [simple, C-based drivers](/docs/development/drivers/developer_guide/simple.md),
the drivers had a driver declaration (usually at the bottom of the source file), like this:

--->

# 从驱动程序出发

让我们来看看驱动程序是如何使用 DDKTL 。

我们将用这组代码示例来使用 USB XHCI 驱动程序；你可以在[here: `//src/devices/usb/drivers/xhci/usb-xhci.cpp`](/src/devices/usb/drivers/xhci/usb-xhci.cc)中找到。

在 [simple, C-based drivers](/docs/development/drivers/developer_guide/simple.md)中回顾的话，
驱动程序有这样的驱动声明（通常在源代码文件的底部），正如这样：

```c
ZIRCON_DRIVER(driver_name, driver_ops, "zircon", "0.1");
```

<!---

The second parameter to the **ZIRCON_DRIVER()** macro is a `zx_driver_ops_t` structure.
In the C++ version we use a lambda function to help with initialization:

--->

对于 **ZIRCON_DRIVER()**宏定义来讲，第二个参数是一个`zx_driver_ops_t`结构体。
在 C++ 版本中，我们使用一个 lambda 函数来进行初始化：

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

<!---

This executes the **driver_ops()** lambda, which returns an initialized `zx_driver_ops_t` structure.
Why the lambda? C++ doesn't like partial initialization of structures, so we start with an
empty instance of `ops`, set the fields we're interested in, and then return the structure.

The **UsbXhci::Create()** function is just like its C counterpart (e.g., **null_bind()**
from the [Simple Drivers](/docs/development/drivers/developer_guide/simple.md) section),
but with a few extras:

--->

这里运行**driver_ops()** 的 lambda 函数，它返回了一个已经初始化的`zx_driver_ops_t` 结构体。
为什么要使用 lambda 呢？ C++ 中不喜欢部分初始化结构，所以我们以一个`ops`的空实例开始，设置我们感兴趣的字段，然后返回结构体。

**UsbXhci::Create()** 函数就像它对应的 C 的部分（例如在[Simple Drivers](/docs/development/drivers/developer_guide/simple.md)章节中的**null_bind()**），
但是有几个额外的参数：

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
[14]     __UNUSED auto* dummy = dev.release();
[15]     return ZX_OK;
[16] }
```

<!---

First, note the constructor for `dev` (it's the `new ... UsbXhci(parent)` call
on line `[03]`) &mdash; we'll come back to it shortly.

Once `dev` is constructed, line `[08]` calls **dev->Init()**, which serves as
a de-multiplexing point calling one of two initialization functions:

--->

首先，注意`dev`的构造函数（它是在`[03]`行`new ... UsbXhci(parent)`调用）— 我们稍微会回到这个话题。

一旦`dev`被构造，`[08]`行调用**dev->Init()**，它作为一个去复用点，调用两个初始化函数中的一个：

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

<!---

## Parent protocol usage

Let's follow the path of the `pci_` member by way of the **InitPci()** function.
We'll see how the device uses the functions from the parent protocol.

In **UsbXhci::Create()** the constructor for `dev` initialized the member `pci_`
from the `parent` argument.
Here are the relevant excerpts from the class definition:

--->

## 父协议的使用

让我们通过**InitPci()**函数来跟踪 `pci_`成员的路径。
我们将看到设备是怎样在父协议中使用这些函数的。

在**UsbXhci::Create()**中，`dev`的构造函数从`parent`参数中初始化了`pci_`成员。
以下是类型定义中的相关摘录：

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

<!---

The first use that **InitPci()** makes of the `pci_` member is to get a
[**BTI** (Bus Transaction Initiator)][bti] object:

--->

**InitPci()**对`pci_`成员的第一次使用是为了获得一个[**BTI** (Bus Transaction Initiator)][bti] 对象。

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

<!---

This usage is typical.

--->

这是一种典型用法。

<!-- xref table -->
[bti]: /docs/reference/kernel_objects/bus_transaction_initiator.md
[crtp]: https://en.wikipedia.org/wiki/Curiously_recurring_template_pattern
[dev/block/zxcrypt/device.cpp]: /src/devices/block/drivers/zxcrypt/device.cc
[dev/block/zxcrypt/device.h]: /src/devices/block/drivers/zxcrypt/device.h
[dev/block/zxcrypt]: /src/devices/block/drivers/zxcrypt
[include/ddktl/device.h]: /src/lib/ddktl/include/ddktl/device.h
[mixins]: https://en.wikipedia.org/wiki/Mixin
[usb-xhci.cc]: /src/devices/usb/drivers/xhci/usb-xhci.cc
