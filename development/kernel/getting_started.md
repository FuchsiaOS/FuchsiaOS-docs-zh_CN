# Getting started with Zircon

## Checking out the Zircon source code

Note: The Fuchsia source includes Zircon. See Fuchsia's
[Getting Started](/docs/get-started/README.md) documentation.

This guide assumes that the Fuchsia project is checked out into `$FUCHSIA_DIR`,
and `fx` has been configured.

## Build Zircon with the default toolchain

The `fx` command wraps the various tools used to configure, build and interact
with Fuchsia. The `fx set` command is used to specify the product and the board
architecture. For example, to set your build target to be Zircon for `arm64`,
run the following command:

```sh
fx set bringup.arm64
```

Fuchsia uses the concept of
[products](/docs/concepts/build_system/boards_and_products.md#products) to
create a collection of build targets. The
[bringup product](/docs/concepts/build_system/boards_and_products.md#bringup-product)
is the smallest product with a minimal feature set.

The following command prints a list of other product configurations:

```sh
fx list-products
```

The following command prints a list of the defined board architectures:

```sh
fx list-boards
```

To execute the build, run the following command:

```sh
fx build
```

The build results are saved in `$FUCHSIA_DIR/out/default`.

## Explicitly set the target toolchain

By default Fuchsia uses the `clang` toolchain. This can be set to `gcc` by using
the `variants` argument with `fx set`:

```sh
fx set bringup.x64 --variant gcc
```

You can also enable asan by using the variant flag.

## Building Zircon for all targets

You can build for all targets with `fx multi` and using a file that contains all
the specifications to build. The output for each target is found in
`$FUCHSIA_DIR/out/<product>.<board>.variant`. An example of a multi build spec
is <code>[bringup-cq](/tools/devshell/lib/multi-specs/bringup-cq)</code>, which
approximates what is built for a CQ test.

Please build for all targets before submitting to ensure builds work on all
architectures.

## QEMU

You can skip this if you're only testing on actual hardware, but the emulator is
handy for quick local tests and generally worth having around.

See [QEMU](/docs/development/debugging/qemu.md) for information on building and
using QEMU with zircon.

## Build Toolchains (Optional)

If the prebuilt toolchain binaries do not work for you, you can build your own
from vanilla upstream sources.

*   The Clang toolchain is used to build Zircon by default or if you build with
    `variants = [ "clang" ]` or `variants = [ "asan" ]`.
*   The Clang toolchain is also used by default to build host-side code, but any
    C++14-capable toolchain for your build host should work fine.
*   The GCC toolchain is also available.

Build one or the other or both, as needed for how you want build Zircon.

### GCC Toolchain

We use GNU `binutils` 2.30[^1] and GCC 8.2, configured with
`--enable-initfini-array --enable-gold`, and with `--target=x86_64-elf
--enable-targets=x86_64-pep` for x86-64 or `--target=aarch64-elf` for ARM64.

For `binutils`, we recommend `--enable-deterministic-archives` but that switch
is not necessary to get a working build.

For GCC, it's necessary to pass `MAKEOVERRIDES=USE_GCC_STDINT=provide` on the
`make` command line. This should ensure that the `stdint.h` GCC installs is one
that works standalone (`stdint-gcc.h` in the source) rather than one that uses
`#include_next` and expects another `stdint.h` file installed elsewhere.

Only the C and C++ language support is required and no target libraries other
than `libgcc` are required, so you can use various `configure` switches to
disable other things and make your build of GCC itself go more quickly and use
less storage, e.g. `--enable-languages=c,c++ --disable-libstdcxx
--disable-libssp --disable-libquadmath`. See the GCC installation documentation
for more details.

You may need various other `configure` switches or other prerequisites to build
on your particular host system. See the GNU documentation.

[^1]: The `binutils` 2.30 release has some harmless `make check` failures in the
    `aarch64-elf` and `x86_64-elf` configurations. These are fixed on the
    upstream `binutils-2_30-branch` git branch, which is what we actually
    build. But the 2.30 release version works fine for building Zircon; it
    just has some spurious failures in its own test suite.

### Clang/LLVM Toolchain

We use a trunk snapshot of Clang and update to new snapshots frequently. Any
build of recent-enough Clang with support for `x86_64` and `aarch64` compiled in
should work. You'll need a toolchain that also includes the runtime libraries.
We normally also use the same build of Clang for the host as well as for the
`*-fuchsia` targets. See [here](/docs/development/build/toolchain.md) for
details on how we build Clang.

### Set up build arguments for toolchains

If you're using the prebuilt toolchains, you can skip this step, since the build
will find them automatically.

Set the build argument that points to where you installed the toolchains:

```sh
fx set bringup.x64 --variant clang --args clang_tool_dir = "<absolute path to>/clang-install/bin/"
```

or for GCC:

```sh
fx set bringup.x64 --variant gcc --args gcc_tool_dir = "<absolute path to>/gcc-install/bin/"
```

Note that `*_tool_dir` should have a trailing slash. If the `clang` or `gcc` in
your `PATH` works for Zircon, you can just use empty prefixes.

## Copying files to and from Zircon

With local link IPv6 configured you can use `fx cp` to copy files to and from
the device.

## Including Additional Userspace Files

The Zircon build creates a bootfs image containing necessary userspace
components for the system to boot (the device manager, some device drivers,
etc). The kernel is capable of including a second bootfs image, which is provided
by QEMU or the bootloader as a ramdisk image.

To create such a bootfs image, use the zbi tool that's generated as part of the
build. It can assemble a bootfs image for either source directories (in which
case every file in the specified directory and its subdirectories are included)
or via a manifest file that specifies, on a file-by-file basis, which files to
include.

```none
$BUILDDIR/tools/zbi -o extra.bootfs @/path/to/directory

echo "issue.txt=/etc/issue" > manifest
echo "etc/hosts=/etc/hosts" >> manifest
$BUILDDIR/tools/zbi -o extra.bootfs manifest
```

On the booted Zircon system, the files in the bootfs will appear under /boot, so
in the above manifest example, the "hosts" file would appear at /boot/etc/hosts.

## Network Booting

Network booting is supported via two mechanisms: Gigaboot and Zirconboot.
Gigaboot is an EFI based bootloader whereas zirconboot is a mechanism that
allows a minimal zircon system to serve as a bootloader for zircon.

On systems that boot via EFI (such as Acer and NUC), either option is viable. On
other systems, zirconboot may be the only option for network booting.

### Via Gigaboot

The [GigaBoot20x6](/src/firmware/gigaboot) bootloader speaks a simple network
boot protocol (over IPV6 UDP), which does not require any special host
configuration or privileged access to use.

It does this by taking advantage of IPV6 Link Local Addressing and Multicast,
allowing the device being booted to advertise its bootability and the host to
find it and send a system image to it.

If you have a device (for example a Broadwell or Skylake Intel NUC) running
GigaBoot20x6, first
[create a USB drive](/docs/development/hardware/usb_setup.md).

```none
$BUILDDIR/tools/bootserver $BUILDDIR/zircon.bin

# if you have an extra bootfs image (see above):
$BUILDDIR/tools/bootserver $BUILDDIR/zircon.bin /path/to/extra.bootfs
```

By default bootserver will continue to run and every time it observes a netboot
beacon it will send the kernel (and bootfs if provided) to that device. If you
pass the -1 option, bootserver will exit after a successful boot instead.

### Via Zirconboot

Zirconboot is a mechanism that allows a zircon system to serve as the bootloader
for zircon itself. Zirconboot speaks the same boot protocol as Gigaboot
described above.

To use zirconboot, pass the `netsvc.netboot=true` argument to zircon via the
kernel command line. When zirconboot starts, it will attempt to fetch and boot
into a zircon system from a bootserver running on the attached host.

## Network Log Viewing

The default build of Zircon includes a network log service that multicasts the
system log over the link local IPv6 UDP. Please note that this is a quick hack
and the protocol will certainly change at some point.

For now, if you're running Zircon on QEMU with the -N flag or running on
hardware with a supported ethernet interface (ASIX USB Dongle or Intel Ethernet
on NUC), the loglistener tool will observe logs broadcast over the local link:

```none
$BUILDDIR/tools/loglistener
```

## Debugging

For random tips on debugging in the zircon environment see
[debugging](/docs/development/debugging/tips.md).

## Contribute changes

*   See [CONTRIBUTING.md](/CONTRIBUTING.md#contributing-patches-to-zircon)
