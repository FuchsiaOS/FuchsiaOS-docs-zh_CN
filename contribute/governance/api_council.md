{% set areas | yamlloads %}
{% include "docs/contribute/governance/areas/_areas.yaml" %}
{% endset %}

# Fuchsia API Council Charter

## Overview

This document describes the *Fuchsia API Council*, which is a group of people
who are accountable for the quality and long-term health of the Fuchsia API
Surface. The council will collaborate constructively with the people who create
and modify Fuchsia’s APIs to help guide the evolution of those APIs. The council
will communicate its decisions clearly, including the underlying rationale, and
will document best practices by contributing to Fuchsia’s API readability
rubrics.

## Definitions

The *Fuchsia System Interface* is the binary interface that the Fuchsia
operating system presents to software running on the system. For example, the
entry points into the vDSO as well as all the FIDL protocols used by the system
are part of the Fuchsia System Interface.

A *client library* is a library that people writing software for Fuchsia might
choose to use rather than interfacing directly with the Fuchsia System
Interface. For example, FDIO is a client library that provides a POSIX-like
abstraction over the underlying fuchsia.io protocol in the Fuchsia System
Interface.

The *Fuchsia IDK* is a collection of libraries, metadata, and tools that the Fuchsia
project provides to people writing software for Fuchsia. Among other things, the
Fuchsia IDK contains a definition of the Fuchsia System Interface as well as a
number of client libraries.

The *Fuchsia API Surface* is the collection of artifacts that we include in the IDK.
This includes, but is not limited to, the Fuchsia System Interface,
the client libraries included in the Fuchsia IDK, the IDK metadata, and the core
Fuchsia developer tools.

*Fuchsia contributors* are people who are involved in creating the Fuchsia
operating system, including people who work for Google and people who do not.

*Fuchsia API* designers are people who create or modify the Fuchsia API Surface,
including people who work for Google and people who do not.

*End-developers* are people who write software that consumes the Fuchsia API
Surface.

*Users* are people who use devices that run the Fuchsia operating system.

## Goals

Ultimately, the end-goal of the Fuchsia API Council is to foster a healthy
software ecosystem around the Fuchsia operating system. Fostering a healthy
ecosystem requires balancing many concerns, including growing the ecosystem and
guiding the ecosystem towards particular outcomes.

### Values

The ecosystem has many participants who play many different roles. Ideally, we
would be able to design APIs that meet the needs of everyone in the ecosystem
simultaneously, but API designers are often called upon to make decisions that
involve trade-offs. The council should help API designers make these decisions
in a way that respects the following *priority of constituencies*:

1.  Users
1.  End-developers
1.  Contributors
1.  API designers
1.  Council members

For example, we should design APIs that protect user privacy, even at the
expense of not fulfilling all the desires of end-developers. Similarly, we
should design APIs that are better for end-developers even if those designs
place a higher burden on the people implementing the APIs.

These values help guide the ecosystem towards meeting the needs of users, which
promotes the health and growth of the ecosystem in the long run because users
are more likely to join, and remain in, an ecosystem that meets their needs.

### Strategy

To achieve these goals, the council focus on the following metrics:

*   *Functionality*. The council is accountable for the functionality of the
    Fuchsia API Surface. Specifically, functionality refers to whether the APIs
    meet the needs of the ecosystem participants. For example, the council is
    accountable for how well our APIs protect the privacy of users, how well our
    APIs help end-developers accomplish a given task, and how well our APIs let
    Fuchsia contributors improve their implementations over time.

*   *Usability*. The council is accountable for the usability of the Fuchsia API
    Surface. For example, the council should strive for consistency in how
    similar concepts are expressed in our APIs, which makes our APIs easier for
    end-developers to learn. Similarly, the council should ensure that our APIs
    are well-documented and that the semantics of interfaces are intuitive from
    their declaration.

*   *System impact*. The council is accountable for the burden on the system as
    a whole incurred through the use of the Fuchsia API Surface, including both
    intended and unintended usage. For example, APIs that use polling impose a
    large burden on the system because they require their clients to run
    continuously to monitor changes in conditions. Assessing system impact
    requires a significant amount of judgement and experience, especially to
    predict unintended uses of APIs.

*   *Communication clarity*. The council is responsible for clearly
    communicating decisions and the rationale behind those decisions to Fuchsia
    contributors. This communication should provide transparency about the
    decision-making process and should help educate API designers about how to
    create high-quality APIs. For example, the council should document best
    practices by contributing to Fuchsia’s API readability rubrics.

