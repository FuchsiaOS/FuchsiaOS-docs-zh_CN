# FIDL API Rubric

[TOC]

## General Advice

This section presents techniques, best practices, and general advice about
defining protocols in the [Fuchsia Interface Definition
Language](/docs/development/languages/fidl/README.md).

See also the [FIDL Style Guide](/docs/development/languages/fidl/guides/style.md).

### Protocols not objects

FIDL is a language for defining interprocess communication protocols.  Although
the syntax resembles a definition of an object-oriented interface, the design
considerations are more akin to network protocols than to object systems.  For
example, to design a high-quality protocol, you need to consider bandwidth,
latency, and flow control.  You should also consider that a protocol is more
than just a logical grouping of operations: a protocol also imposes a FIFO
ordering on requests and breaking a protocol into two smaller protocols means
that requests made on the two different protocols can be reordered with respect
to each other.

### Focus on the types

A good starting point for designing your FIDL protocol is to design the data
structures your protocol will use.  For example, a FIDL protocol about
networking would likely contain data structures for various types of IP
addresses and a FIDL protocol about graphics would likely contain data
structures for various geometric concepts.  You should be able to look at the
type names and have some intuition about the concepts the protocol manipulates
and how those concepts might be structured.

### Language neutrality

There are FIDL back ends for many different languages.  You should avoid
over-specializing your FIDL definitions for any particular target language.
Over time, your FIDL protocol is likely to be used by many different languages,
perhaps even some languages that are not even supported today.  FIDL is the
glue that holds the system together and lets Fuchsia support a wide variety of
languages and runtimes.  If you over-specialize for your favorite language, you
undermine that core value proposition.

### Ordinals

Protocols contain a number of methods.  Each method is automatically assigned a
unique 32 bit identifier, called an ordinal.  Servers use the ordinal value
to determine which protocol method should be dispatched.

The compiler determines the ordinal value by hashing the library, protocol, and
method name.  In rare cases, ordinals in the same protocol may collide.  If
this happens, you can use the `Selector` attribute to change the name of the
method the compiler uses for hashing.  The following example will use the method
name "C" instead of the method name "B" for calculating the hash:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="ordinals" %}
```

Selectors can also be used to maintain backwards compatibility with the wire
format in cases where developers wish to change the name of a method.

### Diagnostics

Sometimes there’s the need to expose information useful for debugging purposes or diagnostics of a
program. This data can take the form of statistics and metrics (like number of errors, calls,
sizes, etc), information useful for development, health state of a component or similar.

It's tempting to expose this information in test protocols or debug methods in a production
protocol. Fuchsia, however, provides a separate mechanism for exposing this type of information:
[Inspect][inspect] which should be taken into account to make the best decision on how to expose
this type of data. Inspect should be used instead of a FIDL method/protocol when there’s a need to
expose diagnostics information about a program that is useful for debugging in tests, used by dev
tools, or retrieved in the field through crash reports or metrics as long as no other program uses
that information to make runtime decisions.

FIDL should be used when runtime decisions will be made based on diagnostics information by other
programs. Inspect must never be used for communication between programs, it’s a best effort system
that must not be relied on to make decisions or alter behavior during runtime in production.

A heuristic to decide whether to use Insepct or FIDL could be:

1. Is the data used by other programs in production?
   - Yes: Use FIDL.

1. Is the data used by crash reports or in metrics?
   - Yes: Use Inspect.

1. Is the data used by tests or developer tools? Any chance that it will ever be used in production?
   - Yes: Use FIDL.
   - No: Use either.

## Library structure

Grouping of FIDL declarations into FIDL libraries has two specific goals:

* Help FIDL developers (those using the FIDL libraries) navigate the API
  surface.
* Provide structure to hierarchically scope FIDL declarations within FIDL
  libraries.

Carefully consider how you divide your type and protocol definitions into
libraries.  How you decompose these definitions into libraries has a large
effect on the consumers of these definitions because a FIDL library is the unit
of dependency and distribution for your types and protocols.

The FIDL compiler requires that the dependency graph between libraries is a DAG,
which means you cannot create a circular dependency across library boundaries.
However, you can create (some) circular dependencies within a library.

To decide whether to decompose a library into smaller libraries, consider the
following questions:

 * Do the customers for the library break down into separate roles that would
   want to use a subset of the functionality or declarations in the library?  If
   so, consider breaking the library into separate libraries that target each
   role.

 * Does the library correspond to an industry concept that has a generally
   understood structure?  If so, consider structuring your library to match the
   industry-standard structure.  For example, Bluetooth is organized into
   `fuchsia.bluetooth.le` and `fuchsia.bluetooth.gatt` to match how these
   concepts are generally understood in the industry.  Similarly,
   `fuchsia.net.http` corresponds to the industry-standard HTTP network
   protocol.

 * Do many other libraries depend upon the library?  If so, check whether those
   incoming dependencies really need to depend on the whole library or whether
   there is a "core" set of definitions that could be factored out of the
   library to receive the bulk of the incoming dependencies.

Ideally, we would produce a FIDL library structure for Fuchsia as a whole that
is a global optimum.  However, Conway's law states that "organizations which
design systems \[...\] are constrained to produce designs which are copies of
the communication structures of these organizations."  We should spend a
moderate amount of time fighting Conway's law.

### Access control is at protocol granularity

When deciding in which library to define a protocol, do not take into account
access control considerations. Generally, access control is expressed at
protocol granularity. The library in which a protocol is defined has no bearing
on access control, and cannot be used to determine whether it can or cannot be
accessed.

As an example, a process may access `fuchsia.logger.LogSink`, or a process is
given a client end of the `fuchsia.media.StreamSource` protocol. However, FIDL
is not designed and cannot be used to express access to the `fuchsia.logger`
library, or prevent access the `fuchsia.ldsvc` library.

Note: Finer-grained access control is possible. You can further reduce the
granularity discussed to method level or further with dynamic access control
based on authentication schemes.

### The `fuchsia` namespace {#fuchsia-namespace}

FIDL libraries defined in the Platform Source Tree (i.e., defined in
[fuchsia.googlesource.com](https://fuchsia.googlesource.com)) must be in the
`fuchsia` top-level namespace (e.g., `fuchsia.ui`) unless one of the following
is true:

* The library defines the portions of the FIDL language itself or its
  conformance test suite, in which case the top-level namespace must be `fidl`.
* The library is only used for internal testing and is not included in the SDK
  or in production builds, in which case the top-level namespace must be `test`.

FIDL libraries in the top-level namespace `fuchsia` namespace are strongly
encouraged to have no more than four components, i.e. `fuchsia.<api-namespace>`,
`fuchsia.<api-namespace>.<name>` or `fuchsia.<api-namespace>.<name>.<subname>`.
Choose an appropriate `api-namespace`, possibly with the help of an [API Council
member][api-council-membership].

For instance, FIDL libraries defined in the Platform Source Tree for the purpose
of exposing hardware functionality to applications must be in the
`fuchsia.hardware` namespace.  For example, a protocol for exposing an ethernet
device might be named `fuchsia.hardware.ethernet.Device`.  Higher-level
functionality built on top of these FIDL protocols does not belong in the
`fuchsia.hardware` namespace. For example, it is more appropriate for network
protocols to be under `fuchsia.net` than `fuchsia.hardware`.

### Avoid nesting too deeply

Prefer library names with three components (e.g. `fuchsia.hardware.network`),
and avoid library names with more than four components (e.g.
`fuchsia.apps.foo.bar.baz`).  If you use more than four components, you should
have a specific reason for that choice.

### Library dependencies

It is preferable to introduce dependencies from libraries with more specific
names to libraries with less specific names.  For example, `fuchsia.foo.bar`
might depend on `fuchsia.foo`, but `fuchsia.foo` should not depend on
`fuchsia.foo.bar`. This pattern is better for extensibility because over time we
can add more libraries with more specific names but there are only a finite
number of libraries with less specific names.

### Visibility to importing libraries

To expand on the second goal of grouping of FIDL declarations into FIDL
libraries, we expect to evolve FIDL to provide visibility rules altering whether
elements may be used by importing libraries ("child libraries"), e.g `public` or
`private` modifiers.

The `internal` library component name is intended to be treated specially, and
indicates a local restriction of visibility rules. For instance, a public
declaration in the `fuchsia.net.dhcp.internal.foo` library might only be visible
to its parent `fuchsia.net.dhcp`, or its siblings e.g.
`fuchsia.net.dhcp.internal.bar`.

### Using multi-word library components

While library names with components that join multiple words (e.g.
`fuchsia.modular.storymodel`) are allowed, their use should be exceptional.
Library authors can resort to joining multiple words together if the library
name would violate nesting rules, or if neither word should take precedence over
the other when thinking hierarchically about placement of the library.

### Version strings

Should a library need to be versioned, a single version number should be
suffixed e.g. `fuchsia.io2` or `fuchsia.something.something4.` Version numbers
should not be multi-part, e.g. `fuchsia.io2.1` is not acceptable, and should
instead be `fuchsia.io3`. Any library component may be versioned, though it is
strongly discouraged to have multiple versioned components, e.g.
`fuchsia.hardware.cpu2.ctrl` but not `fuchsia.hardware.cpu2.ctrl4`.

Version numbers should only indicate a more recent version of a library, rather
than a materially different domain. As a counterexample, `fuchsia.input` library
is used for lower level device handling, while `fuchsia.ui.input{2,3}` is used
for input that's interacting with scenic and with software components that
render UIs. Focusing solely on versioning, it would have been clearer as
`fuchsia.ui.scenic.input` and `fuchsia.ui.scenic.input2` to distinguish from the
other domain that `fuchsia.input` serves.

## Types

As mentioned under "general advice," you should pay particular attention to the
types you used in your protocol definition.

### Be consistent

Use consistent types for the same concept.  For example, use a `uint32` or an
`int32` for a particular concept consistently throughout your library.  If you
create a `struct` for a concept, be consistent about using that struct to
represent the concept.

Ideally, types would be used consistently across library boundaries as well.
Check related libraries for similar concepts and be consistent with those
libraries.  If there are many concepts shared between libraries, consider
factoring the type definitions for those concepts into a common library.  For
example, `fuchsia.mem` and `fuchsia.math` contain many commonly used types for
representing memory and mathematical concepts, respectively.

### Prefer semantic types

Create structs to name commonly used concepts, even if those concepts could be
represented using primitives.  For example, an IPv4 address is an important
concept in the networking library and should be named using a struct even
through the data can be represented using a primitive:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="semantics" %}
```

