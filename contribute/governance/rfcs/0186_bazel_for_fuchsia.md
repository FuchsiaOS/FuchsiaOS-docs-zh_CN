<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0186" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

Our plan is to converge on and adopt Bazel as the primary build system of choice
for Fuchsia components by bringing Fuchsia SDK and Bazel into fuchsia.git.

The Fuchsia team is committed to completing this migration. Specifically,
the Fuchsia Build Team, the Fuchsia Bazel SDK Integration Team, Fuchsia
Platform Infra, Fuchsia Software Assembly, Fuchsia TPM team, and Fuchsia DevRel
have expressed commitment in delivering and supporting this plan.

### Definitions

* "In-tree" is everything stored in fuchsia.git.
* "Out-of-tree" (OOT) means code that targets Fuchsia and builds on and with the
  Fuchsia SDK, not in the presence of platform source code, and which is stored
  outside of fuchsia.git. For instance an
  example of the Intel wlan driver builds in a [separate repository][wlan-repo],
  using the Fuchsia Bazel SDK.
* Bazel - an open source build and test tool for software development.

## Motivation

As of Q1 2022, Fuchsia's platform (i.e. the code in fuchsia.git) build uses
GN/Ninja. In 2021, SDK-based development accelerated to pay tech debt and enable
future strategy. [Fuchsia chose Bazel][bazel-sdk-rfc] as the well-lit path
for SDK-based development, and is adopting it for existing and new source bases.

We observe that many of the use cases for building and working with Fuchsia are
the same for all component and product developers, regardless of what tree/repo
they are working in. For example: compiling source code into a component and
packaging that component and its assets into a Fuchsia package, running that
software on a target, assembling a product, generating symbols, streaming logs,
etc. We are motivated to reduce complexity and costs by converging on a unified
set of Bazel and SDK workflows and on implementations.

To generate empathy for our out-of-tree developers, and to create and deliver
an excellent Bazel + SDK experience, it's imperative that the Fuchsia team
itself uses the SDK and its Bazel integration day to day and as part of a well
supported workflow.


## Stakeholders

_Facilitator:_ rlb

_Reviewers:_

* digit - Fuchsia's build team
* chaselatta - Fuchsia's Bazel SDK team
* mangini - Fuchsia's Bazel SDK team and Fuchsia DevRel
* nsylvain - Fuchsia's EngProd team
* abarth - Fuchsia Architecture

_Consulted:_

* aaronwood - Product Assembly
* awolter - Product Assembly
* dannyrosen - Eng Excellence
* keir - Pigweed
* tmandry - Rust for Fuchsia
* brunodalbo - Connectivity
* surajmalhotra - Driver Framework
* amathes - Fuchsia PM Lead
* cphoenix - Diagnostics
* akbiggs - Flutter on Fuchsia

_Socialization:_

This RFC's initial socialization proposal went through reviews by the Fuchsia
Build team, the Fuchsia SDK team, Fuchsia's Toolchain team, Fuchsia EngProd
team, Fuchsia's Rust team, and various representatives and leads across Fuchsia.

## Design

This RFC is intended to [make project policy][when-to-use-rfcs]. The following
is a list of design principles for this policy.

*Incremental*: We will migrate one component at a time, learning as we go.

*Inclusive*: Even though the Fuchsia team adopted, and will continue to expand
its usage of, Bazel and create well-lit paths with Bazel for our users, it is
not a strict requirement to use Bazel to build software for Fuchsia. The
[IDK][idk] must continue to be agnostic of build systems.

We also operate with the principle that the dimension of _build system_
can be orthogonal from the dimension of _source repository_. A migration
of build targets to Bazel and the SDK does not necessitate a migration of
code to another repository.

Eventually, we would like to organize Fuchsia development in terms of a number
of *projects*, where each project takes as input the Fuchsia SDK and produces
as output some number of packages or other binary artifacts that can be
assembled into a Fuchsia system image. These projects can be owned by Fuchsia,
our partners, or third-party developers. These projects can also vary in size,
from a small project that delivers a single binary to a large project, perhaps
that delivers a significant fraction of the Fuchsia platform (e.g.,
a fuchsia.googlesource.com/platform.git).  As we get closer to this situation,
we can decide how to organize our code into projects to maximize efficiency.

