{% set rfcid = "RFC-0083" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

This document proposes a way to annotate FIDL elements with versions and a
mechanism to generate bindings at a given version. This decouples API evolution
from its adoption, making it easier for library authors to make changes while
providing stability to end-developers. This lays the groundwork for FIDL's role
in [RFC-0002: Platform Versioning][rfc-0002].

## Motivation

While FIDL provides many affordances for ABI compatibility during changes, in
practice evolving APIs is difficult. In the Fuchsia SDK, making a FIDL change
that is ABI-compatible but not API-compatible requires a carefully coordinated
soft transition to avoid breaking compilation downstream. When something does
break, we usually have to revert the change in Fuchsia. As usage of SDK
libraries increases, so does the difficulty of making these changes.

FIDL Versioning addresses this, allowing FIDL library authors and consumers to
move forward at their own pace. When a library author adds, removes, or modifies
an API, the change is released in a new API level. Applications targeting the
old API level see no change in the bindings until they adopt the new API level.
In addition to providing stability, this lets end-developers migrate to new APIs
one component at a time, since target API levels are specified per component.

[Figure 1](#fig-1) illustrates a breaking API change. Without versioning, it
breaks an application and leads to a revert. With versioning, the application
simply stays pinned to the old API level. Of course, the same problems will
arise when attempting to bump the application's pinned API level from 12 to 13.
But these can be fixed asynchronously, without reverting the original change in
Fuchsia and grinding the project to a halt.


![API evolution diagram with text description above]
(resources/0083_fidl_versioning/fig_1.png){:#fig-1}

**Figure 1: API evolution before (left) and after (right) FIDL Versioning**

## Terminology {#terminology}

The terms _API level_ and _ABI revision_ are defined in [RFC-0002: Platform
Versioning][rfc-0002], with the following change:

> _Amendment to [RFC-0002]_. A Fuchsia _API level_ is an unsigned, 63-bit
> integer. In other words, it is 64 bits but the high bit must be zero.

A _FIDL element_ is a discrete part of a FIDL library that affects the generated
bindings. This includes:

* FIDL libraries themselves
* Constant, enum, bits, struct, table, union, protocol, and service declarations
* Aliases and new types (from [RFC-0052: Type Aliasing and New Types][rfc-0052])
* Members of enums, bits, structs, tables, unions, and services (including
  `reserved` members of tables and unions)
* Methods and `compose` stanzas in protocols
* Request and response parameters in methods

Note: When one protocol composes another, we consider the latter's _original_
methods and the former's _composed methods_ to be distinct FIDL elements. This
is counter to the [compositional model], which considers them identical.

A _FIDL property_ is a modifiable aspect of a FIDL element that is not itself a
distinct element. This includes:

* [Attributes][attrs]
* The modifiers `strict`, `flexible`, and `resource`
* Values of constants, enum members, and bits members
* Default values of struct members
* Type constraints (on members, parameters, and type aliases)
* Method kinds (one-way, two-way, event)
* Method error syntax (its presence and type)

This leaves only a few things that are neither FIDL elements nor properties:

* Individual `.fidl` files
* Imports of other FIDL libraries
* FIDL-only `using` aliases, which [RFC-0052] removes from the language
* Experimental [`resource_definition`] declarations
* Comments, including documentation comments

## Design

The design described in this document provides a general-purpose facility for
versioning FIDL elements. Its primary use case is to version FIDL libraries in
the Fuchsia Platform by API level.

### Scope {#scope}

This design introduces versioning as a concept in the FIDL language, giving a
temporal dimension to FIDL libraries. It specifies the syntax and semantics of
versioning attributes, including how they interact with other aspects of FIDL
such as parent-child and use-define relationships. It establishes how versions
are provided as input when generating FIDL bindings.

This design does _not_ propose a package manager for FIDL. Topics such as
version resolution algorithms, package distribution, and dependency conflicts
are out of scope. That being said, a system addressing those problems should be
able to reuse the tools provided by this design.

This proposal does not address runtime behavior: it focuses on API, not on ABI.
The topic of _protocol evolution_ is therefore out of scope. This includes
questions such as, "How can a FIDL server support multiple ABI revisions?" There
are a variety of protocol evolution strategies FIDL and Component Manager could
adopt in the future. This proposal paves the way to protocol evolution by
introducing the concept of versions in FIDL, but it goes no further than this.

The ability to represent a transition under this design has no bearing on
whether that transition is safe or compatible. On the contrary, a versioned FIDL
library can represent almost any sequence of syntactically valid changes.

### Formalism {#formalism}

A _platform identifier_ is a label that gives context to versions. Platform
identifiers must be a valid FIDL library name element, i.e. as of this writing
match the regex `[a-z][a-z0-9_]*`.

A _version identifier_ is an unsigned 64-bit integer between 1 and 2^63-1
(inclusive) or equal to 2^64-1. The latter version identifier is known as `HEAD`
and is treated specially.

> *Amendment (Oct 2022).* To support [legacy methods](#legacy), we instead use
> 2^64-2 for `HEAD` and 2^64-1 for `LEGACY`.

Version identifiers are totally ordered by an "is newer than" relationship.
Version _X_ is newer than version _Y_ when _X_ > _Y_.

The _availability_ of a FIDL element with respect to a platform refers to the
version when the element was _introduced_, and optionally the versions when it
was _deprecated_ and _removed_. Deprecation and removal must be newer than
introduction.[^1] If both are supplied, removal must be newer than deprecation.

A FIDL element is _versioned under_ a platform if it has an availability with
respect to that platform. It is _versioned_ if versioned under any platform.

A FIDL element is _available_ with respect to a platform version if the version
is newer than or equal to the element's introduction, but older than its
deprecation and removal (if any). It is _deprecated_ if the version is newer
than or equal to the element's deprecation, but older than its removal (if any).
It is _present_ if it is available or deprecated. Otherwise, it is _absent_.

A _version selection_ is an assignment of versions to a set of platforms. For
example, one could select version 2 of `red` and version `HEAD` of `blue`.

A FIDL element is _available_ with respect to a version selection if it is
available with respect to all platforms. It is _deprecated_ if it is present
with respect to all platforms, and deprecated with respect to one or more
platforms. Otherwise, it is _absent_.

Note: The primary use case for this design is to version FIDL libraries in the
Fuchsia Platform. In this case, all libraries are versioned with respect to the
`fuchsia` platform identifier, and version identifiers correspond to API levels.

### Syntax

An _availability attribute_ has the following form,[^2] inspired by [Swift's
`available` attribute][swift-attr]:

    @available(added=<V>, deprecated=<V>, removed=<V>)

Each `<V>` is a version identifier. The `added`, `deprecated`, and `removed`
fields denote the introduction, deprecation, and removal of the element,
respectively. They are all optional, but at least one must be supplied.

On libraries, the `added` field must be provided (`deprecated` and `removed` are
optional). There is also an optional field `platform` specifying a platform
identifier. All version identifiers in the library refer to versions of this
platform. For example:

    @available(platform="red", added=2)
    library colors.red.auth;

When omitted, it defaults to the first component of the library name:

    @available(added=HEAD)  // implies platform="blue"
    library blue.auth;

With the `deprecated` field, an additional `note` field can be given for
inclusion in warning messages. For example:

    @available(added=12, deprecated=34, note="Use X instead")

The availability attribute makes the `[Deprecated]` attribute from [RFC-0058:
Introduce a `[Deprecated]` Attribute][rfc-0058] obsolete.

### Versioning elements

FIDL elements are versioned using availability attributes. Each FIDL element can
have at most one availability attribute, and this can only be done in versioned
libraries. In other words, if any FIDL element in a library is annotated, then
the library must be annotated as well.

Each file in a FIDL library has its own library declaration, but they all
represent the same FIDL element: the library. This is consistent with the [FIDL
style guide][style]:

> The division of a library into files has no technical impact on consumers of
> the library. ... Divide libraries into files to maximize readability.

Therefore, only one library declaration in a library can have an availability
attribute. Doc comments are restricted in the same way, so it makes sense to
choose the same file to specify the library's availability and its doc comment.

### Versioning properties {#versioning-properties}

FIDL properties cannot be versioned directly. To change a property, you must
_swap_ the element it belongs to. This means duplicating the element, removing
the old copy and introducing the new copy at the same version. For example, to
change a string bound at version 12:

    @available(removed=12)
    string:50 info;
    @available(added=12)
    string:100 info;

Or to change an enum from `strict` to `flexible`:

    @available(removed=12)
    strict enum Color { ... };
    @available(added=12)
    flexible enum Color { ... };

All FIDL elements except libraries can be swapped. Naming conflicts do not arise
because the availabilities do not overlap.

This proposal does not preclude a future syntax for applying availability
attributes directly to FIDL properties. If such a syntax were introduced, it
could only support `added` and `removed`, as there is no interpretation for
`deprecated` that makes sense across all FIDL properties.

### Inheritance

FIDL elements form directed acyclic graphs, with child elements inheriting
their availability from parent elements.

Top level declarations inherit from the library. Members of enums, bits,
structs, tables, unions, and services inherit from the enclosing declaration.
Request/response parameters inherit from their method. Methods and `compose`
stanzas inherit from the enclosing protocol. Composed methods inherit from the
original method and the `compose` stanza. When protocol composition is not used,
the graph is a tree.

If a child element has an availability attribute, it overrides the inherited
availability. In doing so, it must be neither redundant nor contradictory:
introduction versions can only be made newer, and deprecation and removal
versions can only be made older.

For a composed method, if both parents are versioned under the same platform,
its availability is the intersection of its parents' availabilities (newest
introduction, oldest deprecation, and oldest removal). If they are versioned
under different platforms, the composed method inherits two separate
availabilities. In this case, the [definition](#formalism) of "available with
respect to a version selection" becomes relevant. In both cases, the deprecation
`note` is combined from both parents.

Here is the general case of composition within a platform:

    library foo;
    protocol Def { @available(added=A, deprecated=B, removed=C) Go(); };
    protocol Use { @available(added=D, deprecated=E, removed=F) compose Def; };

The original method `foo/Def.Go` is introduced at `A`, deprecated at `B`, and
removed at `C`. The composed method `foo/Use.Go` is introduced at `max(A,D)`,
deprecated at `min(B,E)`, and removed at `min(C,F)`. This means all composed
methods are bound by the `compose` stanza's availability, but some can have a
narrower availability if `Def` introduces them after the `compose` introduction,
or deprecates/removes them before the `compose` deprecation/removal.

### Use validation {#use-validation}

Certain FIDL elements are related in that one _uses_ the other. A
struct/table/union member or request/response parameter uses a FIDL element if
the element occurs in its type; a method, if the element occurs in its error
type; a const or enum/bits member, if the element occurs in its value; a struct
member, if the element occurs in its default value. Some examples:

    const uint32 X = 10;
    const uint32 Y = X;  // Y uses X

    table Entry {};
    protocol Device {};
    resource struct Info {
        vector<Entry>:X entries;  // entries uses Entry and X
        request<Device> req;      // req uses Device
        uint32 val = Y;           // val uses Y
        Info? next;               // next uses Info
    };

Given a version selection, fidlc produces an error if:

* a present element uses an absent element; or
* an available element uses a deprecated element.

### Lifecycle semantics

Given a version selection, if a FIDL element is available, it is emitted as
usual. If it is deprecated, we [denote this in the JSON IR](#json-ir), and the
behavior in bindings is as described in [RFC-0058: Introduce a `[Deprecated]`
Attribute][rfc-0058]. If it is absent, we omit it from the JSON IR.

If a FIDL element is not [used](#use-validation) by any other, annotating it
with `@available(removed=<N>)` is equivalent to deleting it from the `.fidl`
file, except that using the `removed` attribute maintains historical accuracy,
whereas deleting the element does not. This provides a way to avoid `.fidl`
files becoming bloated and unreadable as changes accumulate.

### Purpose of `HEAD`

The `HEAD` version identifier represents the bleeding edge of development.
Clients are free to program against `HEAD` bindings, but they should not expect
them to be stable. For example, suppose you download `red.fidl`, where the
highest version identifiers used in annotations are 12 and `HEAD`. If you
download a newer copy of `red.fidl`, it is reasonable to expect the API at
version 12 to be identical, i.e. that the authors have not altered history. But
the `HEAD` API might be completely different.

This feature provides continuity when adopting FIDL Versioning. To depend on the
`HEAD` bindings of a versioned library is the same as depending on the bindings
of an unversioned library.

It also makes FIDL changes easier in collaborative projects. When authoring a
CL, looking up the current version is tedious and race-prone, especially if it
changes during code review. Instead, contributors can simply use `HEAD`, and
project owners can replace it with a specific version later.

### Legacy support {#legacy}

> *Amendment (Oct 2022).* This section was added after the RFC was accepted.

When an API is removed using `@available(removed=<N>)`, it no longer appears in
the generated bindings for versions _N_ and above. This makes it hard to build a
Fuchsia system image that supports multiple API levels. If the system image is
built against _N_-1 bindings, it cannot provide implementations for methods
added at _N_. If it is built against _N_ bindings, it cannot provide
implementations for methods removed at _N_.

To solve this problem, we introduce a new version called `LEGACY` that acts the
same as `HEAD` but also includes legacy methods. A _legacy method_ is a method
marked `@available(removed=<N>, legacy=true)`. This uses a new boolean argument
called `legacy` which is false by default, and only allowed when `removed` is
present. For example:

    @available(added=1)
    library example;

    protocol Foo {
        @available(removed=2)  // implies legacy=false
        NotLegacy();

        @available(removed=2, legacy=true)
        Legacy();
    };

Here are the methods included in that example's bindings when targeting
different versions:

| Target version | Methods included      |
|:--------------:|:---------------------:|
| 1              | `NotLegacy`, `Legacy` |
| 2              |                       |
| `HEAD`         |                       |
| `LEGACY`       | `Legacy`              |

As a matter of policy, all methods in the Fuchsia platform should retain legacy
support when they are removed. Once the Fuchsia platform drops support for all
API levels before the method's removal, it is safe to remove `legacy=true` and
the method's implementation.

When the Fuchsia platform acts as a client instead of a server, legacy methods
allow the platform to continue calling the method for those targeting old API
levels. For those targeting newer API levels that do not expect it, the method
must be marked `flexible` so that the calls can be ignored. See [RFC-0138:
Handling unknown interactions][rfc-0138] for more details.

The `legacy` argument can be used on any FIDL element, not just on methods. For
example, if you are removing a type along with the method that uses it, that
type must be marked `legacy=true` as well. This is just a consequence of [use
validation](#use-validation), not a new rule.

As another example, consider a table used in a request. When removing one of
its fields, you might wish to use `legacy=true` so that the server can continue
supporting clients that set the field. On the other hand, if ignoring the field
is sufficient to preserve ABI, there is no need for legacy support. Similarly,
for a table used in a response, it is only necessary to use `legacy=true` when
removing a field if setting that field is required to preserve ABI for old
clients.

Legacy support should never be used when [swapping](#versioning-properties) an
element because the availabilities represent change, not removal. If you were to
do so, it would cause an error:

    protocol Foo {
        @available(removed=2, legacy=true)
        Bar();

        @available(added=2)
        Bar();
    }

Since the first `Bar` gets added back at `LEGACY`, and the second `Bar` is never
removed, they both exist at `LEGACY` and fidlc will emit an error like it
already does for same-named elements with overlapping availabilities.

### JSON IR {#json-ir}

To represent deprecation in the IR, we add two fields:

    deprecated: <bool>,          // required
    deprecation_note: <string>,  // optional

These are added to the following [JSON IR Schema][schema] definitions:

    #/definitions/bits
    #/definitions/bits-member
    #/definitions/const
    #/definitions/enum
    #/definitions/enum-member
    #/definitions/interface
    #/definitions/interface-method
    #/definitions/service
    #/definitions/service-member
    #/definitions/struct
    #/definitions/struct-member
    #/definitions/table
    #/definitions/table-member
    #/definitions/union
    #/definitions/union-member
    #/definitions/type_alias

Note that the IR does not represent the deprecation of a library. It still has
an effect via inheritance, as well as warnings described in the next section.

### Command-line interface {#command-line-interface}

To specify a version selection, fidlc will accept `--available <P>:<V>` where
`<P>` is a platform identifier and `<V>` is a version identifier. The flag can
be given multiple times for distinct platform identifiers. For example:

    fidlc --json out.json --available red:2 --available blue:HEAD
          --files red.fidl --files blue.fidl

If the version selection is missing a platform or has an unused platform
(compared to the platforms the given libraries are versioned under), fidlc
produces an error. If any library is deprecated/absent with respect to the
version selection, fidlc produces a warning/error.[^3]

## Policy {#policy}

> *Note (Oct 2022)*. This section sketched out an initial policy, and is no
> longer up to date. In particular, most new changes should be added at the
> current in-development API level, not at `HEAD`. See [FIDL API compatibility
> testing][api-compat-testing] for details.

FIDL Versioning makes it possible to evolve APIs without breaking applications,
but it does not guarantee it. To that end, we adopt the following policies,
specifically for the Fuchsia Platform:

* Annotate all new changes as occurring at `HEAD`.
* Do not alter the history of a FIDL library. The only exception is the process
  of _deleting old FIDL elements_, described below.
* Deprecate FIDL elements before removing them, except when
  [swapping](#versioning-properties) to change a property.
* When deprecating an element:
    * Use the `note` field to tell developers what to use instead.
    * Write a `# Deprecation` section in the doc comment giving a more detailed
      explanation and communicating the deprecation timeline.
* Be careful when changing [FIDL properties](#versioning-properties). For
  example, changing a type from [strict to flexible][strict-vs-flexible], or
  from [value to resource][value-vs-resource], can have a significant API
  impact. The API Council should judge these changes on a case-by-case basis.

These policies will be enforced as follows:

* All FIDL changes in the SDK will continue to require API Council approval.
* `fidl-lint` should check that deprecated elements have the `note` field set
  and a `# Deprecation` section in their doc comment.
* In the future, there should be a CQ job that enforces the other policies
  (altering history, deprecating before removal, and API/ABI incompatible
  changes) based on [FIDL API summaries][RFC-0076].

There are also two new processes whose details we defer to a later RFC:

* _Releasing new API levels._ This will likely happen on a fixed schedule, where
  some or all of the changes made since the last API level are released in a new
  API level by replacing occurrences of `HEAD` with the new level.
* _Deleting old FIDL elements._ Once enough time has passed, elements marked as
  removed can be deleted from `.fidl` files. An element can only be deleted if
  it is not referenced anywhere, so this process will likely involve deleting
  all elements older than a particular API level on a fixed schedule.

We can build tools to make both of these processes easier, using the same tree
visitor approach as fidl-format.

## Implementation

This design can mostly be implemented in fidlc. Parsing the `@available` syntax
is dependent on [another RFC][rfc-0086] to change FIDL's annotation syntax. The
semantics will likely be implemented behind an experimental flag at first.

When fidlc compiles a library, even though it produces JSON IR at a single
version, it should validate all possible versions simultaneously. It should
_not_ do so by generating and checking each version sequentially. Instead, it
should temporally decompose elements into (name, version range) tuples. This
process is analogous to [converting an NFA to a DFA][nfa-dfa]. For example:

    type MyTable = table {
        @available(added=2)
        1: name string;
        @available(added=HEAD)
        2: age uint32;
    };

This would decompose as follows (using pseudo syntax to demonstrate):

    type «MyTable, [0,1]»    = table {};
    type «MyTable, [2,HEAD)» = table { 1: name «string, [0,HEAD]»; }
    type «MyTable, HEAD»     = table { 1: name «string, [0,HEAD]»; 2: age «uint32, [0,HEAD]» };

Just before emitting IR, fidlc will prune the declarations to only include those
requested in the version selection.

> _Open problem_. The temporal decomposition approach is difficult to generalize
> when FIDL libraries versioned under different platforms are compiled together.
> Since this is not needed for our primary use case (versioning the Fuchsia
> Platform by API level), we can defer this problem and initially have fidlc
> only allow one `--available` flag.

The `HEAD` version identifier can be implemented as a context-specific constant,
similar to the [`MAX` constant][max-bound] that is allowed as a length
constraint on strings and vectors.

There is also some implementation work outside fidlc. First, fidldoc needs to
take versioning into account. For example, if an element is deprecated, the
documentation should indicate this prominently. It could also provide an API
level dropdown for viewing historical documentation. Second, fidlgen backends
needs to use the `"deprecated"` field in the JSON IR. For example, fidlgen_rust
could translate it to the `#[deprecated]` Rust attribute. See [RFC-0058:
Introduce a `[Deprecated]` Attribute][rfc-0058] for examples in other languages.

Before libraries in the SDK start using the annotations, we will need to add
`--available fuchsia:HEAD` to the GN templates for building FIDL bindings. This
is based on the assumption that all in-tree code will use `HEAD` bindings. When
we have a Platform Versioning proposal for C++, it might be necessary to build
in-tree code against other versions of FIDL bindings for testing.

In petal build systems, we will add `fuchsia_api_level` declarations and wire
them up to the `--available` flag. This will need to be coordinated with [fidlc
CLI changes](#command-line-interface) by at first accepting and ignoring the
`--available` flag before requiring it.

## Performance

This proposal has no impact on runtime performance. It affects build performance
to the extent that fidlc must do more work, but FIDL compilation has never been
a significant factor in Fuchsia build times.

## Security considerations

This proposal should have a positive impact on security, since versioning makes
it easier to migrate to new FIDL APIs with better security properties. This
should outweigh the negative impact of increasing the attack surface by having
to support old ABI revisions.

This proposal does not provide a mechanism for hiding ABIs based on an
application's target ABI revision, [as suggested in
RFC-0002][rfc-0002-security]. While this could enhance security, it would be
better designed as part of a comprehensive RFC on protocol evolution.

## Privacy considerations

This proposal should have a positive impact on privacy, since versioning makes
it easier to migrate to new FIDL APIs with better privacy properties.

## Testing

We currently test the FIDL toolchain with a combination of unit tests and golden
tests. Unit testing is mostly used for fidlc internals. Golden testing works by
compiling a suite of `.fidl` files and ensuring the resulting artifacts (JSON IR
and all bindings) are identical to previously vetted _golden files_.

FIDL Versioning will take a similar approach. It will use unit tests for small
pieces of logic in fidlc. For example, there will be a test ensuring that
compilation fails with an appropriate error message when a table member's
annotation says it was introduced before the table itself. It will also use
golden tests, but not by extending the existing golden testing framework.
Generating artifacts for libraries at every version would bloat the golden files
and make it hard to verify correctness. Instead, this project will have its own
set of `.fidl` files with golden _diffs_ of the JSON IR at each version. This
should make it easy to verify that versioning behaves as expected.

This won't make testing harder for implementers of platform APIs: tests will be
written against `HEAD`, the same way we currently don't run tests against FIDL
files from old git revisions. Nor does it make testing harder for SDK users:
they will test against a single version of the platform the same way they
currently test using a single release of the SDK.

## Documentation

The `@available` syntax will be documented in the [FIDL language
specification][language]. More documentation will be needed once there is a
process in place for releasing new API levels. For instance, we need to teach
library authors to use `@available(added=HEAD)` whenever adding a new API
element. With proper tooling, there should be no danger of forgetting to do
this. See the [Policy section](#policy) for details.

We also need to remove the `[Deprecated]` attribute from the [FIDL
attributes][attrs] page, since the availability attribute makes it obsolete.

The [FIDL source compatibility documentation][source-compat] should be updated
either to show FIDL changes using availability attributes, or to show how to
apply different kinds of FIDL diffs when using versioning. The documentation
should also describe how FIDL versioning interacts with transitions in general.
With versioning, changing a FIDL element is just as easy whether it is used
out-of-tree or not. This should reduce the need for some kinds of soft
transitions. But it does not eliminate all multi-step transitions; it just
removes the constraint of a single shared timeline when coordinating them.

## Drawbacks, alternatives, and unknowns

### What are the costs of implementing this proposal?

This proposal adds complexity to FIDL (the language) and to fidlc. It will make
it more tedious for library authors to make simple, safe changes, but easier for
them to make other types of changes (e.g. adding a member to a strict enum) with
confidence.

### Alternative: Use old SDKs

FIDL Versioning allows applications to stay pinned to old API levels while
continuing to roll new SDKs. But why not simply use an old SDK, rendering this
whole proposal unnecessary? There are a couple reasons:

* With an up-to-date SDK, users get the latest copies of everything else, such
  as the FIDL toolchain.
* Target API levels are specified per component. Using a different SDK for each
  component is complicated and impractical.

### Alternative: Changelog file

Instead of availability attributes, a separate changelog could record the
history of a FIDL library. One approach would be a set of textual diffs going
back from the each `.fidl` file to its original. This would simplify many
things, such as the difficulty of versioning FIDL properties. It would be
impractical to validate all versions of a library at once, as in the proposed
design. However, this is perhaps less necessary as this alternative eliminates
the problem of altering history by accident. But it would make it harder to
answer questions such as, "When was this element introduced?" It would
essentially duplicate git history, with the main difference being that history
is preserved when creating a downloadable SDK.

Textual diffs would be difficult to maintain if we make changes to FIDL syntax
in the future. Another variant of the changelog design would be defining a new
format to record changes to a FIDL library, and the version when they occurred.
This design is compatible with validating all versions at once, since fidlc
could read the changelog and produce a temporally decomposed AST the same as if
the information had come from attributes. However, it would require more
tooling. For example, we might want developers to edit `.fidl` files as they do
today, and run a tool to append to the changelog file before committing.

### Alternative: Per-library versions

An alternative design is to have a separate version for each FIDL library. This
would lead to a mapping from API levels to versions of every FIDL library in the
SDK. For example, API level 42 might represent `fuchsia.auth` v1.2,
`fuchsia.device` v5.7, and so on.

This approach has advantages for those concerned with an individual library.
Each version would be meaningful with respect to that library, and you could
estimate how much a library has evolved from its current version number. In
contrast, with per-platform versions there can be large gaps between versions
where something changes in a library.

But it raises lots of questions. Does having per-library versions mean that SDK
libraries must track the versions of other SDK libraries they depend on? Can SDK
consumers mix and match different versions of SDK libraries? Answering either
with "yes" adds a lot of complexity to FIDL Versioning. How do we know if a
given set of versions works together? How do we avoid compiling multiple copies
of the same library's bindings together? If the answer to both is "no", then
per-library versioning seems like needless indirection, making it appear that
versioning happens at the library level when it does not.

### Alternative: Asymmetric deprecation

RFC-0002 states in its [Lifecycle section][rfc-0002-lifecycle]:

> The element might be deprecated. Components that target older ABI revisions
> can still use the element when running on newer platform releases. However,
> end-developers that target a newer API level can no longer use the element.

It goes on to say [what this means for FIDL][rfc-0002-fidl]:

> When a protocol element (e.g., a field in a table or a message in protocol) is
> deprecated at a given API level, we would ideally like components that target
> that API level to be able to receive messages containing that protocol element
> but would like to prevent those components from sending messages that contain
> that protocol element.

FIDL Versioning departs from this behavior, and so it is included here as an
alternative. Preventing end-developers from using FIDL elements at a given API
level, while allowing code in the Fuchsia platform to support it at runtime, is
difficult. As stated, it relies on the incorrect assumption that the Fuchsia
Platform always acts as a server and the SDK consumer always acts as a client.
There are cases where the roles are reversed, or even ambiguous. We could
distinguish these by introducing attributes such as `@platform_implemented` and
`@user_implemented`. That helps with methods, but asymmetric behavior for types
and members of types (called _type elements_ below) is harder to solve.

One way to achieve asymmetric deprecation of type elements is to generate stubs
preventing their use. For example, a deprecated table field could appear in
bindings as a value of type `FidlDeprecated`, which generates typechecking
errors when used. Code in the Fuchsia Platform could continue supporting the
deprecated element via a new fidlgen flag `--allow-deprecated` that generates
code as if nothing is deprecated. But there are two problems with this approach.
First, it makes it difficult to eliminate use of deprecated elements within
Fuchsia, since they do not appear as deprecated. Second, it would be very easy
for end developers to use the flag as well. This negates the [desired
incentive][rfc-0002-dynamics]:

> This approach incentivizes developers to migrate away from deprecated
> interfaces by coupling access to new APIs to performing those migrations.
> Specifically, to gain access to a newly introduced API, the developer must
> change their target API level, which requires them to migrate off any
> interfaces that were deprecated in that API level.

Namely, with `--allow-deprecated`, developers can gain access to newly
introduced APIs without migrating off deprecated ones simply by using the flag.

Another approach for type elements would be to generate errors at runtime. For
example, if a table field is deprecated, bindings could produce an error during
encoding if the field is present (but leave decoding unchanged). However,
runtime behavior is [out of scope](#scope) for this proposal.

In summary, asymmetric deprecation is too subtle and complex to be included in
this proposal. These challenges could possibly be worked out in a future RFC if
the benefit of asymmetric deprecation is worth the complexity.

### Alternative: Full history IR

Under this proposal, version information only exists prior to the JSON IR. Once
the IR has been produced, we are working with a fixed version. This is
sufficient for generating bindings, but it is less useful for tools like fidldoc
which might want to use version information. Rather than these tools parsing
`.fidl` files, or inferring lifecycles by comparing the JSON IR at multiple
versions, an alternative would be to introduce a new mode of JSON IR that
includes all history and availability information. This is different from simply
including availability attributes in the IR because it would mean including
elements that are marked as removed at the latest version.

There are two problems with this alternative. First, it is undesirable that some
JSON IR files should have a slightly different schema and purpose than others.
It might be better to design an entirely new format, but this has downsides too.
Second, it is difficult to determine what this full history IR should look like
without an idea of the UI fidldoc should present. For example, what would it
show for a type whose `resource` modifier has been added and removed ten times?
This sort of question, and the representation used, would be better addressed in
a separate RFC.

## Prior art and references

This proposal is part of the overall plan laid out in [RFC-0002: Platform
Versioning][rfc-0002]. Reading that RFC is important to understanding the
context and motivation behind this one. Its [Prior art and
references][rfc-0002-prior-art] section focuses on other operating systems:
Android, Windows, and macOS/iOS. Here, we focus on other programming language
and IDLs, and their approaches to API versioning.

### Swift, Objective-C

Swift uses [`@available` attributes][swift-attr] much like the ones in this
proposal, and Objective-C uses similar [`API_AVAILABLE` attributes][objc-attr].
They are limited to a hardcoded list of Apple platforms such as macOS and iOS.
They can also use the `swift` platform to control availability based on the
Swift language version being used during compilation. Versions are specified as
one, two, or three numbers separated by dots, following [semver] semantics. Both
languages provide a similar syntax for checking platform versions at runtime.

### Rust

Rust annotates its standard library with [stability attributes][rust-attr]
`#[stable]`, `#[unstable]`, and `#[rustc_deprecated]`. Each unstable element is
linked to a GitHub issue, and can only be used by developers who opt in with the
corresponding `#[feature]` attribute. Stable attributes indicate the Rust
version at which the element was stabilized. However, this is just for
documentation; it does not control visibility.

### Protobuf, gRPC

[Protocol Buffers] do not provide tools for versioning. Instead, they place a
greater focus on forward and backward compatibility than FIDL does. For example,
there are no structs (only _messages_, which are like FIDL tables), no strict
types (all types have flexible behavior), and no exhaustive matching supported
on enumerations (as of proto3).

Google Cloud APIs use Protocol Buffers with [gRPC], and provide guidelines on
[versioning][gcp-versioning] and [compatibility][gcp-compatibility]. The
versioning strategy is based on conventions, not features built into the system.
APIs encode their major version number at the end of the protobuf package, and
include it in URI paths. In this way services can support multiple major
versions at once, and clients receive backwards-compatible updates in place,
i.e. without taking action to migrate.

[^1]: During implementation, this rule was relaxed to allow introduction and
    deprecation to coincide. This makes it possible to manually decompose FIDL
    declarations at any version boundary by [swapping](#versioning-properties).

[^2]: This document uses the syntax introduced by [RFC-0086: Updates to
    RFC-0050: FIDL Attributes Syntax][rfc-0086].

[^3]: During implementation, these rules were omitted to simplify integration
    with the build system. For the version selection, fidlc uses `HEAD` by
    default and ignores unused platforms. For library declarations, the
    availability has no effect apart from inheritance, so an absent library is
    equivalent to an empty one, and there is no warning for deprecation.

<!-- xrefs -->
[rfc-0002]: /docs/contribute/governance/rfcs/0002_platform_versioning.md
[rfc-0002-lifecycle]: /docs/contribute/governance/rfcs/0002_platform_versioning.md#lifecycle
[rfc-0002-fidl]: /docs/contribute/governance/rfcs/0002_platform_versioning.md#fidl
[rfc-0002-dynamics]: /docs/contribute/governance/rfcs/0002_platform_versioning.md#dynamics
[rfc-0002-security]: /docs/contribute/governance/rfcs/0002_platform_versioning.md#security-considerations
[rfc-0002-prior-art]: /docs/contribute/governance/rfcs/0002_platform_versioning.md#prior-art-and-references
[rfc-0076]: /docs/contribute/governance/rfcs/0076_fidl_api_summaries.md
[rfc-0058]: /docs/contribute/governance/rfcs/0058_deprecated_attribute.md
[rfc-0052]: /docs/contribute/governance/rfcs/0052_type_aliasing_named_types.md
[rfc-0086]: /docs/contribute/governance/rfcs/0086_rfc_0050_attributes.md
[rfc-0138]: /docs/contribute/governance/rfcs/0138_handling_unknown_interactions.md
[language]: /docs/reference/fidl/language/language.md
[attrs]: /docs/reference/fidl/language/attributes.md
[swift-attr]: https://docs.swift.org/swift-book/ReferenceManual/Attributes.html#ID583
[objc-attr]: https://developer.apple.com/documentation/swift/objective-c_and_c_code_customization/marking_api_availability_in_objective-c
[rust-attr]: https://rustc-dev-guide.rust-lang.org/stability.html
[schema]: /tools/fidl/fidlc/schema.json
[source-compat]: https://fuchsia.googlesource.com/fuchsia/+/f8c48f630f43202e3e5c6d7395459ed0fc5e4c6d/src/tests/fidl/source_compatibility
[`resource_definition`]: https://fuchsia.googlesource.com/fuchsia/+/f8c48f630f43202e3e5c6d7395459ed0fc5e4c6d/zircon/vdso/zx_common.fidl#93
[semver]: https://semver.org/
[Protocol Buffers]: https://developers.google.com/protocol-buffers
[gRPC]: https://grpc.io/
[gcp-versioning]: https://cloud.google.com/apis/design/versioning
[gcp-compatibility]: https://cloud.google.com/apis/design/compatibility
[nfa-dfa]: https://en.wikipedia.org/wiki/Powerset_construction
[compositional model]: /docs/contribute/governance/rfcs/0023_compositional_model_protocols.md#compositional_model
[strict-vs-flexible]: /docs/reference/fidl/language/language.md#strict-vs-flexible
[value-vs-resource]: /docs/reference/fidl/language/language.md#value-vs-resource
[max-bound]: https://fuchsia-review.googlesource.com/c/fuchsia/+/325737
[style]: /docs/development/languages/fidl/guides/style.md#files
[api-compat-testing]: /docs/development/testing/ctf/fidl_api_compatibility_testing.md
