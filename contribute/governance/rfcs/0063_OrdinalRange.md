{% set rfcid = "RFC-0063" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-010.

## Rejection rationale

We've had a number of discussions about how we want interfaces to evolve,
and be inheritable.
For instance, introducing ordinals on interfaces, which along with method
ordinals would uniquely determine a method being invoked.
As such, we're rejecting the `[OrdinalRange]` proposal.

However, we recognize the need to control inheritance, since the pattern
we have today can be problematic: unbeknownst to a library author,
defined interfaces could be inherited, such that introducing new methods
would irreparably break inheritors.

As a result, we're going to introduce, on a temporary basis (until we have
a better solution), the requirement that any interface needing to be
inherited must be annotated with `[FragileBase]`.

### Relation to other RFCs

This RFC was superseded by:

* [RFC-0020: Interface ordinal hashing](0020_interface_ordinal_hashing.md)

## Summary {#summary}

Add an optional `[OrdinalRange]` attribute to interfaces.
If present, ordinals defined by the interface must fall within the
declared range.
An interface cannot be inherited from unless it declares an
`[OrdinalRange]` and the subinterface cannot use any of the ordinals
in the ranges claimed by its superinterfaces.

## Motivation {#motivation}

In FIDL, an interface can inherit from another interface.
The FIDL compiler prevents the two interfaces from assigning different
semantics to the same ordinal, but there is no mechanism for the super
interface to reserve ordinals for future use.
As currently defined, adding an ordinal to any interface could potentially
break other (unseen) interfaces that inherit from that interface and assign
that ordinal.

The [FIDL style rubric][fidl-style] suggests working around this
issue by documented reserved ordinal ranges in comments (at least for
interfaces that expect to be inherited from).
This FTP proposes putting those reservations in an `[OrdinalRange]` attribute
on the interface instead.

## Design {#design}

This change affects only the FIDL source language.
It has no effect on the wire format or any of the generated bindings.
The implementation can take place entirely in the FIDL frontend compiler.

1. Introduce an optional `[OrdinalRange]` attribute for interfaces.
   When present, the value of this attribute must match the following grammar:
    1. **`NUMERIC-LITERAL`**, "**`-`**", **`NUMERIC-LITERAL`**
2. When present, all the ordinals defined by the interface must be in the
   specified range (inclusive).
3. An interface cannot inherit from another interface unless the second
   interface has the `[OrdinalRange]` attribute.
4. All the super interfaces of an interface must have `[OrdinalRange]`
   attributes that define disjoint ranges.
5. An interface cannot define an ordinal that belongs to the
   `[OrdinalRange]` of any of its super interfaces.

If any of these invariants is violated, the FIDL frontend must generate an
error.

Replace the passage in the [FIDL style rubric][fidl-style] that
recommends using comments for this purpose with a passage that requires
using the `[OrdinalRange]` attribute for this purpose.

## Documentation and examples {#documentation-and-examples}

This feature will be documented (and an example provided) in the [FIDL style
rubric][fidl-style].

## Backwards compatibility {#backwards-compatibility}

This feature is not backwards compatible because existing uses of
inheritance will fail to compile until we add `[OrdinalRange]` attributes
to the superinterfaces.
However, we use inheritance rarely and updating all the superinterfaces
will be easy.

## Performance {#performance}

This proposal has no impact on performance.

## Security {#security}

This proposal has no impact on security.

## Testing {#testing}

The `fuchsia.io` library will provide a positive compilation test.

## Drawbacks, alternatives, and unknowns {#drawbacks-alternatives-and-unknowns}

The main drawback of implementing this proposal is extra ceremony for
creating inheritance relationships between interfaces.
However, that ceremony already exists in the form of comments about
reserved ordinal ranges.

An alternative is to continue to use comments to informally reserve
ordinal ranges.

## Prior art and references {#prior-art-and-references}

We assume other similar systems do something similar, but haven't
researched it.

<!-- xrefs -->
[fidl-style]: /docs/development/languages/fidl/guides/style.md
