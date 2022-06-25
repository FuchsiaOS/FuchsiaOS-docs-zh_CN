{% set rfcid = "RFC-0006" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

This document proposes a refinement to the Fuchsia RFC process that takes into
account the challenges of the Zircon codebase. Specifically, changes to Zircon
that meet one (or more) of the criteria described below, must follow the RFC
process before being accepted.

## Motivation

Currently Zircon uses an ad-hoc process to make system-wide changes, but now
that Fuchsia has a clear [RFC process](contribute/governance/rfcs/0001_rfc_process.md),
Zircon should follow it. However, given that Zircon sits at the very bottom of the
software stack, it is far more sensitive to certain types of changes than code that
sits closer to the leafs of the project (e.g., when greatly increasing the dependency
graph or how it balances memory usage vs performance).

## Design

The vast majority of changes to Zircon do not require a RFC and a code review should
be sufficient. However, in addition to the considerations outlined in Fuchsia RFC process,
changes in the source directories:

* /zircon
* /src/zircon
* /src/bringup

that meet the following criteria must use RFC process:

 * *Adding or removing Zircon system interfaces.* The syscall interface, associated
   structures and constants is the ground truth for the entire system and has broad
   impact beyond Zircon itself and needs broad consensus before implementation.

 * *Changing resource handling behaviors.*  How the system handles partitioning or
   virtualizing resources such as memory, I/O, processor time or energy consumption
   and what it does when they are oversubscribed or scarce.

 * *Modifying isolation guarantees.* How and what is private and isolated among
    equal tasks and what privileged tasks can observe and modify. Changes here need to
    be approved via this process in consultation with the security team.

 * *Significant changes of performance or memory use.* Sometimes when additional
    security, monitoring or features are added, there is a corresponding decrease in
    performance, or higher memory use, which need to be vetted via this process.

 * *Favoring a single platform.* Zircon strives to have an equal baseline of features
    and services across all supported architectures and boards. Changes that leverage
    one platform capabilities but are not feasible or practical on other supported
    platforms need to use this process.

 * *Adding or Downgrading support for a platform.* Adding new boards or architectures,
    or deprecating/reducing support for an existing platform needs to be vetted via
    this process.

 * *New build configurations.* Adding new build configurations increases the development
   and testing burden across the entire project and needs to be vetted beforehand.

 * *Significant increases on the dependency graph.* Zircon dependencies affect the
   entire project and significant changes, for example a new dependency on a package
   that itself has significant dependencies or that is large by itself, should use
   the RFC process.

Other changes that might benefit of the RFC process are ones that require manual or
automated large scale changes of the codebase. For example how logs are written or how
error paths are handled. Rather than live with islands of consistency, the aspiration
is to find the best patterns and uniformly apply them to the entire codebase.


## Documentation

This RFC along with the [Fuchsia RFC process](contribute/governance/rfcs/0001_rfc_process.md)
serves as documentation for the RFC process as it applies to Zircon.

## Drawbacks, Alternatives, and Unknowns

The Fuchsia RFC process introduces friction that might slow down the pace of making and
executing decisions. The criteria in the ["when to use the process" section](contribute/governance/rfcs/0001_rfc_process.md#when-to-use-the-process)
attempts to mitigate this by scoping the process to consequential situations but such
scoping is bound to have false positives and false negatives.

Recording decisions in the source repository has the effect of making those decisions
more difficult to change. That effect might be positive in some scenarios, but the effect
might also be negative in other scenarios.

There are many possible alternative strategies for managing the decision-making process
but being aligned with the Fuchsia RFC process and its evolution as it addresses these
issues seems the best way forward at this time.

