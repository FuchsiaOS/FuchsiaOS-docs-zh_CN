# Zircon Kernel IPC Limits

## Introduction

The Zircon API is asynchronous for both sending and receiving. It requires that kernel buffers
data on behalf of the senders until receivers can drain it. If the total rate of sending data
is greater than the rate of reading the data over an extended period of time system can run out
of kernel buffers to service even critical tasks.

The experience is that for asynchronous systems that (in very rare occasions) return error codes
such as "try again", the application handling code is rarely correct; these paths are hard to test
and hard to get right: in particular the simple treatment of retry in a loop converts an
asynchronous service into a synchronous service, leading to livelocks and deadlocks.

Instead, when programming for Zircon, developers should assume that sending IPC messages at a
reasonable rate in a healthy system always succeeds. The kernel in turn implements some limits on
the amount of data it can buffer and when the limit is crossed, a
[policy exception](concepts/kernel/exceptions.md) is raised in the calling thread for the
following syscalls:

 - [`zx_channel_write()`]
 - [`zx_channel_write_etc()`]
 - [`zx_port_queue()`]

The precise limits are enforced per each instance of a kernel object and are not disclosed in the
form of constants to applications to prevent code from depending on these limits, which can be
further modified by product-specific considerations.

It is not expected that the application or library handles the exception; in most cases the
appropriate action so to let the exception propagate to the crash analysis service.

## Strategies to Avoid IPC Limits

At a high level the main strategy is to equalize the rate of buffers being added and being drained
from each pair of consume and producer. There are many possible schemes such as flow control,
request/response, request-expiration, sidecar VMOs, etc. The most appropriate method depends
on the nature of the service.


[`zx_channel_write()`]: reference/syscalls/channel_write.md
[`zx_channel_write_etc()`]: reference/syscalls/channel_write_etc.md
[`zx_port_queue()`]: reference/syscalls/port_queue.md

