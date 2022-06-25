# Source code layout

## Searching source code

To view and search the Fuchsia source code, there are the following options:

* [Download the Fuchsia source code](/docs/get-started/get_fuchsia_source.md):
  Requires cloning the git repo locally.
* [View in Code Source](https://cs.opensource.google/fuchsia/fuchsia):
  Feature rich source viewer that supports full navigation and edit features.
* [View in Fuchsia git repository](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main):
  Basic repository viewer.

## Overview

Most first-party, open-source code is in the ["fuchsia.git"
repository](https://fuchsia.googlesource.com/fuchsia). Most code in this
repository is organized into a recursive tree of *areas*.

Areas have a regular internal and dependency structure. The `fuchsia.git`
repository itself follows the structure of an area, but also has additional
structure unique to the top level.

Specifically, the `src` top level directory of `fuchsia.git` can be considered
the root area. It follows the structure required of an area, and is the place
where sub areas are located. However, some directories required of an area also
exist next to `src` rather than inside it, e.g. `third_party`. These can be
thought of global ones for all areas to depend on. There are also other places
outside `src` that hold further top-level areas, e.g. in `vendor/*`.
Being open source code `third_party` is available to all areas.

Source repositories, whether open- or closed-source, also follow the conventions
for areas and are mapped into subdirectories of `src` in fuchsia.git. Currently,
we have small number of such "petal" repositories, but we will "promote" areas
currently in the `fuchsia.git` repository into separate repositories as the
system stabilizes.

The `vendor/*` directories contain closed-source code, organized by the vendor
of that code. Nothing outside of `//vendor` can depend on `//vendor`.
Dependencies between different vendors is supported, `vendor/A` can have a
dependency on `vendor/B`.

The `products` directory contains a list of products that you can build. Some
products are quite small and build quickly (e.g., the [core](/products/core.gni)
product), whereas others are more elaborate (e.g., the
[workstation](/products/workstation.gni) product).

The `sdk` directory contains libraries that make up the Fuchsia API Surface.
Some of these libraries are client libraries whereas others are FIDL libraries.
Not all of these libraries are distributed in the Fuchsia SDK. All non-test
FIDL libraries should be placed in the `//sdk/fidl` directory, organized by
FIDL library name, including libraries intended to be used only within the
Fuchsia Platform Source Tree. These libraries can use the default
`sdk_category` of `internal`, which will prevent them from being distributed
to partners, or they can be marked `internal` explicitly with a comment
alerting people to their intended usage.

Most third-party dependencies are stored in separate repositories. These
repositories are included in a local checkout only when needed to support one of
the following source tree configurations:

 * [Bringup](/docs/development/build/build_system/boards_and_products.md#bringup-product).
   This source tree configuration contains enough code to build the
   [bringup](/products/bringup.gni) product.
 * Open Source. This source tree configuration contains all the open source code
   in the Fuchsia Source Tree.
 * All Source.  This source tree configuration contains all the open and closed
   source code in the Fuchsia Source Tree.

See the [guidelines](third-party-metadata.md) on writing the metadata for
third-party code in README.fuchsia files.

## Areas

Most code is organized into a recursive tree of areas. Each area has a regular
internal and dependency structure, which helps people understand code structure
across the whole project.

### Directory Structure

Each area is required to have an [OWNERS](owners.md) file as well as
documentation and tests. Areas can also include binaries, libraries, drivers,
and other source code. In addition, areas can have subareas, which repeat the
pattern:

 * `OWNERS`
    * Each area or subarea must have a [list of owners](owners.md)
 * `BUILD.gn`
    * Build file defining the [canonical targets](#canonical-targets) for the
      area. The area owners may add additional targets to this in addition to
      the canonical targets.
 * `docs/`
    * This directory should contain docs for people working in this area
    * Docs for end-developers (or people working in other areas of Fuchsia)
      should be in the top-level docs or sdk repository
 * `bundles/`
    * This directory contains bundles of package targets in this area. Each area
      should contain at least a `tests` bundle with unit tests for the area, but
      may include other bundles.
 * `bin/` (optional)
 * `lib/` (optional)
 * `drivers/` (optional)
 * `examples/` (optional)
 * `tests/` (optional)
    * This directory contains integration tests that span multiple source code
      directories within the area
    * If disparate areas can have tests in subdirectories, it is suggested
      to add OWNERS files for different test directories to clarify ownership.
    * Unit tests that cover a single binary or library are better placed
      alongside the code they test
 * `testing/` (optional)
    * This directory contains utilities and libraries useful for writing tests
      in this area and subareas.
    * Targets in this directory can only be depended on by testonly targets.
 * `third_party/` (optional)
    * Most third_party dependencies should be in separate repositories
    * Include third_party dependencies in an area only if all of the following:
        * The code is required to be in a third_party directory by policy
        * You intend to fork upstream (i.e., make major changes and not plan to
          integrate future changes from upstream)
        * You make a new name for the code that (a) does not match upstream and
          (b) does not appear in any other third_party directory anywhere in the
          Fuchsia Source Tree
        * The code is open source
    * See more details about `third_party` source layout in [third party source
      management](/docs/concepts/source_code/third-party-management.md)
 * `tools/` (optional)
   * This directory contains command-line tools provided by the area.  These
     are usually things that can (or must) be built for the development host
     rather than for Fuchsia.  They may or may not be used directly in the
     area's own build, but can also be used by developers.  They may or may
     not be published in an SDK.  Special-purpose tools that are used in the
     build but really are not intended for developers to use directly should
     be kept near their uses rather than here.
   * This should contain a a subdirectory named for each tool (or collection
     of related tools with a natural collective name), rather than putting all
     of the area's tools together into the top `tools/BUILD.gn` file.
 * `[subareas]` (optional)
    * Subareas should follow the generic area template
    * Do not create deeply nested area structures (e.g., three should be enough)

Areas may use additional directories for internal organization in addition to
the enumerated directories.

### OWNERS

Each area and subarea must contains an OWNERS file. Directories may contain
`OWNERS` without being considered areas, e.g. the top level `products`
directory, or subdirectories of the `/src/lib` directory. A directory lacking an
`OWNERS` file is considered to have the same owners as its parent directory of
the same area.

One exception is the `//src/tests` directory where tests from different areas
that cover multiple aspects of the system (not just a particular area) are
expected to live. Because of this, every area should add OWNERS files for any
tests that live in this directory.

### Dependency Structure

In addition to depending on itself, an area can depend only on the top-level
`build`, `sdk`, and `third_party` directories, as well as the `lib` directory
from anywhere in the tree:

 * `//build`
 * `//sdk`
 * `//third_party`
 * `//src/**/lib/`

Targets in an area that are marked testonly in the build system may
additionally depend on the `testing` directory in that area and ancestors:

 * `(../)+testing/` (testonly=true targets only)

### Canonical targets

Each area and subarea must define the following canonical targets in their
top-level BUILD.gn file:

* `<dir-name>`
  * All directories should have a target with the same name as the directory.
    The directory target is essentially an "all" target, intended and used to
    produce "build-everything" builds.
    * All buildable artifacts defined in the directory and subdirectories.
    * All tests in the current directory and subdirectories.
  * The directory target should only produce maximal builds - it should not
    include configuration targets or changes that would modify the "product
    behavior" of a particular product - for example, including the directory
    target should not cause new software to be automatically started at boot
    time, or override default service topologies or service maps.
  * When a new subdirectory is added to an area, it should define this
    directory-name target, as well as including the directory name target in
    the parent directory target.

* `tests`
  * All of the tests within this area
  * When a new subdirectory is added with a new tests target, the tests target
    should be added to the parent directories tests target.

### Naming conventions

Typically, when naming files and directories, the best practice is to use names
that are short and clear. In those cases where a name is comprised of multiple
words, those words should be separated by underscores .e.g `long_file_name`.

#### Example

The following is an example for a directory called `fortune`.

```gn
import("//build/drivers.gni")
import("//build/components.gni")

group("fortune") {
  testonly = true
  deps = [
    ":pkg",
    ":tests",
  ]
}

group("tests") {
  testonly = true
  deps = [
    ":fortune_tests"
  ]
}

executable("bin") {
  output_name = "fortune"

  sources = [
    "fortune.cc"
  ]
}

executable("test") {
  testonly = true
  output_name = "fortune-test"

  sources = [
    "test.cc"
  ]
}

fuchsia_component("component") {
  manifest = "meta/fortune.cmx"
  deps = [ ":bin" ]
}

fuchsia_package("pkg") {
  package_name = "fortune"
  deps = [ ":component" ]
}

fuchsia_unittest_package("fortune_tests") {
  deps = [ ":test" ]
}
```

## Repository layout

This section depicts the directory layout for the Fuchsia Source Tree.
Non-starred entries are directories or files in the fuchsia.git repository.
Starred (`*`) entries are separate repositories that are mapped into the
directory structure using `jiri` (except for the prebuilt directory, which is
populated from CIPD).

 * `.clang-format`
 * `.dir-locals.el`
 * `.gitattributes`
 * `.gitignore`
 * `AUTHORS`
 * `CODE_OF_CONDUCT.md`
 * `CONTRIBUTING.md`
 * `LICENSE`
 * `OWNERS`
 * `PATENTS`
 * `README.md`
 * `rustfmt.toml`
 * `sdk/banjo/fuchsia.hardware.gpio/`
 * `sdk/banjo/...`
 * `sdk/fidl/fuchsia.media/`
 * `sdk/fidl/fuchsia.mediacodec/`
 * `sdk/fidl/...`
 * `sdk/lib/ddk/`
 * `sdk/lib/fit/`
 * `sdk/lib/fidl/`
 * `sdk/lib/zircon/`
 * `sdk/lib/...`
 * `.gn`
 * `BUILD.gn`
 * `build/`
 * `bundles/`
 * `configs/`
 * `infra/`
    * `configs/`
       * `generated/`
 * `integration/`
 * `products/`
 * `scripts/`
 * `docs/`
 * `examples/`
 * `third_party/`
    * `boringssl/` *
    * `icu/` *
    * `rust_crates/` *
    * `...` *
 * `prebuilt/`
    * `chromium/` *
    * `dart/` *
    * `flutter/` *
    * `llvm/` *
 * `tools/`
    * `banjo/`
    * `fidl/bin/backend/{c,cpp,dart,go,llcpp,rust}`
    * `fidl/bin/frontend/`
    * `fidl/docs/`
    * `fidl/examples/`
    * `fidl/tests/`
 * `src/`
    * `lib/`
    * `cobalt/`
    * `component/`
    * `connectivity/`
    * `developer/`
    * `experiences/` *
    * `graphics/`
    * `identity/`
    * `media/`
    * `modular/`
    * `storage/`
    * `testing/`
    * `ui/`
       * `scenic/`
    * `updater/`
    * `virtualization/`
    * `zircon/kernel/`
    * `zircon/drivers/`
    * `zircon/userspace/`
 * `vendor/`
    * `[closed-source code from various vendors]` *

## Evolution

As the system stabilizes, we can promote areas out of fuchsia.git into separate
repositories. Generally, we should promote an area to a separate repository when
the interface between the area and the rest of the system is sufficiently stable
(requires approval by top-level OWNERS).

New code can be:

 * Added to an existing directory in fuchsia.git
 * Added to a new top-level area or subarea of an existing area
 * Added to an existing repository
 * Added to a new repository (requires approval by top-level OWNERS)
