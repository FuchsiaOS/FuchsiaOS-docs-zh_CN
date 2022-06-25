# Cultural tenets of the Fuchsia FIDL team

Here are four cultural tenets which have helped keep the members of the [Fuchsia
FIDL team](/src/fidl/OWNERS) focused. Having these values helps to choose the
appropriate course of action when multiple seemingly appropriate paths are
available. These cultural tenets are meant to guide how we approach our work,
our projects, how we present our team, and helps to define how we work together.

## Keep design inventory in check

> We care about keeping the "designed but not yet implemented" inventory low. We
> code our ideas, and refrain from ideating too far out in the future.

We want to keep the number of accepted-not-yet-implemented RFCs low, or avoid
engaging on plans which we know will not see the light of day in a reasonable
amount of time, i.e. our time horizon. Over time, what a "reasonable amount of
time" is has grown from months, to half a year, to now about 1 to 2 years.This
horizon expansion was fueled by various things[^1]. We have also observed that
unimplemented designs do not age well, and that requirements and architecture
that made sense when they were first etched are rarely the right fit for the
reality we find ourselves in down the road.

We strive to avoid the "current is deprecated, future is not working" trap by
maintaining high velocity towards building the future, actively migrating our
customer base to these new features, and generally keeping the number of
in-flight migrations low.

We also strive to provide concrete answers as to how to solve the problem with
current tooling, rather than provide a non-answer by deferring to future
features "it will be fixed with \[insert outdated design doc here\]". Providing
advice with current tooling when we know something better is coming around the
corner is painful, and is a good reminder of the urgency we should feel to
deliver this value to our users in a codebase near them (as opposed to in a
design doc, which is only _potential_ impact).

## One voice

> When representing the Fuchsia FIDL team's position, we speak in harmony. We
> strive to present a clear and consistent message to our peers. We should all
> answer questions consistently, speak about our roadmap consistently, describe
> technical direction consistently, align ourselves on prioritization, etc.

For instance, when providing help on fidl-dev@fuchsia.dev, any team member
should provide the same advice. If we have differing opinions in the team, it is
our responsibility to resolve them, so that we can present a common view to our
users.

Another example, when authoring an [RFC] on behalf of the FIDL team, we are
implicitly stating that all of the team is supportive of the general direction
of the design (though often details are discussed on the CL). This implies that
before an RFC moves to the [iterate] step some alignment has been reached
within the team, possibly through pre-work occurring in [another medium][medium]
than a Gerrit CL such as a Google Doc.

Failure to align internally spills our indecisiveness or doubts onto our users,
who have less context of the technical trade offs that we do. Sometimes, that
means that we have to acknowledge that "we don't know", or that we "don't have a
best practice yet". In such cases, we should describe the options, and their
pros and cons, and work with our users to guide them as to what is the best
course of action for them.

## Bias for action

> We lead with action, we place a premium on work, and deliverables. We comment,
> steer, and critique where we care to walk the talk.

In everything we do, it is useful to think about what is the positive outcome,
what is the stepping stone that we can lay to move forward. For instance, if we
have an interesting design conversation during a team sync, it is quick to
summarize what we discussed in a few bullet points. The next time around, this
context will be there at the ready to re-open the conversation from a stronger
basis. Or if we discuss during a 1:1 the presence of technical debt in some code
base, and complain about the state of things, we can choose to turn this into
something concrete, even if it is just raising awareness to all, and naming the
problem and desired end state.

<!-- Due to a parsing bug in the fuchsia.dev infrastructure, we have to set the
     heading for the TOC to show properly. -->
## We start small[^2] {:data-text='We start small'}

> We execute on a big vision in steps, break down the work, and start with small
> proof-of-concepts. We are comfortable with taking temporary shortcuts, and
> focus on keeping momentum towards our long term goals.

This is about prioritizing, breaking the work down,
and doing incremental improvements towards an ideal. For instance, we have taken
liberties and sometimes done work out-of-order to first unblock the user facing
issue, knowing we would later close the architectural gap.

For instance, the [introduction of Dart FIDL JSON
template](https://fuchsia-review.googlesource.com/c/topaz/+/205416) was
problematic because it took advantage of careful assembly (ensuring no handles
we present, by visual review sort of) and was received negatively initially. The
architectural gap this Dart FIDL JSON feature took advantage of was only closed
by [RFC-0057: Default No
Handles](/docs/contribute/governance/rfcs/0057_default_no_handles.md), more than
18 months later.

Another example is work on code size analysis (see [PS1 of
76](https://fuchsia-review.googlesource.com/c/fuchsia/+/378353/1)). We decided
to start with an imperfect solution which got us some data. Then, little by
little, we iterated and refined this solution. What was key is that improvements
to the measurements were made in tandem with concrete changes to reduce binary
size in FIDL owned code. As a result, we invested enough to generate an "ideas
list". We executed on that list, and rinsed, and repeated the process. After a
quarter following this cycle, the resulting tool became quite precise, and its
reporting capabilities were perfectly geared towards a concrete use case. This
tool then generalized, and is now a key piece of Fuchsia's size dieting efforts.

A third example, work on describing the Zircon API using the FIDL language. This
initially started with [very hacky FIDL
files](https://fuchsia-review.googlesource.com/c/fuchsia/+/298531/1), some that
didn't even compile. A new backend (kazoo) [was
born](https://fuchsia-review.googlesource.com/c/fuchsia/+/298790), with [support
for various
targets](https://fuchsia-review.googlesource.com/q/kazoo+owner:scottmg%2540google.com+before:2019-10-01)
added one after the other. Still today, the FIDL files rely on hacks,
experimental features, and the like. But, we are confident in the long term
direction of this work, and have been careful not to take design debt that would
paint FIDL in a corner.

There is a continuum between pure research on one end, and addressing the
immediate issue right in front without regard to impact on future plans. We
should be conscious about where we are on that continuum, and actively work to
right size how future proof or expedient we are. Navigating the balance of what
shortcuts are fine relative to shortcuts which can be devastating is part of the
craft of being an engineer. Practice makes perfect.

[^1]: Contributing factors to expanding the time horizon:

    * Fuchsia FIDL team size, i.e. more fire power;
    * Solving short term issues (e.g.
      [FTP](docs/contribute/governance/deprecated-ftp-process.md) backlog in
      2018, `fidlc` bugs blocking other teams);
    * Clear expectations of how work happens (e.g. guidelines for team syncs, or
      project updates), further speeding up execution;
    * Design precedent (e.g. RFCs) anchoring design principles, and simplifying
      the research, and review of new ideas.

[^2]: This tenet's punchy title "We start small" is borrowed from Jack Dorsey's
 "Square's Four Corners" which were the company values circa 2012. The tenet
 itself is expanded here to be relevant to our work.

<!-- link labels -->
[iterate]: /docs/contribute/governance/rfcs/rfc_process.md#iterate
[medium]: /docs/contribute/governance/rfcs/0017_folding_ftp_into_rfc.md#medium
[RFC]: /docs/contribute/governance/rfcs/rfc_process.md
