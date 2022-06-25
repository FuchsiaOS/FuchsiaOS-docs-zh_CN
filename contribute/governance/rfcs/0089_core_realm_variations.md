{% set rfcid = "RFC-0089" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- *** DO NOT EDIT ABOVE THIS LINE -->

<!--
*** This should begin with an H2 element (for example, ## Summary).
-->

## Summary

GN variables set in product definitions are used to generate a custom core realm
in the component framework for that product.

## Motivation

Today the core realm in CFv2 of the component framework is the component which
holds most other packaged components in the v2 system. There is a single version
of this component, which results in every product having the same set of static
v2 components and capability routes for those components. This results in both
components existing that have no purpose and will never be run, and overly broad
capabilities being offered to components, as the core realm has to be built to
be a union of all product configurations.

Additionally, the v1 component framework supports at build time conditionally
adding capabilities and components to the `sys` realm for specific products. As
components are migrated from v1 to v2, components which already depend on this
will further exacerbate the issues with the current model requiring the core
realm to have a single definition that meets the requirements of every product
at once.

Some components have already been migrated from the v1 `sys` realm into the v2
`core` realm, with an increased pace of migrations expected in the second
quarter of 2021.

Due to the above, we need the ability to tailor the exact contents of the
central structure of the CFv2 realms to fit the product on which it is running.

## Scope

This design aims to unlock additional component migrations in the very short
term until a more advanced, future-proof solution is ready. This design is not
aiming to be suitable for out of tree products, nor is it aiming to be a
suitable long-term solution. The PDK efforts, which aim to enable out of tree
product assembly, will solve the same problems addressed here in a more holistic
fashion.

## Design

The contents of the core realm manifest will be split up into a common base and
CML shards that contain components and their capability routes that are
optionally included by the product. For example, the `/core/temperature-logger`
component which is present in CFv2 today is only functional on the `astro`
product. Instead of including things relevant to `temperature-logger` in the
base core manifest, the following shard would be added in a separate file:

```
{
    children: [
        {
            name: "temperature-logger",
            url: "fuchsia-pkg://fuchsia.com/temperature-logger#meta/temperature-logger.cm",
        },
    ],
    offer: [
        {
            protocol: "fuchsia.thermal.test.TemperatureLogger",
            from: "#temperature-logger",
            to: [ "#appmgr" ],
        },
        {
            protocol: [ "fuchsia.logger.LogSink" ],
            from: "parent",
            to: [ "#temperature-logger" ],
        },
        {
            directory: "dev",
            from: "parent",
            as: "dev-temperature",
            to: [ "#temperature-logger" ],
            subdir: "class/temperature",
        },
        {
            directory: "dev",
            from: "parent",
            as: "dev-thermal",
            to: [ "#temperature-logger" ],
            subdir: "class/thermal",
        },
        {
            directory: "config-data",
            from: "parent",
            to: [ "#temperature-logger" ],
            subdir: "temperature-logger",
        },
        {
            protocol: [
                "fuchsia.device.Controller",
                "fuchsia.hardware.temperature.Device",
            ],
            from: "parent",
            to: [ "#temperature-logger" ],
        },
        {
            protocol: "fuchsia.tracing.provider.Registry",
            from: "#appmgr",
            to: [ "#temperature-logger" ],
            dependency: "weak_for_migration",
        },
    ],
}
```

This shard will use a GN template to create a target for it, which contains the
shard's path. For our `temperature-logger` example, the target would look like
this:

```
core_shard("temperature_logger_shard") {
  shard = "//meta/temperature-logger.shard.cml"
}
```

Each product's `.gni` file (such as `//products/workstation.gni`) will be able
to specify the GN targets of the shards which they would like to be included in
the product's core realm.

```
core_realm_shards += [
  "//src/sys/core:temperature_logger_shard"
]
```

At product assembly time these targets are then gathered up using [GN's
`generated_file()` target][generated-file] and used as input to [CMC's merge
operation][cmc-merge], where the shards to be included in the product are merged
together with the core realm's base shard. In our current example, the shard for
`temperature-logger` is the only shard merged with the core base shard.

This generated core realm will be included in a package whose name is derived
from the product. So for example, the core manifest for the workstation product
would be packaged at `fuchsia-pkg://fuchsia.com/core-workstation#meta/core.cm`.

This is because components are identified using URLs. These identifiers follow
the webarch principles for identification, that "global naming leads to global
network effects". As such, a given component URL refers to the same logical
entity regardless of the context in which the URL exists. This approach implies
there is one universe of software for Fuchsia that spans all the product and
board configurations. Having each product be able to use the same URL to refer
to the core realm, and having the contents behind that URL vary per-product,
would not be following this principle.

Because the URL for the core realm will now be different between products, the
root component manifest is modified at build-time to include the correct URL for
the core realm for the current product. This product-specific root realm does
not need to be packaged in a varying fashion like the core realm, as it is
included directly in the ZBI, which is already product specific.

All components which are present on all products will be in the common base, and
shards will only be created for components which are not included on some
products. If a new product is added that wishes to exclude some component that
up until that point was included on all previous products, then this component
will be refactored out of the common base and into a shard.

### Sessions and product-specific routes

In scenarios where product-specific capabilities in the core realm are to be
routed through session manager to the session, there are two possible options.

One would be to have session manager route the superset of all capabilities
across all products. In this case the routing for unavailable capabilities would
fail at the core realm.

The second option would be to use a shard-based approach to also build
product-specific session manager manifests, identical to how this proposal
outlines this is done for the core realm. In this case the routing for
unavailable capabilities would fail at the session manager realm. This approach
would require the session manager and core shard lists to be kept in sync, and
add additional build complexity and maintenance costs.

Both of these options would appear identical from the perspective of the
session.

## Implementation

This change can be made in a single gerrit CL. As GN arguments have default
values, product definitions can add overrides to enable features they want at a
later time.

An incomplete yet functional implementation, demonstrating how this approach
works, can be found [here][wip-cl].

## Performance

As all of the work in this proposal happens at build-time, there will not be any
run-time performance impact.

## Security considerations

This proposal would result in components that should not be run on a product not
existing on that product. It will also enable us to provide components with the
capabilities they need for the product on which they are running, instead of
always giving components the union of capabilities they may need across
products.

This proposal would result in multiple possible variations of the CFv2 realm
structure, which would result in security audits and tools (such as scrutiny)
needing to be able to handle the new mechanics along with less of a shared base
between products.

## Privacy considerations

This proposal would result in components only existing on the products on which
they should run and never receiving an overly broad set of capabilities, both of
which have the potential to improve the system's privacy guarantees.

## Testing

Product owners will need to ensure that required components are included on
their products, and that the presence of these components is covered in testing.
While this is already true due to product-specific package sets, the core realm
variations proposed here add an additional point at which product definitions
could accidentally omit an important component. This results in a slightly
higher burden on product owners to ensure that test configurations remain
consistent with product configurations.

## Documentation

The exact GN arguments available to product owners to set will be documented at
the place they are defined in GN. Additionally, markdown documentation will be
maintained describing each manifest shard and the impact it will have on the
product's core realm.

[build-policy]: development/build/build_system/policies.md
[wip-cl]: https://fuchsia-review.googlesource.com/c/fuchsia/+/486644
[generated-file]: https://gn.googlesource.com/gn/+/master/docs/reference.md#func_generated_file
[cmc-merge]: /tools/cmc/build/cmc.gni
