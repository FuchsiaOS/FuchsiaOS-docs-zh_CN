# Build variants in the Fuchsia build

## Abstract

This document explains the implementation of "build variants", a feature of
the Fuchsia build system that allows building _instrumented_ or
_specially-optimized_ versions of the host and device binaries.

The reader must be familiar with [GN toolchain() instances][gn-toolchain]
and should have read the following documents:

- [GN toolchains overview][gn-toolchains-overview]
- [ELF shared library redirection][elf-shared-library-redirection]


## Overview of build variants

The Fuchsia build defines several types of build variants, for example:

- The `asan` and `ubsan` variants are used to build machine code with
  Clang's [Address Sanitizer][clang-asan], and
  [Undefined Behaviour Sanitizer][clang-ubsan], respectively.
  There is even an `asan-ubsan` variant that combines both.

- The `coverage` variant is used to build machine code with Clang's
  instrumentation-based profiling enabled, to support code coverage
  collection.

- The `profile` variant is used to build instrumented code as well,
  but to support profile-guided optimization.

- The `thinlto` and `lto` variants are used to build binaries with
  link-time optimization enabled.

- The `gcc` variant is used to build certain pieces of the Zircon
  kernel with the GCC compiler instead of Clang (which has been useful
  to weed out subtle machine code generation issues that can affect
  the kernel in very important ways).

- The `release` and `debug` variants, which are provided to override
  the default compilation mode, which is determined by the value of
  the `is_debug` build configuration variable in `args.gn`.

- A few other variants for specialized needs, which are all defined
  in the `//build/config/BUILDCONFIG.gn` file, using conventions
  described in the rest of this document.

Generally speaking, a single build variant models:

- A set of extra configs that define compiler, assembler or linker flags
  to be applied when building variant binaries and their dependencies.

- A set of optional implicit dependencies to be added to the final
  variant binary targets in the build graph (i.e. executables, loadable
  modules and even sometimes shared libraries).