In performance-critical target languages, structs are represented in line, which
reduces the cost of using structs to name important concepts.

### `zx.time` has a well-defined timebase

The [`zx.time`](/zircon/vdso/zx_common.fidl) type monotonically measures the
number of nanoseconds from a
[device-specific timebase](/docs/concepts/kernel/time/monotonic.md).
Uses of `zx.time` can assume this timebase, and it does not need to be spelled
out.

### Use anonymous types judiciously {#anonymous-types}

Anonymous types are very useful to describe an API more fluently. In particular,
anonymous types are well suited for cases where you have a priori knowledge that
a sub-element of a named type is inherently tied to that named type, and will
not be useful or meaningful when used outside of the context of the containing
named container.

Consider for instance a union variant, which aggregates together a few things.
It is exceedingly rare for the union variant to be used by itself, i.e. we know
a priori that the union variant has meaning only in the context of its specific
use. As a result, using anonymous types for union variants is appropriate and
recommended.

Ideally, types should both map one-to-one to key concepts of an API, and no two
types should have the same definition. Achieving both is however not always
possible, especially in cases where the naming of type — which introduces a
different concept[^named-types] — is meaningful beyond its use as an API surface
element. Consider for instance named identifiers `type EntityId = struct { id
uint64; };` and `type OtherEntityId = struct { id uint64; };` which represent
different concepts, yet have the same type definition except for their names.

Using anonymous types creates multiple types, all incompatible with each
other. As such, if multiple anonymous types are used to represent the same
concept, this will lead to an overly complex API, and prevent generic handling
in most target languages.

When using anonymous types, you must therefore avoid multiple anonymous types
representing the same concept. For instance, you must not use anonymous types in
cases where evolution of the API will likely lead to multiple anonymous types
representing the same concept.

[^named-types]: While the FIDL type system is a structural type system when it
    comes to ABI, i.e. names have no bearing, only the structure of types
    matters, the FIDL type system has named types semantics when it comes to
    API.

### Consider using Virtual Memory Objects (VMOs)

A Virtual Memory Object (VMO) is a kernel object that represents a contiguous
region of virtual memory along with a logical size. Use this type to transfer
memory in a FIDL message and use the `ZX_PROP_VMO_CONTENT_SIZE` property to track
the amount of data contained in the object.

Note: This rubric previously recommended using the `fuchsia.mem.Buffer` type for
this purpose. This recommendation is no longer applicable. New and updated APIs
should use VMOs directly instead of using `fuchsia.mem.Buffer`.

### Specify bounds for vector and string

All `vector` and `string` declarations should specify a length bound.
Declarations generally fall into one of two categories:

* There is a constraint inherent to the data. For example, a string containing
  a filesystem name component must not be longer than
  `fuchsia.io.MAX_FILENAME`.
* There is no constraint other than "as much as possible." In these cases, you
  should use the built-in constant `MAX`.

Whenever you use `MAX`, consider whether the receiver of the message would
really want to process arbitrarily long sequences or whether extremely long
sequences represent abuse.

Bear in mind that all declarations are implicitly bounded by the maximum message
length when sent over a `zx::channel`. If there really are use cases for
arbitrarily long sequences, simply using `MAX` might not address those use cases
because clients that attempt to provide extremely long sequences might hit the
maximum message length.

To address use cases with arbitrarily large sequences, consider breaking the
sequence up into multiple messages using one of the pagination patterns
discussed below or consider moving the data out of the message itself, for
example into a VMO.

### String encoding, string contents, and length bounds

