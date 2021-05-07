# Unstable Rust Features

## Background

### What is Rust's `#![feature(...)]`?

When using the `nightly` channel of Rust or when compiling `Rust` manually with the
appropriate flags, it's possible to use unstable features. These features include
language additions, library additions, compiler features, and other capabilities
not subject to Rust's usual stability guarantees. Most of these features are
temporarily-unstable additions that will become stable after a period of time has
passed during which testing, discussion, and further design has completed. Some
features, however, are intentionally permanently-unstable features intended for
internal compiler use. Other features may be removed completely when a better solution
has been found or when it was determined that the downsides of the feature outweighed the
advantages. Each feature has an [associated tracking issue on the
`rust-lang/rust` Github repository][tracking issues].

### Our Rust Versioning Process

Fuchsia currently builds using a pinned revision of upstream Rust's master branch.
We mirror Rust into [this repository][third_party/rust]. The version used to compile
Fuchsia is set [in the `prebuilts` manifest][prebuilts]. The latest revision of
Rust that can be set in `prebuilts` is the most recently completed build [here][rust builder].
We currently update the Rust version fairly often to pick up new changes we upstream,
such as changes to syscalls used by the standard library.

## The Goal

We want to be able to roll forward or backward to other versions of Rust to pick up
bugfixes or roll back problem-inducing changes. Depending on too many unstable nightly
features could make this process extremely painful.

We also want to have code that is clear and easy to use, and use of unstable or rapidly
changing features can make code harder to understand or modify. Unstable features are
often poorly documented, and what documentation exists is often out of date.

That said, there are also a number of features that are designed explicitly for Fuchsia's
use cases. These features provide great readability or performance benefits, and our use
of them helps to prove them out and move them further along the path to stabilization.

## The Process

Unstable feature requests should be sent to the [rust-users@fuchsia.dev] mailing list.
They should be hosted on docs.google.com to allow for comments and suggestion on the
document itself. Proposals should include the following information:

- A quick summary of the feature
- What the feature is used for in Fuchsia
- A summary of what is left before the feature can be stabilized
- A person in charge of owning a particular feature who will follow the tracking issue,
  participate in discussion on how to modify or stabilize the feature, and manage any
  necessary updates to Fuchsia code that result from breaking changes to the feature
  or removal of the feature.

Following this email is a week-long comment period during which any arguments for or
against a feature should be laid out on the doc. Once this period is over, a group
of reviewers will meet and come to a consensus decision on whether or not to allow
use of the feature. This decision will be based on arguments previously discussed on
the doc, and will not include new arguments brought by the review board members. If
new arguments surface, they will be added to the doc and more time will be given for
others to respond.

If the feature is approved, the feature summary, usage, stabilization report,
and owner listed in the doc are added to the "Currently Used Features" section listed
below. This documentation must be checked in before the feature can be used.

The current list of reviewers is as follows:

- bwb@google.com
- etryzelaar@google.com
- raggi@google.com
- tkilbourn@google.com
- tmandry@google.com

## Currently Used Features

There are no longer any unstable features used in Fuchsia! The last one was
`async_await`, which was stabilized in 2019 Q3.

[the edition guide]: https://rust-lang-nursery.github.io/edition-guide/editions/index.html
[Rust 2018: an early preview]: https://internals.rust-lang.org/t/rust-2018-an-early-preview/7776
[Rust 2018: the home stretch]: https://internals.rust-lang.org/t/rust-2018-the-home-stretch/7810

[prebuilts]: https://fuchsia.googlesource.com/integration/+/HEAD/prebuilts
[rust builder]: https://ci.chromium.org/p/fuchsia/g/rust/console
[third_party/rust]: https://fuchsia.googlesource.com/third_party/rust/
[tracking issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AC-tracking-issue

[rust-users@fuchsia.dev]: https://groups.google.com/a/fuchsia.dev/g/rust-users
