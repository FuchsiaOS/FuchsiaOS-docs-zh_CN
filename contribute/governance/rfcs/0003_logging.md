{% set rfcid = "RFC-0003" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- ## Summary -->
## 总览

<!-- 
[Logging](/concepts/components/diagnostics/logs/README.md) is used extensively throughout
Fuchsia's code, but messages are logged with inconsistent severities. This
diminishes the value of information in the logs. Below we propose semantic
meaning for different log severity levels, and implications for logging at
certain severities in various environments.
 -->
[日志](/concepts/components/diagnostics/logs/README.md) 被广泛地使用在 Fuchsia 的代码，但是消息记录的严重程度是不一致的。这会减少信息的在日志中。下面我们提出了不同日志严重程度的语义，以及在不同环境下以特定严重程度进行日志记录的意义。

<!-- ## Motivation and problem statement -->
## 动机和问题陈述

<!-- 
Logs are the most common way to send a diagnostics signal about a change in
the internal state of running software. They offer valuable information that can
be used to detect and correct faults in software and products.
 -->
日志是发送关于运行软件内部状态变化的诊断信号的最常用方法。它们提供了有价值的信息，可用于检测和纠正软件和产品中的错误。

<!-- 
Historically, logs contain a free-text message, as well as an enumerated
severity value. The severity value offers an indication of how important the
message is, differentiating the signal from the noise. However, the application
of severity levels is made inconsistently throughout Fuchsia code and its
dependencies, resulting in logs that are difficult to use both in manual
troubleshooting and in automated analysis.
 -->
过去，日志包含自由文本消息，以及枚举的严重性值。严重性值表示信息的重要程度，将信号与噪声区分开来。然而，在 Fuchsia 代码及其依赖项中，严重级别的应用并不一致，导致在手动故障排除和自动化分析中都难以使用日志。

<!-- 
By providing common guidelines for how to use log severities, and by creating
processes that attach consequences to certain ways of using the logs, we hope to
create a virtuous cycle that will generate more diagnostics value out of logs
while also improving the signal-to-noise ratio. For instance, we would like the
frequency of ERROR log entries to be one of the proxies to system stability.
 -->
通过提供如何使用日志严重性的通用指南，并通过创建与使用日志的特定方式相关的过程，我们希望创建一个良性循环，在提高信噪比的同时，从日志中产生更多的诊断价值。例如，我们希望 ERROR 日志条目的频率是系统稳定性的代理之一。

<!-- ## Implementation -->
## 执行

<!-- ### Log severity levels -->
### 日志严重等级

<!-- Fuchsia supports a number of standard log severity levels: -->
Fuchsia 支持多种标准的日志级别：

#### FATAL

<!-- 
This indicates an unrecoverable error and that the component should
self-terminate shortly after logging the message. This typically indicates that
a core invariant in the system has been violated and proceeding further could
lead to data corruption or security vulnerabilities.
 -->
这表示一个不可恢复的错误，并且组件应该在记录消息后不久自动终止。这通常表明系统中的核心不变量已被破坏，进一步可能导致数据损坏或安全漏洞。

<!-- 
This log level should be used sparingly. This log level may indicate a hardware
issue or software bug that needs to be fixed. Since the given message will be
the final message logged before the component self-terminates, its contents
should indicate the reason for termination.
 -->
这个日志级别应该谨慎地使用。此日志级别可能指示需要修复的硬件问题或软件错误。由于给定的消息将是组件自我终止前记录的最后一条消息，因此它的内容应该表明终止的原因。

#### ERROR

<!-- 
This log level indicates an undesired event has occurred that the program can
recover from. An ERROR log entry's appearance in the logs is an indication of
incorrect program behavior that needs to be fixed. Developers should strive for
a one-to-one correspondence between a bug in the bug tracker and the ERROR log
entry. In other words, for every unique bug, there should be an ERROR log entry,
and for every ERROR log entry, there should be a separate bug. Developers should
attempt to maintain this correspondence in code where possible. Note that the
bug might not be in the code issuing the log statement. The bug may be in one of
the callers or one of its dependencies.
 -->
这个日志级别表明发生了一个不希望发生的事件，程序可以从中恢复。ERROR 日志条目在日志中的出现，表明了不正确的程序行为需要修复。开发人员应该努力使错误跟踪器中的错误与 ERROR 日志条目之间保持一对一的对应关系。换句话说，对于每一个独特的 bug，都应该有一个 ERROR 日志条目，而对于每一个 ERROR 日志条目，都应该有一个单独的 bug。开发人员应尽可能在代码中保持这种对应关系。请注意，错误可能不在发布日志语句的代码中。该错误可能是在某个调用者或其依赖关系中。

<!-- 
This log level and above will serve as a signal to system stability metrics.
Additionally, this log level and above will typically be present in bug reports.
This log message will be consumed by people outside the team that introduced
the message and so it is expected to provide sufficient context to guide the
reader toward the root cause of the problem.
 -->
这个日志级别及以上将作为系统稳定性指标的一个信号。此外，这个日志级别及以上通常会出现在错误报告中。该日志消息将被引入该消息的团队之外的人员使用，因此希望它提供足够的上下文来指导读者找到问题的根本原因。

#### WARNING

<!-- 
This log level indicates an unexpected event has occurred within the normal
operation of the system. These unexpected events may be environmental and
therefore outside the control of the system itself. For example, a lost network
connection may be logged as a WARNING or invalid input that prevents a program
from proceeding further may be logged as WARNING. This log level can be used to
find the root cause that led to a problem (typically an ERROR log entry). This
log level may serve to make salient that an uncommon code path was taken as a
result of the aforementioned unexpected event.
 -->
系统正常运行中发生了不可预知的事件。这些意外事件可能与环境有关，因此不受系统本身的控制。例如，丢失的网络连接可能被记录为 WARNING，或者阻止程序进一步进行的无效输入可能被记录为 WARNING。此日志级别可用于查找导致问题的根本原因（通常是 ERROR 日志条目）。这个日志级别可以表明，由于前面提到的意外事件而采用了不常见的代码路径。

#### INFO

<!-- 
This is typically the lowest log level present in bug reports. This indicates
that a noteworthy state change has occurred in a program. This log level will be
used to indicate the context that led to a problem (typically an ERROR log
entry). As with higher log levels, these logs will be consumed by
other teams for diagnosis and so context is critical.
 -->
这通常是错误报告中出现的最低日志级别。这表明程序中发生了值得注意的状态变化。此日志级别将用于指示导致问题的上下文（通常是 ERROR 日志条目）。与较高的日志级别一样，这些日志将被其他团队用于诊断，因此上下文至关重要。

<!-- 
Long-lasting states should be reported using
[Component Inspection](/development/diagnostics/inspect).
Such conditions should not be logged if they have not changed. For example,
instead of logging "[INFO] no configs found" at a fixed interval, the Inspect
data could contain a flag "configs_found = false," or even "config_count = 0."
 -->
使用 [组件检查](/development/diagnostics/inspect) 报告持久状态。如果这些条件没有更改，就不应该记录它们。例如，Inspect 数据可以包含一个标志“configs_found = false”，甚至“config_count = 0”，而不是以固定的间隔记录“[INFO] no configs found”。

#### DEBUG

<!-- 
This log level is typically used in engineering environments, such as during
tests or while reproducing an issue in a commit queue bot. Logs at severity
level DEBUG and below are typically not collected in bug reports, so there
should be no expectation to receive DEBUG and lower logs in bug reports. This
log level is typically more verbose than INFO and helps individual teams better
understand the state of the system they're developing.
 -->
此日志级别通常用于工程环境，例如在测试期间或在提交队列机器人中重现问题时。错误报告中通常不会收集严重级别为 DEBUG 及以下的日志，因此不应该期望在错误报告中收到 DEBUG 和更低级别的日志。这个日志级别通常比 INFO 更详细，可以帮助个人团队更好地了解他们正在开发的系统的状态。

#### TRACE

<!-- 
This log level will typically be used by individual teams and perhaps in
the commit queue and continuous integration. This log level is typically used to
indicate that a stage in a multistage process has completed.
 -->
这个日志级别通常被个人团队使用，也可能用于提交队列和持续集成中使用。这个日志级别通常用于指示多阶段流程中的某个阶段已经完成。

<!-- ### Consequences of logging -->
### 记录的结果

<!-- 
Having established guidelines for choosing log severity levels, we propose
several ways in which we automatically treat logs based on their severity.
 -->
在建立了选择日志严重程度的准则后，我们提出了几种根据日志严重程度自动处理日志的方法。

<!-- #### Logs as in-field analytics -->
#### 日志作为现场分析

<!-- 
We choose to treat the presence of an ERROR log entry as an indication of a
software bug that needs to be fixed. It's safe to assume that a software author
would be interested in knowing that their software is logging errors outside
their local environment. Therefore we will count the presence of ERROR and FATAL
log entries broken down by source component instance, report them via Cobalt,
and track these counters as an in-field stability metric.
 -->
我们选择将 ERROR 日志条目的出现视为软件错误需要修复的指示。可以肯定的是，软件作者会有兴趣知道他们的软件在本地环境之外错误记录。因此，我们将对源组件实例分解的 ERROR 和 FATAL 日志条目进行计数，通过 Cobalt 报告，并跟踪这些计数器作为现场稳定性指标。

<!-- 
To view an overview of logging statistics on a device, use the following
command:
 -->
使用如下命令查看设备的日志统计信息：

```
cs --log-stats
```

<!-- 
Component Stats (cs) provides a log stats mode that parses Archivist's inspect
tree to provide a summary view of logs generated per component. This view
enables engineers to see the types of logs their component generates and the
frequency with which it generates various logs relative to other components.
 -->
Component Stats (cs) 提供了一个日志统计模式，该模式解析 Archivist 的检查树，以提供每个组件生成的日志摘要视图。这个视图使工程师能够看到他们的组件生成的日志类型，以及相对于其他组件生成各种日志的频率。

<!-- #### Logs as (un)expected behavior under test -->
#### 记录测试中预期或不预期的行为

<!-- 
It's safe to assume that a software author would be interested in knowing that
their software is logging errors under controlled conditions set during a test,
when such behavior is not expected. Therefore we will introduce behavior into
the test runtime that will cause tests to fail if unexpected ERROR logs are
observed within the test realm.
 -->
可以认为，软件作者会有兴趣知道，测试期间他们的软件在设定的控制条件下的错误记录，而这种行为是不被期望的。因此，如果在测试领域中观察到意外的 ERROR 日志，我们将在测试运行时中引入将导致测试失败的行为。

<!-- 
The test runtime behavior described above has been implemented, tested, and
[documented](/concepts/testing/v1_test_component.md#restricting_log_severity).
 -->
上面描述的测试运行时行为已经被实现、测试，和 [证明](/concepts/testing/v1_test_component.md#restricting_log_severity)。

<!-- ## Security Consideration -->
## 安全考虑

<!-- 
* Logging with FATAL severity typically indicates that a core invariant in the
system has been violated and proceeding further could lead to data corruption or
security vulnerabilities. Termination is the safest course of action in this case.
 -->
* 以 FATAL 严重程度记录通常表明系统中的一个核心不变因素被违反了，继续下去可能导致数据损坏或安全漏洞。在这种情况下，终止是最安全的行动方案。

<!-- 
* The component framework can securely identify the source of a log to a component
level. Within a component, modules will likely be self-attested and so cannot be
trusted to the same degree as component-level identification.
 -->
* 组件框架可以安全地将日志的来源识别到组件级别。在一个组件内，模块很可能是自我证明的，因此不能像组件级识别那样被信任。

<!-- ## Privacy Consideration -->
## 隐私考虑

<!-- 
Any logging analytics should avoid exposing any Personally Identifiable
Information (PII). For example, we should not expose the full set of components
on a user's device in off-device metrics nor the full text of any log messages
that might contain any PII.
 -->
任何日志分析都应该避免暴露任何个人可识别信息（PII）。例如，我们不应该在设备外数据记录中暴露用户设备上的全部组件，也不应该暴露可能包含任何 PII 的任何日志消息全文。

<!-- ## Documentation -->
## 文档

<!-- 
We intend to publish these logging guidelines as a [Fuchsia Development
Guide](/development) as well.
 -->
我们打算将这些日志指南作为 [Fuchsia开发指南](/development)。

<!-- ## Drawbacks, alternatives, and unknowns -->
## 缺点、替代方案和未知

<!-- 
The proposed changes above create a system of incentives and disincentives to
log in certain ways. The intention is to get more value out of logs - notice the
important log entries by giving them appropriately high severity, while also
reducing the number of less significant log entries with overly high severity.
 -->
上述建议的修改创造了一个激励和抑制以特定方式记录的系统。这样做的目的是为了从日志中获得更多的价值——通过给重要的日志条目赋予适当的高严重性来注意它们，同时也减少严重程度过高的不太重要的日志条目的数量。

<!-- 
We consider the risk that these incentives will promote unwanted behaviors, such
as avoiding ERROR log entries entirely (even under conditions that call for it).
We believe that the motivation to create better software will drive developers
to make responsible decisions in how they react to the proposed changes.
 -->
我们考虑到这些激励会促进不必要的行为风险，例如完全避免 ERROR 日志条目（即使在需要它的条件下）。我们相信，创建更好的软件的动机将推动开发人员做出负责任的决定，决定他们如何对建议的更改做出反应。

<!-- 
We will test the above hypothesis by monitoring developer behavior via analysis
of bug reports and occasional interviews with team members, and course-correct
as necessary.
 -->
我们将通过分析错误报告和偶尔与团队成员面谈来监控开发人员的行为，并在必要时进行修正，从而测试上述假设。


<!-- ## Prior art and references -->
## 现有技术和参考文献

<!-- 
The [Google Testing Blog](https://testing.googleblog.com/2013/06/optimal-logging.html)
provides some basic guidelines on logging levels.
 -->
[谷歌测试博客](https://testing.googleblog.com/2013/06/optimal-logging.html) 提供关于日志级别的一些基本指南。