FIDL `string`s are encoded in [UTF-8](https://en.wikipedia.org/wiki/UTF-8), a
variable-width encoding that uses 1, 2, 3, or 4 bytes per
[Unicode code point](http://unicode.org/glossary/#code_point).

Bindings enforce valid UTF-8 for strings, and strings are therefore not
appropriate for arbitrary binary data. See
[Should I use string or vector?](#should-i-use-string-or-vector).

Because the purpose of length bound declarations is to provide an easily
calculable upper bound on the total byte size of a FIDL message, `string` bounds
specify the maximum _number of bytes_ in the field. To be on the safe side, you
will generally want to budget <code>(4 bytes · <var>code points in
string</var>)</code>. (If you know for certain that the text only uses code
points in the single-byte ASCII range, as in the case of phone numbers or credit
card numbers, 1 byte per code point will be sufficient.)

How many code points are in a string? This question can be complicated to
answer, particularly for user-generated string contents, because there is not
necessarily a one-to-one correspondence between a Unicode code point and what
users might think of as "characters".

For example, the string

```none
á
```

is rendered as a single user-perceived "character", but actually consists of two
code points:

```
1. LATIN SMALL LETTER A (U+0061)
2. COMBINING ACUTE ACCENT (U+0301)
```

In Unicode terminology, this kind of user-perceived "character" is known as a
[grapheme cluster](https://unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries).

A single grapheme cluster can consist of arbitrarily many code points. Consider
this longer example:

```none
á🇨🇦b👮🏽‍♀️
```

If your system and fonts support it, you should see **four grapheme clusters**
above:

```
1. 'a' with acute accent
2. emoji of Canadian flag
3. 'b'
4. emoji of a female police officer with a medium skin tone
```

These four grapheme clusters are encoded as **ten code points**:

```
 1. LATIN SMALL LETTER A (U+0061)
 2. COMBINING ACUTE ACCENT (U+0301)
 3. REGIONAL INDICATOR SYMBOL LETTER C (U+1F1E8)
 4. REGIONAL INDICATOR SYMBOL LETTER A (U+1F1E6)
 5. LATIN SMALL LETTER B (U+0062)
 6. POLICE OFFICER (U+1F46E)
 7. EMOJI MODIFIER FITZPATRICK TYPE-4 (U+1F3FD)
 8. ZERO WIDTH JOINER (U+200D)
 9. FEMALE SIGN (U+2640)
10. VARIATION SELECTOR-16 (U+FE0F)
```

In UTF-8, this string takes up **28 bytes**.

From this example, it should be clear that if your application's UI displays a
text input box that allows _N_ arbitrary grapheme clusters (what users think of
as "characters"), and you plan to transport those user-entered strings over
FIDL, you will have to budget _some multiple_ of <code>4·<var>N</var></code> in
your FIDL `string` field.

What should that multiple be? It depends on your data. If you're dealing with a
fairly constrained use case (e.g. human names, postal addresses, credit card
numbers), you might be able to assume 1-2 code points per grapheme cluster. If
you're building a chat client where emoji use is rampant, 4-5 code points per
grapheme cluster might be safer. In any case, your input validation UI should
show clear visual feedback so that users aren't surprised if they run out of
room.

### Integer types

Select an integer type appropriate for your use case and be consistent about how
you use them.  If your value is best thought of as a byte of data, use `byte`.
If a negative value has no meaning, use an unsigned type.  As a rule of thumb if
you're unsure, use 32-bit values for small quantities and 64-bit values for
large ones.

### Avoid booleans if more states are possible

When adding a boolean field, consider using an enum instead if the field could
be extended to represent additional states in the future. For example a boolean
`is_gif` field might be better represented by

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="boolean-enum" %}

```

The enum may then be extended with `JPEG = 2` if required.

### How should I represent errors?

Select the appropriate error type for your use case and be consistent about how
you report errors.

Use the [`error` syntax](#error-syntax) to clearly document and convey a
possible erroneous return, and take advantage of tailored target language
bindings.

(The use of the [optional value with error
enum](#using-optional-value-with-error-enum) pattern is deprecated.)

#### Using the error syntax {#error-syntax}

Methods can take an optional `error <type>` specifier to indicate that they
return a value, or error out and produce `<type>`. Here is an example:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="errors" %}
```

When using this pattern, you can either use an `int32`, `uint32`, or an enum
thereof to represent the kind of error returned. In most cases, returning an
enum is the [preferred approach](#prefer-domain-specific-enum-for-errors).

It is preferred to have a single error type across all methods of a protocol.

#### Prefer domain-specific enum {#prefer-domain-specific-enum-for-errors}

Use a purpose built enum error type when you define and control the domain. For
example, define an enum when the protocol is purpose built, and conveying the
semantics of the error is the only design constraint. As noted in the
[enum](#enum) section, it is best to avoid using the value `0`.

In some cases, it may be appropriate to start by using an empty flexible enum:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="empty-error-enum" %}
```

Flexible enums have a [default unknown member][bindings-spec-unknown-enums]. An
empty flexible enum is therefore a typed placeholder providing affordances for
evolvability. When using this pattern, it is recommended to define a standalone
type to be reused by multiple methods within a protocol (or a library) [rather
than using an anonymous enum](#anonymous-types). Using an anonymous enum creates
multiple types, all incompatible with each other, which will lead to an overly
complex API, and prevent generic handling of errors in most target languages.

Use a domain-specific enum error type when you are following a well defined
specification (say HTTP error codes), and the enum is meant to be an ergonomic
way to represent the raw value dictated by the specification.

In particular, use the `zx.status` type for errors related to kernel objects or
IO. For example, `fuchsia.process` uses `zx.status` because the library is
largely concerned with manipulating kernel objects. As another example,
`fuchsia.io` uses `zx.status` extensively because the library is concerned with
IO.

#### Using optional value with error enum {#using-optional-value-with-error-enum}

Note: This pattern is deprecated.

In the past, there was a slight performance benefit to defining a method with
two returns, an optional value and an error code. See for instance:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="optional-error" %}

```

However, this pattern is now deprecated in favor of the [error
syntax](#error-syntax): the performance benefits which existed have been
obsoleted by [inlining small values in envelopes][rfc-0114], and low-level
support for unions is now prevalent.

#### Avoid messages and descriptions in errors

In some unusual situations, protocols may include a string description of the
error in addition to a `status` or enum value if the range of possible error
conditions is large and descriptive error messages are likely to be useful to
clients.  However, including a string invites difficulties.  For example,
clients might try to parse the string to understand what happened, which means
the exact format of the string becomes part of the protocol, which is
especially problematic when the strings are
[localized](#localizing-strings-and-error-messages).

*Security note:*
Similarly, reporting stack traces or exception messages to the client can
unintentionally leak privileged information.

### Localizing strings and error messages

If you are building a service that acts as a backend for a UI, use structured,
typed messages, and leave the rendering to the UI layer.

If all your messages are simple and unparameterized, use `enum`s for error
reporting and general UI strings. For more detailed messages, with parameters
such as names, numbers, and locations, use `table`s or `xunion`s, and pass
the parameters as string or numeric fields.

It may be tempting to generate messages (in English) in the service and provide
them to the UI as strings—the UI just receives a string and pops up a
notification or error dialog box.

However, this simpler approach has some serious drawbacks:

* Does your service know what locale (language and region) is being used in the
  UI? You would either have to pass the locale with each request (see
  [example][locale-passing-example]), or keep track of state for each connected
  client, in order to provide messages in the right language.
* Does your service's development environment have good support for
  localization? If you're writing in C++, you have easy access to the
  <abbr title="International Components for Unicode">ICU</abbr> library and
  `MessageFormat`, but if you're using Rust, library support is currently much
  more limited.
* Do any of your error messages need to include parameters that are known to
  the UI but not to the service?
* Does your service only serve a single UI implementation? Does the service
  know how much space the UI has to display a message?
* Are errors only displayed as text? You might also need error-specific alert icons,
  sound effects, or text-to-speech hints.
* Could the user change the display locale while the UI is still running? If
  this happens, pre-localized strings might be difficult to update to the new
  locale, particularly if they were the result of some non-idempotent operation.

Unless you are building a highly specialized service that is tightly coupled to
a _single UI implementation_, you probably shouldn't expose user-visible UI
strings in your FIDL service.

### Should I define a struct to encapsulate method parameters (or responses)?

Whenever you define a method, you need to decide whether  to pass parameters
individually or to encapsulate the parameters in a struct.  Making the best
choice involves balancing several factors.  Consider the questions below to help
guide your decision making:

 * Is there a meaningful encapsulation boundary?  If a group of parameters makes
   sense to pass around as a unit because they have some cohesion beyond this
   method, you might want to encapsulate those parameters in a struct.
   (Hopefully, you have already identified these cohesive groups when you
   started designing your protocol because you followed the "general advice"
   above and focused on the types early on.)

 * Would the struct be useful for anything beyond the method being called?  If
   not, consider passing the parameters separately.

 * Are you repeating the same groups of parameters in many methods?  If so,
   consider grouping those parameters into one or more structures.  You might
   also consider whether the repetition indicates that these parameters are
   cohesive because they represent some important concept in your protocol.

 * Are there a large number of parameters that are optional or otherwise are
   commonly given a default value?  If so, consider using use a struct to reduce
   boilerplate for callers.

 * Are there groups of parameters that are always null or non-null at the same
   time?  If so, consider grouping those parameters into a nullable struct to
   enforce that invariant in the protocol itself.  For example, the
   `FrobinateResult` struct defined above contains values that are always null
   at the same time when `error` is not `MyError.OK`.

### Should I use `string` or `bytes`?

In FIDL, `string` data must be valid UTF-8, which means strings can represent
sequences of Unicode code points but cannot represent arbitrary binary data.  In
contrast, `bytes` or `array<uint8>` can represent arbitrary binary data and do
not imply Unicode.

Use `string` for text data:

 * Use `string` to represent package names because package names are required to
   be valid UTF-8 strings (with certain excluded characters).

 * Use `string` to represent file names within packages because file names
   within packages are required to be valid UTF-8 strings (with certain excluded
   characters).

 * Use `string` to represent media codec names because media codec names are
   selected from a fixed vocabulary of valid UTF-8 strings.

 * Use `string` to represent HTTP methods because HTTP methods are comprised of
   a fixed selection of characters that are always valid UTF-8.

Use `bytes` or `array<uint8>` for small non-text data:

 * Use `bytes` for HTTP header fields because HTTP header fields do not
   specify an encoding and therefore cannot necessarily be represented in UTF-8.

 * Use `array<uint8>:6` for MAC addresses because MAC address are binary data.

 * Use `array<uint8>:16` for UUIDs because UUIDs are (almost!) arbitrary binary
   data.

Use shared-memory primitives for blobs:

 * Use `zx.handle:VMO` for images and (large) protobufs, when it makes
   sense to buffer the data completely.
 * Use `zx.handle:SOCKET` for audio and video streams because data may arrive over
   time, or when it makes sense to process data before completely written or
   available.

### Should I use `vector` or `array`?

A `vector` is a variable-length sequence that is represented out-of-line in the
wire format.  An `array` is a fixed-length sequence that is represented in-line
in the wire format.

Use `vector` for variable-length data:

 * Use `vector` for tags in log messages because log messages can have between
   zero and five tags.

Use `array` for fixed-length data:

 * Use `array` for MAC addresses because a MAC address is always six bytes long.

### Should I use a `struct` or a `table`?

Both structs and tables represent an object with multiple named fields. The
difference is that structs have a fixed layout in the wire format, which means
they *cannot* be modified without breaking binary compatibility. By contrast,
tables have a flexible layout in the wire format, which means fields *can* be
added to a table over time without breaking binary compatibility.

Use structs for performance-critical protocol elements or for protocol elements
that are very unlikely to change in the future. For example, use a struct to
represent a MAC address because the structure of a MAC address is very unlikely
to change in the future.

Use tables for protocol elements that are likely to change in the future.  For
example, use a table to represent metadata information about camera devices
because the fields in the metadata are likely to evolve over time.

### How should I represent constants?

There are three ways to represent constants, depending on the flavor of
constant you have:

1. Use `const` for special values, like **PI**, or **MAX_NAME_LEN**.
2. Use `enum` when the values are elements of a set, like the repeat
   mode of a media player: **OFF**, **SINGLE_TRACK**, or **ALL_TRACKS**.
3. Use `bits` for constants forming a group of flags, such as the capabilities
   of an interface: **WLAN**, **SYNTH**, and **LOOPBACK**.

#### const

Use a `const` when there is a value that you wish to use symbolically rather
than typing the value every time. The classical example is **PI** &mdash; it's
often coded as a `const` because it's convenient to not have to type
`3.141592653589` every time you want to use this value.

Alternatively, you may use a `const` when the value may change, but needs to
otherwise be used consistently throughout. A maximum number of characters that
can be supplied in a given field is a good example (e.g., **MAX_NAME_LEN**). By
using a `const`, you centralize the definition of that number, and thus don't
end up with different values throughout your code.

Another reason to choose `const` is that you can use it both to constrain a
message, and then later on in code. For example:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="constants" %}

```

You can then use the constant `MAX_BATCH_SIZE` in your code to assemble batches.

#### enum {#enum}

Use an enum if the set of enumerated values is bounded and controlled by the
Fuchsia project.  For example, the Fuchsia project defines the pointer event
input model and therefore controls the values enumerated by `PointerEventPhase`.

In some scenarios, you should use an enum even if the Fuchsia project itself
does not control the set of enumerated values if we can reasonably expect that
people who will want to register new values will submit a patch to the Fuchsia
source tree to register their values.  For example, texture formats need to be
understood by the Fuchsia graphics drivers, which means new texture formats can
be added by developers working on those drivers even if the set of texture
formats is controlled by the graphics hardware vendors.  As a counterexample,
do not use an enum to represent HTTP methods because we cannot reasonably expect
people who use novel HTTP methods to submit a patch to the Platform Source Tree.

For _a priori_ unbounded sets, a `string` might be a more appropriate choice if
you foresee wanting to extend the set dynamically.  For example, use a `string`
to represent media codec names because intermediaries might be able to do
something reasonable with a novel media codec name.

If the set of enumerated values is controlled by an external entity, use an
integer (of an appropriate size) or a `string`.  For example, use an integer (of
some size) to represent USB HID identifiers because the set of USB HID
identifiers is controlled by an industry consortium.  Similarly, use a `string`
to represent a MIME type because MIME types are controlled (at least in theory)
by an IANA registry.

We recommend that, where possible, developers avoid use of `0` as an enum value.
Because many target languages use `0` as the default value for integers, it can
be difficult to distinguish whether a `0` value was set intentionally, or
instead was set because it is the default. For instance, the
`fuchsia.module.StoryState` defines three values:  `RUNNING` with value `1`,
`STOPPING` with value `2`, and `STOPPED` with value `3`.

There are two cases where using the value `0` is appropriate:

  * The enum has a natural default, initial, or unknown state;
  * The enum defines an error code used in the
    [optional value with error enum](#using-optional-value-with-error-enum)
    pattern.

#### bits

If your protocol has a bitfield, represent its values using `bits` values (for
details, see [`RFC-0025`: "Bit Flags"][rfc-0025]).

For example:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="bits-hex" %}
```

This indicates that the `InfoFeatures` bit field is backed by an unsigned 32-bit
integer, and then goes on to define the three bits that are used.

You can also express the values in binary (as opposed to hex) using the `0b`
notation:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="bits-binary" %}
```

This is the same as the previous example.

### Should I use `resource`?

The FIDL compiler will enforce that any types that already contain a
[`resource`][resource-lang] are marked as such.

If a `flexible` type does not contain resources but is likely to in the future,
the `resource` modifier should be added pre-emptively to avoid a difficult
transition later on. This situation is rare: experience has shown that most
messages do not contain resources, and passing resources in protocols requires
care and upfront planning.

### Should I use `strict` or `flexible`?

Marking a type as [`flexible`][flexible-lang] makes it possible to handle data
that is unknown to the current FIDL schema, and is recommended for types that
may add or remove members in the future (e.g., configs, metadata, or errors). It
is always possible to [soft transition][flexible-transition] between `strict`
and `flexible` for an existing type. For types that accept the
`strict`/`flexible` modifier (`bits`, `enum`, and `union`), such a modifier
should always be specified rather than relying on defaults (e.g., always prefer
`flexible bits ...` to just `bits ...`).

Using `strict` or `flexible` does not have any significant performance impact.

### Handle rights

This section describes best practices for assigning rights constraints on
handles in FIDL.

See the FIDL [bindings spec](/docs/reference/fidl/language/bindings-spec.md)
or [RFC-0028](/docs/contribute/governance/rfcs/0028_handle_rights.md) for more
details on how rights are used in bindings.

#### Always specify rights on handles

All handles should specify rights to favor being explicit about the intended
use. This requirement forces an upfront decision of which rights are to be
passed, rather than basing them on observed behavior. Having explicit rights
also contributes to the auditability of API surfaces.

#### Use the minimum rights the recipient needs

When determining which rights to provide, prefer being minimal, i.e. the least
amount of rights needed to achieve the functionality sought. For instance, if
it is known that only `zx.rights.READ` and `zx.rights.WRITE` will be needed,
then only these two rights should be specified.

Do not add rights based on speculative needs. If a right needs to be added
at a future time, it can be added by starting with the source and adding it
to each location along the call path up until the final point of use.

#### Use `zx.rights.SAME_RIGHTS` sparingly

`zx.rights.SAME_RIGHTS` is well suited for a protocol that forwards a handle of
unknown rights, but for most cases a specific set of rights should be used
instead. Part of the motivation for this is that `zx.rights.SAME_RIGHTS` tells
bindings to skip rights checks, so it disables the security protection that
handle rights may offer. Additionally, `zx.rights.SAME_RIGHTS` makes the rights
set dynamic, meaning that a process might receive fewer or greater rights than
it actually needs.

It is worth noting that `zx.rights.SAME_RIGHTS` is not the same as the defaults
rights set for a type, e.g. `zx.DEFAULT_CHANNEL_RIGHTS`. While the former skips
rights checks, the latter requires all normal rights for a given object type to
be present.

## Good Design Patterns

This section describes several good design patterns that recur in many FIDL
protocols.

### Protocol request pipelining {#request-pipelining}

One of the best and most widely used design patterns is _protocol request
pipelining_.  Rather than returning a channel that supports a protocol, the
client sends the channel and requests the server to bind an implementation of
the protocol to that channel:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="pipelining-1" %}
```

This pattern is useful because the client does not need to wait for a round-trip
before starting to use the `Bar` protocol.  Instead, the client can queue
messages for `Bar` immediately.  Those messages will be buffered by the kernel
and processed eventually once an implementation of `Bar` binds to the protocol
request.  By contrast, if the server returns an instance of the `Bar` protocol,
the client needs to wait for the whole round-trip before queuing messages for
`Bar`.

If the request is likely to fail, consider extending this pattern with a reply
that describes whether the operation succeeded:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="pipelining-2" %}
```

To handle the failure case, the client waits for the reply and takes some other
action if the request failed.  Another approach is for the protocol to have an
event that the server sends at the start of the protocol:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="pipelining-3" %}
```

To handle the failure case, the client waits for the `OnReady` event and takes
some other action if the `Codec2` channel is closed before the event arrives.

However, if the request is likely to succeed, having either kind of success
signal can be harmful because the signal allows the client to distinguish
between different failure modes that often should be handled in the same way.
For example, the client should treat a service that fails immediately after
establishing a connection in the same way as a service that cannot be reached in
the first place.  In both situations, the service is unavailable and the client
should either generate an error or find another way to accomplish its task.

### Flow control

FIDL messages are buffered by the kernel.  If one endpoint produces more
messages than the other endpoint consumes, the messages will accumulate in the
kernel, taking up memory and making it more difficult for the system to recover.
Instead, well-designed protocols should throttle the production of messages to
match the rate at which those messages are consumed, a property known as _flow
control_.

Flow control is a broad, complex topic, and there are a number of effective
design patterns.  This section discusses some of the more popular flow control
patterns but is not exhaustive. The patterns are listed in descending order of
preference. If one of these patterns works well for a particular use case it
should be used. But if not, protocols are free to use alternative flow control
mechanisms that are not listed below.

#### Prefer pull to push

Without careful design, protocols in which the server pushes data to the client
often have poor flow control.  One approach to providing better flow control is
to have the client pull one or a range from the server.  Pull models have
built-in flow control since the client naturally limits the rate at which the
server produces data and avoids getting overwhelmed by messages pushed from the
server.

#### Delay responses using hanging gets {#hanging-get}

A simple way to implement a pull-based protocol is to "park a callback" with the
server using the _hanging get pattern_:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="hanging-gets" %}
```

In this pattern, the client sends a `WatchFoo` message but the server does not
reply until it has new information to send to the client. The client consumes
`foo` and immediately sends another hanging get.  The client and server each do
one unit of work per data item, which means neither gets ahead of the other.

The hanging get pattern works well when the set of data items being transferred
is bounded in size and the server-side state is simple, but does not work well
in situations where the client and server need to synchronize their work.

For example, a server might implement the hanging get pattern for some mutable
state `foo` using a "dirty" bit for each client. It would initialize this bit to
true, clear it on each `WatchFoo` response, and set it on each change of `foo`.
The server would only respond to a `WatchFoo` message when the dirty bit is set.

Note: When consuming an API that provides hanging gets, be mindful of dropping
pending requests, since the server implementation of the protocol is often
stateful and can't be notified of dropped requests. This is especially easy to
get wrong in Rust; see [Rust hanging get patterns][rust-hanging-get] for
examples.

#### Throttle push using acknowledgements

One approach to providing flow control in protocols that use push is the
_acknowledgment pattern_, in which the caller provides an acknowledgement
response that the caller uses for flow control.  For example, consider this
generic listener protocol:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="throttle-push" %}
```

The listener is expected to send an empty response message immediately upon
receiving the `OnBar` message.  The response does not convey any data to the
caller.  Instead, the response lets the caller observe the rate at which the
callee is consuming messages.  The caller should throttle the rate at which it
produces messages to match the rate at which the callee consumes them.  For
example, the caller might arrange for only one (or a fixed number) of messages
to be in flight (i.e., waiting for acknowledgement).

#### Push bounded data using events

In FIDL, servers can send clients unsolicited messages called _events_.
Protocols that use events need to provide particular attention to flow control
because the event mechanism itself does not provide any flow control.

A good use case for events is when at most one instance of the event will be
sent for the lifetime of the channel.  In this pattern, the protocol does not
need any flow control for the event:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="events-1" %}
```

Another good use case for events is when the client requests that the server
produce events and when the overall number of events produced by the server is
bounded.  This pattern is a more sophisticated version of the hanging get
pattern in which the server can respond to the "get" request a bounded number of
times (rather than just once):

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="events-2" %}
```

#### Throttle events using acknowledgements {#throttle-events-using-acknowledgements}

If there is no known bound on the number of events ahead of time, consider
having the client acknowledge the events by sending a message.  This pattern is
a more awkward version of the [acknowledgement
pattern](#throttle-events-using-acknowledgements) in which the roles of client
and server are switched.  As in the other pattern, the server should throttle
event production to match the rate at which the client consumes the events:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="ack-1" %}
```

One advantage to this pattern over the normal acknowledgement pattern is that
the client can more easily acknowledge multiple events with a single message
because the acknowledgement is disassociated from the event being acknowledged.
This pattern allows for more efficient batch processing by reducing the volume
of acknowledgement messages and works well for in-order processing of multiple
event types:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="ack-2" %}
```

Unlike throttle push using acknowledgements, this pattern does not express the
relationship between the request and the response in FIDL syntax and therefore
it is prone to misuse. Flow control will only work when clients correctly
implement sending of the notification message.

### Feed-forward dataflow

Some protocols have _feed-forward dataflow_, which avoids round-trip latency by
having data flow primarily in one direction, typically from client to server.
The protocol only synchronizes the two endpoints when necessary.  Feed-forward
dataflow also increases throughput because fewer total context switches are
required to perform a given task.

The key to feed-forward dataflow is to remove the need for clients to wait for
results from prior method calls before sending subsequent messages.  For
example, protocol request pipelining removes the need for the client to wait
for the server to reply with a protocol before the client can use the
protocol.  Similarly, client-assigned identifiers (see below) remove the need
for the client to wait for the server to assign identifiers for state held by
the server.

Typically, a feed-forward protocol will involve the client submitting a sequence
of one-way method calls without waiting for a response from the server.  After
submitting these messages, the client explicitly synchronizes with the server by
calling a method such as `Commit` or `Flush` that has a reply.  The reply might
be an empty message or might contain information about whether the submitted
sequence succeeded.  In more sophisticated protocols, the one-way messages are
represented as a union of command objects rather than individual method calls;
see the _command union pattern_ below.

Protocols that use feed-forward dataflow work well with optimistic error
handling strategies.  Rather than having the server reply to every method with a
status value, which encourages the client to wait for a round trip between each
message, instead include a status reply only if the method can fail for reasons
that are not under the control of the client.  If the client sends a message
that the client should have known was invalid (e.g., referencing an invalid
client-assigned identifier), signal the error by closing the connection.  If the
client sends a message the client could not have known was invalid, either
provide a response that signals success or failure (which requires the client to
synchronize) or remember the error and ignore subsequent dependent requests
until the client synchronizes and recovers from the error in some way.

Example:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="feed-forward" %}
```

### Privacy by design

The client and server in a protocol frequently have access to different sets of
sensitive data. Privacy or security problems can be caused by unintentionally
leaking more data than necessary over the protocol.

When designing a protocol pay particular attention to fields in your protocol
that:

* Contain personally identifiable information such as names, email addresses,
  or payment details.
* Are supplied by the user so potentially contain personal information. Examples
  include device names and comment fields.
* Act as a unique identifier that can be correlated across vendors, users,
  devices, or resets. Examples include serial numbers, MAC addresses, IP
  addresses and global account IDs.

These types of fields are reviewed thoroughly and the availability of protocols
that include them may be restricted. Make sure that your protocols don't contain
more information than is needed.

If a use case for an API requires personal or linkable data and other use cases
do not, consider using two different protocols so that access to the more
sensitive use case may be controlled separately.

Consider two hypothetical examples that illustrate privacy violations caused by
API design choices:

* [Example 1 - Serial numbers in a peripheral control API](#privacy-example-1)
* [Example 2 - Device names in a device setup API](#privacy-example-2)

#### Example 1 - Serial numbers in a peripheral control API {#privacy-example-1}

Consider a peripheral control API that includes the serial numbers of USB
peripherals. A serial number does not contain personal data but it is a very
stable identifier that is easy to correlate. Including the serial number in this
API leads to many privacy concerns:

* Any client with access to the API could correlate the different accounts
  using the same Fuchsia device.
* Any client with access to the API could correlate the different personae
  within an account.
* Different software vendors could collude to learn whether they are being used
  by the same users or on the same device.
* If a peripheral is moved between devices, any client with access to the API
  could correlate the set of devices and users the peripheral is shared between.
* If a peripheral is sold, clients with access to the API could correlate the
  old and new owner of the peripheral.
* Some manufacturers encode information in their serial numbers. This may let
  clients with access to the API deduce where or when the user purchased the
  peripheral.

In this example, the intent of the serial number is to allow clients to detect
when the same USB peripheral is reconnected. Meeting this intent does require a
stable identifier but it does not require a global identifier. Different clients
do not need to receive the same identifier, the same client does not need to
receive the same identifier across different Fuchsia devices, and the identifier
does not need to remain constant across factory reset events.

In this example, a good alternative is to send an identifier that is only
guaranteed to be stable for a single client on a single device. This identifier
could potentially be a hash of the peripheral's serial number, the Fuchsia
device identifier, and the moniker of the connection.

#### Example 2 - Device names in a device setup API {#privacy-example-2}

Consider a device setup API that includes the model of the phone that is used to
assist in the setup of a device. In most cases a phone's model string is set by
the OEM, but some phones report a user-supplied device name as their model. This
leads to many model strings containing the real names or pseudonyms of their
users. Therefore, this API risks associating a user across identities or across
devices. A rare or pre-release model string might reveal sensitive information
even when it isn't supplied by the user.

In some cases, it might be appropriate to use the model string but restrict
which clients can access the API. Alternatively, the API could use fields that
are never controlled by the user such as the manufacturer string. Another
alternative is to sanitize the model string by comparing it to an allowlist of
popular phone models and replacing rare model strings with a generic string.

### Client-assigned identifiers

Often a protocol will let a client manipulate multiple pieces of state held by
the server.  When designing an object system, the typical approach to this
problem is to create separate objects for each coherent piece of state held by
the server.  However, when designing a protocol, using separate objects for each
piece of state has several disadvantages.

Creating separate protocol instances for each logical object consumes kernel
resources because each instance requires a separate channel object.
Each instance maintains a separate FIFO queue of messages.  Using
separate instances for each logical object means that messages sent
to different objects can be reordered with respect to each other, leading to
out-of-order interactions between the client and the server.

The _client-assigned identifier pattern_ avoids these problems by having the
client assign `uint32` or `uint64` identifiers to objects retained by the server.
All the messages exchanged between the client and the server are funnelled
through a single protocol instance, which provides a consistent FIFO ordering
for the whole interaction.

Having the client (rather than the server) assign the identifiers allows for
feed-forward dataflow because the client can assign an identifier to an object
and then operate on that object immediately without waiting for the server to
reply with the object's identifier.  In this pattern, the identifiers are valid
only within the scope of the current connection, and typically the zero
identifier is reserved as a sentinel.  *Security note:* Clients should not use
addresses in their address space as their identifiers because these addresses
can leak the layout of their address space.

The client-assigned identifier pattern has some disadvantages.  For example,
clients are more difficult to author because clients need to manage their own
identifiers.  Developers commonly want to create a client library that provides
an object-oriented facade for the service to hide the complexity of managing
identifiers, which itself is an antipattern (see [client
libraries](#client-libraries) below).

A strong signal that you should create a separate protocol instance to
represent an object rather than using a client-assigned identifier is when you
want to use the kernel's object capability system to protect access to that
object.  For example, if you want a client to be able to interact with an object
but you do not want the client to be able to interact with other objects,
creating a separate protocol instance means you can use the underlying channel
as a capability that controls access to that object.

### Command union {#command-pattern}

In protocols that use feed-forward dataflow, the client often sends many one-way
messages to the server before sending a two-way synchronization message.  If the
protocol involves a particularly high volume of messages, the overhead for
sending a message can become noticeable.  In those situations, consider using
the _command union pattern_ to batch multiple commands into a single message.

In this pattern, the client sends a `vector` of commands rather than sending an
individual message for each command.  The vector contains a union of all the
possible commands, and the server uses the union tag as the selector for command
dispatch in addition to using the method ordinal number:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="command-union" %}
```

