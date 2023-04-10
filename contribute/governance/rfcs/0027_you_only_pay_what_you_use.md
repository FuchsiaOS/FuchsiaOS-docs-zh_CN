{% set rfcid = "RFC-0027" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-027.

## Summary

This document proposes a design principle that we should apply during the RFC
review process:

> You only pay for what you use

Specifically, when adding functionality to FIDL, we should evaluate the costs
that adding that functionality imposes on people who use FIDL but do not use the
new functionality.  We should then have a very high bar for accepting
functionality that imposes costs on people who do not use the functionality.

## Motivation

One of the most important aspects of FIDL is that Fuchsia uses FIDL pervasively
for interprocess communication and therefore for defining the [system
ABI](/docs/concepts/packages/system.md).

Many use cases for interprocess communication are performance-critical.  When
people are evaluating technologies for these performance-critical use cases,
FIDL is competing against custom message formats and ad-hoc serialization and
deserialization routines.

Other uses cases for interprocess communication are flexibility-critical.  When
people are evaluating technologies for these flexibility-critical use cases,
FIDL is competing against protobufs or a host of other networking-oriented
message formats.

In order to succeed at being used pervasively throughout Fuchsia, FIDL needs to
address both needs.  Specifically, protocol designers working in FIDL need the
ability to make tradeoffs between performance and flexibility in order to meet
their needs.

Adopting the design principle of "you only pay for what you use" lets
performance-critical customers avoid paying for functionality that support
flexibility whereas its dual, "do pay for what you do use," lets FIDL serve
flexibility-critical customers.

## Design

This section describes the history of how we arrived at this design principle as
well as positive and negative examples.

### History

FIDL is an evolution of the
[Mojo](https://chromium.googlesource.com/chromium/src/+/HEAD/mojo/)
interprocess communication system.  At the time, Mojo was significantly more
flexible than FIDL, which worked well for flexibility-critical use cases.
However, customers with performance-critical use cases were unwilling to adopt
Mojo because the flexibility offered by the system did not meet their needs.

The original design for FIDL2 (the current version iteration of FIDL as of this
writing, circa 2017-03-01) picked a different point in the design space.  In
order to win over performance-critical customers, FIDL2 is significantly less
flexible than Mojo, which lets FIDL2 be performance-competitive with custom
message formats and ad-hoc serialization and deserialization routines.  Some
clients still demand ad-hoc serialization and deserialization routines, but
FIDL2 has succeeded in being used pervasively for message formats for
interprocess communication.

The original design for FIDL2 over-rotated towards performance and needed to be
improved to meet the needs of flexibility-critical customers.  In order to be
successful, FIDL needs to add functionality that supports use cases for
flexibility without compromising on performance for customers that do not
require flexibility.

### Structs and tables

A positive example for "you only pay for what you use" is [RFC-0047](/docs/contribute/governance/rfcs/0047_tables.md),
which introduced tables.  Rather than replace structs (which have a fixed size
and layout, supporting performance-critical use cases), tables are a separate
data type that support flexibility-critical use cases.  Protocol designers are
able to choose whether to pay for this flexibility.

### Extensible unions

Another important example is [RFC-0061](/docs/contribute/governance/rfcs/0061_extensible_unions.md), which
introduces extensible unions.  This example illustrates that we should not
blindly apply the principle.  In that design, there is a choice of whether to
introduce extensible unions as a separate concept or whether to replace all
non-extensible unions with extensible unions.

This choice boils down to making a value judgment weighing the performance cost
of imposing flexibility on all clients of union against the complexity cost of
having two largely overlapping constructs (e.g., imposing cognative load on
protocol designers to pick the right construct for their use case).  In this
case, we analyzed the clients of unions and decided that the vast majority of
them value flexibility, which means imposing the costs of flexibility upon the
vast majority of union client does not cause them to pay for functionality they
do not use.  For the handful of uses that did not value flexibility, we
consulted with the customers and agreed that the extra costs would not be
burdensome.

## Implementation strategy

One strategy for designing an interprocess communication system is figure out
the ideal balance of all concerns up front and then implement the system.
Unfortunately, the concerns involved in designing an interprocess communication
system are sufficiently complex that this strategy is beyond human ability.
Instead, we are pursuing a strategy by which we do as well as we can today and
then iteratively refine the design to better address the needs of our customers.

Broadly speaking, there are two strategies we can use to balance the concerns of
performance and flexibility: we can approach the ideal balance from either
overemphasizing performance or overemphasizing flexibility.

Another way to interpret this document is as proposing that we structure the
engineering program for FIDL to start by overemphasizing performance (as in the
original FIDL2 design) and then approach the idea balance between performance
and flexibility by adding flexibility while holding the line on performance.

In evaluating changes to FIDL, and as part of the RFC process, we expect this
principle to be weighed against other design considerations. When two principles
are at odds, there are a number of approaches appropriate to resolving ties:
evaluate the impact with potentially affected users, look at prior art (e.g.,
optimization work of Protobuf, or FlatBuffers design choices), think of who
needs to absorb the complexity (e.g., users, language designers, binding
authors), consider whether the design puts a limit on the theoretical max
performance (even if today's implementation falls short of that). Ultimately, we
will need to use our judgment about how best to balance these factors.

## Documentation and examples

This document proposes adding the "you only pay for what you use" principle to
the list of efficiency goals for FIDL.

## Backwards compatibility

This principle is backwards compatible with the current FIDL design and
engineering program.

## Performance

This principle values performance.

## Security

This principle could potentially have a negative impact on security because
satisfying the principle might result in a more complex system (e.g., that has
both structs and tables).

## Drawbacks, alternatives, and unknowns

One cost of this proposal is foreclosing design space that could be used to meet
flexibility-critical use cases at the expense of performance-critical use cases.

Another cost is adopting this design principle will cause the FIDL system to be
more complex than it would otherwise have been.  For example, using tables
everywhere might be simpler than using structs in some places and tables in
other places.  This added complexity is a burden both for the FIDL
implementation and for developers who use FIDL.  To mitigate this drawback, we
should consider this complexity cost when applying the principle.

Another strategy for balancing the concerns of performance and flexibility would
be to approach the ideal balance by overemphasizing flexibility.  The difficulty
with this approach is that human beings engineer systems by adding code rather
than removing code (e.g., similar to sculpting in clay rather than sculpting in
marble).  It's easier to add flexibility by adding code than it is to add
performance by adding code.

## Prior art and references

Many other languages have adopted the "you only pay for what you use" design
principle, including C++[^1] and Rust[^2].

[^1]: B. Stroustrup. The Design and Evolution of C++. Addison Wesley, ISBN
    0-201-54330-3. March 1994.

[^2]: J. Orendorff, J. Blandy. Programming Rust. O'Reilly Media, ISBN
    9781491927274. https://www.oreilly.com/library/view/programming-rust/9781491927274/ch01.html
