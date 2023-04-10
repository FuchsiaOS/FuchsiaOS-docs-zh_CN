<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0204" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

The objective of this document is to introduce a new VMO child type meant for
explicitly tracking references to a VMO.

## Motivation

Filesystems that represent files in memory using VMOs need to track when it is
safe to garbage collect these  VMOs. This might include activities like flushing
out modified VMO contents to disk, detaching the VMO from a userpager service,
tearing down the VMO itself, thereby freeing its pages, and also releasing any
internal metadata the filesystem might be using to track the open file.

There are several ways that filesystems can share file VMOs with clients. They
can vend out duplicate handles to the VMO, which clients can use to read from
and write to pages in the VMO directly. Clients can also create a VM mapping
using the VMO handle with `zx_vmar_map()`, drop the VMO handle, and then
continue accessing the VMO's pages through addresses in the VM mapping.
Filesystems can also create streams using the VMO with `zx_stream_create()` and
hand out stream handles to clients. A unified reference counting mechanism is
required to track all these various kinds of references.

`Blobfs`, which is the immutable filesystem that serves executables on Fuchsia,
attempts to solve this reference counting problem by creating Copy-on-Write
(`ZX_VMO_CHILD_SNAPSHOT_AT_LEAST_ON_WRITE`) clones of the file VMO. The only
handle to the file VMO is held by `blobfs`, which is used to create CoW clones
each time a file is requested by a client. It is a handle to the clone that
`blobfs` hands out, not the original VMO. Reference counting then maps to
tracking the number of outstanding clones, and `blobfs` can garbage collect when
the clone count drops to zero, by waiting on the `ZX_VMO_ZERO_CHILDREN` signal
to become active on the original VMO. This works with VM mappings as well
because a VMO will internally be held alive by the mapping even if the VMO
handle is dropped.  `Blobfs` does not hand out streams today.

This strategy of reference counting using CoW clones works fine for an immutable
filesystem, but breaks when applied to a mutable one like `fxfs` or `minfs`. In
a mutable filesystem, any changes that are made to a client's reference to a
file should be reflected back in the original file. In other words, any writes
performed in the CoW clone should make it back to the parent VMO. This violates
CoW semantics because the moment a page is written in the CoW clone the kernel
creates a copy of the parent's page in the clone; the clone's changes are never
visible in the parent. There is a similar problem with snapshot clones
(`ZX_VMO_CHILD_SNAPSHOT`) as well. Updates in the child are never visible in the
parent and vice versa.

This leaves us with the `ZX_VMO_CHILD_SLICE` VMO child type. On the surface it
might appear that slices are what we need here, since they provide a direct
window to the parent's pages. However, slices have the limitation that they
cannot be created against VMOs that are resizable, and cannot be resized
themselves. A mutable filesystem would want to allow clients to resize its file
VMOs. Therefore all three VMO child types available today fall short in a
reference counting scheme using VMO children.

## Stakeholders

_Facilitator:_

- cpu@google.com

_Reviewers:_

- adanis@google.com, godtamit@google.com

_Consulted:_

- brettw@google.com, cdrllrd@google.com, csuter@google.com, travisg@google.com

_Socialization:_

The design was socialized with the Zircon team and discussed in the Kernel
Evolution Working Group.

## Design

`ZX_VMO_CHILD_REFERENCE` is a new type of VMO child intended to track
outstanding references to a VMO. The intent is that each time the filesystem
needs a new reference to a file VMO for a client, it will create a reference
child and treat it as the original VMO. Streams would be created against a
reference child instead of the original VMO. Similarly VM mappings would be
created for reference children. This gives the kernel a way to track the number
of references to a VMO by using the existing VMO child tracking mechanism.
Filesystems can use the `ZX_VMO_ZERO_CHILDREN` signal to determine when no
references remain and it is safe to destroy the VMO.

Note that reusing the `ZX_VMO_ZERO_CHILDREN` signal means that filesystems would
not be able to tear down the original VMO as long as _any_ type of child exists,
not just a reference child. This, however, is fine and allows filesystems to mix
different child types per their requirements. For example, `blobfs` can continue
to use CoW clones for data segments where writes need to be restricted to the
client, and use reference children (created read-only with
`ZX_VMO_CHILD_NO_WRITE`) for text segments. It would still be able to use the
`ZX_VMO_ZERO_CHILDREN` signal for a unified reference counting scheme.

### Reference child creation

The `zx_vmo_create_child()` syscall will support a new option flag,
`ZX_VMO_CHILD_REFERENCE`. Since this is conceptually a reference to the parent
VMO, the `offset` and `length` parameters are not meaningful, and both will be
required to be set to `0`. A reference child will always refer to the entirety
of the parent VMO. An alternative would be to have the caller specify `offset`
as `0` and `length` as the current size of the VMO, however this places an undue
burden on the caller of always being aware of the parent VMO's size, which might
require a `zx_vmo_get_size()` call. In scenarios where filesystem clients are
constantly changing the VMO size (for example, by appending to the file), the
filesystem might be required to make several `zx_vmo_get_size()` calls until it
sees a stable size, which is cumbersome.

