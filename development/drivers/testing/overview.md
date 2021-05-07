# Driver testing

## Manual hardware unit tests

A driver may choose to implement the `run_unit_tests()` driver op, which
provides the driver a hook in which it may run unit tests at system
initialization with access to the parent device. This means the driver may test
its bind and unbind hooks, as well as any interactions with real hardware. If
the tests pass (the driver returns `true` from the hook) then operation will
continue as normal and `bind()` will execute. If the tests fail then the device
manager will assume that the driver is invalid and never attempt to bind it.

Since these tests must run at system initialization (in order to not interfere
with the usual operation of the driver) they are activated with a
[kernel command line flag](/docs/reference/kernel/kernel_cmdline.md). To enable
the hook for a specific driver, use `driver.<name>.tests.enable`. Or for all
drivers: `driver.tests.enable`. If a driver doesn't implement `run_unit_tests()`
then these flags will have no effect.

`run_unit_tests()` passes the driver a channel for it to write test output to.
Test output should be in the form of `fuchsia.driver.test.Logger` FIDL messages.
The driver-unit-test library contains a [helper class] that integrates with
zxtest and handles logging for you.

[helper class]: /zircon/system/ulib/driver-unit-test/include/lib/driver-unit-test/logger.h

## Integration tests

Driver authors can use several means for writing integration tests. For simple
cases, the [fake-ddk](/src/devices/testing/fake_ddk) library is recommended. For
more complicated ones,
 [isolated-devmgr](/src/lib/isolated_devmgr) is
recommended.

TODO(fxbug.dev/51320): Fill out more detail here.
