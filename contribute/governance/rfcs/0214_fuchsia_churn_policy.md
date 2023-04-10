<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0214" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

Update Fuchsia's governance model to include platform churn policies by adding
requirements to report the cost of changes, proactively notify impacted teams,
and limit the amount of work placed on impacted teams.

## Motivation

Fuchsia is now a large project with several independent teams working hard to
meet the goals of various customers. At the same time, platforms like Fuchsia
must evolve in ways that require intermittent effort from many contributors
across the codebase, informally referred to as "churn".

### Goals

 1. Align Fuchsia engineering practices with the project's stated support and
    stability goals.
 2. Accurately estimate the time spent on migrations and other externally
    imposed changes.
 3. Clarify the boundary between RFC designs and FPS staffing decisions.
 4. Provide a collection of migration strategies for teams to consider.

### Non-goals

 1. Change the strategy for large scale changes that have already begun.
 2. Decide what changes should be made.
 3. Mandate a specific migration strategy for all changes.
 4. Reduce the rate of changes.
 5. Set policy for changes that require no effort from client teams.

## Stakeholders

_Facilitator:_

- abarth@google.com

_Reviewers:_

- abarth@google.com
- keir@google.com
- shayba@google.com

_Consulted:_

- neelsa@google.com
- tombergan@google.com

_Socialization:_

This RFC was initially written as a Google-internal document shared widely
within the Fuchsia team, including members of the [Fuchsia Eng Council][fec].

## Implementation

This proposal is a change to Fuchsia's governance model, laid out in the
"Impacted teams" and "Initiating teams" subheadings. If this RFC is accepted, it
will be implemented by updating the [FEC][fec] charter, adding a [governance
policy][governance-policies], and amending the [new contributor
guide][contributor-guide].

Informally, a change is considered "Fuchsia-wide" if it requires development
effort or workflow changes from other contributors. This includes changes that
impact the [ABI][fuchsia-abi], any public portion of the [SDK][fuchsia-sdk], the
contents of a [product][build-products], or generated code.

All Fuchsia-wide changes will incur an engineering cost for many Fuchsia
contributors. This policy centralizes those costs so that they can be minimized.
For the purposes of this policy, an "impacted team" is any team that must either
approve or make changes to their own code, workflows, or docs in order to
accommodate churn.

It is the responsibility of the contributor or team that initiated the change to
resolve any breakages. Most of the time, this will mean taking a "revert first,
diagnose second" approach.

### Impacted teams

If your team is impacted by an approved change:

 * Respond to incoming CL reviews or other changes within two business days.

 * Support the change author in arriving at a satisfactory conclusion, such as
   by approving their work, responding to surveys, clearly rejecting proposed
   changes, requesting specific modifications to the change, and/or answering
   questions from the change author about the subject matter.

 * If more than 10% of your time is spent responding to churn, you may flag this
   issue with eng-council@fuchsia.dev

 * If the change is in a style guide (e.g. lints, compiler warnings, etc.), you
   decide how quickly to resolve new style violations.

### Initiating teams

Without the churn policy, there are no formal requirements or expectations for
changes that create churn. This section adds responsibilities for the authors of
such changes.

If your team is initiating a change and will be doing 100% of the work:

 1. Send mail to the [FEC][fec] explaining the minimal impact to other teams.
 2. Send mail to announce@fuchsia.dev to notify contributors of the migration.
 2. Proceed with the migration.

If your team initiates a change that will require substantial effort from
others, including changes initiated by an RFC:

 1. Create a plan that demonstrates to the [FEC][fec] that your team will expend
    at least 80% of the manual effort that is not addressed by automation. Plans
    must include a list of the impacted teams and the estimated cost of the
    change.
 2. After the [FEC][fec] approves your plan, notify impacted teams. They must be
    able to schedule the work using quarterly planning over at least two
    quarters, so notify teams more than a week before the quarter begins.
 3. Proceed with the migration.

## Backwards Compatibility

This policy does not impact any active migrations.

## Documentation

To assist change authors, example migration plans will be provided on
fuchsia.dev.

## Drawbacks, alternatives, and unknowns

The main drawback of this proposal is the required involvement of the [FEC][fec]
at the beginning of the process. It is important that change authors receive
timely responses, both from the FEC and from impacted contributors.

One alternative policy would require impacted contributors to approve changes.
This has the benefit of always allowing change authors to make progress. This
alternative was dismissed because it removes an opportunity for impacted
contributors to give feedback about the value and harms of a proposed change.

Another alternative is a more rigid approach, where all migrations must be [soft
transitions][soft-transitions] using available versioning mechanisms and a
slower pace to spread the effort across all Fuchsia contributors. This differs
from the proposed policy by setting no engagement expectations for impacted
teams. This alternative was dismissed because it sets no bounds on the review
timeline, making it difficult for initiating teams to plan and make timely
progress.

## Prior art and references

Is there any background material that might be helpful when reading this
proposal? For instance, do other operating systems address the same problem this
proposal addresses?

[build-products]: /development/build/build_system/boards_and_products.md#products
[contributor-guide]: /CONTRIBUTING.md
[fec]: /contribute/governance/eng_council.md
[fuchsia-abi]: /concepts/packages/system.md
[fuchsia-sdk]: /development/sdk/index.md
[governance-policies]: /contribute/governance/governance.md
[soft-transitions]: /development/source_code/working_across_petals.md#soft-transitions
