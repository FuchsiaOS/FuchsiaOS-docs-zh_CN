# Magma: Porting Guide

For an overview of Magma including background, hardware requirements, and
description of architecture, please see [Magma: Overview](/development/graphics/magma/README.md).

This document gives a process a driver developer can follow to port a Vulkan
driver to Fuchsia.

## Requirements

Porting a Vulkan driver to Fuchsia requires the following:

* Hardware documentation (register spec, theory of operation).
* A reference Vulkan implementation (Linux).
* The client driver library (ICD) should provide a conformant implementation
  of Vulkan 1.1/1.2.

The hardware must have passed bringup so that network access, disk storage,
and [fx pave][paving] function.

At the moment Magma only supports UMA devices, but if you're ambitious you
could attempt to port to a GPU with discrete memory.

## Creating a stub system driver

The Magma system driver (MSD) is analogous to a kernel driver on other
operating systems. At this point, consider whether to port an existing kernel
driver from another operating system or write one from scratch.

At this point you should read the [driver getting started guide][dgsd] to get
an understanding of the Fuchsia driver model.

This choice depends on many aspects of the existing kernel driver code.
Some considerations:

* Does the driver have a platform abstraction layer or is it dependent on Linux
  kernel interfaces?
* Licensing.
* How similar is its ioctl interface to the Magma entry-points?

The first coding step is to create a basic MSD that builds. MSDs are located
in [the driver directory][driverdir]. They are built as `fuchsia_drivers`
using the GN build system. The Magma system driver must be open source but
not GPL and hosted on *fuchsia.googlesource.com*.

Fuchsia drivers are normal userspace processes. That means they have access
to most of the [c library][libc] and a subset of [POSIX APIs][fdio]. Unlike
most processes, no filesystem access is allowed.

At the moment Magma supports two different device types - platform devices
and PCI devices.

* SoCs generally have platform devices. These are not plug and play, but
  require a [board driver][boarddriver] to delegate the appropriate resources
  to them. Platform devices need a [magma_pdev_entry][magma_pdev_entry] GN target.
* PCI devices are on the PCI bus and are delegated resources
  by the PCI bus driver. PCI drivers need special GN targets.

### Splitting responsibilities? (SoC version)

On SoCs, the GPU hardware will often come as a separate IP block that can be
customized by the SoC vendor. Depending on the level of customization by the
vendor, there are a couple of possibilities for how to proceed.

* Unified MSDs, but with a SoC-specific driver loaded before.
* Make a separate MSD per SoC or SoC vendor.

If the customizations to the SoC are small, it's better to have a unified
MSD. A vendor-specific driver will bind first and will export [banjo][banjo]
interfaces for the MSD to power on/off the GPU, change clocks, etc. This has
the advantage that it's easier to port the MSD to new hardware without
modifications by implementing a new vendor-specific driver. See
[msd-arm-mali][msd-arm-mali] and [aml-gpu][aml-gpu] for an example of this
approach.

Having a separate MSD per SoC gives more flexibility and might be necessary
if the GPU IP vendor allows many customizations to the IP block in an SoC.
MSD implementations for different SoCs may share a library of SoC-independent
code, but will be compiled as independent drivers.

### Splitting responsibilities? (PCI version)

PCI GPUs often include display controller hardware. The display controller
driver should ideally be implemented separately from the GPU hardware,
because then it can be stored in
[bootfs][glossary.bootfs] and can provide a boot
console before disk access is possible. The display controller should expose
a hardware-specific [banjo][banjo] interface and the MSD can bind to the
display driver.

See [msd-intel-gen][msd-intel-gen] and [intel-i915][intel-i915] for an
example of a PCI driver that's split into two parts.

## Powering on

With the MSD now building, the next step is to write code to reset the device
and get it into operating mode. This may include:

* Powering on the device (possibly using the
  [fuchsia.hardware.power.Power][fuchsia.hardware.power.Power] banjo interface).
* Enabling clocks (possibly using the
  [fuchsia.hardware.clock.Clock][fuchsia.hardware.clock.Clock] banjo interface).
* Enabling bus mastering or memory access.
* Loading firmware.

The driver should also get access to MMIO ranges as needed and should start
handling interrupts. For SoCs, the [board driver][boarddriver] must be
modified to pass these resources to the MSD or SoC-specific driver and must
add a device for the MSD to bind to.

Testing at this stage:

* Logging MMIO registers on driver startup.

## Implementing the MSD

Here is an organized list of the [main functions][msdheader] the driver can implement:

* Initialize hardware
	* *msd_driver_create*
	* *msd_driver_configure*
	* *msd_driver_destroy*
	* *msd_driver_create_device*
	* *msd_device_destroy*
* Support for parameter querying
	* *msd_device_query*
* Support for status dump
	* *msd_device_dump_status*
* Create connections
	* *msd_device_open*
	* *msd_connection_close*
