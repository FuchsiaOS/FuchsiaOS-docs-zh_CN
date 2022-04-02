<!---

# The Device ops

Please refer to the [header comments][device] for descriptions of the methods.

--->

# 设备操作

请参考[header comments][device] 中的方法描述。

<!---

## Hook ordering guarantees

![Hook ordering guarantees](/docs/images/zircon/ddk/driver-hook-ordering.png)

The hooks that a driver implements will be invoked by other drivers and by the
runtime. These invocations in some occasions may occur in parallel with
invocations of other or even the same hook. This section will describe the
ordering properties that you may rely on.

--->

## 钩子函数顺序保证

![Hook ordering guarantees](/docs/images/zircon/ddk/driver-hook-ordering.png)

驱动实现的钩子函数将被其他驱动和运行时调用。在某些使用场景中，这些调用会和其他或者甚至相同钩子函数同时发生。本节内将会描述你可能依赖的顺序属性。

<!---

### Terminology

This section uses the terms *unsequenced*, *indeterminately sequenced*, and
*sequenced before* as they are used in the C++ execution model.

--->

### 术语表

本节内使用术语 *乱序*，*不确定序列* 和 *序列前* 作为在 C++ 运行模型中使用。

<!---

### Driver Initialization

The [zx_driver_ops_t][driver] *init* hook will execute completely before any other
hooks for that driver.

--->

### 驱动初始化

 [zx_driver_ops_t ][driver] *init*  钩子函数将在驱动中其他任意钩子函数之前完成运行。

<!---

### Driver Teardown

The [zx_driver_ops_t][driver] *release* hook will begin execution only after all
devices created by this driver have been released.

--->

### 驱动卸载

[zx_driver_ops_t][driver] *release* 钩子函数仅在驱动创建的所有设备都完成释放后开始运行。

<!---

### Driver Bind

If tests are enabled, the [zx_driver_ops_t][driver] *bind* hook will begin execution only after the
run_unit_tests hook.

--->

### 驱动绑定

如果测试被打开， [zx_driver_ops_t ][driver]  *bind* 钩子函数将仅在 run_unit_tests 钩子函数后才开始运行。

<!---

### Device Lifecycle

The device lifecycle begins when some driver successfully invokes **device_add()**. This may
occur on any thread. No [zx_device_ops_t][device] hooks will run before the
device's lifecycle has begun or after it has ended.

The device lifecycle ends when the device's *release* hook has begun executing.

The [zx_device_ops_t][device] hooks are unsequenced with respect to each other
unless otherwise specified.

**Note**: This means that any code that occurs after a call to **device_add()**, even in *bind* hooks,
is unsequenced with respect to the end of the created device's lifecycle.

--->

### 设备生命周期

在驱动成功调用**device_add()**时，设备生命周期就此开始。这可能发生在任意线程中。在设备生命周期开始之前或者结束之后， [zx_device_ops_t][device] 钩子函数都不会运行。

设备生命周期在设备 *release* 钩子函数开始运行时结束。

除非另有规定，否则[zx_device_ops_t][device]钩子函数彼此之间是没有顺序的。

**注意**：这意味着调用**device_add()**后，任何代码都可能运行，甚至在*bind*钩子函数中，对于所创建的设备生命周期的结束也是没有顺序的。

<!---

### Device Connection Lifecycle

A device connection lifecycle begins when the [zx_device_ops_t][device] *open* hook begins
executing. None of the [zx_device_ops_t][device] *read*/*write*/*message*/*close* hooks
will be invoked if the number of alive device connections is 0.

A device connection lifecycle ends when the [zx_device_ops_t][device] *close* hook
begins executing. Any execution of *read*/*write*/*message* hooks is sequenced before
this.

Since the *read*/*write*/*message* hooks only execute on the driver host's main thread,
they will never be executed concurrently but the processing of outstanding requests from
different connections will be indeterminately sequenced.

--->

### 设备连接生命周期

当[zx_device_ops_t][device] *open*钩子函数开始运行时，设备连接生命周期开始。如果存活设备连接数为0的话，[zx_device_ops_t][device] 中 *read*/*write*/*message*/*close* 钩子函数都不会被调用。

当 [zx_device_ops_t][device] *close*钩子函数开始运行时，设备连接生命周期结束。任意*read*/*write*/*message* 钩子函数在此之前顺序运行。

因为*read*/*write*/*message*钩子函数只会在驱动主机的主线程中运行，它们将永远不会同时运行，但是从不同的连接中的外部请求处理将以不确定的序列执行。

<!---

### Misc Device APIs

The [zx_device_ops_t][device] *get_size* and *get_protocol* hooks are
unsequenced with respect to all hooks (including concurrent invocations of themselves).
The one exception to this is that they are sequenced before the *release* hook.

--->

### Misc设备API

[zx_device_ops_t][device] 的 *get_size*  和  *get_protocol* 钩子函数与所有钩子函数相比是乱序的（包括对自己的并发性调用）。
但是唯一例外为在 *release* 钩子函数之前被排序。

[device]: /src/lib/ddk/include/lib/ddk/device.h
[driver]: /src/lib/ddk/include/lib/ddk/driver.h

