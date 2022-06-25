{% set rfcid = "RFC-0069" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

Introduce two new flags, `forward_stdout_to` and `forward_stderr_to`,
controlling an opt in behavior for components wanting to receive stdout and/or
stderr stream upon startup. When enabled, the streams will be backed by the
[LogSink] service.

Note: This proposal only addresses supporting stdout and stderr. stdin is not
within the scope of this document.

## Motivation

When [component manager] is started, its stdout and stderr streams are
bound to the [debuglog] since no alternatives exist at this phase of the boot
process. As an example, the Archivist, the component offering the LogSink
service is itself started by component manager.

Until [recently][fxr-370683], component manager redirected all components'
stdout and stderr streams into the kernel's debuglog. This was achieved by
duplicating component manager's own stdout and stderr handles, themselves
bound to the debuglog as detailed above. However, this posed a few issues.
First, user-mode components should not write to debuglog as that is reserved
for kernel usage and highly privileged user-mode components -- rather,
most user-mode components should instead write to the  LogSink service. Second,
providing two outbound streams without an explicit opt in violates the
[Principle of Least Privilege][wiki-least-privilege].

Today, component manager lacks this feature altogether, and we want to bring it
back while addressing the two shortcomings which existed previously. Instead of
implicitly granting stdout and/or stderr to all components, we propose adding
a new flag to the [ELF runner][doc-elf-runner] to favor an explicit opt in.

The Component Framework team is in the midst of a
[long-running migration][doc-cf-migration] from [appmgr][doc-appmgr]
(Components v1) to [Component Framework][doc-cf-intro] (Components v2). One 
of the major projects of this effort is migrating all of
the components owned by the Netstack team. stdout/stderr support is a
prerequisite for migrating all of these components. The reason for this
requirement is that the Netstack component is written in Go. Go programs,
unlike those written in C++ or Rust, are executed in a runtime environment that
emits errors to stderr during setup before developer-written programs begin
running. Since errors are logged before a program's entry point, there is no
way for authors of Go components to bind the stdout and stderr handles to a
logging service. To solve this problem, we can fork and modify Go's runtime to
include the necessary logging initialization before it begins execution of
user-written Go programs. This will of course add a major technical burden on
our end, maintaining a Go fork. Alternatively, component manager can bind the
stdout and stderr handles to a logging service before the component (and the Go
runtime) is even launched, thus allowing the error messages to be captured by
Fuchsia's logging services.

More broadly, we'd like to reduce the technical burden for the v1 -> v2
migration effort mentioned above. Currently, appmgr provides stdout and stderr
handles to all v1 components and routes them to debuglog. Thus,
it's reasonable to assume that many developers within Fuchsia depend on this
feature. As we move into 2021 and start migrating more and more components, we
should allow developers to maintain the stdout and stderr support they depend on.

## Requirements

The backing logging service must be LogSink and not debuglog. There are several
reasons why we must switch to LogSink. First, debuglog is intended for kernel
usage, as mentioned above. Second, debuglog uses a small (128kb) shared ring
buffer for all processes and rotates messages on a FIFO basis. Archivist
periodically [drains][doc-logs-recording] these messages and forwards them to
LogSink. However, they are liable to get "lost" if they are rolled out
of the debuglog buffer before Archivist reads them into LogSink. Using LogSink
for the stdout and stderr logging service will not only eliminate any chances of
messages being dropped but will also decrease the likelihood that other
components and processes that use debuglog will have their messages dropped.
Plus, there is no mechanism in place to track how much was lost when the buffer
rotates faster than it can be drained. Thirdly, debuglog doesn't support severity
levels (e.g. DEBUG, INFO, etc.) This is a critical requirement because we need
to distinguish stdout messages from stderr ones. With regards to logging, we
could only do this by mapping each output stream to a particular severity level.

## Design

The proposal is to introduce two new enum values to the `program` stanza for ELF
components, `forward_stdout_to` and `forward_stderr_to`. The enum will be
optional for ELF components, and will default to `none`. When it is none, ELF
runner will not bind the _stdout_ and _stderr_ handles to the LogSink service
(current behavior) and those streams will continue to be ignored. When these
values are set to `log`, the ELF runner will create a socket that will capture
the output of the _stdout_ and _stderr_ streams. It will then forward the bytes
read to the LogSink service. For _stdout_, it'll send INFO messages;
for _stderr_, it'll send WARN messages. Messages will be newline-delimited,
and each line will be partitioned as an atomic message to the LogSink service.
The maximum message size for the LogSink service is 32KB, so we'll cap the
byte stream buffer at 30KB (to allow for some space for message metadata).
Bytes beyond that boundary will be discarded and only the partial message will
be sent to the LogSink service. This raises an interesting edge case where part
of code point can be split at the 30KB boundary. In this case, no special
handling will be done and the invalid first half of the code point will be
decoded as is. All the input bytes will be decoded as UTF-8 using
[String::from_utf8_lossy][rust-doc-lossy]. Per this function's API,
all invalid UTF-8 sequences will be replaced with `U+FFFD REPLACEMENT CHARACTER`.


```json
{
    program: {
        "runner": "elf",
        "forward_stdout_to": "log",
        "forward_stderr_to": "log",
    }
}
```

## Implementation

Since the feature will be restricted to ELF runner, the changes required for
implementation are fairly small. We predict that no more than 2 CLs would be
needed to implement this change.

## Performance

Performance costs are minimal since lines will just be emitted to LogSink. The most
notable overhead that this feature will introduce is the parsing of the byte streams
and splitting of newlines. However, this overhead is relatively low-cost since
logging is an irregular operation and the byte streams themselves *tend* to be
short. More importantly, this will only affect components that explicitly opt in
to this behavior. Therefore if a performance problem does come up, we can quickly
address it by simply setting `forward_stdout_to` and `forward_stderr_to` to
`none` in those components, temporarily going back to square one until we
resolve the underlying performance problem.

## Security considerations

Archivist already has the mechanisms in place for attribution and flow
control, so a misbehaving component can't deny service by spamming its stdout.
So no additional work is needed. However, we should note that this feature
provides a mechanism to put arbitrary bytes into another process' address space
(from component to Archivist). This could be problematic if the buffer is too
large, though we've mentioned mitigation efforts for that above. Also, since
this implementation minimally processes the input stream we reduce the risk
of escalating privileges. Had this been a complex parser, the likelihood of
bugs and vulnerabilities would have increased.

## Privacy considerations

The LogSink backend and all LogSink clients are already privacy-compliant, with
logs being attributed to their source and with sufficient PII scrubbing
mechanisms already being in place. So no additional work is needed.

## Testing

Alongside unit tests, we'll add integration tests that ensure that logs are
written to Archivist. These integrations tests will be written in all supported
languages: C/C++, Rust, and Go. Dart will not be tested because Dart component
execution is not handled by the ELF runner.

## Documentation

This flag will be documented in the [ELF runner][doc-elf-runner] section of
the Components v2 doc.

## Drawbacks, alternatives, and unknowns

### Numbered Handles
We've explored raising numbered handles from the process framework to Component
Framework as a way of supporting this feature. While we decided against it,
the rough design of how the manifest file would have looked like was:

```json
{
    program: {
        "runner": "elf",
        "handles": ["STDOUT", "STDERR"]
    }
}
```

The ELF runner would read the constant strings and map it to its appropriate
numbered handle internally. Ultimately, we decided against this because we
didn't know of an immediate use case for another numbered handle. Also, if we
do find another numbered handle that should be configured in the ELF runner's
`program` stanza, then we can trivially update the manifest file syntax and
ELF runner implementation.

### Component Manager
We've also explored introducing a new framework-level capability for numbered
handles. The rough design of how the manifest file would have looked like was:

```json
{
    use: [
        {
            "handle": "stdout-to-log",
            "from": "framework",
            "number": "STDOUT",
        },
        {
            "handle": "stderr-to-log",
            "from": "framework",
            "number": "STDERR",
        }
    ]
}
```

This approach was deemed undesirable for the reasons listed above regarding
numbered handles and because introducing this at the component manager-level
raises new questions about POSIX-compability from component manager. For example,
should all runners have to implement this? How would it look like for a runner
that doesn't use stdout/stderr, like the "web" runner? Thus, we've decided to
pursue the POSIX-compatibility question as a separate workstream, outside of the
scope of this RFC.

### Introducing New Component

We've also explored implementing the translation layer, the part that
parses the stdout/stderr byte streams and forwards them to the LogSink service,
in a new component, owned and managed by the Component Framework team. However,
after several discussions, it was decided that this approach would be
impractical because we'd have to devise a way to retain logging attribution
when we log the messages. Archivist uses the event capability to acquire
component source info (e.g. moniker) and all that information would be lost if
we use a middle-layer component.

### FDIO

We've also explored using fdio, Fuchsia's POSIX-compatibility
library, to implement this feature. That is, making a new type that recognizes
the stdout/stderr file descriptors and internally (within fdio) redirecting the
output to LogSink. However, after several discussions, it was decided to forgo
modifying fdio because to do so would make implementation more difficult. We
discovered that there are edge cases for POSIX compatibility that couldn't be
implemented using a LogSink forwarder in fdio. Also, an fdio-based implementation
would yield more uncertainties, and duplicate effort. Alternatively, if we use
a socket, as proposed above, it'll be POSIX compliant "out of the box".

[LogSink]: /docs/development/diagnostics/logs/recording.md
[component manager]: /docs/concepts/components/v2/component_manager.md
[debuglog]: /docs/reference/kernel_objects/debuglog.md
[doc-appmgr]: /docs/concepts/components/v1/component_manifests.md
[doc-cf-intro]: /docs/concepts/components/v2/introduction.md
[doc-cf-migration]: /docs/contribute/open_projects/components/migration.md
[doc-elf-runner]: /docs/concepts/components/v2/elf_runner.md
[doc-logs-recording]: /docs/development/diagnostics/logs/recording.md
[doc-principles-inclusive]: /docs/concepts/principles/inclusive.md
[doc-principles-pragmatism]: /docs/concepts/principles/pragmatic.md
[fxr-370683]: https://fuchsia-review.googlesource.com/c/fuchsia/+/370683/
[rust-doc-lossy]: https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy
[wiki-least-privilege]: https://en.wikipedia.org/wiki/Principle_of_least_privilege
