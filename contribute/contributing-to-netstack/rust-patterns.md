# Netstack Team's Rust Patterns

This document enumerates patterns that the netstack team has found to produce:

- Code that is more resilient to behavior change at a distance.
- Code that is more easily reviewed with less "working memory".

These guidelines are considered additive to the [Rust API rubric][api-rust].

The patterns suggested here provide centralized guidance and knowledge.
Contributions of all types - edits, additions, removals, etc. - are
encouraged.

Rough edges encountered during code authoring or reviewing under the proposed
guidelines are expected to make their way back into this document, so we can
codify alternative patterns when needed.

## Avoid unused results

This is mostly machine-enforced since <https://fxrev.dev/510442> via rustc's
[unused-results lint][unused-results]. See the
[documentation][unused-results-explanation] for an explanation of the
reasoning.

When discarding results, encode the types of ignored fields, so it becomes part
of the contract.

When discarding a result *prefer* to use a named variable prefixed with `_`
when the semantic meaning is not immediately clear from the type, and it won't
affect `drop` semantics.

The added information serves as a prompt for both author and reviewer:

- Is there an invariant that should be checked using the return?
- Should this function return a value at all?

> *Note:* Historically the team has used the form `let () = ` on assignments as
a statement that no information is being discarded. That practice is still
accepted, but it is no longer necessary with the introduction of the lint.

### Examples

#### Use the prompts

```rust
// Bad. The dropped type is unknown without knowing the signature of write.
let _ = writer.write(payload);
let _n = writer.write(payload);

// Okay. The dropped type is locally visible and enforced. Absence of invariant
// enforcement such as a check for partial writes is obvious and can be flagged
// in code review.
let _: usize = writer.write(payload);
let _n: usize = writer.write(payload);

// Good.
let n = writer.write(payload);
assert_eq!(n, payload.len());
```

#### Adopted dropping formats

```rust
// Encode the type of the ignored return value.
let _: Foo = foo::do_work(foo, bar, baz);

// When destructuring tuples, type the fields not in use.
let (foo, _) : (_, Bar) = foo::foo_and_bar();

// When destructuring named structs, no annotations are necessary, the field's
// type is encoded by the struct.
let Foo{ fi, fo: _ } =  foo::do_work(foo, bar, baz);

// Encode the most specific type possible.
let _: Option<_> = foo.maybe_get_trait_impl(); // Can't name opaque type.
let _: Option<usize> = foo.maybe_get_len(); // Can name concrete type.

// Use best judgement for unwieldy concrete types.
// If the type expands to formatted code that would span >= 5 lines of type
// annotation, use best judgement to cut off the signature.
let _: futures::future::Ready<Result<Buffer<_>>, anyhow::Error> = y();
// Prefer to specify if only a couple of levels of nesting and < 5 lines.
let _: Result<
    Result<(bool, usize), fidl_fuchsia_net_interfaces_admin::AddressRemovalError>,
    fidl::Error,
> = x();
```

#### Defeating patterns

Be mindful of defeating patterns:

```rust
// Bad, this is a drop that does not encode the type.
std::mem::drop(foo());
// Prefer instead:
let _: Foo = foo();
```

## Tests

### Declaration

Name tests after what they're testing without the `test_` prefix. That's the
adopted pattern in the [Rust standard library][std_addr_tests].

If the test name is not sufficient to encode the objective of the test, add a
short non doc comment before it or at the top of the function's body explaining
what this test is exercising. We use non-doc comments because we expect the
target audience to be readers of the code, and not of the public API.

Tests should always be in a module named `tests` or one of its descendents.
Crates that contain only integration tests do not need a `tests` module, e.g.
[network/tests/integration], [network/tests/fidl].

Example:

```rust
// Tests Controller::do_work error returns.
#[test]
fn controller_do_work_errors() { /* ... */ }
```

Test support functions should be in a module named `testutil`. If the module is
meant for use in-crate only, it should be declared `pub(crate)` and
`#[cfg(test)]`. This module should not contain any `#[test]` functions. If tests
are needed for the functionality in the `testutil` module, a sub-module called
`tests` should be created (i.e., `testutil::tests`).

### Prefer panics

Do **not** use Rust's support for [**tests which return
Result**][rust_test_result]; such tests do not automatically emit backtraces,
relying on the errors themselves to carry a backtrace. Test failures that don't
emit backtraces are typically much harder to interpret. At the time of writing,
the [backtrace feature in Rust is unstable][rust_backtrace_stabilize] and
[disabled in the Fuchsia Rust build configuration][rust_backtrace_disabled], but
even if enabled not all errors contain a backtrace; best to panic to avoid
relying on external factors for backtraces.

