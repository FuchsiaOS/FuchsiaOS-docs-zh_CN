<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0141" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

The Fuchsia [Compatibility Test Suite (CTS)][cts] is now running in our commit
queue (CQ), ensuring that new platform code doesn't break backwards
compatibility.  However, a test suite is only as good as the tests it contains,
and ours contains few tests.  This document outlines a CTS Program for the
platform to get to full API and ABI coverage.

## Motivation

The CTS exists to determine whether a build of the Fuchsia platform, running on
a particular device, correctly implements (or *is compatible with*) the API and
ABI exposed by a particular Fuchsia SDK.  To put it another way, it demonstrates
that the build correctly implements Fuchsia.

If a system running Fuchsia passes the CTS tests for a particular ABI revision,
then its developers can have confidence that components built for that revision
will work on the system, and that the system is backwards compatible with that
revision.  The tests associated with particular ABI and API revisions we care
about are run in our CQ system.

Fuchsia Software Development Kits (SDKs) contain tools, libraries, and headers
that allow developers to target Fuchsia's APIs and ABIs.  We refer to the API
and ABI exposed to out-of-tree developers via SDKs as Platform Surface Area
(PlaSA).  Each SDK is paired with a set of CTS tests that exercise the surface
area it exposes.  The tests are available in both source and binary form.

In order for this system to provide value, we must have a robust, complete set
of tests.  Robust tests are repeatable: they allow the developer to identify the
platform behavior they are testing easily and do not exhibit flakiness. A set of
tests is complete if it exercises all of the documented behavior for that
interface.

As of September 2021, there are a handful of tests in the CTS suite.  In the
rest of this document, we identify how we will build out a robust, complete set
of tests over time.

We note that there are a lot of process issues related to how each team will
build out coverage that are not addressed in this RFC.  For example, how we
track progress is not specifically called out, and we do not provide
implementation guidance.  These issues are out of scope; if necessary,
subsequent RFCs will address them.

## Stakeholders

Section to be filled out later.

_Facilitator:_

The person appointed by FEC to shepherd this RFC through the RFC process.

_Reviewers:_

shayba@google.com, test components and libraries
ananthak@google.com, test infrastructure
abarth@google.com, Fuchsia TL

List people whose vote (+1 or -1) will be taken into consideration by FEC when
deciding whether this RFC is accepted or rejected.

_Consulted:_

List people who should review the RFC, but whose approval is not required.

_Socialization:_

This document was socialized with the CTS and test infrastructure teams.

## Design

### Requirements

We have two basic requirements that drive CTS policy.

First, CTS tests should, in the long term, provide complete coverage of the
PlaSA, which consists of ABI (currently defined as anything in the [Fuchsia
System Interface][fsi]) and API (currently defined as anything requiring API
Review +1) exposed to end-developers via an SDK, and expected tooling behavior.
A discussion of what constitutes coverage can be found in the next section.

Second, CTS tests should, to the extent possible, cover both intended and
real-world use cases of the PlaSA.  We need to cover real world cases because we
cannot claim that our tests provide any degree of compatibility if they do not
reflect what developers do with surface area elements.  We need to cover
intended cases for two reasons.  First, when the developer writes the API for
the first time, they will only have intended cases.  Second, we believe that it
is a useful exercise for developers to write intended client code when
developing an API.

In this section, we present a path forward to providing broad platform coverage
by the CTS.

### Coverage

Before we discuss what it means for our platform to reach complete coverage, we
will discuss what coverage looks like for the CTS.

Note that, in this document, "complete coverage" does not mean it has to
exercise every possible behavior - merely that it has to exercise every
documented behavior - both success and error conditions - and all of the
interfaces.  Informally, each interface a test exercises is called an element.

The CTS team is tracking coverage of FIDL and C++ methods exported via SDKs.
The team will be driving a process to ensure that we cover every FIDL and C/C++
method, and that test code covers code shipped with SDKs (e.g., the code in
libfdio).  In this document, when we refer to _full coverage_, we mean as
tracked by the CTS program.

