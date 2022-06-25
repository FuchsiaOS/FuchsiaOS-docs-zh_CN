# ELF Shared Library Redirection

## ELF shared object requirements

On ELF-based systems, like Linux and Fuchsia, machine code that goes into
shared objects (i.e. `shared_library()` and `loadable_module()` target in
GN speak) must be built with the `-fPIC` compiler and linker option.

This is unlike executable code, which uses `-fPIE` instead. This generates
smaller and faster code than `-fPIC`, but which cannot be used for shared
objects.

The Fuchsia build does not support generating shared libraries for Linux,
however for Fuchsia, it defines separate toolchain instances to compile
executables and shared libraries.

This separate toolchain is called the _"shlib" toolchain_ or even
_companion toolchain_ in the internal build rules, and is always named by
appending a `-shared` suffix to a _base toolchain_ label. For example
`//build/toolchain/fuchsia:x64-shared` is the shlib toolchain for the base
`//build/toolchain/fuchsia:x64` toolchain.

## ELF shared library redirection

The Fuchsia build implements a feature that ensures that ELF `shared_library()`
and `loadbable_module()` targets are always built in an _shlib toolchain_ as
defined in the previous section.

Fuchsia developers do not need to know about it, since this is completely
transparent: writing `shared_library()` targets as usual, without having to
worry about ELF / non-ELF at all.

The rest of this section explains how it is implemented, which first requires
understanding why it is needed, with a few practical examples:

Consider the case of a C++ static library (e.g. `libutil`) that can be linked
to either executables or shared library objects. Since both types of binaries
require code to be built with different compiler flags, one way to do this
is to _define two targets_ as in:

```gn
  # A variant of the library to be linked into executables.
  static_library("libutil-static") {
    sources = [ ... ]
    ...
  }

  # A variant of the library to be linked into shared libraries.
  static_library("libutil-shared") {
    sources = [ ... ]
    ...
    if (is_fuchsia) {
      cflags = [ "-fPIC" ]  # Required for shared library code on Fuchsia.
    }
  }

  executable('program') {
    sources = [ ... ]
    deps = [ ":libutil-static" ]
  }

  shared_library('foo') {
    sources = [ ... ]
    deps = [ ":libutil-shared" ]
  }
```

This works but there are a number of inconveniences:

- The library needs two target definitions, instead of one, and
  they need to be kept in sync. Even when not building Fuchsia binaries.

- The explicit `is_fuchsia` check and compiler flag additional need
  to be added to every shared variant definition, which is subtle,
  error prone, and makes these definitions less abstract.

  (Some of that can be simplified with GN configs, but still).

- Any target that uses the library needs to select one of the
  two variants explicitly.

To make things just a little bit more realistic, let's consider that
the `libutil` library also depends on another `liblog` static library.
The latter will also need to provide both a static and shared variant,
as in:

```gn
  static_library("liblog-static") {
    sources = [ ... ]
    ...
  }

  static_library("liblog-shared") {
    sources = [ ... ]
    if (is_fuchsia) {
      cflags = [ "-fPIC" ]
    }
  }

  static_library("libutil-static") {
    ...
    deps = [ ":liblog-static" ]
  }

  static_library("libutil-shared") {
    ...
    if (is_fuchsia) {
      cflags = [ "-fPIC" ]  # Required for shared library code.
    }
    deps = [ ":liblog-shared" ]
  }

  ... same as above
```
This can be illustrated by the following dependency graph:

```none
  program ->
      libutil-static ->
          liblog-static

  foo ->
      libutil-shared ->
          liblog-shared
```

Keeping all these target definitions and dependencies properly
in sync is tedious and makes build rules far less abstract and useful.

Using a dedicated toolchain to build ELF shared object code avoids the
target duplication for `libutil` and its dependents. Something like:

