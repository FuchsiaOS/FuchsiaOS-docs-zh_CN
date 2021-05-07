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


[device]: /src/lib/ddk/include/lib/ddk/device.h
