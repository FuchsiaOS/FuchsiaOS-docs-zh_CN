<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0175" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}

<!-- mdformat on -->

## Summary

Change [ELF runner] to default to forwarding stdout & stderr to LogSink if
available.

## Motivation

This RFC builds on [RFC-0069] which specified a feature for the ELF runner to
support stdout & stderr forwarding to [`fuchsia.logger.LogSink`][logsink]. Since
implementing that feature, we've discovered a number of scenarios where
components have `LogSink` but miss important log messages when migrating from
components v1 to v2 because their stdout/stderr messages are silently dropped
in the transition.

Fuchsia's [test runners] capture and print stdout & stderr by default and users
are frequently confused by the different behavior of ELF runner which requires
explicit forwarding.

The surprise of an unconfigured stdout or stderr is amplified when code only
uses stdio in failure modes that are not commonly exercised, resulting in
production crash reports that are missing critical information. For example, the
`ZX_ASSERT` macro reports the failed assertion to stderr, and the HLCPP FIDL
bindings report some kinds of parse failures to stderr.

## Stakeholders

_Facilitator:_ hjfreyer@google.com

_Reviewers:_

- geb@google.com (Component Framework)
- yaneury@google.com (RFC-0069 author)

_Consulted:_ hjfreyer@google.com, abdulla@google.com, miguelfrde@google.com,
surajmalhotra@google.com, wez@google.com, shayba@google.com

_Socialization:_ This RFC is the result of email and offline discussions on the
Component Framework team.

## Design

If the author of an ELF component leaves the `forward_stdout_to` or
`forward_stderr_to` fields unspecified in their `program` section, then the ELF
runner will attempt to set up forwarding for the unspecified stream(s). Any
failure to forward the stdio stream to LogSink in this configuration will be
silent and the component will still be started.

## Implementation

We will relax the behavior of forwarding stdio to LogSink so that components
are still started if they request forwarding but do not have a LogSink.

The ELF runner will assume a default value of `forward_stdout_to: "log"` and
`forward_stderr_to: "log"` if either is unspecified in the component's manifest.

The ELF runner will still provide stdout and/or stderr handles to a launched
component that requested or defaulted to forwarding if there were errors in
initializing that forwarding.

## Performance

We do not anticipate significant performance differences from the result of this
change.

## Ergonomics

Having a useful default behavior should reduce the number of decisions that need
to be made by a component developer, improving ergonomics.

We will delete `sdk/lib/syslog/elf_stdio.shard.cml` after landing the
implementation for this change and switch its clients over to the main syslog
shard which will have the same behavior for ELF components.

## Backwards Compatibility

Components without LogSink with the default stdio forwarding config will
generate routing errors in Component Manager's logs when it attempts to open
LogSink in their namespace. In the initial rollout of the new default we will
add explicit `forward_std{out,err}_to: "none"` fields to platform component
manifests without LogSink. In the future these error messages can be eliminated
by using more sophisticated namespace probing to determine whether a component
should have a LogSink available or not.

## Security considerations

This design does not increase the privilege of components by granting them any
additional capabilities, the forwarding is only enabled if their manifest
requests LogSink and they are routed that capability.

## Privacy considerations

This will result in additional log messages being written to the system log
which may contain user-generated data. The logging pipeline already has controls
in place to audit and scrub privacy-sensitive data, so no change is needed here.

## Testing

There are existing integration tests for the stdio forwarding feature which will
be extended to cover the default scenarios.

## Documentation

A number of component developer guides need to be updated, usually to remove
warnings that stdio forwarding must be set up. This is a sign that this change
will reduce the cognitive complexity of developing or migrating a component.

## Drawbacks, alternatives, and unknowns

### Drawback: increased "magic"

This change adds behavior to launching ELF components without requiring them to
specify anything in their manifest, unlike most component features. However,
this change in behavior is not an increase in the privilege granted to a
component.

The behavior proposed in this RFC is an attempted compromise between Component
Framework design principles (explicitness, least-privilege) and the expectations
of developers in existing POSIX-like systems (stdio is ubiquitous and useful).

### Alternative: rationalizing runners

We could make ELF runner and test runners behave more similarly if we changed
test runners to actually use the ELF runner as their base implementation.
Currently ELF and test runners share implementation code from a Rust library,
instead they could share implementation code by allowing runners to delegate to
another runner for their implementation.

This is an attractive direction to pursue for many other reasons. It is a
significantly larger task than changing ELF runner's defaults, and the author
believes we would still want this RFC's behavior as the default in ELF runner to
ensure a unified developer experience.

## Prior art and references

This RFC proposes an extension to a feature described in [RFC-0069].

[ELF runner]: /docs/concepts/components/v2/elf_runner.md
[RFC-0069]: /docs/contribute/governance/rfcs/0069_stdio_in_elf_runner.md
[logsink]: /docs/concepts/components/diagnostics/logs/README.md
[test runners]: /docs/development/testing/components/test_runner_framework.md
