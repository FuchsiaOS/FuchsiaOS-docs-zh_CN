# libc

## What do we mean by libc?

On Posix-y systems, programs link against a library (either
dynamically or statically) called libc. This library provides the
functions defined by the C standard, as well as the runtime
environment for C programs. Many systems also define other
platform-specific interfaces in the same library. Many of these
interfaces are the preferred way for userspace to access kernel
functionality. For example, Posix-y systems have an `open` function in
their libc which calls an `open` system call. Sometimes these are
cross-platform standards, such as pthreads. Others are interfaces to
kernel-specific functionality, such as `epoll` or `kqueue`. In any
case, this library is present on the system itself and is a stable
interface. In constast, Windows does not provide a systemwide libc in
its stable win32 interface.

On Fuchsia the story is a bit different from Posix systems. First, the
Zircon kernel (Fuchsia's microkernel) does not provide a typical
Posix system call interface. So a Posix function like `open` can't
call a Zircon `open` syscall. Secondly, Fuchsia implements some parts
of Posix, but omits large parts of the Posix model. Most conspicuously
absent are signals, fork, and exec. Third, Fuchsia does not require
that programs use libc's ABI. Programs are free to use their own libc,
or to do without. However, Fuchsia does provide a libc.so which
programs can dynamically link, which provides implementations both of
the C standard library and of the parts of Posix Fuchsia supports, as
typical Posix systems do.

## Piece by piece

This is a partial list of what is implemented (or not) in Fuchsia's
libc.

### The C standard library

Fuchsia's libc implements the C11 standard. In particular this
includes the threading-related interfaces such as threads (`thrd_t`)
and mutexes (`mtx_t`). A small handful of extensions are also in this
portion of the system to bridge the C11 structures, like a `thrd_t`,
to underlying kernel structures, like the `zx_handle_t` underlying it.

### Posix

Posix defines a number of interfaces. These include (not
exhaustively): file I/O, BSD sockets, and pthreads.

#### File I/O and BSD sockets

Recall that Zircon is a microkernel that is not in the business of
implementing file I/O. Instead, other Fuchsia userspace services
provide filesystems. libc itself defines weak symbols for Posix file
I/O functions such as `open`, `write`, and `fstat`. However, all these
calls simply fail. In addition to libc.so, programs can link the
fdio.so library. fdio knows how to speak to those other Fuchsia
services over
[Channel IPC][zircon-concepts-message-passing], and
provides a Posix-like layer for libc to expose. Sockets are similarly
implemented via fdio communicating with the userspace network stack.

#### pthreads

Fuchsia's libc provides parts of the pthread standard. In particular,
the core parts of `pthread_t` (those that map straightforwardly onto
the corresponding C11 concepts) and synchronization primitives like
`pthread_mutex_t` are provided. Some details, like process-shared
mutexes, are not implemented. The implemented subset does not aim to
be comprehensive.

#### Signals

Fuchsia does not have Unix-style signals. Zircon provides no way to
directly implement them (the kernel provides no way to cause another
thread to jump off its thread of execution). Fuchsia's libc does not,
therefore, have a notion of signal-safe functions, and is not
implemented internally to be aware of mechanisms like signals.

Because of this fact, libc functions will not `EINTR`, and it is not
necessary for Fuchsia-only code to consider that case. However, it is
perfectly safe to do so. Fuchsia still defines the `EINTR` constant,
and code written for both Posix and Fuchsia may still have
`EINTR`-handling loops.

#### fork and exec

Zircon does not have fork or exec. Instead, process creation is
provided by [fdio] (https://fuchsia.googlesource.com/zircon/+/master/system/ulib/fdio). While Zircon has Process and
Thread objects, these are pretty raw and know nothing about
ELF. The `fdio_spawn` function family knows how to turn an ELF and some initial
state into a running process.


[zircon-concepts-message-passing]: https://fuchsia.googlesource.com/zircon/+/master/docs/concepts.md#message-passing-sockets-and-channels
