{% set rfcid = "RFC-0148" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}

## Summary

Guidelines for project and infrastructure owners in the Fuchsia ecosystem to
create sustainable CI (Continuous Integration) experiences.

## Motivation

Until mid-2021, we kept most of our source code and prebuilts centralized in one
"Fuchsia tree". Accordingly, the infrastructure and its owners have been mostly
dedicated towards supporting that one tree.

As new out-of-tree projects e.g. [RFC-0095][rfc-0095] are brought up, in-tree
contributors may newly become out-of-tree contributors. The out-of-tree CI
systems should deliver a comparable or better experience than the in-tree
experience, and the experience should be familiar enough such that switching
between projects is low-friction. Otherwise, working out-of-tree represents a
productivity loss which can discourage evolution of the platform.

At the same time, the infrastructure team size won't be able to scale linearly
relative to the number of out-of-tree projects. We need to generalize our CI
capabilities from "mostly tailored to the Fuchsia project" to "usable by many
projects in the Fuchsia ecosystem". Otherwise, each project will demand custom
infrastructure and its own dedicated maintainers.

The lessons learned from building and maintaining Fuchsia's CI over the last
several years offer us a foundation for what to do, continue, and/or avoid with
respect to project-infrastructure integration going forward. Ultimately, the
goals of our CI systems are to make our projects easy to change, hard to break,
and efficient to ship: this RFC gives high-level recommendations to project and
infrastructure owners such that said systems can best achieve these goals.

## Stakeholders

*Facilitator:*

*   Hunter Freyer (hjfreyer@google.com)

*Reviewers:*

*   Aidan Wolter (awolter@google.com) - Product Assembly
*   Chase Latta (chaselatta@google.com) - Product Development Kit
*   David Gilhooley (dgilhooley@google.com) - Drivers
*   Jiaming Li (lijiaming@google.com) - Product Development Kit, Workstation OOT
*   Marc-Antoine Ruel (maruel@google.com) - Engineering Productivity
*   Nicolas Sylvain (nsylvain@google.com) - Engineering Productivity
*   Renato Mangini Dias (mangini@google.com) - Bazel

*Consulted:*

*   Anirudh Mathukumilli (rudymathu@google.com) - Foundation Infrastructure
*   Nathan Mulcahey (nmulcahey@google.com) - Foundation Infrastructure
*   Oliver Newman (olivernewman@google.com) - Platform Infrastructure
*   Petr Hosek (phosek@google.com) - Toolchain
*   Sébastien Marchand (sebmarchand@google.com) - 1P Infrastructure

*Socialization:*

This design was initially socialized with the Fuchsia engineering productivity
mailing list, iterated on in a Google doc, and shared with relevant stakeholders
to identify the reviewers listed in the above section. It was then converted to
markdown following the RFC template and moved to the RFC "Iterate" stage.

## Design

The "Avoid" subsections below enumerate common pitfalls which negatively impact
a project's CI, the project's contributors, and/or the infrastructure owners.
Conversely, the "Must Have" and "Consider" subsections are guidelines to help
navigate said pitfalls and more. They do not form an exhaustive list: they do
not include considerations for performance tracking, flake detection, etc. which
may also improve long-term project health but aren't required for a minimally
viable CI implementation.

### Avoid: Infrastructure dependence on project internals

When the infrastructure depends on project internals, both sides become harder
to change. Hitting infrastructure sharp edges when making seemingly benign
changes has been a long-standing pain point when working in Fuchsia, and is one
of the bigger complaints that contributors have about the engineering process.

For example, the infrastructure used to know many (and still knows some)
internal details of the Fuchsia build system which created sharp edges in
development i.e. the Fuchsia build was not free to change if it violated any of
the infrastructure's expectations. The infrastructure code does not live
alongside the Fuchsia code and thus its expectations can be hard to discover:
they are often only made known at presubmit or postsubmit runtime when something
fails. Other harmful examples include the infrastructure hardcoding paths in the
checkout, the names of tests, etc. Such references tend to organically
accumulate, progressively creating more and more friction over time.

