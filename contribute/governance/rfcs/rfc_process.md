# Fuchsia RFC Process

The Fuchsia RFC process has evolved from the following RFCs:

* [RFC-0001: Fuchsia Request for Comments process](0001_rfc_process.md)
* [RFC-0006: Addendum of the RFC process for Zircon](0006_addendum_to_rfc_process_for_zircon.md)
* [RFC-0067: Additions to Fuchsia RFC process](0067_rfc_process_additions.md)
* [RFC-0017: The FTP Process is dead, long live the RFC Process!](0017_folding_ftp_into_rfc.md)
* [RFC-0122: RFC Stakeholders](0122_stakeholders.md)

This page collates the above RFCs and captures the current process.

After reviewing this process, you can follow
[this guide to create an RFC][create].

[TOC]

## Summary

The Fuchsia RFC process is intended to provide a consistent and transparent path
for making project-wide, technical decisions. For example, the RFC process can
be used to evolve the project roadmap and the system architecture.

## Motivation

Currently, the Fuchsia project does not have a formal process for making
project-wide, technical decisions. At our current scale, this informality
results in different people having different, sometimes inconsistent, viewpoints
on where the project is going and how the system is put together. By
establishing a consistent and transparent path for making project-wide,
technical decisions, all the stakeholders can be confident about the technical
direction of the project.

## Design

This section describes the design of the RFC process.

### When to use the process {#when-to-use-the-process}

The RFC process can be used for any change to Fuchsia that would benefit from
its structured approach to decision making and its durable record of the
decision.

The vast majority of changes do not _require_ an RFC. Instead, these changes can
be made using the [code review
process](/docs/development/source_code/contribute_changes.md). However,
technical decisions that have broad impact across the project require broader
agreement and must be socialized with the project using the RFC process.

The following kinds of changes must use the RFC process:

 * *Adding constraints on future development.* Some decisions, once made,
   constrain the future development of the system. We need to be careful when
   making such decisions because they can be difficult to revise later.

 * *Making project policy.* Project policies have broad impact across the
   system, often affecting contributors throughout the project. Examples
   include: changing the set of supported languages (impacts everyone who needs
   to debug and understand the system), deprecating a widely-used API, and
   changing testing requirements for a broad class of code changes.

 * *Changing the system architecture.* The system architecture describes how the
   system fits together as a whole. Changing the system architecture, by
   definition, crosses boundaries between subsystems and requires careful
   consultation with many stakeholders.

 * *Delegating decision-making authority.* There are often classes of decisions
   that the project needs to make frequently and that benefit from specialized
   expertise. Rather than making all these decisions through the RFC process,
   the project can delegate decision-making authority for those classes of
   decisions to another group or process. For example, we often need to make
   decisions about platform APIs, which add constraints on future development,
   but it would not be practical to use the RFC process for every change to the
   platform API.

 * *Escalations.* Finally, contentious changes can benefit from the transparency
   and clarity of the RFC process. If there is a disagreement about technical
   direction that cannot be resolved by an individual technical leader, the
   decision can be escalated to the RFC process either by one of the disagreeing
   parties or by another contributor.

In addition to the general considerations outlined above, some areas declare
additional criteria. Please consult these documents when relevant:

| Area                | Criteria RFC |
|---------------------|--------------|
| Component Framework | [RFC-0098](0098_component_framework_rfc_criteria.md)
| FIDL                | [RFC-0049](0049_fidl_tuning_process_evolution.md)
| Software Delivery   | [RFC-0103](0103_software_delivery_rfc_criteria.md)
| Zircon              | [RFC-0006](0006_addendum_to_rfc_process_for_zircon.md)

Other changes that might benefit of the RFC process are ones that require manual
or automated large scale changes of the codebase. For example how logs are
written or how error paths are handled. Rather than live with islands of
consistency, the aspiration is to find the best patterns and uniformly apply
them to the entire codebase.

### Roles and responsibilities {#roles-and-responsibilities}

People interact with the RFC process in several roles:

 * *RFC Authors.* An RFC Author is a person who writes an RFC. Everyone who
   contributes to Fuchsia can be an RFC Author. A given RFC can have one or more
   authors. The authors of a given RFC drive the process for that RFC.

 * *Eng Council.* The [Eng Council (FEC)](../eng_council.md) facilitate
   discussion and make the final decision as to whether the project accepts an
   RFC.

 * *Facilitator.* The person appointed by FEC to shepherd this RFC through the
   RFC process. Today, this person must be an FEC member.

 * *Stakeholder.* A stakeholder is a person who has a stake in whether the
   project accepts a given RFC. Stakeholders are typically Fuchsia contributors,
   but some RFCs might have stakeholders beyond the Fuchsia project. For
   example, stakeholders might be involved in other projects that use Fuchsia or
   are otherwise affected by changes to Fuchsia. Stakeholders do not always
   participate directly in discussions about RFCs. Instead, stakeholders are
   often *represented* by someone, often a technical lead or other person
   responsible for a group of stakeholders.

 * *Reviewer(s).* The stakeholders whose +1 or -1 will be considered when the
   FEC decides to accept or reject the RFC. (While a +2 is the "approve" on code
   CLs, we tend to look to reviewers to +1 or -1 to indicate their support or
   lack thereof, and look to the facilitator to +2 upon approval.)

 * *Consulted.* The stakeholders whose feedback on the RFC was sought, but whose
   +1 or -1 is not considered when the FEC decides to accept or reject the RFC.

