<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0165" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

Note: This RFC describes SDK Categories as they were first documented. See [SDK
Categories][sdk-categories] for the current state of the world.

## Summary

Each SDK Atom has a category that defines which kinds of SDK consumers can see
the Atom. As SDK Atoms mature, we can increase their visibility, which implies
increasing their stability guarantees.

## Motivation

Fuchsia is built by combining many different components that interact using
protocols with schemas defined in FIDL. Components that are part of the Fuchsia
project interact with each other using the same mechanism that components
written by third parties interact with the Fuchsia platform. For this reason,
we benefit from having a uniform mechanism that can be used both to develop
Fuchsia and to develop for Fuchsia.

The simplest approach would be to put all the FIDL definitions into the Fuchsia
SDK, and then have all the developers use those same FIDL definitions in
developing their components. However, this approach breaks down because of a
common tension in designing APIs: API designers need the ability to iterate on
their designs and API consumers need stability in order to build on top of the
APIs.

This document describes SDK categories, which is an existing mechanism for
balancing these concerns. SDK categories predate the RFC process. This document
simply documents the existing mechanism.

SDK categories are not the only mechanism Fuchsia uses to address this tension.
For example, Fuchsia also limits which FIDL protocols a component can depend
upon through capability routing. These mechanisms are complementary because
without capability routing, component authors might be tempted to create local
copies of internal FIDL definitions rather than wait for them to be added to
the SDK. Similarly, without SDK categories, component authors could start
depending upon internal FIDL protocols to communicate among their own
components.

## Stakeholders

Who has a stake in whether this RFC is accepted? (This section is optional but
encouraged.)

_Facilitator:_

 * neelsa@google.com

_Reviewers:_

 * dschuyler@google.com
 * jamesr@google.com
 * jeremymanson@google.com
 * neelsa@google.com
 * sebmarchand@google.com
 * sethladd@google.com

_Socialization:_

This RFC skipped the socalization phase because the mechanism is already fully
implemented.

## Definitions

An *SDK Atom* is a collection of files that can be included in an SDK. Fuchsia
represents SDK Atoms using the `sdk_atom` template in GN.

An *SDK target* is a build target that creates an SDK. For example,
`//sdk:core` is the GN label for the build target that creates the Core SDK.
It might be more accurate to refer to these targets as *IDK targets* because
they create the JSON metadata for the [IDK][IDK]. A later stage of the SDK
production pipeline uses this JSON metadata to produce a fully integrated
Fuchsia SDK (e.g., with a build system like Bazel).

## Design

FIDL libraries are one example of an SDK Atom, but there are other kinds of
SDK Atoms, including C++ client libraries, documentation, and tools. SDK
categories apply to every kind of SDK Atom, but this document uses FIDL
libraries as a running example.

SDK categories balance the needs for iteration and stability in APIs by
recognizing that different API consumers have different stability needs. API
consumers that are "closer" to API designers typically have less need for
stability and often are the first customers that provide implementation
feedback for API designers.

Each SDK Atom is annotated with an SDK category, which defines which SDK
consumers can depend upon the SDK Atom. For example, if the `fuchsia.foo` FIDL
library has an SDK category of `internal`, that means only SDK consumers within
the Fuchsia project can depend upon `fuchsia.foo`. If someone wants to change
`fuchsia.foo`, they run the risk of breaking consumers inside the Fuchsia
project but they do not run the risk of breaking consumers in other projects.

As another example, consider a `fuchsia.bar` FIDL library with an SDK category
of `partner`, which means `fuchsia.bar` can be used both within the Fuchsia
project and by SDK consumers who have partnered[^1] with the Fuchsia project.
When someone changes `fuchsia.bar`, they run a larger risk of breaking
consumers because they might break the partners that depend upon `fuchsia.bar`.

Finally, consider a `fuchsia.qux` FIDL library with an SDK category of
`public`, which means `fuchsia.qux` can be used by the general public. Changing
`fuchsia.qux` is very risky because the set of software developed by the
general public is potentially unbounded and unknowable.

Along with defining concentrically increasing sets of API consumers, SDK
categories also define increasing stablity windows. For example, `fuchsia.foo`
can change dramatically from one day to the next because the `internal`
category limits the exposure to the Fuchsia project itself. Someone changing
`fuchsia.foo` can change all the clients and servers at the same time, which
means the stability window needed for the API is either very small or zero.

By way of contrast, the agreement that Fuchsia has with partner projects
includes an expectation for compatibility windows. For example, we currently
have an agreement with partners to maintain a six-week compatibility window,
although this window is likely to change in a soon-to-be-published RFC. This
agreement implies that `fuchsia.bar` cannot change dramatically from one day
to the next. Instead, we need to change `fuchsia.bar` gradually and in a way
that maintains the compatibility window.

Currently, Fuchsia do not have any SDK Atoms with an SDK category of `public`,
which means Fuchsia has not made any commitments to supporting the general
public using its APIs. However, at some point, the Fuchsia project will begin
supporting the general public using its APIs. At that time, the Fuchsia project
will need to define the compatibility window for those APIs, which will likely
be longer than the compatibility window for `partner` APIs.

