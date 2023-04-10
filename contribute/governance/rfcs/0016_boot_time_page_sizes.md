{% set rfcid = "RFC-0016" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

Fuchsia based systems should be able to take advantage of larger page sizes when
desired for optimal performance. To make this feasible page size needs to be a
run time, and not compile time, constant. This constant should be determined
somehow by the kernel during boot, and then provided for run time querying of
the user via the VDSO.

## Motivation

To perform optimally the system should be able to select a page size based on
either information known statically, or queried at boot. Additionally, the
static decision should be changeable as needs or requirements change, ideally
in a non ABI breaking way.

Different page sizes have different performance trade offs. Larger pages can
reduce CPU overheads by increasing effective TLB coverage, and proportionally
improving the performance of any algorithm or operation that operates at page
granularity, such as page allocations, faults and scanning. Where large pages
can reduce memory utilization of page tables, they also waste memory by causing
overallocation to happen, and as such smaller pages can provide more optimal
memory usage.

This performance versus memory usage trade off can vary depending on the
hardware and system workload. Informing the user at boot time of the page size
allows for changing the page size statically at kernel compile time, or
dynamically at kernel boot, without breaking binary compatibility with user
level components.

## Design

The approach is to add an additional constant to the VDSO, along with a VDSO
call (`zx_system_get_page_size`) to retrieve it. Any usages of existing compile
time constants can then be migrated to use the VDSO call, until the compile time
constants can be removed.

Min and max page sizes should also be declared for each platform. This is to
allow users to know the max page size to link against so that they can ensure
their components are portable.

## Implementation

There are three phases to the implementation. Although the C/C++ names are
being used, equivalents need to be done across all Fuchsia supported languages.

 1. Add the `zx_system_get_page_size` VDSO call and associated VDSO constant as
    well as `PAGE_MIN_SIZE` and `PAGE_MAX_SIZE` definitions.
 2. Migrate usages of `PAGE_SIZE` (or language equivalent) to use VDSO call
 3. Remove `PAGE_SIZE` (or language equivalent) definitions once unused.

The first and third stages are trivial and would be small single CLs.

The migration stage should be uncomplicated, but should be done as many CLs
scoped by component.

Although not strictly part of this RFC, to actually vary the page size for a
given product the following also needs to be done:

 1. Low level kernel implementation support for larger pages.
 2. User components, such as BlobFS, would require modifications to support non
    4KiB pages.
 3. Alignment of ELF sections needs to be increased so that pages do not require
    overlapping security permissions.

## Performance

Although this migrates a compile time constant into a run-time query it is not
expected to have any measurable performance implications as page size
calculations are not known to be on any hot paths. Nevertheless When performing
the migration to the VDSO call, any usages found to not be in initialization
or testing code should be noted and the performance of the effected components
evaluated.

## Security considerations

None

## Privacy considerations

None

## Testing

Existing tests should be sufficient to catch any silly mistakes that might
happen during migration. Code coverage of tests should be checked when migrating
any code in a component.

## Documentation

The `zx_system_get_page_size` VDSO call needs to be documented. The
documentation should say that

 * This is the smallest page size and the base unit of all allocations.
 * The vdsocall can never fail.
 * Page size is guaranteed to be a power of 2.
 * Page size, once read, is a constant and cacheable by the user.

Existing documentation on VMOs and other memory related syscalls and objects is
otherwise already abstract and always refers to the "system page size".

Platform documentation should have minimum and maximum page sizes documented and
reflect the `PAGE_MIN_SIZE` and `PAGE_MAX_SIZE` constants. These values are

 * ARM aarch64: 4KiB minimum and 64KiB maximum.
 * x86-64: 4KiB minimum and 2MiB maximum.

## Drawbacks, alternatives, and unknowns

The system page size largely has relevance for users to correctly perform VMO
operations, or implement protocols with other Fuchsia services. As such it is
unclear when non-fuchsia native code would need to know, or have a dependency
on, the page size, but if this situation arises it may require source
modifications in order to port.

Performing the migration from a compile time constant although not conceptually
complicated, will result in non-trivial code churn and there is ample
opportunity to introduce bugs in the process.

Removing references to the compile time constants does not however imply that
code is actually able to tolerate different page sizes. There is plenty of
opportunity for algorithms to have baked in assumptions on the current 4KiB
page size, or to have simply defined their own page size constant. These would
also be issues if the compile time constant was changed and so should be
considered unrelated bugs.

The primary alternative is to continue using a compile time constant, but either
fix it for a given product, or fix some combinations of it for a given product.
Fixing for a given product may work for some tightly controlled products, but is
less suitable for long running products that desire binary compatibility over a
long time frame across different hardware iterations. Requiring multiple
versions of a binary to be built with different page sizes provides the desired
flexibility, but at great cost to developer time and storage. In general,
sticking to a compile time constant has many downsides, with the only
perceivable upside being avoiding a one of migration.

Instead of a boot time constant, page sizes could be truly variable and
potentially change over time, or be different for different components. Although
this provides ultimate flexibility, given that objects, such as VMOs, that have
semantics linked to the page size can be shared arbitrarily between components,
attempting to have different page sizes would create an unreasonable burden on
the user to both query and avoid race conditions of the page size changing. For
times when a page size different the system page size would be beneficial to a
particular sub system some separate mechanism to explicitly opt in VMOs, or
otherwise optimize page size, should be developed.

It is also useful for applications to have know the size of the page in bits, to
perform shift arithmetic. For this a `zx_system_get_page_shift` could be added
as well as, or instead of, `zx_system_get_page_size`. Given that using the shift
is a micro-optimization it is probably only beneficial if the result of the
vdsocall is cached by the application. Given this, it becomes equivalent for
the user to convert the page size into a shift and cache that. Therefore there
is no actual benefit in providing both variants as a vdsocall.

## Prior art and references

Unix derivatives report page size via [`sysconf(_SC_PAGE_SIZE)`].

[`sysconf(_SC_PAGE_SIZE)`]: https://man7.org/linux/man-pages/man3/sysconf.3.html

A `PAGE_SIZE` compile time constant is provided as a constant inside kernel code
and by some distributions as part of `<sys/user.h>`, but it is not standard or
portable.

Windows reports page size through the [`GetSystemInfo()`] syscall.

[`GetSystemInfo()`]: https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsysteminfo?redirectedfrom=MSDN

MacOS reports page size via [`sysctl()`] call or the [`vm_page_size`] variable.

[`sysctl()`]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/sysctl.3.html
[`vm_page_size`]: https://developer.apple.com/documentation/apple_silicon/addressing_architectural_differences_in_your_macos_code
