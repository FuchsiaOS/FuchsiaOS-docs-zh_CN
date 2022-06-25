{% set rfcid = "RFC-0049" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-049.

## Summary

We propose various evolutions of the FTP Process:

* Most importantly by **splitting the review from decision making**;
* Introduce a way for **authors to withdraw their FTPs**;
* Provide **guidelines for what needs an FTP**; and lastly
* Describe **other avenues to contribute to FIDL**.

## Motivation {#motivation}

The FIDL Tuning Process was [introduced](contribute/governance/rfcs/0018_ftp_process.md)
over a year ago. By all accounts, this process has successfully improved on
the three motivations and goals that drove its creation: carefully considering
design constraints; the thinking behind each decision is transparent, recorded,
and public; and lastly this process has provided a forum for discussing ideas
(sometimes at length), while at the same time giving closure.

The net result has been a sustained velocity of changes, and a playbook to
iterate. To date, 48 proposals (dubbed "FTPs") have been submitted, of which 25
have been accepted, and 9 rejected (all with properly recorded rejection
rationales).

And yet, like all things, there is room for improvement. Specifically, we look
at:

* How to best review FTPs?
* When can an FTP be withdrawn?
* Which changes necessitate an FTP?
* What are other ways to contribute to the design and evolution of FIDL?

## Design

### Reviewing FTPs {#reviewing}

The process is currently silent as to the manner in which an FTP should be
reviewed, noting only "The proposal is reviewed by members of the Fuchsia FIDL
team (unofficially known as luthiers), and anyone they see fit to include or to
delegate to in the process."

The most common forum to review an FTP has been in person meeting. The meeting
starts with the FTP author(s) presenting their design. The meeting facilitator
(often the FIDL lead today) will then work through the comments in the design
doc, possibly starting with setting up a quick agenda of 'open items' that
should be discussed.

Unlike the 'eng review process', it has been common to resolve the open items
during the meeting (with notes taken, and later incorporated into the FTP), and
ending the meeting by the FIDL lead making a decision to accept or reject the
FTP.

The expectation of making a decision by the end of the meeting, as well as the
FIDL lead playing the dual role of facilitator and decision maker, has created
friction: a certain rush during FTP meetings, participants feeling pressure to
make their voices heard, last minute back-of-the-envelope design decisions in
order to ratify a quickly amended FTP.

To address this poor dynamic, we amend the 'Review' step, and add an explicit
'Decision making' step. Some language is borrowed from the Eng Review Process.
The 'Review' step is changed as follows:

> _At this point the FTP, along with all outstanding commentary, is reviewed._
>
> _The proposal is reviewed by members of the Fuchsia FIDL team (defined by an
> OWNERS file in the
> [fuchsia.git repository](/src/fidl/OWNERS), and unofficially known
> as luthiers), and anyone they see fit to include or to delegate to in the
> process. For example, they may include a particular language expert when making
> a decision about that language's bindings. If necessary, controversial decisions
> can be escalated like any other technical decision in Fuchsia._
>
> _[ADD] Most commonly, the review is conducted during one or multiple in-person
> meetings 'The FTP review meeting'. The review can also occur using asynchronous
> communication if appropriate)._
>
> _The FTP review meeting starts by the author(s) presenting their design. The
> facilitator will then work through the comments in the FTP, asking people who
> left comments in the doc to present their feedback._
>
> _[ADD] The facilitator and presenter are ideally different people. The goal of
> the facilitator is to ensure that all aspects of the design are addressed, and
> to keep the meeting flowing. Ideally, the facilitator does not have a particular
> stake in the outcome to avoid the perception of bias, and the presenter
> implicitly has a stake in the design they're presenting._
>
> _[ADD] We don't necessarily need to come to closure on every piece of feedback
> during the meeting or discuss every last comment (e.g., if there are a large
> number of comments or several comments are getting at the same underlying
> issue). Instead, the facilitator should optimize for giving the presenter a
> broad range of feedback rather than driving each point of debate to a
> conclusion. Pending open questions may be resolved in further review sessions,
> or during Decision making._
>
> ~~The review can ultimately have three outcomes.~~
>
> ~~First, there may be outstanding questions or feedback required to make a
> decision. In this case the FTP is moved back to the Comment stage.~~
>
> ~~Second, the proposal may be Rejected, with reviewers providing a rationale
> as to why.~~
>
> ~~Third, it may be Accepted.~~

