# Review process for external Rust crates

*Here because you need to do a review? Skip straight to the [process](#process)
and [review guidelines](#review-guidelines).*

Rust's external ecosystem of libraries, or crates, is an enormous benefit of
using the language. This body of existing code can be a force multiplier for a
team of any size, accelerating and de-risking development.

At the same time, external code, like all code, comes with risks. We should use
external crates, but use them wisely! This doc is about how to maximize the
benefits while understanding and minimizing risks.

## Background

With roughly 1 million lines of external Rust code in our tree at the time of
writing, code review of new crates and updates to existing crates takes
considerable work. In order to cope with the scale of Rust in our codebase
today, we must empower any team using Rust to review crates that the team
depends on.

At the same time, there is uncertainty about how to review external code.
Reviews of external code are different from first-party code reviews, and norms
in the industry are not well established. In fact, many smaller organizations do
not do external code review at all, relying entirely on other quality signals.

On a project like Fuchsia, we cannot tolerate such risks and must review code
that we use to ship our products. At the same time, we should focus our efforts
to maximize benefits while minimizing process overhead.

## Principles

### Allocate review effort according to risk

Time and effort are limited. Identify the biggest risks that external code may
introduce, and focus efforts on minimizing these. Compare these risks to those
of writing a new implementation.

### Consider future costs and benefits
Look beyond the here and now – a convenient shortcut today could become a big
problem later. Solving a more general class of problems now can have repeated
benefits over the life of the project.

### Be wary of surprises

Remember that we will pay the cost of clearing up a confusing piece of code or
its API many times over.

### No default outcome

All of these principles apply both to the decision to use external code **and**
to the decision to implement ourselves.

### Give back

Open source libraries can be a major benefit to the project. When we review
them, make the details of those reviews accessible and legible to people outside
the project.

## Process {#process}

Reviewing third-party code can be a lot of work for everyone involved. It's
important to follow the process for requesting and performing third-party
reviews to keep the amount of overhead and repeated work to a minimum.

### Adding or updating an external crate

When you're adding or updating an external crate and need your changes reviewed
by someone else:

1.  Follow the
    [external Rust crates](/docs/development/languages/rust/external_crates.md)
    documentation to add the crate to your tree. **Do not upload a change
    to Gerrit yet.**
2.  If there are any new crates, including dependencies or changes to licenses
    in new versions of existing crates, request
    [OSRB approval](/docs/contribute/governance/policy/osrb-process.md) for
    them.
3.  After receiving OSRB approval for any new crates, upload a change to Gerrit.
    Make sure to:
    *   Cite OSRB approval bugs in the commit message.
    *   Separate first-party code changes from external code changes where
        possible.
    *   If the crate requires updating a number of transitive dependencies,
        consider using
        [`cargo update`](https://doc.rust-lang.org/cargo/commands/cargo-update.html)
        to batch the transitive updates into one or more batches to reduce the
        CL size. To reduce review overhead, dependencies that aren't used on our
        supported platforms can be
        [replaced](/docs/development/languages/rust/external_crates.md#importing_a_subset_of_files_in_a_crate)
        with an empty crate using cargo's manifest patching feature.
4.  Add reviewers to the CL. Anyone (including you!) can review as long as they
    understand this section and the guidelines below.
    *   You can find reviewers by asking in the #rust channel on the Fuchsia
        Discord or other chat rooms where Fuchsia rustaceans can be found.
    *   If the review requires domain-specific expertise (for example, unsafe
        code), look for reviewers with that expertise.
    *   Make sure reviewers know which crates they're being asked to review. You
        may assign crates to individual reviewers, which helps with large CLs.
5.  Once all code has been reviewed, add one of the
    [`rust_crates` `OWNERS`](https://cs.opensource.google/fuchsia/fuchsia/+/main:third_party/rust_crates/OWNERS)
    for final approval.[^2] Their job is to make sure the process has been
    correctly followed, which should be clearly supported in CL comments. Being
    proactive will help ensure a quick approval.

### Reviewing an external crate

When someone else has requested a review from you, make sure to:

1.  Review the code according to the [review guidelines](#review-guidelines),
    applying your best judgment and seeking outside assistance when necessary.
2.  Comment on the code saying which crates you reviewed. Note any concerns you
    have and caveats about the review. Note any surprising behavior or bugs
    inline. In general, note any risks even if you do not think they should
    block merging.
    *   On the top line of your CL-level comment, say which crates and versions
        you reviewed.
    *   Use an asterisk if there are any caveats or risks noted in your review.
        You can abbreviate this by saying "all crates" or "all crates except..."
        so the OWNERS can quickly skim the comments before giving final
        approval.
3.  If the code looks acceptable for merging, add "Code Review +1". If you found
    significant bugs or other red flags, add "Code Review -1" and optionally
    suggest a resolution such as:
    *   Patching the crate upstream before merging.
    *   Closing the CL and looking for alternatives.
    *   If the offending code is in a dependency that isn't used on
        Fuchsia's supported platforms, we can
        [replace the crate](/docs/development/languages/rust/external_crates.md#importing_a_subset_of_files_in_a_crate)
        with an empty one using cargo's manifest patching feature.

### Approving an external addition or update (OWNERS)

1.  Look for evidence the [review guidelines](#review-guidelines) have been
    followed on the CL:
    *   Review the process for
        [adding or updating an external crate](#adding-or-updating-an-external-crate)
        to make sure it has been followed.
    *   Prefer comments on the CL to informal communication whenever possible,
        as comments leave an audit trail.
    *   If evidence is missing, ask the CL owner to complete the review process
        and refer them to this document.
2.  Review any risks noted by the reviewers:
    *   Request clarifications where needed.
    *   Mention any problems that should block merging or warrant further
        discussion. Add "Code Review -2" to ensure that the CL is not merged
        before those problems are addressed.
3.  Once the guidelines have been appropriately followed, any risks are
    acceptable, and you're comfortable with merging, add "Code Review +2".

## Review Guidelines {#review-guidelines}

An external crate review involves up to four primary components:

*   [Architecture review][Architecture] assesses the external code at a high
    level, and is meant to catch "obvious" problems.
*   [Quality review][Quality] lends extra scrutiny to newly-introduced crates.
*   [Code review][Code] focuses on verifying the correctness of the code.
*   [OSRB approval][OSRB] ensures that the code we add complies with Fuchsia's
    licensing policies. OSRB approval is a separate process and must be
    completed before a changelist introducing external code is uploaded.

Which components are required depend on the nature of the change:

Crate action                                        | [Architecture] review required | [Quality] review required | [Code] review required | [OSRB] review required
--------------------------------------------------- | :----------------------------: | :-----------------------: | :--------------------: | :--------------------:
Added as a direct dependency in Cargo.toml          | ✅                              | ✅                         | ✅                      | ✅
Vendored as an indirect dependency of another crate | ❌                              | ✅                         | ✅                      | ✅
Updated version                                     | ❌                              | ❌                         | ✅                      | if licenses changed\*

[Architecture]: #architecture
[Quality]: #quality
[Code]: #code-review
[OSRB]: /docs/contribute/governance/policy/osrb-process.md

*\* When updating an external crate, OSRB approval is only required if licenses
change (including per-file license changes).*

### Architecture review for crates used by Fuchsia code {#architecture}

The architectural review is meant to catch "obvious" problems, and is intended
to be lightweight. If there is uncertainty as to whether a crate makes sense
from an architectural perspective, it should be noted on the CL so the author
and reviewers can make a final judgment.

When adding a direct dependency for use in-tree (i.e. it is listed directly in
our `Cargo.toml` manifest), new users and reviewers should ask these questions:

*   Do we have a similar crate in the tree that could be used instead?
*   How many dependencies will this crate pull in, and how big are they?
*   Is the cost of reviewing and updating those dependencies disproportionate to
    the benefit we gain by using the crate?
*   Does this crate make sense in the context of our architecture? And for code
    running on Fuchsia targets:
    *   Is this usable in an async context? (e.g. Does the API require blocking
        semantics?)
    *   Does this crate rely on POSIX emulation heavily?
*   Does this crate have a sensible API with sufficient documentation?
    *   If the API is simple and self-evident, minimal documentation is OK.
    *   If the API contains complex abstractions, lack of documentation has a
        cost.
    *   If the API has undocumented invariants and especially unsafe code, it's
        highly risky.

Note that these questions do not apply to transitive dependencies that we won't
use directly. We should ask and answer these questions for existing transitive
dependencies when they are promoted to direct dependencies.

### Quality review for newly vendored crates {#quality}

In addition to the review guidelines below, reviewers should give extra
consideration to new crates, whether they are being used directly by us or as a
dependency of another crate.

#### Make sure the crate is [OSRB approved](/docs/contribute/governance/policy/osrb-process.md).

Warning: You must receive approval from the OSRB before pushing a commit to
Gerrit that adds external code.

#### Think about the future.

Consider how our relationship with this crate may change over time.

##### Is the review of this crate and its dependencies worth the benefits of using its API?

Consider the cost of reviewing future updates as well.

##### What other contexts might this crate be used in?

The implementation will not be reviewed again for each new use. If you think
a crate should only ever be used on a particular platform, or you review it with
that assumption in mind, state it in a comment in the `Cargo.toml`.

Some crates are not safe to use in every possible context, such as on particular
platforms. If there are any contexts where the crate is not safe to use, then we
must modify the build to prevent the crate from being used in in them.
Otherwise, the crate cannot be imported.

##### What would happen if the maintainer abandons the crate?

Would we be willing to fork and maintain it ourselves?

#### Pay attention to signals of quality.

All of these quality signals can be found using [crates.io](https://crates.io/)
or the source repository which is usually linked to from there.

First-order signals provide us direct evidence of a crate's quality:

*   Code review
    *   Code review is the most fundamental first-order signal. While code
        review is always required, it is almost never totally exhaustive.
*   Testing
    *   Lack of testing in a crate is a red flag. Tests don't all need to be
        to be reviewed, but it is worth spot checking some tests for meaningful
        semantic checks and good coverage. Also check if a crate's tests are
        passing in its CI, or if they pass in a local checkout with
        `cargo test`. Solid testing is a very good sign.

Second-order signals provide indirect evidence of a crate's quality, and should
be used as supporting evidence. Missing second-order signals should not
disqualify using a crate. Instead, these signals should help fill in gaps in
confidence and tip the balance in moments of ambiguity:

*   Multiple maintainers
*   Well-known authors
*   Well-used reverse dependencies
    *   These are listed under "Dependents" on crates.io.
*   Activity in the repository and issue tracker

### Code review for all external code {#code-review}

When reviewing external code, whether a new crate or updates to an existing one:

#### Look for risks

Common risks present in external code include:

*   `unsafe` code
    *   Unsafe code should only be used when necessary. It should be easy to
        easy to follow and/or document its invariants and how they are
        preserved. Unsafe APIs should be very rare and must always document the
        invariants the caller needs to uphold.[^3]

        Unsafe code in external crates must be reviewed by an unsafe Rust code
        reviewer. See ["Request an unsafe code review"] for more details.
*   Code that requires specialized domain expertise to understand
    *   If possible, find a domain expert to review this code. Examples include
        unsafe, low-level atomics and concurrency, cryptography, and network
        protocol implementations.
*   Code meant to be used in critical paths
    *   This includes security-critical paths, like cryptography, as well as
        performance-critical paths. Pay special attention to this code to make
        sure it doesn't compromise the critical path.
*   Overly complicated code
    *   Idiomatic Rust leverages appropriate levels of abstraction, using the
        type system to manage invariants when possible. If the code is hard to
        follow and leaves you without confidence that those invariants are well
        managed, it may be of low quality and contain avoidable bugs.

As always, it's important to remember our alternatives. Assuming we need this
functionality, would we be navigating the same risks if we wrote the code
ourselves? Would doing so actually produce better results, and would it be worth
the effort of writing and maintaining that code?

["Request an unsafe code review"]: #request-an-unsafe-code-review

#### Verify for correctness, but don't go overboard.

In an ideal world, we'd be able to formally verify all of the external code that
we use. That's usually not realistic though, so we need to make the best use of
our time and effort to raise our confidence while still keeping the process
moving along. Do your best to:

*   Verify that the implementation makes sense
    *   Given the function signature, trait, and so on.
*   Look for surprises
    *   Make sure to note any that you find in CL comments (ideally inline with
        the surprising code).
*   Focus on the code in front of you
    *   It's okay to assume that other functions do what they say since you'll
        review them eventually. Tracing the entire function call graph shouldn't
        be necessary, and it's usually a bad sign if it is. Use your best
        judgment to focus your efforts.
*   Make sure that `build.rs` changes are reflected in our build
    *   [`build.rs` scripts] do not run in our build, but need to get translated
        when vendoring crates. `cargo-gnaw` has limited support for this, but it
        doesn't catch everything. Look over changes to these and verify anything
        relevant to our build is reflected in our build rules. If you aren't
        comfortable reviewing these, ask the CL owner to find another reviewer
        for them.

[`build.rs` scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

#### Skip what's irrelevant.

You don't need to review the following:

*   Code style
*   Unchanged code that was already reviewed
*   Individual test cases and benchmarks
*   Platform-specific code on platforms we are sure we would never use it on[^4]
*   Documentation that isn't directly helpful in understanding the API surface
    and implementation well enough to review it

Some of these are still relevant when assessing the quality of new crates,
discussed above.

#### Request an unsafe code review

To make sure Fuchsia is built on external code that is sound, we do a thorough
review of all unsafe code in external crates. Unsafe code usually requires
special expertise to review, and so when a crate adds or updates unsafe code it
must be approved by an unsafe reviewer.

To request an unsafe code review:

1.  File a bug using the "Unsafe review for external crate" template and fill
    out:
    -   The modified crates that are direct dependencies in the title.
    -   The review link for your CL.
    -   The date your CL was uploaded.
    -   The total number of lines changed (located at the bottom of your files
        list in Gerrit).
    -   The constituent crates that have been added or updated.
    -   Here's [an example](https://fxbug.dev/121497) for reference.
1.  Add "Fuchsia Rust Unsafe Reviews <fuchsia-rust-unsafe-reviews@google.com>"
    as a reviewer to your CL. A reviewer will be chosen at random and assigned
    to your CL.

If your review is time-sensitive, increase the priority on your bug and leave a
comment explaining your situation.

## More reading

*   [Fuchsia Rust 3p OWNERS](http://fxrev.dev/502938)

[^1]: In the near future we expect to create a Rust reviewer list that
    automatically finds and assigns reviewers from a pool of volunteers.
[^2]: In the near future we
    [expect Rust third party crates to be assigned more granular ownership](http://fxrev.dev/502938)
    for reviews which will allow crate upgrades to be managed by a broader set
    of people.
[^3]: Also see our
    [guidelines for unsafe](/docs/development/languages/rust/unsafe.md)
    in first-party code.
[^4]: We support Fuchsia, Linux, and Mac, and should expect to support Windows
    at some point. Some of our Rust code is also compiled for wasm targets.
