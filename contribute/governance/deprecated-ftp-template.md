# [FTP](deprecated-ftp-process.md)-NNN: Your Boring Title

Note: This process was deprecated in [RFC-0017](/docs/contribute/governance/rfcs/0017_folding_ftp_into_rfc.md).
FTP proposals should now use the [Fuchsia RFC process](/docs/contribute/governance/rfcs/rfc_process.md).

_(optional) Your Witty Title_

Field     | Value
----------|--------------------------
Status    | Draft
Authors   | *your emails*
Submitted | *leave blank until submitted*
Reviewed  | *leave blank until reviewed*

## Summary

A one paragraph description of the rest of the proposal.

## Motivation

What problem does this proposal solve?

## Design

This is the technically detailed version of your proposal.

One important high-level bit of your proposal is what part of FIDL your proposal
modifies. This includes at least:

* The FIDL source language
* The FIDL wire format
* The first-class language bindings (C, C++, Dart, Go, Rust)
* The FIDL style guide and API rubric
* The FIDL tuning process

Your proposal should talk about all the relevant areas. For instance, if your
proposal adds a new type to the FIDL language, it also needs to discuss the
style guide for that feature, and how to implement it in the bindings.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED",  "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [RFC 2119][ietf-rfc2119].

## Implementation Strategy

How will you go about making this change? Some FTPs will not involve breaking
changes, and might happen as a single CL. Others might be sweeping changes.
Either way, think about how you would break down the implementation into tasks.

For more complex implementations that are likely to change, we recommend writing
an Intent-to-Implement ("I2I") doc and linking to it in this section. The I2I
can then serve as a living, separate design doc that may be updated
independently, so that this document isn't constantly re-edited as details
change. This also enables your FTP to focus on the high-level important
concepts, rather than implementation and execution.

## Ergonomics

Does your change make FIDL easier to use, and simpler to understand? Does it
make the bindings easier to use? If it doesn't, what's the justification for the
complexity?

Focus on both the end-user API and the cognitive effort required to understand
the concept.

## Documentation and Examples

There are potentially several kinds of documentation to address.

How would you write or change a tutorial for this feature, in the style of the
various FIDL tutorials? Imagine explaining your feature to
someone new to Fuchsia.

How would you write reference documentation? For example, suppose your proposal
extends the FIDL wire format. How would you update the documentation of the wire
format? Imagine explaining your feature to someone in sufficient detail that
they could implement it.

What are important examples or use cases of your proposed feature?

## Backwards Compatibility

Backwards compatibility comes in two flavors: FIDL file source compatibility,
and ABI or wire format compatibility. This section should speak to both. Over
time, the ability to make backwards-incompatible changes will get harder.

If you are introducing a new data type or language feature, consider what
changes you would expect users to make to FIDL definitions without breaking
users of the generated code. If your feature places any new source
compatibility restrictions on the generated language bindings, list
those here.

## Performance

What impact will this proposal have on IPC performance? On build performance?

## Security

How will your feature impact security? Will your feature be easy to use safely
in non-memory-safe languages? Does your feature make it easy to accidentally
leak details of a process's address space?

## Testing

How will your feature be tested? For instance, do you need to write new tests
for `fidlc`, or for the C++ bindings?

If your change affects encoding or decoding, plan to update the [conformance
test suite][conformance-suite].

If your change affects source compatibility, plan to update the [source
compatibility test suite][source-compatibility-suite.

How will uses of your new feature be tested? If you add a language feature, how
will you test it in each language's bindings?

## Drawbacks, Alternatives, and Unknowns

What are the costs of implementing this proposal?

What other strategies might solve the same problem?

What questions still need to be resolved, or details iterated upon, to accept
this proposal? Your answer to this is likely to evolve as the proposal evolves.

## Prior Art and References

Is there any background material that might be helpful when reading this
proposal? For instance, do other serialization or IPC systems address the same
problem this proposal addresses?

<!-- xref -->
[ietf-rfc2119]: https://tools.ietf.org/html/rfc2119
[conformance-suite]: /src/tests/fidl/conformance_suite/
[source-compatibility-suite]: /src/tests/fidl/source_compatibility/
[fidl-tutorials]: /docs/development/languages/fidl/tutorials/overview.md
