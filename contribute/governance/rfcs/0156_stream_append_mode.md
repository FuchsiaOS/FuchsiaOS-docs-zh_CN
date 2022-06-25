<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0156" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC proposes modifying the stream API by adding an
append mode to streams and removing the append option from
`zx_stream_writev`.

## Motivation

The stream append option, `ZX_STREAM_APPEND`, is currently only used in the C++
VFS for implementing `fuchsia.io/File.Write` for memfs. All of the memfs streams
live on the server side of the FIDL connection where the append mode of the
connection is known. Accessing the append mode to optionally pass
`ZX_STREAM_APPEND` to each `zx_stream_writev` call works for this specific
situation.

With the implementation of writeback for userspace pagers, streams can be sent
to the client side of the FIDL connection to significantly increase IO
performance. Moving streams to the client side, with the current stream API,
will require fdio to keep track of the append mode of the connection.

## Design

### Overview

The goal of this design is to avoid keeping track of the append mode in fdio and
bring the stream API more inline with POSIX.

The stream API is very similar to reading, writing, and seeking a file
in POSIX, with the exception of how appends are handled. In POSIX a file can be
opened in append mode by passing `O_APPEND` to `open`. All POSIX `write` calls
to a file in append mode behave similarly to `zx_stream_writev` with the
`ZX_STREAM_APPEND` option.

Adding an append mode to streams will avoid fdio storing the append mode and be
more inline with POSIX. With this addition, the `ZX_STREAM_APPEND` option for
`zx_stream_writev` is no longer necessary.

### API Changes

This RFC consists of 3 API changes:

  1. A new option for `zx_stream_create` named, `ZX_STREAM_MODE_APPEND`, will be
  added. A stream created with `ZX_STREAM_MODE_APPEND` will be placed in append
  mode. All `zx_stream_writev` calls to a stream in append mode will behave as
  if `ZX_STREAM_APPEND` was passed to the `zx_stream_writev` call.

  2. A new property named `ZX_PROP_STREAM_MODE_APPEND` will be added. This
  property is necessary for fdio to implement `fcntl` with `F_SETFL` and
  `O_APPEND`.
      - `ZX_PROP_STREAM_MODE_APPEND` will have a value type of `uint8_t`.
      - Calling `zx_object_set_property` with `ZX_PROP_STREAM_MODE_APPEND` and
      a value of `0` will take a stream out of append mode.
      - Calling `zx_object_set_property` with `ZX_PROP_STREAM_MODE_APPEND` and
      any value other than `0` will put a stream into append mode.
      - For `zx_object_get_property` calls with `ZX_PROP_STREAM_MODE_APPEND` the
        property will have a value of `1` if the stream is in append and a value
        of `0` if the stream is not in append mode.

  3. The `ZX_STREAM_APPEND` option for `zx_stream_writev` will be removed.

## Implementation

This RFC will be implemented in 3 Gerrit changes:

1. Add `ZX_STREAM_MODE_APPEND` and `ZX_PROP_STREAM_MODE_APPEND` to the stream
   API.

2. Migrate the C++ VFS from `ZX_STREAM_APPEND` to `ZX_STREAM_MODE_APPEND` and
   `ZX_PROP_STREAM_MODE_APPEND`

3. Remove the `ZX_STREAM_APPEND` option from the stream API.

## Performance

This proposal should have no performance impact.

## Ergonomics

Storing the append mode inside of the stream brings the stream API inline with
`fuchsia.io/Directory.Open` and `open` which developers will already be familiar
with.

## Security considerations

None.

## Privacy considerations

None.

## Testing

Core tests will be written to exercise the new API including tests with multiple
threads.

The existing fs_test suite used by memfs already includes append related tests
which should catch any potential regressions during the migration.

## Documentation

The Zircon stream documentation will be updated with the changes to the API.

## Drawbacks, alternatives, and unknowns

### Alternative: Store the append mode in fdio
Store the append mode of the connection directly in fdio and use the stream API
as is.

Storing the append mode in the stream is preferred because it matches how
appends work in POSIX.

### Alternative: Keep `ZX_STREAM_APPEND` for `zx_stream_writev`

Filesystems that support streams will typically implement
`fuchsia.io/File.Read`, `Write`, and `Seek` by dispatching the requests to a
stream internally. Each connection already keeps track of its append mode for
responding to `GetFlags` requests which makes using `ZX_STREAM_APPEND` with
`zx_stream_writev` convenient.

Removing `ZX_STREAM_APPEND` forces the filesystem to keep the append mode of the
connection and the append mode of the stream in sync, which is not difficult.
Reducing the API surface area and matching POSIX is preferred over keeping
`ZX_STREAM_APPEND`.

## Prior art and references

[`zx_stream_create`](/docs/reference/syscalls/stream_create.md)
with `ZX_STREAM_MODE_APPEND` is similar to
[`open`](https://pubs.opengroup.org/onlinepubs/9699919799/functions/open.html)
with `O_APPEND` from POSIX.

[`zx_object_set_property`](/docs/reference/syscalls/object_set_property.md)
with `ZX_PROP_STREAM_MODE_APPEND` is similar to
[`fcntl`](https://pubs.opengroup.org/onlinepubs/9699919799/functions/fcntl.html)
with `F_SETFL` and `O_APPEND` from POSIX.
