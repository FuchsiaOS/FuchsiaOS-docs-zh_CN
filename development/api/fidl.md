# FIDL API Readability Rubric

[TOC]

## General Advice

This section contains some general advice about defining interfaces in the
[Fuchsia Interface Definition Language](https://fuchsia.googlesource.com/docs/+/master/development/languages/fidl/README.md).

### Protocols not objects

FIDL is a language for defining interprocess communication protocols.  Although
the syntax resembles a definition of an object-oriented interface, the design
considerations are more akin to network protocols than to object systems.  For
example, to design a high-quality interface, you need to consider bandwidth,
latency, and flow control.  You should also consider that an interface is more
than just a logical grouping of operations: an interface also imposes a FIFO
ordering on requests and breaking an interface into two smaller interfaces means
that requests made on the two different interfaces can be reordered with respect
to each other.

### Focus on the types

A good starting point for designing your FIDL protocol is to design the data
structures your protocol will use.  For example, a FIDL protocol about
networking would likely contain data structures for various types of IP
addresses and a FIDL protocol about graphics would likely contain data
structures for various geometric concepts.  You should be able to look at the
type names and have some intuition about the concepts the protocol manipulates
and how the interfaces for manipulating those concepts might be structured.

### Language neutrality

There are FIDL backends for many different languages.  You should avoid
over-specializing your FIDL definitions for any particular target language.
Over time, your FIDL protocol is likely to be used by many different languages,
perhaps even some languages that are not even supported today.  FIDL is the
glue that holds the system together and lets Fuchsia support a wide variety of
languages and runtimes.  If you over-specialize for your favorite language, you
undermine that core value proposition.

## Names

```
The Naming of Cats is a difficult matter,
It isn't just one of your holiday games;
  --- T.S. Eliot
```

Names defined in FIDL are used to generate identifiers in each target language.
Some languages attach semantic or conventional meaning to names of various
forms.  For example, in Go, whether the initial letter in an identifier is
capitalized controls the visibility of the identifier.  For this reason, many of
the language backends transform the names in your library to make them more
appropriate for their target language.  The naming rules in this section are a
balancing act between readability in the FIDL source, usability in each target
language, and consistency across target languages.

Avoid commonly reserved words, such as `goto`.  The language backends will
transform reserved words into non-reserved identifiers, but these transforms
reduce usability in those languages.  Avoiding commonly reserved words reduces
the frequency with which these transformations are applied.

While some FIDL keywords are also commonly reserved words in target languages,
(such as `struct` in C and C++), and should thus be avoided, other FIDL keywords,
particularly `request` and `handle`, are generally descriptive and can be
used as appropriate.

Names must not contain leading or trailing underscores.  Leading or trailing
underscores have semantic meaning in some languages (e.g., leading underscores
control visibility in Dart) and conventional meaning in other languages (e.g.,
trailing underscores are conventionally used for member variables in C++).
Additionally, the FIDL compiler uses leading and trailing underscores to munge
identifiers to avoid collisions.

### Libraries

Library names are period-separated lists of identifiers. Portions of the library
name other than the last are also referred to as namespaces.  Each component of
the name is in lowercase and must match the following regular expression:
`[a-z][a-z0-9]*`.

We use these restrictive rules because different target languages have different
restrictions on how they qualify namespaces, libraries, or packages.  We have
selected a conservative least common denominator in order for FIDL to work well
with our current set of target languages and with potential future target
languages.

Prefer functional names (e.g., `fuchsia.media`) over product or codenames (e.g.,
`fuchsia.amber` or `fuchsia.mozart`).  Product names are appropriate when the
product has some external existence beyond Fuchsia and when the interface is
specific to that product.  For example, `fuchsia.cobalt` is a better name for
the Cobalt interface than `fuchsia.metrics` because other metrics
implementations (e.g., Firebase) are unlikely to implement the same protocol.

FIDL libraries defined in the Fuchsia source tree (i.e., defined in
fuchsia.googlesource.com) must be in the `fuchsia` top-level namespace (e.g.,
`fuchsia.ui`) unless (a) the library defines portions of the FIDL language
itself or its conformance test suite, in which case the top-level namespace must
be `fidl`, or (b) the library is used only for internal testing and is not
included in the SDK or in production builds, in which case the top-level
namespace must be `test`.

Avoid library names with more than two dots (e.g., `fuchsia.foo.bar.baz`).
There are some cases when a third dot is appropriate, but those cases are rare.
If you use more than two dots, you should have a specific reason for that
choice.

Prefer to introduce dependencies from more libraries with more specific names to
libraries with less specific names rather than the reverse.  For example,
`fuchsia.foo.bar` might depend on `fuchsia.foo`, but `fuchsia.foo` should not
depend on `fuchsia.foo.bar`.  This pattern is better for extensibility because
over time we can add more libraries with more specific names but there are only
a finite number of libraries with less specific names.  Having libraries with
less specific names know about libraries with more specific names privileges the
current status quo relative to the future.

Library names must not contain the following components: `common`, `service`,
`util`, `base`, `f<letter>l`, `zx<word>`.  Avoid these (and other) meaningless
names.  If `fuchsia.foo.bar` and `fuchsia.foo.baz` share a number of concepts
that you wish to factor out into a separate library, consider defining those
concepts in `fuchsia.foo` rather than in `fuchsia.foo.common`.

### Top-level

Avoid repeating the names from the library name.  For example, in the
`fuchsia.process` library, an interface that launches process should be named
`Launcher` rather than `ProcessLauncher` because the name `process` already
appears in the library name.  In all target languages, top-level names are
scoped by the library name in some fashion.

### Primitive aliases

Primitive aliases must be named in `lower_snake_case`.

```
using vaddr = uint64;
```

Primitive aliases must not repeat names from the enclosing library.  In all
target languages, primitive aliases are replaced by the underlying primitive
type and therefore do not cause name collisions.

### Constants

Constants must be named in `ALL_CAPS_SNAKE_CASE`.

```
const uint64 FOO_BAR = 4096;
```

Constant names must not repeat names from the enclosing library.  In all target
languages, constant names are scoped by their enclosing library.

### Interfaces

Interfaces must be named in `UpperCamelCase` and must be noun phrases.  Typically,
interfaces are named using nouns that suggest an action.  For example,
`AudioRenderer` is a noun that suggests that the interface is related to
rendering audio.  Similarly, `Launcher` is a noun that suggests that the
interface is related to launching something.  Interfaces can also be passive
nouns, particularly if they relate to some state held by the implementation.
For example, `Directory` is a noun that suggests that the interface is used for
interacting with a directory held by the implementation.

Interface may be named using object-oriented design patterns.  For example,
`fuchsia.fonts.Provider` uses the "provider" suffix, which indicates that the
interface provides fonts (rather than represents a font itself).  Similarly,
`fuchsia.tracing.Controller` uses the "controller" suffix, which indicates that
the interface controls the tracing system (rather than represents a trace
itself).

The name `Manager` may be used as a name of last resort for an interface with
broad scope.  For example, `fuchsia.power.Manager`.  However, be warned that
"manager" interfaces tend to attract a large amount of loosely related
functionality that might be better factored into multiple interfaces.

Interfaces must not include the name "service."  All interfaces define services.
The term is meaningless.  For example, `fuchsia.net.oldhttp.HttpService`
violates this rubric in two ways.  First, the "http" prefix is redundant with
the library name.  Second, the "service" suffix is banned.  Notice that the
successor FIDL library, `fuchsia.net.http` simply omits this useless interface.

### Methods

Methods must be named in `UpperCamelCase` and must be verb phrases.  For
example, `GetBatteryStatus` and `CreateSession` are verb phrases that indicate
what action the method performs.

Methods on "listener" or "observer" interfaces that are called when an event
occurs should be prefixed with `On` and describe the event that occurred in the
past tense.  For example, the `ViewContainerListener` interface has a method
named `OnChildAttached`.  Similarly, events (i.e., unsolicited messages from the
server to the client) should be prefixed with `On` and describe the event that
occurred in the past tense.  For example, the `AudioCapturer` interface has an
event named `OnPacketCaptured`.

### Parameters

Parameter must be named in `lower_snake_case`.

### Structs and unions

Structs and unions must be named in `UpperCamelCase` and must be noun phrases.
For example, `Point` is a struct that defines a location in space and
`KeyboardEvent` is a struct that defines a keyboard-related event.

### Struct and union members

Struct and union members must be named in `lower_snake_case`.  Prefer names with
a single word when practical because single-word names render more consistently
across target languages.  However, do not be afraid to use multiple words if a
single word would be ambiguous or confusing.

Member names must not repeat names from the enclosing type (or library).  For
example, the `KeyboardEvent` member that contains the time the event was
delivered should be named `time` rather than `event_time` because the name
`event` already appears in the name of the enclosing type.  In all target
languages, member names are scoped by their enclosing type.

### Enums

Enums must be named in `UpperCamelCase` and must be noun phrases.  For example,
`PixelFormat` is an enum that defines how colors are encoded into bits in an
image.

### Enum members

Enum members must be named in `ALL_CAPS_SNAKE_CASE`.

Enum member names must not repeat names from the enclosing type (or library).
For example, members of `PixelFormat` enum should be named `ARGB` rather than
`PIXEL_FORMAT_ARGB` because the name `PIXEL_FORMAT` already appears in the name
of the enclosing type.  In all target languages, enum member names are scoped by
their enclosing type.

## Organization

### Syntax

 * Use 4 space indents.
 * Never use tabs.
 * Avoid trailing whitespace.
 * Separate declarations for `struct`, `union`, `enum`, and `interface` constructs from other declarations with one newline.
 * End files with exactly one newline character.

### Comments

Use `// comments` to document your library.  Place comments above the thing
being described.  Use reasonably complete sentences with proper capitalization
and periods:

```
struct Widget {
    // Widgets must be published with monotonically increasing ids.
    uint64 id;
    // Relative to the center.
    Point location;
};
```

Types or values defined by some external source of truth should be commented
with references to the external thing.  For example, reference the WiFi
specification that describes a configuration structure.  Similarly, if a
structure must match an ABI defined in a C header, reference the C header.

If you would like your comments to "flow through" to the target language,
then use either `///` as the comment introducer (yes, three forward slashes
in a row) or the `[Doc = "this is a comment"]` attribute:

```fidl
/// this is a comment that flows through to the target

[Doc = "and so is this"]
```

#### Flow-through vs. regular comment guidelines

For flow through comments, the `///` form is preferred over the `[Doc = ]`
form; the latter is intended as an internal implementation hook.

When deciding what should be a regular "`//`" comment versus a flow-through
comment, keep in mind the following.

Regular comments:

 * internal "todo" comments
 * copyright notices
 * implementation details

Flow-through comments:

 * descriptions of parameters, arguments, function
 * usage notes

For example:

```fidl
// TODO -- this function needs additional error checks
/// WatchedEvent describes events returned from a DirectoryWatcher.
struct WatchedEvent {
...
```

### Files

A library is comprised of one or more files.  The files are stored in a
directory hierarchy with the following conventions:

```
fidl/<library>/[<dir>/]*<file>.fidl
```

The `<library>` directory is named using the dot-separated name of the FIDL
library.  The `<dir>` subdirectories are optional and typically not used for
libraries with less than a dozen files.  This directory structure matches how
FIDL files are included in the Fuchsia SDK.

The division of a library into files has no technical impact on consumers of the
library.  Declarations, including interfaces, can reference each other and
themselves throughout the library, regardless of the file in which they appear.
Divide libraries into files to maximize readability.

 * Prefer a DAG dependency diagram for files in a library.

 * Prefer keeping mutually referring definitions textually close to each other,
   ideally in the same file.

 * For complex libraries, prefer defining pure data types or constants in leaf
   files and defining interfaces that reference those types together in a trunk
   file.

### Ordinals

Interfaces contain a number of methods.  In its declaration, each method is
assigned a unique 32 bit identifier, called an ordinal.

Interfaces evolve in two directions.  First, an interface can grow new methods,
with new ordinals.  Second, a superinterface can be extended by a subinterface.
The subinterface has all of the methods of its superinterface plus its own.

The goal of the guidelines here is to avoid these extension mechanisms
colliding.

 * Never use the zero ordinal. (The compiler forbids the zero ordinal.)

 * Ordinals within an interface should be allocated in contiguous blocks. For example:
   * 0x80000001--0x80000007
   * 1, 2, 3
   * 1000--1010, 1100--1112, 1200--1999

 * New ordinals in an interface should use the next ordinal in the block. After
   1, 2, and 3, use 4.

 * Related interfaces should consider using nearby and distinct ordinal blocks:

 * Interfaces A and B, in the same library, that refer to each other might
   choose to allocate in blocks 0x100-0x1ff and 0x200-0x2ff respectively.

 * Interfaces that expect to be extended by subinterfaces should explicitly
   claim ordinal blocks in a comment.

### Library structure

Carefully consider how you divide your type and interface definitions into
libraries.  How you decompose these definitions into libraries has a large
effect on the consumers of these definitions because a FIDL library is the unit
of dependency and distribution for your protocols.

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
design systems \[...\] are constrained to produce designs which are copies of the
communication structures of these organizations."  We should spend a moderate
amount of time fighting Conway's law.

## Types

As mentioned under "general advice," you should pay particular attention to the
types you used in your protocol definition.

### Be consistent

Use consistent types for the same concept.  For example, use a uint32 or a int32
for a particular concept consistently throughout your library.  If you create a
struct for a concept, be consistent about using that struct to represent the
concept.

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

```
struct Ipv4Address {
    array<uint8>:4 octets;
};
```

In performance-critical target languages, structs are represented in line, which
reduces the cost of using structs to name important concepts.

### Consider using fuchsia.mem.Buffer

A Virtual Memory Object (VMO) is a kernel object that represents a contiguous
region of virtual memory.  VMOs track memory on a per-page basis, which means a
VMO by itself does not track its size at byte-granularity.  When sending memory
in a FIDL message, you will often need to send both a VMO and a size.  Rather
than sending these primitives separately, consider using `fuchsia.mem.Buffer`,
which combines these primitives and names this common concept.

### Specify bounds for vector and string

Most `vector` and `string` declarations should specify a length bound.  Whenever
you omit a length bound, consider whether the receiver of the message would
really want to process arbitrarily long sequences or whether extremely long
sequences represent abuse.

Bear in mind that declarations that lack an upper bound are implicitly bounded
by the maximum message length when sent over a `zx::channel`.  If there really
are use cases for arbitrarily long sequences, simply omitting a bound might not
address those use cases because clients that attempt to provide extremely long
sequences might hit the maximum message length.

To address use cases with arbitrarily large sequences, consider breaking the
sequence up into multiple messages using one of the pagination patterns
discussed below or consider moving the data out of the message itself, for
example into a `fuchsia.mem.Buffer`.

### Errors

Select the appropriate error type for your use case and be consistent about how
you report errors.

Use the `status` type for errors related to kernel objects or IO.  For example,
`fuchsia.process` uses `status` because the library is largely concerned with
manipulating kernel objects.  As another example, `fuchsia.io` uses `status`
extensively because the library is concerned with IO.

Use a domain-specific enum error type for other domains.  For example, use an
enum when you expect clients to receive the error and then stop rather than
propagate the error to another system.

If a method can return either an error or a result, use the following pattern:

```
enum MyStatus { OK; FOO; BAR; ... };

interface Frobinator {
    1: Frobinate(...) -> (MyStatus status, FrobinateResult? result);
};
```

In some unusual situations, interfaces may include a string description of the
error in addition to a `status` or enum value if the range of possible error
conditions is large and descriptive error messages are likely to be useful to
clients.  However, including a string invites difficulties.  For example,
clients might try to parse the string to understand what happened, which means
the exact format of the string becomes part of the interface, which is
especially problematic when the strings are localized.  *Security note:*
Similarly, reporting stack traces or exception messages to the client can
unintentionally leak privileged information.

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

### Should I use string or vector?

In FIDL, `string` data must be valid UTF-8, which means strings can represent
sequences of Unicode code points but cannot represent arbitrary binary data.  In
contrast, `vector` or `array` can represent arbitrary binary data and do not
implicate Unicode.

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

Use `vector` or `array` for non-text data:

 * Use `vector<uint8>` for HTTP header fields because HTTP header fields do not
   specify an encoding and therefore cannot necessarily be represented in UTF-8.

 * Use `array<uint8>:6` for MAC addresses because MAC address are binary data.

 * Use `array<uint8>:16` for UUIDs because UUIDs are (almost!) arbitrary binary
   data.

### Should I use vector or array?

A `vector` is a variable-length sequence that is represented out-of-line in the
wire format.  An `array` is a fixed-length sequence that is represented in-line
in the wire format.

Use `vector` for variable-length data:

 * Use `vector` for tags in log messages because log messages can have between
   zero and five tags.

Use `array` for fixed-length data:

 * Use `array` for MAC addresses because a MAC address is always six bytes long.

### When should I use an enum?

(Note: This section depends on a proposed FIDL 2.1 feature that makes enums
extensible.)

Use an enum if the set of enumerated values is bounded and controlled by the
Fuchsia project.  For example, the Fuchsia project defines the pointer event
input model and therefore controls the values enumerated by `PointerEventPhase`.

In some scenarios, you should use an enum even if the Fuchsia project itself
does not control the set of enumerated values if we can reasonably expect that
people who will want to register new values will submit a patch to the Fuchsia
source tree to register their values.  For example, texture formats need to be
understood by the Fuchsia graphics drivers, which means new texture formats can
be added by developers working on those drivers even if the set of texture
formats is controlled by the graphics hardware vendors.  As a counter example,
do not use an enum to represent HTTP methods because we cannot reasonably expect
people who use novel HTTP methods to submit a patch to the Fuchsia source tree.

For _a priori_ unbounded sets, a `string` might be a more appropriate choice if
you foresee wanting to extend the set dynamically.  For example, use a `string`
to represent media codec names because intermediaries might be able to do
something reasonable with a novel media code name.

If the set of enumerated values is controlled by an external entity, use an
integer (of an appropriate size) or a `string`.  For example, use an integer (or
some size) to represent USB HID identifiers because the set of USB HID
identifiers is controlled by an industry consortium.  Similarly, use a `string`
to represent a MIME type because MIME types are controlled (at least in theory)
by an IANA registry.

## Good Design Patterns

This section describes several good design patterns that recur in many FIDL
protocols.

### Interface request pipelining

One of the best and most widely used design patterns is _interface request
pipelining_.  Rather than returning a channel that implements an interface, the
client sends the channel and requests the server to bind an implementation of
the interface to that channel:

```
GOOD:
interface Foo {
    1: GetBar(string name, request<Bar> bar);
};

BAD:
interface Foo {
    1: GetBar(string name) -> (Bar bar);
};
```

This pattern is useful because the client does not need to wait for a round-trip
before starting to use the `Bar` interface.  Instead, the client can queue
messages for `Bar` immediately.  Those messages will be buffered by the kernel
and processed eventually once an implementation of `Bar` binds to the interface
request.  By contrast, if the server returns an instance of the `Bar` interface,
the client needs to wait for the whole round-trip before queuing messages for
`Bar`.

If the request is likely to fail, consider extending this pattern with a reply
that describes whether the operation succeeded:

```
interface CodecProvider {
    1: TryToCreateCodec(CodecParams params, request<Codec> codec) -> (bool succeed);
};
```

To handle the failure case, the client waits for the reply and takes some other
action if the request failed.  Another approach is for the interface to have an
event that the server sends at the start of the protocol:

```
interface Codec2 {
    1: -> OnReady();
};

interface CodecProvider2 {
    1: TryToCreateCodec(CodecParams params, request<Codec2> codec);
};
```

To handle the failure case, the client waits for the `OnReady` event and takes
some other action if the `Codec2` channel is closed before the event arrives.

However, if the request is likely to succeed, having either kind of success
signal can be harmful because the signal allows the client to distinguish
between different failure modes that often should be handled in the same way.
For example, the client should treat a service that fails immediately after
establishing a connection in the same way as a service that cannot be reached in
the first place.  In both situations, the service is unavailable and the client
should either generate an error or find another way to accomplishing its task.

### Flow Control

FIDL messages are buffered by the kernel.  If one endpoint produces more
messages than the other endpoint consumes, the messages will accumulate in the
kernel, taking up memory and making it more difficult for the system to recover.
Instead, well-designed protocols should throttle the production of messages to
match the rate at which those messages are consumed, a property known as _flow
control_.

The kernel provides some amount of flow control in the form of back pressure on
channels.  However, most protocols should have protocol-level flow control and
use channel back pressure as a backstop to protect the rest of the system when
the protocol fails to work as designed.

Flow control is a broad, complex topic, and there are a number of effective
design patterns.  This section discusses some of the more popular flow control
patterns but is not exhaustive.  Protocols are free to use whatever flow control
mechanisms best suit their use cases, even if that mechanism is not listed
below.

#### Prefer pull to push

Without careful design, protocols in which the server pushes data to the client
often have poor flow control.  One approach to providing better flow control is
to have the client pull one or a range from the server.  Pull models have
built-in flow control the client naturally limits the rate at which the server
produces data and avoids getting overwhelmed by messages pushed from the server.

A simple way to implement a pull-based protocol is to "park a callback" with the
server using the _hanging get pattern_.  In this pattern, the client sends a
`GetFoo` message, but the server does not reply immediately.  Instead, the
server replies when a "foo" is available.  The client consumes the foo and
immediately sends another hanging get.  The client and server each do one unit
of work per data item, which means neither gets ahead of the other.

The hanging get pattern works well when the set of data items being transferred
is bounded in size and the server-side state is simple, but does not work well
in situations where the client and server need to synchronize their work.

#### Throttle push using acknowledgements

One approach to providing flow control in protocols that use the push, is the
_acknowledgment pattern_, in which the caller provides an acknowledgement
response that the caller uses for flow control.  For example, consider this
generic listener interface:

```
interface Listener {
    1: OnBar(...) -> ();
};
```

The listener is expected to send an empty response message immediately upon
receiving the `OnBar` message.  The response does not convey any data to the
caller.  Instead, the response lets the caller observe the rate at which the
callee is consuming messages.  The caller should throttle the rate at which it
produces messages to match the rate at which the callee consumes them.  For
example, the caller might arrange for only one (or a fixed number) of messages
to be in flight (i.e., waiting for acknowledgement).

#### Events

In FIDL, servers can send clients unsolicited messages called _events_.
Protocols that use events need to provide particular attention to flow control
because the event mechanism itself does not provide any flow control.

A good use case for events is when at most one instance of the event will be
sent for the lifetime of the channel.  In this pattern, the protocol does not
need any flow control for the event:

```
interface DeathWish {
    1: -> OnFatalError(status error_code);
};
```

Another good use case for events is when the client requests that the server
produce events and when the overall number of events produced by the server is
bounded.  This pattern is a more sophisticated version of the hanging get
pattern in which the server can respond to the "get" request a bounded number of
times (rather than just once):

```
interface NetworkScanner {
    1: ScanForNetworks();
    2: -> OnNetworkDiscovered(string network);
    3: -> OnScanFinished();
};
```

If there is no a priori bound on the number of events, consider having the
client acknowledge the events by sending a message.  This pattern is a more
awkward version of the acknowledgement pattern in which the roles of client and
server are switched.  As in the acknowledgement pattern, the server should
throttle event production to match the rate at which the client consumes the
events:

```
interface View {
    1: -> OnInputEvent(InputEvent event);
    2: NotifyInputEventHandled();
};
```

One advantage to this pattern over the normal acknowledgement pattern is that
the client can more easily acknowledge multiple events with a single message
because the acknowledgement is disassociated from the event being acknowledged.
This pattern allows for more efficient batch processing by reducing the volume
of acknowledgement messages and works well for in-order processing of multiple
event types:

```
interface View {
    1: -> OnInputEvent(InputEvent event, uint64 seq);
    2: -> OnFocusChangedEvent(FocusChangedEvent event, uint64 seq);
    3: NotifyEventsHandled(uint64 last_seq);
};
```

### Feed-forward dataflow

Some protocols have _feed-forward dataflow_, which avoids round-trip latency by
having data flow primarily in one direction, typically from client to server.
The protocol only synchronizes the two endpoints when necessary.  Feed-forward
dataflow also increases throughput because fewer total context switches are
required to perform a given task.

The key to feed-forward dataflow is to remove the need for clients to wait for
results from prior method calls before sending subsequent messages.  For
example, interface request pipelining removes the need for the client to wait
for the server to reply with an interface before the client can use the
interface.  Similarly, client-assigned identifiers (see below) removes the need
for the client to wait for the server to assign identifiers for state held by
the server.

Typically, a feed-forward protocol will involve the client submitting a sequence
of one-way method calls without waiting for a response from the server.  After
submitting these messages, the client explicitly synchronizes with the server by
calling a method such as `Commit` or `Flush` that has a reply.  The reply might
be an empty message or might contain information about whether the submitted
sequence succeeded.  In more sophisticated protocols, the one-way messages are
represented as a union of command objects rather than individual method calls,
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

```
interface Canvas {
    1: Flush() -> (status code);
    2: Clear();
    3: UploadImage(uint32 image_id, Image image);
    4: PaintImage(uint32 image_id, float x, float y);
    5: DiscardImage(uint32 image_id);
    6: PaintSmileyFace(float x, float y);
    7: PaintMoustache(float x, float y);
};
```

### Client-assigned identifiers

Often an interface will let a client manipulate multiple pieces of state held by
the server.  When designing an object system, the typical approach to this
problem is to create separate objects for each coherent piece of state held by
the server.  However, when designing a protocol, using separate objects for each
piece of state has several disadvantages:

Creating separate interface instances for each logical object consumes kernel
resources because each interface instance requires a separate channel object.
Each interface instance maintains a separate FIFO queue of messages.  Using
separate interface instances for each logical object means that messages sent
to different objects can be reordered with respect to each other, leading to
out-of-order interactions between the client and the server.

The _client-assigned identifier pattern_ avoids these problems by having the
client assign uint32 or uint64 identifiers to objects retained by the server.
All the messages exchanged between the client and the server are funnelled
through a single interface instance, which provides a consistent FIFO ordering
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
an object-oriented facades for the service to hide the complexity of managing
identifiers, which itself is an antipattern (see _client libraries_ below).

A strong signal that you should create a separate interface instance to
represent an object rather than using a client-assigned identifier is when you
want to use the kernel's object capability system to protect access to that
object.  For example, if you want a client to be able to interact with an object
but you do not want the client to be able to interact with other objects,
creating a separate interface instance means you can use the underlying channel
as a capability that controls access to that object.

### Command union

In protocols that use feed-forward dataflow, the client often sends many one-way
messages to the server before sending a two-way synchronization message.  If the
protocol involves a particularly high volume of messages, the overhead for
sending a message can become noticeable.  In those situations, consider using the
_command union pattern_ to batch multiple commands into a single message.

In this pattern, the client sends a `vector` of commands rather than sending an
individual message for each command.  The vector contains a union of all the
possible commands, and the server uses the union tag as the selector for command
dispatch in addition to using the method ordinal number:

```
struct PokeCmd { int32 x; int32 y; };

struct ProdCmd { string:64 message; };

union MyCommand {
    PokeCmd poke;
    ProdCmd prod;
};

interface HighVolumeSink {
  1: Enqueue(vector<MyCommand> commands);
  2: Commit() -> (MyStatus result);
};
```

Typically the client buffers the commands locally in its address space and sends
them to the server in a batch.  The client should flush the batch to the server
before hitting the channel capacity limits in either bytes and handles.

For protocols with even higher message volumes, consider using a ring buffer in
a `zx::vmo` for the data plane and an associated `zx::fifo` for the control
plane.  Such protocols place a higher implementation burden on the client and
the server but are appropriate when you need maximal performance.  For example,
the block device protocol uses this approach to optimize performance.

### Pagination

FIDL messages are typically sent over channels, which have a maximum message
size.  In many cases, the maximum message size is sufficient to transmit
reasonable amounts of data, but there are use cases for transmitting large (or
even unbounded) amounts of data.  One way to transmit a large or unbounded
amount of information is to use a _pagination pattern_.

#### Paginating Writes

A simple approach to paginating writes to the server is to let the client send
data in multiple messages and then have a "finalize" method that causes the
server to process the sent data:

```
interface Foo {
    1: AddBars(vector<Bar> bars);
    2: UseTheBars() -> (...);
};
```

For example, this pattern is used by `fuchsia.process.Launcher` to let the
client send an arbitrary number of environment variables.

A more sophisticated version of this pattern creates an interface that
represents the transaction, often called a _tear-off interface_:

```
interface BarTransaction {
    1: Add(vector<Bar> bars);
    2: Commit() -> (...);
};

interface Foo {
    1: StartBarTransaction(request<BarTransaction> transaction);
};
```

This approach is useful when the client might be performing many operations
concurrently and breaking the writes into separate messages loses atomicity.
Notice that `BarTransaction` does not need an `Abort` method.  The better
approach to aborting the transaction is for the client to close the
`BarTransaction` interface.

#### Paginating Reads

A simple approach to paginating reads from the server is to let the server send
multiple responses to a single request using events:

```
interface EventBasedGetter {
    1: GetBars();
    2: -> OnBars(vector<Bar> bars);
    3: -> OnBarsDone();
};
```

Depending on the domain-specific semantics, this pattern might also require a
second event that signals when the server is done sending data.  This approach
works well for simple cases but has a number of scaling problems.  For example,
the protocol lacks flow control and the client has no way to stop the server if
the client no longer needs additional data (short of closing the whole
interface).

A more robust approach uses a tear-off interface to create an iterator:

```
interface BarIterator {
    1: GetNext() -> (vector<Bar> bars);
};

interface ChannelBasedGetter {
    1: GetBars(request<BarIterator> iterator);
};
```

After calling `GetBars`, the client uses interface request pipelining to queue
the first `GetNext` call immediately.  Thereafter, the client repeatedly calls
`GetNext` to read additional data from the server, bounding the number of
outstanding `GetNext` messages to provide flow control.  Notice that the
iterator need not require a "done" response because the server can reply with an
empty vector and then close the iterator when done.

Another approach to paginating reads is to use a token.  In this approach, the
server stores the iterator state on the client in the form of an opaque token,
and the client returns the token to the server with each partial read:

```
struct Token { array<uint8>:16 opaque; }
interface TokenBasedGetter {
  // If  token  is null, fetch the first N entries. If  token  is not null, return the N items starting at  token
  // Returns as many entries as it can in  results  and populates  next_token  if more entries are available.
  1: GetEntries(Token? token) -> (vector<Entry> entries, Token? next_token);
}
```

This pattern is especially attractive when the server can escrow all of its
pagination state to the client and therefore no longer need to maintain
paginations state at all.  The server should document whether the client can
persist the token and reuse it across instances of the interface.  *Security
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

```
interface Foo {
    1: ExportThing(uint32 client_assigned_id, ..., handle<eventpair> export_token);
};

interface Bar {
    1: ImportThing(uint32 some_other_client_assigned_id, ..., handle<eventpair> import_token);
};
```

To correlate the objects, the server calls `zx_object_get_info` with
`ZX_INFO_HANDLE_BASIC` and matches the `koid` and `related_koid` properties from
the entangled event objects.

### Eventpair cancellation

When using tear-off transactions, the client can cancel long-running operations
by closing the client end of the interface.  The server should listen for
`ZX_CHANNEL_PEER_CLOSED` and abort the transaction to avoid wasting resources.

There is a similar use case for operations that do not have a dedicated channel.
For example, the `fuchsia.net.http.Loader` interface has a `Fetch` method that
initiates an HTTP request.  The server replies to the request with the HTTP
response once the HTTP transaction is complete, which might take a significant
amount of time.  The client has no obvious way to cancel the request short of
closing the entire `Loader` interface, which might cancel many other outstanding
requests.

The _eventpair cancellation pattern_ solves this problem by having the client
include one of the entangled events from a `zx::eventpair` as a parameter to the
method.  The server then listens for `ZX_EVENTPAIR_PEER_CLOSED` and cancels the
operation when that signal is asserted.  Using a `zx::eventpair` is better than
using a `zx::event` or some other signal because the `zx::eventpair` approach
implicitly handles the case where the client crashes or otherwise tears down
because the `ZX_EVENTPAIR_PEER_CLOSED` is generated automatically by the kernel
when the entangled event retained by the client is destroyed.

### Empty interfaces

Sometimes an empty interface can provide value.  For example, a method that
creates an object might also receive a `request<FooController>` parameter.  The
caller provides an implementation of this empty interface:

```
interface FooController {};
```

The `FooController` does not contain any methods for controlling the created
object, but the server can use the `ZX_CHANNEL_PEER_CLOSED` signal on the
interface to trigger destruction of the object.  In the future, the interface
could potentially be extended with methods for controlling the created object.

## Antipatterns

This section describes several antipatterns: design patterns that often provide
negative value.  Learning to recognize these patterns is the first step towards
avoiding using them in the wrong ways.

### Client libraries

Ideally, clients interface with protocols defined in FIDL using
language-specific client libraries generated by the FIDL compiler.  This
approach lets Fuchsia provide high-quality support for a large number of target
languages, but sometimes the protocol is too low-level to program directly and
a a hand-written client library is appropriate to provide an interface to the
same underlying protocol that is easier to use correctly.

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

### Service hubs

A _service hub_ is a `Discoverable` interface that simply lets you discover a
number of other interfaces, typically with explicit names:

```
BAD:
[Discoverable]
interface ServiceHub {
    1: GetFoo(request<Foo> foo);
    2: GetBar(request<Bar> bar);
    3: GetBaz(request<Baz> baz);
    4: GetQux(request<Qux> qux);
};
```

Particularly if stateless, the `ServiceHub` interface does not provide much
value over simply making the individual services discoverable directly:

```
[Discoverable]
interface Foo { ... };

[Discoverable]
interface Bar { ... };

[Discoverable]
interface Baz { ... };

[Discoverable]
interface Qux { ... };
```

Either way, the client can establish a connection to the enumerated services.
In the latter case, the client can discover the same services through the normal
mechanism used throughout the system to discover services.  Using the normal
mechanism lets the core platform apply appropriate policy to discovery.

However, service hubs can be useful in some situations.  For example, if the
interface were stateful or was obtained through some process more elaborate than
normal service discovery, then the interface could provide value by transferring
state to the obtained services.  As another example, if the methods for
obtaining the services take additional parameters, then the interface could
provide value by taking those parameters into account when connecting to the
services.

### Overly object-oriented design

Some libraries create separate interface instances for every logical object in
the protocol, but this approach has a number of disadvantages:

 * Message ordering between the different interface instances is undefined.
   Messages sent over a single interface are processed in FIFO order (in each
   direction), but messages sent over different channels race.  When the
   interaction between the client and the server is spread across many channels,
   there is a larger potential for bugs when messages are unexpectedly
   reordered.

 * Each interface instance has a cost in terms of kernel resources, waiting
   queues, and scheduling.  Although Fuchsia is designed to scale to large
   numbers of channels, the costs add up over the whole system and creating a
   huge proliferation of objects to model every logical object in the system
   places a large burden on the system.

* Error handling and teardown is much more complicated because the number of
  error and teardown states grows exponentially with the number of interface
  instances involved in the interaction.  When you use a single interface
  instance, both the client and the server can cleanly shut down the interaction
  by closing the interface.  With multiple interface instances, the interaction
  can get into states where the interaction is partially shutdown or where the
  two parties have inconsistent views of the shutdown state.

 * Coordination across interface boundaries is more complex than within a single
   interface because protocols that involve multiple interfaces need to allow
   for the possibility that different interfaces will be used by different
   clients, who might not completely trust each other.

However, there are use cases for separating functionality into multiple
interfaces:

 * Providing separate interfaces can be beneficial for security because some
   clients might have access to only one of the interfaces and thereby be
   restricted in their interactions with the server.

 * Separate interfaces can also more easily be used from separate threads.  For
   example, one interface might be bound to one thread and another interface
   might be bound to another thread.

 * Clients and servers pay a (small) cost for each method in an interface.
   Having one giant interface that contains every possible method can be less
   efficient than having multiple smaller interfaces if only a few of the
   smaller interfaces are needed at a time.

 * Sometimes the state held by the server factors cleanly along method
   boundaries.  In those cases, consider factoring the interface into smaller
   interfaces along those same boundaries to provide separate interfaces for
   interacting with separate state.

A good way to avoid over object-orientation is to use client-assigned
identifiers to model logical objects in the protocol.  That pattern lets clients
interact with a potentially large set of logical objects through a single
interface.
