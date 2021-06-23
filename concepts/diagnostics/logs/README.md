<!-- 
# Logs
 -->
# 日志

<!-- 
Logs are time-ordered streams of diagnostic data emitted by Fuchsia programs.
They most commonly take the form of human-oriented text strings describing
changes to state in a subsystem.

See [Recording] for information about how Fuchsia software writes logs.

See [Viewing] for information about how to view the recorded logs.

See [Attributing LogSink connections] for information about how Fuchsia identifies
the producer of each log message.
 -->
日志（log）是由 Fuchsia 程序发出的时间顺序的诊断数据流。它们最常采用面向人类的文本字符串的形式，描述子系统中的状态变化。

参阅[记录][Recording]以获取关于 Fuchsia 软件如何撰写日志的更多信息。

参阅[查看][Viewing]以获取关于如何查看记录的日志的更多信息。

参阅[添加 LogSink 连接属性][Attributing LogSink connections]以获取关于 Fuchsia 如何辨认每条日志消息生成者的更多信息。

<!-- 
## Contents
 -->
## 内容

<!-- 
[Log records][LogMessage] have a few pieces of metadata, mostly self-reported by
the programs that generate the logs. At minimum, messages have a timestamp and
string contents.

If a message is written to the [`LogSink`] protocol it also has a severity, a
PID, a TID, a count of prior dropped logs, and a list of string tags.

Unless a component's parent realm provides its own `LogSink`, diagnostics also
includes trusted [SourceIdentity] metadata for incoming log connections. This
information is used to provide a default tag for messages without tags.
 -->
[日志记录][LogMessage]（log record）拥有一些元数据，它们多是由生成日志的程序自行报告的。消息（即前文的“记录”）至少拥有时间戳和字符串内容。

如果一条消息被写入了 [`LogSink`]（“日志槽”）协议，那么它还有一个严重性（severity）、一个PID、一个TID、一些先前的丢弃的日志和一个字符串标签的列表。

除非组件的父界（realm）提供了自己的 `LogSink`，否则诊断还会包含用于传入日志连接的的可信 [SourceIdentity]（源身份）元数据。该信息用于为无标签信息提供默认标签。

<!-- 
## Storage
 -->
## 存储

<!-- 
Currently all log stores are rotated on a first-in-first-out (FIFO) basis, with
newer messages overwriting older ones. Messages from any component can roll out
messages from any other component. There is currently very limited tracking of
when messages are rolled out of their buffers.
 -->
目前，所有日志的存储按照先进先出（FIFO，first-in-first-out）的方式轮换，新消息会覆盖掉旧消息。来自任何组件的消息可以推出来自任何其他组件的消息。当前对于消息何时被推出其缓冲区的追踪非常有限。

<!-- 
### Volatile
 -->
### 易失

<!-- 
There are two in-memory stores for logs on a Fuchsia device:

*   The "klog" or [debuglog], which is a [128kb buffer in the kernel].
*   The "syslog" which is a [4MB buffer in the Archivist] that runs as a
    singleton in `sys`.

Note: If you can't find some logs, see [Recording] logs to find out which of
these buffers is the intended point of transit for your message. See [Viewing]
logs once you know the location.
 -->
Fuchsia 设备上有两个针对日志的内存存储区（in-memory store）：

*   “klog”，或称 [debuglog]（调试日志），一个[内核中的 128kb 缓冲区][128kb buffer in the kernel]。
*   “syslog”，一个在 `sys` 中以单例（singleton）运行的[归档器中的 4MB 缓冲区][4MB buffer in the Archivist]。

注意：如果您无法找到某些日志，那么可以参阅[记录][Recording]日志，以查明哪个缓冲区是您消息预期的传输点。当您知道位置后，请参阅[查看][Viewing]日志。

<!-- 
### Persistent
 -->
### 持久

<!-- 
The [feedback data] component maintains a [persistent disk store] of messages
from the previous boot. These messages appear when running [`fx snapshot`].
 -->
[反馈数据][feedback data]（feedback data）组件维护来自上次启动消息的[持久磁盘存储区][persistent disk store]。这些消息在运行 [`fx snapshot`] 时出现。

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
[Attributing LogSink connections]: /docs/concepts/diagnostics/logs/attribution.md
