# FIDL project checklist

Changes to FIDL can often have a large number of downstream effects. This
document provides checklists for items that should be considered when changing
FIDL. Note that not all elements of the checklist will necessarily apply to
every change, but they should at least be helpful for developers to double check
that their changes are complete.

## Changes to the `fidlc` compiler

This checklist is for changes that affect FIDL itself, such as syntax or
semantics changes.

* Update specifications.
  * [Language reference][fidl-ref] and code snippets where applicable.
  * [FIDL grammar][fidl-grammar].
  * [Bindings spec][bindings-spec] for guidance for how backends should generate
    code for this feature.
  * When appropriate, update the [lexicon].
* Update associated tools.
  * The FIDL formatter/linter and their associated TreeVisitors.
  * The [fidldoc] tool, for language changes that should be reflected in the
    generated documentation, e.g. a new data type or modifier.
  * [Editors] and syntax highlighting.
* Update FIDL files. For syntax changes, update existing FIDL source including:
  * Actual FIDL source files in tree (and out of tree, if required).
  * FIDL source specified as strings in test source code, such as the
    [compiler unit tests][fidlc-tests] and the [fidlgen end-to-end tests][fidlgen-tests].
  * FIDL snippets in documentation.
* Ensure that interactions with attributes are handled.
  * For example, does the feature require a new placement?
* Update integration tests
  * For larger language features, it may be worth adding test coverage in
    existing integration tests. For example, ensuring cross binding consistency
    through the compatibility tests, or transitionability through the source
    compatibility tests.
* For parser changes specifically:
  * Update the [`span_tests`][span-tests] to ensure that the parsed spans are
    correct, and not just that parsing succeeds
* For raw AST changes:
  * Ensure updates to the AST are propagated to the raw AST tree visitor and its
    downstream users as well. For example, adding a new field to a node probably
    should be accompanied with a visitor change that visits that new field.
* For semantics changes:
  * Update the feature specific unittest, e.g. [`table_tests`][table-tests] when
    making a modification to tables, or create a new test file.

It is expected for changes to FIDL which cascade to backends to update all
Fuchsia FIDL team owned bindings, i.e. Rust, Go, Dart, HLCPP, New C++ (includes
natural and wire APIs; the wire APIs were called LLCPP).

## Changes to the JSON IR

* Ensure that the change is reflected by a change in the IR goldens.
* Update the [JSON schema][fidlc-schema].
* Update any JSON consumers, including:
  * [fidlgen lib][fidlgen-lib]
  * [fidl codec][fidl-codec]
  * [fidlmerge]
  * [fidldoc]
  * [banjo]
  * [measure-tape]
  * [GIDL][gidl]
  * [zither]
  * [summarize][summarize]

## Changes to `fidlgen_<lang>` backends

* Update the [bindings docs][bindings-refs] if the generated code has
  changed.
