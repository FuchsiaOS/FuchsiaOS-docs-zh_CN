# Zircon program loading and dynamic linking

In Zircon, the kernel is not directly involved in normal program loading.
Instead, the kernel provides the building blocks from
which userspace program loading is built, such as
[Virtual Memory Objects](/docs/reference/kernel_objects/vm_object.md), [processes](/docs/reference/kernel_objects/process.md),
[Virtual Memory Address Regions](/docs/reference/kernel_objects/vm_address_region.md), and [threads](/docs/reference/kernel_objects/thread.md).

Note: The only time that the kernel is involved in program loading is when you bootstrap the
userspace environment at system startup. See [`userboot`](userboot.md) for more information.

[TOC]

## ELF and the system ABI

The standard Zircon userspace environment uses the [Executable and Linkable Format (ELF)](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
for machine-code executable files, and provides a dynamic linker and
C/C++ execution environment based on ELF. Zircon processes can
use [system calls] only through the Zircon [vDSO](/docs/concepts/kernel/vdso.md), which is
provided by the kernel in ELF format and uses the C/C++ calling conventions
common to ELF-based systems.

Userspace code (given the appropriate capabilities) can use [system calls] to
directly create processes and load programs without
using ELF, but Zircon's standard ABI for machine code uses ELF as described here.

## Background: Traditional ELF program loading

[ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) was
introduced with Unix System V Release 4 and became the common standard
executable file format for most Unix-like systems. In these systems,
the kernel integrates program loading with filesystem access using the POSIX
`execve` API. There are some variations in how these systems load ELF programs, but
most follow this pattern:

 1. The kernel loads the file by name, and checks whether it's ELF or some
    other kind of file that system supports. This is where `#!` script
    handling is done, and non-ELF format support when present.
 2. The kernel maps the ELF image according to its `PT_LOAD` program
    headers. For an `ET_EXEC` file, this places the program's segments at
    fixed addresses in memory specified in `p_vaddr`. For an `ET_DYN`
    file, the system chooses the base address where the program's first
    `PT_LOAD` gets loaded, and following segments are placed according to
    their `p_vaddr` relative to the first segment's `p_vaddr`. Usually the
    base address is chosen randomly (ASLR).
 3. If there was a `PT_INTERP` program header, its contents (a range of
    bytes in the ELF file given by `p_offset` and `p_filesz`) is looked up
    as a file name to find another ELF file called the *ELF interpreter*.
    This must be an `ET_DYN` file. The kernel loads it in the same way as it
    loaded the executable, but always at a location of its own choosing.
    The interpreter program is usually the ELF dynamic linker with a name
    like `/lib/ld.so.1` or `/lib/ld-linux.so.2`, but the kernel loads
    whatever file is named.
 4. The kernel sets up the stack and registers for the initial thread, and
    starts the thread running with the PC at the chosen entry point address.

     * The entry point is the `e_entry` value from the ELF file header,
       adjusted by base address. When there was a `PT_INTERP`, the entry
       point is that of the interepreter rather than the main executable.
     * There is an assembly-level protocol of register and stack contents
       that the kernel sets up for the program to receive its argument and
       environment strings and an *auxiliary vector* of useful values. When
       there was a `PT_INTERP`, these include the base address, entry point,
       and program header table address from the main executable's ELF
       headers. This information allows the dynamic linker to find the main
       executable's ELF dynamic linking metadata in memory and do its work.
       When dynamic linking startup is complete, the dynamic linker jumps to
       the main executable's entry point address.

Zircon program loading is inspired by this tradition, but does it somewhat
differently. A key reason for the traditional pattern of loading the
executable before loading the dynamic linker is that the dynamic linker's
randomly-chosen base address must not intersect with the fixed addresses
used by an `ET_EXEC` executable file. Zircon does not support
fixed-address program loading (ELF `ET_EXEC` files) at all, only
position-independent executables or *PIE*s, which are ELF `ET_DYN` files.

## The **launchpad** library

The main implementation of program loading resides in
the [`launchpad` library](/zircon/system/ulib/launchpad/). It has a C API
in
[`<launchpad/launchpad.h>`](/zircon/system/ulib/launchpad/include/launchpad/launchpad.h) but
is not formally documented. The `launchpad` API is not described here. Its
treatment of executable files and process startup forms the Zircon system
ABI for program loading.
The [lowest userspace layers of the system](userboot.md) implement the same
protocols.

Filesystems are not part of the lower layers of Zircon API. Instead,
program loading is based on [VMOs](/docs/reference/kernel_objects/vm_object.md) and on IPC
protocols used through [channels](/docs/reference/kernel_objects/channel.md).

A program loading request starts with:

 * A handle to a VMO containing the executable file (`ZX_RIGHT_READ` and
   `ZX_RIGHT_EXECUTE` rights are required)
 * A list of argument strings (to become `argv[]` in a C/C++ program)
 * A list of environment strings (to become `environ[]` in a C/C++ program)
 * A list of initial [handles](/docs/concepts/kernel/handles.md), each with
   a [*handle info entry*](#handle-info-entry)

Three types of file are handled:

### A script file starting with `#!` {#hashbang}

  The first line of the file starts with `#!` and must be no more than 127
  characters long. The first non-whitespace word following `#!` is the
  *script interpreter name*. If there's anything after that, it all
  together becomes the *script interpreter argument*.

   * The script interpreter name is prepended to the original argument
     list (to become `argv[0]`).
   * If there was a script interpreter argument, it's inserted between the
     interpreter name and the original argument list (to become `argv[1]`,
     with the original `argv[0]` becoming `argv[2]`).
   * The program loader looks up the script interpreter name via
     the [loader service](#the-loader-service) to get a new VMO.
   * Program loading restarts on that script interpreter VMO with the
     modified argument list but everything else the same. The VMO handle
     for the original executable is just closed; the script interpreter only
     gets the original `argv[0]` string to work with, not the original VMO.
     There is a maximum nesting limit (currently 5) constraining how many
     such restarts will be allowed before program loading just fails.

### An ELF `ET_DYN` file with no `PT_INTERP`

  * The system chooses a random base address for the first `PT_LOAD` segment
    and then maps in each `PT_LOAD` segment relative to that base address.
    This is done by creating a [VMAR](/docs/reference/kernel_objects/vm_address_region.md) covering
    the whole range from the first page of the first segment to the last
    page of the last segment.
  * A VMO is created and mapped at another random address to hold the stack
    for the initial thread. If there was a `PT_GNU_STACK` program header
    with a nonzero `p_memsz`, that determines the size of the stack (rounded
    up to whole pages). Otherwise, a reasonable default stack size is used.
  * The [vDSO](/docs/concepts/kernel/vdso.md) is mapped into the process
    (another VMO containing an ELF image), also at a random base address.
  * A new thread is created in the process with [`zx_thread_create()`].
  * A new [channel](/docs/reference/kernel_objects/channel.md) is created, called the *bootstrap
    channel*. The program loader writes into this channel a message
    in [the `processargs` protocol](#the-processargs-protocol) format. This
    *bootstrap message* includes the argument and environment strings and
    the initial handles from the original request. That list is augmented
    with handles for:

     * the new [process](/docs/reference/kernel_objects/process.md) itself
     * its root [VMAR](/docs/reference/kernel_objects/vm_address_region.md)
     * its initial [thread](/docs/reference/kernel_objects/thread.md)
     * the VMAR covering where the executable was loaded
     * the VMO just created for the stack
     * optionally, a default [job](/docs/reference/kernel_objects/job.md) so the new
       process itself can create more processes
     * optionally, the vDSO VMO so the new process can let the processes
       it creates make system calls themselves

    The program loader then closes its end of the channel.
   * The initial thread is launched with
     the [`zx_process_start()`] system call:

      * `entry` sets the new thread's PC to `e_entry` from the executable's
        ELF header, adjusted by base address.
      * `stack` sets the new thread's stack pointer to the top of the
        stack mapping.
      * `arg1` transfers the handle to the *bootstrap channel* into the
        first argument register in the C ABI.
      * `arg2` passes the base address of the vDSO into the second argument
        register in the C ABI.

     Thus, the program entry point can be written as a C function:

     ```c
     noreturn void _start(zx_handle_t bootstrap_channel, uintptr_t vdso_base);
     ```

### An ELF `ET_DYN` file with a `PT_INTERP` {#pt-interp}

  In this case, the program loader does not directly use the VMO containing
  the ELF executable after reading its `PT_INTERP` header. Instead, it
  uses the `PT_INTERP` contents as the name of an *ELF interpreter*. This
  name is used in a request to the [loader service](#the-loader-service) to
  get a new VMO containing the ELF interpreter, which is another ELF
  `ET_DYN` file. Then that VMO is loaded instead of the main executable's
  VMO. Startup is as described above, with these differences:

   * An extra message
     in [the `processargs` protocol](#the-processargs-protocol) is written
     to the *bootstrap channel*, preceding the main bootstrap message. The
     ELF interpreter is expected to consume this *loader bootstrap message*
     itself so that it can do its work, but then leave the second bootstrap
     message in the channel and hand off the bootstrap channel handle to
     the main program's entry point. The *loader bootstrap message*
     includes only the necessary handles added by the program loader, not
     the full set that go into the main *bootstrap message*, plus these:

      * the original VMO handle for main ELF executable
      * a channel handle to the [loader service](#the-loader-service)

     These allow the ELF interpreter to do its own loading of the
     executable from the VMO and to use the loader service to get
     additional VMOs for shared libraries to load. The message also
     includes the argument and environment strings, which lets the ELF
     interpreter use `argv[0]` in its log messages, and check for
     environment variables like `LD_DEBUG`.

   * `PT_GNU_STACK` program headers are ignored. Instead, the program
     loader chooses a minimal stack size that is just large enough to
     contain the *loader bootstrap message* plus some breathing room for
     the ELF interpreter's startup code to use as call frames. This
     "breathing room" size is `PTHREAD_STACK_MIN` in the source, and is
     tuned such that with a small bootstrap message size the whole stack is
     only a single page, but a careful dynamic linker implementation has
     enough space to work in. The dynamic linker is expected to read the
     main executable's `PT_GNU_STACK` and switch to a stack of reasonable
     size for normal use before it jumps to the main executable's entry
     point.

Note: The program loader chooses three randomly-placed chunks of the new
process's address space before the program (or dynamic linker) gets
control: the vDSO, the stack, and the dynamic linker itself. To make it
possible for the program's own startup to control its address space more
fully, the program loader currently ensures that these random placements
are always somewhere in the upper half of the address space. This is
for the convenience of sanitizer runtimes, which need to reserve some lower
fraction of the address space.

## The **processargs** protocol

[`<zircon/processargs.h>`](/zircon/system/public/zircon/processargs.h) defines
the protocol for the *bootstrap message* sent on the *bootstrap channel* by
the program loader. When a process starts up, it has a handle to this
bootstrap channel and it has access to [system calls] through
the [vDSO](/docs/concepts/kernel/vdso.md). The process has only this one handle and so it can
see only global system information and its own memory until it gets more
information and handles through the bootstrap channel.

The `processargs` protocol is a one-way protocol for messages sent on the
bootstrap channel. The new process is never expected to write back onto
the channel. The program loader usually sends its messages and then closes
its end of the channel before the new process has even started. These
messages must communicate everything a new process will ever need, but the
code that receives and decodes messages in this format must run in a very
constrained environment. Heap allocation is impossible and nontrivial when
library facilities may not be available.

See the [header file](/zircon/system/public/zircon/processargs.h) for full
details of the message format. It's anticipated that this ad hoc protocol
will be replaced with a formal IDL-based protocol eventually, but the
format will be kept simple enough to be decoded by simple hand-written
code.

A bootstrap message conveys:

 * a list of initial [handles](/docs/concepts/kernel/handles.md)
 * a 32-bit *handle info entry* corresponding to each handle
 * a list of name strings that a *handle info entry* can refer to
 * a list of argument strings (to become `argv[]` in a C/C++ program)
 * a list of environment strings (to become `environ[]` in a C/C++ program)

### Handle info entry {#handle-info-entry}
The handles serve many purposes, indicated by the *handle info entry* type:

 * essential handles for the process to make [system calls](/docs/reference/syscalls/README.md):
   [process](/docs/reference/kernel_objects/process.md), [VMAR](/docs/reference/kernel_objects/vm_address_region.md),
   [thread](/docs/reference/kernel_objects/thread.md), [job](/docs/reference/kernel_objects/job.md)
 * [channel](/docs/reference/kernel_objects/channel.md) to the [loader service](#the-loader-service)
 * [vDSO](/docs/concepts/kernel/vdso.md) [VMO](/docs/reference/kernel_objects/vm_object.md)
 * filesystem-related handles: current directory, file descriptors, name
   space bindings (these encode an index into the list of name strings)
 * special handles for system processes:
   [resource](/docs/reference/kernel_objects/resource.md), [VMO](/docs/reference/kernel_objects/vm_object.md)
 * other types used for higher-layer or private protocol purposes

Most of these are just passed through by the program loader,
which does not need to know what they're for.

## The **loader service**

In dynamic linking systems, an executable file refers to and uses at
runtime additional files containing shared libraries and plugins. The
dynamic linker is loaded as an [*ELF interpreter*](#pt-interp) and is
responsible getting access to all these additional files to complete
dynamic linking before the main program's entry point gets control.

All of Zircon's standard userspace uses dynamic linking, down to the very
first process loaded by [`userboot`](userboot.md). Device drivers and
filesystems are implemented by userspace programs loaded this way. So
program loading cannot be defined in terms of higher-layer abstractions
such as a filesystem paradigm,
as
[traditional systems have done](#background_traditional-elf-program-loading).
Instead, program loading is based only on [VMOs](/docs/reference/kernel_objects/vm_object.md) and
a simple [channel](/docs/reference/kernel_objects/channel.md)-based protocol.

This *loader service* protocol is how a dynamic linker acquires VMOs
representing the additional files it needs to load as shared libraries.

This is a simple RPC protocol, defined in
[`<zircon/processargs.h>`](/zircon/system/public/zircon/processargs.h).
The code sending loader service
requests and receiving their replies during dynamic linker startup may
not have access to nontrivial library facilities.

An ELF interpreter receives a channel handle for its loader service in its
`processargs` bootstrap message, identified by the *handle info entry*
`PA_HND(PA_LDSVC_LOADER, 0)`. All requests are synchronous RPCs made
with [`zx_channel_call()`]. Both requests and replies start with the
`zx_loader_svc_msg_t` header; some contain additional data; some contain
a VMO handle. Request opcodes are:

 * `LOADER_SVC_OP_LOAD_SCRIPT_INTERP`: *string* -> *VMO handle*

   The program loader sends the *script interpreter name* from
   a [`#!` script](#hashbang) and gets back a VMO to execute in place of
   the script.

 * `LOADER_SVC_OP_LOAD_OBJECT`: *string* -> *VMO handle*

   The dynamic linker sends the name of an *object* (shared library or
   plugin) and gets back a VMO handle containing the file.

 * `LOADER_SVC_OP_CONFIG` : *string* -> `reply ignored`

   The dynamic linker sends a string identifying its *load configuration*.
   This is intended to affect how later `LOADER_SVC_OP_LOAD_OBJECT`
   requests decide what particular implementation file to supply for a
   given name.

 * `LOADER_SVC_OP_DEBUG_PRINT`: *string* -> `reply ignored`

   This is a simple ad hoc logging facility intended for debugging the
   dynamic linker and early program startup issues. It's convenient
   because the early startup code is using the loader service but doesn't
   have access to many other handles or complex facilities yet. This will
   be replaced in the future with some simple-to-use logging facility that
   does not go through the loader service.

 * `LOADER_SVC_OP_LOAD_DEBUG_CONFIG`: *string* -> *VMO handle*

   **This is intended to be a developer-oriented feature and might not
   ordinarily be available in production runs.**

   The program runtime sends a string naming a *debug configuration* of
   some kind and gets back a VMO to read configuration data from. The
   sanitizer runtimes use this to allow large options text to be stored in
   a file rather than passed directly in environment strings.

 * `LOADER_SVC_OP_PUBLISH_DATA_SINK`: *string*, *VMO handle* -> `reply ignored`

   **This is intended to be a developer-oriented feature and might not
   ordinarily be available in production runs.**

   The program runtime sends a string naming a *data sink* and transfers
   the sole handle to a VMO it wants published there. The *data sink*
   string identifies a type of data, and the VMO's object name can
   specifically identify the data set in this VMO. The client must
   transfer the only handle to the VMO (which prevents the VMO being
   resized without the receiver's knowledge), but it might still have the
   VMO mapped in and continue to write data to it. Code instrumentation
   runtimes use this to deliver large binary trace results.

## Zircon's standard ELF dynamic linker

The ELF conventions described above and
the [`processargs`](#the-processargs-protocol)
and [loader service](#the-loader-service) protocols are the permanent
system ABI for program loading. Programs can use any implementation of a
machine code executable that meets the basic ELF format conventions. The
implementation can use the [vDSO](/docs/concepts/kernel/vdso.md) [system calls].

ABI, the `processargs` data, and the loader service facilities as it sees
fit. The exact details of what handles and data that programs receive through
these protocols depend on the higher-layer program environment. Zircon's
system processes use an ELF interpreter that implements basic ELF dynamic
linking, and a simple implementation of the loader service.

Zircon's standard C library and dynamic linker have
a [unified implementation](/zircon/third_party/ulib/musl/) originally derived
from [`musl`](http://www.musl-libc.org/). It's identified by the
`PT_INTERP` string `ld.so.1`. It uses the `DT_NEEDED` strings naming
shared libraries as [loader service](#the-loader-service) *object* names.

The simple loader service maps requests into filesystem access:

 * *script interpreter* and *debug configuration* names must start with `/`
   and are used as absolute file names.
 * *data sink* names become subdirectories in `/tmp`, and each VMO
   published becomes a file in that subdirectory with the VMO's object name
 * *object* names are searched for as files in system `lib/` directories.
 * *load configuration* strings are taken as a subdirectory name,
   optionally followed by a `!` character. Subdirectories by that name in
   system `lib/` directories searched are searched before `lib/` itself.
   If there was a `!` suffix, *only* those subdirectories are searched.
   For example, sanitizer runtimes use `asan` because that instrumentation
   is compatible with uninstrumented library code, but `dfsan!` because
   that instrumentation requires that all code in the process be
   instrumented.

A version of the standard runtime instrumented with
LLVM [AddressSanitizer](https://clang.llvm.org/docs/AddressSanitizer.html)
is identified by the `PT_INTERP` string `asan/ld.so.1`. This version sends
the *load configuration* string `asan` before loading shared libraries.
When [SanitizerCoverage](https://clang.llvm.org/docs/SanitizerCoverage.html)
is enabled, it publishes a VMO to the *data sink* name `sancov` and uses a
VMO name including the process KOID.

[system calls]: /docs/reference/syscalls/README.md
[`zx_channel_call()`]: /docs/reference/syscalls/channel_call.md
[`zx_process_start()`]: /docs/reference/syscalls/process_start.md
[`zx_thread_create()`]: /docs/reference/syscalls/thread_create.md
