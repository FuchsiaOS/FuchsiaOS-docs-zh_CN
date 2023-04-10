{% set rfcid = "RFC-0007" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

In the past, `zx_task_kill` allowed usermode to kill individual threads. However,
killing individual threads encourages bad practices and has a high chance of leaving
the process in a bad state. For this reason, the ability to kill individual threads
should be removed.

## Motivation and problem statement

There is no reasonable use for usermode to kill individual threads. Exposing such facility
encourages bad practices.

On Fuchsia, like other systems, killing a thread is done asynchronously; for running threads there
is no practical way to determine the exact place where it is safe to terminate a thread. For a
blocked (waiting)  thread, the safer and often simple solution is to add logic so upon wakeup the
thread exits by itself.

Dangers killing a thread

* Locks can be left acquired, including global locks like ones controlling the heap.
* Memory can be leaked. At the very least the thread stack, but often many other pieces.
* Runtime left in an inconsistent state. This is at least true for the C and Go runtime.
* Killing a thread in its way to a syscall leaves the process in an unknown state. Kernel is
  fine but the process does not have a way to know what happened and what did not happen.
* Defeats RAII wrappers and automatic cleanup. In fact, it defeats most guarantees from the high
  level languages Fuchsia uses.

## Design

The following syscall will fail with `ZX_ERR_NOT_SUPPORTED` when passed a handle to a thread:

```
zx_status_t zx_task_kill(zx_handle_t handle);
```

Processes and jobs will still be killable as normal.

## Implementation

Luckily, thread killing is not used very much in Fuchsia. The only use cases are in test code
that checks that a thread hits a specific exception. This code is going to be updated so that
the excepting thread exits itself after the exception is handled. For code where the exception
is unrecoverable, the excepting thread's instruction pointer can be set directly to
zx_thread_exit or the runtime's thread exit function before the thread resumes. These tests
may still leak what the excepting thread had stored on the heap, but the runtime is in
a better state, and the leaks will be collected when the test's process exits.

## Performance

N/A

## Security considerations

N/A

## Privacy considerations

N/A

## Testing

The zircon core-tests will be updated to ensure that the zx_task_kill syscall behaves as intended.
Some amount of static analysis can be done to find call sites of zx_task_kill that are passed
threads.

The full Fuchsia Large Scale Change (LSC) process will be followed to ensure this change is
properly tested.

## Documentation

The documentation for [zx_task_kill](/docs/reference/syscalls/task_kill.md) will be updated to
reflect that threads are not killable.

## Drawbacks, Alternatives, and Unknowns

The alternative to this proposal is the current status quo, which is to allow threads to be
killed. Threads have been killable for the entire history of Fuchsia, but there has not been
any acceptable use cases where programs have relied on this behavior. For this reason,
we believe that thread killing can be safely removed.

## Prior art and references

* [Windows Vista tries to remove
TerminateThread](https://devblogs.microsoft.com/oldnewthing/20150814-00/?p=91811)
