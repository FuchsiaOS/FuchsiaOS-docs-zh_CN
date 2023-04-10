SDK Categories
==============

Each [SDK Atom] has a category that defines which kinds of SDK consumers can see
the Atom. As SDK Atoms mature, we can increase their visibility, which implies
increasing their stability guarantees.

[SDK Atom]: /docs/glossary#sdk-atom

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

This document describes SDK categories, which is Fuchsia's primary mechanism
for balancing these concerns.

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
categories also define increasing stability windows. For example, `fuchsia.foo`
can change dramatically from one day to the next because the `internal`
category limits the exposure to the Fuchsia project itself. Someone changing
`fuchsia.foo` can change all the clients and servers at the same time, which
means the stability window needed for the API is either very small or zero. By
way of contrast, the agreement that Fuchsia has with partner projects includes
an expectation for compatibility windows.

Currently, Fuchsia do not have any SDK Atoms with an SDK category of `public`,
which means Fuchsia has not made any commitments to supporting the general
public using its APIs. However, at some point, the Fuchsia project will begin
supporting the general public using its APIs. At that time, the Fuchsia project
will need to define the compatibility window for those APIs, which will likely
be longer than the compatibility window for `partner` APIs.

An additional type of SDK category is required for the APIs used in the prebuilt
`partner` or `public` SDK atoms when it's undesirable to expose these APIs to
SDK users. These `partner_internal` and `public_internal` categories will enforce
the same API compatibility windows as the `partner` and `public` categories
without requiring adding those APIs to the SDK API surface area. Only the
`partner_internal` category will be introduced for now as there's no `public`
SDK atoms.

A typical SDK Atom begins its lifecycle in the `internal` SDK category. At some
point, the API Council might graduate the SDK Atom might to the `partner` SDK
category, often when a partner needs access to an API contained in the Atom.
Sometime in the future, when Fuchsia has a non-empty `public` SDK category, SDK
Atoms will be able to graduate from the `partner` category to the `public`
category as well. Some SDK Atoms might remain in the `internal` SDK category
indefinitely. Others might graduate to `partner` but never graduate to
`public`.

Please note that this mechanism is complementary to `@available` mechanism for
[platform versioning][fidl-versioning]. The `@available` mechanism *records*
when and how FIDL APIs change. The SDK category mechanism determines the
*policy* for how quickly API designers can make changes.

[^1]: Currently, the set of partners is not public. As the project scales, we
      will likely need to revisit our approach to partnerships.
      
[fidl-versioning]: /docs/reference/fidl/language/versioning.md

## Categories

SDK categories have been implemented in the [`sdk_atom`][gn-sdk-atom] GN Rule.
Each SDK Atom has an `category` parameter with one of the following values:

 - `excluded`: the Atom may not be included in SDKs;
 - `experimental`: (this SDK category does not make much sense);
 - `internal`: supported for use within the Fuchsia platform source tree;
 - `cts`: supported for use in the Compatibility Tests for Fuchsia;
 - `partner_internal`: supported for use in non-source SDK atoms in the
   `partner` category but not exposed to the SDK users;
 - `partner`: supported for use by select partners;
 - `public`: supported for use by the general public.

These categories form an ordered list with a monotonically increasing audience.
For example, an SDK Atom in the `public` category is necessarily available to
select partners because `public` comes after `partner` in this list.

The `experimental` category does not make much sense because we have better
mechanisms (e.g., GN `visibility`) to control use of code within the Fuchsia
platform source tree. Perhaps this category will be removed soon.

Each [`sdk`][gn-sdk] GN target also has a `category` parameter that defines the
set of consumers to whom that SDK ships. The build system enforces that
everything included in an SDK target has an SDK category that is acceptable for
that audience. For example, an SDK for `partner` can include SDK Atoms
authorized for `public` (because `public` comes after `partner` in this list
above) but cannot include SDK Atoms authorized only for `internal` use (because
`internal` comes before `partner` in this list).

The `partner_internal` SDK category is used to give some APIs the same
compatibility constraints as `partner` APIs without exposing them to the SDK
users.

The `excluded` SDK category is used as a double-check to prevent certain
targets from ever being included in an SDK. Effectively, `excluded` is
documentation about that intent and is a hook for code reviewers to consider
changes to that value carefully.

[gn-sdk-atom]: /build/sdk/sdk_atom.gni
[gn-sdk]: /build/sdk/sdk.gni


## Change history

- First documented in [RFC-0165: SDK categories][rfc-0165].

[rfc-0165]: /docs/contribute/governance/rfcs/0165_sdk_categories.md
