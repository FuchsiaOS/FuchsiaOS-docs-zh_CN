{% set rfcid = "RFC-0067" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

This RFC proposes a few additions to the the Fuchsia RFC process detailed in
[RFC-0001](0001_rfc_process.md). A "Last call" step is introduced in place of
the "Approve" step. Exit criteria for each step of the process is called out.
The process of amending an RFC is introduced.


## Motivation

The motivation for these changes is to address feedback received on the process,
and to explicitly make some clarifications.

The "Last call" step requires an Eng Council member to send an email to
eng-council-discuss@fuchsia.dev when the iterations on an RFC are converging.
Introducing this step attempts to extend the reach of RFCs by sending a push-style
notification to solicit any additional feedback before making a final decision.

Explicitly calling out exit criteria for each step of the process attempts to
summarize and clarify what's expected of that step.

In addition to these, at the end of the "iterate" step, Eng Council is required
to confirm the stakeholders and make any changes if needed. This is an attempt
to ensure that the list of stakeholders is complete, and that they have been
consulted before moving to the next stage of the process.


## Implementation

### Proposed changes to each step of the RFC process

#### Socialize

No proposed changes here. Adding the following exit criteria.

*Exit criteria*: None specifically. This is per the author's discretion.
This step is meant to help the author crystalize the goal(s) and potential solutions.
If they feel that this is accomplished, then they can proceed to the next step.

#### Draft

Adding the following exit criteria for this step.

*Exit criteria*: CL containing your RFC is created.

#### Iterate

Additions to this step include having the author solicit wider feedback early in
the process, and to have a final list of stakeholders verified by Eng Council.
The following text will be added to the RFC process:

"In addition, you may email your CL to eng-council-discuss@fuchsia.dev
soliciting additional feedback."

"At the end of this step, provide a list of stakeholders and their roles to
eng-council@fuchsia.dev. Eng Council will provide confirmation on the stakeholders
identified, and will suggest any changes, if needed. Iterate with any new
stakeholders identified."

In addition to the above, the following note will be added to help reviewers
with formulating their feedback:

"*Note to reviewers:* The RFC process is meant to encourage a variety of
perspectives and vibrant discussions. Often, giving negative feedback in a public
forum might be difficult. If needed, reviewers can reach out to their leads,
peers or Eng Council to help them formulate the feedback so it can be delivered
effectively in the CL."

*Exit criteria:* All stakeholders identified and approved by Eng Council; feedback
solicited and incorporated.

#### Last call

The "Approve" step is being renamed to "Last call". The additional step here
is to have an Eng Council member solicit any final feedback before making a decision
on the RFC. The following text will be added to this step:

"Once the iterations on the RFC are converging, the author must email
eng-council@fuchsia.dev requesting them to move the RFC's status to last call.
An Eng Council member will send an email to all stakeholders and
eng-council-discuss@fuchsia.dev to solicit any final feedback before moving to
the decision step. The RFC will be open for feedback for the next 7 calendar days."

*Exit criteria:* Feedback provided by all stakeholders; all feedback addressed.

#### Submit

If there were objections in approved RFCs, the corresponding rationale and any
tradeoffs being made must be incorporated into the RFC. The following text will
be added to this section:

If there were objections in approved RFCs, when flags are cleared, an Eng Council
member will indicate if any additional information needs to be documented in the RFC.
To this effect, the following text will be added to this section:

"Eng Council will indicate if any additional information needs to be documented
in your RFC, such as rationale for a different approach or tradeoffs being made."

Rejected RFCs will be assigned the next available RFC number. The operative text
will change as follows:

"If the project decides to reject your RFC, a member of the Eng Council will
comment on your CL stating that the RFC is rejected, provide a rationale
for the rejection and will assign the RFC a number."

*Exit criteria:* RFC number assigned; any applicable rationale, tradeoffs and
Eng Council feedback incorporated; RFC merged.


### Process to amend RFCs

An existing RFC can be amended if the following criteria are met:

 * Clarifications on what was already approved.
 * Mechanical amendments such as updating links, documentation, usage, etc.
 * Any improvement or minor changes in design discovered later, for example,
 during implementation.

For changes in design, please capture what the original design goals were, and why
and how they changed.

For any significant changes in design, please submit a new RFC.

 * In the new RFC, please reference the original RFC(s) and explicitly call out the
 type of change in the title, e.g., Addendum.
 * If the design in the original RFC is being deprecated, amend the original RFC
  to call this out and reference the new RFC.
 * If there are multiple RFCs that make changes to the same area, create a new RFC
 compiling the existing RFCs. Please also amend the existing RFCs to reference the new one.

If the RFC process is being updated, please also update the [RFC process page]
(rfc_process.md).

## Drawbacks, alternatives, and unknowns

Drawbacks: In the "Last call" step, though we have introduced push-style notifications
to eng-council-discuss@, it is possible that we miss receiving feedback within the 7
calendar day window. This is a tradeoff that's being made to ensure RFCs don't
remain open for too long.

Unknowns: As the RFC process gets used more and more, the process will continue to evolve.

## Prior art and references

[RFC-0001: RFC process](0001_rfc_process.md)

[RFC-0006: Addendum of the RFC process for Zircon](0006_addendum_to_rfc_process_for_zircon.md)