## Imports

The following rules apply to styling imports.

Employ one `use` statement per crate or direct child module. Combine children of
those into the same use statement.

```rust
use child_mod::{Foo, Bar};
use futures::{
    stream::{self, futures_unordered::FuturesUnordered},
    Future, Stream,
};
```

Always alias imported-but-not-referenced traits to `_`. It avoids cluttering the
namespace and informs the reader that the identifier is not referenced.

```rust
use futures::FutureExt as _;
```

Avoid bringing symbols from other crates into the scope. Especially if things
all have similar names. Exceptions apply for widely used, self-explanatory `std`
types like `HashMap`, `HashSet`, etc.

Importing symbols from the same crate is always allowed, including to follow the
pattern of declaring crate-local `Error` and `Result` types.

Some crates rely heavily on external types. If usage is expected to be
ubiquitous throughout the crate, it's acceptable to import those types as a
means to reduce verbosity, as long as it doesn't introduce ambiguity.

```rust
// DON'T. Parsing this module quickly grows out of hand since it's hard to make
// sense where types are coming from.
mod confusing_mod {
    use packet_formats::IpAddress;
    use crate::IpAddr;
    use fidl_fuchsia_net::Ipv4Address;
}

// DO. Import only types from this crate.
mod happy_mod {
    use crate::IpAddress;

    fn foo() -> packet_formats::IpAddress { /* .. */ }
    fn bar() -> IpAddress { /* .. */ }
    fn baz() -> fidl_fuchsia_net::Ipv4Address { /* .. */ }
}
```

Some well-known crates have adopted aliases or alias formation rules. Those are:

- `fuchsia_async` may be aliased to `fasync`.
- `fuchsia_zircon` may be aliased to `zx`.
- `fidl_fuchsia_*` prefixes may be aliased to `f*`, e.g.:
  - `use fidl_fuchsia_net_interfaces_admin as fnet_interfaces_admin;`
  - `use fidl_fuchsia_net_routes as fnet_routes;`

Importing `*` is strongly discouraged. Tests modules that import from `super`
are acceptable exceptions; it is assumed that a test module will use most if not
all symbols declared in its parent.

Importing types in function bodies is always allowed, since it's easy to reason
locally about where the types come from.

```rust
fn do_stuff() {
    use some::deeply::nested::{Foo, bar::Bar};
    let x = Foo::new();
    let y = Bar::new();
    /* ... */
}
```

## Prefer exhaustive matches

Match exhaustively whenever possible, avoiding [catch-all patterns]. Matching
exhaustively provides more local context during reviews, acts as a prompt to
revisit when the enumeration is updated, and requires more explicit forms to
drop information.

Some patterns implicitly defeat exhaustive matches. We should use them with
care:

- Avoid `if let` patterns, since that's effectively a non-exhaustive match. We
  carve out the exception to allow binding to the `Some` value of an
  `Option<T>`, since it's a well known type and the `None` variant doesn't carry
  any information. Note that if `T` is an enum, `if let Some(Foo::Variant)` is
  not a valid use of the exception, since it sidesteps an exhaustive match on
  `T`.
- Avoid methods on enum receivers that indicate variants without matching, e.g.:
  - `is_foo(&self) -> bool` enum methods like [`Result::is_ok`] or
  [`Option::is_none`].
  - `foo(self) -> Option<T>` enum methods like [`Result::ok`], which serve as
  single-variant extraction helpers that are easy to miss in review.

Rust provides the [`non_exhaustive`] attribute which defeats the intent of
matching exhaustively, and `flexible` FIDL types are annotated with that
attribute. When dealing with such types, attempting to match exhaustively is
prone to becoming stale - the type can evolve without breaking the code - and
should, thus, be avoided.

```rust
// Don't attempt to match exhaustively, exhaustive enumeration is prone to
// becoming stale and misleading future readers if `Foo` takes more variants.
fn bad_flexible_match(foo: fidl_foo::FlexibleFoo) {
    match foo {
        fidl_foo::FlexibleFoo::Bar => { /* ... */ },
        foo @ fidl_foo::FlexibleFoo::Baz |
        foo => panic!("unexpected foo {:?}", foo)
    }
}

// Use the catch-all pattern instead when the type is non_exhaustive.
fn good_flexible_match(foo: fidl_foo::FlexibleFoo) {
    match foo {
        fidl_foo::FlexibleFoo::Bar => { /* ... */ },
        foo => panic!("unexpected foo {:?}", foo)
    }
}

// Note that if the type was not marked non_exhaustive we'd prefer to match
// exhaustively.
fn strict_match(foo: fidl_foo::StrictFoo) {
    match foo {
        fidl_foo::StrictFoo::Bar => { /* ... */ },
        foo @ fidl_foo::StrictFoo::Baz |
        foo @ fidl_foo::StrictFoo::Boo => panic!("unexpected foo {:?}", foo)
    }
}
```

