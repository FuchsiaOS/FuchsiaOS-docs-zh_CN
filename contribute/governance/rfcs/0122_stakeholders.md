<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0122" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC amends the [RFC
process](contribute/governance/rfcs/rfc_process.md) and [RFC
template](contribute/governance/rfcs/TEMPLATE.md) to add clarity around
identifying and managing stakeholders for an RFC. It adds a "Stakeholders"
section to the RFC template and clarifies possible stakeholder roles.

## Motivation

Today, the Fuchsia Eng Council (FEC) is responsible for ensuring that each RFC
has been reviewed by appropriate stakeholders. This centralization of
responsibility has some downsides. While this relies on FEC council members'
knowledge of the Fuchsia organization (an [expected
qualification](contribute/governance/eng_council.md#qualifications)), it
lacks an externally visible artifact allowing this knowledge to be organically
recorded such that others can learn this aspect of the Fuchsia project.

Separately, there is some confusion around stakeholder identification,
specifically when it comes to identifying reviewers – those whose opinion and
vote will carry blocking weight in the decision process – from others whose
opinion is advisory. For instance, while anyone building a UI on top of Fuchsia
might be affected by a graphics change, the Graphics lead's opinion will carry
more weight.

By making stakeholder identification part of the RFC process, we can have an
explicit discussion about who must approve an RFC, and why. This explicit
stakeholder identification can help with both issues described above.

This change to the RFC process is intended to make the process easier to
navigate by helping RFC authors understand which feedback is advisory and which
feedback is blocking, and to provide a way to ensure that we have the right
number of reviewers for each RFC, with clear guidelines on choosing them. This
change also makes discussions about who the stakeholders are and their
respective roles open to all, thus further leveling the playing field.

## Stakeholders

_Facilitator:_ abarth

_Reviewers:_ abarth (FEC member), wittrock, pascallious (coauthors). Several
recent RFC authors: bprosnitz, simonshields, aaronwood, dgilhooley.

_Consulted:_ Members of FEC.

_Socialization:_ A draft of this RFC was sent to the FEC discuss mailing list
for comment.

## Design

### Changes to the RFC process (by section)

#### Roles and responsibilities

We further subdivide the [roles and
responsibilities](rfc_process.md#roles-and-responsibilities) of stakeholders
into:

 * *Facilitator.* The person appointed by FEC to shepherd this RFC through the
   RFC process. Today, this person must be an FEC member.

 * *Reviewer(s).* The stakeholders whose +1 or -1 will be considered when the
   FEC decides to accept or reject the RFC. (While a +2 is the "approve" on code
   CLs, we tend to look to reviewers to +1 or -1 to indicate their support or
   lack thereof, and look to the facilitator to +2 upon approval.)

 * *Consulted.* The stakeholders whose feedback on the RFC was sought, but whose
   +1 or -1 is not considered when the FEC decides to accept or reject the RFC.

#### Socialize

Add the following text:

During this phase, the RFC author should start to identify the stakeholders for
this RFC.

#### Draft

Add the following text:

> The RFC author should propose an initial set of stakeholders in consultation
> with the experts in their RFC [area](contribute/governance/areas). The
> set of stakeholders may initially be left empty or incomplete. If there is any
> ambiguity, they should consult FEC for assistance identifying stakeholders.

#### Iterate

Edit the second paragraph:

> Mechanically, you should invite stakeholders to provide feedback on your RFC
> by adding them to the "Reviewers" (for stakeholders whose +1 is required) or
> "CC" fields (for "consulted" stakeholders) in the CL, as you would for a
> normal code review. In addition, you may email your CL to
> eng-council-discuss@fuchsia.dev soliciting additional feedback. The
> stakeholders should provide you feedback by leaving comments on your RFC in
> the code review tool.

Add the following text:

> Anyone can propose an additional stakeholder for a given RFC, including
> themselves, by commenting on the RFC CL, although these proposals may not
> always be accepted. If there is broad agreement, the RFC author should add the
> stakeholder. FEC may also request that the author add stakeholders.

> A stakeholder may 'opt out' and ask to be removed, or may delegate their
> review (for example, to another expert in the relevant area). FEC may request
> that a stakeholder be removed or moved between "reviewer" to "consulted".

> Feedback may include comments from people who are not stakeholders. The author
> should respond to these comments if relevant, but settling them is not
> necessarily required to move to the last call stage. If the comments point to
> a disagreement about who is a stakeholder, FEC can help resolve this.

Note: the exit criteria for the iterate phase already include "All stakeholders
identified and approved by Eng Council".

#### Last Call

Edit the second paragraph:

> Typically, reviewers sign off with a +1 and the facilitator will sign off with
> a +2. Consulted stakeholders may also sign off with a +1 if they wish to
> express their enthusiasm for the RFC, although this is not required.

### Changes to metadata
Additions to [Creating an RFC](contribute/governance/rfcs/create_rfc.md):

> Consulted - Required once approved or rejected. Stakeholders who were consulted
> about this RFC, but whose +1 is not required.

### Changes to the RFC template

Add the following optional section to the [RFC-0000: RFC
template](contribute/governance/rfcs/TEMPLATE.md) (after "Motivation",
before "Design"):


> **Stakeholders**

> Who has a stake in whether this RFC is accepted? (This section is optional but
> encouraged.)

> _Facilitator:_

> The person appointed by FEC to shepherd this RFC through the RFC
> process.

> _Reviewers:_

> List people whose vote (+1 or -1) will be taken into consideration by FEC when
> deciding whether this RFC is accepted or rejected. Where applicable, also list
> the area they are expected to focus on, such as "FIDL" or "security".  In some
> cases this section may be initially left blank and stakeholder discovery
> completed after an initial round of socialization. In general, "reviewers"
> should be listed on the reviewers line in gerrit and people who are
> "consulted" should be CCed. Care should be taken to keep the number of
> reviewers manageable, although the exact number will depend on the scope of
> the RFC in question.


> _Consulted:_

> List people who should review the RFC, but whose approval is not required.


> _Socialization:_

> This section may be used to describe how the design was socialized before
> advancing to the "Iterate" stage of the RFC process. For example: "This RFC
> went through a design review with the Component Framework team."

## Drawbacks, alternatives, and unknowns

This RFC introduces a small amount of additional overhead for RFC authors.

## Prior art and references

[RFC-0001: RFC process](0001_rfc_process.md)

[RFC-0006: Addendum of the RFC process for
Zircon](0006_addendum_to_rfc_process_for_zircon.md)

[RFC-0067: Additions to Fuchsia RFC process](0067_rfc_process_additions.md)
