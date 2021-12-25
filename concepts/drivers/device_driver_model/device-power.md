<!---

# Device power management

The [zx_device_ops_t][device] *suspend* hook is sequenced before itself (e.g.
if a request to suspend to D1 happens, and while that is being executed a
request to suspend to D2 happens, the first will finish before the latter
begins). It is also sequenced before the *resume* hook.

The `set_performance_state` hook is sequenced before itself.
It has no particular ordering with suspend/resume hooks.
After the driver returns from the set_performance_state hook with success,
it is assumed by power manager that the device is operating at the requested
performance state whenever the device is in working state. Since the hook only
executes on the driver host's main thread, multiple requests are not executed
concurrently.
On success, the out_state and the requested_state is same. If the device is in a
working state, the performance state will be changed to requested_state immediately.
If the device is in non-working state, the performance state will be the requested_state
whenever the device transitions to working state.
On failure, the out_state will have the state that the device can go into.

The `configure_autosuspend` hook is sequenced before itself and is used to configure whether
devices can suspend or resume themselves depending on their idleness. The hook is called with
the deepest sleep state the device is expected to be in which is when the device is suspended.
If the entire system is being suspended to a sleep state, the driver should expect `suspend`
hook to be called, even if the auto suspend is configured. It is not supported to selectively
suspend a device when auto suspend is configured.

--->

# 设备电源管理

 [zx_device_ops_t][device] *suspend* 钩子函数对于自身也是有序的（例如，如果对 D1 发起挂起请求，并且在执行该请求的同时对 D2 也发起挂起请求，那么第一个请求将在后者开始之前就完成）。同样*resume*钩子函数也是如此。

`set_performance_state`钩子函数对于自身也是有序的。 suspend/resume 钩子函数是没有特殊顺序的。在驱动 set_performance_state 钩子函数成功返回之后，只要设备处在工作状态中，电源管理就会认为设备是以要求的性能状态运行的。因为钩子函数仅在驱动主机主线程运行，则不会有多个请求需要同时运行。

成功运行中，out_state 和 requested_state 是相同的。如果设备处在正在工作状态中，其性能状态会立即变更到 requested_state 。如果设备处在非工作状态中，无论什么时候设备变换工作状态，性能状态都将为requested_state。
在失败运行中，out_state 将表示有设备可进入的状态。

`configure_autosuspend` 钩子函数对于自身也是有序的，它被用作配置设备是否可以根据其空闲状态而挂起或恢复自身。当设备挂起时，`configure_autosuspend` 钩子函数在设备预期处于的最深睡眠状态下被调用。如果整个系统都处于睡眠状态被挂起，即使配置为自动挂起，驱动也会期待`suspend`钩子函数调用。当配置为自动挂起时，有选择地挂起设备是不支持的。


[device]: /src/lib/ddk/include/lib/ddk/device.h