This RFC describes an important step towards that eventual future, which is to
refactor the bulk of Fuchsia development to be hosted on top of the Fuchsia SDK.
Once we have rehosted onto the Fuchsia SDK, we will have more flexibility for
how we want to organize that development into projects.


## Implementation

### Existing adoption plans

We have begun by evaluating and adopting Bazel and the SDK to drive the
following:

* Build and test a simple, but load-bearing driver that is incorporated into
  Workstation
* Build and test a simple, but user-accessible, component that is incorporated
  into Workstation
* Build and test a select and growing number of drivers
* Build and test samples and other projects to help developers get started
  developing for Fuchsia
* Build and test Flutter on Fuchsia embedder
* Build and test Workstation Experiences code
* Drive product assembly for Google products

### Additional adoption proposal

Before proceeding below, we will collect evidence that the Bazel SDK
demonstrates minimum viability and functionality sufficient to support
software and systems that end up being used by some number of users.
The simple driver and simple component (mentioned above) will be built
with the SDK and published to global integration, and then included in the
current Workstation product assembly process. Then, the code in the Bazel
SDK, plus its configuration in the repositories that host the simple
driver and simple component, and the mechanism by which we publish
those prebuilt components to be consumed by the Workstation build process
will be reviewed by Fuchsia Engineering Council and Fuchsia Security
teams. We will process only after those reviews pass and users are
receiving products with the simple driver and simple component will
we proceed.

In parallel to the ongoing "Existing adoption plans" mentioned above,
we will create a Bazel build tree inside fuchsia.git, alongside the
GN build tree. The Bazel build will be able to build and test packaged
components and assemble products. We will share logic (Bazel bootstrap, Starlark
rules) with the Fuchsia Bazel SDK for out-of-tree developers.

We will begin by implementing _product assembly_ use cases in fuchsia.git with
Bazel and the SDK. We'll be working backwards from the end of the `fx build`
process, which means we can deliver a linear GN/Ninja -> Blaze flow.

<!-- For Googlers, to access the source illustration: go/fuchsia-intree-bazel-rfc-diagrams -->

