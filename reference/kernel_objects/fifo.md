# FIFO

## NAME

FIFO - first-in first-out interprocess queue

## SYNOPSIS

FIFOs are intended to be the control plane for shared memory
transports.  Their read and write operations are more efficient than
[sockets](socket.md) or [channels](channel.md), but there are severe
restrictions on the size of elements and buffers.

## DESCRIPTION

TODO

## SYSCALLS

 - [`zx_fifo_create()`] - create a new fifo
 - [`zx_fifo_read()`] - read data from a fifo
 - [`zx_fifo_write()`] - write data to a fifo

[`zx_fifo_create()`]: /docs/reference/syscalls/fifo_create.md
[`zx_fifo_read()`]: /docs/reference/syscalls/fifo_read.md
[`zx_fifo_write()`]: /docs/reference/syscalls/fifo_write.md
