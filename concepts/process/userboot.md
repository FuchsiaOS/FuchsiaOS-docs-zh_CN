# Zircon kernel to userspace bootstrapping (`userboot`)

Zircon has a microkernel style of design.  A complexity for microkernel
designs is how to bootstrap the initial userspace processes.  Often this
is accomplished by having the kernel implement minimal versions of
filesystem reading and program loading just for the purpose of
bootstrapping, even when those kernel facilities are never used after boot
time.  Zircon takes a different approach.

[TOC]

## Boot loader and kernel startup

A boot loader loads the kernel into memory and transfers control to the
kernel's startup code.  The details of the boot loader protocols are not
described here.  The boot loaders used with Zircon load both the kernel
image and a data blob in Zircon Boot Image format.
The [ZBI format](/zircon/system/public/zircon/boot/image.h) is a
simple container format that embeds items passed by the boot loader,
including hardware-specific information,
the [kernel "command line"](/docs/reference/kernel/kernel_cmdline.md) giving boot options, and RAM
disk images (which are usually compressed).  The kernel extracts some
essential information for its own use in the early stages of booting.

## BOOTFS

One of the items embedded in the Zircon Boot Image is an initial RAM disk
filesystem image.  The image is usually compressed using the **LZ4**
format.  Once decompressed, the image is in **BOOTFS** format.  This is a
trivial read-only filesystem format that simply lists file names, and for
each file the offset and size within the BOOTFS image (both values must be
page-aligned both fields and are limited to 32 bits).

The primary BOOTFS image contains everything that the userspace system
needs to run: executables, shared libraries, and data files.  These include
the implementations of device drivers and more advanced filesystems that
make it possible to read more code and data from storage or network
devices.

After the system has bootstrapped itself, the files in the primary
BOOTFS become the read-only filesystem tree rooted at `/boot` (and served by
component manager).

## Kernel loads userboot

The kernel does not include any code for decompressing LZ4 format, nor
any code for interpreting the BOOTFS format.  Instead, all of this work
is done by the first userspace process, called `userboot`.

