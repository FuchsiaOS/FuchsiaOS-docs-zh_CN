# Choosing severity for log records

The purpose of this document is to help you choose the correct severity for your log records.
Fuchsia supports log records with severity equal to one of `FATAL`, `ERROR`, `WARNING`, `INFO`,
`DEBUG`, or `TRACE`. This document summarizes the normative guidelines in [RFC-0003: Logging].

<table>
  <tr>
    <td>Severity</td>
    <td>Summary</td>
  </tr>
  <tr>
    <td><code>FATAL</code></td>
    <td>An unrecoverable error event. Should cause termination.</td>
  </tr>
  <tr>
   <td><code>ERROR</code></td>
   <td>
   An unexpected event from which the program can recover, indicates a bug or undesired program
   behavior.
   </td>
  </tr>
  <tr>
   <td><code>WARNING</code></td>
   <td>An unexpected event has occurred within normal operation of the system.</td>
  </tr>
  <tr>
   <td><code>INFO</code></td>
   <td>An informative event, typically expected. Usually the lowest level in bug reports.</td>
  </tr>
  <tr>
   <td><code>DEBUG</code></td>
   <td>An informative event for development.</td>
  </tr>
  <tr>
   <td><code>TRACE</code></td>
   <td>An informative event for development, emitted at fine granularity or high frequency.</td>
  </tr>
</table>

## Not sure which to use?

Log records describe events in the system. Choosing the severity for a log record is one way of
describing the event itself.

Unexpected events should generally be logged at `FATAL` for unrecoverable events, `ERROR` for
recoverable events which represent incorrect program behavior (i.e. "bugs"), and `WARNING` for
recoverable events which are not themselves bugs but which may provide important context about other
failures.

You should log events that are only relevant to a contributor working locally on a project as
`DEBUG` or `TRACE`. If the events are expected, you should generally log them as `INFO`.

If you are still unsure how to label the event severity, consider the following questions:

* What change(s) did the event cause? What was different before and after the event?

  This is usually a matter of writing or reading the message for the event in question. If no change
  occurred in the system, it may be better to skip the event entirely. For important heartbeat events,
  consider `TRACE` severity.

* Who wants to know about this event?

  Events at different severities are available by default in different contexts, which in practice
  defines their audience. Many events are relevant to an engineer working on a project but would
  create too much noise for other teams — it's usually best to emit these events at `DEBUG` or
  `TRACE` severity because those are excluded from bug reports by default.

  Because `INFO` and above are more visible, events which do not provide critical context for an
  implementor when debugging should be limited to information which is useful to clients or others
  outside the implementing team.

* Is this event recurring or ongoing?

  Log events which describe a frequent background activity or an ongoing failure state should be
  limited to `DEBUG` or `TRACE` and also reported by Inspect and Cobalt to avoid production log spam.
  While the entry into a failure state may be useful to log, it is not generally useful to log events
  that say things like "still in the failure state".

  It is difficult to avoid repeating log statements this way when a failure state is "flapping" or
  resolving and unresolving itself regularly. Rate limiting by repetition count or time interval can
  be an appropriate response, but Fuchsia's logging libraries do not currently offer out-of-the-box
  tools for this. It may be explored in the future if a clearer set of patterns and best practices
  emerge.

<!--xrefs-->
[RFC-0003: Logging]: /contribute/governance/rfcs/0003_logging.md