The 'Decision making' step is added after the 'Review' step:

> _Within five (5) business days, members of the Fuchsia FIDL team (defined by
> [Owners](/src/fidl/OWNERS) file),
> with the ultimate decision maker being the Fuchsia FIDL team lead, decide on
> the outcome of the review._
>
> _The decision can ultimately have three outcomes._
>
> _First, there may be outstanding questions or feedback required to make a
> decision. In this case the FTP is moved back to the Comment stage._
>
> _Second, the proposal may be Rejected, with reviewers providing a rationale as
> to why._
>
> _Third, it may be Accepted._
>
> _Typically, the venue for decision making will take the form of a meeting. It
> may also be an email thread, or happen during a review meeting._

### Withdrawing FTPs

Sometimes, the author of an FTP wishes to withdraw their FTP.

The 'Withdrawn' step is added after the 'Comment' step:

> Withdrawn FTPs are valuable records of engineering ideation. When an author
> withdraws their FTP, the withdrawal rationale must be added to the FTP. The
> FTP will then be copied to the public record of all FTPs for posterity.
>
> The withdrawal rationale is written by the FTP author, possibly in conjunction
> with members of the Fuchsia FIDL team.
>
> The rationale should be actionable in the following two ways.
>
> What did the author learn through the FTP process that would have led them to
> propose an alternative design?
>
> What are alternatives to the withdrawn FTP which are promising?

### Criteria for requiring an FTP {#criteria}

It's understood that not all changes to 'FIDL' require an FTP. No one expects a
design doc for a small refactoring in `fidlc`. At the other end of the spectrum,
introducing a new message layout, or changing the wire format absolutely
requires an FTP.

What do we think of the threshold over which a change rises to needing an FTP?
The general rule we follow is:

_A change MUST go through the FTP process when either:_

1. _The **solution space is large**, i.e. the change is one of many possibly good
   other solutions and there is a difficult design tradeoff to make;_

2. _The **change has a large impact**, i.e. The change modifies the behavior of FIDL
   in a substantial way such that it may introduce risk to many-or-all users of
   FIDL;_

3. _The **change has a large scope**, i.e. The change touches enough pieces of FIDL
   such that careful attention is required to determine whether it may or may
   not have a large impact._

For instance, changes to the following areas will likely require an FTP:

* FIDL governance
* Design principles
* Language grammar
* Type system
* Protocol semantics
* Wire format
* Bindings specification

Here are some example FTPs, and the areas they changed:

* [RFC-0047: Tables](contribute/governance/rfcs/0047_tables.md):<br />
  Type system) Introduced a new way to represent record-like data, first use
  of envelopes.<br />
  Wire format) New wire format for tables, and envelopes.<br />
  Bindings specification) Some API recommendation for bindings to follow.

* [RFC-0023: Compositional Model for Protocols](contribute/governance/rfcs/0023_compositional_model_protocols.md):<br />
  Language grammar) Replaced interface syntax, with protocol syntax.<br />
  Protocol semantics) Made explicit the semantics of protocol composition,
  including absence of an "is-a" relationship is supported.<br />
  Bindings specification) Forbid bindings to leverage polymorphism to<br />
  model composition.

* [RFC-0027: You only pay for what you use](contribute/governance/rfcs/0027_you_only_pay_what_you_use.md):<br />
  Design principles) Introduced a new design principle.<br />
  FIDL Governance) Explicitly called for the newly introduced design<br />
  principle to be considered as part of the FTP process.

* [RFC-0024: Mandatory Source Compatibility](contribute/governance/rfcs/0024_mandatory_source_compatibility.md):<br />
  FIDL governance) Modified the FTP template to add callout for source<br />
  compatibility.
  Bindings specification) Bootstrapped source compatibility requirements on<br />
  bindings.

* [RFC-0049: FIDL Tuning Process Evolution, i.e. this change](contribute/governance/rfcs/0049_fidl_tuning_process_evolution.md):<br />
  FIDL governance) Aims to provide additional guidance to the FTP process,<br />
  and recognize alternate ways to contribute to FIDL.

In contrast, here are examples of changes that didn't rise to being FTPs:

* Creating new FIDL bindings (e.g. the LLCPP bindings):<br />
  FIDL is designed to support many implementations interacting together.<br />
  Creating new bindings is expected, and can be done without any ratification<br />
  from the Fuchsia FIDL team.

* Make references to enum members explicit[[1]](#footnote1):<br />
  Before the change `my.library.MY_ENUM_MEMBER` was supported, and it was
  replaced by explicit syntax `my.library.MyEnum.MY_ENUM_MEMBER`. Despite
  changing the language grammar, this change is a bug fix of a feature that
  previously existed. In the case where a library had both an enum member
  `CLASHING_NAME` and a constant `CLASHING_NAME`, referencing either was
  ambiguous, and `fidlc` resorted to hacky type resolution rules to break the
  tie. The scoping rules for members were unchanged, members are scoped to their
  declaration, and both `fidlc` and all bindings respect that rule.

* Intent to Implement: Deferring Type Construction Post Raw to Flat Conversion:<br />
  While a rework of the implementation of the type system, no observable changes
  were done. This is a refactoring effort.

* Intent to Implement: Changing our Representation of 'Messages':
  While the JSON IR is impacted by this change, this presentational change makes
  certain concepts that previously needed to be implicitly known by bindings
  (header shape), explicitly known. Again, no observable change, and no impact
  beyond code structure (in `fidlc`, and in bindings).

### Contributing to FIDL, beyond FTPs

The Fuchsia FIDL team has a goal to "foster collaboration and inclusiveness
around FIDL" and specifically to ensure that "the Fuchsia team at large feels
they can be heard about rough edges, and contributions by non-FIDL team members
are appropriately guided and supported to land, or rejected early with a
rationale."

Authoring an RFC [typically](contribute/governance/rfcs/0030_fidl_is_little_endian.md)
requires quite a bit of work, and a
knowledge of the solution space to properly justify the specific design choice
made relative to alternatives. The FTP process is also quite heavy, and can be
in itself displeasing. As a result, the FTP process by itself does not help the
collaboration goal.

Other ways to contribute are:

* Discussing use cases and issues on the Fuchsia FIDL Team chat room. All
  members of the team closely follow conversations, and it's frequent that
  threads in this forum lead to changes small-and-large, or a prioritization
  shift.

* Participate in [Fuchsia API
  review](contribute/governance/api_council.md). This venue is key in
  seeing concrete use cases and measuring how well various FIDL features combine
  to support them. Multiple evolution have been driven by recognizing a pattern
  of API design that could be bolstered by tweaks to features, or new features
  altogether.

* Filing bugs against the Fuchsia FIDL team.

* Describing a problem statement, and working with the Fuchsia FIDL team and the
  Fuchsia team at large to explore possible solutions.

* Describing a possible solution, and working with the Fuchsia FIDL team and the
  Fuchsia team at large to evaluate it.

* Prototyping alternative approaches.

* Joining the FIDL team as a '20%-er', and working on FIDL team assigned
  projects.

All these could lead to capturing a crisp problem statement, doing a direct
change to FIDL, or turn into an FTP.

## Implementation strategy

Update the [FIDL Tuning Proposals page](contribute/governance/deprecated-ftp-process.md). Communicate this change
broadly. Follow through on the changes when conducting reviews.

## Ergonomics

No impact.

## Documentation and examples

See implementation strategy.

## Backwards compatibility

No impact.

## Performance

No impact.

## Security

No impact.

## Testing

No impact.

## Drawbacks, alternatives, and unknowns

While there are many ways to amend a process, we believe the proposed change is
a modest iteration that is expected to have a net benefit. Future process
evolutions are expected.

## Prior art and references

Many. One is the 'Eng Review' process that Fuchsia has adopted.

--------------------------------------------------------------------------------

##### Footnote1

CLs
[fxr/299869](https://fuchsia-review.googlesource.com/c/fuchsia/+/299869/),
[fxr/301089](https://fuchsia-review.googlesource.com/c/fuchsia/+/301089/),
[fxr/300672](https://fuchsia-review.googlesource.com/c/fuchsia/+/300672/),
[fxr/302294](https://fuchsia-review.googlesource.com/c/fuchsia/+/302294/),
[fxr/302728](https://fuchsia-review.googlesource.com/c/fuchsia/+/302728/).