Keeping the infrastructure compatible with the project becomes increasingly
difficult the more branches are involved and/or the longer they live. Either the
infrastructure is versioned in the project's history, or the live version of the
infrastructure must maintain compatibility with all active branches of the
project.

Also, when the infrastructure is encoding a lot of project-specific knowledge,
it's likely that each project has its own accompanying set of tailored CI
scripts, which has linearly-scaling implementation and maintenance costs.

### Avoid: Non-trivial reproduction of infrastructure behavior

When contributors cannot reproduce what the infrastructure is doing, the
infrastructure's results become much less actionable.

To debug unreproducible test failures, one will need to repeatedly submit
patches to the infrastructure until the test(s) pass, which is generally slower
and more resource-intensive than debugging locally. It also feeds the notion
that testing locally is pointless because the pass/fail correlation to
infrastructure-run tests is low.

The same goes for builds which are difficult to reproduce or cannot be
reproduced locally. The infrastructure should not be configuring builds in a way
that diverges heavily from developer workflows in non-obvious ways. For example,
as of this writing, the
[Fuchsia SDK remains difficult to build locally](https://fxbug.dev/44889#c18).
The infrastructure maintains its own [logic][sdk-recipe] which significantly
differs from the internal-only fx script, and there is no automation which
checks that they produce the same output.

In degenerate cases, unreproducible infrastructure behavior can force
"temporary" disabling of failing builds or tests to unblock submission and
recover the CI. In this state, they can further degrade from stacking breakages,
effectively becoming permanently disabled due to the impracticality of a fix.

### Avoid: Floating dependencies

Projects should avoid using floating dependencies, e.g. "fetch the latest
version of Bazel on the fly". Floating dependencies include the machine's
pre-installed software.

Any floating dependencies can flow into builds and tests, rendering them
[non-hermetic][hermeticity]. With floating dependencies, the infrastructure's
results cannot be fully attributable to the exact CL or commit(s) under test,
because they are not the only possible sources of change. Note that parts of the
infrastructure itself can often effectively be floating dependencies. Network
flakiness is an example of a common source of unpredictability in test results.

Floating dependencies create correspondingly larger headaches the more stable
the build is expected to be. For example, release branches typically only accept
hotfixes to minimize the risk of introducing new bugs, but floating dependencies
always represent such a risk.

They also contribute to the mysterious "it works locally, but not in the
infrastructure" phenomenon and vice-versa.

### Must Have: Reproducible checkout

A project's checkout must be fully reproducible with a simple series of steps on
a "clean" workspace. That workspace could be a developer's machine or an
infrastructure machine. An "update" of an existing checkout at a
[commit-ish][commit-ish] must always yield the same result as if the checkout
was created freshly from that commit-ish, at any point in time. This means that
all fetched dependencies must be pinned. A pinned (non-floating) dependency is
ideally cryptographic and deterministic e.g. a content hash. An immutable
reference can also be acceptable e.g. a semantic version as a git tag, though
the former is preferred.

Not only does a reproducible checkout provide a great experience for developers
getting started with a project, but it also makes the infrastructure's view of
the project less likely to diverge from the developer's view.

Non-reproducibility can also come from source code or binaries being deleted
and/or made inaccessible at any point in time. Hosting locations must be
approved by the Fuchsia infrastructure owners before they are integrated into a
project's checkout.

### Must Have: Clear separations between checkout, build, and test

A project must have clear separations of its checkout, build, and test phases.
This is necessary for the infrastructure to enforce security boundaries, as well
as optimize checkout, build, and test runtimes and resource usage. Clearly
separated phases also allow for better attribution of failures, especially
infrastructure failures versus user errors. For example, a failing build should
be attributable to a code issue and not, say, a timeout when fetching a remote
dependency.

The checkout phase fetches the source code and any dependencies. After the
checkout phase, one must have everything required to build. This means that the
build phase is hermetic i.e. cannot fetch any dependencies on the fly.

A build must be able to run without internet access. In practice, it still may
access the internet when using a remote distributed compiler, but only as a
performance optimization (it should not change the result of the build). This
requirement also benefits users working offline or with limited internet access
e.g. airborne users.

A project must not assume that the build and test phases are run on the same
machine in the infrastructure. For example, Fuchsia builds are run on separate
machines (with more cores) from test orchestrators and executors. This allows
the infrastructure to allocate machine resources more efficiently and speed up
builds.

Similarly, tests should be hermetic i.e. their inputs are explicitly mapped. See
[Testing scope][testing-scope] for more information. Tests shouldn't assume the
existence of a full checkout or build on the machine they are being run on, and
should not depend on other tests running on the same machine. The infrastructure
may shard tests onto separate machines, passing over only the explicitly mapped
inputs.

As for linters, they may be run post-checkout or post-build to provide
non-binary pass/fail hints in the context of code analysis and/or code review.
Linters which operate on the checkout can be considered part of the checkout
phase; likewise, linters which operate on build outputs can be considered part
of the build phase. They can be assumed to run on the same machine as their
associated phase.

### Consider: Reproducible build

Any two builds, given the same checkout and dependencies, should ideally yield
bit-for-bit identical outputs whether on a developer's machine or on an
infrastructure machine. If not bit-for-bit identical, builds should be at
minimum be functionally equivalent. [Reproducible builds][reproducible-builds],
like reproducible checkouts, help to create consistent views of the project
across users and across time.

Build reproducibility includes not depending on system-provisioned tools or
services, e.g. not depending on curl, ping, ip, etc. from the system. The build
should depend only on the checkout, which is thus responsible for vendoring all
build dependencies. Along similar lines, projects should be wary of using any
technologies which are not easily portable across platforms. Ideally, a project
should be runnable on vanilla installations of Debian/Ubuntu Linux, MacOS, or
Windows.

Note that the minimal set of dependencies required to actually bootstrap a
checkout should never flow beyond the checkout. For example, if bash is required
to perform the checkout, and bash is also required by the build, the checkout
should be pulling in a vendored bash. The build should then use that vendored
bash, *not* the bash used to bootstrap the checkout.

To speed up the build in presubmit, the infrastructure may seed the build
directory from a cache during the checkout phase. If incremental builds are not
always handled correctly, this strategy can create non-deterministic behavior.
In presubmit, the occasional incremental build issue can often be worth the
tradeoff for build speed. However, this optimization should not be used beyond
presubmit, and absolutely never for official builds where correctness and
security cannot be compromised.

### Consider: Clear layering of project and infrastructure

The infrastructure is responsible for automating builds and tests for projects
at scale. Emphasis on "automation at scale": a project should support performing
these tasks locally, mostly or entirely independently of the infrastructure.

This implies that the infrastructure holds very little logic to build and test
any specific project. These capabilities should be surfaced by the projects
themselves, and invoked by the infrastructure without knowledge aside from
well-known entrypoints, outputs, and configurations. A useful mental model is to
view the infrastructure as a new contributor going through a project's "Getting
Started" guide on building and testing.

For example, [fint][fint] is an abstraction over Fuchsia's build system which
obscures its internals from the infrastructure's view. With fint, the
infrastructure does not even know or care that Fuchsia uses [GN][gn]. This
reduces the amount of sharp edges that Fuchsia contributors can encounter when
modifying the build.

The infrastructure should also not be holding the configuration to fetch any
project dependencies, e.g. Bazel, Python3, miscellaneous Toolchains, etc. The
dependencies should be declared by the projects themselves. Infrastructure
machines should not be assumed to include any dependencies by default aside from
the minimal set of tools required to bootstrap a checkout. Project owners should
expect the available pre-installed set of tools to be reduced in the future.

There are still some cases where a project needs to know infrastructure
expectations. Some special kinds of outputs which are post-processed by the
infrastructure should follow an infrastructure-defined contract. For example,
binary size reports or code coverage reports to be displayed in Gerrit should
conform to the expected formats. This way, the infrastructure doesn't need
custom handling for each project which uses a particular infrastructure feature.

### Consider: Favoring CI configuration over code

In order to scale the number of supported projects, the infrastructure should
favor new configuration over new code. As an example, the CI code used to build
a class of similar projects should mostly be shared either at the scripting or
library levels. Configuration can account for any necessary differences between
projects e.g. repository URL, service accounts, checkout strategy, build
entrypoint, artifact upload destination, etc.

We support two checkout tools: [Jiri][jiri] or Git (with or without submodules).
Projects should use one of these options. Prebuilt dependencies should be hosted
by [Git-on-Borg][gob] or [CIPD][cipd]. The infrastructure code for building
should also be mostly shared if the logic to build each project is
well-abstracted per the section above.

By favoring configuration, the implementation cost for new CIs should be lower
than writing new CI code from scratch, which benefits projects needing to spin
up quickly. They also benefit from ongoing support and maintenance of the shared
infrastructure codebase and services.

### Consider: Build output abstraction

To facilitate the consumption of build artifacts, the build should have a
well-documented contract for its output surface area. The infrastructure is
likely to be a consumer of this surface area in order to perform various
post-build actions, e.g. uploading data to [BigQuery][bigquery], sharding and
running tests, or running binary size checks. This is in contrast to
"intermediate" build outputs which should be considered internals, and not
depended directly on by downstream consumers.

Project-defined tools can also be consumers of the build output. For example,
the [artifactory tool][artifactory] reads Fuchsia's build output to locate and
organize build artifacts in cloud storage. The infrastructure is only
responsible for invoking the tool with the infrastructure-specific arguments
i.e. a storage bucket name and a unique build identifier.

The build contract may adhere to some common infrastructure APIs. This helps
keep integrations robust, e.g. integration with the infrastructure's code
coverage service. Changes to the build internals of generating code coverage
metrics shouldn't require code changes on the infrastructure side.

The build contract should be tested e.g. schema changes don't result in
hard-transitions for downstream consumers.

### Consider: Main-first development

Projects should aim to keep the build healthy at tip-of-tree. This lets all
contributors live near the latest version of the code without needing to spin
off branches or work on an older version of the tree to sidestep bugs. This
helps reduce merge conflicts and prevents contributors from having significantly
different views of the project at any given time.

By default, the infrastructure's presubmit will try to rebase CLs onto
tip-of-tree (as this is a proxy for testing a clean submission), so it is
practical for a contributor's workflow to be as close as possible to this
behavior. Just as developers have similar views of the codebase, so should the
infrastructure.

The infrastructure's postsubmit facilitates keeping the build healthy at
tip-of-tree by continually testing tip-of-tree as new CLs land. If the build
goes red at tip-of-tree, this should be quickly reported by the infrastructure
and actioned by developers.

Sandbox branches may be used for code which is not intended to be submitted.
Note that their use is generally an exception to the norm, and not a first-class
flow backed by the infrastructure.

### Consider: Fast roll and release cadences

Each project should attempt to roll its dependencies at a fast cadence. The
infrastructure should facilitate this by automating the process of rolling
dependencies, and project owners should fix failing roll attempts with high
priority. Ideally, dependencies are rolled within O(hours) of release. The
staler a dependency is, the harder it becomes to roll forward and/or apply
cherry-picks. This is especially critical for security patches which are
time-sensitive.

In the same vein, each project should attempt to release at a fast cadence. The
infrastructure should facilitate this by automating the release process after
code integrates cleanly into mainline (commonly referred to as "continuous
deployment"). Project owners should invest heavily in writing automated tests
such that releases from near-tip-of-tree can be reliably integrated downstream,
following the main-first development model.

The infrastructure should also provide visibility into the dependency graph of
projects, where projects form the "nodes", and rolls and releases form the
"edges". Project owners should be able to trace CLs flowing through the graph
and discover where CLs have landed, or have gotten stuck, etc.

## Implementation

This RFC gives high-level guidelines on how projects should interface with the
infrastructure, but is intentionally light on implementation details. Each
project may follow the guidelines in any number of ways, and we don't want to
create artificial constraints by prescribing specifics. New out-of-tree projects
are still getting off the ground at this time, and anything we map out here is
likely to go stale as the projects evolve.

## Security considerations

While projects are encouraged to own their build and test logic, the
infrastructure must still own the security boundaries. Source code and/or
artifacts for each project must be able to securely flow into the next in order
for the many-project ecosystem to ultimately ship onto products.

The inputs to a CI task must be trusted: all source code and binaries must be
fetched from hosting locations which are approved by the Fuchsia infrastructure
owners. After the checkout phase is complete, there can be no more inputs, and
this should be enforced by the infrastructure e.g. attempting to fetch a
dependency during the build phase should result in an error.

Any outputs of the task should provide provenance i.e. *artifact* was built from
*project* at *revision:X*. When artifacts are uploaded, the infrastructure
should enforce that the artifacts are uploaded to storage with appropriate
scope. For instance, a project which depends on internal source code must be
prevented from uploading artifacts to a public bucket.

## Testing

The CI systems referred to in this RFC will enable building and testing new
projects at scale in a similar fashion as they do for the Fuchsia project today.
This reduces the amount of manual testing and debugging that project
contributors will need to do at their desks, in favor of offloading work to
infrastructure machines.

On the infrastructure side, Fuchsia's CI has already been worked on extensively
to enable automated testing at scale of its own code: in other words, the CI is
capable of testing changes to itself. Though some generalization may be needed,
we will largely inherit these capabilities when building new CIs.

## Documentation

This RFC will serve as a reference for new and existing projects.

On the infrastructure side, we will write documentation on new CI configuration
once we have generalized those capabilities, such that the process can be mostly
self-service. We will also generalize the existing documentation to account for
new out-of-tree projects rather than only applying to the in-tree
infrastructure.

## Drawbacks, alternatives, and unknowns

Like many software development best practices, following these best practices
may be more upfront effort for project contributors. For example, tracking
floating dependencies is a commonly used shortcut for quick iteration on the
cutting edge without the need for rollers. It can be argued that they are a
useful hack in the short term, but they should be considered technical debt,
among the other discouraged practices in this RFC.

Finding the best balance of technical debt for each new project is unknown, as
it had been during the development of Fuchsia. We continue to pay down build,
test, and infrastructure technical debt over time which was often taken to meet
project goals. This RFC does not seek to prevent technical debt, but rather to
make such tradeoffs more informed and intentional.

[rfc-0095]: /contribute/governance/rfcs/0095_build_and_assemble_workstation_out_of_tree.md
[sdk-recipe]: https://fuchsia.googlesource.com/infra/recipes/+/179288fb999a853cc4cf78ccbf3de2ee5be57707/recipes/sdk.py
[commit-ish]: https://git-scm.com/docs/gitglossary#Documentation/gitglossary.txt-aiddefcommit-ishacommit-ishalsocommittish
[hermeticity]: https://docs.bazel.build/versions/main/hermeticity.html
[fint]: /tools/integration/fint/README.md
[artifactory]: /tools/artifactory/README.md
[gerrit-size-plugin]: https://chromium.googlesource.com/infra/gerrit-plugins/binary-size/+/HEAD/README.md
[testing-scope]: /contribute/testing/scope.md
[reproducible-builds]: https://reproducible-builds.org
[jiri]: https://fuchsia.googlesource.com/jiri
[repo]: https://gerrit.googlesource.com/git-repo
[gob]: https://opensource.google/docs/glossary/#gitonborg
[cipd]: https://chromium.googlesource.com/infra/luci/luci-go/+/refs/heads/main/cipd
[gn]: https://gn.googlesource.com/gn
[bigquery]: https://cloud.google.com/bigquery/docs
