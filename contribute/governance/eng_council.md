# Fuchsia Eng Council Charter

## Overview

The Fuchsia Eng Council is a small group of senior technical leaders
responsible for providing a coherent technical vision for Fuchsia. The council
largely operates by delegation and ratification, promulgating engineering
standards, values, and objectives throughout the community and then reviewing
and ratifying concrete engineering proposals from project contributors.
Concretely, the Eng Council is charged with approving or rejecting Fuchsia RFCs
using the [Fuchsia RFC Process][rfc-process] and resolving technical disputes
that cannot be resolved within subteams.

## Goals

The goal of the Fuchsia Eng Council is to drive technical excellence in the
Fuchsia platform by providing a coherent technical vision for the project.

> *Any organization that designs a system (defined broadly) will produce a
> design whose structure is a copy of the organization's communication
> structure. — Melvin E. Conway*

By being a top-level node in the communication structure of the project, the
council helps produce a system design that makes coherent, project-wide
trade-offs.

In making trade-offs, the council aims to uphold the project’s values:

 * *Respect the user.* The council gives the most weight to factors that impact
   the end-user of products built using Fuchsia. For example, the council
   prefers designs that improve the security, privacy, and performance of the
   system because those aspects of the system directly benefit end-users.

 * *Respect the developer.* The council gives more weight to factors that
   impact end-developers, who write software that runs on Fuchsia, than to
   factors that impact project contributors, who write software that is part of
   Fuchsia itself. For example, breaking interface contracts might be
   convenient for project contributors but such changes impose costs on
   end-developers that must be weighed heavily.

 * *Be pragmatic.* The council prefers approaches that work well in practice
   over approaches that are perfect in theory. For example, the council favors
   designs that have been proven out by running code over designs that exist
   only on paper.

## Activities

The council is responsible for the following activities. The bulk of these
activities occur in public, but the council can communicate privately if the
council needs to consider non-public information.

### System architecture

The council maintains a set of documents that describe the system architecture.
These documents are descriptive of the current state of the system rather than
prescriptive about how the system architecture should evolve. Prescriptive
proposals for changing the system architecture should instead be published
using the [Fuchsia RFC Process][rfc-process].

The system architecture documents drive technical coherence throughout the
system because they help contributors understand how the system works overall
and how their part of the system fits into the whole.

### Engineering standards

The council maintains documentation about the engineering standards for the
project. The engineering standards describe the engineering values the project
applies when reviewing code contributions and design documents. For example,
the standards describe the level of testing expected for code contributions and
the balance between short- and long-term considerations in designs.

The council may delegate part of this responsibility. For example, it oversees
the [API Council][api-council], which is responsible for Fuchsia’s API surface.

### Engineering review {#eng-review}

The council facilitates engineering design reviews. The council establishes the
norm that engineering design documents should be published as RFCs, including
supporting RFC authors in socializing their designs and in identifying
appropriate stakeholders to review the designs in detail.

#### Request for comments (RFCs)

The most common way to review engineering designs will be through the
[Fuchsia RFC Process][rfc-process]. In this process, the council makes the
formal decision about whether the project accepts or rejects an RFC. The
council’s role in this process is largely to ensure that an RFC has received a
Code-Review +2 from an appropriate set of stakeholders, who are responsible for
reviewing the document in detail.

If there is a technical dispute that cannot be resolved between the RFC author
and one or more stakeholders, the council can resolve the dispute by accepting
or rejecting the RFC. If there is a technical dispute that arises in the course
of the project, the council can ask one of the disputants to record the
resolution of the dispute in an RFC.

#### Eng Reviews

The council can also review engineering designs in an Eng Review meeting. The
council prefers to use the RFC process when possible because the RFC process
allows for broader participation in the review. However, Eng Reviews can be
appropriate when the issue is time-sensitive, involves confidential
information, or when the discussion is too complex for a code review thread.

The council’s role in an Eng Review is largely to facilitate the discussion and
make a formal decision about the resolution of the issue being reviewed. The
non-confidential outcome of an Eng Review should be published in an RFC.

### Dispute resolution

The council is responsible for resolving technical disputes that cannot be
resolved within individual teams. Disputes can be escalated to the council by
people either directly or indirectly involved in the dispute. The council
prefers to resolve disputes by mediation, but the council is empowered to
arbitrate disputes that cannot be resolved through mediation.

## Decision process

The council makes formal decisions by
[rough consensus][rough-consensus]{:.external} among council
members, as assessed by the chair. If the council cannot come to rough
consensus, the chair will make the final decision.

## Membership

There is no predetermined number of people on the council. However, in order to
provide a coherent technical vision, the council has a small number of members.
Members are appointed by the governing authority for the project.

### Qualifications

Members are expected to meet the following criteria:

 * Members must be *deeply knowledgeable* about Fuchsia. Typically,
   contributors acquire this knowledge by working on the project for a
   substantial amount of time and by interacting with multiple parts of the
   system.

 * Members must be *widely respected* by the Fuchsia community. Although the
   council does have some formal decision-making authority, council members
   largely drive technical excellence indirectly through influence, which works
   best when members are widely respected in the community.

 * Members must have *strong conflict-resolution skills*. One important
   function of the council is to resolve technical disputes, which requires
   council members to exhibit strong conflict resolution skills, which often
   involves *strong communication* skills.

 * Members must also have a *demonstrated track record of technical
   leadership*. For example, a member might be the de facto or formal authority
   for a significant component of Fuchsia or might be someone whose expertise
   and judgment is sought after for evaluating proposed changes to the system.

Whether a candidate meets the criteria above is to be determined by the
appointing body. Council members need not be employed by any specific company
or organization.

### Current members

The current members of the Fuchsia Eng Council are listed in
[this OWNERS file][rfc-owners].

<!-- Reference links -->

[api-council]: /contribute/governance/api_council.md
[rfc-process]: /contribute/governance/rfcs/README.md
[rough-consensus]: https://en.wikipedia.org/wiki/Rough_consensus
[rfc-owners]: https://fuchsia.googlesource.com/fuchsia/+/HEAD/docs/contribute/governance/rfcs/OWNERS