*   *Customer satisfaction*. The council is responsible for collaborating
    constructively with API designers. The council should foster an environment
    in which council members and API designers work in partnership to improve
    the Fuchsia API Surface. API designers should see the council as providing
    positive value, helping them make better APIs, rather than as bureaucratic
    burden. For example, council members should respond promptly and
    respectfully to requests for API reviews.

## Membership {#membership}

The council is comprised of Fuchsia contributors who have demonstrated:

*   Good judgement about the quality and long-term health of APIs, either within
    Fuchsia or in their past work with other platforms.

*   Strong communication and collaboration skills, as viewed by API designers
    (i.e., their collaborators).

Members are appointed by each [functional area](#area) of the project.

The council is overseen by the [Fuchsia Eng Council][eng-council].

The council has a *chair*, who is appointed by Fuchsia leadership and
facilitates the operations of the Fuchsia Eng council. The chair has the
following responsibilities:

* Schedules meetings.
* Sets the agenda for each meeting.
* Assesses if the council has reached [rough consensus][rough-consensus].

## Functional areas {#area}

<table class="fixed">
    <colgroup>
        <col width="30%">
        <col width="30%">
        <col width="40%">
    </colgroup>
    <thead>
        <tr>
            <th>Area</th>
            <th>Primary</th>
            <th>Secondary</th>
        </tr>
    </thead>
    <tbody>
        {% for area in areas %}
        <tr>
            <!-- Cannot using HTML, and getting links to be re-written, so putting direct link. -->
            <td><a href="https://fuchsia.dev/fuchsia-src/contribute/governance/areas/#{{ area.name|replace(" ", "-")|lower() }}">{{ area.name }}</a</td>
            <td>{{ area.api_primary }}</td>
            <td>{% if area.api_secondary == "" %}<i>none.</i>{% else %}{{ area.api_secondary }}{% endif %}</td>
        </tr>
        {% endfor %}
    </tbody>
</table>

As the project evolves, the list of functional areas (and therefore the makeup
of the council) will evolve as well. The list of functional areas is maintained
by Fuchsia leadership.

When considering adding an area, the council considers:

1. **Coverage.** Is this area already covered by one of the other areas, or is
   there a gap?
2. **Scope.** Is this area sufficiently large to warrant a dedicated appointee?
3. **Consistent need.**. Has the need to have this as an area arisen before?

## Decision process

If the council is called upon to make a decision, the decision process is as
follows. The council member for the area in question is the *primary decision
maker*, but the council as a whole is the *final decision maker*. The council as
a whole makes decisions by *rough consensus*, as assessed by the chair.

*   The primary decision maker can *defer* a decision, in which case the council
    will make the decision. If the council fails to reach rough consensus, the
    chair will make the final decision.

*   A council member can ask the council to *overrule* the primary decision
    maker. If the council fails to reach rough consensus, the decision made by
    the primary decision maker stands.

## Operations

The council has two major functions: API review and API calibration.

### API review

Every change to the Fuchsia API Surface requires approval from a council member.
A change in a particular functional area should typically be approved by the
council member responsible for that area, but any council member can approve the
change if the responsible council member is unavailable.

Before being merged, every change that modifies the Fuchsia API Surface must receive
an API-Review+1 from a member of [api-council@fuchsia.dev][api-council-group] in
addition to the usual Code-Review+2. The same person can provide both
API-Review+1 and Code-Review+2 for a given change, but someone cannot give their
own CLs API-Review+1. See [Review Labels][review-labels] for documentation about
this Gerrit feature.

For small API changes, especially incremental refinements to existing APIs, a
code review is usually sufficient for an API reviewer to give the change
API-Review+1. However, for larger changes, especially those that expand the API
surface significantly, the API designer should write an RFC (see [Fuchsia RFC
Template][rfc-template]), which explains the design of the API, including use
cases and examples, as well as security and privacy considerations. An API
reviewer can always request the API designer to write an RFC, even for small
changes if the API reviewer does not feel comfortable approving the change
purely through code review.

For inclusion in the Fuchsia SDK, an API must clear two hurdles: there must be a
ready and willing customer, and the API must have gone through [API
calibration](#calibration).

API designers are also encouraged to seek early feedback from council members.
For example, API designers should consider sharing work-in-progress API Design
Documents with council members to get input early in the design process. Council
members should engage in these discussions with the goal of partnering with API
designers to help design the best API. API designers can also seek feedback
early in the design process from the full council by asking the chair for a slot
in the agenda for an upcoming API calibration session (see the next section).

The API reviewer should work with the API designer to improve the API RFC to the
point where the API reviewer feels comfortable approving the document. An
approved document serves as the plan of record for the API in question. However,
individual CLs that modify the API surface still need to review API-Review+1
before being merged. API designers should expect that CLs that follow the plan
laid out in an approved API RFC should review API-Review+1 quite easily, even
from other council members.

API designers or reviewers can refer an API RFC to the full council by asking
the chair for a slot in the agenda for an upcoming API calibration session (see
the next section). For example, an API reviewer might refer a document to the
full council if the API reviewer does not feel sufficiently calibrated, if the
API is particularly complex or important, or if the reviewer feels pressured by
looming deadlines or other teams.

### API calibration {#calibration}

Periodically, the API council will meet for *API calibration*. The purpose of
API calibration is to promote consistency of API reviews across the project and
to improve the quality of API reviews by cross-pollinating best practices across
the council. These meetings often have a *facilitator*, who keeps the meeting on
topic and helps ensure each participant has a chance to provide their feedback.

Fuchsia contributors can observe API calibration meetings. Observing these
meetings can be a good way to learn best practices about evolving our API
surface.

#### Review RFCs with API changes

In some cases, an API change may warrant an [RFC][rfc] discussing the proposed
changes. The first priority in API calibration is to review any RFCs that
have been referred to the full council. If there are multiple pending documents,
the chair will select the order in which the council works through the
documents.

The API designer who wrote the document should present the document, providing
the council with the necessary context to understand the issues at stake.
Afterwards, the person who referred the document should lead a discussion of the
areas of the API design for which they are seeking feedback. Council members are
encouraged to focus their feedback on those areas but are free to provide
feedback about the document as a whole.

#### Review backlog

The Fuchsia API Surface contains a large number of APIs that were designed
before the council was formed. The council will work through that backlog of API
reviews, eventually reaching the point where every API in the Fuchsia API
Surface has been reviewed. Ideally, the council will have a chance to review the
entire Fuchsia API Surface before Fuchsia commits to the backwards compatibility
of its APIs.

The chair selects the order in which the council works through the backlog,
attempting to balance reviewing APIs from diverse areas of the project with the
urgency to review APIs that are accreting a large number of clients.

When reviewing an API, the council member who is responsible for the area that
contains the API (hereafter the *responsible member*) will present the API,
providing the council with the necessary context to understand the use cases and
motivation for the API. The responsible member can invite one or more subject
matter experts to help provide additional context and technical details.
Ideally, the responsible member will have pre-reviewed the API and will have a
list of proposed modifications.

#### Secondary review

The council will also cycle through the functional areas of the project,
performing a secondary review of changes to the API surface for each area since
the last cycle. This activity lets the council provide feedback to members on
their recent API reviews.

The chair will select the order in which the areas are reviewed, attempting to
balance reviewing APIs from diverse areas of the project with the urgency to
review APIs that have a large volume of changes.

During secondary review, the council member who was the primary reviewer for the
API change will present the change as well as any associated API Design
Documents, providing the council with the necessary context to understand the
use cases and motivation for the changes. The API designer who made the change
in question is also encouraged (but not required) to attend.

Generally, the council should respect the decisions made during the primary API
review, but council members are encouraged to provide feedback about how the API
could have been improved, which benefits future reviews. Depending on the
maturity of the API, the primary reviewer might decide to incorporate these
improvements into the API. In rare cases, the council can overrule the primary
reviewer, per the council’s decision process.

## Acknowledgements

This document draws heavily from the governance structure used by the Android
API Council, the Web API OWNERS, the W3C, and the IETF. Special thanks to Jeff
Brown, Dimitri Glazkov, Jeremy Manson, Rebecca Silberstein, and Greg Simon for
sharing their experience with API governance and for their thoughtful feedback
on early drafts of this document.

<!-- Reference links -->

[api-council-group]: https://groups.google.com/a/fuchsia.dev/forum/#!forum/api-council
[rfc-template]: /docs/contribute/governance/rfcs/TEMPLATE.md
[eng-council]: /docs/contribute/governance/eng_council.md
[review-labels]: https://gerrit-review.googlesource.com/Documentation/config-labels.html
[rfc]: /docs/contribute/governance/rfcs/rfc_process.md
[rough-consensus]: https://en.wikipedia.org/wiki/Rough_consensus