Tests that are not part of the CTS do not count towards coverage.  Proprietary
tests - that is, tests that are not open and part of the Fuchsia platform - also
do not count towards coverage.

Note that full coverage does not ensure high quality, useful testing of all
APIs.  There is much the CTS program will not track; for example, we cannot have
tests that exercise every possible set of parameter values (every API that takes
a 32-bit integer can have 2^32 possible values; we are not likely to exercise it
with 2^32 possible tests).  API developers and reviewers are therefore
responsible for ensuring that we have high quality coverage.

To phrase it another way, CTS is a mechanism that helps us enforce
compatibility, but is not enough by itself.  It is necessary, but not
sufficient.

To ensure quality coverage, we provide two rules of thumb.  They are intended to
be used by developers to inform whether they have adequate coverage.

*First, every behavior exposed via the PlaSA that should be documented per the
[API documentation rubric][doc-rubric] should also have a test that guarantees
that behavior.* For example, the documentation rubric states that behavior when
a parameter is null should be documented.  That behavior should be exercised
with a CTS test.

Note that many APIs do not conform to the documentation rubric.  Since we
encourage writing tests as part of API development, the process of writing such
tests is a good time to flesh out the documentation and ensure that it meets the
rubric requirements.

*Second, if a change in behavior causes a regression in application code running
out of tree, then there is likely a CTS test missing.* CTS tests are supposed to
be the guarantee that new releases exhibit behavior compatible with older
releases.  If a Fuchsia build is compatible with an older release, that build
must be able to run software that targets that release.  Therefore, if a change
causes a regression that breaks application code running out of tree, there is a
compatibility behavior that was not tested - a missing CTS test.

In some cases, platform behavior may be changed to make platform behavior
conform better to the documentation.  In this case, the change should come with
a new CTS test.

A change in platform behavior may also result from a change to undocumented
behavior that we want to remain undocumented.  What we do in that case is
outside the scope of this document.  CTS tests should not exercise deliberately
undocumented behavior.

### CTS tiers

In order to encourage as much CTS coverage as possible, we will gradually
introduce more requirements for CTS coverage.  As areas build their coverage,
they move from less covered tiers to more covered tiers.  Prior to adding
tests, areas are in no tier at all.

Specific policy for the given tier is set off by blockquotes:

This is non-normative text.

> This is normative text.

#### Tier 1 (starting)

##### Covering breakages

Because one of the goals of the CTS is to ensure compatibility, and a critical
sign of incompatibility is when a platform or SDK release breaks a customer due
to a change in behavior, we propose introducing the following policy as soon as
possible:

> If a platform or SDK release causes an SDK roll failure or a failed canary
> release due to backwards incompatible changes to Fuchsia PlaSA API or
> behavior, the author of that breaking change is responsible for ensuring there
> is documentation for the new behavior and a CTS test that covers the new
> behavior.  If it is not currently possible to write CTS tests for that surface
> area element, then the team that owns that PlaSA element must prioritize a
> plan for developing them.

> If the behavior is intended to be undocumented, then an exception to the above
> may be granted.  The exception may involve documenting the fact that the
> behavior is intentionally undocumented, or providing tools to users to help
> them identify the fact that they are relying on undocumented behavior.

##### New SDK-facing features

We expect developers to start to develop platform testing capabilities alongside
new SDK-facing features.

> All additions to the PlaSA (e.g., new SDK tools, FIDL protocols or methods,
> and C++ headers) shipped with partner or public SDKs require CTS tests. Anyone
> taking a new API element through API Review and including it in a public or
> partner SDK must have a plan to include CTS tests with their new API.

It is the responsibility of the platform team owning the platform change to
provide the CTS test, and work with the CTS program to deliver that test.

As API developers work through their test plans, we strongly recommend that they
surface what kinds of tests they intend to write with their API reviewers so
that they can develop a shared understanding of the test plan, including what
tests are highest priority.

If a platform team responds to all backwards-compatibility-breaking changes to
API and ABI you own with a test, and write tests for new platform contract
functionality and behaviors, that team is in Tier 1.

