{% set rfcid = "RFC-0073" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!--
*** This should begin with an H2 element (for example, ## Summary).
-->

## Summary

Proposes raising the minimum x86-64 platform from 'x86-64-v1' with extensions to
'x86-64-v2'.

Fuchsia currently runs on x86-64 CPUs with the baseline 64-bit ISA and the
CMPXCHG16B extension. This RFC proposes raising the minimum to the
x86-64-v2 ISA, adding the LAHF-SAHF, POPCNT, SSE3, SSE4.1, SSE4.2, and
SSSE3 extensions. Raising the minimum platform and allowing compilers
to generate code using those extensions raises system performance
noticeably while only marginally reducing the number of systems
Fuchsia runs on.

## Design

The Fuchsia kernel and system are currently compiled targeting a 'generic'
x86-64 CPU with the CMPXCHG16B extension.

This covers every Intel x86-64 CPU and every AMD64 CPU since K8 rev F (2006);
while broadly compatible, this does mean many very useful ISA extensions
cannot be assumed or used without runtime testing + dispatch.

Raising the baseline ISA to 'x86-64-v2' (defined in a recent update to the
ELF ABI) allows compilers to assume that LAHF/SAHF, POPCNT, SSE3, SSE4.1,
SSE4.2, and SSSE3 are present. This feature set was first introduced on the
Intel Nehalem processor family (released 2008), Intel Silvermont Atom
processor family (released 2013), AMD Bulldozer processor family (released
2011), and AMD Bobcat processor family (released 2011). In initial tests,
this improves performance of key system building blocks (FIDL HLCPP/LLCPP,
malloc/free) substantially, likely due to the baseline including CRC32C
instructions (used by our memory allocator Scudo).

Raising the baseline past x86-64-v2, to -v3 adds AVX, AVX2, BMI1, BMI2,
F16C, FMA, LZCNT, MOVBE, and XSAVE; this would exclude Intel Sandy Bridge,
Intel Ivy Bridge, AMD Bulldozer/Piledriver/Steamroller, and all Intel
Atom CPUs, which would not be tenable.

## Implementation

This change can be implemented in a single CR, 484898. The CR will pass
'-march=x86-64-v2' to compilers for the Fuchsia build, allowing compilers
to generate code using new baseline features.

We will also change the default in the compiler/toolchain when x86_64-fuchsia
is passed as a target, so that any third-party code targeted at Fuchsia and
built with the toolchain will benefit from the ABI.

## Performance

This proposal is expected to improve the performance of all compiled C/C++ code
on the x86-64 platform. Initial microbenchmarks in perfcompare bear this out:

* 716 test cases out of 2914 got faster; some key ones by substantial amounts
* Of note, free() / delete of 100 byte objects got 30 ns faster (baseline 83ns);
  this is substantially because CRC32C instructions, used in malloc, are now
  assumed to be available.
* Most FIDL HLCPP and LLCPP encode/decode microbenchmarks got faster; some
  substantially (HLCPP/Builder/OptionalStructTree/Depth8/Steps.Build/WallTime
  saved ~4 millseconds (15 ms baseline) for example).

[perfcompare](https://logs.chromium.org/logs/fuchsia/buildbucket/cr-buildbucket.appspot.com/8855652594502552032/+/u/compare_perf_test_results_without_and_with_CL/stdout)

## Security considerations

This proposal has no security considerations.

## Privacy considerations

This proposal has no privacy considerations.

## Testing

This change will be tested by running it through Fuchsia CI/CQ tests and
by local testing of core-tests. In addition, we will boot/test Fuchsia on
various processor families near the feature-definition edge - AMD Piledriver,
Intel Apollo Lake Atoms, and other processors as available.

## Documentation

We will need to document the new feature baseline in the Fuchsia platform
support docs, under the 'Set up Fuchsia hardware' section of fuchsia.dev.
Currently a handful of specific x86-64 devices are listed, but no feature
baseline is specified.

## Drawbacks, alternatives, and unknowns

* What are the costs of implementing this proposal?

This change will prevent certain older x86-64 processors from running Fuchsia,
processors which can do so today:
** Intel Pentium 4 / Pentium D 64-bit processors
** Intel Core 2 Duo / Quad processors
** Intel 'Bonnell' Atom processors
** AMD K8 NPT (rev F) processors
** AMD K10 processors

These processors would also not be able to run Fuchsia under KVM or other
hypervisors that do not provide emulation of the missing instructions.

The newest of these processor families was released in 2008. The last new
processor in any of these families was released in 2013.

* What other strategies might solve the same problem?

Dynamic enumeration of these features and handwritten assembly/intrinsics could
capture some/much of this performance win, but would require manual analysis
across our codebase and would not capture as much performance as assuming
certain features are available and generating branchless code to use them.

## Prior art and references

* [Define additional x86-64 micro-architecture levels](https://gitlab.com/x86-psABIs/x86-64-ABI/-/commit/77566eb03bc6a326811cb7e9)
* [ELF x86-64-ABI psABI](https://gitlab.com/x86-psABIs/x86-64-ABI)
* [New x86-64 micro-architecture levels](https://gcc.gnu.org/pipermail/gcc/2020-July/233088.html)
