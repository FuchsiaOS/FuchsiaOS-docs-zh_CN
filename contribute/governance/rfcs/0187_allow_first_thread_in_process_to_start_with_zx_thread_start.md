<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0187" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View
the #} {# fully rendered RFCs at
https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

This RFC proposes allowing the first thread in a process to be started using
`zx_thread_start`, without changing the semantics of handle transfer.

## Motivation

Currently the first thread in a process must be started using
[`zx_process_start`][process-start], while subsequent threads must be started
using [`zx_thread_start`][thread-start]. These system calls differ in two ways:

  1. `zx_process_start` requires a handle to both the process and thread to
     start, while `zx_thread_start` only requires a handle to the thread.
  2. `zx_process_start` allows the caller to transfer a handle to the started
     process.

If a client does not explicitly start threads using `zx_thread_start` (e.g., it
uses a standard threading library) this restriction can force significant
workarounds.

Consider the case of [Starnix][starnix], which uses Rust's `std::thread` to
create and start new threads. When Starnix is creating the first thread in a
process it can't tell the underlying library to "create but don't start" the
thread, nor can it tell the library to use `zx_process_start` instead of
`zx_thread_start`.

Instead of forcing clients to work around such problems, this RFC proposes
allowing clients to use `zx_thread_start` for all threads in a process'
lifecycle.

It's important to note that this does not require changing the semantics of how
handles are transferred between processes. `zx_process_start` will remain the
only way to transfer a handle to the new process.

## Stakeholders

Who has a stake in whether this RFC is accepted? (This section is optional but
encouraged.)

_Facilitator:_ davemoore@google.com

_Reviewers:_

- abarth@google.com
- cpu@google.com
- mcgrathr@google.com
- travisg@google.com

_Consulted:_

_Socialization:_

This change has been discussed with various members of the kernel team over the
past year.

## Design

This RFC does not require changing interfaces; it only changes the semantics of
calling `zx_thread_start` on the first thread in a process.

## Implementation

The change allows `zx_thread_start(thread, thread_entry, stack, arg1, arg2)` to
be used to start any thread in a process. Calling `zx_thread_start` will be
equivalent to calling
`zx_process_start(process, thread, entry, stack, ZX_HANDLE_INVALID, arg2)`.

The only differences being that `zx_thread_start` does not require the explicit
process handle, and it allows callers to set `arg1` (which is reserved for the
`ZX_HANDLE_INVALID` in `zx_process_start`).

The implementation can be done in a single change, see [here][prototype].

## Performance

The standard kernel benchmarks will be run on the change.

## Ergonomics

This change improves ergonomics for clients by allowing them to use standard
thread libraries when creating all threads in a process.

## Security considerations

The transfer of handles between processes stays the same, and calling
`zx_process_start` after the first thread in the process has been started will
result in an error.

## Documentation

The documentation for `zx_process_start` and `zx_thread_start` will be updated
to reflect the new semantics.

[process-start]: /reference/syscalls/process_start.md
[prototype]: https://fuchsia-review.googlesource.com/c/fuchsia/+/707482
[starnix]: /contribute/governance/rfcs/0082_starnix.md
[thread-start]: /reference/syscalls/thread_start.md