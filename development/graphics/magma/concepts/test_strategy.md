Magma Test Strategy
===================

## Architecture Diagram

* [Magma Block Diagram](development/graphics/magma/block_diagram.svg)

Four major interfaces

* [Vulkan](https://www.khronos.org/vulkan)
* [magma](/src/graphics/lib/magma/include/magma/magma.h)
* [magma system](/src/graphics/lib/magma/src/magma_util/platform/platform_connection.h)
* [msd](/src/graphics/lib/magma/include/msd/msd.h)

Four major components

* libvulkan
* libmagma
* magma_system
* vendor msd (magma service driver)

## Challenges

* Vulkan drivers require hardware for complete testing
    * Each supported gpu requires testing of a different hardware platform
* GPUs are complex pieces of hardware with flaws that may trigger misbehavior infrequently
    * There may be tests that flake rarely
* Vulkan CTS (conformance) takes several hours to run
    * Should be run on a daily build, not part of normal CQ
* Upstreaming libvulkan changes to the vendor
    * Vendor must be provided a build and test environment with which they can validate Vulkan CTS on Fuchsia
* Source access to gfxbench is restricted
    * Should we test a binary package?

## Unit Tests

Some of these require hardware; those that don't are included in pre-submit checks for CQ.

* magma_util_tests
    * Coverage 100% of magma_util (not including platform)
* magma_platform_tests:
    * Coverage 100% of magma_util/platform
* magma_system_tests
    * Coverage 100% of magma system
    * Uses mock MSD
* vendor MSD
    * Coverage 80-100% (may not be worth mocking out some portions of the hardware interacting code)
    * Several mocks used in place of hardware
        * platform mmio
        * platform bus mapper
        * address space
        * mapped batch
* libvulkan
    * Supplied by vendor
    * Uses mock magma system (if not sufficient, becomes a hardware interaction test)

Unit tests that require hardware may be built into a test MSD which runs the
tests when bound. A vendor-specific test harness unbinds the production driver,
binds the test driver in its place, and queries the results of the tests. The
test harness then unbinds the test driver and rebinds the production driver.

## Hardware Interaction Tests

The interaction between app, libvulkan, msd, and gpu is complex.  Generally speaking the app generates Vulkan command buffers and shader programs, which are created in a gpu specific binary format by libvulkan.
Those command buffers as well as other resources are shared with the magma system driver, which maps resources into the gpu's address space and schedules command buffers on the gpu's execution units.

* magma_conformance_tests
    * Does not execute command buffers; rely on Vulkan CTS for command buffer coverage
* msd_conformance_tests
    * Validates a vendor's msd implementation
    * Coverage goal 100%, currently ~50% (fxbug.dev/13060 for implementing vendor specifics)
* vendor specific
    * Shutdown
    * Hang/fault recovery
* vkreadback
    * Validates Vulkan end-to-end as simply as possible
* vkloop
    * Validates hang detection and recovery
* vkext
    * Validates Fuchsia Vulkan extensions
* [Vulkan CTS](https://github.com/KhronosGroup/VK-GL-CTS)
    * Takes several hours to run
    * Should be run at least once a day
    * Vendor must be provided a build and test environment with which they can validate Vulkan CTS on Fuchsia

### Hardware required

Fuchsia supports devices with the following gpus:

* Intel Gen 9 - Intel HD Graphics
* ARM Mali - Bifrost
* Verisilicon GC7000

GPUs are complex pieces of hardware with flaws that may trigger misbehavior infrequently. There may be tests that flake rarely.  If detected these should be treated as driver bugs.

## Performance Tests

* [Gfxbench](https://gfxbench.com)
    * A large and complex performance benchmark.
    * Fuchsia details, forthcoming.

## Fuzzing

MSDs should be fuzzed through the magma FIDL interface. Each MSD should have
its own fuzzer, as every driver is unique in how it processes command buffer
or immediate command data and as such needs different input pre-processing to
ensure adequate coverage. [libfuzzer][libfuzzer] fuzzers should link against
the MSD using [fake-ddk][fake-ddk] to ensure coverage information can be
received from the driver. Since the real GPU hardware can't be used in a
fuzzer environment, the fuzzer must have a fake device implementation.

A [seed corpus][seedcorpus] should be used to ensure MSD inputs give full
coverage. This can be gathered either through an instrumented ICD or a
proxy MSD that intercepts all Vulkan commands.

Fuzzing Vulkan ICDs is not required as part of Fuchsia ICD development
because of these difficulties:

* The Vulkan spec is full of undefined behavior and as such it's difficult to
  ensure a fuzzer tests only legitimate inputs. Incorrect inputs are allowed
  to crash or hang the application - see [the Vulkan specification][vulkanerrors].
* The ICD is in the application's address space, and as such fuzzing cannot
  ensure security guarantees against a malicious application that can also
  arbitrarily modify its own address space. Protection of the integrity of
  the operating system and other applications must be performed at the MSD
  layer.

ICD developers are encouraged to create fuzzers for parts of the ICD that do
not have undefined behavior. This may include defining test ICD builds that
define behavior that the specification would otherwise leave undefined.

Most testing to ensure conformance of ICDs with the Vulkan specification
happens in connection with the Vulkan CTS. There are external efforts such as
[graphicsfuzz][graphicsfuzz] to use fuzzing to help add CTS tests.

## See Also
* [Contributing](contributing.md)

[graphicsfuzz]: https://github.com/google/graphicsfuzz
[fake-ddk]: /src/devices/testing/fake_ddk
[libfuzzer]: development/testing/fuzzing/write-a-fuzzer.md
[seedcorpus]: development/testing/fuzzing/improve-a-fuzzer.md#measure_code_coverage
[vulkanerrors]: https://www.khronos.org/registry/vulkan/specs/1.1-extensions/html/vkspec.html#fundamentals-errors