* Update the [bindings tutorials][bindings-tutorials] to reflect the current
  best practices and patterns.
  * Incorporate example code which compiles and runs in the build (e.g.
    [//examples/fidl/dart/fidl_packages/test/types_test.dart][dart-example]).
* Update any relevant [guides].
  * [FIDL API rubric][api-rubric] for guidance on using this feature.
  * For example, when changing the memory allocation APIs in C++ wire types, the
    [C++ wire domain object memory ownership guide][llcpp-allocators] should be
    updated.
  * We've also found it good practice to present to API Council in order to
    socialize new features, and explain to council members how to review APIs in
    light of evolutions.
* If changing bindings API, update [GIDL backends][gidl] to use the updated API.
* If changing behavior or adding dynamic features, update [client][client suite]
  and [server][server suite] dynamic test suites.

## Changes to the FIDL wire format

* Update the bindings to support the new wire format.
* Update the [wire format specification][wire-format-spec] to document the new
  wire format.
* Update [measure-tape] to support size calculation in new wire format.
* Update [fidl-codec] to support encoding/decoding in the new wire format.
* Update targets which have to manually encode/decode FIDL (the following list
  and reasons was reviewed on Feb 2023):
  * [userboot]: as the first userspace process, it cannot use mutable data
    segment, cannot use [RELRO] (a flavor of relocation hardening technique used
    on Fuchsia), and cannot use thread local storage. The lack of RELRO
    precludes const global/static variables whose initializer contains non-NULL
    pointer values, which rules out many libraries including FIDL bindings. This
    will be addressed by [fxbug.dev/121753](https://fxbug.dev/121753).
  * [josh]: uses [fidl-codec] for encoding/decoding, but sets its own
    transactional message header.
  * [ldmsg]: used by the C library to load libraries during process startup. It
    cannot use full-fledged FIDL bindings due to
    [sanitizer ABI requirements][sanitizer-abi]. This will be addressed by
    [fxbug.dev/121817](https://fxbug.dev/121817).
  * [libc sanitizers/debugdata][debugdata]: the sanitizer support libraries
    themselves cannot use functions built with sanitizers. This will be
    addressed by [fxbug.dev/121754](https://fxbug.dev/121754).

## Horizontal testing requirements

Add coverage to:

* The [at-rest conformance suite].
* The [dynamic compatibility suite].
* The dynamic [client suite] and [server suite].
* The [dangerous identifiers suite].
* The [source compatibility suite].

<!-- xrefs -->
[api-rubric]: /development/api/fidl.md
[at-rest conformance suite]: /src/tests/fidl/conformance_suite/
[banjo]: /src/devices/tools/fidlgen_banjo
[bindings-refs]: /reference/fidl/bindings/overview.md
[bindings-spec]: /reference/fidl/language/bindings-spec.md
[bindings-tutorials]: /development/languages/fidl/tutorials/overview.md
[client suite]: /src/tests/fidl/client_suite/
[dart-example]: /examples/fidl/dart/fidl_packages/test/types_test.dart
[dangerous identifiers suite]: /src/tests/fidl/dangerous_identifiers/
[debugdata]: /zircon/system/ulib/c/sanitizers/debugdata.cc
[dynamic compatibility suite]: /src/tests/fidl/compatibility/
[editors]: /development/languages/fidl/guides/editors.md
[fidl-codec]: /src/lib/fidl_codec
[fidlc-tests]: /tools/fidl/fidlc/tests
[fidlc-schema]: /tools/fidl/fidlc/schema.json
[fidl-grammar]: /reference/fidl/language/grammar.md
[fidl-ref]: /reference/fidl/language/language.md
[fidldoc]: /tools/fidl/fidldoc
[fidlgen-lib]: /tools/fidl/lib/fidlgen
[fidlgen-tests]: /tools/fidl/lib/fidlgentest
[fidlmerge]: /tools/fidl/fidlmerge
[gidl]: /tools/fidl/gidl
[josh]: /src/developer/shell/josh/lib/fidl.cc
[ldmsg]: /zircon/system/ulib/ldmsg/ldmsg.c
[lexicon]: /reference/fidl/language/lexicon.md
[llcpp-allocators]: /development/languages/fidl/tutorials/cpp/topics/wire-memory-ownership.md
[measure-tape]: /tools/fidl/measure-tape
[RELRO]: https://www.redhat.com/en/blog/hardening-elf-binaries-using-relocation-read-only-relro
[sanitizer-abi]: /zircon/system/ulib/ldmsg/BUILD.gn
[summarize]: /tools/fidl/fidl_api_summarize/
[server suite]: /src/tests/fidl/server_suite/
[source compatibility suite]: /src/tests/fidl/source_compatibility/
[span-tests]: /tools/fidl/fidlc/tests/span_tests.cc
[table-tests]: /tools/fidl/fidlc/tests/table_tests.cc
[userboot]: https://cs.opensource.google/fuchsia/fuchsia/+/28e6aba6f37d7c5430f41d93e31674d2d401a47a:zircon/kernel/lib/userabi/userboot/start.cc;l=308
[wire-format-spec]: /reference/fidl/language/wire-format/README.md
[zither]: /zircon/tools/zither
