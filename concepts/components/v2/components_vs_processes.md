# Components vs. processes

This document explains how the concept of components differs from processes and
related concepts.

The Zircon kernel defines [processes] and other [task objects] that are common
in modern operating systems. The abstraction of [component instances] sometimes
correlates with Zircon task abstractions, but not always.

## Examples

The relationship between components and Zircon tasks differs, often as defined
by [component runners], which implement strategies for launching component
instances.

-   [ELF Runner] launches components by creating a new [job] that contains a
    process that's created from a given executable file in ELF format.
-   Dart Runner launches a new Dart isolate in a Dart Virtual Machine. A Dart
    VM is implemented as a process that can host one or more Dart isolate.
    Dart isolates execute on [threads], but don't necessarily have an
    assigned thread (this is a VM implementation detail).
-   Web runner can launch one or more web pages as components, and host them
    the same web engine container or in separate containers per its isolation
    policy. Web pages are typically isolated by being hosted in separate
    processes.

[processes]: /docs/reference/kernel_objects/process.md
[task objects]: /docs/reference/kernel_objects/objects.md#tasks
[component instances]: /docs/concepts/components/v2/topology.md#component-instances
[component runners]: /docs/concepts/components/v2/capabilities/runners.md
[ELF Runner]: /docs/concepts/components/v2/elf_runner.md
[job]: /docs/reference/kernel_objects/job.md
[threads]: /docs/reference/kernel_objects/thread.md