The same rules governing the `ZX_VMO_ZERO_CHILDREN` signal will apply to
reference children as existing child types. The creation of a reference child
will deactivate the signal (if it was previously active), and the destruction of
the last child will activate the signal.

### Resizability

`ZX_VMO_CHILD_RESIZABLE` will only be permitted if the parent VMO was created
with `ZX_VMO_RESIZABLE`. It is possible to create a non-resizable reference or a
resizable reference of a resizable VMO. However, it is not possible to create a
resizable reference of a non-resizable VMO; a reference child can simply be
thought of as a reference to the parent VMO, and if the parent VMO is not
resizable, there would exist no way for a reference child to be resized.

References will inherit the `ZX_VMO_RESIZABLE` option from the parent.
Resizability through the reference handle itself will be determined by a new
`ZX_RIGHT_RESIZE` right on the handle, which will be added only if the reference
was created with `ZX_VMO_CHILD_RESIZABLE`. A non-resizable reference can still
get resized by the parent; the non-resizability here is only restricting the
ability to directly resize through the reference. Separating the
`ZX_RIGHT_RESIZE` right from the `ZX_VMO_RESIZABLE` option flag captures this
distinction.

Introducing a new `ZX_RIGHT_RESIZE` would require integrating it into all
existing VMO APIs where applicable. This can be done by adding it to the default
set of rights on a VMO handle if it is created with the `ZX_VMO_RESIZABLE` flag,
or the `ZX_VMO_CHILD_RESIZABLE` in the case of VMO children.

All VM mappings created for references of a resizable VMO will require the
`ZX_VM_ALLOW_FAULTS` flag, regardless of whether the reference handle itself
has `ZX_RIGHT_RESIZE`.

### Difference from VMO slices

A lot of the functionality provided by reference children overlaps with
`ZX_VMO_CHILD_SLICE`. There are a couple of key differences:

- Slices can provide a window to a sub-range in the VMO. References always span
  the entire VMO.
- Slices cannot be created for resizable VMOs and cannot be resizable
  themselves.  References can be created for resizable VMOs and can be resizable
  themselves (if the parent was resizable).

### Supporting VMO types

Reference children can be created for all VMOs that were created with
`zx_vmo_create()` or `zx_pager_create_vmo()` and descendants of such VMOs. This
excludes VMOs created with `zx_vmo_create_contiguous()` and
`zx_vmo_create_physical()`, however, it does not preclude those use cases
entirely. Both contiguous and physical VMOs are not resizable, and so the user
would be able to create slices over the entire VMO instead to obtain equivalent
behavior.

### Interaction with VMO operations

All VMO operations on references will simply be forwarded to the parent. For
example:

- VMO reads and writes will function as if performed directly on the parent VMO.
- Commits and decommits will be forwarded to the parent as well.
- VM mappings when created against reference children will map corresponding
  pages in the parent.

### Creating children of references

The ability to create children of references will be controlled by the same set
of rules as child creation on the parent VMO. For example, if the parent VMO is
a pager-backed VMO and hence does not support `ZX_VMO_CHILD_SNAPSHOT`, its
reference child will not support `ZX_VMO_CHILD_SNAPSHOT` either.

When a child is created against a reference, the child's `parent_koid` will
point to the reference, even though the child can conceptually be thought of as
a child of the original VMO. The intent is to accurately represent the creation
chain of a descendant VMO; this behavior is in line with existing behavior of
child creation in the case of nested slices.

### Page attribution

Since all pages in the reference refer to pages in the parent VMO, the parent
will continue being attributed for all the pages, as reported by
`committed_bytes` (`zx_object_get_info()`). A reference does not directly hold
any pages and will always have a page attribution count of zero. This is in line
with the existing model for VMO page attribution where each page is attributed
to exactly one VMO.

### Resizing references

Resizing a reference will resize the parent VMO. Similarly, a resize of the
parent VMO will be reflected by the reference. References will also follow the
content size of the parent VMO, and be able to access and manipulate it similar
to the VMO size.

### Kernel changes

These are the broad changes that will be required in the kernel implementation
of VMOs.

- A reference child will point to the same page container (`VmCowPages`) as its
  parent. This has the desired effect of operations on the reference being
  forwarded to the parent, as they will both be performed against the same
  internal object.
- References will be tracked in the children list of the parent, allowing the
  use of the `ZX_VMO_ZERO_CHILDREN` signal on the parent.
- The parent VMO will also maintain a new list of all reference children. This
  is required for certain operations like propagating updates to VM mappings,
  which are tracked outside of the page container.
