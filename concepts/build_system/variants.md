# Fuchsia Build System: Variants

The Fuchsia GN build machinery allows for separate components to be built
in different "variants".  A variant usually just means using extra compiler
options, but they can do more than that if you write some more GN code.
The variants defined so far enable things like
[sanitizers](https://github.com/google/sanitizers/wiki) and
[LTO](https://llvm.org/docs/LinkTimeOptimization.html).

The GN build argument
[`select_variant`](/docs/gen/build_arguments.md#select_variant)
controls which components are built in which variants.  It applies
automatically to every `executable`, `loadable_module`, or `driver_module`
target in GN files.  It's a flexible mechanism in which you give a list of
matching rules to apply to each target to decide which variant to use (if
any).  To support this flexibility, the value for `select_variant` uses a
detailed GN syntax.  For simple cases, this can just be a list of strings.

Using `fx set`:

```sh
fx set core.x64 --variant=host_asan --variant=asan/cat --variant=asan/ledger
```

Alternatively, you can add or modify the variants on an existing build by
editing the GN args (substituting your build's GN output directory
for `out/default` as necessary):

```sh
gn args out/default
```

That command will bring up an editor. Append to that file:

```
select_variant = [ "host_asan", "asan/cat", "asan/ledger" ]
```

 1. The first switch applies the `host_asan` matching rule, which enables
    [AddressSanitizer](https://clang.llvm.org/docs/AddressSanitizer.html)
    for all the executables built to run on the build host.

 2. The second switch applies the `asan` matching rule, which enables
    AddressSanitizer for executables built to run on the target (i.e. the
    Fuchsia device).  The `/cat` suffix constrains this matching rule only
    to the binary named `cat`.

 3. The third switch is like the second, but matches the binary named `ledger`.

The GN code supports much more flexible matching rules than just the binary
name, but there are no shorthands for those. See the
[`select_variant`](/docs/gen/build_arguments.md#select_variant)
build argument documentation for more details.

To see the list of variants available and learn more about how to define
new ones, see the
[`known_variants`](/docs/gen/build_arguments.md#known_variants)
build argument.

## Troubleshooting notes

### Replicating ASan failures

Our commit queue runs tests in an ASan-enabled configuration. To replicate the
build in this configuration, use the following `args.gn` file:

```sh
import("//boards/<x64-or-arm64>.gni")
import("//products/core.gni")

base_package_labels+=[ "//bundles/buildbot:core" ]
goma_dir="<path-to-goma-dir>"
is_debug=true
select_variant=["asan","host_asan"]
target_cpu="<x64-or-arm64>"
use_goma=true
```

Replace `x64-or-arm64` with your desired target architecture, and replace
`<path-to-goma-dir>` with the path to your goma dir (for those who use goma). This
can also be generated from the command line with:

```sh
fx set core.x64 --with-base //bundles/buildbot:core --variant host_asan --variant asan --goma
```

Note that this will build all of the tests that are run by the commit queue and
install them in the system image. This may be undesirable for two reasons:

 * Building all of the tests is typically slow and unnecessary. Developers may
   find it more effective to limit the package labels to the tests they need.
 * Installing all of the tests in the system image ahead of time means that the
   software deployment workflow does not get exercised.

### Launching executables from within ASan-enabled binaries

If you are trying to use the ASan variant, you may encounter an error that looks
like this:

```sh
launcher: error: Launch: elf_load: handle_interp failed
dlsvc: could not open 'asan/ld.so.1'
```

Fuchsia is structured around packages and components. Each component contains
all of the shared libraries it needs to run. This helps Fuchsia avoid library
versioning issues that plague other operating systems. It also means that, if
you want to run a binary from within a component, you must provide the
appropriate shared library loader for that binary.

There are a set of command line programs located in the `/boot/` directory of
Fuchsia installs that are not contained in packages, but in the boot filesystem.
These programs do not have their own shared library loader, and will use
whatever shared libraries the component executing them provides. This normally
works, as programs like `sh` and `ls` have very minimal, very common
dependencies. However, there's no guarantee that the component's package will
have sufficient or compatible shared libraries for the command line program's
needs. ASan-enabled packages usually do not contain the right launcher for these
programs, so most ASan-enabled components cannot run executables out of
`/boot`. If an ASan-enabled component tries to do so, it gets the error above.

Fortunately, it turns out that the fix involves doing what all packages should
do anyway, which is to declare their dependencies explicitly. If your package
depends on a binary, it should declare it as a dependency, and then use that
declared dependency instead of the one in the `/boot` directory. In the case of
our build system, the `zircon_extras_manifest` rule defined in
`//build/config/fuchsia/zircon_images.gni` will allow you to depend on any of
the binaries found in the `/boot` directory. They will be installed in
`/pkg/bin/`, and you should execute them from there.
