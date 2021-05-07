{% set rfcid = "RFC-0079" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!--
*** This should begin with an H2 element (for example, ## Summary).
-->

## Summary

This document proposes an update to the mechanism for detecting dropped messages
from the kernel's debuglog object.

## Background and motivation

The kernel debuglog subsystem is a simple logging facility that enables user
mode programs to read and write log messages. Logically, this system provides a
single FIFO log buffer that can be written to or read from by multiple writers
or readers.

The debuglog can be lossy. Assuming the readers can keep up with the write rate,
all readers will see all messages written by all writers in the order in which
they were written. However, if a reader is slow and cannot keep up, it will miss
messages.

Messages can be written to the debuglog by user mode programs via
`zx_debuglog_write` or by the kernel via `printf`. Messages can be read via
`zx_debuglog_read`. Additionally, the kernel has a specialized thread, known as
the debuglog_dumper, for reading messages from the debuglog and writing them to
the debug serial port.

The debuglog buffer has a fixed capacity. When that capacity is reached, the
least recently written message(s) may be dropped to make room for a new
message. Any readers that haven't "caught up" will never see the dropped
message(s).

Knowing that a log is complete allows one to reason about the absence of certain
events so detecting dropped log messages is an important feature of a log
system. Debuglog readers need to be able to detect when log messages have been
dropped.

Currently, the debuglog provides a mechanism for readers to detect when and how
many bytes of log data have been dropped, the `uint32_t rolled_out` field of
`zx_log_record_t`.

After reading a `zx_log_record_t` with `zx_debuglog_read`, the `rolled_out`
field will contain the number of bytes of log messages that have been dropped
from the debuglog since that reader's last successful read. The value includes
both the bytes of the dropped log headers and the bytes of the dropped log
bodies.

The `rolled_out` mechanism is implemented by having each reader maintain a
pointer into the debuglog buffer that points to the next message that it has not
yet read. The debuglog maintains a write pointer that points to the location
where the next message will be written. If a reader notices that the write
pointer has passed its read pointer, then it knows it has missed one or more
messages. By subtracting the pointer values, a reader can determine how many
bytes of log data (including headers) it has missed.

The `rolled_out` mechanism is currently unused.

## Proposal

This RFC proposes...

1. Replacing byte oriented data loss detection with record oriented detection.

    To more closely match the expectations of debuglog readers, in particular
    the debuglog_dumper, the existing byte oriented `rolled_out` mechanism will
    be replaced with a per-record sequence number that can be used to detect
    data loss (gaps in the sequence).

    The debuglog_dumper must write each message it reads to the debug serial
    port. Because the serial port may not be particularly fast, the
    debuglog_dumper often cannot keep up with the debuglog, which results in
    dropped messages. When this happens, we'd like to print a message to the
    serial port indicating that data loss has occurred and how many messages
    were lost, similar to Linux's `printk` output.

2. Eliminating the possibility of undetected data loss using a 64-bit value.

    Because the `rolled_out` field is 32-bits in size and counts bytes rather
    than records, if 4GB of log data is written in between two calls to
    `zx_debuglog_read` it is possible to overflow the value, which could result
    in undetected data loss. This is unlikely to occur in practice, but it would
    be nice to entirely eliminate the possibility. If we were to replace the
    32-bit byte sequence field with a 32-bit per-record sequence field the
    amount of data required to create an overflow grows to approximately
    128GB. By using a 64-bit sequence field, we can entirely ignore the
    possibility of overflow even at very high logging rates.

3. Enabling future implementation optimizations.

    There are some potential future optimizations that would depend on allowing
    multiple debuglog readers to "share" a single `zx_log_record_t`.

    Absent slow readers, all debuglog readers should see the exact same log data
    in the same order. With the exception of `rolled_out`, all fields of
    `zx_log_record_t` are fixed at the time the record is written to the debug
    log. `rolled_out` is different in that it's computed per-reader, at the time
    the record is read out of the debuglog. If we had another way to detect data
    loss without using fields that could vary per-reader, we could enable some
    potential future optimizations where a single record is shared among all
    readers.

## Design

The debuglog assigns a 64-bit sequence number, starting with 0, to each record
when it's written to the debuglog.

Each record's sequence number will be exactly one greater than the preceding
record. Remove `zx_log_record_t`'s `rolled_out` field and replace it with the
record's sequence number, `uint64_t sequence`.

Callers of `zx_debuglog_read` can then detect gaps in the sequence and calculate
how many messages were dropped.

## Implementation

`zx_debuglog_read` and `zx_log_record_t` are not used out of tree. While the
full blown Fuchsia Large Scale Changes (LSC) process is not required, an FYI LSC
bug will be filed and the implementation will be completed in phases.

The `rolled_out` field is unused, but the containing struct, `zx_log_record_t`
is used in a few places within fuchsia.git. Care needs to be taken to not break
existing code. `zx_log_record_t` is not used out of tree.

The syscall definition and documentation of `zx_debuglog_read` don't actually
specify that it returns a `zx_log_record_t`. Instead they specifies a `void*`
and `size_t` and callers must know to cast or "overlay" a `zx_log_record_t` on
top of the result. Casting to `zx_log_record_t*` is error prone so the `void*`
syscall parameter will be changed to `zx_log_record_t*` and callers will be
updated.

There is currently no Rust equivalent of `zx_log_record_t` and Rust callers use
hard coded offsets to access the fields so changing the size or offsets of its
fields can silently break these callers. As part of the implementation we'll
create a Rust equivalent of `zx_log_record_t` and update Rust callers to use it.

The steps are as follows:

1. Add a 64-bit sequence number to the private, internal representation of
   debuglog messages (`dlog_header_t`).

2. Change debuglog consumers to use `zx_log_record_t` (or language equivalent)
   rather than hard coded field offsets. In particular, create a Rust equivalent
   type and update Rust code to use it.

3. Change `zx_debuglog_read`'s `void*` parameter to `zx_log_record_t*`.

4. Change `zx_log_record_t`'s `rolled_out` field to a zero-filled `unused`
   field.

5. Replace `zx_log_record_t`'s `unused` field with `uint64_t sequence` and
   ensure no implicit struct padding is created. Do the same for all
   `zx_log_record_t` equivalent types, regardless of language.

6. Update `zx_debuglog_read` documentation to explain how callers may use the
   new sequence field to detect data loss.

Steps 1, 2, and 3 will each get their own CL. Steps 4, 5, and 6 will occur in a
single CL.

## Performance

### Runtime cost of managing the sequence counter

Debuglog operations are performed while holding a lock so we can use a regular
`uint64_t` value to generate the sequence. No measurable performance impact is
anticipated.

### Size impact of per record sequence value

Both `zx_log_record_t` and `dlog_header_t`, the kernel's private record
implementation, will grow in size. `zx_log_record_t`'s 32-bit `rolled_out` field
will be replaced by a 64-bit record sequence field, yielding a net gain of 4
bytes.

The `dlog_header_t` size change is more interesting because it's the native form
in which log records are stored in the FIFO. `dlog_header_t` is 32 bytes in size
and has no `rolled_out` field so its net gain will be the full 8 bytes. FIFO
space is limited in the debuglog object so increasing each log record by 8 bytes
will reduce the maximum number of records that may be stored in the FIFO and
also reduce the maximum message size (from 224 bytes to 216).

The FIFO can store 128KB of headers and messages. A sampling of messages
indicates the average size is about 100 bytes. Assuming this average size, with
a 32 byte header, the FIFO can store approximately 971 messages. With a per
record sequence number, the number is reduced to approximately 917.

## Security considerations

The proposal does not alter the security of the system. Debuglog readers are
privileged components. Absent data loss, a debuglog reader could already
synthesize the log record sequence with perfect accuracy.

## Privacy considerations

No privacy impact.

## Testing

In-kernel unit tests will verify the underlying debuglog implementation and
debuglog core tests will verify the behavior observable at the syscall layer.

## Documentation

`zx_log_record_t`'s documentation will be updated.

## Drawbacks, alternatives, and unknowns

### Do nothing

Because `rolled_out` is not yet used, a relatively small amount of engineering
effort is required to implement the proposal at this time. Once downstream code
makes use of `rolled_out` it will be more expensive to implement this or a
similar proposal.

The issue of 32-bit wraparound could be somewhat mitigated by documenting the
semantics of `UINT32_MAX` to mean "`UINT32_MAX` or more". Or we could change
`rolled_out`'s type to `uint64_t`.

### Record sequence and byte sequence

If space were free, we could put both a record sequence value and a byte
sequence value in each `zx_log_record_t`. Debuglog readers could then measure
the data loss in either number of records or number of bytes lost. However, that
would further increase the size of the `dlog_header_t` and it's not clear we
have a use case for bytes dropped.

## Prior art and references

Linux's `printk` reports the number of messages dropped/suppressed rather than
the bytes of log data dropped/suppressed.
