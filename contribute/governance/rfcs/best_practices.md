{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}

# RFC best practices

This document provides guidelines for how authors and reviewers can get the most
out of the RFC process.

## Think of RFCs as design docs

The RFC process is how the Fuchsia project makes decisions. Some RFCs contain
decisions to adopt some policy across the project. For example, [RFC-0004]
specifies that storage sizes should be given in IEC notation (e.g., KiB, not
KB). When that RFC was accepted, it meant that we as a project decided to follow
that policy. These policy-setting RFCs are relatively rare, though.

The majority of RFCs are essentially design docs. When a design doc RFC is
accepted, it means that we as a project decided to solve a problem in a certain
way. An accepted RFC is a historical document, recording the fact that everyone
who had a stake in that design was comfortable moving forward with it.

[RFC-0004]: 0004_units_of_bytes.md

## Focus on getting buy-in

When writing an RFC (or anything, for that matter), ask yourself: "why am I
writing this?"

Presumably you're writing an RFC because you want to change something, but you
think your change could have an impact on other members of the project. Before
moving forward, you want to make sure they know what you're planning to do, and
that they don't mind. These folks who might be impacted by your change are your
[stakeholders][roles]. Once they +1 your RFC and it gets accepted, you'll have a
written record that they're happy with your change. Proceed with confidence!

Let this goal guide the contents of your RFC:

* Emphasize the aspects of your design that need stakeholder buy-in. For
  example, the interfaces your stakeholders use are more important than the
  implementation.
* Spell out the design's implications. If your stakeholders need to work hard to
  understand how the design affects them, the review will be slow, and there's a
  greater chance of one of them coming to you during implementation saying,
  "this isn't what I agreed to!"
* Be clear about what you're proposing. For example, your RFC might include
  commitments about how a design _will_ work, as well as illustrative examples
  of how it _could_ work. It might provide context by describing the system _as
  it is_, as well as requirements for how the system _will be_ when
  implementation is done. Make it easy for your stakeholders to tell which is
  which. You don't want your stakeholders to see a suggestion where you meant to
  give a requirement.

[roles]: rfc_process.md#roles-and-responsibilities

## Include the _right_ details

Your RFC does not need to specify every detail of your design. In fact, too much
detail can make it harder for your stakeholders to understand the design's
essential trade-offs. On the other hand, if you don't include _enough_ detail,
different readers might come to different conclusions about what your RFC means.
Finding the right level of detail is an art.

Your RFC should include all the details that have a _material impact_ on your
design's acceptability, and no more. This will be just enough detail for the
stakeholders to endorse your design and for future readers to understand why it
was approved. Incidental details just make the RFC harder to review.

Some RFCs get more and more detailed during review. This can be frustrating for
authors and stakeholders alike. When a stakeholder asks a question about (or
objects to) some part of your design, your instinct may be to add more details
as clarification or justification. Before you do, consider whether _removing_
detail would be more appropriate. If the bit they commented on doesn't make a
material difference to the RFC, you can short-cut a lot of discussion by
deleting it or making it more general and abstract.

To help decide whether some detail has a material impact on your design,
consider these questions:

* If you changed this detail, would anyone ever change their vote from +1 to a
  -1 because of it?
* If this detail was implemented differently from what was written, would that
  be okay? Would your stakeholders want to be informed about the change?
* Is it important that you decide on this detail _now_? Would it be better to
  let the implementers and their code reviewers make the call?

## Keep future readers in mind

The stakeholders are your RFC's primary audience, but you shouldn't forget about
future readers trying understand your design after it has been accepted. Here
are some tips for making your RFC accessible to a broader audience:

* Pretend that your stakeholders are first hearing about your design via your
  RFC. Include (or link to documentation with) enough background to put your RFC
  in context.
* Pretend that the stakeholders are _only_ hearing about your design via the
  RFC. You and your stakeholders will probably discuss the RFC via email, chat,
  meetings, or at the very least code review comments. Future readers won't have
  easy access to these conversations, so make sure the text of the RFC includes
  any important conclusions you came to out-of-band.
* Use stable links. Your RFC will become much harder to understand if the
  "background information" links dramatically change or become 404. Whenever
  possible, link to a _specific revision_ of a document so future readers see
  the same page you see.

## Don't use RFCs as documentation {#rfcs-vs-docs}

Once accepted, your RFC documents the fact that your stakeholders were happy
with the feature you designed. However, it _shouldn't_ be used as documentation
for the feature itself.

RFCs and documentation simply have different goals:

* Documentation needs to be constantly updated as the system changes, but RFCs
  shouldn't change once they're accepted (beyond [minor amendments]).
* RFCs justify to the stakeholders why the design is sound and preferable to the
  alternatives, which probably isn't very helpful for users.
* Documentation gives precise, procedural examples for how to use the feature,
  which requires a level of detail that isn't very helpful for stakeholders.

**Don't link to an RFC in code or documentation to explain how a feature
works.** Give the feature its own documentation and link to that instead.

Some suggestions for where to put this kind of documentation:

* Use detailed doc comments to add documentation about a specific API or code
  module.
* Create a `//src/my_subsystem/docs/` subdirectory in `fuchsia.git`.
  Documentation hosted in `fuchsia.git` can be rendered by [Gitiles][fx-gitiles]
  and is great for providing conceptual guides to your subsystem.
* [Add your documentation to fuchsia.dev](contribute/docs/README.md) if it
  will be useful to the wider Fuchsia community.

On the other hand, RFCs _may_ link to documentation. In fact, if your RFC
contains lots of background information, consider moving that background into
documentation and linking to it instead!

[minor amendments]: rfc_process.md#process_to_amend_rfcs
[fx-gitiles]: https://fuchsia.googlesource.com/fuchsia

## Update these best practices

If you have suggestions for best practices (or you think some of the ones above
could use updating) reach out to
[eng-council@fuchsia.dev](mailto:eng-council@fuchsia.dev) or send a change to
the FEC!