Typically the client buffers the commands locally in its address space and sends
them to the server in a batch.  The client should flush the batch to the server
before hitting the channel capacity limits in either bytes and handles.

For protocols with even higher message volumes, consider using a ring buffer in
a `zx.handle:VMO` for the data plane and an associated `zx.handle:FIFO` for the
control plane.  Such protocols place a higher implementation burden on the client
and the server but are appropriate when you need maximal performance.  For example,
the block device protocol uses this approach to optimize performance.

### Pagination

FIDL messages are typically sent over channels, which have a maximum message
size.  In many cases, the maximum message size is sufficient to transmit
reasonable amounts of data, but there are use cases for transmitting large (or
even unbounded) amounts of data.  One way to transmit a large or unbounded
amount of information is to use a _pagination pattern_.

#### Paginating writes

A simple approach to paginating writes to the server is to let the client send
data in multiple messages and then have a "finalize" method that causes the
server to process the sent data:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="paginate-write-1" %}
```

For example, this pattern is used by `fuchsia.process.Launcher` to let the
client send an arbitrary number of environment variables.

A more sophisticated version of this pattern creates a protocol that
represents the transaction, often called a _tear-off protocol_:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="paginate-write-2" %}
```

This approach is useful when the client might be performing many operations
concurrently and breaking the writes into separate messages loses atomicity.
Notice that `BarTransaction` does not need an `Abort` method.  The better
approach to aborting the transaction is for the client to close the
`BarTransaction` protocol.

