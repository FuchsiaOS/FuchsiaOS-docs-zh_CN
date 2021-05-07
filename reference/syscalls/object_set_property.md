# zx_object_set_property

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Set various properties of various kernel objects.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_object_set_property(zx_handle_t handle,
                                   uint32_t property,
                                   const void* value,
                                   size_t value_size);
```

## DESCRIPTION

`zx_object_set_property()` modifies the value of a kernel object's property.
Setting a property requires **ZX_RIGHT_SET_PROPERTY** rights on the handle.

See [`zx_object_get_property()`] for a full description.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must have **ZX_RIGHT_SET_PROPERTY**.

If *property* is **ZX_PROP_PROCESS_DEBUG_ADDR**, *handle* must be of type **ZX_OBJ_TYPE_PROCESS**.

If *property* is **ZX_PROP_PROCESS_BREAK_ON_LOAD**, *handle* must be of type **ZX_OBJ_TYPE_PROCESS**.

If *property* is **ZX_PROP_SOCKET_RX_THRESHOLD**, *handle* must be of type **ZX_OBJ_TYPE_SOCKET**.

If *property* is **ZX_PROP_SOCKET_TX_THRESHOLD**, *handle* must be of type **ZX_OBJ_TYPE_SOCKET**.

If *property* is **ZX_PROP_JOB_KILL_ON_OOM**, *handle* must be of type **ZX_OBJ_TYPE_JOB**.

## SEE ALSO

 - [`zx_object_get_property()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_object_get_property()`]: object_get_property.md
