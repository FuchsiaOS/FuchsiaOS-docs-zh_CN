<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0143" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This document proposes changes to the kernel ABI to support tagged userspace
pointers.

## Motivation

[Top-Byte-Ignore (TBI)][tbi] is a feature on all ARMv8.0 CPUs that causes the
top byte of virtual addresses to be ignored on loads and stores. Instead, bit
55 is extended over bits 56-63 before address translation. This feature allows
use of the (ignored) top byte as a tag or for other in-band
metadata. One of the immediate uses of TBI is enabling [Hardware-assisted
AddressSanitizer (HWASan)][hwasan] in userspace, where tags are stored in the
top byte for memory tracking.

This document describes how the kernel should handle tagged user pointers.

While TBI and HWASan are the most relevant use cases of tagged pointers, this
design is not meant to solely cover them. There are other platforms with their
own hardware features similar that support tagged pointers, and other userspace
programs that can use these tag bits for their own specific use cases. This
design should be generic enough to support other implementations of tagged
pointers without any specific focus on one user application.

## Terminology

These are some terms that will be used frequently in this proposal. There are
_addresses_ and there are _pointers_. These are similar concepts but treated
very differently. Semantically, some syscalls operate on addresses while others
operate on pointers.

### Address

An address is a 64-bit integer that represents a location within
the bounds of a user address space. An address is never tagged. Syscalls that
manipulate an address space operate on addresses. The value of an address is
always constrained within the range of an address space (indicated by
`ZX_INFO_VMAR`).

### Pointer

A pointer is a programming language-specific concept that generally
indicates a location of dereferenceable memory. Each language defines the
implementation of a pointer and its translation into hardware. For C/C++, in the
context of HWASan, a pointer is a 64-bit integer that consists of tag bits and
address bits. Pointers can either be tagged (indicating non-zero tag bits) or
untagged (indicating tag bits of zero). Syscalls that access user memory
generally operate on pointers.

### Tag

A tag refers to the upper bits of a pointer, generally used for
metadata. On ARM with TBI enabled, a tag is 8 bits wide consisting of bits
56-63.

### TBI (Top-*Bits*-Ignore)

Other prospective hardware features such as [ARM MTE][mte] or [Intel LAM][lam]
also support a way of "ignoring" some portion of the upper bits of a pointer.
Unless specified, the term "TBI" used in this doc represents any generic hardware
feature that supports ignoring tags rather than exclusively ARM TBI.

## Design {#design}

Kernel code should replicate the behavior of the hardware. Tags
should be handled such that kernel behavior makes sense to users.

These are some examples of how the system should behave when TBI is enabled:

1. `zx_channel_write` can accept tagged pointers, and the call will behave the
   same as if the pointers were untagged.

2. When a process takes a page fault on a tagged user pointer, the page fault
   will be resolved as if the fault occurred on the same untagged pointer, with
   one exception. If the fault generates a Zircon exception, the exception
   report's fault pointer will contain the original tagged pointer to the degree
   that the hardware preserves it.

3. For the purpose of a futex wake/wait resolution, any tag on the supplied
   pointers will be ignored. In other words, waking a pointer that only differs
   by the tag will still wake any waiters waiting on that pointer, regardless of
   any tag they may have specified.

