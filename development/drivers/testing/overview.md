# Driver testing

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

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
[kernel command line flag](/reference/kernel/kernel_cmdline.md). To enable
the hook for a specific driver, use `driver.<name>.tests.enable`. Or for all
drivers: `driver.tests.enable`. If a driver doesn't implement `run_unit_tests()`
then these flags will have no effect.

`run_unit_tests()` passes the driver a channel for it to write test output to.
Test output should be in the form of `fuchsia.driver.test.Logger` FIDL messages.
The driver-unit-test library contains a [helper class] that integrates with
zxtest and handles logging for you.

[helper class]: /zircon/system/ulib/driver-unit-test/include/lib/driver-unit-test/logger.h

## Integration tests

Driver authors should use the [isolated-devmgr](/src/lib/isolated_devmgr)
for integration tests.

## Unit tests
Drivers authors should use the [mock-ddk](/development/drivers/testing/mock_ddk.md)
library for unit tests.

There are a number of helpful mock libraries:

* [fake_pdev](/src/devices/bus/testing/fake-pdev/fake-pdev.h) - Creates info for a fake pdev parent
* [mock-mmio-reg](/src/devices/testing/mock-mmio-reg/include/mock-mmio-reg/mock-mmio-reg.h) Mocking Mmio registers
* [fake-object](/src/devices/testing/fake-object/README.md) - fake userspace versions of kernel objects

TODO(fxbug.dev/51320): Fill out more detail here.
