# zx_socket_set_disposition

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Set disposition of writes.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_socket_set_disposition(zx_handle_t handle,
                                      uint32_t disposition,
                                      uint32_t disposition_peer);
```

## DESCRIPTION

`zx_socket_set_disposition` sets the disposition of
[`zx_socket_write()`] calls for a socket handle and its peer.

Valid disposition flags that can be used:

**ZX_SOCKET_DISPOSITION_WRITE_DISABLED** - Disable writes for the specified
socket endpoint. Once set, writes to the specified socket endpoint will fail
with **ZX_ERR_BAD_STATE**. Reads from the specified socket endpoint will
succeed until all data buffered in the specified socket endpoint is consumed,
and fail with **ZX_ERR_BAD_STATE** thereafter.

**ZX_SOCKET_DISPOSITION_WRITE_ENABLED** - Enable writes for the specified
socket endpoint.  Once set, writes to and reads from the specified socket
endpoint will behave as specified in [`zx_socket_write()`] and
[`zx_socket_read()`], respectively.

It is invalid to specify **ZX_SOCKET_DISPOSITION_WRITE_ENABLED** on a socket
endpoint that has buffered data; doing so will result in
`zx_socket_set_disposition` returning **ZX_ERR_BAD_STATE** and no action being
taken.

It is invalid to specify both **ZX_SOCKET_DISPOSITION_WRITE_DISABLED** and
**ZX_SOCKET_DISPOSITION_WRITE_ENABLED** in *disposition* or *disposition_peer*;
doing so will result in `zx_socket_set_disposition` returning
**ZX_ERR_INVALID_ARGS** and no action being taken.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_SOCKET** and have **ZX_RIGHT_MANAGE_SOCKET**.

## RETURN VALUE

`zx_socket_set_disposition()` returns **ZX_OK** on success.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_BAD_STATE**  *disposition* or *disposition_peer* contains
**ZX_SOCKET_DISPOSITION_WRITE_ENABLED** and *handle* refers to a socket with
buffered data on the specified socket endpoint.

**ZX_ERR_WRONG_TYPE**  *handle* is not a socket handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_RIGHT_MANAGE_SOCKET**.

**ZX_ERR_INVALID_ARGS**  *disposition* or *disposition_peer* contains flags
outside of the ones listed above or an invalid combination of flags.

## SEE ALSO

 - [`zx_socket_create()`]
 - [`zx_socket_read()`]
 - [`zx_socket_write()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_socket_create()`]: socket_create.md
[`zx_socket_read()`]: socket_read.md
[`zx_socket_write()`]: socket_write.md
