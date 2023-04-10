<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0105" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC proposes to ban all regular expression libraries that do not explicitly
guarantee linear time complexity relative to the size of the string to be
matched. The officially-sanctioned libraries are
[RE2](https://github.com/google/re2) (C++),
[regex](https://docs.rs/regex/1.4.6/regex/) (Rust), and
[regexp](https://golang.org/pkg/regexp/) (Go).

## Motivation

It's well-known that pure regular expressions can be matched in `O(mn)` time
where `n` is the length of the input string and `m` is the size of the regular
expression's NFA after constructs like `a{10}b{10}` have been expanded out.
However, many libraries include extensions such as _backreferences_ which make
this problem worst-case exponential. Other libraries use a backtracking
implementation that is worst-case exponential even when the pattern does not use
backreferences. This turns regular expressions into a vector for denial of
service (DoS) attacks: if a malicious user controls the pattern or the string to
match, then the user may be able to trigger an exponential search that exhausts
CPU and causes a DoS. See
[Davis et al](https://people.cs.vt.edu/davisjam/downloads/publications/DavisCoghlanServantLee-EcosystemREDOS-ESECFSE18.pdf),
[Russ Cox](https://swtch.com/~rsc/regexp/regexp1.html),
[V8](https://v8.dev/blog/non-backtracking-regexp), and
[Wikipedia](https://en.wikipedia.org/wiki/ReDoS) for further discussion.

As of April 2021, Fuchsia uses regular expression libraries that may be
vulnerable to denial of service attacks. Below is a discussion for each of
Fuchsia's supported languages:

### C/C++

As of April 2021, there are
[38 uses](https://cs.opensource.google/search?q=include.%3Cregex%3E%20lang:Cc&ss=fuchsia%2Ffuchsia)
of `<regex>`, a library which does
[not make any guarantees](https://github.com/cplusplus/draft/blob/n4885/source/regex.tex)
about time or space complexity. By default, `<regex>` uses the same regular
expression language as `ECMA-262`, which is based on PCRE and supports
backreferences (see
[Hron Martin, Section 3.5.5](https://dspace.cvut.cz/bitstream/handle/10467/87858/F8-DP-2020-Hron-Martin-thesis.pdf)).

As of April 2021, there are
[6 uses](https://cs.opensource.google/search?q=include.%3Cregex.h%3E%20-file:zircon%2Fthird_party%2Fulib%2Fmusl&sq=&ss=fuchsia%2Ffuchsia)
of `<regex.h>`, a library which supports backreferences and uses a backtracking
implementation (see
[Hron Martin, Section 3.5.7](https://dspace.cvut.cz/bitstream/handle/10467/87858/F8-DP-2020-Hron-Martin-thesis.pdf)).

Google's [RE2](https://github.com/google/re2) does not support backreferences
and guarantees linear time matching relative to the size of the input string.
Further, RE2 has a configurable bound on memory usage which prevents the `m`
term in `O(mn)` from causing DoS. As of April 2021, this library is imported
into Fuchsia's `//third_party` but there are no users. Hence, it appears that
Fuchsia's C and C++ code _may_ be vulnerable to regular expression DoS.

### Rust

Rust's [regex](https://docs.rs/regex/1.4.6/regex/) crate does not support
backreferences, it guarantees `O(mn)` time matching, it bounds the NFA as does
RE2, and therefore it is not vulnerable to DoS. As of April 2021, this is the
only regular expression library used directly by Rust programs within Fuchsia.

Rust's [regex_automata](https://docs.rs/regex-automata/0.1.9/regex_automata/)
create allows creating arbitrary DFAs and has worst-case exponential time and
space usage. As of April 2021, this crate is not used directly by Fuchsia
programs, however it is used indirectly via other crates, such as
[bstr](https://docs.rs/bstr/0.2.16/bstr/trait.ByteSlice.html), which uses
`regex_automata` in a restricted way such that it can
[guarantee worst-case linear time and constant space](https://docs.rs/bstr/0.2.16/bstr/trait.ByteSlice.html#method.find).

### Go

Go's standard [regexp](https://golang.org/pkg/regexp/) package is based on RE2,
and therefore is not vulnerable to DoS. As of April 2021, this is the only
regular expression library used by Go programs within Fuchsia.

### Dart

Dart's standard
[RegExp](https://api.dart.dev/stable/2.12.4/dart-core/RegExp-class.html) library
is built on JavaScript's RegExp library, therefore it _may_ be vulnerable to
DoS.

## Design

To avoid DoS attacks, all Fuchsia code MUST use one of the following libraries
for regular expression parsing and matching:

*   C++ code MUST use RE2
*   Rust code MUST use the `regex` crate
*   Go code MUST use the standard `regexp` package

These sanctioned libraries are chosen because they all guarantee linear time
matching. It's not sufficient to simply ban features such as backtracking,
because even pure regular expressions can have
[pathological behavior](https://swtch.com/~rsc/regexp/regexp1.html) in libraries
that do not guarantee worst-case linear time matching.

We place no restrictions on Dart code. Dart's built-in RegExp class uses a
[backtracking implementation](https://github.com/dart-lang/sdk/blob/a6ffc74a4a6b75e20968b42ad2d19fc07a5b1e82/runtime/vm/regexp.cc#L60)
which supports backreferences and does not guarantee linear-time matching. We
considered banning backreferences in Dart programs, but decided this is
infeasible for Dart programs that receive regular expressions as user input:
such programs would be required to parse and validate regular expressions
received from the user, and currently, no widely-used libraries support this.
Further, Dart's RegExp implementation is based on V8's Irregexp, which can
suffer exponential blowup
[even on patterns that do not contain backreferences](https://v8.dev/blog/non-backtracking-regexp).
Since Dart
[cannot be used for core system services](/contribute/governance/policy/programming_languages.md),
DoS is less of a concern.

Exceptions to the above rules MAY be granted. The process for acquiring
exceptions is described below.

## Implementation

We will create a roller for the RE2 library, following the pattern established
for `//third_party/googletest`. A `//third_party/re2/OWNERS` file will be
created to specify who is responsible for maintaining this roller. The initial
OWNER will be `tombergan@google.com`. After this roller is deployed and
`//third_party/re2` has been updated to the latest version, existing C++ code
will be ported from `<regex>` and `<regex.h>` to RE2. No changes are needed to
Go, Rust, and Dart programs, as those are already compliant with this RFC.

After that migration is complete, two kinds of rules will be added:

1.  A presubmit rule based on
    [clang-tidy](https://clang.llvm.org/extra/clang-tidy/checks/portability-restrict-system-includes.html)
    will be added to ban `<regex>` and `<regex.h>` in C++ code.

1.  A
    [visibility](https://gn.googlesource.com/gn/+/master/docs/reference.md#var_visibility)
    rule will be added to prevent Fuchsia programs from importing Rust's
    `regex-automata` crate. The existing usage of this crate in `bstr` is
    granted an exception because `bstr`
    [guarantees worst-case linear time and constant space](https://docs.rs/bstr/0.2.16/bstr/trait.ByteSlice.html#method.find).

## Exceptions

Exceptions MAY be granted if the following conditions are met:

1.  There is a compelling need, such as a need for compatibility with a
    third-party library or tool.

1.  The newly imported library guarantees linear time behavior, OR, DoS is
    provably not an issue for the code in question. For example, if the regular
    expressions are used in a host tool or only as part of a developer workflow
    that is not available in production builds, then DoS is at worst a developer
    annoyance, not a security concern, and an exception MAY be granted.

Exceptions are documented in allowlists. Each allowlist MUST include a comment
linking to this RFC so that readers have appropriate context. To apply for an
exception, you MUST upload a CL to modify the appropriate allowlist. The CL MUST
give a detailed rationale for the exception and the CL MUST be approved by a
member of FEC. There are two kinds of allowlists:

1.  Our clang-tidy-based presubmit rule will have an allowlist in a TBD location
    in the Fuchsia source tree. The allowlist's location will be documented in
    the presubmit rule's error message so it can be discovered by running the
    normal presubmits.

1.  Third-party library exceptions are controlled by an ordinary `visibility`
    allowlist in the BUILD.gn file that defines the library.

New regular expression libraries cannot be imported to `//third_party` without
approval from a member of FEC. Once imported, the library MUST contain a
visibility allowlist along with a comment linking to this RFC.

## Performance

Since this proposal bans libraries with worst-case exponential time, this will
have a positive impact on performance.

## Code size

[fxrev.dev/532721](https://fxrev.dev/532721) analyzes expected changes in code
size for ARM64 builds. Since there are no changes to Go and Rust programs, there
will be no changes in code size for those programs. For C++ programs, the code
size increase depends on whether RE2 is linked statically or dynamically.
Currently, Fuchsia host tools are linked statically, while target programs are
linked either statically or dynamically depending on code size impacts. With
static linking, C++ programs will increase in size by at least 180KB. With
dynamic linking, the RE2 shared library costs about 350 KB. This cost is paid
one time per system image: C++ programs that used `<regex.h>` will see
approximately zero change in code size after switching to RE2, while C++
programs that used `<regex>` should get smaller after switched to RE2 because
`std::regex` makes heavy use of templates, while RE2 does not.

## Ergonomics

This proposal will lead to a more unified regular expression language across the
Fuchsia platform. For example, current C++ tools and services may support
backreferences via `<regex>`, while Rust and Go tools and do not. After this
proposal is implemented, no system tools or services will support backreferences
(except for tooling written in Dart).

Further, RE2 and Go's `regexp` support exactly the same regular expression
syntax, while Rust's `regex` is based on RE2 and supports a superset of RE2's
syntax. This standardizes Fuchsia regular expressions on RE2's syntax. Rust's
`regex` adds a few minor features, such as character class interection and
subtraction, and the `x` flag, which are not available in RE2.

## Backwards compatibility

I have manually inspected all existing uses of `<regex>` and `<regex.h>`. In the
majority of those cases, the regular expression pattern is a compile-time
constant that will work as-is (or with a slight syntax change) in RE2. There are
no backwards compatibility concerns in those cases.

I found just three programs which allow the user to specify a regular
expression: the debugger, `fidlcat`, and `perftest`. To ensure those tools will
not break after this change, I will add the OWNERs of those tools as reviewers
of this RFC.

## Security considerations

This RFC will reduce the likelihood that regular expressions can be used as a
vector for DoS attacks.

## Privacy considerations

None.

## Testing

Existing tests will be run. There are no new features to test.

## Documentation

As explained above, the presubmit and visibility rules will link to this RFC so
that developers know why their imports were rejected. Reviewers of new
third-party code will need to be made aware of this RFC so they know to reject
imports of unapproved regular expression libraries. This includes C++ and C
libraries that include `<regex>` or `<regex.h>` directly.

## Drawbacks, alternatives, and unknowns

See "Backwards Compatibility". Other C++, Rust, or Go libraries may provide
equivalent guarantees. We chose RE2, regex, and regexp (respectively) because
they are widely used and already imported into Fuchsia's `//third_party`.

There is a slight risk that future versions of these libraries might introduce
non-linear-time features. Since these libraries advertise linear-time matching
as a core feature, we think this is unlikely, but if it happens, we will need to
select new libraries.

## Prior art and references

See references linked from the motivation section, particularly the articles by
[Russ Cox](https://swtch.com/~rsc/regexp/regexp1.html), which give a history and
motivation for Google's development of RE2.