> TODO(https://github.com/rust-lang/rust/issues/89554): Revisit `non_exhaustive`
guidance once `non_exhaustive_omitted_patterns` lint is stabilized.

## Avoid default type parameters

Rust supports defining parameterized types with defaulted type parameters.
This can be convenient for certain parameters, e.g. ones that are only used to
override behavior in tests. For example:

```rust
// This can be used as `Foo<X>` or `Foo<X, Y>`.
struct Foo<A, B = u32>(A, B);

// Blanket impl for all possible `Foo`s.
impl<A, B> MyTrait for Foo<A, B> { /* ... */ }
```

The problem with defaulting type parameters is that it can easily lead to
incomplete blanket implementations. Suppose `Foo` is extended with another defaulted type parameter:

```rust
// Now `Foo<A> = Foo<A, u32> = Foo<A, u32, ()>.
struct Foo<A, B = u32, C = ()>(A, B, C);
```

The `impl MyTrait` block still works unmodified, so there's **no signal from the
compiler** that the coverage is incomplete: it only covers `Foo<A, B, ()>`
instead of all possible `Foo<A, B, C>`s. Avoiding defaulted type parameters puts
the onus on the author to make sure any impls, blanket or otherwise, cover the
correct set of types.

## Process for changes to this page

All are invited and welcome to propose changes to the patterns adopted by the
[Netstack team]. Proposed patterns will be accepted or rejected by the team
after a process of consensus building through discussion, falling back to a
go/no-go simple majority vote.

Follow the steps below to propose a change.

1. Author and publish a CL changing this page.
1. *\[optional\]* Socialize with a small group and iterate.
1. Request review from the entire team through email and chat. Non-Googlers can
   reach out through [discussion groups].
1. Iterate on the CL based on review comments and offline sessions.
   Remember to publish outcomes of offline sessions back to the CL.
1. Team members may express support `+1`, opposition `-1`, or indifference.
   Indifference is voiced through a single comment thread on Gerrit where
   members state indifference. That thread is to be kept unresolved until the
   CL merges. Team members may change their vote at any time.
1. Proposals will be in review for at most 2 weeks. A last call announcement is
   sent at the end of the first week. The timeline may be short-circuited if the
   *entire* team has cast their vote.
1. When consensus can't be achieved, the team will tally the votes and make a
   decision to go ahead or not using a simple majority.

Things to keep in mind:

* Authors and leads will shepherd changes through this process.
* Be respectful of others; address points on their merits alone.
* Avoid long comments; express disagreement with supporting arguments
  succinctly.
* Back-and-forth on the CL is discouraged. Fallback to breakout video or
  in-person sessions (public, preferably) and encode the consensus back into
  the comment thread.
* Controversial points can be dropped and addressed in a follow-up proposal if
  necessary.
* Indifference votes are used to measure the perceived benefit of encoding some
  patterns. A strong indifference signal is interpreted as a hint that the point
  being discussed does not merit encoding as a pattern.

[api-rust]: /development/api/rust.md
[unused-results]: https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unused-results
[unused-results-explanation]: https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#explanation-31
[Netstack team]: /src/connectivity/network/OWNERS
[discussion groups]: /contribute/community/get-involved.md#join_a_discussion_group
[rust_test_result]: https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/question-mark-in-main-and-tests.html
[rust_backtrace_stabilize]: https://github.com/rust-lang/rust/pull/72981
[rust_backtrace_disabled]: https://cs.opensource.google/fuchsia/fuchsia/+/main:third_party/rust_crates/Cargo.toml;l=308-309;drc=fb9288396656bf5c9174d39238acc183fa0c4882
[std_addr_tests]: https://github.com/rust-lang/rust/blob/1.49.0/library/std/src/net/addr/tests.rs
[network/tests/integration]: /src/connectivity/network/tests/integration
[network/tests/fidl]: /src/connectivity/network/tests/fidl
[`Result::is_ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.is_ok
[`Result::ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.ok
[`Option::is_none`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none
[`non_exhaustive`]: https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
[catch-all patterns]: https://doc.rust-lang.org/book/ch06-02-match.html#catch-all-patterns-and-the-_-placeholder
