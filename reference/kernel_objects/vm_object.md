# Virtual Memory Object

## NAME

vm\_object - Virtual memory containers

## SYNOPSIS

A Virtual Memory Object (VMO) represents a contiguous region of virtual memory
that may be mapped into multiple address spaces.

## DESCRIPTION

VMOs are used in the kernel and userspace to represent both paged and physical memory.
They are the standard method of sharing memory between processes, as well as between the kernel and
userspace.

VMOs are created with [`zx_vmo_create()`] and basic I/O can be
performed on them with [`zx_vmo_read()`] and [`zx_vmo_write()`].
A VMO's size may be set using [`zx_vmo_set_size()`].
Conversely, [`zx_vmo_get_size()`] will retrieve a VMO's current size.

The size of a VMO will be rounded up to the next page size boundary by the kernel.

Pages are committed (allocated) for VMOs on demand through [`zx_vmo_read()`], [`zx_vmo_write()`], or by writing to a mapping of the VMO created using [`zx_vmar_map()`]. Pages can be committed and decommitted from a VMO manually by calling
[`zx_vmo_op_range()`] with the **ZX_VMO_OP_COMMIT** and **ZX_VMO_OP_DECOMMIT**
operations, but this should be considered a low level operation. [`zx_vmo_op_range()`] can also be used for cache and locking operations against pages a VMO holds.

Processes with special purpose use cases involving cache policy can use
[`zx_vmo_set_cache_policy()`] to change the policy of a given VMO.
This use case typically applies to device drivers.

## SYSCALLS

 - [`zx_vmo_create()`] - create a new vmo
 - [`zx_vmo_create_child()`] - create a new child vmo
 - [`zx_vmo_create_physical()`] - create a new physical vmo
 - [`zx_vmo_get_size()`] - obtain the size of a vmo
 - [`zx_vmo_op_range()`] - perform an operation on a range of a vmo
 - [`zx_vmo_read()`] - read from a vmo
 - [`zx_vmo_replace_as_executable()`] - make an executable version of a vmo
 - [`zx_vmo_set_cache_policy()`] - set the caching policy for pages held by a vmo
 - [`zx_vmo_set_size()`] - adjust the size of a vmo
 - [`zx_vmo_write()`] - write to a vmo

<br>

 - [`zx_vmar_map()`] - map a VMO into a process
 - [`zx_vmar_unmap()`] - unmap memory from a process

[`zx_vmar_map()`]: reference/syscalls/vmar_map.md
[`zx_vmar_unmap()`]: reference/syscalls/vmar_unmap.md
[`zx_vmo_create()`]: reference/syscalls/vmo_create.md
[`zx_vmo_create_child()`]: reference/syscalls/vmo_create_child.md
[`zx_vmo_create_physical()`]: reference/syscalls/vmo_create_physical.md
[`zx_vmo_get_size()`]: reference/syscalls/vmo_get_size.md
[`zx_vmo_op_range()`]: reference/syscalls/vmo_op_range.md
[`zx_vmo_read()`]: reference/syscalls/vmo_read.md
[`zx_vmo_replace_as_executable()`]: reference/syscalls/vmo_replace_as_executable.md
[`zx_vmo_set_cache_policy()`]: reference/syscalls/vmo_set_cache_policy.md
[`zx_vmo_set_size()`]: reference/syscalls/vmo_set_size.md
[`zx_vmo_write()`]: reference/syscalls/vmo_write.md
