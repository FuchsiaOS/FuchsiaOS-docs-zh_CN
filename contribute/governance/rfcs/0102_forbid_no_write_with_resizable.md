<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0102" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC proposes to make it an error to pass the combination of both
`ZX_VMO_CHILD_NO_WRITE` and `ZX_VMO_CHILD_RESIZABLE` to `zx_vmo_create_child`.

## Motivation

Creating a child VMO, using `zx_vmo_create_child`, that has the
`ZX_VMO_CHILD_NO_WRITE` flag set explicitly removes `ZX_RIGHT_WRITE` from the
produced handle. As this is the only, at this point, handle to the newly created
VMO, and its rights can never be upgraded to include `ZX_RIGHT_WRITE`, no
additional handles to the VMO with `ZX_RIGHT_WRITE` can ever be produced.

The resize operation, `zx_vmo_set_size`, that can be performed on VMO children
created with `ZX_VMO_CHILD_RESIZABLE` requires `ZX_RIGHT_WRITE`, as it can
result in modifying VMO data.

The combination of passing both of these flags to `zx_vmo_create_child` results
in the successful creation of a VMO, with a handle returned that will error if
`zx_vmo_set_size` is called. It is highly confusing from a user point of view to
successfully create an object with `ZX_VMO_CHILD_RESZIABLE`, only to have the
resize operation fail. Not only does it fail on the immediately produced handle,
but there is no way for a handle to this object to ever be produced that could
be resized, since handle rights cannot be upgraded.

## Design

The `zx_vmo_create_child` syscall will check at the start if both the
`ZX_VMO_CHILD_NO_WRITE` and `ZX_VMO_CHILD_RESIZABLE` flags are set, and if so
immediately return `ZX_ERR_INVALID_ARGS`.

## Implementation

All changes can be implemented in a single CL.

## Performance

Not relevant.

## Backwards Compatibility

No known existing usages of this flag combination.

## Security considerations

None.

## Privacy considerations

None.

## Testing

Unit test added to validate restriction.

## Documentation

The API documentation of `zx_vmo_create_child` would need updating.

## Drawbacks, alternatives, and unknowns

### Dedicated resize right

The `ZX_RIGHT_WRITE` is used to gate the `zx_vmo_set_size` operation as resizing
modifies the VMO via truncation or zero extension. This is a very limited and
predicted set of modifications and so could have its own dedicated right to
separate it from arbitrary modifications.

Even with such a specialized right, the `ZX_VMO_CHILD_NO_WRITE` would almost
certainly want to strip that permission as well since the purpose of
`ZX_VMO_CHILD_NO_WRITE` is to prevent code modifications. The combination of
truncation and zero extension could, especially on variable instruction length
architectures like x86, result in the formulation of malicious instructions,
even if limited to being done on page boundaries.

### Do nothing

The alternative is to leave the syscall how it is, and possibly document the
fact that on success a produced handle may not be resizable, even if it was
requested.
