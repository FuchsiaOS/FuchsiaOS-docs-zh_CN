{% set rfcid = "RFC-0107" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- *** DO NOT EDIT ABOVE THIS LINE -->

## Summary

In Component Framework v2 (CFv2), a **parent component** can add **dynamic child
components** to the topology at runtime using [collections]. However, the set of
capabilities [offered][offer] to such a dynamic child component cannot be
configured at runtime. Offers are declared in the parent component's manifest,
which is immutable by the time the parent component has started running.

The parent component's manifest treats all the dynamic children in a given
collection uniformly; it has no way of even _talking about_ an individual
component within a collection. This implies certain limitations:

1. It is not possible to offer a capability to an individual dynamic child.
   Offering a capability to a collection offers the capability to _all_ dynamic
   children in that collection.
2. It is not possible to offer a capability from an individual dynamic child to
   another component. Offering a capability from a collection creates an
   aggregate FIDL service that allows connecting to _any_ dynamic child in that
   collection.

This document proposes **dynamic offers**, which can be added to the component
topology at runtime.

## Stakeholders

*   Users of CFv2 that build dynamic topologies at runtime. They will be
    represented by the [**Driver Framework**][driver-framework] team.
*   **Component Framework area owners**, who can verify that the design is
    sensible and doesn't conflict with any future plans.
*   **Security**, who can ensure this feature does not undermine the system's
    security properties.

## Motivation

Most CFv2 use cases involve a static topology of components working together to
accomplish some task. The identities and relationships between all those
components are known up front and can be hard-coded into component manifests.

In some other use cases, the developer doesn't know the exact set of components
that will run, but _does_ know how these dynamic components will relate to each
other. The developer can group all the dynamic components into collections,
where each component in a given collection is offered the same set of
capabilities.

However, some users of CFv2 want to build complex topologies at runtime,
offering a unique set of capabilities to each dynamic child, and routing
capabilities between dynamic children.

### Use case: Drivers-as-Components

In Driver Framework's Drivers-as-Components project, dynamically-instantiated
driver components depend on capabilities provided by other
dynamically-instantiated driver components. We'll call capabilities that are
offered from one driver component to another **inter-driver capabilities**.

CFv2 does not natively support inter-driver capabilities. The Driver Framework
team has worked around this by implementing capability-routing-like
functionality in `driver_manager` itself. This workaround has some pretty rough
edges and exposes confusing differences between inter-driver capabilities and
"normal" capabilities:

*   Components connect to "normal" capabilities by opening a path in their
    incoming namespace, but connect to inter-driver capabilities via a
    Driver-Framework-specific `exposed_dir` field.
*   Once given an `exposed_dir`, the receiving component can access _all_ the
    protocols implemented by its dependency. `driver_manager` doesn't attenuate
    the receiver's access to only the protocols it needs. Driver Framework team
    could fix this, but doing so would further duplicate Component Framework
    mechanisms.
*   Driver components will need to declare the "normal" capabilities they depend
    on in their manifests' `use` section, but _should not_ list inter-driver
    capabilities in the same way. If they did, Component Manager would not be
    able to find an `offer` to satisfy the `use`.
    *   Currently, Component Manager mostly ignores these extraneous `use`s, but
        that could cause confusion in the short term, and extraneous `use`s
        might become errors in the longer term.

Driver authors should not have to see "normal" capabilities as different from
inter-driver capabilities, so the distinction should be removed as soon as
possible. The longer we wait, the more drivers will have to migrate when the
model changes.

## Design

This proposal extends the [`fuchsia.sys2.Realm/CreateChild`][CreateChild] API to
accept a list of [`OfferDecl`][OfferDecl]s describing additional capabilities
that will be offered to the newly created child, beyond those that would
normally be offered. We'll refer to `OfferDecl`s specified this way as **dynamic
offers**. `OfferDecl`s specified in the normal way (that is, listed in the
Component Manifest) will be called **static offers**.

Any kind of offer (i.e., protocol, directory, etc) can be used as a dynamic
offer.

Dynamic offers always target the newly created child component. As a result, the
set of dynamic offers targeting a component is known at component creation time
and is immutable thereafter. (Whether the `target` field of the `OfferDecl` must
actually be specified or not will be determined in API review.)

Any source that would be valid for a static offer is also valid for a dynamic
offer. Additionally, unlike static offers, dynamic offers can use a "sibling"
dynamic child component (i.e. a component with the same parent as the target) as
a source. If a dynamic offer has a dynamic child as a source, there's no
restriction that the target needs to be in the same collection as the source.

Once the child component is created, the dynamic offers should behave exactly
like static offers, including for considerations like component shutdown order.

### Destruction

A dynamic offer will be destroyed when either of its source or its target is
destroyed. After the offer has been destroyed, the system will behave exactly as
if it had never existed.

Where possible, users of dynamic offers should prefer destroying targets before
sources. The process of removing an offer from the topology is trivial if it's
the target that's being destroyed (if an offer falls in a forest and there's no
component around to hear it...). However, sometimes it can't be helped, and the
source will be destroyed before the target. Notably, when using a single-run
collection (introduced in [RFC-0101]), dynamic components are destroyed as soon
as they terminate, which can happen at any time.