#### Paginating reads

A simple approach to paginating reads from the server is to let the server send
multiple responses to a single request using events:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="paginate-read-1" %}
```

Depending on the domain-specific semantics, this pattern might also require a
second event that signals when the server is done sending data.  This approach
works well for simple cases but has a number of scaling problems.  For example,
the protocol lacks flow control and the client has no way to stop the server if
the client no longer needs additional data (short of closing the whole
protocol).

A more robust approach uses a tear-off protocol to create an iterator:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="paginate-read-2" %}
```

After calling `GetBars`, the client uses protocol request pipelining to queue
the first `GetNext` call immediately.  Thereafter, the client repeatedly calls
`GetNext` to read additional data from the server, bounding the number of
outstanding `GetNext` messages to provide flow control.  Notice that the
iterator need not require a "done" response because the server can reply with an
empty vector and then close the iterator when done.

Another approach to paginating reads is to use a token.  In this approach, the
server stores the iterator state on the client in the form of an opaque token,
and the client returns the token to the server with each partial read:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="paginate-read-3" %}
```

This pattern is especially attractive when the server can escrow all of its
pagination state to the client and therefore no longer need to maintain
pagination state at all.  The server should document whether the client can
persist the token and reuse it across instances of the protocol.  *Security
note:* In either case, the server must validate the token supplied by the client
to ensure that the client's access is limited to its own paginated results and
does not include results intended for another client.

### Eventpair correlation

When using client-assigned identifiers, clients identify objects held by the
server using identifiers that are meaningful only in the context of their own
connection to the server.  However, some use cases require correlating objects
across clients.  For example, in `fuchsia.ui.scenic`, clients largely interact
with nodes in the scene graph using client-assigned identifiers.  However,
importing a node from another process requires correlating the reference to that
node across process boundaries.

The _eventpair correlation pattern_ solves this problem using a feed-forward
dataflow by relying on the kernel to provide the necessary security.  First, the
client that wishes to export an object creates a `zx::eventpair` and sends one
of the entangled events to the server along with its client-assigned identifier
of the object.  The client then sends the other entangled event to the other
client, which forwards the event to the server with its own client-assigned
identifier for the now-shared object:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="eventpair" %}
```

