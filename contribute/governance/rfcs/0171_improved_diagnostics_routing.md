<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0171" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

A proposal to provide utilities in `cmc` and CML to simplify the routing of diagnostics protocols
(`fuchsia.logger.LogSink` and `fuchsia.diagnostics.InspectSink`) everywhere in the tree and reduce
the DX pain point of missing logs or Inspect data. While this document focuses on Inspect and
Logging, it can also serve to improve availability of other protocols that most components might
desire to have available, like the `fuchsia.tracing.provider.Registry`.


## Motivation

A DX pain point of using logs is that components need to route `fuchsia.logger.LogSink` everywhere:
in production components, tests, in `RealmBuilder` routes, etc. Logs are a core part of most Fuchsia
experiences that we expect pretty much every component and test to use.

[RFC-0168][rfc-inspectsink] proposes using a protocol `fuchsia.inspect.InspectSink` which would
allow components to publish Inspect bringing some improvements and reducing technical debt. Just
like `fuchsia.logger.LogSink` we expect all (or at least most) components to use Inspect
instrumentation. Today every component can `expose /diagnostics to framework` which allows every
component to expose Inspect and make it available to the Archivist. By moving towards a protocol,
we must ensure that all components continue to be able to expose Inspect data which is highly
valuable to developers to debug what their components are doing at runtime.

This is non-ergonomic and error prone as we need to update all CMLs to route this protocol manually
to all components which are currently writing Inspect. `LogSink` has the same problem and
particularly in tests it’s very easy to forget to route `LogSink` to the components under test
causing missing logs and wasted developer time.

Component Manager leverages a component's `LogSink` to print routing errors attributed to that
component. This improves DX as the developer can quickly spot routing errors. However, if `LogSink`
isn't properly routed these errors end up in the global syslog attributed to Component Manager and
are easier to miss by the developer looking at its own component logs.

This document attempts to improve the situation by introducing utilities in `cmc` and CML to
simplify routing these two protocols to every component.

The Component Framework has plans to review the routing APIs and conceptualize ways to make
the model more consistent and user-friendly. This will take several quarters, so an incremental
approach using existing primitives and extending them is preferred.

## Stakeholders

_Facilitator:_ leannogasawara@google.com

_Reviewers:_

- crjohns@google.com
- geb@google.com
- hjfreyer@google.com
- shayba@google.com
- zarvox@google.com

_Consulted:_

- bryanhenry@google.com
- cgonyeo@google.com
- jmatt@google.com
- thatguy@google.com


_Socialization:_ this design was socialized in the form of a Google Docs document, a decision
document where the listed alternatives were heavily discussed, and meetings and conversations among
the stakeholders.

## Design

To make it easier for developers to never miss logs and Inspect under the current component
capability routing system, align with the principles of least privilege and hierarchical isolation,
and continue to be able to expose Inspect, we will develop the following:

- `cmc` required offers checker.
- Ability to offer a capability to all children and collections in CML.
- New CML shards for diagnostics.
- `RealmBuilder` automatically offers diagnostics capabilities to all components.

### `cmc` required offers and uses

`cmc` will get a command line option `--must-offer-protocol` containing a list of
protocol names for which it will validate that the following statement is true:

> For each child and collection declared in a manifest, there exists an `OfferDecl` from some
> source to it for every protocol defined in the list of required protocols.

Additionally, `cmc` will get an equivalent command line option `--must-use-protocol` that will check
the equivalent, but for `UseDecl`s.

The GN and Bazel tooling will be updated to pass `fuchsia.logger.LogSink` and
`fuchsia.diagnostics.InspectSink` in these options whenever `cmc` is called.

Developers who wish to entirely disable this check, regardless of how `cmc` is called for their
manifest can add the following to their CML file (this is new syntax that is introduced in CML):

```
{
    disable: {
        must_use_protocol: [ "fuchsia.logger.LogSink", "fuchsia.diagnostics.InspectSink" ],
        must_offer_protocol: [ "fuchsia.logger.LogSink", "fuchsia.diagnostics.InspectSink" ],
    }
}
```

Developers who wish to not route `LogSink` or `InspectSink` to some of their children are free to
either:

- Use [optional capability routes][rfc-155]: Route the protocol `from: "void"` to the
  child/collection for which they want to turn a single offer off.
- Manually route the protocol from the source they desire.

The `bootstrap` and `root` realms where these capabilities originate, will need some special
handling:

- `bootstrap`: will have this option turned on to ensure that `LogSink` is routed to all
  components in bootstrap. Additionally, it will add an offer `Inspect`/`LogSink` from `void` to
  Archivist.