Even if it's the source being destroyed, the process is straightforward:

1. Component manager shuts down the source component. This prevents the target
   from opening any new connections to the source, and instructs the runner to
   stop the source component (if it was running).
2. When the source component stops, any channels between the source and target
   will close.
3. The source component and the dynamic offer will be removed from the topology
   simultaneously.

From the target's perspective, the channel corresponding to the capability
closes and any attempts to re-acquire the capability fail permanently, as if the
capability hadn't been routed in the first place.

This RFC provides no mechanism for re-creating such an offer.

### Dependency cycle prevention

CFv2 requires that components and the strong offers between them form a directed
acyclic graph so that components can be gracefully shut down in dependency
order. For static offers, we can ensure there are no dependency cycles by
looking at the component manifests.

For dynamic offers, the following precepts are sufficient to ensure an acyclic
dependency graph:

1. Dynamic component creation order is a strict total order. In order words, no
   two dynamic components are added to the topology at exactly the same time.
2. Dynamic offers are always created at the same time as their target component.
3. All offers (both static and dynamic) are immutable.
4. All offers (both static and dynamic) must always have a valid source and a
   valid target. This property is checked by `CreateChild` on dynamic offer
   creation, and whenever the source or the target is destroyed, the offer is
   destroyed as well.

Put together, this means that a dynamic offer's source is always older than its
target. If the source was younger, that would mean it didn't exist when the
target was created, and therefore the offer would have had an invalid source at
creation time. Since "creation order" is a strict total order, no dependency
cycles are possible.

### Handling invalid routes

As of this writing, Component Manager does not proactively reject component
manifests with invalid routing configuration. That is, if a capability is
offered from a source that doesn't expose it or if a component doesn't receive
an offer for a capability it uses, no error is raised at component creation
time. An error only gets raised at the point when a component tries to open an
invalidly routed capability.

In the spirit of "dynamic offers should behave like static offers", we'll
implement the same behavior here. A component may create a dynamic offer for a
capability that the source doesn't expose. Likewise, a parent component may
create a dynamic child component without providing an offer for every capability
it uses.

In the future, if Component Manager changes to return errors earlier in these
situations, we will alter the contract of `CreateChild` to return errors as
well.

## Implementation

The user-facing surface of this change consists of:

1. A backwards-compatible change to the `CreateChild` API to accept a list of
   dynamic offers as an argument.