![Alt text:
in-tree flow into bazel assembly from ninja platform build - assembly in bazel
](resources/0186_bazel_for_fuchsia/in-tree_flow_into_bazel_assembly_from_ninja_platform_build-assembly_in_bazel.png){:#fig-1}

Then, we will identify a few candidate components to incrementally extract
from the GN/Ninja platform build to the Bazel platform build, while
retaining the ability to assemble a cohesive Fuchsia image from the same
artifacts, and preserving or improving other supported workflows.

<!-- For Googlers, to access the source illustration: go/fuchsia-intree-bazel-rfc-diagrams -->

![Alt text:
in-tree flow into bazel assembly from ninja platform build - moving components
to bazel
](resources/0186_bazel_for_fuchsia/in-tree_flow_into_bazel_assembly_from_ninja_platform_build-moving_components_to_bazel.png){:#fig-2}

As we implement support for those first candidate components,
we will watch KPIs (mentioned below) and we will
engage with early adopters to ensure their engineering productivity does not
regress. We will report status and measurements to the Fuchsia team
along the way.

We envision that fuchsia.git contributors will use `fx` as the frontend for
driving their build, and `fx` will manage both the Bazel and GN/ninja
invocations behind the scenes. For the transition, retaining the `fx set`, `fx
build`, etc frontend will help us encapsulate the details of the migration and
retain workflows.

The current plan is to encapsulate any aspects of invoking both GN/Ninja
and Bazel inside [`fint`][fint], and we note that any `fx` build workflows which
are implemented in terms of `fint` and will be preserved as well due to
said encapsulation.

The Fuchsia Build team will be responsible for preserving the capability
for compiler/model training, and working with the GN build to help
preserve the corpus size.

Migrating components in fuchsia.git to Bazel is not to be conflated with moving
components between repositories. Components in fuchsia.git that are migrated to
Bazel can stay in fuchsia.git. Repo moves are not in scope for this RFC.
However, before completing the migration of all target components
to Bazel and the SDK, we plan to revisit the build architecture
and work with FEC to determine if it makes sense to move any components built by
Bazel and the SDK out of the fuchsia.git tree.


### KPIs

We propose the following KPIs for this effort:

* developer satisfaction, as measured by surveys
* null build time
* full build time for the configuration that is the long pole presubmit builder
* presubmit testing time for that configuration

For "null build time", which is one way to understand the latency
of spinning up the build system itself, we will look at this in context
with overall developer satisfaction and productity. We will aim to
not allow a 50% regression in null build time, unless we determine that
null build time is a significant factor limiting developer satisfaction
and productivity.

For "full build time for the configuration" and "presubmit testing time",
we propose to not allow more than a 10% transitory regression to a KPI,
with the expectation that we eventually arrive at improvements to all KPIs.

### Staffing

The details of staffing and funding this effort are TBD and outside
the scope of this RFC. However, we note that a staffing and funding
plan will be critical to a successful evolution of software
to the SDK and Bazel. Fuchsia has two teams (Bazel SDK Integration team,
Fuchsia Build team) that have initially committed to investing resources
to help make this smooth for Fuchsia engineers, in partnership with
component teams. We expect the Bazel SDK Integration team and the Fuchsia
Build team to drive the first migrations to Bazel and SDK, and then from
that experience we will deliver an informed staffing/funding estimation
for further migrations.

### Automated tooling

After we make significant progress in migrating areas of functionality to
Bazel, we will explore how to add automated checks to ensure that future
growth of those areas are not accidentally configured to be built with GN.
This phase would occur towards or at the end of the process, which would
be after we've proven that KPI goals are met and not regressed.

### Possible future considerations

We may migrate 100% of Fuchsia's source code to Bazel in the future
if there is a clear line of sight. In that potential future,
we envision arriving at a pure
Bazel build and retiring `fx` and `fint` commands which manage the build,
and instead managing things entirely in Bazel nomenclature.


## Documentation

As part of [RFC-139][bazel-sdk-rfc], we are updating the documentation on
fuchsia.dev to explain and teach how to use the SDK with Bazel.

We will update the documentation on fuchsia.dev for Contributors to Fuchsia
for when and how to use Bazel in-tree. This documentation will include
guidance for _when_ to configure a component to be built with Bazel or GN,
as well as contributor-facing guides for _how_ to run builds. This
documentation will also be public.

## Drawbacks, alternatives, and unknowns

We note that [RFC-0153][rfc-153], which proposes to use a temporary customized
version of the open source Ninja tool used by the Fuchsia platform build,
is still ongoing and is not superceded by this RFC. Fuchsia will continue to
rely on Ninja for some time, so quality
of life improvements to Ninja are valuable and welcome.

Unknown: If it is technically possible to move all of Fuchsia's code
over to Bazel, while retaining a good user experience and remaining true to
idiomatic Bazel usage. We will explore this potential with our build
partners and determine if there could be a line of sight. We note that this
unknown does not block our intention to move Fuchsia's components and packages
to Bazel, and is not intended to block this RFC from being approved.

Feature Difference: Target configuration transitions within the
same build invocation are poorly supported by Bazel. For instance if a
user wants to build all executables as _release_, then one executable as
_asan_, then that's two `bazel build` invocations. We do not think this is
a risk or showstopper, but it is a departure from Fuchsia's existing
GN-based variant system.

Possible Risk: We know of limitations with Bazel's built-in C/C++
rules where they may not be sufficiently flexible to build low-level
Fuchsia libraries and binaries.
According to Bazel team's updates to Fuchsia and to their public roadmap,
porting Bazel C/C++ to community-owned and maintained  Starlark rules is
a top priority. The concept has been demonstrated sufficiently such that
most of the risk is removed.

Unknown: Effect of Bazel's filesystem sandboxing and net-result trade-offs
of correctness/hermeticity. Bazel uses filesystem
sandboxing and symlinking to deliver incremental correctness
and hermeticity guarantees. This is known to add up to 10% to the clean build
workload on a typical build worker, and will contribute to build durations.
We expect to more than make up the difference with the benefits that hermeticity
delivers, when combined with remote build execution to deliver better cache
utilization, early cutoff, and shallow builds. However if we find that the cost
of hermeticity is greater than anticipated and is not offset by the expected
performance benefits, then we will consider disabling sandboxing
(i.e.  `--spawn_strategy=local` ).

Unknown: What level of support from upstream owners of Bazel the Fuchsia team
will receive. We have no reason to believe we will not receive support, and one
objective of this RFC is to start a stronger partnership with the owners of
Bazel. Our initial meetings and engagements with the Bazel team have been
very productive, and they are supportive and willing to learn more about
our requirements and observations. Our questions to the Bazel team get
quick answers. We believe that as Fuchsia continues to adopt Bazel for
the SDK (RFC-139) and as Fuchsia demonstrates it is an excellent customer
to Bazel, we'll see continued support and engagement from the Bazel team.

Unknown: How we will compile Rust code with Bazel and the Fuchsia SDK.
We see there is active development of Rust support for Bazel, however we
will need to test this and explore if it meets Fuchsia's needs. We expect
there will be a follow-up RFC that describes our approach to support
building Rust code with Bazel and the Fuchsia SDK. This will be in
close partnership with the Rust on Fuchsia team.

Unknown, To be determined: the exact interface between the
artifacts built by GN/Ninja and packaged components built with Bazel in
fuchsia.git such that they are assembled into a product image. We expect
that we'll expand the scope of product assembly to solve this problem,
details TBD.

Unknown: The Fuchsia team is evaluating how to support Windows
as a developer host environment, and this will introduce new requirements
to how Fuchsia supports SDK-based development inclusive of using
Bazel. We do not yet have all these requirements. However, we note
that Bazel supports Windows as a developer host, and we have no current
reason to believe that Bazel will not work for us on Windows. We are in
close contact with the Bazel team, if such a blocker would arise.

Unknown: How testing in infrastructure may or may not be driven
with `bazel test`. This is not a blocker for this RFC, as we are
primarily focused on compiling with Bazel.

Alternative: continue using the GN/Ninja build of today for components in
fuchsia.git. We continue to fund parallel work and lose the ability to benefit
from advantages to engineering productivity that Bazel brings, as well continue
to have an empathy gap with our developer ecosystem. Additionally, the platform
build maintainers indicate that continuing to rely on GN/Ninja presents a
liability since these systems can't guarantee hermetic clean builds or correct
incremental builds.

Alternative: reconsider the decision recorded in RFC-0139 to adopt Bazel,
instead adopting a different build system, then aligning in-tree and out-of-tree
around that same build system. Since there is no basis to reconsider RFC-0139 we
reject this alternative.

Alternative: instead of adopting Bazel and the SDK inside fuchsia.git,
we could prioritize moving component code from fuchsia.git to different repo(s)
powered by Bazel and the SDK. Integration between these repos is
only ever done by prebuilts. The net result is the same: the Fuchsia team
has adopted Bazel and the SDK as the primary way to develop for Fuchsia and
is developing for Fuchsia in the same way as Fuchsia developers outside of
Google. This option was considered, however some teams
have expressed a desire to be able to change code that is accessible through
the SDK (e.g. an interface) and their component code as part of the same
commit. We do note that some teams already plan to adopt Bazel and the SDK
by moving their code to repos outside of fuchsia.git.

Alternative: (considered but rejected) instead of having two build systems
for code inside fuchsia.git, we move components out of tree in order for
those components to be built with the SDK and Bazel.
This would achieve the same long-term goal (more code is built with the SDK
and Bazel), however we believe this scenario would achieve the goal in a much
longer time frame. It would also introduce
multiple variables (migrate the code's build to Bazel + SDK, migrate the code
to a different repo), and we wish to only change one variable at a time. We
prefer to change the component's build to Bazel and the SDK first, and then
optionally move the component's code to another repo.

## Prior art and references

### Other example projects using or migrating to Bazel

Bazel has a growing community of users. The Android Open Source Project is
migrating to Bazel, having previously used a combination of Ninja and Make.
Bazel is the build system of choice for several successful open source projects
from Google such as Abseil and Tensorflow.

### Other build system migrations

#### Fuchsia's ZN->GN

It's instructive to look at another build system migration which Fuchsia
performed: ZN->GN migration aka "build unification".

Prior to build unification, Fuchsia had two builds based on GN/Ninja,
which were called in sequence. The ZN/Ninja build would go first and build
some artifacts, then the GN/Ninja bould would go second and build
additional artifacts. You could use ZN outputs in GN but not the other way.

The boundary between ZN and GN was drawn around artifacts like the ZBI contents.
This was not a good interface because it prevented the use of artifacts that
were built in GN (such as FIDL, components, packages, Rust support) to produce
artifacts that were built in ZN (such as drivers, early boot programs).

A plan was proposed to migrate GN artifacts to ZN, but it proved infeasible.
Instead, work was done to move ZN artifacts into GN. This process of moving
artifacts was validated step-by-step by producing a "summary manifest" of the
build outcomes and ensuring that no single migration step unexpectedly changes
the manifest. This work concluded when the ZN build became empty (did not
contribute to said manifest) and was then removed.

Lessons learned:

1. Maintain continuity of important workflows during a migration.
1. Use explicit and intentional contracts in phasing a migration. The Fuchsia ZN
   contract with Fuchsia GN was not an intentional contract (Fuchsia doesn't
   produce just a ZBI for external consumption or extension). This RFC proposes
   using packages and product assembly as the contract, which demonstrates the
   lesson learned (see
   [RFC-0072][rfc-72], [RFC-0095][rfc-95], upcoming Fuchsia platform roadmap).
1. Migrations should not begin before there is a plan for how they'll be
   completed.

#### Chrome's GYP->GN

Another build system migration which members of the Fuchsia team were involved
in was Chrome's migration from GYP to GN.

Chrome was motivated to migrate from GYP to GN because the GYP build was hard
to reason about, difficult to explain, and the equivalent of "gn gen" took
about one minute. This was a significant productivity drain on the Chrome
team, and another build system was desired.

The migration was eventually successful. We estimate the effort took >8 person
years over ~3 wall-clock years.

The Chrome team tried an incremental approach, but decided to pause the effort
after nine months. They found the impedance mismatches in the build configs to
be a major source of friction to the incremental approach. Later, the team
started a "bottom-up" GN build in parallel with the GYP build. First with an
FYI bot, and then with a real bot. This migration expanded across target
platforms (Linux, Windows, Mac, Android, iOS). One key reason the migration
was successful was because it had a strong champion who was committed to
seeing it through.

What we learned from this migration:

* It's hard to describe in easy-to-follow directions what to do since the
  cases are too general. People doing the conversion have to understand the
  old and new build systems almost completely. Almost nobody had this knowledge.
* A migration like this requires a lot of hustle and grind, and can be
  difficult to maintain contributions to the effort for long periods of time.
* We can make it easier for folks to help with the migration by clearly
  communicating the order in which things need to happen and provide easy to
  access tracking and organization.
* Great care and empathy for users are essential to migration success. For
  this migration, there were a long tail of user features and user workflows
  which needed to be gracefully migrated.



[when-to-use-rfcs]: /docs/contribute/governance/rfcs/rfc_process.md#when-to-use-the-process
[wlan-repo]: https://fuchsia.googlesource.com/drivers/wlan/intel/iwlwifi/+/refs/heads/main/third_party/iwlwifi
[idk]: /docs/development/idk/README.md
[bazel-sdk-rfc]: 0139_bazel_sdk.md
[rfc-153]: 0153_ninja_customization.md
[fint]: https://cs.opensource.google/fuchsia/fuchsia/+/main:tools/integration/fint/README.md
[rfc-72]: 0072_standalone_image_assembly_tool.md
[rfc-95]: 0095_build_and_assemble_workstation_out_of_tree.md