### How the process works

This section describes each step involved in the RFC process.

#### Step 1: Socialize

The first step in the RFC process is to socialize your idea with the project.
For example, you might have noticed a problem that you think is important to
solve. Are other people aware of this problem? Someone else might already be
working on the problem or might have some background or context about the
problem that would be useful to you. The earlier you discover this information,
the better.

Please note that the idea does not need to be polished before starting this step.
It's best to start socializing as early as possible to receive feedback on
whether the idea is feasible and if the direction is correct. This can potentially
save the authors time and effort in case the idea does not materialize or if
the direction needs to change significantly.

While mechanically, an RFC takes shape as a markdown file reviewed using a
Gerrit code change, using a more dynamic medium than a code review during the
socialization phase, e.g. Google Doc or other, can be beneficial. Should another
medium be chosen to socialize, it is strongly encouraged to carry over the
relevant context from the more dynamic medium over to RFC writeup. For instance,
back-and-forth conversations may lead to additional "alternatives considered"
entries to be added.

Compared to the remaining steps in the process, this step is relatively
informal. This document does not contain a rigorous description of how to
socialize your idea. Socializing technical ideas is a skill unto itself.
However, a good place to start is to raise the topic in discussions with the
technical leads for areas related to the problem you are trying to solve. For
example, you might want to consult with people in the `OWNERS` files for the
areas of the codebase will need to be modified to execute your idea.

During this phase, the RFC author should start to identify the stakeholders for
this RFC.

If you are unsure how to socialize your idea, consider asking a technical leader
for advice. They will often have more experience socializing ideas and might be
able to point you in a good direction.

> *Example.* This RFC was socialized by having a discussion in the Eng Forum,
> which is a regular meeting inside Google of various engineering leaders
> involved in the project. The RFC was also socialized with the creators of the
> FTP and CTP process, who have good background and context about these
> processes.

*Exit criteria*: None specifically. This is per the author's discretion.
This step is meant to help the author crystalize the goal(s) and potential solutions.
If they feel that this is accomplished, then they can proceed to the next step.

#### Step 2: Draft {#draft}

Once you have gathered all the background and context you can through
socialization, you are ready to start the formal part of the RFC process. The
next step is to write a first draft of the RFC document itself.

Mechanically, an RFC is a markdown file in the
`//docs/contribute/governance/rfcs` directory. To create an RFC, you create a
CL that adds a file to that directory. You must start by making a copy of the
[RFC template](TEMPLATE.md). The template is designed to guide you towards
writing a high-quality RFC by prompting you to think through the problem you are
trying to solve in a semi-structured way.

Any other files that are part of the RFC, diagrams for example, can be added to
the `resources` directory under a subfolder with the same name as the RFC itself.
Example:`//docs/contribute/governance/rfcs/resources/<RFC_name>/diagram.png`.

Do not worry about assigning a number to your RFC at this stage. Instead, use
`NNNN` as a placeholder. For example, the file name should be something like
`NNNN_my_idea.md`. The RFC will get a number shortly before landing.

The RFC author should propose an initial set of stakeholders in consultation
with the experts in their RFC [area](/docs/contribute/governance/areas). The set
of stakeholders may initially be left empty or incomplete. If there is any
ambiguity, they should consult FEC for assistance identifying stakeholders.

> *Tip.* Consult the [RFC best practices doc](best_practices.md) for advice
> about drafting and iterating on your RFC.

> *Suggestion.* Consider marking the CL containing your RFC as a
> "work-in-progress" until you are ready for feedback.

The act of uploading the CL is sufficient to get a facilitator assigned to
your RFC. FEC monitors new RFC CLs as the
signal to identify the right facilitator for the new RFC.

*Exit criteria*: CL containing your RFC is created.

#### Step 3: Iterate {#iterate}

