# Go Rubric

[TOC]

This document lists conventions to follow when writing Go in the Fuchsia Source
Tree. These conventions are a combination of best practices, project
preferences, and some choices made for the sake of consistency.

The conventions described here can be considered as adding or amending common
best practices as detailed in [Effective Go] and [Go Code Review Comments].

## General

### Ordering of declarations in a file

How declarations are organized within a Go file varies considerably, especially
since it is frequent to have one large file with all-the-things, or many smaller
files with each a-thing. We provide a few rules of thumb here.

For ordering of top-level declarations (constants, interfaces, types along with
their associated methods), it is common to list by group of functionality, with
the following order in each group:

* Constants (which includes enumerations);
* Exported interfaces, types, and functions;
* Unexported interfaces, types, and functions in support of the exported types.

As an example, a group of functionality in fidlgen lib may be the handling of
names, while another group may be reading and denormalizing the JSON IR. Such
groupings are good candidates to become their own separate files as they grow.

For a type with methods, it is common to list in order:

* The type declaration (e.g. `type myImpl struct { ...`);
* The [type assertion(s)](#type-assertions), if any;
* Methods, with exported methods first, and unexported methods second;
* It is best to group methods implementing an interface if applicable, i.e. all
  methods for interface `Foo`, then all methods for interface `Bar`;
* Often, despite being exported, the `String()` method will be last.

For an enumeration, see how to [define an enum](#enum).

### Naming convention

Go is quite opinionated about how to name things. Some conventions:

* `var err error`
* `var buf bytes.Buffer`
* `var buf strings.Builder`
* `var mu sync.Mutex`
* `var wg sync.WaitGroup`
* `var ctx context.Context`
* `_, ok := someMap[someKey]`
* `_, ok := someValue.(someType)`

When defining methods, ensure that the name of the receiver is consistent (when
present). See also [pointers vs values](#pointers-vs-values).

### Avoid exporting

In Go, identifiers can be unexported or exported (the public/private terminology
is not used). An exported declaration starts with a capital letter
`MyImportantThing`, while an unexported declaration starts with a lowercase
letter `myLocalThing`.

Only export things you really need, and that you intend to reuse. Keep in mind
that exporting in a follow up change is relatively easy so prefer to not export
until necessary. This keeps the documentation cleaner.

If your type will be used by reflection, e.g. within templates or automatic
diffing à la [go-cmp], you can have an unexported identifier with exported
fields. Again, prefer unexported fields except in the situation where you are
forced to do otherwise.

There is nothing wrong with a local variable having the same name as a type. If
you would write `server := Server{...}` for an exported type, then `server :=
server{...}` is fine for an unexported type.

### Never use named return values

Semantics are confusing, and error prone. Even if it might be a good usage at
some point (rare, and doubtful), evolutions typically cause this not to be the
case anymore, which then introduces a diff that is larger than the actual
change.

You'll see advice saying that "it's fine to use named return values for this
special case", or "that special case is perfect for named return values". Our
rule is simpler, never use named return values.

(When defining interfaces, it is fine to name things. This is different from
named returns in implementations, it has no semantic implications.)

### Defining an enum {#enum}

The standard pattern to define an enumeration is:

```go
type myEnumType int

const (
    _ myEnumType = iota
    myFirstEnumValue
    mySecondEnumValue
)
```

The type can only be elided when using `iota`. With an explicit value, you must
repeat the type:

```go
const (
    _ myEnumType = iota
    myFirstEnumValue
    mySecondEnumValue
    // ...
    myCombinedValue myEnumType = 100
)
```

Additionally, if you want this enum to have a textual representation:

```go
var myEnumTypeStrings = map[myEnumType]string{
    myFirstEnumValue:  "myFirstEnumValue",
    mySecondEnumValue: "mySecondEnumValue",
}

func (val myEnumType) String() string {
    if fmt, ok := myEnumTypeStrings[val]; ok {
        return fmt
    }
    return fmt.Sprintf("myEnumType(%d)", val)
}
```

For instance [mdlint:
TokenKind](https://cs.opensource.google/search?q=%22type%20TokenKind%20int%22&ss=fuchsia%2Ffuchsia).
Using a `map` is preferred over using a `switch` since it is very common to
evolve the code to compute things off of the map, e.g. a `FromString`
constructor (inefficiently by searching, or by pre-computing a `stringsToMyEnum`
reverse map).

You should only use an enum member with value `0` if it is a natural default.
For more details, see the [enum FIDL API Rubric entry][enum-fidl-rubric].

Another option is to use [`cmd/stringer`] to generate enumerations. This generates
more efficient code, but requires a little more effort to maintain. You can use
this approach when the enumeration has stabilized.

Note: Switching to `stringer` as the default recommended approached is tracked
as [fxbug.dev/73539](https://fxbug.dev/73539).

### Sets

To represent a set, use map to an empty struct:

```go
allMembers := map[uint32]struct{}{
    5: {},
    8: {},
}
```

Note: It is common to see sets defined as maps to `bool` (it is what [Effective
Go] recommends). In Fuchsia, we have chosen to always use an empty struct due to
the small performance improvement (which comes at the cost of small added
syntax).

### Instantiating slices

If you're going to define the slice using a literal, then `[]T{ ... }` is the
way to go.

Otherwise use `var slice []T` to create an empty slice, i.e. do not use
`slice:= []T{}`.

Note that pre-allocating slices can bring meaningful performance improvements,
and may be preferred over a simpler syntax. See for instance [slices usage and
internals](https://go.dev/blog/slices-intro).

### Instantiating maps

If you're going to define the map using a literal, then `map[K]T{ ... }` is the
way to go.

Otherwise use `make(map[string]struct{})` to create an empty map, i.e. do not
use `myEmptyMap := map[K]T{}`.

### Non-normal returns

When an function needs to communicate a non-normal return, i.e. a 'failure', there are a few patterns:

1. `func doSomething(...) (data, bool)`, i.e. either return the data, or false.
2. `func doSomething(...) *data`, i.e. either return the data, or nil.
3. `func doSomething(...) (data, error)`, i.e. either return the data, or an
   error.
4. `func doSomething(...) dataWrapper`, i.e. return a wrapper which carries
   structured information about the result of the operation.

Giving hard and fast rules for when to use which-flavor-where is not possible,
but there are some general principles that apply.

Relying on `nil` vs present is error prone in Go when used in polymorphic
contexts: `nil` is not a unit, there are [many `nil` and they do not equal each
other](https://play.golang.org/p/RvxsIbQ3NLH)! Therefore, prefer returning an
extra `ok` boolean when uses of the method will likely be in polymorphic
contexts. Only use `nil` vs present as an indication when the method will be
used solely in monomorphic contexts. Or said another way, using pattern (2) is
simpler than (1) and a good replacement when all callers will be using the
function without looking at the returned value through the lens of an interface.

Returning an `error` indicates that something problematic occurred, the caller
is provided with an explanation for what happened. It is expected for the caller
to bubble up the error, possibly wrapping it along the way, or do some sort of
error handling and recovery. A key distinction from a method returning an `ok`
boolean (or `nil` data), is that when returning an `error` the method is
asserting that it knows enough of the context to classify what is happening as
erroneous.

For instance, a `LookupUser(user_id uint64)` would favor returning an ok boolean
if no user was found, but a `LookupCountryCode(code IsoCountryCode)` would favor
returning an `error` to identify that something is misconfigured if a country
cannot be lookup up (or an invalid country was requested).

Returning some sort of wrapper should be considered in cases where the result of
a method is complex, and requires structured data to describe. For instance, a
natural candidate to use a data wrapper is a validation API which traverses an
XML document, and returns a list of faulty paths, each with warnings or errors
attached to them.

### Static type assertions {#type-assertions}

Since Go uses structural subtyping, i.e. whether a type implements an interface
is determined by its structure (not by a declaration). It is easy to think a
type implements some interface, when in fact it does not. This results in
breakage at a distance, at the use site, and makes for confusing compiler
errors.

To remedy this, it is required to write type assertions for all interfaces a
type implements (yes, those in the standard library
[too](https://cs.opensource.google/fuchsia/fuchsia/+/main:tools/fidl/lib/fidlgen/formatter.go;drc=e67febe6dbde880bf9632fbe0540cf9251dcfd0a;l=55)):

```go
var _ MyInterface = (*myImplementation)(nil)
```

This creates a typed nil, whose assignment forces the compiler to check for type
conformance.

Often, we have many implementations which have to implement the same interface,
e.g. representing an abstract syntax tree with individual nodes all implementing
the Expression interface. In these cases, you should do all type assertions at
once thus also documenting all subtypes which are expected:

```go
var _ = []MyInterface{
    (*myImplementation)(nil),
    (*myOtherImplementation)(nil),
    ...
}
```

A rule of thumb about where these type assertions should be placed is as
follows:

* Prefer having a single type assertion below the implementation when each
  implementation stands on its own, e.g.
  [here](https://cs.opensource.google/fuchsia/fuchsia/+/main:tools/mdlint/rules/bad_lists.go;drc=b9496c021b59ccfed5eeeaeacee5fb9dc14005a7;l=26);
* Prefer a grouped type assertion below the interface when all implementations
  are meant to be used in concert (e.g. expression nodes of an `Expression`
  interface representing an AST), e.g.
  [here](https://cs.opensource.google/fuchsia/fuchsia/+/main:tools/fidl/lib/summarize/summary.go;drc=41966aa0f0d465f663b1b9928e5c57cf80fe38ef;l=35).

### Embedding

Embedding is a very powerful concept in Go. Make use of it. Read
[Embedding][Effective Go embedding] for an introduction.

When embedding an interface or a struct type, these should be listed first in
their enclosing interface or struct type, i.e. the embedded type(s) should
appear as the first field(s).

### Pointers vs Values {#pointers-vs-values}

For method receivers, read [Pointers vs. Values][Effective Go pointers_vs_values]
and [Receiver Type][Go Code Review Comments receiver-type].
tl;dr is to keep things consistent, but when in doubt, use a pointer receiver.
For a given type, always be consistent about how it is passed around, i.e.
always pass by value, always pass by reference, and this flows from whether
methods are defined on this type (with value or pointer receiver).

It is also worth noting that passing a struct by value, thinking that the caller
will not mutate it, is incorrect. You can easily hold maps, slices, or
references to objects, and therefore mutate those. So in Go it's an incorrect
association to think "pass by value is const".

Refer to [implementing interfaces](#implementing-interfaces) for specific advice
about method receivers.

### Implementing interfaces {#implementing-interfaces}

Generally implement interfaces using a pointer receiver; implementing an interface
using a value receiver causes that interface to be implemented by both value and
pointer, which complicates type assertions that attempt to enumerate possible
implementations of an interface. See for instance
[fxrev.dev/269371](https://fxrev.dev/269371).

There are some specific cases where implementing an interface using a value
receiver is appropriate:

* When the type is never used as a pointer. For example, custom sorting is often
  done by defining `type mySlice []myElement` and implementing
  [`sort.Interface`](https://golang.org/pkg/sort/#Interface) on `mySlice`. The
  type `*mySlice` would never be used because `[]myElement` is already a
  reference. Example
  [here](https://cs.opensource.google/fuchsia/fuchsia/+/main:tools/mdlint/core/reporter.go?q=%22type%20sortableMessages%20%5B%5Dmessage%22).
* When it is never expected to use a type assertion or type switch on values of
  the interface type. For example,
  [`Stringer`](https://golang.org/pkg/fmt/#Stringer) is often implemented on
  value types. It would be unusual for a function accepting `val Stringer` to
  switch on `val.(type)`.

When in doubt, always implement interfaces using a pointer receiver.

### Comments

Read [Commentary][Effective Go commentary], in particular:

> Doc comments work best as complete sentences, which allow a wide variety of
> automated presentations. The first sentence should be a one-sentence summary
> that starts with the name being declared.

```go
// Compile parses a regular expression and returns, if successful,
// a Regexp that can be used to match against text.
func Compile(str string) (*Regexp, error) {
```

Doc comments on types [may have a leading
article](https://github.com/golang/lint/blob/738671d3881b9731cc63024d5d88cf28db875626/lint.go#L832)
"A", "An", or "The". All documentation in the Go standard library follows this
practice, [e.g.](https://golang.org/pkg/bytes/#Buffer) `// A Buffer is a
variable-sized ...`.

For comments which are more than a sentence long, the style generally prefers
one summary sentence, then an empty line, then a paragraph providing further
details. See for instance the
[FileHeader](https://golang.org/pkg/archive/zip/#FileHeader) struct.

### Error wrapping {#error-wrapping}

When propagating errors using `fmt.Errorf`:

* Use `%s` to only include their string values;
* Use `%w` to allow callers to unwrap and observe wrapped errors; note `%w`
  makes those wrapped errors part of your API.

See [Working with Errors in Go
1.13](https://blog.golang.org/go1.13-errors), and more specifically [Whether
to Wrap](https://blog.golang.org/go1.13-errors#TOC_3.4.).

There are some specific cases where error propagation must be done in a way that
satisfies an API contract, e.g. often the case in a RDBMS driver where specific
error codes returned indicate situations callers may recover from. In such
cases, it is necessary to specifically wrap underlying errors rather than rely
on `fmt.Errorf`.

### `fmt` verbs

Avoid `%v` when possible, preferring specific fmt verbs that are supported by
the operand. This has the benefit of allowing `go vet` to check that the verb is
indeed supported by the operand.

In cases where the operand is a struct that doesn't implement `fmt.Stringer`,
`%v` is unlikely to produce a good result anyhow; `%+v` or `%#v` are likely much
better choices.

A common exception to this rule is an operand that may be `nil` at run-time -
`nil` values are well handled by `%v` but not all other verbs.

When quoting strings, use `%q` instead of explicitly calling `strconv.Quote`.

When propagating errors, see [error wrapping](#error-wrapping).

### GN targets

A typical `BUILD.gn` file for a Go tool will look something like this:

```gn
go_library("gopkg") {
  sources = [
    "main.go",
    "main_test.go",
  ]
}

go_binary("foo") {
  library = ":gopkg"
}

go_test("foo_test") {
  library = ":gopkg"
}
```

If you have nested packages (and [only in this
case](https://fuchsia-review.googlesource.com/c/fuchsia/+/406682/)), use
`name = "go.fuchsia.dev/fuchsia/<path>/..."` form in go_library to enable
recursive package sources:

```gn
go_library("gopkg") {
  name = "go.fuchsia.dev/fuchsia/tools/foo/..."
  sources = [
    "main.go",
    "subdir/bar.go",
    "extra/baz.go",
  ]
}
```

Note: We are moving away from allowing `...` in `go_library` altogether,
reserving this form for third-party code which we want to purposefully view as a
black box. However, it is frequent for code to be decomposed into multiple
packages, without individual packages being deemed "library quality", i.e. in a
shape to be reused at will. We plan to update the various GN templates to
reflect these subtleties, and offer a `go_private_library` template allowed to
be used solely by a single `go_binary` rule, and `go_third_party_library` which
would be the only allowing the use `...`. This is tracked as
[fxbug.dev/73815](https://fxbug.dev/73815).

## Testing

### Packages ending in _test

Usually a `_test.go` file is in the same package as the code it tests (e.g.
`package foo`), and it can access unexported declarations. [Go also allows
you](https://tip.golang.org/cmd/go/#hdr-Test_packages) to suffix the package
name with `_test` (e.g. `package foo_test`), in which case it is compiled as a
separate package, but linked and run with the main binary. This approach is
called external testing, as opposed to internal testing. Do not name a package
with the `_test` suffix unless you are writing external tests, see instead
[testing package](#testing-packages).

Prefer external tests when doing integration level testing, or to interact with
the exported only portion of a package under test.

Using packages ending in `_test` is also interesting to provide compiled example
code, which is copy pastable as-is with the package selector. For instance
[example
code](https://pkg.go.dev/github.com/maruel/panicparse/v2/stack#example-package-Simple)
and [its
source](https://github.com/maruel/panicparse/blob/master/stack/example_test.go#L161).

### Test utilities

Testing utilities are helpers used within a package, to help with testing. It is
'testing code' in that it lives in a file with a `_test.go` suffix. Place
testing utilities in a file named `testutils_test.go` file; this convention is
interpreted by the compiler such that this code is not included in non-test
binaries, which ensures it isn't used outside of tests.

### Testing packages {#testing-packages}

A testing package is a library whose focus is to make writing tests easier. It
is 'production code' — does not end in `_test.go` suffix — but intended only to
be used from test code.

The naming convention for testing packages is to use a "test" suffix with the
package name it is used to test. Examples in the standard library are
[httptest](https://golang.org/pkg/net/http/httptest/) and
[iotest](https://golang.org/pkg/testing/iotest/), in the Fuchsia tree
[fidlgentest](/tools/fidl/lib/fidlgentest/), and
[emulatortest](/tools/emulator/emulatortest).

## Additional resources

* [Effective Go]
* [Go Code Review Comments], view this as a supplement to [Effective Go]
* [Go FAQ](https://golang.org/doc/faq)
* [golint](https://pkg.go.dev/golang.org/x/lint/golint)
* [Go at Google: Language Design in the Service of Software
  Engineering](https://talks.golang.org/2012/splash.article)

<!-- link labels -->

[`cmd/stringer`]: https://pkg.go.dev/golang.org/x/tools/cmd/stringer
[Effective Go commentary]: https://golang.org/doc/effective_go#commentary
[Effective Go embedding]: https://golang.org/doc/effective_go#embedding
[Effective Go pointers_vs_values]: https://golang.org/doc/effective_go#pointers_vs_values
[Effective Go]: https://golang.org/doc/effective_go
[enum-fidl-rubric]: /docs/development/api/fidl.md#enum
[Go Code Review Comments receiver-type]: https://github.com/golang/go/wiki/CodeReviewComments#receiver-type
[Go Code Review Comments]: https://github.com/golang/go/wiki/CodeReviewComments
[go-cmp]: https://github.com/google/go-cmp
