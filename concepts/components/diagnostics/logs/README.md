# Logs

Logs are time-ordered streams of diagnostic data emitted by Fuchsia programs.
They most commonly take the form of human-oriented text strings describing
changes to state in a subsystem.

See [Recording] for information about how Fuchsia software writes logs.

See [Viewing] for information about how to view the recorded logs.

See [Attributing LogSink connections] for information about how Fuchsia identifies
the producer of each log message.

## Contents

[Log records][LogMessage] have a few pieces of metadata, mostly self-reported by
the programs that generate the logs. At minimum, messages have a timestamp and
string contents.

If a message is written to the [`LogSink`] protocol it also has a severity, a
PID, a TID, a count of prior dropped logs, and a list of string tags.

Unless a component's parent realm provides its own `LogSink`, diagnostics also
includes trusted [SourceIdentity] metadata for incoming log connections. This
information is used to provide a default tag for messages without tags.

## Storage

Currently all log stores are rotated on a first-in-first-out (FIFO) basis, with
newer messages overwriting older ones. Messages from any component can roll out
messages from any other component. There is currently very limited tracking of
when messages are rolled out of their buffers.

### Volatile

There are two in-memory stores for logs on a Fuchsia device:

*   The "klog" or [debuglog], which is a [128kb buffer in the kernel].
*   The "syslog" which is a [4MB buffer in the Archivist] that runs as a
    singleton in `sys`.

Note: If you can't find some logs, see [Recording] logs to find out which of
these buffers is the intended point of transit for your message. See [Viewing]
logs once you know the location.

### Persistent

The [feedback data] component maintains a [persistent disk store] of messages
from the previous boot. These messages appear when running [`fx snapshot`].

[LogMessage]: https://fuchsia.dev/reference/fidl/fuchsia.logger#LogMessage
[`LogSink`]: https://fuchsia.dev/reference/fidl/fuchsia.logger#LogSink
[SourceIdentity]: https://fuchsia.dev/reference/fidl/fuchsia.sys.internal#SourceIdentity
[debuglog]: /docs/reference/kernel_objects/debuglog.md
[128kb buffer in the kernel]: /zircon/kernel/lib/debuglog/debuglog.cc
[4MB buffer in the archivist]: /src/diagnostics/archivist/src/logs/mod.rs
[Recording]: /docs/development/diagnostics/logs/recording.md
[Viewing]: /docs/development/diagnostics/logs/viewing.md
[feedback data]: /src/developer/forensics/feedback_data
[persistent disk store]: /src/developer/forensics/feedback_data/system_log_recorder/system_log_recorder.h
[`fx snapshot`]: /src/developer/forensics/snapshot/README.md
[Attributing LogSink connections]: /docs/concepts/components/diagnostics/logs/attribution.md