2. A per-collection opt-in setting to allow dynamic offers to target the
   components in that collection. (See the discussion in [Security
   considerations](#security).)
3. An allowlist for parent components that may opt-in. (Note: see [Update:
   allowlist removal](#allowlist))

Implementation of (1) will be entirely local to Component Manager.
Implementation of (2) and (3) may involve changes to tools which parse CML
files. Either way, cross-team coordination should not be necessary.

## Performance

This change should have no impact on system performance outside of calls to
`CreateChild` and `DestroyChild`, which are rare and non-performance-critical
operations. Even for these operations, no adverse performance impact is
expected.

## Ergonomics

The offer destruction behavior is new, and could create ergonomic challenges for
authors of components whose dependencies suddenly vanish.

From the target component's perspective, determining what happened after the
offer is destroyed isn't completely straightforward: it simply sees the channel
close. The target component cannot immediately tell whether the source component
was destroyed, stopped, or closed the channel intentionally. To determine the
source was in fact destroyed, the target must attempt to re-connect and await a
response from Component Manager indicating a permanent failure to route.

## Security considerations {#security}

On its own, this design undermines an important security goal of CFv2: all
capability routes (or potential capability routes) should be declared, so they
can be audited. Any usage of dynamic offers outside of tests will need to ensure
that potential capability routes can be audited via some other mechanism besides
component manifests.

By using this feature, Driver Framework accepts responsibility for ensuring that
the space of potential capability routes can be audited. Currently, the rules
are encoded in a number of places: decentralized bind rules, source code for
board drivers, ACPI, and maybe others. This makes auditing a challenge. This gap
is not new and will need to be addressed by Driver Framework, but how to do so
is beyond the scope of this RFC.

An opt-in setting will be added to `CollectionDecl`, which when present will
allow the parent component to pass dynamic offers to components created in that
collection. This will make it clear to a CML reviewer that dynamic offers are in
play, and that sources outside the component manifest need to be consulted in
order to understand the scope of potential capability routes.

Furthermore, the ability to opt-in will be allowlisted by Component Framework
team, so they can ensure all usage is appropriate. (Note: see [Update: allowlist
removal](#allowlist))

### Future work

If we find more use cases for dynamic offers, we may want to introduce **offer
upper-bounds** to the component manifest schema. Offer upper-bounds can be used
to constrain the kinds of dynamic offers that can be created at runtime.

The manifest would include statements like:

* "Protocol `fuchsia.example.Foo` can be dynamically offered from component `A`
  to any component in collection `B`", or
* "Any directory capability can be dynamically offered between any two
  components in collection `C`".

For the initial Driver Framework use case, we consider the coarser
collection-level opt-in to be sufficient. This stance MUST be re-evaluated
before allowlisting any other clients.

## Privacy considerations

This proposal does not expose any part of the system to data to which it didn't
already have access, so it should have no impact on privacy.

## Testing

Unit and integration test coverage will be added, similar to any other Component
Framework feature. Additional coverage will be provided indirectly by Driver
Framework's integration tests.

This feature should neither help nor harm the testability of the Component
Framework API. Using collections and the `Realm` protocol already requires
working with a real Component Manager instance in an integration testing
setting, and that will continue to be the case.

## Documentation

Since dynamic offers will be allowlisted and initially used only by a single
client, high-level Component Framework concept documentation should not be
updated to mention the existence of this feature. Documentation on the changes
to `CreateChild`, `DestroyChild`, and `CollectionDecl` should be sufficient.

This stance MUST be re-evaluated before allowlisting any other clients.

## Drawbacks, alternatives, and unknowns

The primary drawback of this design is that it makes auditing capability routes
more complex, since not all potential capability routes appear in component
manifests. However, this is intrinsic to Driver Framework: the topology of
driver components is inherently dynamic, as devices can be added and removed
from a machine on the fly. Static offers are clearly insufficient, so the story
*must* become more complicated. Dynamic offers attempts to keep this additional
complexity to a minimum by mirroring the behavior of static offers as much as
possible.

### Alternative: Capability tokens

**Capability tokens** have previously been proposed to solve a related problem.
That proposal can be thought of as a generalization of this RFC, where dynamic
offers can cross between realms. The parent of a source component "mints" a
capability token, which it passes to another parent component (potentially via a
chain of intermediate components). The receiving component can pass this token
into `CreateChild`, which acts as an authorization mechanism to demonstrate both
parents consent to the offer.

Once created, this dynamic offer should behave like a static offer, with the
major exception that these dynamic offers can cross _between realms_. Static
offers (and in the main proposal, dynamic offers as well) are always local to a
single parent component.

The implications of cross-realm offers aren't fully clear, and we do not believe
them to be necessary, so we have rejected this option.

### Alternative: Offering "live capability providers"

[fxrev.dev/483166](https://fxrev.dev/483166)

In the main proposal, dynamic offers are inert data. Offers represent
instructions for how Component Manager ought to route a request for a
capability, but don't provide access to the capability on their own.

We could instead pass around **capability providers**, channels that can be used
to obtain capabilities directly. `CreateChild` would then accept a vector of
named capability providers. When the dynamic child component tries to open a
dynamically-offered capability, that `Open` request will be forwarded to the
capability provider.

A parent component could obtain such a capability provider by:

*   implementing the capability provider on its own,
*   calling a `Realm` API to get a capability provider for one of its parent's
    or children's exposed capabilities, or
*   receiving a capability provider from another component over a channel.

This would unlock the same use cases as capability tokens, but has the added
benefit (or debatably, downside) that parents can offer capability providers
directly to their children: simply start up a server in memory and pass a
capability provider to `CreateChild`.

However, capability providers are incompatible with many valuable properties of
CFv2's routing framework: auditability of capability routes, persistence between
restarts, etc.

### Alternatives: Destruction

When a dynamic component that is the source of an offer is destroyed, there are
a number of reasonable behaviors other than destroying the offer. This section
discusses these alternatives.

#### Forbidding source destruction

An earlier draft of this RFC proposed simply forbidding destruction of a
component if it was the source of a dynamic offer. Components would be "pinned"
in this way, until the targets of those offers had been destroyed.

Even from the start, this seemed brittle and presented potential ergonomic
issues (i.e. a "component could not be destroyed" error may be very difficult to
handle). However, it became truly unworkable with the acceptance of [RFC-0101],
which introduced scenarios where components are destroyed automatically.

#### Destruction propagation

Another proposal was for destruction to propagate along offers. That is, if the
source of a dynamic offer was destroyed, the target would be destroyed as well,
and so on, recursively.

This option seemed scary and inconsistent with other CFv2 behavior. There are
many circumstances where capabilities are not routable or become unavailable,
and none of those circumstances cause compulsory termination or destruction of
components that depend on them, even when the dependency is strong. Components
can observe the fact that their dependencies are unavailable and quit if they so
choose.

## Update: allowlist removal {#allowlist}

This feature has proven successful at enforcing least privilege in dynamic
components.

This feature may have utility in the Chromium codebase with the migration of the
`web_instance` component to the modern component framework. Today this component
launches dynamic children using the functionality of the legacy component
framework. The feature set of these children can be varied at runtime, and when
certain features are enabled the children are provided with extra protocol
capabilities through the `additional_services` field during legacy component
creation. Dynamic offers would provide a clear and easy migration path for this
behavior to the modern component framework, by allowing `web_instance` to decide
at runtime which dynamic children will receive additional capabilities.

The allowlist is thus being removed, and clients may use the feature without
receiving approval from the Component Framework team.

In order to remove the allowlist, the following changes will occur:

- Write documentation on how to use dynamic offers.
- Remove the `dynamic_offers` restricted feature key from CMC.
- Delete the `dynamic_offers` allowlist in
  `//tools/cmc/build/restricted_features`.

[collections]: /docs/concepts/components/v2/realms.md#collections
[offer]: /docs/concepts/components/v2/capabilities/README.md#routing-terminology
[driver-framework]: /docs/development/drivers/concepts/fdf.md
[CreateChild]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.sys2/realm.fidl;l=58;drc=599c35934155b755453a2d9c228a434436e62db5
[OfferDecl]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.sys2/decls/offer_decl.fidl;l=14;drc=1969824ecf7b1e2096ca1b6c1587545699706da8
[RFC-0101]: /docs/contribute/governance/rfcs/0101_dynamic_components_with_numbered_handles.md
