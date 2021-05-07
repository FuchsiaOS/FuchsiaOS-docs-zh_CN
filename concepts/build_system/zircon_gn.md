# GN in Zircon

This discussion assumes basic familiarity with GN syntax and concepts.
[This introduction to GN](intro.md) can provide that background.

GN uses a templating structure to abstract many of the build details away from
the end user.  Below are a subset of the templates the Zircon GN defines,
focusing on the ones with which Zircon hackers are most likely to interact.

## `//zircon/` prefix

As discussed in [the introduction](intro.md), GN uses "source-absolute" paths
that look like `//a/b/c`.  In the Zircon GN files, we **never** use `//`.
Instead, use `//zircon/foo` to refer to `//zircon/foo`,
e.g. `"//zircon/system/ulib/zircon"`.

## `executable()` and `test()`

The primary target type in producing a binary is `executable()`.  This produces
an executable binary from the listed sources.  The Zircon build also provides a
means to indicate the location in the image wherein that binary should be
installed via the `install_path` variable in the target scope.
`install_path` can be:

 * a string: the path relative to the root of the BOOTFS (with no leading `/`)
 * omitted: use the default path of `bin/<binary_name>`
 * `false`: do not install this file at all

The build also provides a `test()` target, which is identical to
`executable()` except that it sets `testonly = true` and that its default
`install_path` is `test/<binary_name>` instead of `bin/<binary_name>`.

`test()` can be used for a test program that runs on Zircon or for a test
program that runs on the host side.  In fact, the same `test()` target can
serve to build the same test program for both situations with no extra work
required.  (It's just what dependency paths reach that target that will
determine whether it's built for host or for Zircon or for both.)

## `library()`

The `library()` template is for any kind of "library" in the Zircon tradition,
whether for the kernel, Zircon user code, or host-side code.  The basic thing
it means to be a "library" is that there is an `include/` subdirectory of
public header files.  Dependents that list this `library()` target in their
`deps` will automatically get `-I` switches for that `include/` directory.

The default case with the most concise syntax is a static-only userland
library.  Making a library available as a shared library just requires adding
the line `shared = true`.  Likewise, making a library available for host-side
use just requires adding the line `host = true`.  These are in addition to the
default `static = true` that makes the library available for userland static
linking.  For a library that should *never* be statically linked (aside from
host-side or kernel uses), you can override the default with `static = false`.

For a library in the kernel, set `kernel = true`.  This is the same whether
it's a kernel-only library, or is code shared between kernel and user (and/or
host).  Setting `kernel = true` changes the default to `static = false`, so if
a library can be used either in the kernel or in userland, then you must set
`static = true` explicitly alongside `kernel = true` (unless you set `shared =
true` and want to prohibit static linking of that library in userland).