To correlate the objects, the server calls `zx_object_get_info` with
`ZX_INFO_HANDLE_BASIC` and matches the `koid` and `related_koid` properties from
the entangled event objects.

### Eventpair cancellation

When using tear-off protocol transactions, the client can cancel long-running operations
by closing the client end of the protocol.  The server should listen for
`ZX_CHANNEL_PEER_CLOSED` and abort the transaction to avoid wasting resources.

There is a similar use case for operations that do not have a dedicated channel.
For example, the `fuchsia.net.http.Loader` protocol has a `Fetch` method that
initiates an HTTP request.  The server replies to the request with the HTTP
response once the HTTP transaction is complete, which might take a significant
amount of time.  The client has no obvious way to cancel the request short of
closing the entire `Loader` protocol, which might cancel many other outstanding
requests.

The _eventpair cancellation pattern_ solves this problem by having the client
include one of the entangled events from a `zx::eventpair` as a parameter to the
method.  The server then listens for `ZX_EVENTPAIR_PEER_CLOSED` and cancels the
operation when that signal is asserted.  Using a `zx::eventpair` is better than
using a `zx::event` or some other signal, because the `zx::eventpair` approach
implicitly handles the case where the client crashes or otherwise tears down.
The kernel generates `ZX_EVENTPAIR_PEER_CLOSED` when the entangled event
retained by the client is destroyed.

