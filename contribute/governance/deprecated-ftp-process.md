# FIDL language tuning proposals (FTP)

Note: This process was deprecated in [RFC-0017](/contribute/governance/rfcs/0017_folding_ftp_into_rfc.md).
FTP proposals should now use the [Fuchsia RFC process](/contribute/governance/rfcs/rfc_process.md).

## Process

The FIDL Tuning Proposal (FTP) process is designed to provide a
uniform and recorded path for making changes to the [FIDL] language,
bindings, and tools.

* See [RFC-0018: FTP process](/contribute/governance/rfcs/0018_ftp_process.md)
* See [RFC-0049: FIDL Tuning Process Evolution](/contribute/governance/rfcs/0049_fidl_tuning_process_evolution.md)

### Criteria for requiring an FTP

A change MUST go through the FTP process when either:

1. The **solution space is large**, i.e. the change is one of many possibly good
   other solutions and there is a difficult design tradeoff to make;

2. The **change has a large impact**, i.e. The change modifies the behavior of FIDL
   in a substantial way such that it may introduce risk to many-or-all users of
   FIDL;

3. The **change has a large scope**, i.e. The change touches enough pieces of FIDL
   such that careful attention is required to determine whether it may or may
   not have a large impact.

For instance, changes to the following areas will likely require an FTP:

* FIDL governance
* Design principles
* Language grammar
* Type system
* Protocol semantics
* Wire format
* Bindings specification

Additional details are provided in
[RFC-0049: FIDL Tuning Process Evolution](/contribute/governance/rfcs/0049_fidl_tuning_process_evolution.md).

### Design

An FTP (FIDL Tuning Proposal) goes through several stages. These stages
correspond to the `Status:` field of the heading of the [template].

#### Draft

One or more people get excited about a change! They make a copy of the
tuning template, and start writing and designing. The proposal should
address each of the section headings in the template, even if it is
only to say "Not Applicable".

At this stage they may start soliciting feedback on the draft from impacted
parties.

#### Comment

At this stage, the FTP is formally circulated for commentary to the
Fuchsia engineering organization. The authors of the proposal should
solicit feedback from those especially likely to be impacted by the
proposal.

For now, proposals should be left open for comment for at least one
week, subject to reviewer discretion. It may be reasonable to be
shorter for less controversial FTPs, and longer to wait for feedback
from a particular person or group to come in.

Anyone may make a blocking comment on an FTP. Blocking comments do not
prevent a particular accept-or-reject outcome from the review process,
but reviewers are required to acknowledge the feedback given in the
comment as part of the final FTP.

#### Withdrawing

Withdrawn FTPs are valuable records of engineering ideation. When an author
withdraws their FTP, the withdrawal rationale must be added to the FTP. The FTP
will then be copied to the public record of all FTPs for posterity.

The withdrawal rationale is written by the FTP author, possibly in conjunction
with members of the Fuchsia FIDL team.

The rationale should be actionable in the following two ways.

What did the author learn through the FTP process that would have led them to
propose an alternative design?

What are alternatives to the withdrawn FTP which are promising?

#### Review

At this point the FTP, along with all outstanding commentary, is reviewed.

The proposal is reviewed by members of the Fuchsia FIDL team (defined by an
OWNERS file in the [fuchsia.git repository](/src/fidl/OWNERS), and unofficially known
as luthiers), and anyone they see fit to include or to delegate to in the
process. For example, they may include a particular language expert when making
a decision about that language's bindings. If necessary, controversial decisions
can be escalated like any other technical decision in Fuchsia.

Most commonly, the review is conducted during one or multiple in-person meetings
‘The FTP review meeting’. The review can also occur using asynchronous
communication if appropriate).

The FTP review meeting starts by the author(s) presenting their design. The
facilitator will then work through the comments in the FTP, asking people who
left comments in the doc to present their feedback.

The facilitator and presenter are ideally different people. The goal of the
facilitator is to ensure that all aspects of the design are addressed, and to
keep the meeting flowing. Ideally, the facilitator does not have a particular
stake in the outcome to avoid the perception of bias, and the presenter
implicitly has a stake in the design they're presenting.

We don't necessarily need to come to closure on every piece of feedback during
the meeting or discuss every last comment (e.g., if there are a large number of
comments or several comments are getting at the same underlying issue). Instead,
the facilitator should optimize for giving the presenter a broad range of
feedback rather than driving each point of debate to a conclusion. Pending open
questions may be resolved in further review sessions, or during Decision making.

#### Decision making

Within five (5) business days, members of the Fuchsia FIDL team (defined by
[OWNERS](/src/fidl/OWNERS) file), with
the ultimate decision maker being the Fuchsia FIDL team lead, decide on the
outcome of the review.

The decision can ultimately have three outcomes.

First, there may be outstanding questions or feedback required to make a
decision. In this case the FTP is moved back to the Comment stage.

Second, the proposal may be Rejected, with reviewers providing a rationale as to
why.

Third, it may be Accepted.

Typically, the venue for decision making will take the form of a meeting. It may
also be an email thread, or happen during a review meeting.

#### Rejected

Rejected FTPs are valuable records of engineering decisions. When
rejected, the rationale for rejected should be added to the FTP. The
FTP will then be copied to the public record of all FTPs for
posterity.

The given rationale should be actionable in the following two senses.

First, what would have to change about the world to have accepted this
proposal?

Second, the rationale should address any blocking comments raised
during the Comment period.

#### Accepted

Accepted FTPs will also have a rationale section appended to them
after review, and will receive a tracking bug.

The same constraints apply to the acceptance rationale as the
rejection rationale. In particular, any blocking comments need to be
addressed.

Then it's off to the races to implement the change.

#### Implemented

At this stage, the proposal is landed. All the code has been
changed. The [tutorial] has been updated. The bug is marked
done. [FIDL] is in a more perfect tuning.

The final step of the process is landing a markdown-ified version of
the FTP into the Fuchsia tree. This applies whether or not the
proposal was accepted, as being able to point at already considered
but rejected proposal is a substantial part of the value of this
process.

<!-- xref -->
[FIDL]: /development/languages/fidl/README.md
[tutorial]: /development/languages/fidl/tutorials/overview.md
[template]: /contribute/governance/deprecated-ftp-template.md
