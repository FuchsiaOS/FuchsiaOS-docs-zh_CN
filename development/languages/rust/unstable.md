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
Fuchsia is set [in `buildtools/fuchsia.ensure`][buildtools]. The latest revision of
Rust which can be set in `buildtools` is the most recently completed build [here][rust builder].
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

Unstable feature requests should be sent to the rust-fuchsia@fuchsia.com mailing list.
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
- cramertj@google.com
- etryzelaar@google.com
- raggi@google.com
- tkilbourn@google.com

## Currently Used Features

This list includes all of the unstable features currently used in Fuchsia.

* Rust 2018 Edition
    * Summary: The Rust 2018 edition is the upcoming release of Rust that makes several breaking
      changes while still allowing interop with non-Rust 2018 crates. The main source of
      breakages are (1) changes to the module system and (2) the addition of new keywords.
      For more details on how editions work, see [the edition guide].
    * Use in Fuchsia: An “edition” key has been added to the rustc_xxx set of GN templates.
      All Rust targets will gradually move towards the use of `edition = “2018”` to prepare
      for the upcoming release and allow for early use of the async/await feature. `async`
      blocks and closures are only allowed in Rust 2018 because `async` was not a keyword in
      Rust 2015.
    * Remaining before stabilization: There’s some ongoing discussion around final non-breaking
      tweaks to the new module system and perhaps some in-band-lifetimes lints against single-letter
      lifetimes in impl body headers, but overall the edition is shaping up fast and is set to
      ship later this year.
      For more details, see [Rust 2018: an early preview] and [Rust 2018: the home stretch].
      There are no major breaking changes expected before stabilization, and shipping the 2018
      edition is a primary goal for this year, so the risk of never stabilizing is extremely low.
    * Owner: cramertj@
* `async_await` and `await_macro`
    * Summary: These two features enable the syntactic transformation from code that looks like
      `move async { await!(x); await!(y), await!(z); }` into an anonymous
      `enum State { First(X, Y, Z), Second(Y, Z), Third(Z), Done }` that implements the `Future`
      trait. This also makes it possible to await non-’static futures by internally creating a
      self-referential generator, allowing for significantly more performant, idiomatic, readable,
      and writable code than the current futures combinators.
    * Use in Fuchsia: These features will be used in fuchsia-async and in applications which make
      use of the async/await feature in order to achieve more performant, ergonomic, and idiomatic
      async code.
    * Remaining before stabilization: Apart from being dependent on all the other features, there
      are no major additional stability risks (read: no major unresolved design decisions)
      introduced by these features. There are changes planned to the implementation, but they’re
      mostly polish, performance optimizations, and a few scattered bugfixes.
    * Owner: cramertj@
* `pin`
    * Summary: This feature includes the `PinMut` type and the `Unpin` trait. `PinMut` is a mutable
      reference to an object that does not allow moving the underlying object unless it implements
      the `Unpin` trait. This is used to create sound self-referential types.
    * Use in Fuchsia: Needed for `async_await`
    * Remaining before stabilization: This feature has gone through a number of design iterations
      and is the most likely to receive backwards-incompatible changes -- there are a number of
      design ideas floating around about the potential to make it safe to project from
      `PinMut<MyType>` to `PinMut<FieldOfMyType>`, but these have so far been fruitless
      (mostly depending on features that aren’t even RFC’d, let alone implemented). It’s likely
      that PinMut will stabilize without these extensions, but cramertj@ will be in charge of
      managing any transition necessary. This feature has also been proposed for stabilization.
    * Owner: cramertj@
* `arbitrary_self_types`
    * Summary: This simple feature enables the use of `self: PinMut<Self>` which is necessary to
      write implementations of the `Future` trait.
    * Use in Fuchsia: Needed for `async_await`
    * Remaining before stabilization: This is a particularly simple feature and likely won’t
      see any breaking changes (especially to our minimal usage).
    * Owner: cramertj@
* `futures_api`
    * Summary: This feature enables the `std::task` and `std::future` modules which provide the
      `Future` interface for the standard library.
    * Use in Fuchsia: Needed for `async_await`
    * Remaining before stabilization: As in async/await, cramertj@ wrote the implementation of
      this feature, and while its surface area is large, there aren’t significant planned
      changes to it. cramertj@ will manage any necessary transition.
    * Owner: cramertj@

[the edition guide]: https://rust-lang-nursery.github.io/edition-guide/editions/index.html
[Rust 2018: an early preview]: https://internals.rust-lang.org/t/rust-2018-an-early-preview/7776
[Rust 2018: the home stretch]: https://internals.rust-lang.org/t/rust-2018-the-home-stretch/7810

[buildtools]: https://fuchsia.googlesource.com/buildtools/+/master/fuchsia.ensure#46
[rust builder]: https://ci.chromium.org/p/fuchsia/g/rust/console
[third_party/rust]: https://fuchsia.googlesource.com/third_party/rust/
[tracking issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AC-tracking-issue