### Empty protocols

Sometimes an empty protocol can provide value.  For example, a method that
creates an object might also receive a `request<FooController>` parameter.  The
caller provides an implementation of this empty protocol:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="empty" %}
```

The `FooController` does not contain any methods for controlling the created
object, but the server can use the `ZX_CHANNEL_PEER_CLOSED` signal on the
protocol to trigger destruction of the object.  In the future, the protocol
could potentially be extended with methods for controlling the created object.

### Controlling settings-like data {#controlling-settings}

Often, servers will expose settings that the client can modify. Prefer using a
`table` to represent such settings. For instance, the `fuchsia.accessibility`
library defines:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="settings" %}
```
(Comments are omitted for readability.)

There are various ways to provide clients the ability to change these settings.

The **partial update** approach exposes an `Update` method taking a partial
settings value, and changes fields _only_ if they are present in the partial
value.

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="settings-partial" %}
```

The **replace** approach exposes a `Replace` method taking a complete
settings value, and changes the settings to the newly provided one.

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="settings-replace" %}
```

Things to avoid:

 * Avoid using the verb `Set` or `Override` for either the partial update or
   the replace approach since what semantics are offered will be ambiguous.

 * Avoid individual methods to update settings' fields such as
   `SetMagnificationEnabled`. Such individal methods are more burdensome to
   maintain, and callers rarely want to update a single value.

 * Avoid removing a setting with a magic value, like `-1`. Instead, remove a
   setting by the [absence of that settings field](#magic_values).

### Referring to union variants and table fields

It is often useful to refer to fields of types, such as referring to one or
multiple fields of a table or referring to a specific union variant.

Consider an API which provides metadata as a `table` with many fields. If this
metadata can grow to be quite large, it is often useful to have a mechanism for
the recipient to indicate to the sender which fields in this metadata will be
read, thus avoiding sending superfluous fields which will not be considered by
the recipients. In such cases, having a parallel `bits` whose members match
one-to-one with the fields of the `table` can be a strong foundation to build
your API:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="bits-fields-for-table" %}
```

Now, consider the [command union](#command-pattern). In a complex scenario, the
server may want the ability to describe the commands it supports. In such cases,
having a parallel `enum` whose members match one-to-one with the variants of the
`union` can be a strong foundation to build your API:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="enum-variants-for-union" %}
```