- `root`: will have this option turned on to ensure we route `LogSink` from Archivist to all of
  its siblings. Since `bootstrap` is the one exposing this capability, we’ll add an offer
  `Inspect`/`LogSink` from `void` to `bootstrap`.

By doing this we hope that every developer on Fuchsia is less likely to mistakenly miss logs or
Inspect data.

zarvox@ built a [prototype][cmc-prototype] (and relation chain) for this section.

### Allow offering a capability to all children and collections in CML

To improve the DX of _routing to all children_ we’ll introduce syntax sugar in CML to allow routing
a capability to “all children and collections”.

This syntax sugar can be used as follows:

```
offer: [
    {
        protocol: "fuchsia.logger.LogSink",
        from: "parent",
        to: "all",
    }
]
```

When a CML file containing that syntax is compiled, N `OfferDecl`s will be generated where N is the
total number of collections and children that the component has.

An `OfferDecl` with target `all` will be gated in `cmc` for usage only for the protocols that were
defined in the new optional argument described in the previous section.

### CML shards

The following shards will be created:

```
// syslog/use.shard.cml
{
    use: [
        { protocol: "fuchsia.logger.LogSink" },
    ],
}

// syslog/offer.shard.cml
offer: [
    {
        protocol: "fuchsia.logger.LogSink",
        from: "parent",
        to: "all"
    }
]

// inspect/use.shard.cml
{
    use: [
        { protocol: "fuchsia.diagnostics.InspectSink" },
    ],
}

// inspect/offer.shard.cml
offer: [
    {
        protocol: "fuchsia.diagnostics.InspectSink",
        from: "parent",
        to: "all"
    }
]
```

The following existing shards will be updated:

- `syslog/client.shard.cml`: includes `syslog/use.shard.cml` and `syslog/offer.shard.cml`.
- `inspect/client.shard.cml`: includes `inspect/use.shard.cml` and `inspect/offer.shard.cml`.

Logical components that just perform routing and do not execute any program, can use the
`offer.shard.cml`. Components that need to use these protocols, but need to configure what to route
to their children can use the `use.shard.cml`. The rest can use the standard and convenient
`client.shard.cml`.

If a component has no children or collections, but still uses the `client.shard.cml` (since it is
using a protocol), then the `offer to all` statement in the shard will be a no-op given that it is
syntax sugar that just expands to `OfferDecl`s as mentioned earlier.

For convenience, we will provide a `diagnostics/client.shard.cml` that includes both
`client.shard.cml` files.

### `RealmBuilder` updates to support `offer to all`

To facilitate routing diagnostics protocols to all components under test, `RealmBuilder` will
receive couple updates to allow routing a protocol to all children and collections:

- Automatically offering LogSink and InspectSink to all children and collections. In Rust, it could
  look as follows:

  ```
  builder
      .add_route(
          Route::new()
              .capability(Capability::protocol_by_name("fuchsia.diagnostics.InspectSink"))
              .from(Ref::parent())
              .to(Ref::all()),
      )
      .await?;
  ```

- Since we expect all tests will want to do this, except on some niche scenarios, RealmBuilder will
  automatically route these protocols to all components. This might seem unaligned with the approach
  taken in `cmc` and CML, but the `RealmBuilder` API already deviates in some areas to provide more
  convenient workflows that are better adapted for tests. Given that we expect that 99% of the time
  we will be routing these protocols to tests components, then we will teach `RealmBuilder` to do
  it automatically and provide a way to turn it off:

  ```
  let builder = RealmBuilder::new().await?;
  …
  let instance = builder
      .route_logs_to_all(false)     // defaults to true
      .route_inspect_to_all(false)
      .build()
      .await;
  ```

## Implementation

1. Update `cmc` to support the new flags and `offer to all` in CML.
1. Add a `syslog/offer.shard.cml` that contains `offer LogSink to all`.
1. Update `cmc` usage in tree to use the new flag and update existing CMLs that might be missing
  routes. The GN and Bazel SDKs will be updated, but will default to `[]` for the set of required
  protocols until OOT CMLs have been migrated to have a complete set of offers.
1. Update `cmc` usage out of tree to use the new flag and update existing CMLs that might be missing
  routes (leveraging the offer shard).
1. Update GN and Bazel SDKs to require the diagnostics protocols.
1. Include the `syslog/offer.shard.cml` from `syslog/client.shard.cml`.
1. Once rolled, refactor OOT manifests that are using the offer shard but don't need it anymore,
   due to it being included through the client shard.

## Performance

`cmc` will perform some additional work, but it is not expected to have any significant impact in
compile time.

## Security considerations

This change aligns with Component Framework security properties, in particular the principle
of least privilege and the principle of hierarchical isolation.

