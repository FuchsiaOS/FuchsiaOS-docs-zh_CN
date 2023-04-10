{% set rfcid = "RFC-0018" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-001.

## Summary

The FIDL Tuning Proposal (FTP) process is designed to provide a
uniform and recorded path for making changes to the [FIDL] language,
bindings, and tools.

## Motivation

There are several motivations for creating such an FTP system.

[FIDL][FIDL] (the Fuchsia IPC system) is subject to a number of design
constraints. These include performance, safety, and ergonomics. These
are often at odds with each other, and the requirement to support IPC
bindings in various target languages adds further tradeoffs. The FTP
proposal system provides a way to litigate and to record decisions
about these tradeoffs.

Recording decisions is valuable for several reasons. First, it
provides a way to prevent revisiting the same decisions over and over
when nothing has changed, while still allowing revisiting decisions
when underlying assumptions actually have changed. Second, it provides
new team members, or new clients of Fuchsia, some context into how
FIDL has evolved and why certain decisions were made.

Finally, FIDL, as a sort of programming language, invites bikeshedding
at a scale only [Wadler's law] can enable. This provides a place for
such things to occur that isn't a several hundred person email list.

## Design

An FTP (FIDL Tuning Proposal) goes through several stages. These
stages correspond to the Status: field of the heading of the [template].

### Draft

One or more people get excited about a change! They make a copy of the
tuning template, and start writing and designing. The proposal should
address each of the section headings in the template, even if it is
only to say "Not Applicable".

At this stage they may start soliciting feedback on the draft from impacted parties.

### Comment

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

### Review

At this point the FTP, along with all outstanding commentary, is
reviewed.

The proposal is reviewed by members of the Fuchsia FIDL team
(unofficially know as luthiers), and anyone they see fit to include or
to delegate to in the process. For example, they may include a
particular language expert when making a decision about that
language's bindings. If necessary, controversial decisions can be
escalated like any other technical decision in Fuchsia.

The review can ultimately have three outcomes.

First, there may be outstanding questions or feedback required to make
a decision. In this case the FTP is moved back to the Comment stage.

Second, the proposal may be Rejected, with reviewers providing a
rationale as to why.

Third, it may be Accepted.

### Rejected

Rejected FTPs are valuable records of engineering decisions. When
rejected, the rationale for rejected should be added to the FTP. The
FTP will then be copied to the public record of all FTPs for
posterity.

The given rationale should be actionable in the following two senses.

First, what would have to change about the world to have accepted this
proposal?

Second, the rationale should address any blocking comments raised
during the Comment period.

### Accepted

Accepted FTPs will also have a rationale section appended to them
after review, and will receive a tracking bug.

The same constraints apply to the acceptance rationale as the
rejection rationale. In particular, any blocking comments need to be
addressed.

Then it's off to the races to implement the change.

### Implemented

At this stage, the proposal is landed. All the code has been
changed. The [tutorial] has been updated. The bug is marked
done. [FIDL] is in a more perfect tuning.

The final step of the process is landing a markdown-ified version of
the FTP into the Fuchsia tree. This applies whether or not the
proposal was accepted, as being able to point at already considered
but rejected proposal is a substantial part of the value of this
process.

## Documentation and examples

This document (FTP-001) is the first such example of this process.

Ideally the [template], plus the final version of this proposal, are
sufficient documentation for the process.

## Backwards compatibility

n/a

## Performance

n/a

## Security

I believe this plan will have the modest benefit of providing a place
for security review to happen. Currently all changes to FIDL are
discussed via chat or code review. There's no paper trail, prior to
the FTP process.

## Testing

It feels easier to talk about success than about testing for this
plan.

The immediate success criteria for this process will be whether the
several outstanding ideas for changing FIDL go through the process
without it being onerous.

One long term success metric would be whether old FTPs are regularly
pointed at.

## Drawbacks, alternatives, and unknowns

There's a small cost to serializing changes to FIDL through a slightly
formal process. I believe that the cost is in fact small, in
comparison to the engineering work needed to implement any change
(especially as our ABIs harden and breaking changes get harder), and
to the payoff of recording these decisions.

The biggest alternative I considered was a more open
version. Currently, the comment and review process is currently only
visible or open to Googlers. I believe that this is the correct
decision for now, with an eye towards re-evaluating in the future.

I also wonder if there is a better way to capture commentary than a
Google Doc, especially at the point of "freezing" the FTP into an
accepted or rejected state.

I suspect we may want a version of this that captures decisions made
about FIDL prior to the adoption of this process.

Finally, I wondered about how formal to be about acception or
rejection criteria. I believe that this can evolve into something more
formal over time, if needed, with the help of early FTP's decision
rationales.

## Prior art and references

Several open source programming languages have enhancement proposals
or RFC mechanisms.

In particular, I looked a lot at the [Python PEP] process and the
[Rust RFC] process while drafting this document.

[FIDL]: /docs/development/languages/fidl/README.md
[Python PEP]: https://www.python.org/dev/peps/
[Rust RFC]: https://github.com/rust-lang/rfcs
[tutorial]: /docs/development/languages/fidl/tutorials/overview.md
[Wadler's Law]: https://wiki.haskell.org/Wadler's_Law
[template]: /docs/contribute/governance/deprecated-ftp-template.md