* Create buffers
	* *msd_buffer_import*
	* *msd_buffer_destroy*
* Set up memory spaces and buffer mappings
	* *msd_connection_map_buffer_gpu*
	* *msd_connection_unmap_buffer_gpu*
	* *msd_connection_commit_buffer*
	* *msd_connection_release_buffer*
* Set up hardware contexts
	* *msd_connection_create_context*
	* *msd_context_destroy*
* Command buffer scheduling
	* *msd_context_execute_command_buffer*
	* *msd_context_execute_immediate_commands*
	* *msd_connection_set_notification_callback*
* Create semaphores
	* *msd_semaphore_import*
	* *msd_semaphore_destroy*
* Fault handling
* Power management

With the hardware successfully powering on, the next step is to decide how to
map your existing ioctls onto MSD entry-points.

In most cases, the mapping between linux DRI ioctls and MSD functions is
straightforward. One exception is the case of memory management: in Magma,
it's the ICD that allocates and maps memory, not the MSD (or kernel driver).
This may change the flow around some commands that allocate [VMOs][vmo], since the
MSD has to import already-existing buffers into the GPU hardware.

If that approach doesn't work for some types of memory, a driver may
use a [Sysmem][sysmem] heap to handle allocation of that memory. The client
allocates memory using Sysmem and imports the handle using the normal Magma
interface. Then the MSD can communicate with sysmem to get more information
about the memory.

Drivers may not require implementations of all functions. We recommend
implementing MSD functions gradually as needed by the ICD. This can provide
context when implementing the MSD functions and can help avoid wasted effort
on unneeded functions.

Testing at this stage:

* driver-specific unit tests (not hardware-specific)
* hardware-specific driver tests (see [an example][hardwareunit]). These tests
  should exercise the GPU in a minimal way, such as writing to a register or
  causing the GPU to modify a memory location.
* driver-specific integration tests that use the Magma interface.
* magma-conformance-tests (part of [Magma L0][l0]).
* magma-info-test (part of [Magma L0][l0]).

## Building the ICD

The ICD must be ported to Fuchsia. ICD code must be checked out with the
rest of the Fuchsia tree and built along with the rest of Fuchsia.

The ICD may be either given a completely new GN build, or the Fuchsia GN
build can execute actions in the driver's existing build system.

Because of [ICD abi][icdabi] restrictions, ICDs must be statically linked
against all their dependencies. An ICD must be a single shared library, and
may only reference libc.so and libzircon.so. At this stage you can stub out
all other references as necessary. The ICD must also link to
[libmagma][libmagma], which provides the Magma runtime.

The Vulkan loader service retrieves the ICDs from packages and advertises them
to Vulkan clients. The ICD must be packaged with metadata and manifest JSON
files, as described in the [loader service documentation][loader-readme].

If the ICD package is included in [universe][package-deployment] it can be
reloaded by doing `fx shell killall vulkan_loader.cm`. Components launched
afterwards will get the new ICD package, while older components will either use
the old ICD or may fail when creating Vulkan instances.

The ICD must export a certain set of symbols - see
[the Vulkan ABI definition][icdabi]. You should implement them at this point.

Testing at this stage:

* `readelf -d` on the shared library to ensure it has no dependencies besides
  libc.so and libzircon.so.
* Launching the vulkan loader using `fx shell cat
  /svc/fuchsia.vulkan.loader.Loader` and checking `ffx inspect show
  core/vulkan_loader` to see if it's loaded. Errors will go to syslog.
* Run the [icd_load][icd_load] test. This test will check if any ICD on the
  system works, so ensure no other ICDs are on the system before running it.

## Connect the ICD to Magma

At this point the ICD should connect to /dev/class/gpu/<n> using zxio and
provide that device to libmagma using [magma_device_import][magmaheader].

After this stage the [magma_*][magmheader] functions will work, so ioctl
calls can gradually be converted over to equivalent Magma calls.

Testing at this stage:

* [vkreadback][vkreadback] (draws a color then reads back the framebuffer
  values). This is part of [Magma L0][l0]).
* Vulkan conformance testing. Ideally a 100% pass rate will be seen after this
  stage is completed. See the [Magma testing strategy][teststrategy] for details.

## Remove disallowed symbols

Use the [version script][versionscript] when linking your ICD to ensure it
only exposes the symbols allowed by the Fuchsia system ABI.

Only symbols listed in [the symbol allow list][allowlist] may be used from
the ICD. To check this, either pass the allowlist to
`imported_symbols_allowlist` in your `magma_vulkan_icd` target or use the
`verify_imported_symbols` GN template to check your prebuilt ICD.

Some unsupported file operations may be replaced with calls to the
`OpenInNamespace` callback provided to `vk_icdInitializeOpenInNamespaceCallback`.

Testing at this stage:

* `verify_imported_symbols` succeeds.

## Implement Fuchsia extensions

At this point, the ICD can't be used with [scenic][scenic] and doesn't have
window system integration. The driver must implement Fuchsia-specific Vulkan
extensions. The client driver library should provide a conformant
implementation of Vulkan 1.0/1.1/1.2.

### VK_FUCHSIA_external_memory

 This extension is similar to VK_KHR_external_memory_fd and allows
 importing/exporting VkDeviceMemory from/to VMOs. This extension has been upstreamed to the [Vulkan specification][extmemoryspec].

Testing at this stage:

* `vkext --gtest_filter=VulkanExtension.*` (part of [Magma L0][l0]).

### VK_FUCHSIA_external_semaphore

 This extension is similar to VK_KHR_external_semaphore_fd and allows
 importing/exporting binary semaphores to/from zircon events. This extension has been upstreamed to the [Vulkan specification][extsemaphorespec].

Testing at this stage:

* `vkext --gtest_filter=VulkanExtension.*` (part of [Magma L0][l0]).
* `vulkan-cts-zircon` (part of the [Vulkan CTS][teststrategy]).

### VK_FUCHSIA_buffer_collection

This extension interacts with sysmem and allows clients to negotiate image
formats and allocate memory. See the [sysmem][sysmem] documentation for more details.

 This extension is currently WIP and subject to change, but can be found in the
 Fuchsia internal [Vulkan header][vulkanheader].

Testing at this stage:

* `vkext` (part of [Magma L0][l0]).
* [vkcube-on-fb][vkcube] (animated, using VK_KHR_swapchain - part of [Magma L1][l1]).


## Validation

All tests listed in each of the subsections above must pass. See the [test
strategy documentation][teststrategy] for more details and a complete list of
test cases.

## Long term support

The MSD and ICD must be updated with new code drops from the hardware vendor.
Ideally the code is upstreamed and the GPU vendor will supply and maintain
the system driver using the Zircon DDK.

[glossary.bootfs]: /glossary/README.md#bootfs
[paving]: /development/build/fx.md#what-is-paving
[boarddriver]: /development/drivers/concepts/device_driver_model/platform-bus.md
[icdabi]: /concepts/packages/system.md#vulkan-icd
[banjo]: /development/drivers/concepts/device_driver_model/banjo.md
[sysmem]: /development/graphics/sysmem/concepts/sysmem.md
[vkreadback]: /src/graphics/tests/vkreadback
[hardwareunit]: /src/graphics/drivers/msd-arm-mali/tests/integration/run_unit_tests.cc
[vulkanheader]: https://fuchsia.googlesource.com/third_party/Vulkan-Headers/+/refs/heads/master/include/vulkan/vulkan_fuchsia.h
[scenic]: /concepts/ui/scenic/index.md
[msd-arm-mali]: /src/graphics/drivers/msd-arm-mali
[aml-gpu]: /src/graphics/drivers/aml-gpu
[msd-intel-gen]: /src/graphics/drivers/msd-intel-gen
[intel-i915]: /src/graphics/display/drivers/intel-i915
[driverdir]: /src/graphics/drivers
[vkcube]: /src/graphics/examples/vkcube
[icd_load]: /src/graphics/tests/icd_load
[libmagma]: /src/graphics/lib/magma/src/libmagma
[intelgn]: /src/graphics/lib/magma/gnbuild/magma-intel-gen/BUILD.gn
[fuchsia.hardware.clock.Clock]: /sdk/banjo/fuchsia.hardware.clock/clock.fidl
[fuchsia.hardware.power.Power]: /sdk/banjo/fuchsia.hardware.power/power.fidl
[dgsd]: /development/drivers/concepts/getting_started.md
[libc]: /concepts/kernel/libc.md
[fdio]: /concepts/filesystems/life_of_an_open.md#fdio
[versionscript]: /src/graphics/lib/magma/scripts/libvulkan.version
[allowlist]: /src/graphics/lib/magma/gnbuild/imported_symbols.allowlist
[magma_pdev_entry]: /src/graphics/lib/magma/src/magma_util/platform/zircon/driver_entry.gni
[vmo]: /reference/kernel_objects/vm_object.md
[msdheader]: /src/graphics/lib/magma/include/msd/msd.h
[magmaheader]: /src/graphics/lib/magma/include/magma/magma.h
[l0]: /development/graphics/magma/concepts/contributing.md#l0
[l1]: /development/graphics/magma/concepts/contributing.md#l1
[teststrategy]: /development/graphics/magma/concepts/test_strategy.md
[loader-readme]: /src/graphics/bin/vulkan_loader/README.md
[extmemoryspec]: https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_FUCHSIA_external_memory.html
[extsemaphorespec]: https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_FUCHSIA_external_semaphore.html
[package-deployment]: /development/build/fx.md#package_deployment_options
