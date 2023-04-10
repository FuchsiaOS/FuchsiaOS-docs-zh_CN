<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0199" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

Allow `zx_vmar_protect` to reduce permissions for child VMARs in the range.

## Motivation

It is not uncommon for software, such as component loaders, on Fuchsia to place
mappings in sub VMARs, but not retain and pass on the handles to those sub
VMARs. In such circumstances it is no longer possible to perform operations such
as `zx_vmar_protect` on these mappings, even using the root VMAR, which is
notionally the parent authority for the range.

`protect` cannot elevate permissions beyond what was allowed by the handles used
to create the mapping, and is typically used to provide additional error
checking in an application. For example an application might remove access
permissions to portions of their heap that are not in use, catching any
use-after-frees or other stray accesses.

For some use cases elevating permissions is not required and `protect` is used
strictly to reduce permissions. In these scenarios not being able to use
`protect` to reduce permissions if the mappings are in a child VMAR serves to
reduce the error detection that applications can perform.

## Stakeholders

_Facilitator:_

cpu@google.com

_Reviewers:_

travisg@google.com, rashaeqbal@google.com, nickcano@google.com

_Consulted:_

wez@google.com, palmer@google.com, mvanotti@google.com

_Socialization:_

This proposal was previously discussed with the Zircon team.

## Design

Introduce a new right, `ZX_RIGHT_OP_CHILDREN`, that controls whether operations
on an object are allowed to recurse into child objects. This proposal only gives
meaning to this right for VMARs, and will not be allowed for other objects. The
right will become part of the `ZX_DEFAULT_VMAR_RIGHTS`.

Definitions of existing operations will be changed to understand this right in
the following ways:

* `zx_vmar_protect`: Gains the ability to recursively apply to mappings in
subregions, with the limitation that subregion mappings can only have their
permission reduced. This is discussed further in the next section.
* `zx_vmar_unmap` and `zx_vmar_op_range`: Already recursive and will respect the
new right to limit this.
* `zx_vmar_destroy`: Logically is recursive and will require the right to
operate.

For all operations their API descriptions will be defined such that they return
`ZX_ERR_ACCESS_DENIED` when the `ZX_RIGHT_OP_CHILDREN` right is not present
where the range specified contains subregions.

The kernel VMAR code is already capable of recursing into child VMARs when
operating over a range and no new methods need to be designed for this as
existing enumeration tools can be used.

### Protect

`zx_vmar_protect` is the only operation gaining functionality in the presence of
`ZX_RIGHT_OP_CHILDREN` and this section explores how mapping permissions work
and how this will apply to `protect` when applied to child mappings.

#### Mapping permissions

A mapping has two notions of permission.

1. The intersection of the access rights of the VMAR and VMO handle used to
create the mapping. This represents the maximal permissions of the mapping.
2. The current operating permissions of the mapping that any accesses will be
checked against. These will always be equal or less than the maximal
permissions.

When first constructed the maximal permissions are inferred only from the
handle permissions, and the user has no mechanism to constrain this. The initial
operating permissions are set in the options to `zx_vmar_map`, and must be less
than or equal to the maximal permissions of the handles.

After construction the operating permissions may be changed at any time using
the `zx_vmar_protect` operation, provided the requested permissions do not
exceed the original maximal permissions.

The consequence of this is that even if the maximal permissions of the mapping
allow for reads, if the current permissions do not permit read then the access
is refused and an exception is generated. Other VMAR operations, such as
`zx_vmar_op_range`, act similar to `protect` in being gated by the maximal
permissions, and not the current permissions.

#### Hierarchy permissions

Unlike a mapping, a VMAR only has a single notion of permission, and it cannot
be changed. A VMAR is created using `zx_vmar_allocate` and it takes as input:

* Handle to parent VMAR
* Permissions for new VMAR

Here the requested permissions must, as expected, be equal or less than the
permissions on the handle to the parent VMAR.

The produced child VMAR handle has its handle permissions set to match the
permissions of the created VMAR, which match the requested permissions.

This produces two consequences:

1. As a VMAR hierarchy is traversed permissions can only be dropped, and never
increased.
2. A VMAR handle never has handle rights in excess of the VMAR permissions,
although it can have reduced permissions.

#### Implications

The `zx_vmar_protect` operation currently has three requirements imposed on the
requested permissions:

1. The VMAR handle invoked must have the rights requested.
2. The VMAR invoked by the innermost VMAR covering the range, sub VMARs will not
   be recursed into.
3. Every mapping in the range must have sufficient maximal permissions.

With this change the second requirement is changed to instead be: every mapping
in a sub-VMAR in the range must have current operating permissions equal or
greater than the right requested.

## Implementation

This can be completely implemented in a few small CLs that:

* Introduce the `ZX_RIGHT_OP_CHILDREN` into the kernel API.
* Add support to each VMAR operation for the right, updating documentation and
adding tests as required.

## Performance

No performance impacts are expected. Changing the implementation to support
child VMAR iteration could pessimise existing cases and the Zircon
micro-benchmarks will be used to check for this.

## Backwards compatibility

The `ZX_RIGHT_OP_CHILDREN` will be a default right and for existing operations
that already recursed, behavior will be unchanged. `protect` will gain the
ability to recurse by default, but it is not expected, outside of tests, that
users are relying on an error being generated in this case. Therefore having
`protect` succeed instead of failing should not break any existing users.

## Security considerations

The ability to place a mapping in a VMAR, protect that mapping and then throw
away the handle to the VMAR is presently relied upon to create mappings that
cannot then have their permissions altered. Even though the mapping has greater
maximal permissions, as there is no VMAR handle, the current permissions can
never be altered. Should it be possible to increase the permissions on these
mappings, then security properties would be violated.

Although this proposal allows for changing the permissions in sub VMARs, it only
permits a reduction, and not an increase in permissions. As a comparable example
it is already possible to effectively reduce a subregions permissions to nothing
by using the `zx_vmar_unmap` operation. Being able to selectively reduce the
permissions to something between nothing and current does not provide any
additional kinds of privilege.

Security conscious components can also remove the `ZX_RIGHT_OP_CHILDREN` from
any handles they have.

## Drawbacks, alternatives, and unknowns

### Pass all the VMAR handles

The primary alternative is to leave the kernel unchanged and change user space
to track and pass along all child VMAR handles. Although conceptually simple,
given this information exchange needs to happen on the component creation
boundary, defining or extending APIs to transfer this information would be a
non-trivial change.

Aside from the greater implementation complexity of this approach, it has
generally become accepted that needing to retain and track all child VMARs or
VMO handles in order to perform operations is an unreasonable burden for
userspace.
