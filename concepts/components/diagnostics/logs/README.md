<!--
# Logs
 -->
# 日志

<!--
Logs are time-ordered streams of diagnostic data emitted by Fuchsia programs.
They most commonly take the form of human-oriented text strings describing
changes to state in a subsystem.
 -->
日志是由 Fuchsia 程序发出的按时间排序的诊断数据流。其最常采用面向人类的文本字符串的形式，描述了子系统中的状态变化。

<!--
See [Recording] for information about how Fuchsia software writes logs.
 -->
要获取关于 Fuchsia 软件日志编写方式的信息，请参阅[记录][Recording]。

<!--
See [Viewing] for information about how to view the recorded logs.
 -->
要获取关于已记录日志查看方式的信息，请参阅[查看][Viewing]。

<!--
See [Attributing LogSink connections] for information about how Fuchsia identifies
the producer of each log message.
 -->
要获取关于 Fuchsia 对各日志消息产生者识别方式的信息，请参阅[归因 LogSink 连接][Attributing LogSink connections]。

<!--
## Contents
 -->
## 内容

<!--
[Log records][LogMessage] have a few pieces of metadata, mostly self-reported by
the programs that generate the logs. At minimum, messages have a timestamp and
string contents.
 -->
[日志记录][LogMessage]含有一些元数据，大部分由生成日志的程序自行报告。消息中至少具有时间戳和字符串内容。

<!--
If a message is written to the [`LogSink`] protocol it also has a severity, a
PID, a TID, a count of prior dropped logs, and a list of string tags.
 -->
消息如果写入 [`LogSink`] 协议，那么还会具有严重性、PID、TID、先前丢弃日志的计数以及字符串标签列表。

<!--
Unless a component's parent realm provides its own `LogSink`, diagnostics also
includes trusted [SourceIdentity] metadata for incoming log connections. This
information is used to provide a default tag for messages without tags.
 -->
除非组件的父领域提供自身的 `LogSink`，否则诊断还包括针对传入日志连接的可信[来源身份][SourceIdentity]（SourceIdentity）元数据。该信息用于为无标签消息提供默认标签。

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
目前，所有日志存储均按照先进先出（first-in-first-out，FIFO）的规则轮转，新消息覆盖旧消息。任何组件的消息都可以对其他消息进行推挤（roll out）。目前对于消息被推挤出缓冲区的时间的跟踪非常有限。

<!--
### Volatile
 -->
### 易失性

<!--
There are two in-memory stores for logs on a Fuchsia device:
 -->
Fuchsia 设备上的日志有两个内存存储区：

<!--
*   The "klog" or [debuglog], which is a [128kb buffer in the kernel].
*   The "syslog" which is a [4MB buffer in the Archivist] that runs as a
    singleton in `sys`.
 -->
*   “klog”，或称 [debuglog]，是[内核中的一个 128kb 缓冲区][128kb buffer in the kernel]。
*   “syslog”，是[归档器中的一个 4MB 缓冲区][4MB buffer in the Archivist]，在 `sys` 中作为单例（singleton）的运行。

<!--
Note: If you can't find some logs, see [Recording] logs to find out which of
these buffers is the intended point of transit for your message. See [Viewing]
logs once you know the location.
 -->
注意：如果您无法找到某些日志，请参阅[记录][Recording]日志，以查明这些缓冲区中您消息的预期传输点。在您知晓位置之后，请参阅[查看][Viewing]日志。

<!--
### Persistent
 -->
### 持久性

<!--
The [feedback data] component maintains a [persistent disk store] of messages
from the previous boot. These messages appear when running [`fx snapshot`].
 -->
[反馈数据][feedback data]组件维护来自上一次引导的消息的[持久性磁盘存储][persistent disk store]。这些消息在运行 [`fx snapshot`] 时出现。

[LogMessage]: https://fuchsia.dev/reference/fidl/fuchsia.logger#LogMessage
[`LogSink`]: https://fuchsia.dev/reference/fidl/fuchsia.logger#LogSink
[SourceIdentity]: https://fuchsia.dev/reference/fidl/fuchsia.sys.internal#SourceIdentity
[debuglog]: /reference/kernel_objects/debuglog.md
[128kb buffer in the kernel]: /zircon/kernel/lib/debuglog/debuglog.cc
[4MB buffer in the archivist]: /src/diagnostics/archivist/src/logs/mod.rs
[Recording]: /development/diagnostics/logs/recording.md
[Viewing]: /development/diagnostics/logs/viewing.md
[feedback data]: /src/developer/forensics/feedback_data
[persistent disk store]: /src/developer/forensics/feedback_data/system_log_recorder/system_log_recorder.h
[`fx snapshot`]: /src/developer/forensics/snapshot/README.md
[Attributing LogSink connections]: /concepts/components/diagnostics/logs/attribution.md
