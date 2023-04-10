{% set rfcid = "RFC-0004" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

In Fuchsia, specific notation is used to denote multiples of bytes. This
improves clarity by eliminating the need to guess the value of "MB".

The [IEC](https://en.wikipedia.org/wiki/Kibibyte) notation will be used for
multiples of bytes (e.g. RAM, disk, file sizes).

- When denoting groups of bytes, all Fuchsia developer documentation and
  tools will present multiples of bytes in 1024 units (i.e. powers of 2).
- The labels kibibyte, mebibyte, gibibyte are used for 1024, 1024^2, and 1024^3
  respectively.
- When describing bytes in abbreviated form, always use the "i" notation: KiB,
  MiB, GiB, etc. The first and last letters are capitalized and the middle "i"
  is always lowercase.
- Abbreviations KB, MB, GB (regardless of letter case) will not be used in
  developer facing materials. Nor will kilobyte, megabyte, gigabyte be used.

These guidelines are for developer facing materials. This RFC does not attempt
to provide guidelines for user facing (marketing/sales) notation.

## Motivation

Some folks measure bytes in powers of 2. So while a kilometer is 1000 meters,
these folks read a kilobyte as 1024 bytes.
[Henkemans](https://www.google.com/books/edition/C++_Programming_for_the_Absolute_Beginne/ea2SOugw6g8C?gbpv=1&pg=PP56)
See also,
[Google search](https://www.google.com/search?tbm=bks&q=kilobyte+is+1024+bytes)

In [SI units](https://en.wikipedia.org/wiki/International_System_of_Units), the
values of kilo-, mega-, giga- and so on are powers of 10.

The two possible definitions for kilobyte create confusion with developers,
for example:

- in powers of two, 65536 bytes is written as 64 KB
- in powers of ten, 65536 bytes is written as 65 KB (with some leftover)

The same value can be written unambiguously as 64 KiB, since KiB is always a
power of two (1024).

This difference can lead to a significant drift in accuracy. The
[error increases with larger values](https://en.wikipedia.org/wiki/Byte#/media/File:Binaryvdecimal.svg)
.

See also:
[Wikipedia](https://en.wikipedia.org/wiki/Kilobyte#Base_2_)

## Design

All Fuchsia developer documentation and tools will present multiples of bytes in
1024 units (i.e. powers of 2).

E.g. the size of 65536 is 64 KiB, exactly.

## Implementation

Bugs will be filed as found. These bugs should be given a relatively high
priority and addressed prior to the next version launch.

The expectation is that these changes will not be burdensome if handled quickly,
as they appear. Whereas it may be burdensome to conduct a thorough review of all
our documentation and tools.

This document will serve as a decision on which units to use. Tip: include a
link to this RFC in bugs filed to provide context.

## Performance

This change could alter the perceived performance of other software. E.g. if an
existing test measured throughput as `bytes / 1000 / time` and changed to
`bytes / 1024 / time`, the throughput may appear to lessen, even though the true
performance is identical (i.e. when `bytes` and `time` are unchanged).

## Security considerations

If an API or config accepts a size input that is internally multiplied by 1000,
an issue may arise if that is changed to 1024. (This case is not known to
happen as of this writing).

## Privacy considerations

No privacy issues are foreseen.

## Testing

Since the solution proposed is an ongoing list of bugs and CLs as issues are
found, testing will be handled by the Testability requirements of the individual
CLs.

## Documentation

Clear communication is the heart of this proposal. Documentation will be updated
to conform this this RFC as non-conformance issues are found.

### Guidelines

These guidelines are for developer facing materials. This RFC does not attempt
to provide guidelines for user facing (marketing/sales) notation.

When describing bytes in abbreviated form, always use the "i" notation: KiB,
MiB, GiB, etc. The first and last letters are capitalized and the middle "i" is
always lowercase.

When describing bytes without abbreviating, use kibibyte, mebibyte, gibibyte,
etc. as described by [IEC](https://en.wikipedia.org/wiki/Kibibyte).
Capitalization for the unabbreviated values follow US English or coding
style guides, depending on context.

Abbreviations KB, MB, GB (regardless of letter case) will not be used in
developer facing materials. Nor will kilobyte, megabyte, gigabyte be used.

## Drawbacks, alternatives, and unknowns

What are the costs of implementing this proposal?

- By declaring a design choice and allowing changes over time, the cost is
  expected to be low (i.e. blended into the review process and normal bug
  handling).

What other strategies might solve the same problem?

- Some OSes use multiples of 1000 or 1024 depending on the version of the OS.
  The differences may occur due to partnership agreements, and appear to go
  either way (using powers of 2 or powers of 10). This happens at a consumer
  facing level and is not expected at a developer level. Since this RFC is
  concerned with developer facing documentation, tools, API, etc. this
  alternative is not used.

- Some tools offer a choice of displaying values in different multiples. We
  believe this unnecessarily complicates those tools.

- Continue to use powers of two for bytes (without having the 'i' infix). This
  isn't reasonable since we are unable to remove uses of powers of ten for
  bytes. I.e. it will continue to be unclear to some readers how many bytes are
  in a KB.

## Prior art and references

There is quite a bit of prior art and references available. Some examples are
included here, but interested readers may easily Google for more history
(including which companies made which choices and when). It can be interesting
trivia, but it's unlikely to have bearing on this proposal for Fuchsia.


Other references:

[Ubuntu](https://wiki.ubuntu.com/UnitsPolicy#References).

### Standards

There are multiple standards to choose from, and this RFC chooses one standard
from the many that exist.

Examples

-  IEC introduces the KiB, MiB, GiB...
    - [wikipedia MiB](https://en.wikipedia.org/wiki/Mebibyte)
-  JEDEC firmly defines units of bytes as base 2
    - [wikipedia Memory Standards](https://en.wikipedia.org/wiki/JEDEC_memory_standards)
- Apparently some manufacturers even used mixed notation, such as 1024 * 1000
  for megabyte.
- Some OSes use multiples of 1000, and some 1024
- Some use different standards depending on whether the topic is RAM or disk
  storage

Rather than trying to introduce another standard or set of rules, this proposal
suggests using the IEC guidelines, where bytes are counted in multiples
of 1024, in all developer facing docs, tools, APIs, and so on.
