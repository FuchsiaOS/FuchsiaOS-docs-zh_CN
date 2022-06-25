{% set rfcid = "RFC-0003" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

[Logging](/docs/concepts/components/diagnostics/logs/README.md) is used extensively throughout
Fuchsia's code, but messages are logged with inconsistent severities. This
diminishes the value of information in the logs. Below we propose semantic
meaning for different log severity levels, and implications for logging at
certain severities in various environments.

## Motivation and problem statement

Logs are the most common way to send a diagnostics signal about a change in
the internal state of running software. They offer valuable information that can
be used to detect and correct faults in software and products.

Historically, logs contain a free-text message, as well as an enumerated
severity value. The severity value offers an indication of how important the
message is, differentiating the signal from the noise. However, the application
of severity levels is made inconsistently throughout Fuchsia code and its
dependencies, resulting in logs that are difficult to use both in manual
troubleshooting and in automated analysis.

By providing common guidelines for how to use log severities, and by creating
processes that attach consequences to certain ways of using the logs, we hope to
create a virtuous cycle that will generate more diagnostics value out of logs
while also improving the signal-to-noise ratio. For instance, we would like the
frequency of ERROR log entries to be one of the proxies to system stability.

## Implementation

### Log severity levels

Fuchsia supports a number of standard log severity levels:

#### FATAL

This indicates an unrecoverable error and that the component should
self-terminate shortly after logging the message. This typically indicates that
a core invariant in the system has been violated and proceeding further could
lead to data corruption or security vulnerabilities.

This log level should be used sparingly. This log level may indicate a hardware
issue or software bug that needs to be fixed. Since the given message will be
the final message logged before the component self-terminates, its contents
should indicate the reason for termination.

#### ERROR

This log level indicates an undesired event has occurred that the program can
recover from. An ERROR log entry's appearance in the logs is an indication of
incorrect program behavior that needs to be fixed. Developers should strive for
a one-to-one correspondence between a bug in the bug tracker and the ERROR log
entry. In other words, for every unique bug, there should be an ERROR log entry,
and for every ERROR log entry, there should be a separate bug. Developers should
attempt to maintain this correspondence in code where possible. Note that the
bug might not be in the code issuing the log statement. The bug may be in one of
the callers or one of its dependencies.

This log level and above will serve as a signal to system stability metrics.
Additionally, this log level and above will typically be present in bug reports.
This log message will be consumed by people outside the team that introduced
the message and so it is expected to provide sufficient context to guide the
reader toward the root cause of the problem.

#### WARNING

This log level indicates an unexpected event has occurred within the normal
operation of the system. These unexpected events may be environmental and
therefore outside the control of the system itself. For example, a lost network
connection may be logged as a WARNING or invalid input that prevents a program
from proceeding further may be logged as WARNING. This log level can be used to
find the root cause that led to a problem (typically an ERROR log entry). This
log level may serve to make salient that an uncommon code path was taken as a
result of the aforementioned unexpected event.

#### INFO

This is typically the lowest log level present in bug reports. This indicates
that a noteworthy state change has occurred in a program. This log level will be
used to indicate the context that led to a problem (typically an ERROR log
entry). As with higher log levels, these logs will be consumed by
other teams for diagnosis and so context is critical.

Long-lasting states should be reported using
[Component Inspection](/docs/development/diagnostics/inspect).
Such conditions should not be logged if they have not changed. For example,
instead of logging "[INFO] no configs found" at a fixed interval, the Inspect
data could contain a flag "configs_found = false," or even "config_count = 0."

#### DEBUG

This log level is typically used in engineering environments, such as during
tests or while reproducing an issue in a commit queue bot. Logs at severity
level DEBUG and below are typically not collected in bug reports, so there
should be no expectation to receive DEBUG and lower logs in bug reports. This
log level is typically more verbose than INFO and helps individual teams better
understand the state of the system they're developing.

#### TRACE

This log level will typically be used by individual teams and perhaps in
the commit queue and continuous integration. This log level is typically used to
indicate that a stage in a multistage process has completed.

### Consequences of logging

Having established guidelines for choosing log severity levels, we propose
several ways in which we automatically treat logs based on their severity.

#### Logs as in-field analytics

We choose to treat the presence of an ERROR log entry as an indication of a
software bug that needs to be fixed. It's safe to assume that a software author
would be interested in knowing that their software is logging errors outside
their local environment. Therefore we will count the presence of ERROR and FATAL
log entries broken down by source component instance, report them via Cobalt,
and track these counters as an in-field stability metric.

To view an overview of logging statistics on a device, use the following
command:

```
cs --log-stats
```

Component Stats (cs) provides a log stats mode that parses Archivist's inspect
tree to provide a summary view of logs generated per component. This view
enables engineers to see the types of logs their component generates and the
frequency with which it generates various logs relative to other components.

#### Logs as (un)expected behavior under test

It's safe to assume that a software author would be interested in knowing that
their software is logging errors under controlled conditions set during a test,
when such behavior is not expected. Therefore we will introduce behavior into
the test runtime that will cause tests to fail if unexpected ERROR logs are
observed within the test realm.

The test runtime behavior described above has been implemented, tested, and
[documented](/docs/concepts/testing/v1_test_component.md#restricting_log_severity).

## Security Consideration

* Logging with FATAL severity typically indicates that a core invariant in the
system has been violated and proceeding further could lead to data corruption or
security vulnerabilities. Termination is the safest course of action in this case.

* The component framework can securely identify the source of a log to a component
level. Within a component, modules will likely be self-attested and so cannot be
trusted to the same degree as component-level identification.

## Privacy Consideration

Any logging analytics should avoid exposing any Personally Identifiable
Information (PII). For example, we should not expose the full set of components
on a user's device in off-device metrics nor the full text of any log messages
that might contain any PII.

## Documentation

We intend to publish these logging guidelines as a [Fuchsia Development
Guide](/docs/development) as well.

## Drawbacks, alternatives, and unknowns

The proposed changes above create a system of incentives and disincentives to
log in certain ways. The intention is to get more value out of logs - notice the
important log entries by giving them appropriately high severity, while also
reducing the number of less significant log entries with overly high severity.

We consider the risk that these incentives will promote unwanted behaviors, such
as avoiding ERROR log entries entirely (even under conditions that call for it).
We believe that the motivation to create better software will drive developers
to make responsible decisions in how they react to the proposed changes.

We will test the above hypothesis by monitoring developer behavior via analysis
of bug reports and occasional interviews with team members, and course-correct
as necessary.


## Prior art and references

The [Google Testing Blog](https://testing.googleblog.com/2013/06/optimal-logging.html)
provides some basic guidelines on logging levels.
