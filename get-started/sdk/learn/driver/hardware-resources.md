# Configure hardware resources

Peripheral Component Interconnect (PCI) devices expose resources to the system
using a variety of interfaces including Interrupts, Memory-Mapped I/O (MMIO)
registers, and Direct Memory Access (DMA) buffers. Fuchsia drivers access these
resources through capabilities from the parent device node. For PCI devices,
the parent offers an instance of the `fuchsia.hardware.pci/Device` FIDL protocol
to enable the driver to configure the device.

In this section, you'll be adding functionality to access the following MMIO
registers on the `edu` device:

Address offset | Register              | R/W | Description
-------------- | --------------------- | --- | -----------
0x00           | Identification        | RO  | Major / minor version identifier
0x04           | Card liveness check   | RW  | Challenge to verify operation
0x08           | Factorial computation | RW  | Compute factorial of the stored value
0x20           | Status                | RW  | Bitfields to signal the operation is complete

Note: For complete details on the `edu` device and its MMIO regions, see the
[device specification][edu-device-spec].

After you complete this section, the project should have the following directory
structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/qemu_edu/drivers
                  |- BUILD.bazel
                  |- meta
                  |   |- qemu_edu.cml
{{ '<strong>' }}                  |- edu_device.cc {{ '</strong>' }}
{{ '<strong>' }}                  |- edu_device.h {{ '</strong>' }}
                  |- qemu_edu.bind
                  |- qemu_edu.cc
                  |- qemu_edu.h
```

## Connect to the parent device

To access the `fuchsia.hardware.pci/Device` interface from the parent device
node, add the `fuchsia.driver.compat.Service` capability to the driver's
component manifest:

`qemu_edu/drivers/meta/qemu_edu.cml`:

```json5
{
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/meta/qemu_edu.cml" region_tag="driver" %}
{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/meta/qemu_edu.cml" region_tag="use_capabilities" exclude_regexp="protocol" %}{{ '</strong>' }}
}
```

This enables the driver to open a connection to the parent device and access the
hardware-specific protocols it offers.

Update the driver's `Start()` method to access the `fuchsia.hardware.pci/Device`
offered by the parent device during driver initialization:

`qemu_edu/drivers/qemu_edu.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="imports" adjust_indentation="auto" %}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="compat_imports" adjust_indentation="auto" %}{{ '</strong>' }}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="namespace_start" adjust_indentation="auto" %}
// ...

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="start_method_start" adjust_indentation="auto" %}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="connect_device" %}{{ '</strong>' }}

  FDF_SLOG(INFO, "edu driver loaded successfully");

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="start_method_end" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="namespace_end" adjust_indentation="auto" %}
```

## Set up interrupts and MMIO

With a connection open to the `fuchsia.hardware.pci/Device`, you can begin to
map the necessary device resources into the driver.

Create the new `qemu_edu/drivers/edu_device.h` file in your project
directory with the following contents:

`qemu_edu/drivers/edu_device.h`:

```cpp
#ifndef FUCHSIA_CODELAB_QEMU_EDU_DEVICE_H_
#define FUCHSIA_CODELAB_QEMU_EDU_DEVICE_H_

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="hw_imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="namespace_start" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="class_header" adjust_indentation="auto" %}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="public_main" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="private_main" %}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="class_footer" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="namespace_end" adjust_indentation="auto" %}

#endif  // FUCHSIA_CODELAB_QEMU_EDU_DEVICE_H_

```

Create the new `qemu_edu/drivers/edu_device.cc` file and add the following code
to implement the `MapInterruptAndMmio()` method.
This method performs the following tasks:

1.  Access the Base Address Register (BAR) of the appropriate PCI region.
1.  Extract Fuchsia's [VMO][concepts-kernel-vmo] (Virtual Memory Object) for
    the region.
1.  Create an MMIO buffer around the region to access individual registers.
1.  Configure an Interrupt Request (IRQ) mapped to the device's interrupt.

`qemu_edu/drivers/edu_device.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.cc" region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.cc" region_tag="namespace_start" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.cc" region_tag="interrupt_mmio" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.cc" region_tag="namespace_end" adjust_indentation="auto" %}

```

Add the new device resources to the driver class:

`qemu_edu/drivers/qemu_edu.h`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="imports" adjust_indentation="auto" %}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="hw_imports" adjust_indentation="auto" %}{{ '</strong>' }}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="namespace_start" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="class_header" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="public_main" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="private_main" %}
{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="fields_hw" %}{{ '</strong>' }}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="class_footer" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="namespace_end" adjust_indentation="auto" %}
```

