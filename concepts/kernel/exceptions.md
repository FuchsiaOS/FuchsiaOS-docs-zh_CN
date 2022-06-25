# Exception Handling

## Introduction

When a thread encounters a fault condition, for example a segfault, execution
is paused and the thread enters exception handling. Handlers that have
registered to receive these exceptions are notified and given a chance to
inspect or correct the condition.

This functionality is commonly used by debuggers or crash loggers, which want
to have a chance to interact with threads before they would otherwise crash.
For applications that just want to track task lifecycles without needing to
intercept crashes, [signals](signals.md) may be a better choice.

## The Basics

Exceptions are handled from userspace by creating an exception channel on a
task (thread, process, or job) with the [`zx_task_create_exception_channel()`]
system call. The created handle is a standard Zircon
[channel](reference/kernel_objects/channel.md), but is created read-only so can only be used
for receiving exception messages.

When an exception occurs, the thread is paused and a message containing a
`zx_exception_info_t` and an exception handle is sent to the channel. The
lifetime of the exception is bound to the lifetime of this exception handle, so
when the receiver is done processing, closing this exception handle will resume
the exception. This exception handle is non-copyable, meaning that at any given
time, there is only one handler for this exception.

By default, closing an exception handle will keep the thread paused and send
the exception to the next handler. If the receiver has corrected the exception
and wants the thread to resume execution instead, it can change the exception
state to `ZX_EXCEPTION_STATE_HANDLED` via [`zx_object_set_property()`] before
closing.

## Exception Handles

Exception handles behave similarly to suspend tokens by keeping the thread
paused until they are closed. Additionally, exception handles have functions
to help receivers process the exception:

* [`zx_object_set_property()`] with `ZX_PROP_EXCEPTION_STATE` to set behavior
  on handle close
* [`zx_exception_get_thread()`] to get a handle to the exception thread
* [`zx_exception_get_process()`] to get a handle to the exception process
  (process or job exception channels only)

Task handles retrieved from exceptions will have the same rights as the task
originally passed into [`zx_task_create_exception_channel()`].

### Example

This simple example creates an exception channel and loops reading exceptions
until the task closes.

```cpp
void ExceptionHandlerLoop(zx_handle_t task) {
  // Create the exception channel.
  uint32_t options = 0;
  zx_handle_t channel;
  zx_status_t status = zx_task_create_exception_channel(task, options,
                                                        &channel);
  // ... check status ...

  while (true) {
    // Wait until we get ZX_CHANNEL_READABLE (exception) or
    // ZX_CHANNEL_PEER_CLOSED (task terminated).
    zx_signals_t signals = 0;
    status = zx_object_wait_one(channel,
                                ZX_CHANNEL_READABLE | ZX_CHANNEL_PEER_CLOSED,
                                ZX_TIME_INFINITE, &signals);
    // ... check status ...

    if (signals & ZX_CHANNEL_READABLE) {
      // Read the exception info and handle from the channel.
      zx_exception_info_t info;
      zx_handle_t exception;
      status = zx_channel_read(channel, 0, &info, &exception, sizeof(info), 1,
                               nullptr, nullptr);
      // ... check status ...

      // Send the exception out to some other function for processing, which
      // returns true if the exception has been handled and we can resume the
      // thread, or false to pass the exception to the next handler.
      bool handled = process_exception(info, exception);
      if (handled) {
        uint32_t state = ZX_EXCEPTION_STATE_HANDLED;
        status = zx_object_set_property(exception, ZX_PROP_EXCEPTION_STATE,
                                        &state, sizeof(state));
        // ... check status ...
      }

      // Close the exception to finish handling.
      zx_handle_close(exception);
    } else {
      // We got ZX_CHANNEL_PEER_CLOSED, the task has terminated.
      zx_handle_close(channel);
      return;
    }
  }
}
```

## Exception Types

At a high level there are two types of exceptions: architectural and synthetic.
Architectural exceptions are things like a segfault (e.g., dereferencing the
NULL pointer) or executing an undefined instruction. Synthetic exceptions are
things like thread start/stop notifications or
[policy violations](reference/syscalls/job_set_policy.md).

Architectural and policy exceptions are considered fatal, and will cause the
process to be killed if they are unhandled. Debugger-only exceptions - thread
start/stop and process start - are informational and will continue execution
normally even if the thread isn't explicitly resumed. These exceptions are
meant to give a debugger a chance to react to these lifetime events correctly,
as the corresponding thread will be paused until the exception is resumed.

