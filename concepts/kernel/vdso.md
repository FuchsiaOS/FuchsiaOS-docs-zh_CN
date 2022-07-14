# Zircon vDSO

The Zircon vDSO is the sole means of access to [system
calls](/reference/syscalls/README.md)
in Zircon. vDSO stands for *virtual Dynamic Shared Object*. (*Dynamic
Shared Object* is a term used for a shared library in the ELF format.)
It's *virtual* because it's not loaded from an ELF file that sits in a
filesystem. Instead, the vDSO image is provided directly by the kernel.

[TOC]

## Using the vDSO

### System Call ABI

The vDSO is a shared library in the ELF format. It's used in the normal
way that ELF shared libraries are used, which is to look up entry points by
symbol name in the ELF *dynamic symbol table* (the `.dynsym` section,
located via `DT_SYMTAB`). ELF defines a hash table format to optimize
lookup by name in the symbol table (the `.hash` section, located via
`DT_HASH`); GNU tools have defined an improved hash table format that makes
lookups much more efficient (the `.gnu_hash` section, located via
`DT_GNU_HASH`). Fuchsia ELF shared libraries, including the vDSO, use the
`DT_GNU_HASH` format exclusively. (It's also possible to use the symbol
table directly via linear search, ignoring the hash table.)

The vDSO uses a [simplified layout](#read-only-dynamic-shared-object-layout)
that has no writable segment and requires no dynamic relocations. This
makes it easier to use the system call ABI without implementing a
general-purpose ELF loader and full ELF dynamic linking semantics.

ELF symbol names are the same as C identifiers with external linkage.
Each [system call](/reference/syscalls/README.md) corresponds to an ELF symbol in the vDSO,
and has the ABI of a C function. The vDSO functions use only the basic
machine-specific C calling conventions governing the use of machine
registers and the stack, which is common across many systems that use ELF,
such as Linux and all the BSD variants. They do not rely on complex
features such as ELF Thread-Local Storage, nor on Fuchsia-specific ABI
elements such as the [SafeStack](/concepts/kernel/safestack.md) unsafe stack pointer.
To see more information about the life of a syscall and its relationship to the vDSO, see
[Life of a Fuchsia syscall](/concepts/kernel/life_of_a_syscall.md).

### vDSO Unwind Information

The vDSO has an ELF program header of type `PT_GNU_EH_FRAME`. This points
to unwind information in the GNU `.eh_frame` format, which is a close
relative of the standard DWARF Call Frame Information format. This
information makes it possible to recover the register values from call
frames in the vDSO code, so that a complete stack trace can be reconstructed
from any thread's register state with a PC value inside the vDSO code.
These formats and their use are just the same in the vDSO as they are in any
normal ELF shared library on Fuchsia or other systems using common GNU ELF
extensions, such as Linux and all the BSD variants.

### vDSO Build ID

The vDSO has an ELF *Build ID*, as other ELF shared libraries and
executables built with common GNU extensions do. The Build ID is a unique
bit string that identifies a specific build of that binary. This is stored
in ELF note format, pointed to by an ELF program header of type `PT_NOTE`.
The payload of the note with name `"GNU"` and type `NT_GNU_BUILD_ID` is a
sequence of bytes that constitutes the Build ID.

One main use of Build IDs is to associate binaries with their debugging
information and the source code they were built from. The vDSO binary is
innately tied to (and embedded within) the kernel binary and includes
information specific to each kernel build, so the Build ID of the vDSO
distinguishes kernels as well.

### `zx_process_start()` argument

The [`zx_process_start()`] system call is how a
program loader tells the kernel to start a new process's first thread
executing. The final argument (`arg2`
in the [`zx_process_start()`] documentation) is a
plain `uintptr_t` value passed to the new thread in a register.

By convention, the program loader maps the vDSO into each new process's
address space (at a random location chosen by the system) and passes the
base address of the image to the new process's first thread in the `arg2`
register. This address is where the ELF file header can be found in memory,
pointing to all the other ELF format elements necessary to look up symbol
names and thus make system calls.

### **PA_VMO_VDSO** handle

The vDSO image is embedded in the kernel at compile time. The kernel
exposes it to userspace as a read-only [VMO](/reference/kernel_objects/vm_object.md).

When a program loader sets up a new process, the only way to make it
possible for that process to make system calls is for the program loader to
map the vDSO into the new process's address space before its first thread
starts running. Hence, each process that will launch other processes
capable of making system calls must have access to the vDSO VMO.

By convention, a VMO handle for the vDSO is passed from process to process
in the `zx_proc_args_t` bootstrap message sent to each new process
(see [`<zircon/processargs.h>`](/zircon/system/public/zircon/processargs.h)).
The VMO handle's entry in the handle table is identified by the *handle
info entry* `PA_HND(PA_VMO_VDSO, 0)`.

## vDSO Implementation Details

### **kazoo** tool

The [`kazoo` tool](/zircon/tools/kazoo/) generates both C/C++ function
declarations that form the public [system
call](/reference/syscalls/README.md) API, and some C++ and assembly code
used in the implementation of the vDSO. Both the public API and the private
interface between the kernel and the vDSO code are specified by the .fidl files
in [//zircon/vdso](/zircon/vdso).

The syscalls fall into the following groups, distinguished by the
presence of attributes after the system call name:

* Entries with neither `vdsocall` nor `internal` are the simple cases
  (which are the majority of the system calls) where the public API and
  the private API are exactly the same. These are implemented entirely
  by generated code. The public API functions have names prefixed by
  `_zx_` and `zx_` (aliases).

* `vdsocall` entries are simply declarations for the public API. These
  functions are implemented by normal, hand-written C++ code found in
  [the kernel source](/zircon/kernel/lib/userabi/vdso/). Those source
  files `#include "private.h"` and then define the C++ function for the
  system call with its name prefixed by `_zx_`. Finally, they use the
  `VDSO_INTERFACE_FUNCTION` macro on the system call's name prefixed by
  `zx_` (no leading underscore). This implementation code can call the
  C++ function for any other system call entry (whether a public
  generated call, a public hand-written `vdsocall`, or an `internal`
  generated call), but must use its private entry point alias, which has
  the `VDSO_zx_` prefix. Otherwise the code is normal (minimal) C++, but
  must be stateless and reentrant (use only its stack and registers).

* `internal` entries are declarations of a private API used only by the
  vDSO implementation code to enter the kernel (i.e., by other functions
  implementing `vdsocall` system calls). These produce functions in the
  vDSO implementation with the same C signature that would be declared
  in the public API given the signature of the system call entry.
  However, instead of being named with the `_zx_` and `zx_` prefixes,
  these are only available through `#include "private.h"` with
  `VDSO_zx_` prefixes.

### Read-Only Dynamic Shared Object Layout {#read-only-dynamic-shared-object-layout}

The vDSO is a normal ELF shared library and can be treated like any
other. But it's intentionally kept to a small subset of what an ELF
shared library in general is allowed to do. This has several benefits:

 * Mapping the ELF image into a process is straightforward and does not
   involve any complex corner cases of general support for ELF `PT_LOAD`
   program headers. The vDSO's layout can be handled by special-case
   code with no loops that reads only a few values from ELF headers.
 * Using the vDSO does not require full-featured ELF dynamic linking.
   In particular, the vDSO has no dynamic relocations. Mapping in the
   ELF `PT_LOAD` segments is the only setup that needs to be done.
 * The vDSO code is stateless and reentrant. It refers only to the
   registers and stack with which it's called. This makes it usable in
   a wide variety of contexts with minimal constraints on how user code
   organizes itself, which is appropriate for the mandatory ABI of an
   operating system. It also makes the code easier to reason about and
   audit for robustness and security.

The layout is simply two consecutive segments, each containing aligned
whole pages:

 1. The first segment is read-only, and includes the ELF headers and
    metadata for dynamic linking along with constant data private to the
    vDSO's implementation.
 2. The second segment is executable, containing the vDSO code.

The whole vDSO image consists of just these two segments' pages, present
in the ELF image just as they should appear in memory. To map in the
vDSO requires only two values gleaned from the vDSO's ELF headers: the
number of pages in each segment.

### Boot-time Read-Only Data

Some system calls simply return values that are constant throughout the
runtime of the whole system, though the ABI of the system is that their
values must be queried at runtime and cannot be compiled into user code.
These values either are fixed in the kernel at compile time or are
determined by the kernel at boot time from hardware or boot parameters.
Examples include [`zx_system_get_version_string()`],
[`zx_system_get_num_cpus()`], and [`zx_ticks_per_second()`].

Because these values are constant, there is no need to pay the overhead
of entering the kernel to read them. Instead, the vDSO implementations
of these are simple C++ functions that just return constants read from
the vDSO's read-only data segment. Values fixed at compile time (such as
the system version string) are simply compiled into the vDSO.

For the values determined at boot time, the kernel must modify the
contents of the vDSO. This is accomplished by the boot-time code that
sets up the vDSO VMO, before it starts the first userspace process and
gives it the VMO handle. At compile time, the offset into the vDSO image
of the
[`vdso_constants`](/zircon/kernel/lib/userabi/include/lib/userabi/vdso-constants.h)
data structure is extracted from the vDSO ELF file that will be embedded
in the kernel. At boot time, the kernel temporarily maps the pages of
the VMO covering `vdso_constants` into its own address space long enough
to initialize the structure with the right values for the current run of
the system.

### Enforcement

The vDSO entry points are the only means to enter the kernel for system
calls. The machine-specific instructions used to enter the kernel (e.g.
`syscall` on x86) are not part of the system ABI and it's invalid for
user code to execute such instructions directly. The interface between
the kernel and the vDSO code is a private implementation detail.

Because the vDSO is itself normal code that executes in userspace, the
kernel must robustly handle all possible entries into kernel mode from
userspace. However, potential kernel bugs can be mitigated somewhat by
enforcing that each kernel entry be made only from the proper vDSO code.
This enforcement also avoids developers of userspace code circumventing
the ABI rules (because of ignorance, malice, or misguided intent to work
around some perceived limitation of the official ABI), which could lead
to the private kernel-vDSO interface becoming a *de facto* ABI for
application code.

The kernel enforces correct use of the vDSO in two ways:

 1. It constrains how the vDSO VMO can be mapped into a process.

    When a [`zx_vmar_map()`] call is made using the vDSO VMO and
    requesting `ZX_VM_PERM_EXECUTE`, the kernel requires that the offset
    and size of the mapping exactly match the vDSO's executable segment.
    It also allows only one such mapping. Once the valid vDSO mapping
    has been established in a process, it cannot be removed. Attempts to
    map the vDSO a second time into the same process, to unmap the vDSO
    code from a process, or to make an executable mapping of the vDSO
    that don't use the correct offset and size, fail with
    `ZX_ERR_ACCESS_DENIED`.

    At compile time, the offset and size of the vDSO's code segment are
    extracted from the vDSO ELF file and used as constants in the
    kernel's mapping enforcement code.

    When the one valid vDSO mapping is established in a process, the
    kernel records the address for that process so it can be checked
    quickly.

 2. It constrains what PC locations can be used to enter the kernel.

    When a user thread enters the kernel for a system call, a register
    indicates which low-level system call is being invoked. The
    low-level system calls are the private interface between the kernel
    and the vDSO; many correspond directly the system calls in the
    public ABI, but others do not.

    For each low-level system call, there is a fixed set of PC locations
    in the vDSO code that invoke that call. The source code for the vDSO
    defines internal symbols identifying each such location. At compile
    time, these locations are extracted from the vDSO's symbol table and
    used to generate kernel code that defines a PC validity predicate
    for each low-level system call. Since there is only one definition
    of the vDSO code used by all user processes in the system, these
    predicates simply check for known, valid, constant offsets from the
    beginning of the vDSO code segment.

    On entry to the kernel for a system call, the kernel examines the PC
    location of the `syscall` instruction on x86 (or equivalent
    instruction on other machines). It subtracts the base address of the
    vDSO code recorded for the process at [`zx_vmar_map()`] time from
    the PC, and passes the resulting offset to the validity predicate
    for the system call being invoked. If the predicate rules the PC
    invalid, the calling thread is not allowed to proceed with the
    system call and instead takes a synthetic exception similar to the
    machine exception that would result from invoking an undefined or
    privileged machine instruction.

### Variants

**TODO(mcgrathr)**: vDSO *variants* are an experimental feature that is
not yet in real use. There is a proof-of-concept implementation and
simple tests, but more work is required to implement the concept
robustly and determine what variants will be made available. The concept
is to provide variants of the vDSO image that export only a subset of
the full vDSO system call interface. For example, system calls intended
only for use by device drivers might be elided from the vDSO variant
used for normal application code.

[`zx_process_start()`]: /reference/syscalls/process_start.md
[`zx_system_get_num_cpus()`]: /reference/syscalls/system_get_num_cpus.md
[`zx_system_get_version_string()`]: /reference/syscalls/system_get_version_string.md
[`zx_ticks_per_second()`]: /reference/syscalls/ticks_per_second.md
[`zx_vmar_map()`]: /reference/syscalls/vmar_map.md