`userboot` is a normal userspace process.  It can only make the standard
system calls through the [vDSO](/docs/concepts/kernel/vdso.md) like any other process would, and
is subject to the full [vDSO enforcement](/docs/concepts/kernel/vdso.md#Enforcement) regime.
What's special about `userboot` is the way it gets loaded.

`userboot` is built as an ELF dynamic shared object, using the
same [RODSO layout](/docs/concepts/kernel/vdso.md#read-only-dynamic-shared-object-layout) as
the vDSO.  Like the vDSO, the `userboot` ELF image is embedded in the
kernel at compile time.  Its simple layout means that loading it does
not require the kernel to interpret ELF headers at boot time.  The
kernel only needs to know three things: the size of the read-only
segment, the size of the executable segment, and the address of the
`userboot` entry point.  At compile time, these values are extracted
from the `userboot` ELF image and used as constants in the kernel code.

Like any other process, `userboot` must start with the vDSO already
mapped into its address space so it can make system calls.  The kernel
maps both `userboot` and the vDSO into the first user process, and then
starts it running at the `userboot` entry point.

## Kernel sends `processargs` message

In normal [program loading](program_loading.md),
a [*bootstrap message*](program_loading.md#the-processargs-protocol) is
sent to each new process.  The process's first thread receives
a [channel](/docs/reference/kernel_objects/channel.md) handle in a register.  It can then read
data and handles sent by its creator.

The kernel uses the exact same protocol to start `userboot`.  The kernel
command line is split into words that become the environment strings in the
bootstrap message.  All the handles that `userboot` itself will need, and
that the rest of the system will need to access kernel facilities, are
included in this message.  Following the normal format, *handle info
entries* describe the purpose of each handle.  These include
the [`PA_VMO_VDSO` handle](/docs/concepts/kernel/vdso.md#pa_vmo_vdso-handle).

## userboot finds system calls in the vDSO

The [standard convention](/docs/concepts/kernel/vdso.md#process_start_argument) for informing
a new process of its vDSO mapping requires the process to interpret the
vDSO's ELF headers and symbol table to locate system call entry points.
To avoid this complexity, `userboot` finds the entry points in the vDSO
in a different way.

When the kernel maps `userboot` into the first user process, it chooses
a random location in memory, just as normal program loading does.
However, when it maps the vDSO in it doesn't choose another random
location as is normal.  Instead, it places the vDSO image immediately
after the `userboot` image in memory.  This way, the vDSO code is always
at fixed offsets from the `userboot` code.

At compile time, the symbol table entries for all the system call entry
points are extracted from the vDSO ELF image.  These are then massaged
into linker script symbol definitions that use each symbol's fixed
offset into the vDSO image to define that symbol at that fixed offset
from the linker-provided `_end` symbol.  In this way, the `userboot`
code can make direct calls to each vDSO entry point in the exact
location it will appear in memory after the `userboot` image itself.

## userboot decompresses BOOTFS

The first thing `userboot` does is to read the bootstrap message sent by
the kernel.  Among the handles it gets from the kernel is one with
*handle info entry* `PA_HND(PA_VMO_BOOTDATA, 0)`.  This is
a [VMO](/docs/reference/kernel_objects/vm_object.md) containing the ZBI from the
boot loader.  `userboot` reads the ZBI headers from this VMO
looking for the first item with type `ZBI_TYPE_STORAGE_BOOTFS`.  That
contains the [BOOTFS](#BOOTFS) image.  The item's ZBI header
indicates if it's compressed, which it usually is.  `userboot` maps in
this portion of the VMO.  `userboot` contains LZ4 format support code,
which it uses to decompress the item into a fresh VMO.

## userboot loads the first "real" user process from BOOTFS

Next, `userboot` examines the environment strings it received from the
kernel, which represent the kernel command line.  If there is a string
`userboot.next=`*file*+*optional_arg1*+*optional_arg2=foo*+... then *file*
will be loaded as the first real user process with the '+' separated
arguments passed to it. If no such option is present, the default *file* is
`bin/component_manager+--boot`.  The files are found in the BOOTFS image.

To load the file, `userboot` implements a full-featured ELF program loader.
Usually the file being loaded is a dynamically-linked executable with a
`PT_INTERP` program header.  In this case, `userboot` looks for the file
named in `PT_INTERP` and loads that instead.

Then `userboot` loads the vDSO at a random address.  It starts the new
process with the standard conventions, passing it a channel handle and the
vDSO base address.  On that channel, `userboot` sends the
standard [`processargs`](program_loading.md#the-processargs-protocol)
messages.  It passes on all the important handles it received from the
kernel (replacing specific handles such as the process-self and thread-self
handles with those for the new process rather than for `userboot` itself).

## userboot loader service

Following the standard program loading protocol, when `userboot` loads a
program via `PT_INTERP`, it sends an additional `processargs` message
before the main message, intended for the use of the dynamic linker.  This
message includes a `PA_LDSVC_LOADER` handle for a channel on which `userboot`
provides a minimal implementation of the
standard [loader service](program_loading.md#the-loader-service).

`userboot` has only a single thread, which remains in a loop handling
loader service requests until the channel is closed.  When it receives a
`LOADER_SVC_OP_LOAD_OBJECT` request, it looks up the object name prefixed
by `lib/` as a file in BOOTFS and returns a VMO of its contents.  Thus, the
first "real" user process can be (and usually is) a dynamically linked
executable needing various shared libraries.  The dynamic linker, the
executable, and the shared libraries are all loaded from the same BOOTFS
pages that will later appear as files in `/boot`.

An executable that will be loaded by `userboot` (i.e. `component manager`) should
normally close its loader service channel once it's completed startup.
That lets `userboot` know that it's no longer needed.

## userboot rides off into the sunset

When the loader service channel is closed (or if the executable had no
`PT_INTERP` and so no loader service was required, then as soon as the
process has been started), `userboot` no longer has anything to do.

If [the `userboot.shutdown` option was given on the kernel command line](/docs/reference/kernel/kernel_cmdline.md#userboot-shutdown),
then `userboot` waits for the process it started to exit, and then shuts
down the system (as if by the `dm shutdown` command).  This can be useful
to run a single test program and then shut down the machine (or emulator).
For example, the command line `userboot.next=bin/core-tests userboot.shutdown`
runs the Zircon core tests and then shuts down.

Otherwise, `userboot` does not wait for the process to exit.  `userboot`
exits immediately, leaving the first "real" user process in charge of
bringing up and taking down the rest of the system.
