# FIDL Project Checklist

Changes to FIDL can often have a large number of downstream effects. This
document provides checklists for items that should be considered when changing
FIDL. Note that not all elements of the checklist will necessarily apply to
every change, but they should at least be helpful for developers to double check
that their changes are complete.

## Change to the `fidlc` compiler

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
Fuchsia FIDL team owned bindings, i.e. Rust, Go, Dart, HLCPP, Unified C++ (was
LLCPP).

## Change to the JSON IR

* Esnure that the change is reflected by a change in the IR goldens.
* Update the JSON schema
* Update any JSON consumers, including:
  * [fidlgen lib][fidlgen-lib]
  * [fidl codec][fidl-codec]
  * [fidlmerge]
  * [fidldoc]
  * [banjo]
  * [measure-tape]
  * [GIDL][gidl]
  * [kazoo]

## Change to `fidlgen_<lang>`

* Update the [bindings docs][bindings-refs] if the generated code has
  changed.
* Update the [bindings tutorials][bindings-tutorials] to reflect the current
  best practices and patterns.
  * Incorporate example code which compiles and runs in the build (e.g.
    [//examples/fidl/dart/fidl_packages/test/types_test.dart](/examples/fidl/dart/fidl_packages/test/types_test.dart)).
* Update any relevant [guides].
  * [FIDL API rubric][api-rubric] for guidance on using this feature.
  * For example, when changing the memory allocation APIs in LLCPP, the
    [LLCPP allocator guide][llcpp-allocators] should be updated.
  * We've also found it good practice to present to API Council in order to
    socialize new features, and explain to council members how to review APIs in
    light of evolutions.

## Horizontal testing requirements

Add coverage to:

* The [at-rest conformance suite].
* The [dynamic compatibility suite].
* The [dangerous identifiers suite].
* The [source compatibility suite].

<!-- xrefs -->
[api-rubric]: /development/api/fidl.md
[at-rest conformance suite]: /src/tests/fidl/conformance_suite/
[banjo]: /src/devices/tools/fidlgen_banjo
[bindings-refs]: /reference/fidl/bindings/overview.md
[bindings-spec]: /reference/fidl/language/bindings-spec.md
[bindings-tutorials]: /development/languages/fidl/tutorials/overview.md
[dangerous identifiers suite]: /src/tests/fidl/dangerous_identifiers/
[dynamic compatibility suite]: /src/tests/fidl/compatibility/
[editors]: /development/languages/fidl/guides/editors.md
[fidl-codec]: /src/lib/fidl_codec
[fidlc-tests]: /tools/fidl/fidlc/tests
[fidl-grammar]: /reference/fidl/language/grammar.md
[fidl-ref]: /reference/fidl/language/language.md
[fidldoc]: /tools/fidl/fidldoc
[fidlgen-lib]: /tools/fidl/lib/fidlgen
[fidlgen-tests]: /tools/fidl/lib/fidlgentest
[fidlmerge]: /tools/fidl/fidlmerge
[gidl]: /tools/fidl/gidl
[kazoo]: /zircon/tools/kazoo
[lexicon]: /reference/fidl/language/lexicon.md
[llcpp-allocators]: /development/languages/fidl/tutorials/llcpp/topics/memory-ownership.md
[measure-tape]: /tools/fidl/measure-tape
[source compatibility suite]: /src/tests/fidl/source_compatibility/
[span-tests]: /tools/fidl/fidlc/tests/span_tests.cc
[table-tests]: /tools/fidl/fidlc/tests/table_tests.cc