Note that while it might be tempting to use a `bits` value to represent the set
of commands, this leads to some more difficult choices down the line, If your
API evolves such that you need to refer to a specific command, having an `enum`
fits naturally. Should you have started with a `bits` value, you are now faced
with one of two bad choices:

1. Introduce a `enum` which means that there are now two ways to refer to
   fields, and possibly conversion issues in client code (to go from one
   representation to the other); or

1. Continue to use the `bits` with the restriction that only one bit be set at
   any given time, and now mapping back to which specific bit is set is
   cumbersome.

In summary, for `table`:

  * Name the `bits` by the name of the `table` along with the suffix `Fields`
    (plural). Each member value should be the bit at the ordinal index, i.e.
    `1 << (ordinal - 1)`.

  * Similarly to the advice for `union`, you want to match the flexibility
    between the `bits` and the `table`, i.e. since FIDL only supports flexible
    tables today, the `bits` must be `flexible`.

For `union`:

  * Name the `enum` listing all variants by the name of the `union` along with
    the suffix `Variant` (singular). Each member value should be the ordinal
    of the variant it describes.

  * Match the flexibility between the `union` and the `enum`, i.e. if the
    `union` is `strict` then the `enum` must also be `strict`.

## Antipatterns

This section describes several antipatterns: design patterns that often provide
negative value.  Learning to recognize these patterns is the first step towards
avoiding using them in the wrong ways.

### Client libraries: use with care

Ideally, clients interface with protocols defined in FIDL using
language-specific client libraries generated by the FIDL compiler.
While this approach lets Fuchsia provide high-quality support for a large
number of target languages, sometimes the protocol is too low-level to program directly.
In such cases, it's appropriate to provide a hand-written client library that
interfaces to the same underlying protocol, but is easier to use correctly.

For example, `fuchsia.io` has a client library, `libfdio.so`, which provides a
POSIX-like frontend to the protocol.  Clients that expect a POSIX-style
`open`/`close`/`read`/`write` interface can link against `libfdio.so` and speak
the `fuchsia.io` protocol with minimal modification.  This client library
provides value because the library adapts between an existing library interface
and the underlying FIDL protocol.

Another kind of client library that provides positive value is a framework.  A
framework is an extensive client library that provides a structure for a large
portion of the application.  Typically, a framework provides a significant
amount of abstraction over a diverse set of protocols.  For example, Flutter is
a framework that can be viewed as an extensive client library for the
`fuchsia.ui` protocols.

FIDL protocols should be fully documented regardless of whether the protocol has
an associated client library.  An independent group of software engineers should
be able to understand and correctly use the protocol directly given its
definition without need to reverse-engineer the client library.  When the
protocol has a client library, aspects of the protocol that are low-level and
subtle enough to motivate you to create a client library should be documented
clearly.

The main difficulty with client libraries is that they need to be maintained for
every target language, which tends to mean client libraries are missing (or
lower quality) for less popular languages.  Client libraries also tend to ossify
the underlying protocols because they cause every client to interact with the
server in exactly the same way.  The servers grow to expect this exact
interaction pattern and fail to work correctly when clients deviate from the
pattern used by the client library.

In order to include the client library in the Fuchsia SDK, we should provide
implementations of the library in at least two languages.

### Service hubs: use with care {#service_hubs}

A _service hub_ is a `Discoverable` protocol that simply lets you discover a
number of other protocols, typically with explicit names:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="service-hub-1" %}
```

Particularly if stateless, the `ServiceHub` protocol does not provide much
value over simply making the individual protocol services discoverable directly:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="service-hub-2" %}
```

Either way, the client can establish a connection to the enumerated services.
In the latter case, the client can discover the same services through the normal
mechanism used throughout the system to discover services.  Using the normal
mechanism lets the core platform apply appropriate policy to discovery.

However, service hubs can be useful in some situations.  For example, if the
protocol were stateful or was obtained through some process more elaborate than
normal service discovery, then the protocol could provide value by transferring
state to the obtained services.  As another example, if the methods for
obtaining the services take additional parameters, then the protocol could
provide value by taking those parameters into account when connecting to the
services.

### Overly object-oriented design: no

Some libraries create separate protocol instances for every logical object in
the protocol, but this approach has a number of disadvantages:

 * Message ordering between the different protocol instances is undefined.
   Messages sent over a single protocol are processed in FIFO order (in each
   direction), but messages sent over different channels race.  When the
   interaction between the client and the server is spread across many channels,
   there is a larger potential for bugs when messages are unexpectedly
   reordered.

 * Each protocol instance has a cost in terms of kernel resources, waiting
   queues, and scheduling.  Although Fuchsia is designed to scale to large
   numbers of channels, the costs add up over the whole system and creating a
   huge proliferation of objects to model every logical object in the system
   places a large burden on the system.

* Error handling and teardown is much more complicated because the number of
  error and teardown states grows exponentially with the number of protocol
  instances involved in the interaction.  When you use a single protocol
  instance, both the client and the server can cleanly shut down the interaction
  by closing the protocol.  With multiple protocol instances, the interaction
  can get into states where the interaction is partially shutdown or where the
  two parties have inconsistent views of the shutdown state.

 * Coordination across protocol boundaries is more complex than within a single
   protocol because multiple protocols need to allow
   for the possibility that different protocols will be used by different
   clients, who might not completely trust each other.

However, there are use cases for separating functionality into multiple
protocols:

 * Providing separate protocols can be beneficial for security because some
   clients might have access to only one of the protocols and thereby be
   restricted in their interactions with the server.

 * Separate protocols can also more easily be used from separate threads.  For
   example, one protocol might be bound to one thread and another protocol
   might be bound to another thread.

 * Clients and servers pay a (small) cost for each method in a protocol.
   Having one giant protocol that contains every possible method can be less
   efficient than having multiple smaller protocols if only a few of the
   smaller protocols are needed at a time.

 * Sometimes the state held by the server factors cleanly along method
   boundaries.  In those cases, consider factoring the protocol into smaller
   protocols along those same boundaries to provide separate protocols for
   interacting with separate state.

A good way to avoid over object-orientation is to use client-assigned
identifiers to model logical objects in the protocol.  That pattern lets clients
interact with a potentially large set of logical objects through a single
protocol.

### Specifying absence with magic values: no {#magic_values}

We typically want to instruct the server to set some state, but allow for the
removal of state too. The following uses magic values to instruct removal:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="absence-1" %}
```

However, FIDL offers optionality on many data types. Using optionality yields
a more idiomatic interface:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/api_rubric.test.fidl" region_tag="absence-2" %}
```


<!-- xrefs -->
[api-council]: /docs/contribute/governance/api_council.md
[api-council-membership]: /docs/contribute/governance/api_council.md#membership
[bindings-spec-unknown-enums]: /docs/reference/fidl/language/bindings-spec.md#unknown-enums
[inspect]: /docs/development/diagnostics/inspect/quickstart.md
[rfc-0025]: /docs/contribute/governance/rfcs/0025_bit_flags.md
[rfc-0114]: /docs/contribute/governance/rfcs/0114_fidl_envelope_inlining.md
[locale-passing-example]: /examples/intl/wisdom/
[rust-hanging-get]: /docs/development/languages/fidl/guides/rust-hanging-get.md
[resource-lang]: /docs/reference/fidl/language/language.md#value-vs-resource
[flexible-lang]: /docs/reference/fidl/language/language.md#strict-vs-flexible
[flexible-transition]: /docs/development/languages/fidl/guides/compatibility/README.md#strict-flexible
