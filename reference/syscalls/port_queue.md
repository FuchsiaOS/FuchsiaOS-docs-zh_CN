# zx_port_queue

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Queue a packet to a port.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>
#include <zircon/syscalls/port.h>

zx_status_t zx_port_queue(zx_handle_t handle, const zx_port_packet_t* packet);
```

## DESCRIPTION

`zx_port_queue()` queues a user *packet* to the port specified by *handle*.

User packets are drained by [`zx_port_wait()`]. Failure to drain packets in a
timely fashion can cause excessive kernel memory to be used, which might generate
an exception. See [ipc limits](/docs/concepts/kernel/ipc_limits.md) for details.

```
typedef struct zx_port_packet {
    uint64_t key;
    uint32_t type;
    zx_status_t status;
    union {
        zx_packet_user_t user;
        zx_packet_signal_t signal;
    };
} zx_port_packet_t;

```

In *packet* *type* should be **ZX_PKT_TYPE_USER** and only the **user**
union element is considered valid:

```
typedef union zx_packet_user {
    uint64_t u64[4];
    uint32_t u32[8];
    uint16_t u16[16];
    uint8_t   c8[32];
} zx_packet_user_t;

```

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_PORT** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_port_queue()` returns **ZX_OK** on successful queue of a packet.

## ERRORS

**ZX_ERR_BAD_HANDLE** *handle* isn't a valid handle

**ZX_ERR_INVALID_ARGS** *packet* is an invalid pointer.

**ZX_ERR_WRONG_TYPE** *handle* is not a port handle.

**ZX_ERR_ACCESS_DENIED** *handle* does not have **ZX_RIGHT_WRITE**.

## SEE ALSO

 - [`zx_port_create()`]
 - [`zx_port_wait()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_port_create()`]: port_create.md
[`zx_port_wait()`]: port_wait.md
