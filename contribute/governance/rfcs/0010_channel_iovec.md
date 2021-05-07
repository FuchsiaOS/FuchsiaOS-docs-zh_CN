{% set rfcid = "RFC-0010" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

This RFC introduces a new mode to `zx_channel_write`, `zx_channel_write_etc`,
`zx_channel_call` and `zx_channel_call_etc` that copies input data from
multiple memory regions rather than from a single contiguous buffer. This
improves performance for certain users / clients by allowing message data to be
directly copied from multiple userspace objects without an intermediate
allocation, copy and layout step. This is accomplished by updating existing
syscalls to take an array of `zx_channel_iovec_t` memory region descriptors
when an option is specified.

## Motivation

The main motivation for this proposal is performance.

For non-linearized domain objects, FIDL bindings currently need to (1) allocate
a buffer and (2) copy objects into the buffer in a standard layout. After these
steps, the buffer is again copied into the kernel. `zx_channel_iovec_t` allows
the objects to be directly copied into the kernel. In addition FIDL message
data no longer needs to be laid out in a standard order -- only the
`zx_channel_iovec_t` array must reflect the needed order.

## Design

zx_channel_write currently has the following signature:

```
zx_status_t zx_channel_write(zx_handle_t handle,
                             uint32_t options,
                             const void* bytes,
                             uint32_t num_bytes,
                             const zx_handle_t* handles,
                             uint32_t num_handles);
```

The input data is a contiguous byte array pointed to by `bytes`.
In `zx_channel_write_etc`, `zx_channel_call` and `zx_channel_call_etc`, there
are analogous arrays. The fact that these arrays must be contiguous leads to
overhead. In particular, for FIDL messages with out-of-line components, the
FIDL encoder must allocate a buffer and relocate data into it which can be
expensive.

`zx_channel_iovec_t` provides an alternative path. `zx_channel_write`,
`zx_channel_write_etc`, `zx_channel_call` and `zx_channel_call_etc`
instead receive a list of locations and sizes of objects and
copying happens within the kernel, avoiding additional duplication and
allocation.

`zx_channel_iovec_t` is defined in C++ as the following:

```
typedef struct zx_channel_iovec {
  void* buffer;            // User-space bytes.
  uint32_t capacity;       // Number of bytes.
  uint32_t reserved;       // Reserved.
} zx_channel_iovec_t;
```

Each `zx_channel_iovec_t` points to the next `capacity` bytes to be copied from
`buffer` to the kernel message buffer. `reserved` must be assigned to zero.
The `buffer` field may be NULL only if the `capacity` is 0. `buffer` pointers
may be repeated in multiple `zx_channel_iovec_t`.

The signatures of `zx_channel_write`, `zx_channel_write_etc`, `zx_channel_call`
or `zx_channel_call_etc` are unchanged. However, when the user specifies the
`ZX_CHANNEL_WRITE_USE_IOVEC` option to these syscalls, the `void* bytes`
argument will be interpreted as a `zx_channel_iovec_t*`. Similarly, the
`num_bytes` argument will be interpreted as the number of `zx_channel_iovec_t`
in the array.

Note that the type of the handle array (`zx_handle_t` or
`zx_handle_disposition_t`) is irrelevant as only the `bytes` array is
changed.

The message described by the `zx_channel_iovec_t` array with either be sent
with all parts of the message included, or the message will not be sent at
all. Handles provided to the syscall are no longer available to the caller
on both success and failure.

### Error conditions

These are the error conditions of `zx_channel_write`, `zx_channel_write_etc`,
`zx_channel_call` and `zx_channel_call_etc` with updates due to the
introduction of iovecs.

*ZX_ERR_OUT_OF_RANGE*  `num_bytes` or `num_handles` are larger than
`ZX_CHANNEL_MAX_MSG_BYTES` or `ZX_CHANNEL_MAX_MSG_HANDLES` respectively.
If the `ZX_CHANNEL_WRITE_USE_IOVEC` option is specified,
*ZX_ERR_OUT_OF_RANGE* will be produced if `num_bytes` is larger than
`ZX_CHANNEL_MAX_MSG_IOVEC` or the sum of the iovec capacities exceeds
`ZX_CHANNEL_MAX_MSG_BYTES`.

*ZX_ERR_INVALID_ARGS*  `bytes` is an invalid pointer, `handles`
is an invalid pointer, or `options` contains an invalid option bit.
If the `ZX_CHANNEL_WRITE_USE_IOVEC` option is specified,
*ZX_ERR_INVALID_ARGS* if the `buffer` field contains an invalid pointer.

### Alignment

There are no alignment restrictions on the bytes specified in a
`zx_channel_iovec_t`. Each `zx_channel_iovec_t` will be copied without padding.

### Limits

The existing limits on the number of bytes (`65536`) and handles (`64`) per
message are unchanged. Note that these limits apply to messages and not
`zx_channel_iovec_t` entries.

The number of `zx_channel_iovec_t` will be limited to `8192` per syscall. This
number comes from the number of 8-byte aligned inline + out of line objects
that can fit in a `65536` byte message, with each inline + out of line object
potentially using a `zx_channel_iovec_t` entry.

## Implementation

### Syscall

- Introduce the `zx_channel_iovec_t` type, as defined in the design section.
- Add `ZX_CHANNEL_WRITE_USE_IOVEC`
- No changes to the visible syscall interface, the `zx_channel_iovec_t` array
  is passed in to the existing `bytes` parameter.

### Kernel

After receiving the `ZX_CHANNEL_WRITE_USE_IOVEC` option, the kernel will:

- Copy the data pointed to by the `zx_channel_iovec_t` objects to the message
  buffer.  While the copy operations will typically also be performed in order
  of the `zx_channel_iovec_t` inputs, it is not mandatory. However, the final
  message must be laid out in the order of the `zx_channel_iovec_t` entries.
- Write the message to the channel.

### FIDL

This is a proposal for a system call change for which the implementation
takes place within the kernel and the specifics of FIDL binding changes
are out of scope. That said, for the sake of evaluating this proposal it is
important to understand the effect on FIDL encoding.

FIDL bindings can optionally take advantage of `zx_channel_iovec_t` support by
adding support for encoding FIDL objects into an array of `zx_channel_iovec_t`.

A key difference betweeen this encode path and existing encode paths is that
the `zx_channel_iovec_t` allow the kernel to copy objects in-place. The main
complication with this is with pointers. FIDL-encoded messages need to be
sent to the kernel, with pointers replaced with `PRESENT` or `ABSENT` marker
values. However, in many cases the objects continue to need to have the
original pointer values after the system call so that destructors can be
called.

This means that bindings taking advantage of `zx_channel_iovec_t` will
sometimes need to do extra bookkeeping work to make sure the objects are
cleaned up correctly.

### Migration

Since this feature is implemented as an option that is default-disabled, it
shouldn't have an immediate effect on existing users. Call-sites can migrate to
use the option as needed.

Practically speaking, the intention is to migrate FIDL bindings that can
benefit from `zx_channel_iovec_t` to use it. This is expected to have minimal
effects on FIDL users.

## Performance

A prototype was implemented and benchmarked.

- [Kernel change CL](https://fuchsia-review.googlesource.com/c/fuchsia/+/432451)
- [FIDL encoder CL](https://fuchsia-review.googlesource.com/c/fuchsia/+/432223)
- [Benchmark CL](https://fuchsia-review.googlesource.com/c/fuchsia/+/432568)

This prototype implemented the zx_channel_write option on the kernel side
and limited FIDL support (inline objects and vectors only).
The message header, along with each inline and out-of-line object each
had a `zx_channel_iovec_t` entry.
An array of 64 entries was used to store the `zx_channel_iovec_t` in both the
kernel and FIDL encode.

These measurements are from a machine with a Intel Core i5-7300U CPU @ 2.60GHz.

Byte vector event benchmarks (zx_channel_write, zx_channel_wait_async and
zx_channel_read) showed a significant improvement:

- 4096 byte vector: 9398 ns -> 4495 ns
- 256 byte vector: 8788 ns -> 3794 ns

FIDL encode also showed performance improvements.

Encode time of the byte vector examples:

- 4096 byte vector: 345 ns -> 88 ns
- 256 byte vector: 251 ns -> 86 ns

Inline objects also show small encode improvement:

- Struct with 256 uint8 fields: 67 ns -> 49 ns

## Security considerations

Given that this is a significant change to an existing system call, a
security review is needed before the implementation lands.

## Privacy considerations

There should be no impact to privacy.

## Testing

Unit and integration tests will be added for each layer that is changed.

No device or system-wide end-to-end tests are intended to be added, though
existing test coverage will help ensure no unexpected bug has been introduced
after a migration has taken place.

## Documentation

The system call documentation needs to be updated to indicate support for this
feature.

No architecture-wide documentation changes are needed.

## Drawbacks, alternatives, and unknowns

The main drawback of this proposal is added complexity from needing to support
the option in the kernel and the practical added complexity for FIDL bindings
that use the `ZX_CHANNEL_WRITE_USE_IOVEC` option that need to ensure that
objects are properly cleaned up after they have been mutated for in-place copy.

### Limits

There is an argument for a lower limit on the number of `zx_channel_iovec_t`,
potentially closer to `16` than `8192`. This would allow the
`zx_channel_iovec_t` array to be copied onto the kernel's stack. However, this
would prevent the implementation strategy of assigning one `zx_channel_iovec_t`
entry per out-of-line FIDL object.

In practice, it might be more performant to linearize in userspace when there
are a large number of `zx_channel_iovec_t`, or at least avoid shifting work
to the kernel. However, the `8192` limit is suggested for simplicity until
it is known if further refinement is needed.

An implementation-level consequence of the higher limit is that the
`zx_channel_iovec_t`  array cannot entirely fit on the kernel stack. A stack
buffer can be used for the common case, but it will need to be copied into
a larger (and slower) buffer when there are sufficiently many entries.

### Vectorized handles

It would be possible to have an equivalent of `zx_channel_iovec_t` for handles,
or include them alongside bytes in the existing `zx_channel_iovec_t`. However,
the benefits are more limited for handles because the handle array tends to be
small. For simplicity, handles remain in a dedicated array.

### Support for multiple messages in single write

A previous version of this RFC included a proposal for support for multiple
messages in a single `zx_channel_write` call.

Three proposals were considered:

- Flat representation: repurpose the `reserved` field on the
  `zx_channel_iovec_t` with two `uint16_t` fields: `message_seq` (which
  message the `zx_channel_iovec_t` is part of) and `handle_count` (the number
  of handles consumed by the bytes in `buffer`). The sequence numbers are
  constrained to be monotonic and have no gaps. This constraint enables
  a more performant kernel implementation, but can be weakened in the future
  if needed. This approach aligns with this RFC and multi-message support can
  be added to the existing structure.
- Array-of-array representation: there is an outer array of messages, each with
  pointers into an inner array of iovecs per message. This is similar to the
  structure used in the Linux syscall `sendmmsg` and might be more familiar to
  users. While the performance of the array-of-array representation wasnt't
  measured, there is evidence that there could be a 5-25% overhead due to
  indirection (see
  [CL](https://fuchsia-review.googlesource.com/c/fuchsia/+/433621)).
- Header-prefixed representation: the buffer begins with a header and is
  followed by the iovec array. The header consists of 16 message descriptors,
  each of which contains only the a `uint16_t` `message_size` field. This field
  determines the number of `zx_channel_iovec_t` entries associated with the
  message. This representation provides a hierarchical structure but eliminates
  the need for additional redirection and copying.

In design discussions, the flat representation was favored due to its
performance properties and simplicity. While a full proposal for multi-message
support is out of scope of this RFC, please note that this RFC is compatible
with the flat representation.

### Dedicated syscall for iovec

Instead of adding a new option to existing syscalls,
`zx_channel_write_iovec`, `zx_channel_write_etc_iovec`, `zx_channel_call_iovec`
and `zx_channel_call_etc_iovec` syscall could be created. However, an option is
preferred to avoid an explosion in the number of syscalls and cognitive load on
users.

### zx_channel_iovec_t support in zx_channel_read

This RFC proposes support for `zx_channel_iovec_t` for channel writes, but not
channel reads. The reason for this is that there is a clear motivation for
iovecs on the write side - avoiding a FIDL linearization step - but there isn't
a clear and immediate benefit on read side.

The Rust bindings could potentially benefit from read-side iovec support by
partitioning the buffer into multiple smaller buffers each with its own
ownership. This would facilitate a variant of the bindings similar to LLCPP
that essentially casts buffers into output objects. However, there is no
short-term plan to change the Rust bindings to work this way and there doesn't
appear to be much cost to deferring adding support for read-path iovec until
it is needed.

## Prior art and references

Fuchsia has existing `zx_stream_readv` and `zx_stream_writev` system calls that
use vectorized io. Linux also provides similar `readv` and `writev` system
calls that respectively read and write to a file descriptor.
