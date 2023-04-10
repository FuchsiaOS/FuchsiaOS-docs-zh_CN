<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0098" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Motivation

The [Component Framework][cf-intro] (CF) system provides the foundations for
running software on Fuchsia. With the exception of a few early-boot processes,
all software spanning from low-level system services to UI-driven front end
apps, are [components][glossary.component] and operate in the context of the
Component Runtime.

For this reason, changes to the Component Framework can have a broad impact on
the Fuchsia architecture and on developers who write software targeting
Fuchsia.

The [Fuchsia-wide RFC process][rfc-process] provides a consistent and
transparent process for making technical decisions with broad impact. This
document seeks to provide the detail necessary to disambiguate between which CF
changes have sufficiently broad impact to deserve a dedicated RFC, and which do
not.

## Design

### Changes that require an RFC

* **Changes resulting in additions or modifications to the public CF APIs,
  libraries, and tools,** as exposed in the Fuchsia SDK. For example, changes to
  `fuchsia.sys2`, `fuchsia.component`, `fuchsia.session`, CML, C++ component
  libraries, and so on.
* **Changes to security policy, including allowlists and capability routing**.
  For example, adding a new allowlisting security policy, or adding a new
  routable capability.
* **Changes to Component Manager that would result in externally visible
  effects**. For example, a change in how shutdown order is calculated, or
  any substantial change to its performance profile (code size, memory use, cpu
  time).
* **Changes to Session Manager that would result in externally visible
  effects**. For example, changes to the set of capabilities routed from the
  platform to sessions, and vice versa, or any substantial change to its
  performance profile.
* **Changes that introduce or remove platform components used in sessions**, as
  part of the SDK. For example: introducing a new "manager" component that is
  intended to be reused across sessions.
* **New debugging or diagnostics features for Component Manager**. For example,
  a new logs analysis feature, or additions to Inspect.
* **Changes that propose modifications to the component architecture**. For
  example, introducing new resource management and quota concepts.
* **Major changes or additions to .CML file syntax**. For example, changing the
  structure of how .CML files express routing to children.

When a change does not fit the above criteria perfectly, the default stance is
to either:

1) Follow the RFC process, or
1) Seek input from the Fuchsia Engineering Council

RFCs that document the status-quo are optional but encouraged. Publishing a
status-quo RFC expands awareness of the existing architecture of Fuchsia to
many more individuals, including those individuals outside Google.
Additionally, they provide references to the current state of the architecture
for future RFC authors to link to.

### Positive examples: past changes that would now require an RFC

Note: Prior to adopting the RFC process, the Component Framework team used a
local design review processed called Component Tuning Proposals (CTP).

* Component resolvers (CTP-013): introduced a CF extension point allowing
  component authors to alter how component URLs are resolved into component
  metadata and executable content.
* Components v2 allow-lists (CTP-020): introduced a mechanism to control the
  security policy of the CF runtime
* Route runners through environments (CTP-021): proposed a change to how runner
  capabilities were routed to children and grandchildren within a component
  topology
* Realm Builder (nee Topology Builder - CTP-033), _when introduced to the SDK_:
  a library for tests to create complex component topologies at runtime.
* New CML capability syntax (CTP-023): changed the syntax of capability routing
  in .CML files
* Stdio as a capability (CTP-031, RFC-69): Introduced new routable capabilities
  for stdout, stdin, and stderr, and defined the .CML file syntax for said
  routing.
* Use from child (CTP-036): although a small change, it impacts constraints
  previously placed on routing between components.

### Negative examples: past changes that still would not require an RFC

* Component Framework API revisions (CTP-030): API revisions for better
  readability, and clearer semantics, without altering the behavior of the
  component runtime itself.
* Component Manager configurability (CTP-024): proposed a new mechanism to
  configure the internal behavior of component_manager to remove tech debt
  introduced through a less advanced configuration mechanism.
* Component Graph Analysis (CTP-034): introduced build-time static analysis on
  component manifests in fuchsia.git in order to catch mismatches in routing
  due to human error.

### Progressing from idea to RFC

Many features require work along spectrum from prototyping, to design feedback
from peers, to getting hands-on customer experience with production-quality
code and APIs. It is not unusual for CF contributors to gain experience with
features, with iteratively-expanding audiences. For example, a feature proposal
may go through a less-formal design process including members of the core team,
an implementation, and then experimentation with `fuchsia.git` developers,
before going through the more
formal RFC process.

Contributors may opt to enter the RFC process earlier, at their discretion,
especially when they can predict that their design is destined for an RFC based
on the criteria described above.

## Implementation strategy

Any work that has already gone through the CF project's established design
review processes will not be retroactively required to adhere to the RFC
criteria defined here.

If a contributor wishes to work with the CF team on an RFC, they should feel
free to reach out to <component-framework-dev@fuchsia.dev> and we'll assign
them a designated contact.

## Performance

No impact, process-only change.

## Ergonomics

We'll revisit these criteria if we find that the criteria herein are allowing
changes to move ahead that would have been served better by the RFC process, or
if the criteria herein are found to be too restrictive.

## Backwards Compatibility

No impact.

## Security Considerations

Changes to the Component Framework area that modify security policy or
strategy will require an RFC.

## Privacy considerations

Changes to the Component Framework area that modify privacy policy or
strategy will require an RFC.

## Testing

No impact.

## Documentation

Does not apply.

## Drawbacks, alternatives, and unknowns

It is unknown if the criteria in this document strike the right balance between
broad, inclusive review at the expense of velocity, versus more targeted review.

Another alternative is to stick with the status-quo: the CF team uses its
internal CTP process.

## Prior art and references

* [Zircon RFC criteria][zircon-criteria]
* [FIDL RFC criteria and process][fidl-criteria]

[cf-intro]: /docs/concepts/components/v2/introduction.md
[glossary.component]: /docs/glossary/README.md#component
[rfc-process]: /docs/contribute/governance/rfcs/0001_rfc_process.md
[fidl-criteria]: 0049_fidl_tuning_process_evolution.md#criteria
[zircon-criteria]: 0006_addendum_to_rfc_process_for_zircon.md