## Magma Contributing and Best Practices

### Submitting a patch

See [Contributing](/CONTRIBUTING.md).

### Source Code

The core magma code is found under:

* [lib/magma](/src/graphics/lib/magma)

Magma service drivers are found under:

* [src/graphics/drivers](/src/graphics/drivers)

Magma client drivers are third party codebases.  Open source client drivers typically live in [third_party/mesa](https://fuchsia.googlesource.com/third_party/mesa).

### Coding Conventions and Formatting

* Use the **[Google style guide](https://google.github.io/styleguide/cppguide.html)** for source code.
* Run **clang-format** on your changes to maintain consistent formatting.

### Build Configuration for Testing

##### Product for L0 and L1 testing:
* core

##### Packages for L0 and L1 testing:
* src/graphics/lib/magma/tests:l1

##### Product for L2 testing:
* workstation_eng

##### Package for L2 testing:
* src/experiences/examples/spinning_cube

### Testing Pre-Submit

For details on the testing strategy for magma, see [Test Strategy](test_strategy.md).

There are multiple levels for magma TPS.  Each level includes all previous levels.

When submitting a change, indicate the TPS level tested, prefaced by the hardware
on which the testing was performed:

TEST:
nuc,vim2:go/magma-tps#L2
nuc,vim2:go/magma-tps#S1
nuc,vim2:go/magma-tps#C0
nuc,vim2:go/magma-tps#P0

#### L0

Includes all unit tests and integration tests.  There are 2 steps at this tps level:

1. Build with magma_enable_developer_build and include the magma-dev package; this will run unit tests that require hardware
when the system driver starts, then exposes the device as usual.  Inspect the syslog for test results.

Example:

fx set core.x64 --with-base=//src/graphics/lib/magma/gnbuild/magma-intel-gen:magma-dev --args magma_enable_developer_build=true

2. Build with `--with-base //src/graphics/bundles:vulkan --with src/graphics/lib/magma/tests:l0` and run the test script [src/graphics/lib/magma/scripts/test.sh](/src/graphics/lib/magma/scripts/test.sh).

#### L1

If you have an attached display, execute the spinning [vkcube](/src/graphics/examples/vkcube).
This test uses an imagepipe swapchain to pass frames to the system compositor.
Build with `--with-base //src/graphics/bundles:vulkan --with src/graphics/lib/magma/tests:l1`.
Test with present direct to display: `run fuchsia-pkg://fuchsia.com/vkcube-on-fb#meta/vkcube-on-fb.cmx --c 500`
Test with present via Scenic: `ffx session add fuchsia-pkg://fuchsia.com/vkcube-on-scenic#meta/vkcube-on-scenic.cmx`.

#### L2

A full UI 'smoke' test.  Build workstation, launch Chromium and navigate to a
WebGL demo such as Aquarium.

#### S0

Run vkcube-on-scenic overnight (12-24 hours).

#### S1

A full UI stress test.  Launch two instances of the spinning_cube flutter example, and let them run overnight.

#### C0

For some changes, it's appropriate to run the Vulkan conformance test suite before submitting.
See [Conformance](#conformance).

#### P0

For some changes, it's appropriate to run benchmarks to validate performance metrics. See [Benchmarking](#benchmarking).

### Conformance

For details on the Vulkan conformance test suite, see

* [third_party/vulkan-cts](https://fuchsia.googlesource.com/third_party/vulkan-cts/+/HEAD/README.md)

### See Also
* [Test Strategy](test_strategy.md)