Update the driver's `Run()` method to call the new method during driver
initialization:

`qemu_edu/drivers/qemu_edu.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="start_method_start" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="connect_device" %}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="hw_resources" %}{{ '</strong>' }}

  FDF_SLOG(INFO, "edu driver loaded successfully");

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="start_method_end" adjust_indentation="auto" %}
```

Update the driver build configuration to include the new source files and depend
on the FIDL binding libraries for `fuchsia.hardware.pci`:

`qemu_edu/drivers/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/BUILD.bazel" region_tag="binary" adjust_indentation="auto" exclude_regexp="edu_server\.cc|edu_server\.h|\/\/src\/qemu_edu|sdk\/\/fidl\/fuchsia\.device" highlight="4,5,11" %}
```

## Read device registers

With the base resources mapped into the driver, you can access individual
registers. Add the following register definitions to the
`qemu_edu/drivers/edu_device.h` file in your project:

`qemu_edu/drivers/edu_device.h`:

```cpp
{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="imports" adjust_indentation="auto" %}{{ '</strong>' }}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="hw_imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="namespace_start" adjust_indentation="auto" %}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="register_definitions" adjust_indentation="auto" %}{{ '</strong>' }}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="class_header" adjust_indentation="auto" %}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="public_main" %}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="public_registers" %}{{ '</strong>' }}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="private_main"%}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="class_footer" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.h" region_tag="namespace_end" adjust_indentation="auto" %}
```

This declares the register offsets provided in the device specification as
constants. Fuchsia's `hwreg` library wraps the registers that represent
bitfields, making them easier to access without performing individual bitwise
operations.

Implement the following additional methods in `qemu_edu/drivers/edu_device.cc`
to interact with the MMIO region to read and write data into the respective
`edu` device registers:

*   `ComputeFactorial()`: Write an input value to the factorial computation
    register and wait for the device to asynchronously signal completion using
    an interrupt.
*   `HandleIrq()`: Read the computation result from the factorial register and
    report it to the pending callback.
*   `LivenessCheck()`: Write a challenge value to the liveness check register
    and confirm the expected result.

`qemu_edu/drivers/edu_device.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.cc" region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.cc" region_tag="namespace_start" adjust_indentation="auto" %}
// ...

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.cc" region_tag="compute_factorial" adjust_indentation="auto" %}{{ '</strong>' }}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.cc" region_tag="liveness_check" adjust_indentation="auto" %}{{ '</strong>' }}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_device.cc" region_tag="namespace_end" adjust_indentation="auto" %}
```

Add the following to the driver's `Start()` method to read the major and minor
version from the identification register from the MMIO region and print it to
the log:

`qemu_edu/drivers/qemu_edu.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="start_method_start" adjust_indentation="auto" %}
  // ...

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="hw_resources" %}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="device_registers" %}{{ '</strong>' }}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="start_method_end" adjust_indentation="auto" %}
```

<<_common/_restart_femu.md>>

## Reload the driver

Use the `bazel run` command to build and execute the component target:

```posix-terminal
bazel run //fuchsia-codelab/qemu_edu/drivers:pkg.component
```

The `bazel run` command rebuilds the package and runs `ffx driver register` to
reload the driver component.

Inspect the system log and verify that you can see the updated `FDF_SLOG()`
message containing the version read from the identification register:

```posix-terminal
ffx log --filter qemu_edu
```

```none {:.devsite-disable-click-to-copy}
[driver_manager][driver_manager.cm][I]: [driver_runner.cc:959] Binding fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm to  00_06_0_
{{ '<strong>' }}[full-pkg-drivers:root.sys.platform.pt.PCI0.bus.00_06_0_][qemu-edu,driver][I]: [fuchsia-codelab/qemu_edu/qemu_edu.cc:75] edu device version major=1 minor=0 {{ '</strong>' }}
```

Congratulations! Your driver can now access the PCI hardware resources provided
by the bound device node.

<!-- Reference links -->

[concepts-kernel-vmo]: /concepts/kernel/concepts.md#shared_memory_virtual_memory_objects_vmos
[edu-device-spec]: https://fuchsia.googlesource.com/third_party/qemu/+/refs/heads/main/docs/specs/edu.txt
