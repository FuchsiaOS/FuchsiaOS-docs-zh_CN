# Fuchsia contributor roles

## Overview {:#overview}

This document defines the roles associated with contributing to
the Fuchsia project.

## Principles {:#principles}

Roles within the Fuchsia project seek to embody the following principles:

*   _Transparency._ We are transparent and open about roles and requirements.
*   _Inclusivity_. Fuchsia lets anyone contribute to the project, regardless of
    their employer.We believe contributions from a diverse, open-source
    community are critical to improving Fuchsia.
*   _Responsibility_. Roles and privileges can be revoked if a person no
    longer meets the requirements.

## Roles {:#roles}

The following are the contributor roles associated with the Fuchsia project:

*   [Member](#member)
*   [Committer](#committer)
*   [Owner](#owner)
*   [Global Approver](#global-approver)

### Member {:#member}

Anyone who contributes to the project by providing patches to code or
documentation, and agrees to the Google Developers' [Contributor License Agreement](https://cla.developers.google.com/){:.external}.

#### Responsibilities {:#responsibilities}

Members are responsible for acting in accordance with
the [Fuchsia Code of Conduct](/CODE_OF_CONDUCT.md).

#### Become a Member {:#become-a-member}

To become a Member you must do the following:

*   Sign the Google Developers' [Contributor License Agreements](https://cla.developers.google.com/){:.external}.
*   Acknowledge the [Fuchsia Code of Conduct](/CODE_OF_CONDUCT.md).

### Committer {:#committer}

A Committer is a person who has write access to the
[Fuchsia repository](https://fuchsia.googlesource.com/){:.external}. A Committer can submit
their own Gerrit changes or Gerrit changes from any other member.

A Committer is not just someone who can make changes, but also someone who
demonstrated the ability to collaborate effectively with other Members of the
Fuchsia community. Example collaboration activities include but are not limited
to:

*   Seeking out the most knowledgeable people to review their code changes.
*   Contributing high-quality, well-tested code.
*   Fixing bugs in code or tests.

Members can become Committers with different kinds of contributions. For
instance, those working on documentation or toolchain can meet the requirements
to become Committers by contributing high-quality documentation or configuration
changes, which would not meet the “traditional” bar for well-tested code.

In order to submit Gerrit changes, Committers need to either be [Owners](#owner)
of the affected files or receive approval from an Owner of the affected files.

#### Responsibilities {:#responsibilities}

Committers are responsible for the following:

*   Ensuring that the code submitted to Fuchsia by Committers is tested
according to the [Testability Rubrics](/docs/concepts/testing/testability_rubric.md).
*   Ensuring that the code submitted to Fuchsia by Committers follows testing
best practices.

#### Become a Committer {:#become-a-committer}

To become a Committer you must do the following:

*   Contribute 10 non-trivial patches to the project, demonstrating the ability
to write high-quality, well-tested code.
*   Be nominated by a current Committer.
*   Obtain reviews and approvals of those 10 non-trivial patches from at least
2 different Committers.
*   Ensure that your nomination is supported by 3 other Committers.
*   Ensure that your nomination is not blocked by any Committer.

Committer nominations are evaluated within seven business days of the initial
nomination request.

### Owner {:#owner}

An Owner is responsible for files or directories within the Fuchsia project and
has comprehensive knowledge of the code in that subtree. Owners are listed in
`OWNERS` files. For directories or files that are outside of an Owner's
responsibility, that Owner has the same privileges as a Committer.

#### Responsibilities {:#responsibilities}

In addition to the responsibilities of a Committer and Member, Owners
are responsible for the following:

*   Nominating other Owners.
*   Approving or removing other Owners.
*   Provide high-quality reviews and design feedback.
*   Approve changes for code in their subtree.

#### Become an Owner {:#become-an-owner}

To become an Owner you must do the following:

*   Be a [Committer](#become-a-committer).
*   Submit a substantial number of non-trivial changes to the affected subtree.
*   Provide high-quality reviews and code design feedback.
*   Provide code reviews in a timely manner.
*   Self-nominate or be nominated by another Committer.
    *   To self-nominate, [submit a Gerrit change](/docs/development/source_code/contribute_changes.md)
    that adds yourself to the `OWNERS` file of your desired repository.
    Current Owners will evaluate your change and either accept or reject your
    request.

### Global Approver {:#global-approver}

A Global Approver is an Owner in the [root `OWNERS` file](/OWNERS){:.external}.
A Global Approver often makes large-scale changes that affect the entire Fuchsia
codebase. For example, Global Approvers are people who tend to maintain
various languages, toolchains, and other build system components.

For the full set of Global Approver expectations as well as the list of current
Global Approvers, see [the root `OWNERS` file](/OWNERS){:.external}.

While Global Approvers are empowered to provide a [Code-Review +2](https://gerrit-review.googlesource.com/Documentation/config-labels.html){:.external}
to large-scale changes, Global Approvers are not expected to have comprehensive
knowledge of the entire Fuchsia codebase.

#### Responsibilities {:#responsibilities}

In addition to the responsibilities of a Member, Committer, and Owner, Global
Approvers are responsible for the following:

*   Approving large scale changes within the Fuchsia codebase with a +2
    in Gerrit.
*   Providing timely reviews for large scale changes.

#### Become a Global Approver {:#become-a-global-approver}

To become a Global Approver you must do the following:

*   Demonstrate considerable proficiency in making large-scale changes across
    the entire Fuchsia codebase.
*   Self-nominate or get nominated by another Committer.
    *  To self-nominate, do the following:
        * [Submit a Gerrit change](/docs/development/source_code/contribute_changes.md)
          that adds yourself to the [root `OWNERS` file](/OWNERS){:.external}.
          Current Owners will evaluate your change and either accept or reject your
          request.
        * Email all [existing Global Approvers](/OWNERS){:.external}
          with your associated Gerrit change and wait one business day for discussion
          and approval. If you are being nominated, existing Global Approvers will be
          emailed by the individual nominating you.

## Code review actions {:#code-review-actions}

The types of code review actions you can provide depend on your role within
the Fuchsia project.

### Initiate a CQ Dry Run {:#initiate-a-cq-dry-run}

A CQ Dry Run runs your change against the available tests in the Commit Queue.
Committers, Owners, and Global Approvers can initiate a CQ Dry Run.

### Score code reviews  {:#score-code-reviews}

#### Code Review {:#code-review}

After you request a code review, reviewers can score your change.

Reviewers can label your change with a score of **-2, -1, 0, +1,** **or +2**.
For more information on review label definitions, see [Gerrit Code Review - Review Labels](https://gerrit-review.googlesource.com/Documentation/config-labels.html){:.external}.

Committers, Owners, and Global Approvers can score code reviews but only a
Global Approver or repository Owner can provide a **+2**.

### Submit approved changes {:#submit-approved-changes}

You need a **Code Review Label +2** to submit your change. A
**Code-Review Label +2** score can only be applied by a repository Owner or
Global Approver.

When a change is submitted, the change is moved to the Commit Queue (CQ).
The Commit Queue verifies, commits, and merges changes to the master branch.

## Role matrix {:#role-matrix}

This table summarizes the actions that each Fuchsia contributor role can
perform.

<table>
  <tr>
   <td><strong>Role</strong>
   </td>
   <td><strong>Create Change</strong>
   </td>
   <td><strong>Code-Review another Committer’s change</strong>
   </td>
   <td><strong>Provide Code-Review +2</strong>
   </td>
   <td><strong>Provide CQ+1 (dry run of CQ)</strong>
   </td>
   <td><strong>Submit Approved Change to CQ</strong>
   </td>
   <td><strong>Add or remove Owners</strong>
   </td>
  </tr>
  <tr>
   <td>Member
   </td>
   <td>Yes
   </td>
   <td><strong>No</strong>
   </td>
   <td><strong>No</strong>
   </td>
   <td><strong>No</strong>
   </td>
   <td><strong>No</strong>
   </td>
   <td><strong>No</strong>
   </td>
  </tr>
  <tr>
   <td>Committer
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td><strong>No</strong>
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td><strong>No</strong>
   </td>
  </tr>
  <tr>
   <td>Owner (outside owned subtree)
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td><strong>No</strong>
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td><strong>No</strong>
   </td>
  </tr>
  <tr>
   <td>Owner (in own subtree)
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
  </tr>
  <tr>
   <td>Global Approver
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
  </tr>
</table>

## Life of a change {:#life-of-a-change}

The following diagram depicts the high-level stages of what happens to a change
after its pushed to Gerrit.

![alt_text](/docs/contribute/community/images/change-resolution.png "Change approval process")

## Specialized roles {:#specialized-roles}

Areas within the Fuchsia repository may have their own unique requirements,
defining their own sets of roles and responsibilities, in addition to the ones
detailed above.

### API Reviewer {:#api-reviewer}

An API Reviewer is accountable for the quality and long-term
health of the [Fuchsia API Surface](/docs/glossary.md#fuchsia-api-surface).
API Reviewers collectively form the API
Council.

Any change that modifies the Fuchsia API Surface must receive an **API-Review+1**.
from a member of API Council in addition to the usual **Code-Review+2**.

For more details about the responsibilities of an API Reviewer and how the API
Council operates, see the [API Council Charter](/docs/contribute/governance/api_council.md).

#### API Reviewer membership {:#api-reviewer-membership}

To become an API Reviewer you must do the following:

*   Be a [Committer](#committer).
*   Demonstrate good judgement about the quality and long-term health of APIs.
*   Be appointed by the functional area of the Fuchsia project, as per the [API Council Charter](/docs/contribute/governance/api_council.md#membership).

### Eng Council member {:#eng-council-member}

The Fuchsia Eng Council is a small group of senior technical leaders responsible
for providing a coherent technical vision for Fuchsia. The Eng Council largely
operates by delegation and ratification, communicating engineering standards,
values, and objectives throughout the community and then reviewing and ratifying
concrete engineering proposals from project contributors.

#### Eng Council membership {:#eng-council-membership}

There is no predetermined number of people on the Eng Council. However, in order
to provide a coherent technical vision, the council has a small number of
members. Eng Council members are appointed by the governing authority for the
project.

For more details about the responsibilities of an API Reviewer and how the API
Council operates, see the [Fuchsia Eng Council Charter](/docs/contribute/governance/eng_council.md).

## Revoking Privileges {:#revoking-privileges}

When contributors no longer meet requirements, their role and
corresponding privileges can be revoked.

### Scenarios {:#scenarios}

Example scenarios for having privileges revoked include, but are not limited to,
the following:

*   Not acting in accordance with the [Fuchsia Code of Conduct](/CODE_OF_CONDUCT.md).
*   Committers repeatedly ignoring testability best-practices in their code
    reviews.
*   Owners discouraging people from requesting code reviews.
*   Owners being unresponsive to review requests.

### Process {:#process}

The process for revoking an individual’s role within the Fuchsia project
involves the following steps:

*   An Owner makes a recommendation to `community-managers@fuchsia.dev` to
    revoke someone’s role, specifying the rationale. Revoking an Owner role
    needs to be approved by an Owner in the same subtree
    or above.
    * Ownership is often revoked when an Owner is no longer actively
      contributing to their associated files or directories.

Revoking a Committer role should be a rare action and requires approval by the
governance authority. Community managers should be involved in the process of
revoking the Committer role.

## Frequently asked questions {:#frequently-asked-questions}

As a Fuchsia Member, you might have the following questions about requesting a
code review:

*   Who can provide a **Code Review +1**?
    * All Committers, Owners, and Global Approvers. Code Review +1 means
    “Looks Good To Me” but a +1 alone doesn’t allow for submission.
    Someone else has to approve the change with a +2. For more information on
    review label definitions see, [Gerrit Code Review - Review Labels](https://gerrit-review.googlesource.com/Documentation/config-labels.html){:.external}.
*   Can specific portions of the Fuchsia source code have different requirements?
    * Yes. For example, API changes have special requirements as described in
     the [Fuchsia API Council Charter](/docs/contribute/governance/api_council.md#api_review).
*   Do I need **API-Review +1**?
    * Changes affecting the Fuchsia API surface require **API-Review +1**, and the
    code review tool will only show the API-Review flag when it is needed.