Note: For kernel modules that do not provide an `include/` subdirectory,
use [`source_set()`](#source_set) instead of `library()`.

Here’s an exemplar showing all the essential options.  Most actual targets
will be little more than a `sources` list and a `deps` list.

```gn
library("foo") {
  # Builds "libfoo.a" when static, "libfoo.so" when shared.

  static = true  # default, omitted unless kernel = true: build userland libfoo.a
  shared = true  # false if omitted: build userland libfoo.so
  kernel = true  # false if omitted: can be used from kernel
  host = true  # false if omitted: can be used in host tools

  sources = [
    "foo.c",
    "bar.cpp",
  ]

  deps = [
    # Can refer here to `source_set()` or other `library()` targets defined
    # locally.
    ":foo_minimal",  # Defined in this same BUILD.gn file.
    "foobar_subsystem",  # Defined in foobar_subsystem/BUILD.gn relative to here.

    # Explicitly link in static libbar.a even if libbar.so is available.
    "//zircon/system/ulib/bar:static",

    # Be explicit about getting libbaz.so as a shared library.
    "//zircon/system/ulib/baz:shared",

    # Compile with -Isystem/ulib/bozo/include, but don't link anything in.
    # This should usually not be used in `deps`, but only in `public_deps`.
    # See below.
    "//zircon/system/ulib/bozo:headers",

    # Let system/ulib/quux/BUILD.gn decide whether static or shared is the
    # norm for that library.  (So far the defining `library()` will always
    # prefer the shared library if it's enabled; it would be easy to add the
    # option to build shared but default to static if that's ever useful.)
    "//zircon/system/ulib/quux",

    # `library("quextras")` appears in system/ulib/quux/BUILD.gn because quux
    # and quextras want to share some private source code or for whatever
    # reason we've decided putting them in a single directory is right.
    # Because we're not using the target with the name of its directory,
    # the `:name` syntax selects the specific target within that BUILD.gn file.
    # For the derived target names, we use `.` before the suffix.
    # In fact, "quux:headers" is just an alias for "quux:quux.headers", etc.
    "//zircon/system/ulib/quux:quextras",
    "//zircon/system/ulib/quux:quextras_more.static",
    "//zircon/system/ulib/quux:quextras_way_more.shared",

    # This is a `library()` that will set `static=false shared=true`
    # so `zircon:static` here wouldn't work but `zircon:shared` would work.
    "//zircon/system/ulib/zircon",
  ]

  # Per-module compilation flags are always optional.
  # *Note*: For cases where the flag order matters, it may be necessary
  # to use a config() instead.
  cflags = [ "-Wfoo", "-fbar" ]
  cflags_cc = [ "-fonly-for-c++" ]
  cflags_c = [ "-fonly-for-c" ]
  asmflags = [ "-Wa,--some-as-switch" ]
  ldflags = [ "-Wl,--only-affects-shlib-link" ]
}
```

A heavily abridged real-world example of a kernel module:

```gn
# deps = [ "//zircon/kernel/object" ] gets -Ikernel/object/include
library("object") {
  kernel = true
  sources = [
    "buffer_chain.cpp",
    "process_dispatcher.cpp",
  ]
  deps = [
    "//zircon/kernel/dev/interrupt",
    "//zircon/system/ulib/fbl",
  ]
}
```

Note `system/ulib/fbl` is not `kernel/lib/fbl`: the one `fbl` serves
all. Here's a heavily abridged example for that case:

```gn
library("fbl") {
  kernel = true
  static = true
  sources = [
    "alloc_checker.cpp",
  ]
  if (is_kernel) {
    sources += [
      "arena.cpp",
      "arena_tests.cpp",
    ]
  } else {
    sources += [ "string.cpp" ]
  }
}
```

The actual `fbl` is a bad example because it has other complications, but this
demonstrates how a library of shared code can be maintained in one place with
one `BUILD.gn` file using one library target to describe both the kernel and
userland incarnations.  They share everything, but can differ as needed based
on `is_kernel` conditionals.

Libraries define a standard set of targets (if relevant):

 * `$target_name.headers`
   is always provided, for just getting the headers and not linking it in
 * `$target_name.static`
   is provided if `static = true` (the default)
 * `$target_name.shared`
   is provided if `shared = true`

If the library is the main target in the file (e.g. `//zircon/foo:foo`)--the common
case--the `static`, `shared`, and `headers` sub-targets are aliased into
`//zircon/foo:static`, `//zircon/foo:shared`, and `//zircon/foo:headers`.

### `public_deps` for header dependencies

In addition to `deps` and `data_deps`, GN also has `public_deps`. This is used
when a target exposes a dependency in its public header files and needs to
forward that dependency's settings up the dependency chain. Every use of
`public_deps` should have a comment explaining why it's needed:

For example, `library("async-loop")` contains this:

```gn
  public_deps = [
    # <lib/async-loop/loop.h> has #include <lib/async/dispatcher.h>.
    "//zircon/system/ulib/async:headers",
  ]
```

## `source_set()` and `static_library()`

Some code that doesn't have an include directory can just use the
native GN `source_set()` or `static_library()` targets.

A source set (see `gn help source_set`) is a way to create a logical grouping
of files or to scope compilation switches narrowly. The object files will be
linked directly into final binaries without going through any intermediate
libraries. In contrast, the files in a static library are only pulled in
as-needed to resolve symbols.

  * Code in the kernel itself should always use `source_set`. Static libraries
    currently interact poorly with inline assembly.

  * A `source_set` *must* be used when creating groups of tests since the
    test harness depends on static initializers while the static library
    linking rules will strip the tests. All kernel code.

  * A `static_library` should be used for a higher-level thing that looks like
    a library or a part of one. Dead code stripping is more efficient, and can
    produce faster links and smaller binaries in cases where some code isn't
    needed.

```gn
source_set("some_code") {
  sources = [
    "this.c",
    "that.cpp",
  ]
}
```

## `loadable_module()`

This is not really used in the Zircon build so far, but could be. A loadable
module is a shared object that's not linked directly but rather loaded
dynamically via `dlopen()` or the like.

`loadable_module()` takes the `install_path` parameter like `executable()`
does.  But it has no default path, so it's like `install_path = false` unless
you supply a path explicitly.

Zircon device drivers are loadable modules, but they have their own special
templates that should be used instead of `loadable_module()`.

## `driver()` and `test_driver()`

Drivers are loadable modules with some special support and constraints.

 * They get a default `install_path` appropriate for drivers, so they will be
   found by `devmgr`.
 * They implicitly depend on `libdriver` so it shouldn't be listed in `deps`.
 * They implicitly use the static C++ standard library.

`test_driver()` is to `driver()` as `test()` is to `executable()`.

```gn
driver("fvm") {
  sources = [
    "fvm.cpp",
  ]
  deps = [
    "//src/lib/storage/fs/cpp",
    "//zircon/system/ulib/ddktl",
    "//zircon/system/ulib/zircon",
  ]
}
```

### `resources()` and `firmware()`

A `resource()` target declares some file that might be needed in the BOOTFS
image, but doesn’t directly cause anything to happen in the build.  The style
of the rule is as if it’s a copy from a source file to an output file in the
build; it’s modelled on GN’s native `copy()` rule, and `gn help copy` explains
why its syntax is exactly the way it is.  `outputs` is single-element list
containing a path in the BOOTFS.

```gn
import("//zircon/public/gn/resource.gni")

resource("tables") {
  sources = [
    "data.tbl",
  ]
  outputs = [
    "data/some_lib/data_v1.tbl",
  ]
}
```

The purpose of `resource()` is to be listed in the `data_deps` of the target
that uses the data:

```gn
library("uses_tables") {
  sources = [
    "read_table.cc",
  ]
  data_deps = [
    ":tables",
  ]
}
```

This can be a `library()`, an `executable()`, a `source_set()`, etc.  Good
practice is to put the `data_deps` in the finest-grained target that holds the
code that uses the file at runtime.  Doing so ensures that the relevant
resource will be available at runtime.

If the resource is generated by the build, then the path in the `sources` list
identifies its location in the build directory, usually using
`$target_out_dir` or `$target_gen_dir`.  In that case, the `resource()` must
also have a `deps` list that includes the target that generates that file.

The build also allows for a special type of resource that is generated from
the dependency graph.  Using `generated_resource()` creates a resource file
that is intended for use in `data_deps`, as in a normal `resource()`, but
instead of using an existing source file it will generate a file at `gn gen`
time with fixed contents or based on a metadata collection (see `gn help
generated_file` for details).

`firmware()` is a special-case variant of `resource()`, intended for drivers.
It places the resource in `/lib/firmware/$path`, where `$path` is a relative
path to the resource in the `/lib/firmware` root.  This mimics the calling
convention in `devhost`, where a driver calls `load_firmware(...)` on a
relative path.

## `fidl_library()`

This template allows the definition of a FIDL library and its associated
bindings.  Declaring a `fidl_library()` target will cause the build to
generate bindings for all supported languages.

Note: To use this template, you must import the `fidl.gni` file scope.

```gn
import("//zircon/public/gn/fidl.gni")

# Defined in //zircon/system/fidl/fuchsia-io/BUILD.gn
fidl_library("fuchsia-io") {
  sources = [
    "io.fidl",
  ]
  public_deps = [
    "//zircon/system/fidl/fuchsia-mem",
  ]
}
```

Note the use of [`public_deps`](#public_deps).  When a FIDL library's source
files have `using other_library;` that's equivalent to a C/C++ library using
`#include <other_library/header>` in its public headers.  Since this is very
common for FIDL (and Banjo) libraries, we don't require comments on every case
when it follows this simple pattern.

Depending on which bindings are defined, the above example will generate a set
of targets of the form `//zircon/system/fidl/fuchsia-io:fuchsia-io.<language>`, or,
in the case where the target name is the same as the directory name as above,
`//zircon/system/fidl/fuchsia-io:<language>`.

The common case today is `"//zircon/system/fidl/fuchsia-io:c"`.

## `banjo_library()`

The definition of Banjo libraries is similar to that of FIDL libraries.  A
`banjo_libary()` target will generate bindings for all supported languages,
though the set of supported languages will be different from that of FIDL.

```gn
import("//zircon/public/gn/banjo.gni")

banjo_library("ddk-driver") {
  sources = [
    "driver.banjo",
  ]
}
```

Currently, listing the plain target with no `:<language>` suffix in `deps`
gets both the C and C++ bindings.  This will probably change in the near
future to more closely follow the FIDL model: specify exactly which bindings
you depend on.

See above about `public_deps`.  Its use in `banjo_library()` is exactly like
its use in `fidl_library()`.
