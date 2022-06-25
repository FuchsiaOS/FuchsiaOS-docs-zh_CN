{% set rfcid = "RFC-0017" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

This RFC **discontinues the FTP process**, **folds existing FTPs into RFCs**,
and **amends the RFC process** as follows:

 * Encourage to use of a more dynamic medium than a code review during the
   socialization phase;
 * Require the use of the RFC template, and include area specific portions in
   this RFC template;
 * Formalize the existence of criteria per area;
 * Have the [Fuchsia Eng Council (FEC)][fec] play a more active role in
   identifying stakeholders;
 * Add an FEC facilitated meeting to discuss an RFC, with specific triggers
   calling for meeting;
 * Submission step at author(s) discretion, with a seven business days SLA for
   an authoritative answer from the FEC.

## Motivation

With the introduction of the [RFC Process][rfc-process] in February 2020,
changes to FIDL which meet the [FTP criteria][ftp-049-criteria] would
technically also require an RFC. In actuality, the various FTPs which were
accepted or rejected since did not double up by also going through the RFC
process: both processes aim for similar goals, and have similar level of
formality, and review.

However, it is our desire to unify Fuchsia around a single review process for
technical changes, and are therefore looking to discontinue the FTP process, in
favor of the RFC process.

The FTP process has been [a][ftp-049-motivation]
[success][rfc-process-prior-art] for the two and a half years it ran, and we
additionally look to bring some of the lessons learned to the RFC process.

## Design

We first survey [differences](#differences) between the FTP process and the RFC
process, and then propose a set of [amendments](#amendments) to the RFC process.

Lastly, we describe how to [fold](#fold) all FTPs into RFCs to further
centralize all artifacts of Fuchsia technical decisions.

### Differences between the two processes {#differences}

#### Medium {#medium}

The FTP process does not mandate a medium, and author(s) are free to choose the
medium they believe is best whilst adhering to the template imposed. In
practice, all FTPs have started as [Google Doc][google-doc] until their approval
or rejection, at which point these documents were converted into a Markdown
document, and committed to the Fuchsia source tree. Final edits and editorial
polish were often done during conversion.

The RFC process mandates the use of a Gerrit change (a.k.a. a CL) as the medium,
and asks that iterations be done by using subsequent patch sets of the change
with stakeholders invited to be reviewers of the change.

It is the author's opinion that the ease of commenting on a Google Doc,
suggesting an edit, or making changes is a catalyst for a healthy technical
conversation. Multiple RFCs actually started in Google Doc themselves during
their [socialization][rfc-process-socialize] phase, such that their
[draft][rfc-process-draft] was actually closer to a finalized document.
When choosing another medium than Gerrit for socialization, care should be taken
to avoid arbitrarily limiting the target audience. For instance, the Google Doc
should be 'world accessible'.

#### Template

The FTP process strictly requires the use of the [FTP template][ftp-template],
and asks for all sections to be filled in, even if only to explicitly state "not
applicable".

In contrast, the RFC process [recommends but does not
mandate][rfc-process-draft] the use of the [RFC template][rfc-template].

By nature of being designed specifically for FIDL, the FTP template asks probing
questions which are specifically catering to this area. The FTP template has
evolved over time, to address new requirements. For instance, a specific call
out for source compatibility implications was added in [RFC-0024: Mandatory
Source Compatibility][ftp-024-backwards-compatibility]. This stricter format and
specific questions has helped FTP authors and their reviewers ensure their
design was complete.

Note: The FTP template was not publicly accessible prior to this change. This
lack of public access was purely logistical. To date, contributors have been
Googlers with @google.com e-mail addresses, the template lived in a Google Doc
due to the ease of copying it to create a new FTP, a Google Doc cannot be shared
openly per google.com domain policy, maintaining two copies (e.g. one in
Markdown and one in a Google Doc) leads to content skew.

#### Review

The FTP process most commonly reviews proposals during [one or multiple
in-person meetings][ftp-049-reviewing]. While using an asynchronous review
mechanism, such as comments on a Gerrit change, is also possible it is the
exception rather than the norm.

The RFC process encourages an asynchronous review mechanism, and author(s) are
[encouraged to schedule a meeting][rfc-process-iterate] if the discussion is too
complex.

An important distinction here is that the FTP review meeting is organized and
scheduled by the Fuchsia FIDL team, whereas the RFC process asks author(s) to
schedule the meeting. There is potentially an important information asymmetry
between author(s) and those driving processes (the FIDL team, the Fuchsia Eng
Council). Placing the responsibility of organizing review meetings with
author(s) instead of those driving the processes introduces an additional
hurdle, which may not be easy to overcome especially when it requires navigating
the project organization and knowing who to invite.

##### Criteria

The FTP process has [specific criteria][ftp-049-criteria] for when it should be
used.

Similarly, the RFC process has [specific criteria][rfc-process-criteria] for
when it should be used, and [special considerations for Zircon][rfc-0006] were
later added.

#### Decision making

The FTP process relies on the [Fuchsia FIDL team][fidl-team] to make decisions,
with the ultimate decision maker being the Fuchsia FIDL team lead.

The RFC process relies on relevant stakeholders voicing their approval or
refusal, with the [Fuchsia Eng Council (FEC)][fec] making the ultimate decision.

The two decision making approaches are similar, especially if you consider that
the FIDL team would be a key stakeholder for all RFCs modifying FIDL. One
important distinction is that the FTP process requires a "push model", where
authors must submit their design to the FIDL team. The RFC process is more in
the "pull model", it is the responsibility of the FIDL team to be proactive and
engage with relevant designs in order to be heard.

#### Service Level Agreement (SLA)

The FTP process has a specific SLA to provide an authoritative answer ("five
business days"). This was specifically added during the iteration phase on
[FTP-049][ftp-049] with someone commenting that "from the perspective of an FTP
author, it'd be good to know when the FIDL team will be making a decision on an
FTP".

The RFC process does not have an SLA today.

#### Numbering

The FTP process assigns a sequential number when an FTP is started.

The RFC process assigns a sequential number when an RFC is accepted or rejected.

### Amendments to the RFC process {#amendments}

**Medium** The RFC process should offer the use of a more dynamic medium than a
code review during the socialization phase, e.g. Google Doc or other. Arguably,
this is not a change to the RFC process which only mandates the use of a Gerrit
change for the formal part of the process, but acknowledging and encouraging
another medium during the socialization phase of an RFC may clear confusion.

The RFC process should also strongly encourage relevant context from the more
dynamic medium to be carried over to RFC writeup. For instance, back-and-forth
conversations may lead to additional "alternatives considered" entries to be
added.

**Template** The RFC process should require the use of a template. Each area may
have relevant probing questions or sections that should be included for
proposals in their respective area.

**Template: FTP specific** The RFC template should be augmented to include the
specific content of the FTP template for use by RFCs touching the FIDL area.

**RFC Criteria** Each area is encouraged to submit additional criteria for when
an RFC should be followed for their respective area. If criteria for an area
exists, the FEC will ensure that appropriate stakeholders are looped in.
Arguably, this is not a change to the RFC process since one can simply submit an
RFC to amend the process, e.g. what was done for [Zircon][rfc-0006]. However, by
streamlining the existence of criteria for all areas, we believe that more areas
will explicitly state what is "important" for them. In addition to ensuring
these areas are appropriately looped in, it will have the additional benefit of
documenting the relative importance of a change.

**RFC Criteria: FTP specific** The RFC process should include the criteria
identified in [FTP-049][ftp-049-criteria] for the FIDL area.

**Stakeholders** While RFC author(s) should do their best to identify
stakeholders, the FEC should have an active role in determining the stakeholders
as well. RFC author(s) should request from the FEC to identify all stakeholders
early in the process, thus reducing the likelihood of a surprise at the
submission step.

**Eng review meeting** At FEC's discretion, RFCs that would benefit from more
socialization should be scheduled for an [engineering
review][eng-council-eng-review] meeting. Some triggers leading to scheduling an
engineering review are:

 * Difficulty to identify relevant stakeholders(s). It might be the case than an
   RFC receives many comments, suggestions, push back, and that the author(s)
   are unclear how to act on this feedback, and which represents core feedback
   which is potentially a blocker to the RFC being accepted, vs auxiliary
   feedback which may be curiosity, future plans, etc.
 * Difficulty for RFC author(s) and stakeholder(s) to converge on open items.

**Submitting** Rather than gating submission of an RFC on conversations
converging — which may be hard for author(s) to identify in certain cases - the
RFC process should instead permit author(s) to request a review to be done by
the FEC. When such a request is made, the FEC has seven (7) business days to
answer authoritatively to the author(s). The answer is either:

 * Approval, _or_
 * Rejection, _or_
 * Unresolved open items: FEC identifies one or many unresolved open items, and
   asks the author(s) to resolve them with relevant stakeholders before another
   request to review the RFC will be granted.

### Folding FTPs into RFCs {#fold}

Mechanically, to fold FTPs into RFCs we will:

 * Re-number all FTPs to sequential numbering, following existing RFCs. For
   instance, as of this writing, FTP-001 would become RFC-0017.

 * Update each FTP writeup to display its RFC number as the main title, while
   also keeping a trace of its former FTP number. Keeping a record of the old
   numbering is important because many git commits or bugs reference FTPs by
   number.

 * For reference reason, we will want to keep a trace of the existence of the
   FTP process (e.g. many links in this RFC).

 * Going forward, FTPs which have now been converted to RFCs should be
   referenced by RFC number (and not by their historical FTP number).

## Implementation

Keep calm, and follow the process.

## Performance, security, privacy, and testing considerations

This proposal is expected to have a positive impact on performance, security,
privacy, and testing considerations.

## Documentation

We need to update:

 * All FTPs.
 * All references to FTPs, both in Markdown and in code comments.
 * The FIDL governance section.
 * The RFC process summary page.

## Drawbacks, alternatives, and unknowns

Keeping two formal processes which are roughly equivalent in the same technical
project introduces confusion. We do not consider leaving things as is to be an
alternative.

Each amendment to the RFC process proposed above can be evaluated in and of
itself. We believe that these are all changes which provide a net benefit.

## Prior art and references

This RFC consolidates the [RFC Process][rfc-process] with the FTP Process i.e.
[FTP-001: A Modest Proposal][ftp-001] and [FTP-049: FIDL Tuning Process
Evolution][ftp-049].

<!-- xrefs -->
[eng-council-eng-review]: contribute/governance/eng_council.md#eng-review
[fec]: contribute/governance/eng_council.md
[fidl-team]: /src/fidl/OWNERS
[ftp-001]: contribute/governance/rfcs/0018_ftp_process.md
[ftp-024-backwards-compatibility]: contribute/governance/rfcs/0024_mandatory_source_compatibility.md#backwards_compatibility
[ftp-049-criteria]: contribute/governance/rfcs/0049_fidl_tuning_process_evolution.md#criteria
[ftp-049-motivation]: contribute/governance/rfcs/0049_fidl_tuning_process_evolution.md#motivation
[ftp-049-reviewing]: contribute/governance/rfcs/0049_fidl_tuning_process_evolution.md#reviewing
[ftp-049]: contribute/governance/rfcs/0049_fidl_tuning_process_evolution.md
[ftp-template]: contribute/governance/deprecated-ftp-template.md
[google-doc]: https://www.google.com/docs/about/
[rfc-process-criteria]: contribute/governance/rfcs/0001_rfc_process.md#criteria
[rfc-process-draft]: contribute/governance/rfcs/0001_rfc_process.md#drafts
[rfc-process-iterate]: contribute/governance/rfcs/0001_rfc_process.md#iterate
[rfc-process-prior-art]: contribute/governance/rfcs/0001_rfc_process.md#prior_art_and_references
[rfc-process-socialize]: contribute/governance/rfcs/0001_rfc_process.md#socialize
[rfc-process]: contribute/governance/rfcs/0001_rfc_process.md
[rfc-0006]: contribute/governance/rfcs/0006_addendum_to_rfc_process_for_zircon.md
[rfc-template]: contribute/governance/rfcs/TEMPLATE.md