```gn
# The following definition is global and should normally be put
# in the BUILDCONFIG.gn file

if (is_fuchsia) {
  # Name of the toolchain used to build ELF shared library code.
  # This toolchain adds the `-fPIC` option to all build commands
  # by default so there is no need to add it in target definitions.
  shlib_toolchain = "${current_toolchain}-shared"
} else {
  # Shared library code can be built directly in the current toolchain
  # on non-Fuchsia platforms.
  shlib_toolchain = current_toolchain
}

# The following is part of a BUILD.gn file

static_library("liblog") {
  ...
}

static_library("libutil") {
  ...
  deps = [ ":liblog" ]
}

executable('program') {
  sources = [ ... ]
  deps = [ ":libutil" ]
}

shared_library("foo") {
  sources = [ ... ]
  deps = [ ":libutils($shlib_toolchain)" ]
}
```

Which now corresponds to the depedency graph:

```
  program ->
      libutil ->
          liblog

  foo ->
      libutil($shlib_toolchain) ->
          liblog($shlib_toolchain)
```

The scheme above solves most of the original issues because:

- The explicit `is_fuchsia` check, and `-fPIC` compiler flag addition
  have been completely abstracted away from the static library
  definitions.

- The `libutil` target doesn't need to worry about which variant
  of its dependencies to select.

On the other hand, each `shared_library()` instance still needs
to carefully select its static library and source set dependencies
from the `shlib_toolchain` to link to the proper variant of the
code.

The Fuchsia build uses one last trick to solve this last issue:
using a _redirection group target_ in the base toolchain, to
reference the real shared library target in the shblib toolchain,
as in:

```gn
# Set to true if the current toolchain's target platform is based
# on ELF and requires an shlib toolchain. This would normally be
# defined in BUILDCONFIG.gn for Fuchsia toolchains.
_requires_shlib_toolchain = ...

if (_requires_shlib_toolchain && current_toolchain != shlib_toolchain) {
  # A simple group that depends on the same target built in
  # the shblib_toolchain. Note that `public_deps` instead of `deps`
  # is required when crossing toolchain boundaries for proper
  # linking.
  group("bar") {
    public_deps = [ ":bar($shlib_toolchain)" ]
  }
} else {
  # The target that actually builds the shared library in
  # the shlib_toolchain, or the base one for non-Fuchsia platforms.
  # It will pick its dependencies in the same toolchain context.
  shared_library("bar") {
   ...
   deps = [ ":libutil" ]
  }
}

executable("program2") {
  deps = [ ":bar" ]
}
```

This ends up creating the following dependency graph on Fuchsia:

```none
  program2 -->
      bar -->                           # redirection group
          bar(shblib_toolchain) -->     # real shared_library()
              libutil(shlib_toolchain)
```

While a non-Fuchsia base toolchain will get:

```none
  program2 -->
      bar -->                           # real shared_library()
          libutil
```

To further simplify usage, the Fuchsia build redefines the
`shared_library()` template in its `BUILDCONFIG.gn`, hiding this
implementation detail from `BUILD.gn` files which can be written in the
most natural way, as in:

```gn
### This would appear in BUILDCONFIG.gn to ensure that `shared_library()`
### does ELF shared library redirection automatically when needed.

if (_requires_shlib_toolchain) {
  template("shared_library") {
    if (current_toolchain != shlib_toolchain) {
      group(target_name) {
        public_deps = [ ":${target_name}(${shlib_toolchain})" ]
      }
    } else {
      # Ensure the built-in shared_library() function is called.
      target("shared_library", target_name) {
        forward_variables_from(invoker, "*")
      }
    }
  }
}

### This would appear in regular BUILD.gn files

# Invokes the custom `shared_library()` template instead
# of the built-in one. On Fuchsia systems, this will create a
# redirection group that refers to a real shared_library()
# target in the shlib_toolchain. On a non-Fuchsia system, this
# simply defines a real shared_library() target.

shared_library("bar") {
  ...
  deps = [ ":libutil" ]
}

executable("program2") {
  ...
  deps = [ ":bar" ]
}
```

For more details, see the [`shared_library()` definition in `BUILDCONFIG.gn`](https://cs.opensource.google/fuchsia/fuchsia/+/main:build/config/BUILDCONFIG.gn;drc=9e1506dfbe789637c709fcc4ad43896f5044f947;l=880){:.external}