For example, let's consider the "asan" build variant, which is used to
enable [Clang's Address Sanitizer][clang-asan]{:.external} support. In
practice, building a Fuchsia executable program with Address Sanitizer
enabled requires (at a minimum):

- Passing the `-fsanitize=address` flag both to the Clang compiler and linker
  when building the executable and all its dependencies (including the
  C library in the case of Fuchsia).

- The Asan runtime (`libclang_rt.asan.so`) being available at runtime,
  as well as its own dependencies (i.e. special prebuilt versions of the
  `libc++.so`, `libc++abi.so` and `libunwind.so` binaries).


## Base and variant toolchains

Build variants are always applied to a specific "base" toolchain, which
provides the default settings that are augmented by the variant itself.
This creates a new [GN toolchain()][gn-toolchain]{:.external} instance,
called a _"variant toolchain"_, which has its own `root_out_dir`. For
example:

- `//build/toolchain:host_x64` is the base toolchain used to build
  host binaries, and its `root_out_dir` is `${root_build_dir}/host_x64`.

  `//build/toolchain:host_x64-ubsan` is the variant toolchain created
  by applying the `ubsan` variant, and its `root_out_dir` is
  `${root_build_dir}/host_x64-ubsan`.

- `//build/toolchain/fuchsia:x64` is the default toolchain (when targeting
  x64-based devices), used to build Fuchsia user-level binaries. Because it is
  the default, its `root_out_dir` is the same as `root_build_dir`.

  `//build/toolchain/fuchsia:x64-asan` is the variant toolchain created by
  applying the `asan` variant to the default toolchain. Its `root_out_dir`
  will be `${root_build_dir}/x64-asan`.

As a general rule, `//${tc_dir}:${tc_name}-${variant}` will be
a variant toolchain created by applying the `${variant}` variant to a base
toolchain named `//${tc_dir}:${tc_name}`, and its `root_out_dir` will
always be `${root_build_dir}/${tc_name}-${variant}`.

If a base toolchain has an [shlib toolchain][elf-shared-library-redirection],
then any of its variant toolchains will have one too. Finally, a single variant
can be applied to several base toolchains.

For example `//build/toolchain:host_x64-asan` and
`//build/toolchain/fuchsia:x64-asan` are variant toolchains created
by applying the same `asan` variant to the base toolchains used to
build host and Fuchsia device binaries.

The latter would also have `//build/toolchain/fuchsia:x64-asan-shared` as
an shlib toolchain to generate ELF-based shared libraries.

Base toolchains must be defined in the build using either `clang_toolchain_suite()`
or `zircon_toolchain_suite()`. Both templates end up calling `variant_toolchain_suite()`
which implements the magic that automatically creates variant toolchains when needed.

### Toolchain and variant tags  {: #toolchain-and-variant-tags }

Each base toolchain in the Fuchsia build can have a number of tags, which
are free-form strings describing properties of the toolchain. For example,
the `"kernel"` tag is used to indicate that a toolchain is used to build kernel
artifacts (this is important because there is no C library, no standard C++
library and a few other constraints that are important for certain target
definitions).

The list of all valid toolchain tags is in
[`//build/toolchain/toolchain_tags.gni`][toolchain_tags.gni].

Similarly, each variant definition has a number of tags, describing properties
of the variant. For example, the `"instrumentation"` tag is used to
indicate that this variant creates machine code that performs runtime
instrumentation (e.g. sanitizers or profilers)

The list of all valid variant tags and their documentation is in
[`//build/toolchain/variant_tags.gni`][variant_tags.gni].

When a variant toolchain is created, the global `toolchain_variant.tags` value
will contain both the tags inherited from the base toolchain, and those inherited
from the variant.

### Toolchain variant instantiation

The build system will only create variant toolchains when they are needed.
There is a very large number of possible toolchain+variant combinations,
and creating all of them at once would make `gn gen` considerably slower.

Instead of eagerly creating all variants, the build system decides which
toolchain variants to create based on the following conditions:

- The list of [variant selectors](#variant-selectors) that appear in
  the `select_variant` global variable.

- The list of [variants descriptor](#variant-descriptors) names that appear as
  the [`enable_variants`][enable-variants]{:.external} argument of the
  `variant_toolchain_suite()` template. It is seldom used to force-enable a
  few variants even if `select_variant` is empty.

  For example, the ASan and UBSan variants of the toolchain used to build the C library
  are always enabled, because these are needed when building the Core Fuchsia IDK
  (see `//zircon/system/ulib/c/BUILD.gn`).

- The list of variant tags that appear in the
  [`exclude_variant_tags`][exclude-variant-tags]{:.external}
  argument of `variant_toolchain_suite()`. It is seldom used to exclude specific
  variants from being applied to a given base toolchain.

  For example, the bootloader excludes variants with the `"instrumented"`
  tag, since it is not possible to run a sanitizer or profiling runtime
  while booting the device (see `//


## Variant descriptors  {: #variant-descriptors }

A variant descriptor is a GN scope that describe the properties of
a given build variant to the build system. These are defined through
the `known_variants` variable in `//build/config/BUILDCONFIG.gn`, and
 each scope should follow the following strict schema:

- **`configs`**: An optional list of GN config labels, that will be
  automatically added to every target with this variant.

  Note that for each config `${label}`, in this list, there must also
  be a target `${label}_deps`, which each target built in this variant
  will automatically depend on. Most of the time, this will be an
  empty `group()`.

- **`remove_common_configs`**: An optional list of GN config labels
  that should be _removed_, if present, from any target built with
  this variant. This is sometimes necessary when some of the default
  configs that the build system sets up for binaries should not be
  used for a specific variant.

- **`remove_shared_configs`**: An optional list of GN config labels,
  similar to `remove_common_configs`, but will only apply when
  building `shared_library()` targets and their dependencies.

- **`deps`**: An optional list of GN target labels, which will be
  added as implicit dependencies to any _linkable_ target that
  is built with this variant.

- **`name`**:  A string uniquely naming the variant descriptor, as
  typically used in `select_variant`. If `name` is omitted, `configs`
  must be non-empty and will be used to derive a name (by joining
  their names with dashes).

- **`tags`**: An optional list of free-form strings describing properties
  of the variant (see
  [toolchain and variant tags](#toolchain_and_variant_tags]).

- **`toolchain_args`**: An optional scope, where each variable defined
  in this scope overrides a build argument in the toolchain context
  of this variant.

- **`host_only`** and **`target_only`**: Optional scopes that can
  contain any of the fields above. These values are used only for host or
  target (i.e. device) toolchains, respectively. Any fields
  included here should not also be in the outer scope.

Here are a few examples:

### Example variant descriptor with a single config

```
{
  configs = [ "//build/config/lto" ]
  tags = [ "lto" ]
}
```
The scope above defines a variant descriptor named `"lto"` (since
there is no `name` key in the scope, the name is deduced from the
values in `configs` which here only contains a single item).

Applying this variant will add the `//build/config/lto:lto` config,
defined in `//build/config/lto/BUILD.gn`, and that file should also
contain a `//build/config/lto:lto_deps` empty group if such a
config has no implicit dependencies. For example:

```
# //build/config/lto/BUILD.gn
config("lto") {
  cflags = [ "-flto" ]
  asmflags = cflags
  ldflags = cflags
  rustflags = [ "-Clto=fat" ]
}

group("lto_deps") {
  # Implicit dependencies for "lto" config.
  # This is an empty group since there are none.
}
```

This descriptor uses the `"lto"` tag to indicate that this
variant performed link-time optimization. This tag can also
be used by the `"thinlto"` descriptor, which would be using
a different config.

### Example variant descriptor with several configs

```
{
  configs = [
    "//build/config/sanitizers:ubsan",
    "//build/config/sanitizers:sancov",
  ]
  remove_common_configs = [ "//build/config:no_rtti" ]
  tags = [
    "instrumented",
    "instrumentation-runtime",
    "kernel-excluded",
    "sancov",
    "ubsan",
  ]
}
```

This defines a variant descriptor named `"ubsan-sancov"` (the
name is derived from the `configs` list by joining the config
names with dashes), used to build machine code that detect
undefined behaviour at runtime, and collects code coverage
information at the same time.

Note that this also requires `//build/config/sanitizers:ubsan_deps`
and `//build/config/sanitizers:sancov_deps` to be defined to list
implicit dependencies from these configs.

This uses `remove_common_config` because `//build/config:no_rtti`
is part of the default configs of many base toolchains, but RTTI
must be enabled for UBSan instrumentation to work properly.

The list of tags used is also much more extensive. Note the
`"kernel-excluded"` tag which is used to prevent this variant
to be applied to any kernel machine code.

### Example variant descriptor with `toolchain_args`

```
{
  name = "release"
  toolchain_args = {
    is_debug = false
  }
}
```

This variant descriptor is named explicitly, and does not
add any configs or dependencies. On the other hand, it ensures
that the global build configuration variable `is_debug` will
be set to false, which change how many default configs are
defined in a corresponding variant toolchain context.


### Universal variants  {: #universal-variants }

A lesser known feature of the build system is called "universal variants". These
are additional variant descriptors that _combine_ with other known variants,
they work as follows:

- If `is_debug=false` is set in `args.gn`, meaning that all binaries should
  be built with maximal optimizations, then the `"debug"`  variant descriptor
  is defined by the build. This allows building specific targets in debug mode if
  necessary.

- Similarly, if `is_debug=true` (the default), then the `"release"` variant
  descriptor is defined by the build. This allows building specific targets with
  full optimizations if necessary.

- Additionally, the universal variants above are combined with all other known
  variant descriptors automatically by the build. E.g. if `is_debug=false`,
  then the build will also create `"asan-debug"`, `"ubsan-debug"`,
  `"thinlto-debug"`, etc. If `is_debug=true`, then it will define `"asan-release"`,
  `"ubsan-release"`, `"thinlto-release"` and so on instead.

Note that these variant descriptors are _conditionally_  defined by the build,
based on the value of `is_debug`. I.e. there is no `"release"` variant and its
combinations when `is_debug=false`, and there is no `"debug"` variant and its
combinations when `is_debug=true`!


## The `toolchain_variant` global variable  {: #toolchain_variant }

When in a `BUILD.gn` or `*.gni` file, the global `toolchain_variant` variable
can be used to retrieve variant-related information for the `current_toolchain`.
This is a scope with the following schema:

- **`name`**: Name of the build variant descriptor. This is an empty string in
  the context of a base toolchain, or the name of the variant descriptor that
  was used to created the current GN `toolchain()` instance otherwise.

    Examples names for various toolchain contexts:

    ```
    //build/toolchain/fuchsia:x64                ""
    //build/toolchain/fuchsia:x64-shared         ""
    //build/toolchain/fuchsia:x64-asan           "asan"
    //build/toolchain/fuchsia:x64-asan-shared    "asan"
    ```

- **`base`**: A fully-qualified GN label to the base toolchain for the
  current one. Note that for the shlib toolchain of a toolchain variant,
  this points to the final base toolchain. Examples:

    ```
    //build/toolchain/fuchsia:x64              //build/toolchain/fuchsia:x64
    //build/toolchain/fuchsia:x64-asan         //build/toolchain/fuchsia:x64
    //build/toolchain/fuchsia:x64-shared       //build/toolchain/fuchsia:x64
    //build/toolchain/fuchsia:x64-asan-shared  //build/toolchain/fuchsia:x64
    ```

- **`tags`**: A list of free-form strings, each one describing a property of the
  current toolchain instance and its variant. This is simply the union of
  [toolchain and variant tags](#toolchain-and-variant-tags).

- **`instrumented`**: A boolean flag which will be set to true iff the `tags` list
  contains the `"instrumentation"` tag value, provided as a convenience to replace
  a complicated testing instruction in GN like:

    ```
    if (toolchain_variant.tags + [ "instrumentation" ]
        - [ "instrumentation" ] != toolchain_variant.tags) {
      # toolchain is instrumented
      ...
    }
    ```

    With:


    ```
    if (toolchain_variant.instrumented) {
      # toolchain is instrumented
      ...
    }
    ```

- **`is_pic_default`**: A boolean that is true in a toolchain that can build
  ELF position independent code (PIC). This means either an shlib toolchain
  (e.g. `//build/toolchain/fuchsia:x64-shared`), or a base toolchain that
  produces such code directly (e.g. `//zircon/kernel/lib/userabi/userboot:userboot_arm64`).

- **`with_shared`**: A boolean that is true if the current toolchain has an shlib
  toolchain to build ELF shared libraries (e.g. `//build/toolchain/fuchsia:x64`)
  *or* when in such a toolchain (e.g. `//build/toolchain/fuchsia:x64-shared`).

- **`configs`**, **`remove_common_configs`**, **`remove_shared_configs`**: List
  of GN labels to `config()` items, that come directly from the current
  [variant descriptor](#variant-descriptors), if any, or empty lists otherwise.

- **`deps`**: List of GN labels to targets that are added as dependencies to
  any _linkable_ target, inherited from the variant descriptor itself, if any.

- **`libprefix`**: For instrumented variants, this is an installation prefix string
  for shared libraries, or an empty string otherwise. See the
  [toolchain variant libprefix](#toolchain-variant-libprefix) section for full
  details.

- **`exclude_variant_tags`**: Used internally by the variant selection logic.
  Inherited from a `clang_toolchain_suite()` or `zircon_toolchain_suite()`
  call, or directly from a target definition. It is is a list of tags used to
  exclude variants to be applied to a base toolchain, or target, as is
  sometimes necessary.

- **`suffix`**: This is `"-${toolchain_variant.name}"`, or `""` if name is empty.
  Used internally to simplify expansions without conditionals.

The content of this global variable is seldom used by target definitions
to alter their configuration based on the current toolchain context. This
mostly happen for low-level targets, like the C library, kernel artifacts,
or instrumentation runtime support that do not come as prebuilts.

### Toolchain variant libprefix  {: #toolchain-variant-libprefix }

In order to be able to mix instrumented and non-instrumented binaries in a single
Fuchsia package, special steps must be performed by the build system:

- The shared libraries that are built using an *instrumented* variant toolchain
  must be installed to `"lib/<variant>/"` instead of the default `"lib/"` location.

- The executable binaries must be compiled with a linker argument like
  `"-Wl,-dynamic-linker=<variant>/ld.so.1"`, which overrides the default value
  (`"ld.so.1"`, which is hard-coded in the Fuchsia clang prebuilt toolchain binaries).

- As a special case, fuzzing build variants use the non-fuzzing build variant
  name for the library sub-directory.

The `toolchain_variant.libprefix` variable is defined in the following way to help
support all of this easily:

```none
  variant name        libdir               libprefix        note

  no variant    --->  lib/                 ""               (default target toolchain)
  thinlto       --->  lib/                 ""               (uninstrumented)
  asan-ubsan    --->  lib/asan-ubsan/      "asan-ubsan/"    (instrumented)
  asan-fuzzer   --->  lib/asan/            "asan/"          (instrumented + fuzzing)
```

This can be used to determine the install location as `"lib/${toolchain_variant.libprefix}"`
and the linker flag as `"-Wl,-dynamic-linker=${toolchain_variant.libprefix}ld.so.1"`.


## Variant selection  {: #variant-selection }

The Fuchsia build system supports selecting which build variants are
enabled, and to which individual targets, or groups of targets, they
apply. This is done by defining the `select_variant` variable in the
build configuration file (`args.gn`). Consider the following example:

```
# From out/default/args.gn
...

select_variant = [
  {
    label = [ "//src/sys/component_manager:bin" ]
    variant = "release"
  },
  "host_asan",
  "thinlto/blobfs",
  "ubsan",
]
```

Each value in the list is an expression, called a
[**variant selector**](#variant-selectors) which can be a scope or a string,
used to configure how the build will apply variants to different sets of targets.

When `select_variant` is defined and not an empty list, its value will
be used to determine how to build *linkable* targets like executables,
loadable modules and shared libraries that appear in the build graph
in the context of a base toolchain, as well as all their dependencies.

The variant selectors that appear in `select_variant` are compared
in order, and the first one that matches the current target is selected.
As such, the example above means that:

- The `//src/sys/component/manager:bin` program binary, and its dependencies
  should always be built with the `release` variant (NOTE: This example
  will result in an error at `gn gen` time is `is_debug=false` is in
  the `args.gn` file, because the `"release"` variant will not exist
  in this case, see [universal variants](#universal-variants) to see why).

- Host binaries should be built in the `"asan"` variant.
  Note that `"host_asan"` is not a variant descriptor name, but a
  [variant shortcut](#variant-shortcuts).

- The `blobfs` program device binary should always be
  built using the `"thinlto"` variant, which performs link-time
  optimizations.

- All other device binaries should be built with the `"ubsan"` variant.


### Variant selectors {: #variant-selectors }

A variant selector is a value that can appear in the global `select_variant` build
configuration variable. They are used by the build system to control variant
selection when defining linkable targets in the context of base toolchains.

Three types of values are supported:

- A scope that defines a set of matching criteria for a set of targets.
  The format of that scope is the following:

    - **`variant`**: The name of a given [variant descriptor](#variant-descriptors)
      that will be used if, and only if, the current target matches all the criteria
      defined in the rest of the scope.

    - **`label`**: If defined, this must be a list of qualified GN labels
      (with `:` but without toolchain labels, e.g. `//src/sys/foo:foo`).

    - **`name`**: If defined, a list of GN label target names (e.g. the name
      of the `//src/sys/foo:bar` target is '"bar"`).

    - **`dir`**: If defined, a list of GN label directory paths (e.g. the path
      of the `//src/sys/foo:bar` target is `"//src/sys/foo"`).

    - **`output_name`**: If defined, a list of target `output_name` value
      (the default being its `target_name`).

    - **`target_type`**: If defined, a list of strings matching the target type.
      Valid values are: `"executable"`, `"test"`, `"loadable_module"`,
      `"shared_library"` and a few others.

    - **`testonly`**: If defined, a boolean. If true the selector matches targets
      with `testonly=true`. If false, the selector matches targets without
      `testonly=true`.

    - **`host`**: If defined, a boolean. If true the selector matches targets in
      a host toolchain. If false, the selector matches in the target toolchain.

- A string that contains a simple name (e.g. `"asan"`) that points to a
  [variant shortcut](#variant-shortcuts), which is an alias for a pre-existing
  selector scope value.

  For example, the `"coverage"` value is equivalent to the following scope:

  ```
  {
    variant = "coverage"
    host = false
  }
  ```

- A string that contains a variant shortcut name and an output name separated by a
  directory path (e.g. `"thintlo/blobfs"`). This is a convenience format that
  avoids writing an equivalent scope, which would look like this in the previous
  example as:

    ```
    {
      variant = "thinlto"
      host = false
      output_name = [ "blobfs" ]
    }
    ```

The order of selectors in the `select_variant` list is important: the first selector
that matches the current target wins and determines how said target will be built.


### Variant shortcuts

In addition to variant descriptors, the build sets up a number of "shortcuts", which
are named aliases for a few hard-coded variant selector scope values. The build adds
a few hard-coded ones, and creates others from the list of known variants:

- The `"host_asan"` shortcut is defined to build host binaries with the `"asan"`
  variant descriptor, and is technically equivalent to the following list of
  selector scope values:

    ```
    # Definition for the `host_asan` variant shortcut

    [
      {
        variant = "asan"
        host = true
      }
    ]
    ```

    Similarly, there exists `host_asan-ubsan`, `host_coverage`, `host_profile`
    and several others.

- _Every_ variant descriptor name has a corresponding shortcut that applies
  it exclusively to device binaries. I.e. the `"ubsan"` shortcut is equivalent
  to this list of one selector scope value:

    ```
    [
      {
        variant = "ubsan"
        host = false
      }
    ]
    ```

    This is the reason why using a variant descriptor name in `select_variant`
    only applies it to device binaries, as in:

    ```
    # Applies the `ubsan` variant to device binaries, not host ones!
    select_variant = [
      "ubsan",
    ]
    ```

- Similarly, a shortcut is defined for every universal variant and its
  cobinations, which again only apply them to device binaries.

  This means, that assuming that `is_debug=true` in `args.gn`, the
  following would force all device binaries to be built in release
  mode, while the host ones would still be built in debug mode.

  ```
  is_debug = true
  select_variant = [ "release" ]
  ```
  Which is equivalent to:

  ```
  is_debug = true
  select_variant = [
    {
      variant = "release"
      host = false
    }
  ]
  ```

  A way to force host binaries to be compiled in release mode would
  be to use an explicit scope value, since there is no shortcut for
  this use case, as in:

  ```
  is_debug = true
  select_variant = [
    {
      variant = "release"
      host = true
    }
  ]
  ```

## Variant target redirection

### The `variant_target()` template  {: #variant_target }

The `variant_target()` template defined in `//build/config/BUILDCONFIG.gn`
implements the core build variant selection mechanism.

This template should not be called directly from `BUILD.gn` files, instead,
it is invoked by the wrapper templates defined by the Fuchsia build for
`executable()`, `loadable_module()`, `shared_library()` and a few others
that correspond to _linkable targets_ (i.e. those created with the static
linker).

What it does is, for each target, in each toolchain context of the build
graph, compare the content of `select_variant` with the target's
properties (i.e. target type and a few additional arguments) to:

1) Compute the "builder toolchain" for the target, which is the GN
   toolchain instance that will be used to build the real binary,
   and its dependencies.

2) If the current toolchain is the builder toolchain, just build the
   target as usual.

3) Otherwise, create either a `group()` or `copy()` target that will
   redirect (i.e. publicly depend) on the target in the builder toolchain.
   Whether this is a group or a copy depends on subtle conditions
   that are fully documented in the implementation of `variant_target()`,
   but see the following sub-section for some explanations.

   The `copy()` target is required to preserve the output location
   of some linkable targets, while the `group()` is used when this
   is not needed.

   Most of the time, an `executable()` or `loadable_module()` target
   will require a `copy()`, and a `shared_library()` one will require
   a `group()` instead.


### Output location of linkable variant binaries

A critical design limitation of the GN configuration language is that,
with a few exceptions, a given target definition _doesn't know anything
about its dependencies_, except for their labels. This is problematic
because there are many cases where a given target needs to know where
its dependencies' outputs are located, or what type of targets these
dependencies really are.

To illlustrate this, let's consider the following example:

- An `executable()` target named `//src/my/program:bin` that generates a
  Fuchsia program binary named `my_program`. Due to the way the build works
  this generates `${root_build_dir}/exe.unstripped/my_program` and
  `${root_build_dir}/my_program`, plus a few minor files (ignored here).

- An `action()` target named `//src/my/program:verify_binary` used to parse
  the program binary to check it or extract information out of it (let's say
  it verifies its import symbol references). This target needs to depend on
  the first one, but also locate where the binary's output location, as in:

```none
    action("//src/my/program:verify_imports")
      script = "check-my-imports.py"
      deps = [ "//src/my/program:bin" ]
      inputs = [ get_label_info(deps[0], "root_out_dir") + "/my_program" ]
      ...
      |
      |  deps
      |
      v

    executable("//src/my/program:bin")
      output_name = "my_program"
      # outputs: [
        ${root_build_dir}/exe.unstripped/my_program,
        ${root_build_dir}/my_program,
      ]
```

Here, the `action()` can guess the location of the program binary by
using `get_label_info("<label>", "root_out_dir")` for its directory,
and hard-code the `output_name` value in the action itself. This violates
abstraction layers, but this is necessary given GN's limitations.

When build variants are enabled, the actual output location of binary targets
will change depending on `select_variant`. If variant redirection is implemented
with a simple `group()`, the graph becomes:

```none
    action("//src/my/program:verify_imports")
      script = "check-my-imports.py"
      deps = [ "//src/my/program:bin" ]
      inputs = [ get_label_info(deps[0], "root_out_dir") + "/my_program" ]
      ...
      |
      |  deps
      |
      v

    group("//src/my/program:bin")
      |
      |  public_deps
      |
      v

    executable("//src/my/program:bin(//build/toolchain/fuchsia:x64-asan")
      output_name = "my_program"
      # outputs: [
      #   ${root_build_dir}/x64-asan/exe.unstripped/my_program,
      #   ${root_build_dir}/x64-asan/my_program,
      # ]
```

The problem is that the value of `inputs` in the top-level action didn't change,
so its command will try to find the program binary at the old location
(`${root_build_dir}/my_program`) instead of the new one
(`${root_build_dir}/x64-asan/my-program`). Either the build will use a stale artifact,
or will fail due to a missing file.

Parsing `select_variant` in the action itself would be too expensive, so solving
this situation, for executable and loadable module targets required a `copy()`
target, instead of `group()`, to ensure that the unstripped binary is copied
to its original location. The graph becomes:

```none

    action("//src/my/program:verify_imports")
      script = "check-my-imports.py"
      deps = [ "//src/my/program:bin" ]
      inputs = [ get_label_info(deps[0], "root_out_dir") + "/my_program" ]
      ...
      |
      |  deps
      |
      v

    copy("//src/my/program:bin")
      outputs = [ "${root_build_dir}/my_program" ]
      sources = [ "${root_build_dir}/x64-asan/my_program" ]
      |
      |  public_deps
      |
      v

    executable("//src/my/program:bin(//build/toolchain/fuchsia:x64-asan")
      output_name = "my_program"
      # outputs: [
        ${root_build_dir}/x64-asan/exe.unstripped/my_program,
        ${root_build_dir}/x64-asan/my_program,
      ]
```

With this setup, the build always succeeds, and the action command always
processes the right binary.

All of this is done automatically in the build. The final effect is that
dependent do not need to care whether their dependencies were built with
a specific variant or not, they can rely on the output locations to be
stable, at least for the unstripped binary paths.

### Output location of ELF shared libraries

TBW

### The special `novariant` descriptor {: #novariant }

TBW

## Special global variables  {: #global_variables }

### The `host_toolchain` and `host_out_dir` global variables {: #host_variables }

TBW

### The `zircon_toolchain` variable  {: #zircon_variable }

TBW

### The `variant()` template  {: #variant_template }

TBW

[gn-toolchain]: https://gn.googlesource.com/gn/+/main/docs/reference.md#toolchain-overview
[gn-toolchains-overview]: /docs/development/build/build_system/internals/toolchains/gn_toolchains_overview.md
[elf-shared-library-redirection]: /docs/development/build/build_system/internals/toolchains/elf_shared_library_redirection.md
[clang-asan]: https://clang.llvm.org/docs/AddressSanitizer.html
[clang-ubsan]: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html
[enable-variants]: https://cs.opensource.google/fuchsia/fuchsia/+/main:build/toolchain/variant_toolchain_suite.gni;drc=28b4b027204084d695ba0659a7ecb733196b543f;l=180
[exclude-variant-tags]: https://cs.opensource.google/fuchsia/fuchsia/+/main:build/toolchain/variant_toolchain_suite.gni;drc=28b4b027204084d695ba0659a7ecb733196b543f;l=151
[toolchain_tags.gni]: https://cs.opensource.google/fuchsia/fuchsia/+/main:build/toolchain/toolchain_tags.gni
[variant_tags.gni]: https://cs.opensource.google/fuchsia/fuchsia/+/main:build/toolchain/variant_tags.gni
