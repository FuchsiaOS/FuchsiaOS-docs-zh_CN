{% set rfcid = "RFC-0013" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

It is currently only possible to create a CoW clone of a VMO if you have a handle to it.
This is not sufficient for zygote use cases,
which requires CoW cloning an entire address space,
including mappings for which all VMO handles have been closed.
This RFC proposes a new system call to address this gap.

## Motivation and problem statement

Chromium on Linux spawns renderers by forking them from a zygote process,
resulting in significant memory and CPU savings.
We would like to realize these savings on Fuchsia.

To do this, we need some way to implement an "address space clone" operation,
which when given:

- A process root VMAR
- Some handles that refer to VMOs mapped in the address space

will return:

- A new process with an address space populated by clones of the VMOs mapped in the input process
- For each handle referring to a VMO in the input process,
  a new handle referring to the corresponding clone of that VMO in the new process

By adding this system call, we can comfortably implement this in userspace.
This works by getting the address space layout with `ZX_INFO_PROCESS_MAPS`
and using `zx_vmar_create_vmo_child` to create a clone of each mapping,
except for any VMOs that there is a handle to,
which are handled using `zx_vmo_create_child`.

## Design

### `zx_vmar_create_vmo_child`

```
zx_status_t zx_vmar_create_vmo_child(zx_handle_t handle,
                                     uint32_t options,
                                     zx_vaddr_t addr,
                                     size_t size,
                                     zx_handle_t* out)
```

Creates a CoW clone of the pages mapped in the range `addr` to `addr`+`size`
in the VMAR referenced by `handle`.
The range must be a subrange of a single VMO mapping,
i.e. it may not span two mappings or include any unmapped pages.
The result is a new VMO that is a child of the VMO mapped in the range.

Allowed options are
`ZX_VMO_CHILD_SNAPSHOT`, `ZX_VMO_CHILD_SNAPSHOT_AT_LEAST_ON_WRITE`, and `ZX_VMO_CHILD_NO_WRITE`.
They are interpreted the same way as in `zx_vmo_create_child`.
(`ZX_VMO_CHILD_SLICE` is not currently allowed due to security concerns:
special care would have to be taken to ensure that
the ability to modify the parent VMO doesn't break any security boundaries.)

Returns `ZX_ERR_INVALID_ARGS` if:

- the range `addr` to `addr`+`size` isn't a subrange of a single VMO mapping
- `addr` or `size` is not page-aligned

Returns `ZX_ERR_ACCESS_DENIED` if:

- the mapping was done using a new flag `ZX_VM_CANNOT_CREATE_VMO_CHILD`,
  or the vmar options include this flag
- `handle` doesn't have `ZX_RIGHT_INSPECT`

Returns `ZX_ERR_BAD_HANDLE` if `handle` isn't a valid handle,
`ZX_ERR_WRONG_TYPE` if `handle` isn't a VMAR,
and `ZX_ERR_BAD_STATE` if the VMAR is destroyed.
Can also return any error returned by `zx_vmo_create_child` if the documented reason applies.

The returned VMO handle will have `ZX_DEFAULT_VMO_RIGHTS`, with the following changes:

- `ZX_RIGHT_WRITE` will be removed if `ZX_VMO_CHILD_NO_WRITE` was specified.
- `ZX_RIGHT_EXECUTE` will be added if the mapping was executable and `ZX_VMO_CHILD_NO_WRITE` was not specified.
  This means it is be possible to create executable clones of executable mappings,
  but they must be read-only.

### `ZX_VM_CANNOT_CREATE_VMO_CHILD`

`ZX_VM_CANNOT_CREATE_VMO_CHILD` is a new `zx_vm_options_t` flag
which can be specified for a VMO mapping or a VMAR.
This allows code that maps VMOs or creates VMARs
to disallow `zx_vmar_create_vmo_child` on them.

## Implementation

Adding a syscall is not a complex change; it can be done in one CL.
Implementation of the clone algorithm is beyond the scope of this RFC.

## Performance

It's not worth benchmarking this syscall on its own,
since it's only used in the context of a zygote implementation.
Instead we should measure the overall performance of the zygote implementation.

## Security considerations

Creating VMO children using VMAR references instead of handles is not currently possible,
but this RFC would make it possible.
It's a bit like granting a new capability out of thin air.

In this case it doesn't seem that risky
because the end result is the same as making a new VMO and copy the data over
(but with less memory usage), which was possible before.
If anything needs to disallow this,
this RFC proposes a `zx_vm_options_t` flag (`ZX_VM_CANNOT_CREATE_VMO_CHILD`)
that can be used.

## Testing

The syscall will be unit tested.
The tests for a future implementation of address space cloning
will also serve as integration tests for the syscall.

## Documentation

This RFC is a starting point for the documentation of `zx_vmar_create_vmo_child`.

## Drawbacks, alternatives, and unknowns

### What other strategies might solve the same problem?

We could add a system call to do the entire address space clone operation.
This would be a very complicated system call, in both interface and implementation.
It's better to push the complexity into userspace.

We could add a system call that allows minting a VMO handle from a mapping.
This allows what is essentially a userspace implementation of `zx_vmar_create_vmo_child`,
by first creating a handle from the mapping and then calling `zx_vmo_create_child`.
But the Zircon API generally only allows minting handles to newly created objects,
in order to make the interface easier to reason about.
This syscall would have to create handles out of thin air.

There are various tricks that might be used to avoid closing any VMO handles,
such as modifying all code that creates and maps VMOs to avoid closing them,
or intercepting `zx_handle_close`.
However, this is not sufficient to implement address space cloning,
since the loader service maps some VMOs into the process before it starts running,
and this doesn't help you get handles to those.
We could modify the loader service to pass the necessary handles to the process,
but that would be more work than adding this syscall.

## Prior art and references

https://chromium.googlesource.com/chromium/src/+/master/docs/linux/zygote.md
http://neugierig.org/software/chromium/notes/2011/08/zygote.html