- References will also share the same `ContentSizeManager` as the parent in
  order to correctly update content sizes, which are used for reads and writes
  on streams created against the VMO.
- References will not hold the parent VMO alive. Once the parent goes away,
  references can continue to access the shared page container, which will have
  been kept alive since the references still point to it. The reference children
  will be rehomed in the grandparent (if any), as is the case for existing child
  types. The parent's reference list will be moved to one of the references so
  that VM mapping updates continue working correctly. All this ensures user
  observed behavior that is consistent with existing behavior in the case of
  slices today.

## Implementation

References can be implemented in a few small CLs that add support for the new
child type in VMO kernel internals, and then expose the new type flag through
the `zx_vmo_create_child()` syscall.

The new `ZX_RIGHT_RESIZE` will be added to the default set of rights for
resizable VMOs at creation time. So enforcement of the right in
`zx_vmo_set_size()` should not break existing users of resizable VMOs. We can
start simple with requiring both `ZX_RIGHT_WRITE` and `ZX_RIGHT_RESIZE`. An
audit can then be performed to find existing cases where `ZX_RIGHT_WRITE` is
being removed to prevent resizing, and have those cases remove `ZX_RIGHT_RESIZE`
instead, or in addition to `ZX_RIGHT_WRITE`. In the future, `zx_vmo_set_size()`
can check only for `ZX_RIGHT_RESIZE`.

## Performance

Operations on a reference occur as if performed directly on the parent VMO. This
is facilitated by sharing the same internal page container in the kernel. So
using references instead of the original VMO should not result in any observable
performance impact for most operations. VM microbenchmarks will be written to
validate this.

## Security considerations

All operations that can be performed on the reference child can be performed on
the parent VMO itself, a handle to which is required to create the reference.
References are simply going to replace certain usages of the original VMO, so it
would have been possible to perform these operations without references too.

## Testing

Kernel unit tests and core tests will be added for reference children.

## Documentation

The `zx_vmo_create_child()` syscall documentation will be updated with
`ZX_VMO_CHILD_REFERENCE`. Documentation for other VMO syscalls will need to be
updated to include the `ZX_RIGHT_RESIZE` where applicable.

## Drawbacks, alternatives, and unknowns

### Counting handles

An alternative would be to introduce a generic handle counting scheme which
activates a signal on the object when only one handle to the object remains.
However, VMO handles are not an accurate representation of the number of
outstanding references. It is possible to hold references to the VMO through VM
mappings and streams without having to hold on to the VMO handle that was used
to create them. We would also need to account for VM mapping, streams, and any
new object in the future that might hold a reference to a VMO. This quickly
becomes complicated and does not scale. Moreover, handle counts are inherently
racy and it is not advisable to use them for purposes besides debugging.

The reference child approach captures the relationship between the filesystem
and its clients better, which is in fact hierarchical. The filesystem's
reference to the VMO can be viewed as the primary one, and all the other
references it hands out to clients are secondary. The parent-child relationship
represents this well. On the other hand, handles are symmetric; in this case not
only do we care that only one reference remains, we also want that one remaining
reference to be the one that the filesystem holds.

### Reference tokens

We could mint a new reference token object which the filesystem could hand out
in addition to the VMO or stream. This could either be a new object that the
kernel provides or something that the filesystem implements. However, this would
require changing the filesystem API surface to support passing around a new
reference token which might not be feasible. Reference children can be used
interchangeably with VMOs, which makes supporting them a lot easier while
retaining the existing filesystem API. Having an external notion of reference
counting outside of the VMO is also bug-prone as it is possible to accidentally
drop the token.

### Resizing slices

The only major way reference children differ from slices is the ability to
resize. So an alternative could be to define the semantics of resize in the
context of slices, insteading of creating a new VMO child type. Slices can span
sub-ranges in the parent however, which makes reasoning about resizing a lot
harder.

For example, consider the case where a slice is resized to a size larger than
its creation size. It would be surprising if the slice would now uncover parent
pages that were not visible to it before. On the other hand, a slice does not
own any pages directly, so it would be awkward if it were to fork zero pages in
the extended range. There is a similar problem if the parent is resized up to a
larger size. Since the motivating use case requires resizes to propagate between
parent and child, this would require resizing any slices as well, which again
could be surprising.

### Resize right

VMO resizability is controlled by `ZX_RIGHT_WRITE` today. Having a separate
`ZX_RIGHT_RESIZE` affords tighter enforcement opportunities in the future. For
example, it can allow users to create writable non-resizable VMO handles to
share with a client. It is not possible to create such a VMO handle today; a
writable VMO handle is also resizable (if the VMO was created resizable).

## Prior art and references

[`zx_vmo_create_child()`](/reference/syscalls/vmo_create_child.md)

[`zx_vmar_map()`](/reference/syscalls/vmar_map.md)

[`zx_stream_create()`](/reference/syscalls/stream_create.md)