Exception types are defined in
[`<zircon/syscalls/exception.h>`](/zircon/system/public/zircon/syscalls/exception.h).

## Exception Channel Types

Exception channels have different characteristics depending on the task type and
whether the `ZX_EXCEPTION_CHANNEL_DEBUGGER` flag is passed to
[`zx_task_create_exception_channel()`]. The table below summarizes the
differences between the various channel types:

Channel Type  | `get_thread` | `get_process` | Architectural & Policy Exceptions | Thread Start/Stop Exceptions | Process Start Exception
------------- | :----------: | :-----------: | :-------------------------------: | :--------------------------: | :---------------------:
Thread        | X            |               | X                                 |                              |
Process       | X            | X             | X                                 |                              |
Process Debug | X            | X             | X                                 | X                            |
Job           | X            | X             | X                                 |                              |
Job Debug     | X            | X             |                                   |                              | X

The channel type also determines the order in which exception channels will be
given the chance to handle an exception:

1. process debug
2. thread
3. process
4. process debug (optionally, if the exception is [`'second-chance'`](#process-debugger-first-and-possibly-again-later))
5. job (parent job -> grandparent job -> etc)

If there are no remaining exception channels to try, the kernel terminates the
process as if [`zx_task_kill()`] was called. The return code of a process
terminated by an exception is `ZX_TASK_RETCODE_EXCEPTION_KILL`, and can be
obtained with [`zx_object_get_info()`] using `ZX_INFO_PROCESS`.

Each task only supports a single exception channel per type, so for example
given a process with a debug exception channel attached, trying to create
a second debug exception channel will fail, but creating a non-debug channel
will succeed.

### `ZX_EXCP_PROCESS_STARTING` and Job Debugger Channels

The `ZX_EXCP_PROCESS_STARTING` behaves differently than other exceptions.
It is only sent to job debugger exception channels, and is only sent to the
first found handler, essentially assuming `ZX_EXCEPTION_STATE_HANDLED`
regardless of actual handler behavior. This is also the only exception that
job debugger channels receive, making them a special-case handler for just
detecting new processes.

### Process Debugger First... and Possibly Again Later

In Zircon the process debugger exception channel is tried first. This is useful
for at least a few reasons:

- Allows "fix and continue" debugging, e.g. if a thread gets a segfault,
  the debugger user can fix the segfault and resume the thread without any
  non-debugger channels seeing the exception.
- Ensures debugger breakpoints get sent directly to the debugger without
  other handlers having to explicitly pass them along.

If an exception has ZX_EXCEPTION_STRATEGY_SECOND_CHANCE set and remains
unhandled after the process exception channel is tried, the process debugger
exception channel will be given a second chance. The utility of this lies in
the case in which a process listens to its own exceptions and uses that
information for correct functioning; in this case, it serves to have debugger
inspect in the event of a failed correction.

## Interaction with Task Suspension

Exceptions and thread suspensions are treated separately.
In other words, a thread can be both in an exception and be suspended.
This can happen if the thread is suspended while waiting for a response
from an exception handler. The thread stays paused until it is resumed
for both the exception and the suspension:

```cpp
zx_handle_close(exception);
zx_handle_close(suspend_token);
```

The order does not matter.

## Interaction with Task Kill

[`zx_task_kill()`] stops any exception handling on the task. If it is called on
a thread (or its parent process/jobs) while the thread is in an exception:

- the thread will stop waiting for the current exception handler
- no further exception handlers will receive the exception
- [`zx_exception_get_thread()`] and [`zx_exception_get_process()`] on the
  outstanding exception handle will continue to provide valid task handles
- [`zx_object_set_property()`] to set the exception's state will still return
  `ZX_OK`, though the state won't have any effect since the thread is no longer
  blocking on the handler

Additionally, a killed thread will still send a `ZX_EXCP_THREAD_EXITING`
exception (if a process debug handler is registered), but as above will not
wait for a response from the handler.

Although [`zx_task_kill()`] is generally asynchronous, meaning the thread may
not finish terminating by the time the syscall returns, it does synchronously
stop exception handling such that once it returns, closing an exception handle
will not resume the thread or pass the exception to another handler.

## Signals

[Signals](signals.md) are the core Zircon mechanism for observing state changes
on kernel objects (a channel becoming readable, a process terminating, an event
becoming signaled, etc).

Unlike exceptions, signals do not require a response from an exception handler.
On the other hand signals are sent to whomever is waiting on the thread's
handle, instead of being sent to the exception channel that could be bound to
the thread's process.

A common pattern in Zircon is to have a message loop that waits for signals on
one or more objects and handles them as they come in. To incorporate exception
handling into this pattern, use [`zx_object_wait_async()`] to wait for
`ZX_CHANNEL_READABLE` (and optionally `ZX_CHANNEL_PEER_CLOSED`) on the
exception channel:

```cpp
zx_handle_t port;
zx_status_t status = zx_port_create(0, &port);
// ... check status ...

// Start waiting on relevant signals on the exception channel.
status = zx_object_wait_async(exception_channel, port, kMyExceptionKey,
                              ZX_CHANNEL_READABLE | ZX_CHANNEL_PEER_CLOSED, 0);
// ... check status ...

// ... add other objects to |port| with wait_async() ...

while (1) {
  zx_port_packet_t packet;
  status = zx_port_wait(port, ZX_TIME_INFINITE, &packet);
  // ... check status ...

  if (packet.key == kMyExceptionKey) {
    if (packet.signal.observed & ZX_CHANNEL_READABLE) {
      // ... extract exception from |exception_channel| and process it ...

      // wait_async() is one-shot so we need to reload it to continue
      // receiving signals.
      status = zx_object_wait_async(
          exception_channel, port, kMyExceptionKey,
          ZX_CHANNEL_READABLE | ZX_CHANNEL_PEER_CLOSED, 0);
      // ... check status ...
    } else {
      // Got ZX_CHANNEL_PEER_CLOSED, task has terminated.
      zx_handle_close(exception_channel);
    }
  } else {
    // ... handle other objects added to |port| ...
  }
}
```

Note: There is both an exception and a signal for thread termination. The
`ZX_EXCP_THREAD_EXITING` exception is sent first. When the thread is finally
terminated the `ZX_THREAD_TERMINATED` signal is set.

## Comparison with Posix (and Linux)

This table shows equivalent terms, types, and function calls between
Zircon and Posix/Linux for exceptions and the kinds of things exception
handlers generally do.

Zircon                             | Posix/Linux
------                             | -----------
Exception/Signal                   | Signal
ZX_EXCP_*                          | SIG*
zx_task_create_exception_channel() | ptrace(ATTACH,DETACH)
zx_task_suspend()                  | kill(SIGSTOP),ptrace(KILL(SIGSTOP))
zx_handle_close(suspend_token)     | kill(SIGCONT),ptrace(CONT)
zx_handle_close(exception)         | kill(SIGCONT),ptrace(CONT)
zx_task_kill()                     | kill(SIGKILL)
N/A                                | kill(everything_else)
TBD                                | signal()/sigaction()
zx_port_wait()                     | wait*()
various                            | W*() macros from sys/wait.h
zx_exception_info_t                | siginfo_t
zx_exception_context_t             | siginfo_t
zx_thread_read_state()             | ptrace(GETREGS,GETREGSET)
zx_thread_write_state()            | ptrace(SETREGS,SETREGSET)
zx_process_read_memory()           | ptrace(PEEKTEXT)
zx_process_write_memory()          | ptrace(POKETEXT)

Zircon does not have asynchronous signals like `SIGINT`, `SIGQUIT`, `SIGTERM`,
`SIGUSR1`, `SIGUSR2`, and so on.

Another significant difference from Posix is that in Zircon a thread cannot
handle its own exceptions, since Zircon exception handling is a synchronous
operation driven by userspace rather than an asynchronous callback invoked by
the kernel.

## Examples

Zircon code that uses exceptions can be viewed for further examples, including:

- `system/core/svchost/crashsvc`: system-level crash handler
- `system/utest/exception`: exception unit tests
- `system/utest/debugger`: debugger-related functionality unit tests

## See Also

- [`zx_task_create_exception_channel()`]
- [`zx_exception_get_thread()`]
- [`zx_exception_get_process()`]
- [`zx_object_set_property()`]
- [`zx_port_wait()`]

[`zx_exception_get_process()`]: reference/syscalls/exception_get_process.md
[`zx_exception_get_thread()`]: reference/syscalls/exception_get_thread.md
[`zx_object_get_info()`]: reference/syscalls/object_get_info.md
[`zx_object_set_property()`]: reference/syscalls/object_set_property.md
[`zx_object_wait_async()`]: reference/syscalls/object_wait_async.md
[`zx_port_wait()`]: reference/syscalls/port_wait.md
[`zx_task_create_exception_channel()`]: reference/syscalls/task_create_exception_channel.md
[`zx_task_kill()`]: reference/syscalls/task_kill.md
