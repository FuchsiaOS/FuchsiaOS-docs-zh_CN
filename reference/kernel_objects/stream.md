# Stream

## NAME

stream - A readable, writable, seekable interface to some underlying storage

## SYNOPSIS

A stream is an interface for reading and writing data to some underlying
storage, typically a VMO.

## DESCRIPTION

A stream maintains a reference to some underlying storage, typically a VMO,
as well as a seek offset, which is used to position read and write operations.

Unlike the read and write operations on a VMO, the read and write operations
on a stream can be short, which the operations can complete successfully
without filling (or, respectively, emptying) the supplied buffers. For example,
a read that extends beyond the end of a VMO will simply fail whereas a read
that extends beyond the end of a stream will succeed in reading to the end of
the stream and partially filling the buffer.

Writes that extend beyond the end of the underlying storage attempt to increase
the size of the underlying storage rather than failing immediately. For
example, a write to a stream that extends beyond the end of the underlying VMO
will attempt to resize the VMO rather than failing. If the resize operation
fails on the underlying VMO, the write can end up being short.

## SYSCALLS

 - [`zx_stream_create()`] - create a stream from a VMO
 - [`zx_stream_readv()`] - read data from the stream at the current seek offset
 - [`zx_stream_readv_at()`] - read data from the stream at a given offset
 - [`zx_stream_writev()`] - write data to the stream at the current seek offset
 - [`zx_stream_writev_at()`] - write data to the stream at a given offset
 - [`zx_stream_seek()`] - modify the current seek offset of the stream

[`zx_stream_create()`]: /docs/reference/syscalls/stream_create.md
[`zx_stream_readv()`]: /docs/reference/syscalls/stream_readv.md
[`zx_stream_readv_at()`]: /docs/reference/syscalls/stream_readv_at.md
[`zx_stream_writev()`]: /docs/reference/syscalls/stream_writev.md
[`zx_stream_writev_at()`]: /docs/reference/syscalls/stream_writev_at.md
[`zx_stream_seek()`]: /docs/reference/syscalls/stream_seek.md