Once you have created a CL containing the first draft of your RFC, you are ready
to iterate on your idea with the appropriate stakeholders. Hopefully you will
have already discovered most the appropriate stakeholders as part of socializing
your idea, but you are very likely to discover additional stakeholders at this
stage. RFC author(s) should request from the FEC to identify all stakeholders
early in the process, thus reducing the likelihood of a surprise at the
submission step.

Mechanically, you should invite stakeholders to provide feedback on your RFC by
adding them to the "Reviewers" (for stakeholders whose +1 is required) or "CC"
fields (for "consulted" stakeholders) in the CL, as you would for a normal code
review. In addition, you may email your CL to eng-council-discuss@fuchsia.dev
soliciting additional feedback. The stakeholders should provide you feedback by
leaving comments on your RFC in the code review tool.

Anyone can propose an additional stakeholder for a given RFC, including
themselves, by commenting on the RFC CL, although these proposals may not always
be accepted. If there is broad agreement, the RFC author should add the
stakeholder. FEC may also request that the author add stakeholders.

A stakeholder may 'opt out' and ask to be removed, or may delegate their review
(for example, to another expert in the relevant area). FEC may request that a
stakeholder be removed or moved from "reviewer" to "consulted".

If the discussion is too complex for the code review tool, consider scheduling a
meeting with the relevant stakeholders to have a more efficient discussion.
After the meeting, you must post a summary of the meeting in a comment on the CL
so that people who were not at the meeting can understand what was discussed
during the meeting.

If the discussion becomes contentious, please escalate to one of the RFC
Editors. The Eng Council can help move the discussion forward, for example by
providing additional structure to the discussion or moving the discussion to
another forum. Regardless of how the discussion proceeds, the results of any
off-CL discussion must be captured in the CL, often by posting a summary of the
discussion as a CL comment.

Feedback may include comments from people who are not stakeholders. The author
should respond to these comments if relevant, but settling them is not
necessarily required to move to the last call stage. If the comments point to a
disagreement about who is a stakeholder, FEC can help resolve this.