Note that the requirement for CTS tests for API review only applies to PlaSA API
elements.  As long as an API is not exposed to end developers via an SDK, it
does not need associated CTS tests.

#### Tier 2 (building)

##### API/ABI evolution

Because breakages in changing code are more likely than breakages in long-term
stable code, and as a way of providing incremental coverage, we propose
introducing the following policy:

> All modifications to the API or behavior of a PlaSA element, regardless of
> whether it is a new PlaSA element or an existing one, must be accompanied by a
> CTS test that covers the change.

> If it is not currently possible to write CTS tests for an existing public or
> partner surface area element, then the responsible team must provide a
> plan for developing those tests prior to making the change, and execute on
> that plan as soon as is feasible, as agreed upon with their API reviewer.

In this tier, we strongly recommend CTS tests be written prior to major rewrites
or replacement of any services, tools, or libraries that affect PlaSA behavior,
regardless of whether the new code is intended to change that behavior.  For
example, if we were to rewrite the kernel from scratch, but keep the same VDSO
behavior, tests of that behavior should be written prior to the rewrite.

If you do everything you need for Tier 1, and you make sure that all changes you
make to ABI or API are covered by CTS tests, you have achieved Tier 2.

#### Tier 3 (complete)

##### The long tail

We recognize that there are many long term stable PlaSA elements, many of which
are not under active development.  Ultimately, we want to provide coverage for
the entire PlaSA.

> Each area must develop a plan to have complete coverage of the PlaSA
> elements it owns and deliver the results of that plan.

If you do everything you need for Tier 2, and have complete coverage of the
PlaSA elements you own, you are complete.

## Implementation

A CTS program will be initiated by the Fuchsia team to encourage full platform
coverage.  CTS coverage will be tracked by several dashboards.  The details of
these dashboards will evolve, and are outside the scope of this RFC, but they
will track basic test coverage for each PlaSA element.

The CTS team is responsible for providing developers with the infrastructure
needed to run their tests.  If test developers do not see the functionality they
need, they should partner with the CTS team to ensure that they can provide
tests that can be run in CTS infrastructure.  Developers can reach out to the
CTS team by [filing a bug in
monorail](https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=Fuchsia+Compatibility+Test+Suite+%28CTS%29).

API review will also evolve to accommodate the requirement that CTS tests
accompany changes.  For areas at Tier 2 or higher, modifications or additions to
PlaSA elements must have CTS tests.  Eventually, this requirement will be
extended to all areas; by that point, we expect to formalize testing as part of
API Review.

## Performance

The existence of tests has no impact on platform performance.  We have to track
machine usage of the CTS tests carefully to make sure that they do not exceed
Fuchsia machine capacity.

## Ergonomics

The CTS team is tasked with ensuring that it is easy for areas to write basic
CTS tests and that infrastructure exists to run those tests.  They are not
responsible for domain-specific infrastructure and frameworks.

## Backwards compatibility

This proposal does not break backwards compatibility.  The goal is to create a
mechanism to enforce backwards compatibility for the Fuchsia platform.


## Security considerations

We do not believe the CTS program will negatively impact Fuchsia security.


## Privacy considerations

We do not believe the CTS program will negatively impact Fuchsia privacy
properties.

## Testing

As this RFC largely details a process, it does not require a detailed test plan.

## Documentation

The team will work with DevRel and Tech Writers to provide detailed and useful
guidance on how to write effective CTS tests.

## Drawbacks, alternatives, and unknowns

This change creates substantial upfront work for API developers, who often do
not include tests with their API changes.  We believe that requiring developers
to write tests that use their APIs prior to releasing them will improve the
ergonomics of those APIs prior to release.

## Prior art and references

The Android system has similar processes about CTS inclusion when developing new
APIs.

Many programming languages have similar testing requirements for new APIs, as
well.

[cts]: /contribute/governance/rfcs/0015_cts.md
[fsi]: /concepts/packages/system.md
[doc-rubric]: /development/api/documentation.md
