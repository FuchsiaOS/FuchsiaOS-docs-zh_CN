# Port

## NAME

port - Signaling and mailbox primitive

## SYNOPSIS

Ports allow threads to wait for packets to be delivered from various
events. These events include explicit queueing on the port,
asynchronous waits on other handles bound to the port, and
asynchronous message delivery from IPC transports.

## DESCRIPTION

TODO

## SYSCALLS

 - [`zx_port_create()`] - create a port
 - [`zx_port_queue()`] - send a packet to a port
 - [`zx_port_wait()`] - wait for packets to arrive on a port
 - [`zx_object_wait_async()`] - cause packets to arrive on a port, when another object is signaled

[`zx_port_create()`]: /docs/reference/syscalls/port_create.md
[`zx_port_queue()`]: /docs/reference/syscalls/port_queue.md
[`zx_port_wait()`]: /docs/reference/syscalls/port_wait.md
[`zx_object_wait_async()`]: /docs/reference/syscalls/object_wait_async.md
