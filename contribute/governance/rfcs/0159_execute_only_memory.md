<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0159" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This document proposes changes to kernel APIs to support binaries with
execute-only segments, by adding a new feature check in
`zx_system_get_features` and changing the `launchpad` and `process_builder`
loaders as well as the dynamic linker in Fuchsia's in-tree libc to support '--x'
segments. It lays out a plan for eventual kernel support for mapping
execute-only pages on hardware that supports it.

We don't typically need to read executable memory after it has been loaded.
Enabling execute-only code by default increases security of Fuchsia’s userspace
processes and furthers the engineering best practice of least permissions.

## Motivation

Support for execute-only pages was added to ARM MMUs in ARMv7m and allows pages
of memory to be mapped such that they are only executable and not readable or
writable. Though writable code pages have been considered a security threat for
a long time, allowing code to remain readable has been shown to expose
applications to needless risk. Specifically, reading code pages is often a first
step in an attack chain, and preventing code from being read hinders
adversaries. See [Readable Code Security](#readable-code-security). Moreover,
supporting execute-only pages fits well with Fuchsia’s permissions model and
more strongly aligns with the principle of least privilege: often code doesn’t
need to be read, but just be executed.

## Stakeholders

_Facilitator:_

- cpu@google.com

_Reviewers:_

- phosek@google.com
- mvanotti@google.com
- maniscalco@google.com
- travisg@google.com

## Background

### Execute-only Memory

Execute-only memory (XOM) describes memory pages that have neither read nor
write permissions and can only be executed. ARMv7m and above have native support
for XOM, however there are some considerations on older ISA’s. Discussed further
in [XOM and PAN](#xom-and-pan).

This doc focuses almost exclusively on AArch64, however the implementation is
architecture agnostic. When hardware and toolchain support matures for other
architectures, they would all easily be able to take advantage of execute-only
support in Fuchsia.

### Permissions of Code Pages

Initially, computers supported direct memory access to physical memory without
any checks or protections. The introduction of MMUs provided a key abstraction,
in the form of virtual memory, by decoupling a program's view of memory from the
underlying physical resources. This facilitated a more flexible, safe, and
secure programming model by allowing OS implementers to provide strong isolation
between their programs via the process abstraction. Today's MMUs provide a
number of critical facilities, such as paged memory, fast address translation,
and permission checking. They also allow users significant control over how
memory regions can be accessed and used, via the page permissions that typically
control if memory pages can be read, written to, or executed. This is a key
property for program safety, fault isolation, and security, since it restricts a
program's ability to misuse system resources through hardware enforced
permission checks.

Memory that is both writable and executable is particularly dangerous because it
provides an easy way for an adversary to achieve arbitrary code execution
through common vulnerabilities, like buffer overflows. For this reason, many OS
configurations explicitly disallow pages to be both writable and executable
(W^X). This has been the standard for over a decade, OpenBSD added support for
W^X in 2003 with OpenBSD 3.3 [openbsd-wxorx]. See also SELinux W^X policies
[selinux-wxorx].  Writable code can be useful for things like just-in-time (JIT)
compilation, which writes executable instructions to memory at runtime. Having
W|X pages can be disallowed and JIT’s need to work around this. An easy way is
to write code to non-executable pages and later change the page protections,
i.e., through `mprotect` or `zx_vmar_protect`, to be executable but not writable
[example-fuchsia-test]. In nearly all cases pages that are W|X are too
permissive. Similarly, executable pages rarely ever need to be read [See
exceptions](#readable-code). Allowing read operations on executable pages is
generally unnecessary and should not be the default.

### Readable Code

Because of ARM’s fixed instruction width, immediate values have size
constraints. For this reason loads are done using PC-Relative addressing. To get
around this, the pseudo instruction `ldr Rd, =imm` will emit `imm` in literal
pools close to the code loading it. This is incompatible with XOM because it
puts data in the text section which must be readable. When searching for use of
literal pools in the codebase to ensure we don’t read executable segments, we
have found some usages of `ldr Rd, =imm` in Zircon, but all has since been
removed. Clang will not use literal pools for aarch64, instead it will emit
multiple instructions to create a large immediate. Clang has a `-mexecute-only`
flag and alias `-mpure-code` but these are only meaningful on arm32 because
these flags are inherent when targeting aarch64.

#### Example: Large Intermediates

This example shows how Clang compiles this C code to assembly given different
targets [clang-example]. The top row shows aarch64, and the bottom shows arm32:

```
uint32_t a() {
    return 0x12345678u;
}
```
```
# -target aarch64
a:
    mov w0, #22136
    movk w0, #4660, lsl #16
    ret
```
```
# -target arm
a:
    ldr r0, .LCPI0_0
    bx lr
.LCPI0_0:
    .long 305419896
```

### XOM and PAN

Privileged access never (PAN) is a security feature on ARM chips that prevents
normal memory access to user pages from kernel mode. It helps protect against
potential kernel vulnerabilities because the kernel cannot touch user memory
with a normal load or store instructions. Instead the OS would need to turn PAN
off or use the `ldtr` and `sttr` instructions for accessing those pages. PAN is
not currently enabled for Fuchsia, but there are already plans to support it in
zircon [pan-fxb].

Aarch64 page table entries have 4 relevant bits to control page permissions. 2
bits are used for user and privileged execute-never. The remaining two are used
to describe read and write page permissions for both access levels. An
execute-only mapping has both read and write access removed but allows user
execution.

This table from the ARMv8 Reference Manual shows the possible memory protections
using the only 4 available bits. EL0 is the exception level for userspace. Rows
0 and 2 show how to create userspace execute-only pages. See Table D5-34 Stage 1
from the ARMv8 Reference Manual.

| UXN | PXN | AP[2:1] | Access from a higher Exception level | Access from EL0  |
|-----|-----|---------|--------------------------------------|------------------|
| 0   | 1   | 00      | R, W                                 | X                |
| 0   | 1   | 01      | R, W                                 | R, W, X          |
| 0   | 1   | 10      | R                                    | X                |
| 0   | 1   | 11      | R                                    | R, X             |

Unfortunately, PAN’s algorithm for deciding if a page should not be privileged
accessible checks if the page is user-readable. From the perspective of PAN, a
user-execute-only page looks like a privileged mapping. This allows the kernel
to access user memory where it otherwise should not, thereby bypassing PAN’s
intended purpose and making PAN and XOM incompatible [pan-issue]. This would
make any future usage of PAN not useful against attacks trying to exploit the
kernel touching user memory, however it would still be useful for detecting
kernel bugs.

This problem caused both Linux and Android to drop support for XOM. This was
particularly noticeable for Android who dropped support indefinitely in Android
11 after being added and made the default for all aarch64 binaries in Android 10
[linux-revert][android-xom]. They plan to re-enable the feature as hardware
which fixes the problem becomes more ubiquitous but there is no concrete time
frame when it will be readded.

ARM has since proposed a solution with “enhanced” PAN or ePAN, which changes PAN
to check not just if a page is user readable but also not user executable.
Unfortunately, hardware with the feature may not be on any Fuchsia-targeted
devices for years. Linux has since re-added their implementation of XOM after
ePAN was made [linux-re-land]. Support for ePAN on devices is out of our control
and the incompatibility with PAN and XOM should not block the kernel’s
implementation of PAN [See more](#risks).

From figure 2, there is no possible configuration where read permission can be
stripped from the kernel. The only exception is PAN, which can cause an
exception when the kernel tries to touch a user-readable page. For this reason,
it is not possible to create an execute-only mapping for the kernel, since the
kernel cannot mark a page executable at EL1 but not readable. Thus, it is only
possible to create an execute-only mapping for userspace processes.

### Targeting XOM Hardware

Segment permissions in ELF indicate what permissions the code requires to run
correctly. In other words, software doesn’t need to know at build time if the
hardware it will run on can support XOM or not. Instead, it should
unconditionally use XOM if it will not need to read code pages. It is up to the
OS and loaders to enforce those permissions to the greatest extent the system
allows [elf-segment-perm].

### Virtual Memory Permissions

POSIX specifies that `mmap` may permit read access to pages where `PROT_READ`
has not been explicitly set [posix-mmap]. Both Linux and macOS on x86, and macOS
on M1 chips, will not fail when requesting pages from mmap with just `PROT_EXEC`
and instead make the pages `PROT_READ | PROT_EXEC`. These implementations have
syscalls which are “best-effort” in their ability to honor a user's requests.
Fuchsia syscalls, on the other hand, are always explicit in what they can and
cannot honor. The `zx_vmar_*` syscalls do not silently escalate permissions of
pages like their POSIX counterparts are permitted to by the standard. Requesting
pages without `ZX_VM_PERM_READ` will currently always fail as the hardware and
the OS do not support mapping pages without read permissions. A graceful
transition to supporting binaries with execute-only segments and userspace
programs which allocate execute-only memory will require a way to check if the
OS can map execute-only pages prior to requesting them.

### Readable Code Security

Many attacks rely on finding out information about the process through reading
code pages to find “gadgets”, or executable code of interest. Address space
layout randomization (ASLR) is a technique used by operating systems to load
binary segments at semi-random places in the process's address space. It is used
by Fuchsia and many other OS to hinder attacks which rely on knowing where code
or other data is in memory.  Making code unreadable further reduces the attack
surface.

Code reuse attacks, like “return-to-libc” [rtl-attack], are used to return
control of a function to a known address. libc is a logical choice to return or
jump into since it contains rich functionality useful to an attacker, and
because it is extremely likely the process will link against libc. It has been
demonstrated that the available gadgets in a typical program are
Turing-complete, giving an adversary the ability to execute arbitrary code.

In many cases an adversary's objective is to obtain a shell. ASLR makes these
kinds of attacks harder because the addresses of functions are different between
invocations of a program. However, ASLR isn’t a comprehensive mitigation,
because attackers can read code pages to find the address of functions that they
would otherwise not know by looking at their address in the binary. XOM makes it
impossible for ASLR to be broken in this way and attackers will need to use
another way to find out information about the location of specific code pages.

### Common Notation

#### ‘rwx/r-x/–x’

These represent permissions of ELF segments, which get mapped into the processes
address space with the corresponding permissions. This notation is used commonly
both when describing permissions of files, as well as ELF segments by tools like
`readelf`. r, w and x mean read, write and execute respectively and ‘-’ means
the permission is not granted. An execute-only segment will have ‘--x’
permissions.

#### R^X, W|X, etc…

As above, R, W and X refer to read, write and execute. ‘^’ and ‘|’ are C-like
operators for xor and or. R^X is read as “read xor execute”.

#### "ax"

This is assembler syntax which marks a section as allocated and executable.
Currently linkers will put “ax” sections into segments that are ‘r-x’. The
`--execute-only` flag in lld will mark these segments as ‘--x’ instead.

## Design

To increase security of our userspace programs by supporting XOM, both our
toolchain and loaders will need to be updated. The clang driver will need to
pass the ‘--execute-only’ flag to the linker to ensure “ax” sections which would
otherwise be mapped to ‘r-x’ segments are instead mapped to ‘--x’ segments. The
loaders will also need to change the sanity checks that all requested
permissions contain at least read, because this will no longer be true.

As it will only be possible to use XOM on hardware that has ePAN, we will need
to gracefully support the transition. We have two options:

1. Change `vmar_*` functions to be best effort like many `mmap` implementations
1. Create a way to query the kernel if it supports execute-only mappings and
have the loader escalate permissions of a ‘--x’ segment to ‘r-x’ if XOM is not
available.
1. Add a new `ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` flag for loaders to use with
‘--x’ segments.

In all cases, there will be a potential silent escalation of permissions. The
first option would be the easiest, the loaders would need no changes other than
removing their sanity checks. The second option is not significantly more
complex, it just would add a simple check in the loaders before deciding what
memory permissions to request from the OS. The third option is helpful because
it is less error prone in user code.

The first option would end up breaking Fuchsia’s current strict contract with
userspace of always being explicit about what a syscall can and cannot honor.
The 2nd and 3rd option also end up with ambiguous handling of memory permissions
when loading ELF files. However this fits within the ELF specification. Segment
permissions don’t specify 1:1 what permissions the memory allocated for a
segment will have, but rather which permissions the memory must at least have
for the program to operate correctly. ELF loaders are within their rights to map
a ‘--x’ segment into ‘r-x’ memory [elf-segment-perm].

The first option of breaking Fuchsia’s current contract of explicit syscall
handling isn’t ideal. Both option 2 and 3 have value and the implementation
proposed in this RFC will be based on both options.

## Implementation

### System Call Additions

A new flag `ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` will be added which will make
the various `zx_vmar_*` syscalls which take a permissions flag in `options`
which will implicitly add read permission if XOM is not supported.
`ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` is logically only useful with
`ZX_VM_PERM_EXEC` and not `ZX_VM_PERM_READ`, however the various syscall which
accept this flag will not be treating this as an invariant. It is safe to have
`ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` with any other combination of flags, it
will just be treated as `ZX_VM_PERM_READ` in contexts where the system
cannot map execute-only pages.

A new `kind` value `ZX_FEATURE_KIND_VM` will be added for
`zx_system_get_features`, which will yield a bitset similar to
`ZX_FEATURE_KIND_CPU`. There will also be a new feature
`ZX_VM_FEATURE_CAN_MAP_XOM`. The current implementation will always keep this
bit false because XOM will not be enabled until later. This will not be used by
the loaders because ‘r-x’ memory permissions are valid for a ‘--x’ segments, but
is still important for userspace to be able to query for this functionality.

### System Loader ABI Changes

Current and future loaders will ensure '--x' segments can be loaded into memory
even if the target can't support XOM. The loaders will add
`ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` when mapping execute-only segments.

### Shipped Dynamic Linker ABI Changes

Similarly, the dynamic linker in Fuchsia’s libc shipped with the SDK will also
escalate permissions where necessary when allocating memory for  ‘--x’ segments
with `ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED`.

### Compiler Toolchain Changes

The clang driver will also be changed to always pass `--execute-only` to the
linker when targeting `aarch64-*-fuchsia`. We will also need a way to opt out of
this behavior, most likely by adding a new ‘--no-execute-only’ flag to the
linker, so programs can easily opt out of the new default behavior.

### Kernel XOM Implementation

Once hardware arrives that supports ePAN, the kernel can service a request for
memory pages to have just `ZX_VM_PERM_EXECUTE`. The arm64 user-copy
implementation may need updates to ensure it's consistent with how user memory
access is constrained. `user_copy` should be updated to use the `ldtr` and
`sttr` instructions. This will ensure that users cannot trick the kernel to read
unreadable pages for them. Moreover, the kernel makes assumptions about mappings
being readable in a couple of places and these will need to be changed where
appropriate. This work will be done later.

### Unnecessary Changes

`zx_process_read_memory` does not need to be changed, and debuggers should work
normally when debugging execute-only binaries. `zx_process_read_memory` ignores
the permissions of the pages it is reading from, and only checks that the
process handle has `ZX_RIGHT_READ` and `ZX_RIGHT_WRITE`.

`zx_vmar_protect` will continue to work as it does currently. Most notably this
means that processes can protect their code pages with read permission in cases
where that is necessary.

## Performance

There is no expected impact in performance.

## Security

Until XOM is implemented in the kernel a binary with ‘--x’ segments will be just
as secure as an equivalent binary using ‘r-x’ segments. Once XOM is supported
both by hardware and the OS, programs which elect to use execute-only memory
will become more secure. See sections [Permissions of Code
Pages](#permissions-of-code-pages), [XOM and PAN](#xom-and-pan) and [Readable
Code Security](#readable-code-security).

## Privacy

No extra considerations other than those mentioned in [Security](#security).

## Testing

`zx_system_get_features` will have trivial testing when we are forcing XOM
support in the kernel where we can know at build time what we expect the
syscall to return.

The `ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` will be tested that it makes a page
readable when it is reported by `zx_system_get_features` that the OS cannot
create execute-only pages.

Likewise, the elfload library doesn't have any real testing, save for fuzz tests
which don't test expected functionality. Instead its functionality is inherently
tested by other components that rely on it. Testing should be added here to
ensure '--x' segments are correctly mapped. The process_builder library does
have tests, and these will ensure it properly requests readable and executable
memory when XOM is not available.

The changes to the current dynamic linker will not be tested directly. A new
dynamic linker is planned and it will have extensive testing, including testing
of ‘--x’ segments.

The changes to the clang driver will have testing in upstream LLVM.

We will also set up testing configuration for enabling XOM on test bots, even if
that hardware does not have ePAN and we would otherwise not enable XOM. This
will help us catch in tree programs that read their code pages and need to opt
out of execute-only.

## Documentation

The changes to `zx_system_get_features` will be documented, as well as the
motivation for why user space would want to query with the kind
`ZX_VM_FEATURE_CAN_MAP_XOM`. Likewise the new
`ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` flag will also be documentated. Changes to
the various loaders and the clang driver defaults will not be documented outside
of this RFC.

## Drawbacks, Alternatives, Unknowns

It is unknown how much current and future out of tree code relies on executable
code being readable. This could be from use of data constants in text from
handwritten assembly, code compiled from other toolchains or program
introspection. Regardless, programs which need to have readable code pages, will
still benefit because their shared library dependencies, including libc, will be
marked execute only. Changing our clang toolchain to default to execute-only
segments will break programs which depend on readable code. There is no easy way
to check at build time if a program relies on this behavior. However once it is
identified that a program needs ‘r-x’ segments, opting out of the default ‘--x’
will be simple.

For programs which need to be able to read some of their code but not all,
current tooling cannot easily support this. The `--execute-only linker` flag
will strip read permissions from any executable segment, and there is no way to
mark a single section as needed to be read. Programs which want this behavior
will need to opt out of execute-only completely.

## Risks

It is possible that the clang driver defaults to using `--execute-only` and code
that reads from a ‘--x’ segment won’t be broken until hardware and kernel
support for XOM lands. This creates potential forward compatibility problems for
software that didn’t change. Testing will exist for in tree software, but most
likely not for out of tree code.

## Prior Art and References

Because of the ambiguous handling of `mmap` permission flags in many POSIX
implementations, they have no need for an analogue to
`zx_system_get_features(ZX_FEATURE_KIND_CAN_MAP_XOM, &feature)`.

Darwin supports XOM on newer Apple chips, but their implementation is more
robust using proprietary hardware features. Their chips have hardware support
for stripping individual permission bits from both kernel and user memory. It is
not enabled for userspace in macOS. [apple-xom]

[example-fuchsia-test]: https://source.corp.google.com/fuchsia/zircon/system/utest/core/memory-mapping/memory-mapping.cc;l=126
[openbsd-wxorx]: https://www.openbsd.org/33.html
[selinux-wxorx]: https://akkadia.org/drepper/selinux-mem.html
[clang-example]: https://godbolt.org/z/hGzr49qYs
[android-xom]: https://source.android.com/devices/tech/debug/execute-only-memory
[elf-segment-perm]: https://www.sco.com/developers/gabi/latest/ch5.pheader.html
[posix-mmap]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/mmap.html
[rtl-attack]: https://dl.acm.org/doi/10.1145/1315245.1315313
[pan-fxb]: https://fxbug.dev/59284
[pan-issue]: https://blog.siguza.net/PAN/
[linux-revert]: https://github.com/torvalds/linux/commit/cab15ce604e550020bb7115b779013b91bcdbc21
[linux-re-land]: https://github.com/torvalds/linux/commit/18107f8a2df6bf1c6cac8d0713f757f866d5af51
[apple-xom]: https://i.blackhat.com/USA-19/Thursday/us-19-Krstic-Behind-The-Scenes-Of-IOS-And-Mas-Security.pdf
