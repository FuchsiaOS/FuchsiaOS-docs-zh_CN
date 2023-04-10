<!-- mdformat off -->

{% set rfcid = "RFC-0190" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}

# {{ rfc.name }}: {{ rfc.title }}

{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View
the #}
{# fully rendered RFCs at
https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}

<!-- mdformat on -->

## Summary

While system calls (i.e., "syscalls") represent Fuchsia's foundational platform
interface, the _Fuchsia Interface_ Definition Language, FIDL, does not have the
ability today to define them. This lack of support lends itself to more
important consequences than an ironic name however.
[_"bring your own runtime"_][bring-your-own-runtime] ("BYOR" henceforth) is one
of our core, aspirational design principles: it refers to facilitating (and
streamlining) the use of arbitrary application frameworks and programming
languages for development atop Fuchsia. One important aspect of this is the
enabling of a runtime to seamlessly interact with the userland system outside
of it; this is already robustly provided by FIDL's language-agnostic IPC
framework. Indeed, it is no coincidence that FIDL is Fuchsia's stated vehicle
for realizing BYOR. But there is far more to enabling a runtime than being able
to communicate with other processes. In order for any runtime to truly 'run',
it must be enabled to communicate with the kernel - that is, to make syscalls.
So FIDL falls short in a fundamental way.

This RFC addresses that gap and sets us on a path toward fully realizing BYOR.
In particular, the discussion includes

- extending FIDL to encode and [_fully specify_](#full-specification) the
  syscall interface in a _first-class_ manner;
- the consequences for FIDL and the syscall interface themselves in enabling
  the latter to conform to a formal model;
- the expectations around how FIDL language bindings would come to consume this
  information and what new facilities they would in turn present to their
  users;
- the substantial quality-of-life wins on offer in the maintenance of both the
  syscall interface and tooling that takes a dependency on it.

The document is a statement of intent rather than a complete proposal. We wish
for the larger design to emerge over the course of figuring out smaller design
questions (in follow-on RFCs), making room for much experimentation and careful
thinking along the way. Here we stake out the desired ends, a rough roadmap to
realizing them, and the principles and constraints to follow along the way.

Note: When we refer to the _syscall interface_ we mean the vDSO syscall binary
API. The interaction between the vDSO and the kernel is an implementation
detail and is out of scope.

## Motivation

There actually already exists a FIDL representation of the syscall interface
found under [//zircon/vdso][zircon-vdso-fidl]; this is FIDL library `zx`, which
we will refer to as the "v1 (FIDL) representation" from here on out. However,
this representation is not first-class and presents a large number of
idiosyncrasies to bindings trying to interpret it: namely a slew of custom
annotations and out-of-band signals around special names and comments, as well
as a framing that assumes C-like code will be generated from it. The
idiosyncrasies are sufficient that it is only practical for one backend,
[`kazoo`][kazoo], to interpret them. From these definitions today, kazoo
generates fundamental logic for the kernel and vDSO, a Rust syscall Foreign
Function Interface (i.e., FFI) wrapper, a Go syscall FFI wrapper for the
runtime in the standard library, as well as _sections_ of the syscall
documentation Markdown.

So we are currently in a transitional state where we have a quasi-FIDL
representation of syscalls that is crucially load-bearing, but it is not usable
as actual FIDL (beyond some basic enums and type aliases that are exported
today), and which is a maintenance burden for any tool to try to interpret.
While "we already started to do X and we're halfway there" is not a sound
argument for "let's follow through on X", it is certainly a sympathetic
motivation, especially given the below pains of the current state.

The v1 FIDL representation is only partially a source of truth though; it
contains syscall definitions, but an incomplete accounting of the data types
that figure into the interface. The source of truth for the data types - with a
fair amount of duplication - instead lies in a group of C headers (namely,
[\<zircon/types.h>, \<zircon/syscalls.h>, and \<zircon/syscalls/\*>][zircon-headers]).
The limiting nature of what can interpret the v1 representation and the fact
that the data types are defined in C both present a host of problems around
realizing BYOR as well as the maintenance of the interface itself.

### Motivation for an IDL

#### Interface evolution and drift

If the syscall interface is changed, so too must all of the disparate forks of
this information. These forks include

- Language-specific syscall wrappers.
  - We have these for [Rust][rust-syscall-wrappers] and
    [Go][go-syscall-wrappers]. Note that the Rust wrapper is an additional
    layer atop the thin FFI layer introduced by kazoo, and is maintained for
    the purposes of more idiomatic usage in the language.
- Language-specific analogues of the C data types.
  - We separately have these for [Rust][rust-syscall-data-types] and
    [Go][go-syscall-data-types].
- Use-cases that do not deal in _issuing_ syscalls but instead require more
  abstracted transformations of the syscall interface.
  - Our syscall Markdown is a prime example of this; `fidlcat`'s
    [synthesis of the interface][fidlcat-syscall-dependency] is another.

With a couple of kazoo-sponsored exceptions, syscall interface and fork
maintainers alike must do quite a bit of work to keep these things in sync and
prevent drift: in particular, syscall interface maintainers must know where all
forks are and remember to update them, and fork maintainers, for lack of a
machine-readable representation of this information, must know every syscall
and explicitly enumerate them.

If the syscall interface was fully encoded in an IDL, then these forks could be
reimagined as backends that could programmatically generate the desired
information (thus ceasing to be real forks). Of the anticipated backends, none
come to mind as being particularly complicated. Once stable, they should be far
lower 'touch' compared to the status quo, adapting automatically to most
changes to the syscall IDL (in the same way that most .fidl file changes today
do not require updating any particular backend).

#### C creep

With only C in view (both in the C headers and in the C-transliterated spelling
of the v1 FIDL representation), it is easy to introduce syscall signatures and
type definitions that are sensible in C, but are hard to model in other
languages. Notable examples are

- Buffers as separate pointer and lengths
  - Many languages have singular types to represent buffers and would not
    necessarily be able to decompose them in this way. Bindings for such
    languages would also need a further signal in order to interpret a
    (pointer, length) pair as a combined buffer rather than as unrelated
    arguments.
  - Modeling becomes more even more difficult in cases where a buffer's pointer
    and length do not appear beside one another in a syscall signature (as is
    the case with `zx_vmo_read()`).
- Untagged unions
  - C unions are referred to as _untagged unions_, as they represent a type
    disjointedly composed of other types but with no canonical means of
    determining what type a given instance actually holds (i.e., a "tag").
    _Tagged unions_ are naturally far more common in other languages (and
    useful), and are more likely to be presented as a singular concept.
  - Most (but not all) instances of C unions in the data types can be modeled
    as tagged unions, as they are featured in structs with an adjacent field
    that functions as a tag. However, similar to the above buffer case, there
    is no signal to indicate that these are related, and we have examples of
    non-adjacent tags.
- Multiplexing through type erasure
  - We have many syscalls that leverage the squishily-typed nature of `void*`
    to conditionally accept or return different types, effectively multiplexing
    many function signatures over one. A prime example is
    `zx_object_get_info(..., uint32_t topic, void* buffer, ...)`, which,
    depending on the `topic` will interpret `buffer` as any number of types.
    This can be rather difficult to model in languages that are not C: in
    particular, bindings would further need some sort of signal as to exactly
    how the signatures are parameterized, and would likely need to rely on
    non-canonically generating separate functions for each possible
    instantiation.

These sorts of C-isms can significantly complicate representing the syscall
interface in non-C languages. If one was instead working in the confines of a
polyglot-friendly IDL, then that framing would sidestep or minimize this sort
of problem.

#### Policy enforcement

We wish to enforce a number of policies around our syscalls and data types.
With IDL-ification, backends would have access to a machine-interpretable
representation of this information, at which point tooling for policy
enforcement could straightforwardly be written and maintained.

To name some example data type policies, consider the following - _as
represented in C_ - which derive from how we share data between the kernel and
userspace:

- **Fixed-sized structures, or dynamic-sized arrays of them**
  - To accept anything otherwise would complicate the sharing and
    interpretation of data.
- **No indirection**
  - The data needs to be wholly `memcpy`-able and understood within a different
    address space from which it was constructed; encoded references/pointers to
    data in the source address space could be left dangling and rendered
    incomprehensible. (`zx_iovec_t` represents an exception to this and is
    specially handled.)
- **No embedded handles**
  - Handles embedded in layouts are a recipe for forgetting to close that
    handle (or implicitly 'borrowing' it).
- **No boolean fields**
  - A `bool` takes up 8 bits in memory while only offering 1 bit of
    information. For better space efficiency, we prefer the use of bitfields to
    convey that 1 bit.
  - Paranoia: the C++ standard allows for an uninitialized `bool` to be neither
    true nor false!

### Motivation for FIDL as that IDL

#### Prior lessons from separate IDLs

The platform has previously introduced a non-FIDL IDL: Banjo, used for
expressing interfaces exposed by drivers. However, it was later recognized as a
misstep: it proved to be a maintenance burden outweighing its added value,
driver authors were confused about the syntax differences between it and FIDL,
and the natural desire to share data types between FIDL and Banjo was difficult
to realize. Since then, Banjo has been deprecated and a large-scale effort has
been underway to evolve FIDL for driver use-cases and use FIDL in its place.
Supposing that an IDL for syscall specification is sufficiently motivated, it
is reasonable to expect a new one for that purpose would suffer from the same
pitfalls.

#### Bindings development

FIDL language bindings rely on inter-process message passing (e.g., over
channels) and so must already have an understanding of the subset of the
syscalls that facilitate that communication. In particular, they already are in
the business of managing FFI wrappers around those syscalls. If these syscalls
were presented in a machine-readable form, then the bindings would be able to
construct and maintain the FFI layer more easily; if, however, the
understanding of the syscalls did not derive from FIDL, then bindings would
fork more information or end up interpreting two different IDL IRs
(intermediate representations).

On the data type side, today we maintain libraries that represent these types
separate from FIDL language bindings - but at the same time expect a degree of
interoperability. That is, we naturally expect both to be idiomatic in roughly
the same ways so that code can easily use them together without having to
contend with incompatible spellings or style decisions - and in the case of
kernel object handles and "`zx_status_t`", we expect these to be _identically_
represented. This is most naturally accomplished by having the bindings to
explicitly use the data type libraries, which is indeed what they do today.
Thus, that these things are presented as separate is mostly a false degree of
freedom - and it would certainly benefit the bindings maintainers and users
alike to have these things come from the same place (more on this
[below](#data-types-on-the-wire-match-their-c-representations)).

#### Polyglot nature

Per its relationship to BYOR, FIDL strives to use abstractions that are
sensibly mapped into any possible target language. This framing would guard
against accidentally biasing the syscall interface to a particular language
([e.g., C](#c-creep)).

#### Platform versioning

FIDL has built-in platform versioning support, which would
[simplify syscall evolution even further](#syscall-evolution-with-minimal-toil)
streamlining soft-transitions in effectively the same way it does for the rest
of FIDL.

## Stakeholders

_Facilitator:_

hjfreyer@google.com

_Reviewers:_

abarth@google.com, brettw@google.com, mcgrathr@google.com,
surajmalhotra@google.com

_Consulted:_

azaslavsky@google.com, bprosnitz@google.com, mcgrathr@google.com,
yifeit@google.com

_Socialization:_

The high-level ideas contained in this RFC have been socialized with both the
FIDL and kernel/vDSO maintainers.

## Design

As mentioned above, this document does not a offer a proper design. We instead
propose design goals, principles, and constraints for the effort here for
ratification.

### Goals

#### FIDL library `zx` is the sole source of truth for syscalls

Many of the problems outlined in this RFC revolve around competing sources of
truth for the shape, definition, and documentation of syscalls. The goal of
ensuring that all syscall information is directly downstream - via
straightforward machine translation - of a single source-of-truth directly
addresses this.

#### FIDL library `zx` is exported in the SDK

This is a basic prerequisite for realizing the BYOR paradigm proposed in this
RFC. Until this is achieved, the new facilities in question would only be
available to developers working within the platform.

Note: though we aim to generate the [syscall C headers][zircon-headers], this
goal is agnostic to whether we continue to export them in the SDK.

#### Syscall specification uses pure FIDL

In the end - with possibly switchbacks along the way - the syscall interface
should be defined only using features in the FIDL language specification. FIDL
backends should not need out-of-band information in order to interpret it -
certainly if its to be in the SDK - and other FIDL code should be free to
import and use its data types.

#### `fidlc` enforces syscall policy

By "syscall policy" here we mean any sort of check that constitutes a 'correct'
syscall declaration that is purely a function of the information present in the
FIDL representation. We enumerate some policies (albeit in C terms)
[above](#policy-enforcement).

The enforcement of syscall policy should happen at compilation-time. `fidlc`
would already be responsible for validating that a syscall declaration is
syntactically correct, so it is natural for it to also judge whether it is
semantically correct. We want to ensure that the latter validation happens at
some point, but deferring it to a later, arbitrary post-validation step would
complicate tooling integration, and would amount to a poor user experience in
delaying the time it takes for things to fail.

#### Syscall bindings

FIDL language backends should offer _syscall bindings_: that is, syscall
wrapper functions that present an interface _idiomatic_ in the style of that
backend's other bindings. In particular, this means that those function
signatures would feature library `zx` data types as they are _represented_ by
those bindings already. This provides canonical interoperation with the kernel
for any user, and it also resolves the awkwardness of contriving otherwise
separate bindings and syscall-issuing logic to cohere.

If we opted _not_ to offer syscall bindings, then the advent of `zx` data types
becoming importable as normal FIDL would introduce new awkwardness. Suppose a
FIDL client pulls a "`zx_port_packet_t`" off a channel and now wishes to
somehow call "`zx_port_queue()`" with it. How would this be accomplished? If
the caller has access to a syscall wrapper that takes as the existing binding
of that packet type as input, then this is trivial; if, however, the wrapper's
packet type differs, then there is the added burden of translation. No matter
where this translation logic lives, there will be poor user experience and a
host of awkward interdependencies. Better to have no translation at all and the
code in question be mutually comprehensible by design.

Given the suggestive object-oriented nature of the syscall interface around
particular kernel objects, a natural choice for syscall bindings there would be
as methods on a class representing that kernel object. This is at the
discretion of the backend maintainers, however.

Note: as mentioned before, we already contrive the appropriate syscall bindings
today; we just maintain them separately from the rest of the language bindings
that implicitly depend on them.

#### Syscall evolution with minimal toil

The aspiration is for syscall evolution to basically function in a few
straightforward steps (at least in the common case):

**Step 1:** Make a change to the FIDL syscall declarations, gating any addition
or removal on reaching platform API level X; update the kernel and vDSO to
follow through on the change.

**Non-Step:** Don't bother to update any backend (e.g., fidldoc for Markdown
generation), as each should automatically generate the correct syscall bindings
at any platform level.

**Step 2**: For each 'codebase', bump the targeted platform API level to >= X
and update the contained code to use the new syscall bindings (or ask
downstream projects to do this themselves).

**Possible Step 3:** If any syscall declarations were deprecated in Step 1,
once the minimum supported platform API level is >= X, go back and remove them.

### Principles

#### General FIDL utility where possible

With the introduction of new types, syntax, and semantics, clean concepts of
general utility to FIDL should be prioritized (or at least carefully
considered). A host of new features are needed and there are synergies with
non-syscall-related use-cases evident already (with more surely to be
realized). The more overlap there is, the more readable the syscall
declarations will be. This might also help increase familiarity with the
relevant support within the FIDL tooling, easing its maintenance.

FIDL language restrictions around the usage of new declaration types introduced
for syscall data types should be avoided - in the long-run - for the following
reasons:

- As mentioned [above](#syscall-specification-uses-pure-fidl), we want the
  specification to use "pure" FIDL - and so any sort of relegation of these new
  constructs will cut against that in spirit and possibly in practice;
- These types will be used at scale by every language backend virtue of
  appearing in library `zx`;
- Their `zx` instances will represent system information in its canonical form,
  and such information should freely flow across any transport;
- The relevant problems around the modeling of syscall data types are not
  actually syscall-specific at heart; they apply more generally to the modeling
  of [memory formats](#data-types-on-the-wire-match-their-c-representations)
  and could well have wider applications.
- A good chunk of the `zx` data types will be boring layouts using the basic,
  unrestricted declarations already present today; it would be strange if a
  seemingly arbitrary subset of these types had exceptional restrictions on
  their usage.

#### Minimizing interpretation burden

The syscall declaration IR should be as simple as possible (and no simpler) -
and backends should ideally be able to employ uniform logic (with minimal
special casing) in its interpretation. We should strive to make interpretation
a straightforward process for the indefinite number of future backend authors
and reduce the number of sharp corners they might have to navigate.

#### Willingness to change the syscall interface

We will try to model the syscall interface as best we can. Some things may
prove exceedingly challenging to model though. In these cases, with the
principle of
[minimizing interpretation burden](#minimizing-interpretation-burden) in mind,
we should be careful about overfit and be open to adapting the interface to
become more amenable to modeling.

### Constraints

#### Continued operational ignorance of FIDL for the vDSO and kernel.

By _operational_, we refer to express knowledge of FIDL in either syscall
signatures or their implementation considerations. This excludes the prospect
of using FIDL-generated code in the implementation of the vDSO and kernel,
which will (continue to) be of immense value.

The inner workings of the vDSO should remain opaque to FIDL. We are attempting
to model its interface, but that is an unrelated task to giving the subject
knowledge of its modeler. Making syscalls operationally aware of FIDL is
arguably an abstraction violation that would make these systems codependent,
but at the very least is a bizarre prospect since it will remain possible to
issue syscalls (not to mention run Fuchsia at large) outside the presence of
FIDL. It would also present a host of new system versioning problems. Syscall
bindings are merely meant to offer a bridge between the worlds of FIDL and the
vDSO. They are free to present syscalls to the user as communication with any
other FIDL endpoint, but that is a frontend choice on the part of the
individual backend.

#### Full specification

The syscall specification should be _full_ in that it encodes the full
semantics of the interface and that it generally contains all of the
machine-readable information that our syscall-dependent backends generally
require (including the backend that generates our documentation).

#### Syscall data types on the wire match their C representations

Imagine one wished to encode a memory format into FIDL. This consideration is
not limited to syscall parameters as they are expected by the C vDSO API, but
could include anything from a register layout, to a network packet header, to
the ZBI format. Consumers of this data naturally expect to consume it in the
defining format, so the FIDL encoding would only _usable_ in a particular
language if there was a means to translate its binding back to the format
proper. Language bindings already support encoding and decoding
[FIDL wire format][fidl-wire-format] for any type that can be defined in FIDL.
If the specific bitwise layout of that memory format exactly matches FIDL wire
format, then by describing each layout in FIDL, every language binding can
support that format too with no new format-specific work.

It is no accident that the wire format for basic FIDL types coincides with the
layout of their natural C analogues (and that is unlikely to change): today
these types are integral and boolean primitives, enums, bits, arrays, and
structs. For the sake of simplicity, and in order to give syscall bindings a
canonical means to render the expected vDSO inputs, we propose restricting the
modeling of the data types to these FIDL types alone. This in particular bars
the use of types with [out-of-line][fidl-wire-format] data, along with
FIDL-specific headers per the above constraint of operational ignorance.

Vectors represent an interesting case here and a possible exception. Syscalls
accept dynamic-sized arrays of data given as buffers with a separately provided
length. This situation is naturally modeled with vector binding. However, the
wire format of a vector is given as (length, pointer), while the syscall
expects the buffer parameters as (pointer, length). (The former's pointer also
has type `void*`, while the latter's can be a pointer to a specific type.) We
_could_ update all buffer parameters in the syscall interface so that the above
principle holds with vectors - but it seems reasonable and prudent to keep this
case as a defining exception. The amount of work that would be involved in
swapping these parameters everywhere is staggering and rather risky; the payoff
would be to sidestep the minimal cognitive burden of special casing the
serialization of vectors to swap two parameters. (We also have `zx_iovec_t`,
which is a vector-like type itself.)

Note: Per this constraint, there are a handful of data types that cannot yet be
modeled. These include anything defined in C with a `union`, as the current
FIDL extensible union construct has a more involved wire format. More on that
in future RFCs.

## Implementation

- For each new feature, the FIDL and kernel/vDSO maintainers will collaborate
  and produce an RFC. Once the feature is approved and implemented, the v1
  representation will be updated to use it.

- The idiosyncrasies of the v1 representation will be consolidated into an
  adaptive layer that more backends can be written atop of, ignorant of however
  the syscalls were originally encoded in FIDL. One straightforward way to
  accomplish this is to transform its IR into another approximating the
  ultimate IR that `fidlc` would yield when we have finished extending the
  language; this would also be a low stakes proving ground for incrementally
  figuring out what that latter IR should look like. As the v1 representation
  is updated to use new FIDL features, the adaptive layer can be winnowed down,
  eventually to the point of deletion. This will allow for various backends to
  proceed with syscall declaration support ahead of it actually being properly
  available.

- The syscall interface will be judiciously updated to meet new means of FIDL
  modeling where appropriate. Each such change will proceed on the basis of an
  RFC.

- The [syscall C headers][zircon-headers] will be generated.

- `fidldoc` will be extended to generate the syscall documentation from the
  FIDL library `zx`.

- Language backends will be extended to generate syscall bindings; code will be
  migrated over to using them.

- FIDL library `zx` will be exported in the SDK.

- The standalone libraries that offer syscall wrappers and data types in
  FIDL-agnostic ways will be deprecated.

## Performance

This proposal has no inherent performance consequences. It faithfully shifts
the performance considerations of the existing syscall-issuing logic for
various languages to their associated FIDL language backend. Although in
practice, given the coherence between the representation of the syscall data
types and the language bindings that we already maintain - the rust bindings
already make use of the fuchsia-zircon-types crate, and the LLCPP bindings of
C++ libzx - it is quite likely that the introduction of syscall bindings for
today's language backends will amount to generating the code that already
exists today with only cosmetic modification.

There will be follow-on RFCs to propose concrete changes in the context of this
effort; if present, relevant performance considerations will be discussed in
those forums.

## Ergonomics

Improvements:

- A single, fully documented source of truth.
- A cleaner and easier path toward BYOR.
- Easier syscall evolution and maintenance,
- Auto-enforced syscall policy.
- Existing syscall-dependent backends become easier to maintain (e.g., zxdb and
  fidlcat) and a host of new, valuable backends become straightforward to write
  (e.g., those for generating syscall descriptions for fuzzing engines (e.g.,
  Syzkaller), or one for generating syscall [MSan (memory sanitizer)][msan]
  checks).
- No integration problems between language bindings and syscall wrappers.

Regressions:

- Syscall bindings would be treated as normal FIDL-generated source and so they
  would likely become less readable and more obscure relative to today's
  equivalent libraries. The lack of documentation for generated source or its
  level of readability, however, is not a syscall-specific problem and we
  should strive to address it more generally irrespective of this effort.
- More responsibilities for existing language bindings, with a larger code
  surface for things to go wrong.

## Backwards Compatibility

There will be follow-on RFCs to propose concrete changes in the context of this
effort; if present, relevant backwards compatibility considerations will be
discussed in those forums.

## Security considerations

As mentioned above, this proposal will pave the way for more easily maintained
syscall-support in our sanitizer and fuzzing infrastructure.

There will be follow-on RFCs to propose concrete changes in the context of this
effort; if present, relevant security considerations will be discussed in those
forums.

## Privacy considerations

There will be follow-on RFCs to propose concrete changes in the context of this
effort; if present, relevant privacy considerations will be discussed in those
forums.

## Testing

Most, if not all, of what will be introduced over the course of this effort
should slot into existing subsystems and testing regimes:

- New FIDL features - like the existing ones - would be tested by the usual
  gamut of golden tests for `fidlc` and each relevant backend
- Syscall bindings for a given language would replace the equivalent syscall
  wrapper library that we hand-maintain today, so whatever testing is deemed
  appropriate for the latter could be updated to use the former. In particular,
  we could update Zircon's [core tests][core-tests] to use our C++ syscall
  bindings, and the unit tests we have in our //src/lib/zircon/rust crate could
  be updated to use of Rust syscall bindings.
- Further, any other backends that get modified in this effort (e.g., fidldoc)
  or rewritten would have their existing testing regimes extended or ported
  over.

## Documentation

New FIDL features will be documented identically to the current ones, extending
the language's official specification. Further, expectations for language
bindings will be updated to include the provision of syscall bindings.

## Drawbacks, alternatives, and unknowns

The status quo is rife with burdens: it is unduly burdensome for the
maintenance of syscalls, leaves them without the guardrails we regard as
necessary in the maintenance of our other public interfaces; it is hostile to
any software that takes the syscall interface as input, which - if Fuchsia is
to be successful - will come to be regarded as an unbounded number; it features
untethered forks of our fundamental system interface, which includes its own
documentation; it stands at odds with a core platform aspiration (BYOR); and it
hinders the cross-process sharing of basic system information in its canonical
form (as `zx_*` types are not represented in FIDL).

Much there is undesirable and much of it is firmly entrenched - which means it
will take a lot to fix things. Realizing the aims of this proposal will be a
long and bumpy road. The FIDL and kernel/vDSO maintainers will need to make
substantial and ongoing investments, ones that will likely be disruptive at
times to everyone alike. To not keep up a reasonable pace might result in
a(nother) protracted transitional state. Requisite changes to the syscall
interface will likely invite a host of hiccups in their follow-through.

The costs that carry into the final state should be limited to the ongoing
syscall support in official FIDL tooling, which should be relative limited and
low-touch. One could argue that the support in `fidlc` is a necessarily sunk
one, as we ultimately need _something_ to robustly understand and enforce the
full set of syscall semantics (and, even then, redundancy on that front is
valuable). Moreover, the logic of generating syscall bindings in our language
backends should amount to a straightforward machine translation from IR-encoded
signatures to source declarations, an application of wire encoding, and
exercise of the already-maintained FFI layer to call into the vDSO.

Alternatives:

- Do nothing.
  - We are already in a state sufficiently unbearable to have motivated this
    far-reaching proposal. Inaction would continue the current maintenance
    burdens, which are due to only exacerbate with time (and accrue to an
    increasingly larger audience as new developers attempt to port existing
    software and runtimes).
- We invest in a new IDL.
  - We could even concede interoperation with FIDL and just aim for a static,
    machine-readable description of the interface, which would still allow us
    to bring a good deal of value to the platform.
  - [Banjo's](#prior-lessons-from-separate-idls) history is instructive here
    though, and likely one that the platform does not wish to repeat.
- Forgo trying to change the syscall interface in order to model it.
  - While doable, this would concede a large number of the above design
    [goals and principles](#design) and enshrine the problems they are intended
    to address.

## Prior art and references

- The GNU Mach microkernel employed the same IDL for defining both its syscall
  and interprocess communication (see the `.def` files [here][mach-defs], and
  see [here][mach-mig] for the compiler (MIG)). This system is still used in
  [Darwin][darwin-defs] today.

- The [//zircon/vdso][zircon-vdso-fidl] v1 representation represents the
  platform's first attempt at FIDL-ifying syscalls.

- [Banjo](#prior-lessons-from-separate-idls) was a non-FIDL platform IDL.

[bring-your-own-runtime]: /docs/concepts/principles/simple.md#bring_your_own_runtime
[core-tests]: https://cs.opensource.google/fuchsia/fuchsia/+/main:zircon/system/utest/core/README.md;l=1;drc=9b242b75aa15d5cdc261fb912192877cc4023564
[darwin-defs]: https://github.com/apple/darwin-xnu/tree/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/libsyscall/mach
[fidl-wire-format]: /docs/reference/fidl/language/wire-format/README.md#primary_and_secondary_objects
[fidlcat-syscall-dependency]: https://cs.opensource.google/fuchsia/fuchsia/+/main:tools/fidlcat/lib/syscall_definition.cc;l=1;drc=934bcf504d3dc6ed23441bacf1d000bd3bdc019f
[go-syscall-data-types]: https://fuchsia.googlesource.com/third_party/go/+/c3e244fb8efd0f82f57f49a90394bdaa6a937680/src/syscall/zx/types.go
[go-syscall-wrappers]: https://fuchsia.googlesource.com/third_party/go/+/c3e244fb8efd0f82f57f49a90394bdaa6a937680/src/syscall/zx
[kazoo]: https://cs.opensource.google/fuchsia/fuchsia/+/main:zircon/tools/kazoo/;drc=00e95434a1a590fc200eab7adb37898728b77123
[mach-defs]: http://git.savannah.gnu.org/cgit/hurd/gnumach.git/tree/include/mach
[mach-mig]: http://git.savannah.gnu.org/cgit/hurd/mig.git/
[msan]: https://github.com/google/sanitizers/wiki/MemorySanitizer
[rust-syscall-data-types]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/lib/zircon/rust/fuchsia-zircon-types/src/lib.rs;drc=213bcacb1fa460ea2a1f7a7a68369f9621f04750
[rust-syscall-wrappers]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/lib/zircon/rust/src/;drc=b9b66708632a845e63cafa24a3bbd9d8b8cd766c
[zircon-headers]: https://cs.opensource.google/fuchsia/fuchsia/+/main:zircon/system/public/zircon/;drc=00e95434a1a590fc200eab7adb37898728b77123
[zircon-vdso-fidl]: https://cs.opensource.google/fuchsia/fuchsia/+/main:zircon/vdso/;drc=00e95434a1a590fc200eab7adb37898728b77123