4. When reading memory from a process (like with `zx_process_read_memory`), the
   kernel will accept an **address** as the argument for the location of the block
   of memory being read. In conjunction to [software debugging](#debugging),
   debuggers will need to explicitly translate debuggee pointer values to
   addresses to read via kernel APIs.

### Tagged Pointer ABI: Tag-Insensitive, but Tag-Preserving {#taggedptrabi}

The following will hold when TBI is enabled:

1. __The kernel will ignore tags on user pointers received from syscalls.__ For
   example, a `zx_channel_read` call with a buffer pointer containing a tag
   would behave exactly the same as if the buffer pointer were untagged.

2. __It is an error to pass a tagged pointer on syscalls that accept
   addresses (ie. `zx_vaddr_t`).__ For example, a virtual address passed to
   `zx_process_read_memory` cannot be tagged. Using a tagged pointer where an
   address is required will be treated like any other invalid address.

3. __When the kernel accepts a tagged pointer, whether through syscall or
   fault, it will try to preserve the tag to the degree that user code may
   later observe it.__
   For example, if a user program faults on a tagged user pointer, then
   the resulting Zircon exception report will include the tag if the hardware
   can preserve it. The tag will be stripped if the hardware does not guarantee
   the tag can be preserved. If there is no mechanism by which userspace may
   observe the tag, the kernel is free to strip it provided it does not alter
   system behavior. If the hardware only guarantees partial preservation of a
   tag, then the kernel may only strip bits not guaranteed to be preserved.

4. __The kernel itself will never generate tagged pointers.__ For example, when
   mapping a VMO (via `zx_vmar_map`), the resulting value selected by the kernel
   will be a pure address with no tag.

5. __When comparing userspace pointers, the kernel will ignore any tags that may
   be present.__ For example if a thread is waiting (via `zx_futex_wait`) on a
   pointer with tag A, and another thread is waking (via `zx_futex_wake`) on a
   pointer with the same address bits but tag B, then the waiter will be woken.

### ARM64 TBI enabled for Everything

TBI will be controlled by a kernel boot-option. When enabled, TBI will be on
for all userspace processes.

### Debugging Software {#debugging}

Debuggers will need to be TBI-aware. ARM TBI does not allow setting a tag on
debug registers. Debuggers will need to explicitly sign-extend the most
significant VA bit before setting debug registers.

## Implementation

A boot-option will control whether user address spaces have ARM TBI enabled.
ARM TBI can be enabled by setting the `TBI0` and `TBI1` bits in the
translation control register (EL1).

In addition to enabling/disabling TBI, we'll need to make sure existing
syscalls correctly handle pointers/addresses. There are only a few places where
the kernel handles user pointers (e.g. `user_ptr`) so the changes required to
implement this proposal are relatively small.

We can indicate to userspace the type of TBI running through new system
[features][features]. We can introduce a new feature kind
`ZX_FEATURE_KIND_ADDRESS_TAGGING` and this kind can support new feature bits
indicating the address tagging, like `ZX_ARM64_FEATURE_ADDRESS_TAGGING_TBI` for
ARM TBI.

## Performance

Performance impact should be negligible and existing microbenchmarks will be
used to verify.

## Testing

We will need to test for:

1. Checking syscalls that use pointers with different tags, and those tags are
   effectively ignored.

2. Waking on a tagged vs untagged pointer (tag-insensitive behavior).

3. Faulting on a tagged pointer preserves the tag in the exception
   (tag-preserving behavior).

4. Any behavior to make kernel TBI known to userspace, such as the presence
   of a system feature or a query that returns the number of top bits
   ignored.

5. Verifying tagged pointers are rejected by syscalls that expect addresses.

These tests need to be skipped if TBI is not supported.

## Documentation

All documentation for the Tagged Pointer ABI is documented in this RFC. Once
this has been implemented, we may need to update some Zircon documentation to
specify which arguments for which syscalls cannot accept tags.

Syscalls that guarantee some degree of tag preservation will need to be
documented to specify which bits are preserved and which can be stripped.

## Drawbacks, alternatives, and unknowns

### TBI Toggle Granularity

We have two levels at which we can control the scope of TBI: globally and
per-process. A per-process approach would involve some mechanism that allows
toggling TBI at either process creation time or start time. This would require
either a new syscall, argument, or bitflag that would need more testing and
potentially introduce new bugs or security issues that will take time to
discover. Having a process toggle could be [expensive][contextswitch].

A global switch is less complex, and helps avoid many of the "unknown unknowns"
with having to implement a runtime switch. It will also likely be safer if all
applications for the system were either TBI-aware or non-TBI-aware rather than
having a mixture of both.

### Stripping Tags in Usermode

This would involve stripping all tags in the syscall layer before they made it
into the kernel. This way, no kernel changes would be needed, and the kernel
could remain agnostic to tags. One issue with this involves fault handling on
userspace pointers. If a fault is generated on a tagged pointer, then it will
be up to each userspace handler to strip the tag.

### Support for Other Addressing Modes

This proposal should be flexible enough to account for other hardware features
that involve "ignoring" top bits. We don't plan to support these in the near
future, but we should be in a state where turning one on would require minimal
changes.

### ARM Memory Tagging Extension (MTE)

[MTE][mte] is a feature that works on top of TBI for finding bad memory
accesses. [Memory tagging][memtag] works by associating each allocation and
pointer with a specific tag value. A pointer with a tag different from the
allocation it's trying to access indicates a bad memory access because of a
tag mismatch. With MTE, this tag is a 4-bit value stored in the top byte of the
pointer.

Under this ABI, should MTE be enabled, the tag would refer to the top 8-bits
of a pointer, but only bits 56-59 would be preserved of faults since the
hardware only guarantees preservation of those 4 bits.

### Intel Linear Address Masking (LAM)

[LAM][lam] is an upcoming feature for x86 where either the top 7 or 16 bits in a
pointer are masked out on loads/stores. This is controlled globally by altering
the CR3 register. Unlike TBI, LAM will not preserve any of the tag bits on a
page fault.

## Prior art and references

### Tagged virtual addresses in AArch64 Linux

Much of the design for this proposal was inspired from the [Tagged Address ABI
from Linux][linuxtbi],
namely most kernel behavior should remain unaffected when accepting tagged
pointers. One major differences is that Linux supports toggling the ABI
per-thread whereas this proposal aims to toggle the ABI globally at build/boot
time. Additionally, ARM TBI is enabled all the time on Linux whereas ARM TBI is
also controlled by the same build option.

[tbi]: https://developer.arm.com/documentation/den0024/a/ch12s05s01
[hwasan]: https://clang.llvm.org/docs/HardwareAssistedAddressSanitizerDesign.html
[mte]: https://community.arm.com/developer/ip-products/processors/b/processors-ip-blog/posts/enhancing-memory-safety
[lam]: https://www.phoronix.com/scan.php?page=news_item&px=Intel-LAM-Glibc
[linuxtbi]: https://www.kernel.org/doc/html/latest/arm64/tagged-address-abi.html
[contextswitch]: https://lore.kernel.org/linux-arm-kernel/20201124184742.GC42276@C02TF0J2HF1T.local/
[memtag]: https://arxiv.org/pdf/1802.09517.pdf
[features]: /docs/reference/syscalls/system_get_features.md
