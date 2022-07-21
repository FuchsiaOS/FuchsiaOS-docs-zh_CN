{% set rfcid = "RFC-0007" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary
<!--  -->
<!-- In the past, `zx_task_kill` allowed usermode to kill individual threads. However,
killing individual threads encourages bad practices and has a high chance of leaving
the process in a bad state. For this reason, the ability to kill individual threads
should be removed. -->
过去，`zx_task_kill` 允许在用户态杀死单个线程。 但是，杀死单个线程会为不良做法提供支持，并且很有可能使进程处于不良状态。 出于这个原因，应该删除杀死单个线程的能力。

## Motivation and problem statement

<!-- There is no reasonable use for usermode to kill individual threads. Exposing such facility
encourages bad practices. -->
用户态没有合理的用途来杀死单个线程。 暴露此类能力会为不良做法提供支持。

<!-- On Fuchsia, like other systems, killing a thread is done asynchronously; for running threads there
is no practical way to determine the exact place where it is safe to terminate a thread. For a
blocked (waiting)  thread, the safer and often simple solution is to add logic so upon wakeup the
thread exits by itself. -->
在 Fuchsia 上，和其他系统一样，杀死一个线程是异步完成的； 对于正在运行的线程，没有实用的方法来确定可以安全终止线程的确切位置。 对于阻塞（等待）状态的线程，通常更安全且简单的解决方案是添加逻辑，以便在唤醒后线程自行退出。(todo)

<!-- Dangers killing a thread -->
杀死线程的危险

<!-- * Locks can be left acquired, including global locks like ones controlling the heap.
* Memory can be leaked. At the very least the thread stack, but often many other pieces.
* Runtime left in an inconsistent state. This is at least true for the C and Go runtime.
* Killing a thread in its way to a syscall leaves the process in an unknown state. Kernel is
  fine but the process does not have a way to know what happened and what did not happen.
* Defeats RAII wrappers and automatic cleanup. In fact, it defeats most guarantees from the high
  level languages Fuchsia uses. -->
* 锁可以被保持获取，包括像控制堆这样的全局锁。
* 内存可能会泄漏。 至少线程堆栈可能泄露，但通常还有许多其他部分的内存会泄露。
* 运行时处于不一致的状态。 至少对于 C 和 Go 运行时来说是这样。
* 以系统调用的方式杀死一个线程会使进程处于未知状态。 内核没有影响，但该进程无法知道发生了什么以及没有发生什么。
* 破坏 RAII 包装器和自动清理。 事实上，它破坏了 Fuchsia 使用的高级语言的大多数保证机制。

## Design

<!-- The following syscall will fail with `ZX_ERR_NOT_SUPPORTED` when passed a handle to a thread: -->
当将句柄被传递给线程时，以下系统调用将失败并返回`ZX_ERR_NOT_SUPPORTED`：

```
zx_status_t zx_task_kill(zx_handle_t handle);
```

<!-- Processes and jobs will still be killable as normal. -->
进程和作业仍然可以正常地被杀死。

## Implementation

<!-- Luckily, thread killing is not used very much in Fuchsia. The only use cases are in test code
that checks that a thread hits a specific exception. This code is going to be updated so that
the excepting thread exits itself after the exception is handled. For code where the exception
is unrecoverable, the excepting thread's instruction pointer can be set directly to
zx_thread_exit or the runtime's thread exit function before the thread resumes. These tests
may still leak what the excepting thread had stored on the heap, but the runtime is in
a better state, and the leaks will be collected when the test's process exits. -->
幸运的是，Fuchsia 中并没有太多使用到杀死线程。 唯一的用例是在检查线程是否遇到特定异常的测试代码中。 此代码将被更新，以便异常线程在异常被处理后自行退出。 对于异常无法恢复的代码，可以在线程恢复前，将异常线程的指令指针直接设置为 zx_thread_exit 或运行时的线程退出函数。 这些测试可能仍然会泄漏异常线程存储在堆上的内容，但运行时会处于一个更好的状态，并且会在测试的进程退出时收集泄漏的内容。

## Performance

N/A

## Security considerations

N/A

## Privacy considerations

N/A

## Testing

<!-- The zircon core-tests will be updated to ensure that the zx_task_kill syscall behaves as intended.
Some amount of static analysis can be done to find call sites of zx_task_kill that are passed
threads. -->
Zircon 核心测试将被更新，以确保 zx_task_kill 系统调用按预期运行。可以进行一些静态分析来找到传递线程的 zx_task_kill 的调用点。

<!-- The full Fuchsia Large Scale Change (LSC) process will be followed to ensure this change is
properly tested. -->
将遵循完整的 Fuchsia Large Scale Change (LSC) 流程，以确保这一变化被正确测试。

## Documentation

<!-- The documentation for [zx_task_kill](/reference/syscalls/task_kill.md) will be updated to
reflect that threads are not killable. -->
[zx_task_kill](/reference/syscalls/task_kill.md) 的文档将被更新以反映线程不可杀死。

## Drawbacks, Alternatives, and Unknowns

<!-- The alternative to this proposal is the current status quo, which is to allow threads to be
killed. Threads have been killable for the entire history of Fuchsia, but there has not been
any acceptable use cases where programs have relied on this behavior. For this reason,
we believe that thread killing can be safely removed. -->
该提议的替代方案是当前的现状，即允许线程被杀死。 在 Fuchsia 的整个历史中，线程都是可以被杀死的，但是没有任何可接受的用例表明程序依赖于这种行为。 出于这个原因，我们相信可以安全地删除线程杀死功能。

## Prior art and references

* [Windows Vista tries to remove
TerminateThread](https://devblogs.microsoft.com/oldnewthing/20150814-00/?p=91811)
