# Zircon Kernel Command Line Options

See [//docs/gen/boot-options.md](/docs/gen/boot-options.md) is now the
source of truth.

The Zircon kernel receives a textual command line from the bootloader, which can
be used to alter some behaviours of the system. Kernel command line parameters
are in the form of *option* or *option=value*, separated by spaces, and may not
contain spaces.

For boolean options, *option=0*, *option=false*, or *option=off* will disable
the option. Any other form (*option*, *option=true*, *option=wheee*, etc) will
enable it.

The kernel command line is passed from the kernel to the userboot process and
the device manager, so some of the options described below apply to those
userspace processes, not the kernel itself.

If keys are repeated, the last value takes precedence, that is, later settings
override earlier ones.

The devmgr reads the file /boot/config/devmgr (if it exists) at startup and
imports name=value lines into its environment, augmenting or overriding the
values from the kernel command line. Leading whitespace is ignored and lines
starting with # are ignored. Whitespace is not allowed in names.

In order to specify options in the build, see
[this guide](/docs/development/kernel/build.md#options).

## blobfs.cache-eviction-policy=\<policy\>

Controls blobfs' eviction strategy for pager-backed blobs with no open handles
or VMO clones. If unset, an internally defined system default is used.

Blobs that are not pager-backed are not affected by this knob.

The following values are supported:

*   `NEVER_EVICT`: Nodes are never evicted. It is recommended to enable kernel
    page eviction (`kernel.page-scanner.enable-eviction`) in this case, as
    otherwise blobfs will indefinitely retain all data pages in memory.
*   `EVICT_IMMEDIATELY`: Nodes are evicted as soon as they have no open handles
    or VMO clones. They will need to be loaded from disk again on next access.

## blobfs.write-compression-algorithm=\<algorithm\>

The compression algorithm that blobfs should use for writing blobs at runtime.
If unset, an internally defined system default is used.

The following values are supported:

*   `ZSTD_CHUNKED`
*   `UNCOMPRESSED`

## boot.usb=\<bool\>

If true, indicates that the boot storage medium is connected over a USB bus.
This is used by the live_usb component in //src/sys/live_usb to determine
whether or not it should run.

## bootsvc.next=\<bootfs path\>

Controls what program is executed by bootsvc to continue the boot process. If
this is not specified, the default next program will be used.

Arguments to the program can optionally be specified using a comma separator
between the program and individual arguments. For example,
'bootsvc.next=bin/mybin,arg1,arg2'.

## clock\.backstop=\<seconds\>

Sets the initial offset (from the Unix epoch, in seconds) for the UTC clock. The
clock will be set by the device coordinator at boot time, and then later, if an
RTC is present, the RTC clock will be sanitized to at least this time.

## console.shell=\<bool\>

If this option is set to true driver_manager will launch the shell if
kernel.shell has not already been launched. Defaults to false.

If this is false, it also disables the zircon.autorun.boot and
zircon.autorun.system options.

## console.allowed\_log\_tags=\<tag\>,\<tag\>...

Add a tag to the allow list. Log entries with matching tags will be output to
the console. This takes precedent over tags passed via process args.

## console.denied\_log\_tags=\<tag\>,\<tag\>...

Add a tag to the deny list. Log entries with matching tags will be prevented
from being printed the console. This takes precedent over tags passed via
process args as well as the allow list.

## driver.\<name>.disable

Disables the driver with the given name. The driver name comes from the
zircon\_driver\_info, and can be found as the first argument to the
ZIRCON\_DRIVER\_BEGIN macro.

Example: `driver.usb_audio.disable`

## driver.\<name>.log=\<flags>

Set the minimum log severity for a driver. The textual constants "error",
"warn", "info", "debug", "trace", may be used, and they map to the corresponding
bits in DDK\_LOG\_... in `ddk/debug.h`. If an unknown value is passed, the
minimum log severity will be set to "trace". The default minimum log severity
for a driver is "info".

Note again that the name of the driver is the "Driver" argument to the
ZIRCON\_DRIVER macro. It is not, for example, the name of the device, which for
some drivers is almost identical, except that the device may be named "foo-bar"
whereas the driver name must use underscores, e.g., "foo_bar".

## driver.\<name>.tests.enable=\<bool>

Enable the unit tests for an individual driver. The unit tests will run before
the driver binds any devices. If `driver.tests.enable` is true then this
defaults to enabled, otherwise the default is disabled.

Note again that the name of the driver is the "Driver" argument to the
ZIRCON\_DRIVER macro. It is not, for example, the name of the device, which for
some drivers is almost identical, except that the device may be named "foo-bar"
whereas the driver name must use underscores, e.g., "foo_bar".

## driver.sysmem.protected_memory_size=\<num>

Overrides the board-driver-specified size for sysmem's default protected memory
pool. Value is in bytes.

## driver.sysmem.protected_memory_size=\<num>

Overrides the board-driver-specified size for sysmem's contiguous memory pool.
Value is in bytes.

## driver.tests.enable=\<bool>

Enable the unit tests for all drivers. The unit tests will run before the
drivers bind any devices. It's also possible to enable tests for an individual
driver, see `driver.\<name>.enable_tests`. The default is disabled.

## driver.tracing.enable=\<bool>

Enable or disable support for tracing drivers. When enabled drivers may
participate in
[Fuchsia tracing](/docs/development/drivers/diagnostics/tracing.md).

Implementation-wise, what this option does is tell each devhost whether to
register as "trace provider".

The default is enabled. This options exists to provide a quick fallback should a
problem arise.

## driver.iommu.enable=\<bool>

This option (disabled by default) allows the system to use a hardware IOMMU if
present.
### x64 specific values

On x64, some additional values are supported for configuring 8250-like UARTs:

-   If set to `legacy`, the legacy COM1 interface is used.
-   If set to `acpi`, the UART specified by the `DBG2` ACPI entry on the system
    will be used, if available.
-   A port-io UART can be specified using `ioport,\<portno>,\<irq>`.
-   An MMIO UART can be specified using `mmio,\<physaddr>,\<irq>`.

For example, `ioport,0x3f8,4` would describe the legacy COM1 interface.

All numbers may be in any base accepted by *strtoul*().

All other values are currently undefined.

## ldso.trace

This option (disabled by default) turns on dynamic linker trace output. The
output is in a form that is consumable by clients like Intel Processor Trace
support.

## live_usb.is_system=\<bool>

This option indicates to the `live_usb` component that the system is booting
into a full system rather than a recovery environment. If this is not set to
true, the `live_usb` component does nothing.

## zircon.autorun.boot=\<command>

This option requests that *command* be run at boot.

Commands should be absolute paths starting at the root '/'.

Any `+` characters in *command* are treated as argument separators, allowing you
to pass arguments to an executable.

This option is disabled if console.shell is false.

## zircon.autorun.system=\<command>

This option requests that *command* be run once the system partition is mounted.
If there is no system partition, it will never be launched.

Commands should be absolute paths starting at the root '/'.

Any `+` characters in *command* are treated as argument separators, allowing you
to pass arguments to an executable.

This option is disabled if console.shell is false.

## zircon.system.disable-automount=\<bool>

This option prevents the fshost from auto-mounting any disk filesystems
(/system, /data, etc), which can be useful for certain low level test setups. It
is false by default. It is implied by **netsvc.netboot=true**

## zircon.system.pkgfs.cmd=\<command>

This option requests that *command* be run once the blob partition is mounted.
Any `+` characters in *command* are treated as argument separators, allowing you
to pass arguments to an executable.

The executable and its dependencies (dynamic linker and shared libraries) are
found in the blob filesystem. The executable *path* is *command* before the
first `+`. The dynamic linker (`PT_INTERP`) and shared library (`DT_NEEDED`)
name strings sent to the loader service are prefixed with `lib/` to produce a
*path*. Each such *path* is resolved to a blob ID (i.e. merkleroot in ASCII hex)
using the `zircon.system.pkgfs.file.`*path* command line argument. In this way,
`/boot/config/devmgr` contains a fixed manifest of files used to start the
process.

The new process receives a `PA_USER0` channel handle at startup that will be
used as the client filesystem handle mounted at `/pkgfs`. `/pkgfs/system` will
also be mounted as `/system`.

## zircon.system.pkgfs.file.*path*=\<blobid>

Used with [`zircon.system.pkgfs.cmd`](#zircon.system.pkgfs.cmd), above.

## zircon.system.volume=\<arg>

This option specifies where to find the "/system" volume.

It may be set to: "any", in which case the first volume of the appropriate type
will be used. "local" in which the first volume that's non-removable of the
appropriate type will be used. "none" (default), which avoids mounting anything.

A "/system" ramdisk provided by bootdata always supersedes this option.

## zircon.system.filesystem-check=\<bool>

This option requests that filesystems automatically mounted by the system are
pre-verified using a filesystem consistency checker before being mounted.

By default, this option is set to false.

## netsvc.netboot=\<bool>

If true, zircon will attempt to netboot into another instance of zircon upon
booting.

More specifically, zircon will fetch a new zircon system from a bootserver on
the local link and attempt to kexec into the new image, thereby replacing the
currently running instance of zircon.

This setting implies **zircon.system.disable-automount=true**

## netsvc.disable=\<bool>

If set to true (default), `netsvc` is disabled.

## netsvc.advertise=\<bool>

If true, netsvc will seek a bootserver by sending netboot advertisements.
Defaults to true.

## netsvc.interface=\<path>

This option instructs netsvc to use only the device whose topological path ends
with the option's value. All other devices are ignored by netsvc. The
topological path for a device can be determined from the shell by running the
`lsdev` command on the device (e.g. `/dev/class/network/000` or
`/dev/class/ethernet/000`).

This is useful for configuring network booting for a device with multiple
ethernet ports, which may be enumerated in a non-deterministic order.

## netsvc.all-features=\<bool>

This option makes `netsvc` work normally and support all features. By default,
`netsvc` starts in a minimal mode where only device discovery is supported.

## userboot.next=\<path>

This option instructs the userboot process (the first userspace process) to
execute the specified binary within the bootfs, instead of following the normal
userspace startup process (launching the device manager, etc).

It is useful for alternate boot modes (like a factory test or system unit
tests).

The pathname used here is relative to `userboot.root` (below), if set, or else
relative to the root of the BOOTFS (which later is ordinarily seen at `/boot`).
It should not start with a `/` prefix.

If this executable uses `PT_INTERP` (i.e. the dynamic linker), the userboot
process provides a
[loader service](/docs/concepts/process/program_loading.md#the-loader-service)
to resolve the `PT_INTERP` (dynamic linker) name and any shared library names it
may request. That service simply looks in the `lib/` directory (under
`userboot.root`) in the BOOTFS.

Arguments to the next program can optionally be specified using a '+' separator
between the program and individual arguments. The next program name will always
be provided as the first argument.

Example: `userboot.next=bin/core-tests+arg1+arg2=foo`

## userboot.root=\<path>

This sets a "root" path prefix within the BOOTFS where the `userboot.next` path
and the `lib/` directory for the loader service will be found. By default, there
is no prefix so paths are treated as exact relative paths from the root of the
BOOTFS. e.g. with `userboot.root=pkg/foo` and `userboot.next=bin/app`, the names
found in the BOOTFS will be `pkg/foo/bin/app`, `pkg/foo/lib/ld.so.1`, etc.

## userboot.shutdown

If this option is set, userboot will attempt to power off the machine when the
process it launches exits. Note if `userboot.reboot` is set then
`userboot.shutdown` will be ignored.

## virtcon.disable

Do not launch the virtual console service if this option is present.

## virtcon.hide-on-boot

If this option is present, the virtual console will not take ownership of any
displays until the user switches to it with a device control key combination.

## virtcon.keep-log-visible

If this option is present, the virtual console service will keep the debug log
(vc0) visible instead of switching to the first shell (vc1) at startup.

## virtcon.keymap=\<name>

Specify the keymap for the virtual console. "qwerty" and "dvorak" are supported.

## virtcon.font=\<name>

Specify the font for the virtual console. "9x16" and "18x32" are supported.

## zircon.nodename=\<name>

Set the system nodename, as used by `bootserver`, `loglistener`, and the
`net{addr,cp,ls,runcmd}` tools. If omitted, the system will generate a nodename
from its MAC address. This cmdline is honored by GigaBoot and Zircon.

## zircon.namegen=\<num>

Set the system nodename generation style. If omitted or unknown, the system uses
style 1. It has no effect if `zircon.nodename` is set. Older name generation
styles may be removed in the future. This cmdline is honored by GigaBoot and
Zircon.

Styles: - 0: Uses a four-word-name-style using based on the MAC address. - 1:
fuchsia-0123-4567-89ab based on the MAC address.

## zvb.current\_slot=\<\_a|\_b|\_r>

Makes Fuchsia aware of the slot booted by the bootloader. Setting this also
informs the paver that ABR is supported, and that it should update the ABR
metadata.

## zvb.boot-partition-uuid=\<UUID>

An alternative to zvb.current_slot - makes Fuchsia aware of the slot booted by
the bootloader by passing the UUID of the partition containing the Zircon kernel
that was booted. Setting this also informs the paver that ABR is supported, and
that it should update the ABR metadata.

## console.device_topological_suffix=\<path>

If this is set then console launcher will connect to the console device whose topological
path matches this suffix. If not specified then console launcher will connect to `/svc/console`.
Only has effect if kernel.shell=false.

# Additional Gigaboot Command Line Options

## bootloader.timeout=\<num>

This option sets the boot timeout in the bootloader, with a default of 3
seconds. Set to zero to skip the boot menu.

## bootloader.fbres=\<w>x\<h>

This option sets the framebuffer resolution. Use the bootloader menu to display
available resolutions for the device.

Example: `bootloader.fbres=640x480`

## bootloader.default=\<network|local|zedboot>

This option sets the default boot device to netboot, use a local zircon.bin or
to netboot via zedboot.

# How to pass the command line to the kernel

## in the emulator or Qemu, using ffx emu or fx qemu

Pass each option using -c, for example:

```
ffx emu start -c gfxconsole.font=18x32 -c gfxconsole.early=false
```

## in GigaBoot20x6, when netbooting

Pass the kernel command line at the end, after a -- separator, for example:

```
bootserver zircon.bin bootfs.bin -- gfxconsole.font=18x32 gfxconsole.early=false
```

## in GigaBoot20x6, when booting from USB flash

Create a text file named "cmdline" in the root of the USB flash drive's
filesystem containing the command line.
