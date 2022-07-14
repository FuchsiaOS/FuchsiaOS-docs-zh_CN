<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0000" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

A one paragraph description of the rest of the proposal.

## Motivation

What problem does this proposal solve?

## Stakeholders

Who has a stake in whether this RFC is accepted? (This section is optional but
encouraged.)

_Facilitator:_

The person appointed by FEC to shepherd this RFC through the RFC
process.

_Reviewers:_

List people whose vote (+1 or -1) will be taken into consideration by FEC when
deciding whether this RFC is accepted or rejected. Where applicable, also list
the area they are expected to focus on, such as "FIDL" or "security".  In some
cases this section may be initially left blank and stakeholder discovery
completed after an initial round of socialization. In general, "reviewers"
should be listed on the reviewers line in gerrit and people who are "consulted"
should be CCed. Care should be taken to keep the number of reviewers manageable,
although the exact number will depend on the scope of the RFC in question.


_Consulted:_

List people who should review the RFC, but whose approval is not required.


_Socialization:_

This section may be used to describe how the design was socialized before
advancing to the "Iterate" stage of the RFC process. For example: "This RFC went
through a design review with the Component Framework team."

## Design

This is the technically detailed version of your proposal.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in
[IETF RFC 2119](https://tools.ietf.org/html/rfc2119).

_Prompt for Area: FIDL, replace with the relevant area(s)_

One important high-level bit of your proposal is what part of FIDL your proposal
modifies. This includes at least:

*   The FIDL source language
*   The FIDL wire format
*   The first-class language bindings (C, C++, Dart, Go, Rust)
*   The FIDL style guide and API rubric
*   The FIDL tuning process

Your proposal should talk about all the relevant areas. For instance, if your
proposal adds a new type to the FIDL language, it also needs to discuss the
style guide for that feature, and how to implement it in the bindings.

## Implementation

How will you go about implementing this design? Can the change be made in a
single Gerrit change or does the change involve a complex migration of
third-party dependencies? Do you plan to structure the implementation into
phases? What dependencies exist at each phase?

## Performance

What impact will this proposal have on performance? What benchmarks should we
create to evaluate the proposal? To evaluate the implementation? Which of those
benchmarks should we monitor on an ongoing basis?

## Ergonomics

_Prompt for Area: FIDL, replace with the relevant area(s)_

Does your change make FIDL easier to use, and simpler to understand? Does it
make the bindings easier to use? If it doesn't, what's the justification for the
complexity?

Focus on both the end-user API and the cognitive effort required to understand
the concept.

## Backwards Compatibility

_Prompt for Area: FIDL, replace with the relevant area(s)_

Backwards compatibility comes in two flavors: FIDL file source compatibility,
and ABI or wire format compatibility. This section should speak to both. Over
time, the ability to make backwards-incompatible changes will get harder.

If you are introducing a new data type or language feature, consider what
changes you would expect users to make to FIDL definitions without breaking
users of the generated code. If your feature places any new source compatibility
restrictions on the generated language bindings, list those here.

## Security considerations

What impact will this proposal have on security? Does the proposal require a
security review?

A good starting point is to think about how the system might encounter untrusted
inputs and how those inputs might be used to manipulate the system. From there,
consider how known classes of vulnerabilities might apply to the system and what
tools and techniques can be applied to avoid those vulnerabilities.

## Privacy considerations

What impact will this proposal have on privacy? Does the proposal require a
privacy review?

A good starting point is to think about how user data might be collected,
stored, or processed by your system. From there, consider the lifecycle of such
data and any data protection techniques that may be employed.

## Testing

How will you test your feature? A typical testing strategy involves unit,
integration, and end-to-end tests. Are our existing test frameworks and
infrastructure sufficient to support these tests or does this proposal require
additional investment in those areas?

If your system defines a contract implemented by other people, how will those
people test that they have implemented the contract correctly? Consider, for
example, creating a conformance test suite for this purpose.

_Prompt for Area: FIDL, replace with the relevant area(s)_

How will your feature be tested? For instance, do you need to write new tests
for `fidlc`, or for the C++ bindings?

If your change affects encoding or decoding, plan to update the conformance test
suite.

If your change affects source compatibility, plan to update the
[source compatibility test suite](/src/tests/fidl/source_compatibility).

How will uses of your new feature be tested? If you add a language feature, how
will you test it in each language's bindings?

## Documentation

Do we need to create or update any documentation to cover this feature? For
example, do we need to add or remove an entry from the project roadmap? Do we
need to change the architecture document? Would end-developers benefit from
documentation related to this proposal?

[Don't use the RFC itself as documentation for your feature.][rfcs-vs-docs]
Documentation needs to kept up-to-date, and RFCs shouldn't change once they're
accepted.

[rfcs-vs-docs]: best_practices.md#rfcs-vs-docs

_Prompt for Area: FIDL, replace with the relevant area(s)_

There are potentially several kinds of documentation to address.

How would you write or change a tutorial for this feature, in the style of the
various FIDL tutorials? Imagine explaining your feature to someone new to
Fuchsia.

How would you write reference documentation? For example, suppose your proposal
extends the FIDL wire format. How would you update the documentation of the wire
format? Imagine explaining your feature to someone in sufficient detail that
they could implement it.

What are important examples or use cases of your proposed feature?

## Drawbacks, alternatives, and unknowns

What are the costs of implementing this proposal?

What other strategies might solve the same problem?

What questions still need to be resolved, or details iterated upon, to accept
this proposal? Your answer to this is likely to evolve as the proposal evolves.

## Prior art and references

Is there any background material that might be helpful when reading this
proposal? For instance, do other operating systems address the same problem this
proposal addresses?
