# Pager

## NAME

pager - Mechanism for userspace paging

## SYNOPSIS

Pagers provide a mechanism for a userspace process to provide demand paging for VMOs.

## DESCRIPTION

A pager object allows a userspace pager service (typically a filesystem) to create VMOs that serve
as in-memory caches for external data. For a given VMO created by a pager object, the kernel
delivers page requests to an associated port. The pager service is then responsible for fulfilling
the requests by supplying the appropriate pages to the VMO.

The kernel does not do prefetching; it is the responsibility of the pager service to implement any
applicable prefetching.

It is possible for a single pager to simultaneously back multiple VMOs. Requests for the different
VMOs can be differentiated by the *key* parameter used when creating the VMO. It is also possible
for multiple independent pager objects to exist simultaneously.

Creating a pager is not a privileged operation. However, the default behavior of syscalls that
operate on VMOs is to fail if the operation would require blocking on IPC back to a userspace
process, so applications generally need to be aware of when they are operating on pager owned
VMOs. This means that services that provide pager owned VMOs to clients should be explicit about
doing so as part of their API. Whether or not accesses into a VMO may result in a pager request
can be determined by checking for the **ZX_INFO_VMO_PAGER_BACKED** flag returned by
[`zx_object_get_info()`] in `zx_info_vmo_t`.

TODO(stevensd): Writeback is not currently implemented. Update the documentation when it is.

## SEE ALSO

+ [vm_object](vm_object.md) - Virtual Memory Objects

## SYSCALLS

+ [pager_create](reference/syscalls/pager_create.md) - create a new pager object
+ [pager_create_vmo](reference/syscalls/pager_create_vmo.md) - create a vmo owned by a pager
+ [pager_detach_vmo](reference/syscalls/pager_detach_vmo.md) - detaches a pager from a vmo
+ [pager_supply_pages](reference/syscalls/pager_supply_pages.md) - supply pages into a pager owned vmo
+ [pager_op_range](reference/syscalls/pager_op_range.md) - performs a pager operation on the specified range

[`zx_object_get_info()`]: reference/syscalls/object_get_info.md