At FEC's discretion, RFCs that would benefit from more socialization should be
scheduled for an [engineering
review](/docs/contribute/governance/eng_council.md#eng-review) meeting. Some
triggers leading to scheduling an engineering review are:

 * Difficulty to identify relevant stakeholders(s). It might be the case than an
   RFC receives many comments, suggestions, push back, and that the author(s)
   are unclear how to act on this feedback, and which represents core feedback
   which is potentially a blocker to the RFC being accepted, vs auxiliary
   feedback which may be curiosity, future plans, etc.
 * Difficulty for RFC author(s) and stakeholder(s) to converge on open items.

If you wish to withdraw your RFC, you can mark the CL containing the RFC as
abandoned. You, or someone else, can always resurrect your RFC later if
circumstances change. If you are resurrecting an RFC created by someone else,
you should start the RFC process over from the beginning, but you can use the
withdrawn RFC as a starting point rather than `TEMPLATE.md`. Please confer with
the original authors to determine whether they wish to continue to have their
names associated with the new incarnation of the RFC.

*Note to reviewers:* The RFC process is meant to encourage a variety of
perspectives and vibrant discussions. Often, giving negative feedback in a public
forum might be difficult. If needed, reviewers can reach out to their leads,
peers or Eng Council to help them formulate the feedback so it can be delivered
effectively in the CL.

> *Suggestion.* If you are interested in RFCs, consider configuring the Gerrit
> Code Review tool to [send you an email > notification](https://gerrit-review.googlesource.com/Documentation/user-notify.html)
> when a CL modifies the `//docs/contribute/governance/rfcs` directory.

*Exit criteria:* All stakeholders identified and approved by Eng Council; feedback
solicited and incorporated.

#### Step 4: Last call {#last-call}

Once the iterations on the RFC are converging, the author must email
eng-council@fuchsia.dev requesting them to move the RFC's status to last call.
An Eng Council member will send an email to all stakeholders and
eng-council-discuss@fuchsia.dev to solicit any final feedback before moving to
the decision step. The RFC will be open for feedback for the next 7 calendar days.

Typically, reviewers sign off with a +1 and the facilitator will sign off with a
+2. Consulted stakeholders may also sign off with a +1 or +2 if they wish to
express their enthusiasm for the RFC, although this is not required.

Stakeholders who wish to object to an RFC can set the Code-Review flag to -1 or
-2, depending on how strongly they feel that the RFC should not move forward.
When setting the Code-Review flag to -1 or -2, a stakeholder must state their
reason for objecting, ideally in a way that would let someone understand the
objection clearly without having to read the entire discussion that preceded
the objection.

A stakeholder setting the Code-Review flag to -1 or -2 does not necessarily
prevent the project from accepting the RFC. See the ["How decisions are made"
section](#how-decisions-are-made) below for more details about how the project
decides whether to accept an RFC.

After all the stakeholders have weighed in with their Code-Review flags, send an
email to eng-council@fuchsia.dev to prompt the Eng Council to decide whether to
accept your RFC.

*Exit criteria:* Feedback provided by all stakeholders; all feedback addressed.

#### Step 5: Submit

If the project decides to accept your RFC, a member of the Eng Council will
comment on your CL stating that the RFC is accepted and will assign the RFC a
number, typically the next available number in the series. If there are any -1
or -2 Code-Review flags, the Eng Council will explicitly clear each flag by
summarizing the objection and by describing why the RFC is moving forward
despite the objection. Eng Council will indicate if any additional information
needs to be documented in your RFC, such as rationale for a different approach
or tradeoffs being made.

If the project decides to reject your RFC, a member of the Eng Council will
comment on your CL stating that the RFC is rejected, provide a rationale
for the rejection and will assign the RFC a number. Rejected RFCs are valuable
engineering artifacts. The Eng Council will work with the RFC Authors to land
a version of the RFC that is marked as rejected and incorporates the rationale.

If the Eng Council identifies one or many unresolved open items, the RFC may be
moved back to the [iterate](#iterate) step. The Eng Council will ask of the
author(s) to resolve the open items identified with the relevant stakeholders
before another request to review the RFC will be granted.

You should upload a new patchset of your RFC with the assigned number, both in
the title of the RFC and in the filename. If your RFC is approved and requires
implementation, please make sure you have an issue filed in the issue tracker
and put a link to the issue in the header of your RFC.

The Eng Council will then mark your CL Code-Review +2 and you can land your RFC!

*Congratulations! You have contributed a valuable engineering artifact to the
project!*

*Exit criteria:* RFC number assigned; any applicable rationale, tradeoffs and
Eng Council feedback incorporated; RFC merged.

### How decisions are made

The decision whether to accept an RFC is made by the Eng Council, acting in
[rough consensus](https://en.wikipedia.org/wiki/Rough_consensus) with each
other. If the decision involves an RFC that has Eng Council members as authors,
those members must recuse themselves from the decision.

If the Eng Council cannot reach rough consensus, the RFC is not accepted.
In deciding whether to accept an RFC, the Eng Council will consider the
following factors:

 * Does the RFC advance the goals of the project?
 * Does the RFC uphold the values of the project?
 * Were all of the stakeholders appropriately represented in the discussion?
 * If any stakeholders objected, does the Eng Council understand the objections
   fully?

Decisions made by the Eng Council can be escalated to the governing authority
for the project.

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

## Documentation

This RFC serves as documentation for the RFC process.

## Drawbacks, Alternatives, and Unknowns

The primary cost of implementing this proposal is that introducing a formal
decision-making process might slow down the pace of decision-making. The process
might be heavier than necessary for some kinds of decisions.

Recording decisions in the source repository has the effect of making those
decisions more difficult to change. That effect might be positive in some
scenarios, but the effect might also be negative in other scenarios.

The criteria in the ["when to use the process"
section](#when-to-use-the-process) attempts to mitigate this drawback by scoping
the process to consequential situations but such scoping is bound to have false
positives and false negatives.

There are a large number of possible alternative strategies for solving the
underlying problem. For example, we could use a decision-making process that
centers around a synchronous meeting, but such a process will have difficulty
scaling to a global open-source project. We could also have selected a different
decision-making mechanism that balanced more towards consensus or more towards
authority.

## Prior art and references

There is a good deal of prior art about decision-making processes for
open-source projects. This proposal is strongly influenced by the following
existing processes:

 * *IETF RFC process.* The IETF has run a successful, large-scale
   [decision-making process](https://ietf.org/standards/process/) for a long
   period of time. The process described in this document draws a number of
   ideas from the IETF process, including some of the terminology.

 * *Rust RFC process.* The Rust community runs an [RFC
   process](https://github.com/rust-lang/rfcs/blob/HEAD/text/0002-rfc-process.md),
   which has been effective at making decisions for somewhat similar software
   engineering project. The process described in this document is fairly
   directly modelled after the Rust RFC process.

 * *Blink Intent-to-implement process.* The Chromium project runs a
   [decision-making process](https://www.chromium.org/blink/launching-features)
   for behaviors that affect web pages. The process described in this document
   is informed by my (abarth) experience helping to design and run that process
   for a period of time.

 * *FIDL Tuning Proposal.* The Fuchsia project has had direct experience using a
   similar process [to make decisions about the FIDL
   language](/docs/contribute/governance/deprecated-ftp-process.md). This
   proposal exists because of the success of that decision-making process.

[swd]: /docs/contribute/governance/rfcs/0103_software_delivery_rfc_criteria.md
[create]: /docs/contribute/governance/rfcs/create_rfc.md
