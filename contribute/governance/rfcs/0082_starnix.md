{% set rfcid = "RFC-0082" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!--
*** This should begin with an H2 element (for example, ## Summary).
-->

## Summary

This document proposes a mechanism for running unmodified Linux programs on
Fuchsia. The programs are run in a userspace process whose system interface is
compatible with the Linux ABI. Rather than using the Linux kernel to implement
this interface, we will implement the interface in a Fuchsia userspace program,
called `starnix`. Largely, `starnix` will serve as a compatibility layer,
translating requests from the Linux client program to the appropriate Fuchsia
subsystem. Many of these subsystems will need to be elaborated in order to
support all the functionality implied by the Linux system interface.

## Motivation

In order to run on Fuchsia today, software needs to be recompiled from source
to target Fuchsia. In order to reduce the amount of source modification needed
to run on Fuchsia, Fuchsia offers a POSIX compatibility layer, _POSIX Lite_,
that this software can target. POSIX Lite is layered on top of the underlying
Fuchsia System ABI as a client library.

However, POSIX Lite is not a complete implementation of POSIX. For example,
POSIX Lite does not contain parts of POSIX that imply mutable global state
(e.g., the `kill` function) because Fuchsia is designed around an
object-capability discipline that eschews mutable global state to provide
strong security guarantees. Instead, software that uses POSIX Lite needs
to be modified to use the Fuchsia system interface directly for those use
cases (e.g., the `zx_task_kill` function).

This approach has worked well so far because we have had access to the source
code for the software we needed to run on Fuchsia, which has let us recompile
the software for the Fuchsia System ABI as well as modify parts of the software
that need to be adapted to an object-capability system.

As we expand the universe of software we wish to run on Fuchsia, we are
encountering software that we wish to run on Fuchsia that we do not have the
ability to recompile. For example, Android applications contain native code
modules that have been compiled for Linux. In order to run this software on
Fuchsia, we need to be able to run binaries without modifying them.

## Design

The most direct way of running Linux binaries on Fuchsia would be to run those
binaries in a virtual machine with the Linux kernel as the guest kernel in the
virtual machine. However, this approach makes it difficult to integrate the
guest programs with the rest of the Fuchsia system because they are running in
a different operating system from the rest of the system.

Fuchsia is designed so that you can _bring your own runtime_, which means the
Fuchsia system does not impose an opinion about the internal structure of
components. In order to interoperate as a first-class citizen with the Fuchsia
system, a component need only send and receive correctly formatted messages
over the appropriate `zx::channel` objects.

Rather than running Linux binaries in a virtual machine, `starnix` creates a
_Linux runtime_ natively in Fuchsia. Specifically, a Linux program can be
wrapped with a _component manifest_ that identifies `starnix` as the _runner_
for that component. Rather than using the _ELF Runner_ directly, the binary
for the Linux program is given to `starnix` to run.

In order to execute a given Linux binary, `starnix` manually creates a
`zx::process` with an initial memory layout that matches the Linux ABI. For
example, `starnix` populates `argv` and `environ` for the program as data
on the stack of the initial thread (along with the `aux` vector) rather than
as a message on the bootstrap channel, as this data is populated in the Fuchsia
System ABI.

### System calls

After loading the binary into the client process, `starnix` registers to handle
all the syscalls from the client process (see [Syscall Mechanism](#syscalls)
below). Whenever the client issues a syscall, the Zircon kernel transfers
control to `starnix`, which decodes the syscall according to Linux syscall
conventions and does the work of the syscall.

For example, if the client program issues a `brk` syscall, `starnix` will
manipulate the address space of the client process using the appropriate
`zx::vmar` and `zx::vmo` operations to change the address of the
_program break_ of the client process. In some cases, we might need to
elaborate the ability for one process (i.e., `starnix`) to manipulate the
address space of another process (i.e., the client), but early experimentation
indicates that Zircon already contains the bulk of the machinery needed for
remote address-space manipulation.

As another example, suppose the client program issues a `write` syscall. To
implement file-related functionality, `starnix` will maintain a
_file descriptor table_ for each client process. Upon receiving a `write`
syscall, `starnix` will look up the identified file descriptor in the file
descriptor table for the client process. Typically, that file descriptor will
be backed by a `zx::channel` that implements the `fuchsia.io.File` FIDL
protocol. To execute the `write`, `starnix` will format a
`fuchsia.io.File#Write` message containing the data from the client address
space (see [Memory access](#memory)) and send that message through the channel,
similar to how _POSIX Lite_ implements `write` in a client library.

### Global state

To handle syscalls that imply mutable global state, `starnix` will maintain
some mutable state shared between client processes. For example, `starnix`
will assign a `pid_t` to each client process it runs and maintain a table
mapping `pid_t` to the underlying `zx::process` handle for that process. To
implement the `kill` syscall, `starnix` will look up the given `pid_t` in this
table and issue a `zx_task_kill` syscall on the associated `zx::process`
handle.

In this way, each `starnix` instance serves as a _container_ for related Linux
processes. If we wish to have strong isolation guarantees between two Linux
processes, we can run those processes in separate `starnix` instances without
the overhead (e.g., scheduling complexities) of running multiple virtual
machines.

Each `starnix` instance will also expose its global state for use by other
Fuchsia processes. For example, `starnix` will maintain a namespace of
`AF_UNIX` sockets. This namespace will be accessible both from Linux binaries
run by `starnix` and from Fuchsia binaries that communicate with `starnix`
over FIDL.

The Linux system interface also implies a global file system. As Fuchsia does
not have a global file system, `starnix` will synthesize a "global" file system
for its client processes from its own namespace. For example, `starnix` will
mount `/data/root` from its own namespace as `/` in the global file system
presented to client processes. Other mount points, such as `/proc` can be
implemented internally by `starnix`, for example by consulting its table
of running processes.

### Security

As much as possible, `starnix` will build upon the security mechanisms of the
underlying Fuchsia system. For example, when interfacing with system services,
such as file systems, networking, and graphics, `starnix` will serve largely as
a translation layer, reformatting requests from the Linux ABI to the Fuchsia
System ABI. The system services will be responsible for enforcing their
own security invariant, just as they do for every other client. However,
`starnix` will need to implement some security mechanisms to protect access to
its own services. For example, `starnix` will need to determine whether one
client process is allowed to `kill` another client process.

To make these security decisions, `starnix` will track a security context for
each client process, including a `uid_t`, `gid_t`, effective `uid_t`, and
effective `gid_t`. Operations that require security checks will use this
security context to make appropriate access control decisions. Initially, we
expect this mechanism to be used infrequently, but as our use cases grow more
sophisticated, our needs for access control are also likely to grow more
complex.

### As she is spoke {#as-she-is-spoke}

When faced with a choice for how `starnix` ought to behave in a certain
situation, the design favors behaving as close to how Linux behaves as
feasible. The intention is to create an implementation of the Linux interface
that can run existing, unmodified Linux binaries. Whenever `starnix` diverges
from Linux semantics, we run a risk that some Linux binary will notice the
divergence and behave improperly.

To be able to discuss this design principle more easily, we say that `starnix`
implements Linux
[_as she is spoke_](https://en.wikipedia.org/wiki/English_as_She_Is_Spoke),
which is to say with all the beauty, ugliness, coincidences, and quirks of a
real Linux system.

In some cases, implementing the Linux interfaces as she is spoke will require
adding functionality to a Fuchsia service to provide the require semantics. For
example, implementing `inotify` requires support from the underlying file
system implementation in order to work efficiently. We should aim to add this
functionality to Fuchsia services in a way that integrates well with the rest
of the functionality exposed by the service.

## Implementation

We plan to implement `starnix` as a Fuchsia component, specifically a normal
userspace component that implements the _runner_ protocol. We plan to implement
`starnix` in Rust to help avoid privilege escalation from the client process to
the `starnix` process.

### Executive

One of the core pieces of `starnix` is the _executive_, which implements the
semantic concepts in the Linux system interface. For example, the executive
will have objects that represent threads, processes, and file descriptions.

The executive will be structured such that it can be unit tested independently
from the rest of the `starnix` system. For example, we will be able to unit
test that duplicating a file descriptor shares an underlying file description
without needing to run a process with the Linux ABI.

### Linux syscall definitions

In order to implement Linux syscalls, `starnix` needs a description of each
Linux syscall as well as the userspace memory layout of any associated input or
output parameters. These are defined in the Linux `uapi`, which is a
freestanding collection of C headers. To make use of these definitions in Rust,
we will use Rust `bindgen` to generate Rust declarations.

The Linux `uapi` evolves over time. Initially, we will target the Linux `uapi`
from Linux 5.10 LTS, but we will likely need to adjust the exact version of the
Linux `uapi` we support over time.

### Syscall mechanism {#syscalls}

The initial implementation of `starnix` will use Zircon exceptions to trap
syscalls from the client process. Specifically, whenever the client process
attempts to issue a syscall, Zircon will reject the syscall because Zircon
requires syscalls to be issued from within the Zircon vDSO, which the client
process is unaware exists.

Zircon rejects these syscalls by generating a `ZX_EXCP_POLICY_CODE_BAD_SYSCALL`
exception. The `starnix` process will catch these exceptions by installing
an exception handler on each client process. To receive the parameters for
the syscall, `starnix` will use `zx_thread_read_state` to read the registers
from the thread that generated the exception. After processing the syscall,
`starnix` sets the return value for the syscall using
`zx_thread_write_state` and then resumes the thread in the client process.

This mechanism works but is unlikely to have high enough performance to be
useful. After we build out a sufficient amount of `starnix` to run Linux
benchmarks, we will likely want to replace this syscall mechanism with a more
efficient mechanism. For example, perhaps `starnix` will associate a `zx::port`
for handling syscalls from the client process and Zircon will queue a packet to
the `zx::port` with register state of the client process. When we have
benchmarks in place, we can prototype a variety of approaches and select the
best design at that time.

### Memory access {#memory}

The initial implementation of `starnix` will use the `zx_process_read_memory`
and `zx_process_write_memory` to read and write data from the address space
of the client process. This mechanism works, but is undesirable for two
reasons:

 1. These syscalls are disabled in production builds due to security concerns.
 2. These syscalls are vastly more expensive than reading and writing memory
    directly.

After we build out a sufficient amount of `starnix` to run Linux benchmarks,
we will want to replace this mechanism with something more efficient. For
example, perhaps `starnix` will restrict the size of the client address space
and map each client's address space into its own address space at some
client-specific offset. Alternatively, perhaps when the `starnix` services a
syscall from a client, Zircon will arrange for that client's address space to
be visible from that thread (e.g., similar to how kernel threads have
visibility into the address space of userspace process when servicing syscalls
from those processes).

As with the syscall mechanism, we can prototype a variety of approaches and
select the best design once we have more running code to use to evaluate the
approaches.

### Interoperability

We will develop `starnix` using a test-driven approach. Initially, we will use
a naively simple implementation that is sufficient to run basic Linux binaries.
We have already prototyped an implementation that can run a `-static-pie` build
of a `hello_world.c` program. The next step will be to clean up that prototype
and teach `starnix` how to run a dynamically linked `hello_world.c` binary.

After running these basic binaries, we will bring up unit test binaries from
various codebases. These binaries will help ensure that our implementation of
the Linux ABI is correct (i.e., as Linux is spoke). For example, we will run
some low-level test binaries from the Android source tree as well as binaries
from the _Linux Test Project_.

## Performance

Performance is a critical aspect of this project. Initially, `starnix` will
perform quite poorly because we will be using inefficient mechanisms for
trapping syscalls and for access client memory. However, those are areas that
we should be able to optimize substantially once we have sufficient
functionality to run benchmarks in the Linux execution environment.

In addition to optimizing these mechanisms, we also have the opportunity to
offload high-frequency operations to the client. For example, we can implement
`gettimeofday` directly in the client address space by loading code into the
client process before transferring control to the Linux binary. For example, if
the Linux binary invokes `gettimeofday` through the Linux vDSO, `starnix` can
provide a shared library in place of the Linux vDSO that implements
`gettimeofday` directly by calling through to the Zircon vDSO.

## Security considerations

This proposal has many subtle security considerations. There is a trust
boundary between the `starnix` process and the client process. Specifically,
the `starnix` process can hold object-capabilities that are not fully exposed
to the client. For example, the `starnix` process maintains a file descriptor
table for each client process. One client process should be able to access
handles stored in its file descriptor table but not handles stored in the
file descriptor table for another process. Similarly, `starnix` maintains
shared mutable state that clients can interact with only subject to access
control.

To provide this trust boundary, `starnix` runs in a separate userspace process
from the client processes. To help avoid privilege escalation, we plan to
implement `starnix` in Rust and to use Rust's type system to avoid type
confusion. We also plan to use Rust's type system to clearly distinguish client
data, such as addresses in the client's address space and data read from the
client address space, from reliable data maintained by `starnix` itself.

Additionally, we need to consider the provenance of the Linux binaries
themselves because `starnix` runs those binaries directly, rather than, for
example, in virtual machine or SFI container. We will need to revisit this
consideration in the context of a specific, end-to-end product use case that
involves Linux binaries.

The access control mechanism within `starnix` will require a detailed security
evaluation, ideally including direct participation from the security team in
its design and, potentially, implementation. Initially, we expect to have a
simple access control mechanism. As the requirements for this mechanism grow
more sophisticated, we will need further security scrutiny.

Finally, the designs for the high-performance syscall and client memory
mechanisms will need careful security scrutiny, especially if we end up using
an exotic address space configuration for `starnix` or attempt to directly
transfer register state from the client thread to a `starnix` thread.

## Privacy considerations

This design does not have any immediate privacy considerations. However, once
we have a specific, end-to-end product use case that involves Linux binaries,
we will need to evaluate the privacy implications of that use case.

## Testing

Testing is a central aspect of building `starnix`. We will directly unit test
the `starnix` executive. We will also build out our implementation of the Linux
system interface by attempting to pass test binaries intended to run on Linux.
We will then run these binaries in continuous integration to ensure that
`starnix` does not regress.

We will also compare running Linux binaries in `starnix` with running those
same binaries in a virtual machine on Fuchsia. We expect to be able to run
Linux binaries more efficiently in `starnix`, but we should validate that
hypothesis.

## Documentation

At this stage, we plan to document `starnix` through this RFC. Once we get
non-trivial binaries running, we will need to document how to run Linux
binaries on Fuchsia.

## Drawbacks, alternatives, and unknowns

There is a large design space to explore for how to run unmodified Linux
binaries on Fuchsia. This section summarizes the main design decisions.

### Linux kernel

An important design choice is whether to use the Linux kernel itself to
implement the Linux system interface. In addition to building `starnix`,
we will also build a mechanism for running unmodified Linux binaries by
running the Linux kernel inside a Machina virtual machine. This approach has
a small implementation burden because the Linux kernel is designed to run
inside a virtual machine and the Linux kernel already contains an
implementation of the hundreds of syscalls that make up the Linux system interface.

There are several ways we could use the Linux kernel. For example, we could
run the Linux kernel in a virtual machine, we could use
[User-Mode Linux (UML)][uml] or we could use the
[Linux Kernel Library (LKL)][lkl]. However, regardless of how we run it, there
is a large cost to running an entire Linux kernel in order to run Linux
binaries. At its core, the job of the Linux kernel is to reduce high-level
operations (e.g., `write`) to low-level operations (e.g., DMA data to an
underlying piece of hardware). This core function is counter-productive for
integrating Linux binaries into a Fuchsia system. Instead of reducing a
`write` operation to a DMA, we wish to translate a `write` operation into a
`fuchsia.io/File.Write` operation, which is at an equivalent semantic level.

Similarly, the Linux kernel comes with a scheduler, which controls the threads
in the processes it manages. The purpose of this functionality is to reduce
high-level operations (e.g., run a dozen concurrent threads) to low-level
operations (e.g., execute this time slice on this processor). Again, this core
functionality is counter-productive. We can compute a better schedule for the
system as a whole if the threads running for each Linux binary are actually
Zircon threads scheduled by the same scheduler as all the other threads in the
system.

### Environment

Once we have decided to implement the Linux system interface directly using
the Fuchsia system, we need to choose where to run that implementation.

#### In-process

We could run the implementation in the same process as the Linux binary. For
example, this approach is used by _POSIX Lite_ to translate POSIX operations
into Fuchsia operations. However, this approach is less desirable when running
unmodified Linux binaries for two reasons:

 1. If we run the implementation in-process, we will need to "hide" the
    implementation from the Linux binary because Linux binaries do not expect
    the system to be running (much) code in their process. For example, any use
    of thread-local storage by the implementation must take care not to collide
    with the thread-local storage managed by the Linux binary's C runtime.

 2. Many parts of the Linux system interface imply mutable global state. An
    in-process implementation would still need to coordinate with an
    out-of-process server to implement those parts of the interface correctly.

For these reasons, we have chosen to start with an out-of-process server
implementation. However, we will likely offload some operations from the server
to the client for performance.

#### Userspace

In this approach, the implementation runs in a separate userspace process from
the Linux process. This approach is the one we have selected for `starnix`. The
primary challenges with this approach are that we need to carefully design the
mechanisms we use for syscalls and client memory access to give sufficient
performance. There is some unavoidable overhead to involving a second userspace
process because we will need to perform an extra context switch to enter that
process, but there is evidence from other systems that we can achieve excellent
performance.

#### Kernel

Finally, we could run the implementation in the kernel. This approach is the
traditional approach for providing foreign personalities for operating
systems. However, we would like to avoid this approach in order to reduce the
complexity of the kernel. Having a kernel that follows a clear object-capability
discipline makes reasoning about the behavior of the kernel much easier,
resulting in better security.

The primary advantage that an in-kernel implementation offers over a userspace
implementation is performance. For example, the kernel can directly receive
syscalls and already has a high-performance mechanism for interacting with
client address spaces. If we are able to achieve excellent performance with
a userspace approach, then there will be little reason to run the
implementation in the kernel.

### Async signals

Linux binaries expect the kernel to run some of their code in async signal
handlers. Fuchsia currently does not contain a mechanism for directly invoking
code in a process, which means there is no obvious mechanism for invoking
async signal handlers. Once we encounter a Linux binary that requires support
for async signal handlers, we will need to devise a way to support that
functionality.

### Futexes

Futexes work differently on Fuchsia and Linux. On Fuchsia, futexes are keyed off
virtual addreses whereas Linux provides the option to key futexes off physical
addresses. Additionally, Linux futexes offer a wide variety of options and
operations that are not available on Fuchsia futexes.

In order to implement the Linux futex interface, we will either need to
implement futexes in `starnix` or add functionality to the Zircon kernel to
support the functionality required by Linux binaries.

## Prior art and references

There is a large amount of prior art for running Linux (or POSIX) binaries on
non-POSIX systems. This section describes two related systems.

### WSL1

The design in this document is similar to the first
[Windows Subsystem for Linux (WSL1)][wsl], which was an implementation of the
Linux system interface on Windows that was able to run unmodified Linux
binaries, including entire GNU/Linux distributions such as Ubuntu, Debian, and
openSUSE. Unlike `starnix`, WSL1 ran in the kernel and provided a Linux
personality for the NT kernel.

Unfortunately, WSL1 was hampered by the performance characteristics of NTFS,
which do not match the expectations of Linux software. Microsoft has since
replaced WSL1 with WSL2, which provides similar functionality by running the
Linux kernel in a virtual machine. In WSL2, Linux software runs against an
`ext4` file system, rather than an NTFS file system.

An important cautionary lesson we should draw from WSL1 is that the performance
of `starnix` will hinge on the performance of the underlying system services
that `starnix` exposes to the client program. For example, we will need to
provide a file system implementation with comparable performance to `ext4` if
we want Linux software to perform well on Fuchsia.

### QNX Neutrino

[QNX Neutrino][qnx] is a commercial microkernel-based operating system that
provides a high-quality POSIX implementation. The approach described in this
document for `starnix` is similar to the `proc` server in QNX, which services
POSIX calls from client processes and maintains the mutable global state
implied by the POSIX interface. Similar to `starnix`, `proc` is a userspace
process on QNX.

[uml]: https://en.wikipedia.org/wiki/User-mode_Linux
[lkl]: https://lkl.github.io/
[wsl]: https://en.wikipedia.org/wiki/Windows_Subsystem_for_Linux
[qnx]: https://en.wikipedia.org/wiki/QNX
