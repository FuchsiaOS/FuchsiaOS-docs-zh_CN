# zx_system_get_num_cpus

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Get number of logical processors on the system.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

uint32_t zx_system_get_num_cpus(void);
```

## DESCRIPTION

`zx_system_get_num_cpus()` returns the number of CPUs (logical processors)
that exist on the system currently running.  This number cannot change
during a run of the system, only at boot time.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_system_get_num_cpus()` returns the number of CPUs.

## ERRORS

`zx_system_get_num_cpus()` cannot fail.

## NOTES

## SEE ALSO

 - [`zx_system_get_physmem()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_system_get_physmem()`]: system_get_physmem.md