A typical SDK Atom begins its lifecycle in the `internal` SDK category. At some
point, the API Council might graduate the SDK Atom might to the `partner` SDK
category, often when a partner needs access to an API contained in the Atom.
Sometime in the future, when Fuchsia has a non-empty `public` SDK category, SDK
Atoms will be able to graduate from the `partner` category to the `public`
category as well. Some SDK Atoms might remain in the `internal` SDK category
indefinitely. Others might graduate to `partner` but never graduate to
`public`.

This lifecycle is currently incomplete in that the lifecycle does not cover
deprecation and removal of SDK Atoms. For example, we might a need to add a
`historical` category for SDK Atoms that are not currently useful but still
have historical value. Such an extension to the existing model are out of scope
for this RFC.

Please note that this mechanism is complementary to `@available` mechanism for
platform versioning. The `@available` mechanism *records* when and how FIDL
APIs change. The SDK category mechanism determines the *policy* for how quickly
API designers can make changes.

[^1]: Currently, the set of partners is not public. As the project scales, we
      will likely need to revisit our approach to partnerships.

## Implementation

SDK categories have been implemented in the
[GN rules for building SDKs.](https://fuchsia.googlesource.com/fuchsia/+/main/build/sdk/sdk_atom.gni)
Each SDK Atom has an `category` parameter with one of the following values:

 - `excluded`: the Atom may not be included in SDKs;
 - `experimental`: (this SDK category does not make much sense);
 - `internal`: supported for use within the Fuchsia platform source tree;
 - `cts`: supported for use in the Compatibility Tests for Fuchsia;
 - `partner`: supported for use by select partners;
 - `public`: supported for use by the general public.

These categories form an ordered list with a monotonically increasing audience.
For example, an SDK Atom in the `public` category is necessarily available to
select partners because `public` comes after `partner` in this list.

The `experimental` category does not make much sense because we have better
mechanisms (e.g., GN `visibility`) to control use of code within the Fuchsia
platform source tree. Perhaps this category will be removed soon.

Each SDK target also has a `category` parameter that defines the set of
consumers to whom that SDK ships. The build system enforces that everything
included in an SDK target has an SDK category that is acceptable for that
audience. For example, an SDK for `partner` can include SDK Atoms authorized
for `public` (because `public` comes after `partner` in this list above) but
cannot include SDK Atoms authorized only for `internal` use (because `internal`
comes before `partner` in this list).

The `excluded` SDK category is used as a double-check to prevent certain
targets from ever being included in an SDK. Effectively, `excluded` is
documentation about that intent and is a hook for code reviewers to consider
changes to that value carefully.

## Backwards Compatibility

Typically, SDK categories change by exposing the SDK Atom to larger and larger
audiences. Shrinking the set of consumers for an SDK Atom effectively deletes
those APIs from their view, which can break those consumers if not coordinated
correctly.

## Security considerations

SDK categories are not a security mechanism. A malicious actor can read all of
the FIDL definitions from the Fuchsia open source project and make use of them
in whatever nefarious ways the attacker can imagine. This mechanism is limited
to making the engineer process run more smoothly.

## Privacy considerations

Everything about SDK categories and the SDK Atoms to which they apply is
public.

## Testing

SDK categories are enforced at build time through build configuration. However,
we do not have many tests of this mechanism. There is a risk that changes to
the SDK-related GN templates could break the mechanism, which would allow SDK
Atoms to be included in inappropriate SDK targets. We have a few redundant
mechanisms, including SDK manifests, to catch misconfigurations, but we are
relying upon code reviewers to notice inconsistencies between SDK manifests and
SDK categories in order to identify regressiosn in the SDK category mechanism.

## Documentation

The purpose of writing this RFC is to document the current state-of-the-world
for SDK categories. There is other developer-facing documentation about what
stability promises Fuchsia makes about the various SDKs that Fuchsia publishes.

## Drawbacks, alternatives, and unknowns

The main drawback of this approach is the coarse granularity at which SDK
categories can be applied. For example, you could imagine another approach in
which individual FIDL protocol elements could be assigned SDK categories,
similar to how individual protocol elements are assigned `@available`
attributes.

The advantage of the existing SDK category mechanism is that it applies
uniformly to all kinds of SDK Atoms. However, we might want a finer-grained
mechanism for certain kinds of SDK Atoms (e.g., FIDL protocols) over time.

## Prior art and references

Most platforms have a similar mechanism for gradually expanding the audience
for APIs as they become more stable. For example, Apple uses a similar
mechanism when developing APIs for macOS and iOS. In the development process
for those operating systems, each framework has three sets of APIs: internal
APIs that are available within the framework itself, private APIs that are
available to other frameworks that are part of the operating system build, and
public APIs that are available to the general public building applications for
the operating system.

Some other platforms have a "header stripping" step in their SDK release
process that removes text from headers prior to releasing SDKs to larger
audiences. Consumers who use the "unstripped" headers can depend upon a larger
set of APIs those consumers who use the stripped headers. This mechanism is
similar to SDK categories but operates at a finer granularity, and often has
fewer audience gradations.

[sdk-categories]: /contribute/sdk/categories.md
[IDK]: ../../../development/idk