## Privacy considerations

No privacy impact.

## Testing

New `cmc` features will be unit tested.

## Documentation

The [`cmc`][cmc-docs] will be updated to include the new option and the [`CML`][cml-docs] will be
updated to describe the new `offer to all` feature.

## Drawbacks, alternatives, and unknowns

### Using `debug_capabilities` in `Environment`s

This was the main alternative considered. Under this alternative we would extend
`fuchsia.sys2.Environment` to have a `diagnostics_capabilities` just like `debug_capabilities` or
turn `debug_capabilities` into `diagnostics_capabilities` or just use `debug_capabilities` for
diagnostics protocols, making them available for usage by any component in the tree
`from: diagnostics` or `from: debug`.

This feature would be gated in Component Manager security policy to ensure it is only used by the
root Archivist and by the Archivist embedded in tests.

_Pros_:

- Every component can use `InspectSink` and `LogSink` from anywhere in the tree.
- Aligned with the current state of the world in which every component can expose Inspect.
- Improved DX as developers won’t need to spend time figuring out why their component isn’t logging
  to find out they were missing an offer in tests.
- Possible to statically check point to point usage of the capability.
- All components start with “nothing” in their namespace unless they explicitly use the capability.
- Covers dynamic components created through `fuchsia.component.Realm/CreateChild`.

_Cons_:

- No more explicit parent-child offering, meaning, this is not aligned with the security principle
  of hierarchical isolation.
- Replacing or mocking `LogSink`/`InspectSink` in the topology requires adjusting the environment
  which requires a security policy change.
- Isn’t eating our own dogfood – third-party developers don’t get to use environments for arbitrary
  purposes for their protocols, why do we?

### Make `LogSink` and `InspectSink` framework capabilities

Allow these protocols to be used `from: framework`. The Archivist can either expose these
capabilities to to framework or there can be a contract between Archivist and Component Manager to
serve these capabilities.

_Pros_:

- Same pros as in the previous alternative with the detail that diagnostics protocols
  (`InspectSink` and `LogSink`) become framework protocols even if not served by Component Manager.
- Removes the need for capability attribution, as attribution would happen directly in the
  framework since each component gets its own unique set of framework capabilities.

_Cons_:

- Same cons as the previous alternative.
- First instance of a protocol used from framework that isn’t offered directly by Component Manager
 : itself.
- Not clear how to provide isolated logs in tests without building an additional mechanism to be
  used by Test Manager.
- Establishes a single log destination for all components, device-wide.

### Have `cmc` offer `LogSink` to all children automatically

Instead of having a flag that requests the user to add the `OfferDecl` to their CML, `cmc` would do
this automatically for every child and collection.

_Pros_:

- Explicit parent-child offering which can help with mocking, replacing the protocol in the
  topology, etc.
- No changes in CML.

_Cons_:

- Special treatment of a capability in CMC.
- Inconsistency in behavior of components declared in `.cml` and components constructed via
  `Realm/CreateChild`.

Providing an option in `cmc` and syntax sugar in `CML` makes this more flexible and provides a
mechanism that others could take advantage of, not only for diagnostics.

### Do nothing and route as usual

Have developers manually offer `LogSink` and `InspectSink` to all their children.

_Pros_:

- Explicit parent-child offering which can help with mocking, replacing the protocol in the
  topology, etc.
- API boundaries remain a local concern between parent and child, rather than one that involves
  other parties.

_Cons_:

- Current problems: it’s easy to miss routing LogSink leading to lost time while debugging tests.
- Additional problems: easy to miss routing `InspectSink` to some component (given that today
  everyone can expose it) leading to missing diagnostics in the field. This is just the same
  problem LogSink has, so now we have it in two protocols rather than only one.

Given how widely used these protocols are, we believe that adding additional options in `cmc` and
`CML` help reduce the likelihood of missing a route.

### Other ideas

Other ideas were discussed such as using capability bundles to route a `diagnostics` bundle
containing both protocols or improving environments in the form of domains or capability sources.
These were discarded in favor of a near term solution using existing mechanisms and APIs given the
plans to review routing APIs.


## Prior art and references

N/A

[cmc-docs]: https://fuchsia.dev/reference/tools/sdk/cmc.md
[cml-docs]: https://fuchsia.dev/reference/cml#capabilities
[cmc-prototype]: https://fuchsia-review.googlesource.com/c/fuchsia/+/678293
[rfc-inspectsink]: /docs/contribute/governance/rfcs/0168_exposing_inspect_through_inspectsink.md
[rfc-155]: /docs/contribute/governance/rfcs/0155_optional_capability_routes.md
