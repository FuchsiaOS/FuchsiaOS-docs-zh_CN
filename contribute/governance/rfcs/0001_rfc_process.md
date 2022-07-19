{% set rfcid = "RFC-0001" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

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

### When to use the process {#criteria}

The vast majority of changes to Fuchsia do not require an RFC. Instead, these
changes can be made using the [code review
process](/development/source_code/contribute_changes.md). However,
technical decisions that have broad impact across the project require broader
agreement and must be socialized with the project using the RFC process.

The following kinds of changes must use the RFC process:

 * *Changing the project roadmap.* The project roadmap describes changes that
   have broad impact across the system, often touching a large fraction of the
   system or crossing boundaries between subsystems.

 * *Adding constraints on future development.* Some decisions, once made,
   constrain the future development of the system. We need to be careful when
   making such decisions because they can be difficult to revise later.

 * *Making project policy.* Project policies have broad impact across the
   system, often affecting contributors throughout the project. For example,
   changing the set of supported languages impacts everyone who needs to debug
   and understand the system, even if not everyone uses the new language.

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

The RFC process may also be used for other kinds of changes that would benefit
from its structured approach to decision making and its durable record of the
decision.

### Roles and responsibilities

People interact with the RFC process in several roles:

 * *RFC Authors.* An RFC Author is a person who writes an RFC. Everyone who
   contributes to Fuchsia can be an RFC Author. A given RFC can have one or more
   authors. The authors of a given RFC drive the process for that RFC.

 * *Stakeholder.* A stakeholder is a person who has a stake in whether the
   project accepts a given RFC. Stakeholders are typically Fuchsia contributors,
   but some RFCs might have stakeholders beyond the Fuchsia project. For
   example, stakeholders might be involved in other projects that use Fuchsia or
   are otherwise affected by changes to Fuchsia. Stakeholders do not always
   participate directly in discussions about RFCs. Instead, stakeholders are
   often *represented* by someone, often a technical lead or other person
   responsible for a group of stakeholders.

 * *Eng Council.* The [Eng Council](../eng_council.md) facilitate discussion
   and make the final decision as to whether the project accepts an RFC.

### How the process works

This section describes each step involved in the RFC process.

#### Step 1: Socialize {#socialize}

The first step in the RFC process is to socialize your idea with the project.
For example, you might have noticed a problem that you think is important to
solve. Are other people aware of this problem? Someone else might already be
working on the problem or might have some background or context about the
problem that would be useful to you. The earlier you discover this information,
the better.

Compared to the remaining steps in the process, this step is relatively
informal. This document does not contain a rigorous description of how to
socialize your idea. Socializing technical ideas is a skill unto itself.
However, a good place to start is to raise the topic in discussions with the
technical leads for areas related to the problem you are trying to solve. For
example, you might want to consult with people in the `OWNERS` files for the
areas of the codebase will need to be modified to execute your idea.

If you are unsure how to socialize your idea, consider asking a technical leader
for advice. They will often have more experience socializing ideas and might be
able to point you in a good direction.

> *Example.* This RFC was socialized by having a discussion in the Eng Forum,
> which is a regular meeting inside Google of various engineering leaders
> involved in the project. The RFC was also socialized with the creators of the
> FTP and CTP process, who have good background and context about these
> processes.

#### Step 2: Draft {#draft}

Once you have gathered all the background and context you can through
socialization, you are ready to start the formal part of the RFC process. The
next step is to write a first draft of the RFC document itself.

Mechanically, an RFC is a markdown file in the
`//contribute/governance/rfcs` directory.
To create and RFC, you create a CL that adds a file to that directory. You
should start by making a copy of the [RFC template](TEMPLATE.md). While not
strictly required, the template is designed to guide you towards writing a
high-quality RFC by prompting you to think through the problem you are trying to
solve in a semi-structured way.

Do not worry about assigning a number to your RFC at this stage. Instead, use
`NNNN` as a placeholder. For example, the file name should be something like
`NNNN_my_idea.md`. The RFC will get a number shortly before landing.

> *Suggestion.* Consider marking the CL containing your RFC as a
> "work-in-progress" until you are ready for feedback.

#### Step 3: Iterate {#iterate}

Once you have created a CL containing the first draft of your RFC, you are ready
to iterate on your idea with the appropriate stakeholders. Hopefully you will
have already discovered most the appropriate stakeholders as part of socializing
your idea, but you are very likely to discover additional stakeholders at this
stage.

Mechanically, you should invite stakeholders to provide feedback on your RFC by
adding them to the "Reviewers" or "CC" fields in the CL, as you would for a
normal code review. The stakeholders should provide you feedback by leaving
comments on your RFC in the code review tool.

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

If you wish to withdraw your RFC, you can mark the CL containing the RFC as
abandoned. You, or someone else, can always resurrect your RFC later if
circumstances change. If you are resurrecting an RFC created by someone else,
you should start the RFC process over from the beginning, but you can use the
withdrawn RFC as a starting point rather than `TEMPLATE.md`. Please confer with
the original authors to determine whether they wish to continue to have their
names associated with the new incarnation of the RFC.

> *Suggestion.* If you are interested in RFCs, consider configuring the Gerrit
> Code Review tool to [send you an email > notification](https://gerrit-review.googlesource.com/Documentation/user-notify.html)
> when a CL modifies the `//contribute/governance/rfcs` directory.

#### Step 4: Approve {#approve}

Once the iterations on the RFC are converging, you are ready to move to the
approval stage, in which the stakeholders sign-off on the RFC by setting the
Code-Review flag to either +1 or +2. Typically, stakeholders who need to approve
a CL (i.e., whose sign-off is required for the RFC to move forward) should
sign-off with a +2 whereas stakeholders whose approval is not required should
sign-off with a +1, but all stakeholders are welcome to sign-off with a +2 if
they wish to express their enthusiasm for the RFC.

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

#### Step 5: Submit {#submit}

If the project decides to accept your RFC, a member of the Eng Council will
comment on your CL stating that the RFC is accepted and will assign the RFC a
number, typically the next available number in the series. If there are any -1
or -2 Code-Review flags, the Eng Council will explicitly clear each flag by
summarizing the objection and by describing why the RFC is moving forward
despite the objection.

If the project decides to reject your RFC, a member of the Eng Council will
comment on your CL stating that the RFC is rejected and providing a rationale
for the rejection.  Rejected RFCs are valuable engineering artifacts. The Eng
Council will work with the RFC Authors to land a version of the RFC that is
marked as rejected and incorporates the rationale.

You should upload a new patchset of your RFC with the assigned number, both in
the title of the RFC and in the filename. If your RFC is approved and requires
implementation, please make sure you have an issue filed in the issue tracker
and put a link to the issue in the header of your RFC.

The Eng Council will then mark your CL Code-Review +2 and you can land your RFC!

*Congratulations! You have contributed a valuable engineering artifact to the
project!*

### How decisions are made {#how-decisions-are-made}

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

## Documentation

This RFC serves as documentation for the RFC process.

## Drawbacks, Alternatives, and Unknowns

The primary cost of implementing this proposal is that introducing a formal
decision-making process might slow down the pace of decision-making. The process
might be heavier than necessary for some kinds of decisions.

Recording decisions in the source repository has the effect of making those
decisions more difficult to change. That effect might be positive in some
scenarios, but the effect might also be negative in other scenarios.

The criteria in the ["when to use the process" section](#criteria) attempts to
mitigate this drawback by scoping the process to consequential situations but
such scoping is bound to have false positives and false negatives.

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
   language](/contribute/governance/deprecated-ftp-process.md). This
   proposal exists because of the success of that decision-making process.


