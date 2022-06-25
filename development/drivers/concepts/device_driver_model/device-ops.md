# The Device ops

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

Please refer to the [header comments][device] for descriptions of the methods.

## Hook ordering guarantees

![Hook ordering guarantees](/docs/images/zircon/ddk/driver-hook-ordering.png)

The hooks that a driver implements will be invoked by other drivers and by the
runtime. These invocations in some occasions may occur in parallel with
invocations of other or even the same hook. This section will describe the
ordering properties that you may rely on.

### Terminology

This section uses the terms *unsequenced*, *indeterminately sequenced*, and
*sequenced before* as they are used in the C++ execution model.

### Driver Initialization

The [zx_driver_ops_t][driver] *init* hook will execute completely before any other
hooks for that driver.

### Driver Teardown

The [zx_driver_ops_t][driver] *release* hook will begin execution only after all
devices created by this driver have been released.

### Driver Bind

If tests are enabled, the [zx_driver_ops_t][driver] *bind* hook will begin execution only after the
run_unit_tests hook.

### Device Lifecycle

The device lifecycle begins when some driver successfully invokes **device_add()**. This may
occur on any thread. No [zx_device_ops_t][device] hooks will run before the
device's lifecycle has begun or after it has ended.

The device lifecycle ends when the device's *release* hook has begun executing.

The [zx_device_ops_t][device] hooks are unsequenced with respect to each other
unless otherwise specified.

**Note**: This means that any code that occurs after a call to **device_add()**, even in *bind* hooks,
is unsequenced with respect to the end of the created device's lifecycle.

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



### Misc Device APIs

The [zx_device_ops_t][device] *get_size* and *get_protocol* hooks are
unsequenced with respect to all hooks (including concurrent invocations of themselves).
The one exception to this is that they are sequenced before the *release* hook.

[device]: /src/lib/ddk/include/lib/ddk/device.h
[driver]: /src/lib/ddk/include/lib/ddk/driver.h
