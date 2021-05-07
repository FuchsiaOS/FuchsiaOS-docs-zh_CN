# FIDL Project Checklist

Changes to FIDL can often have a large number of downstream effects. This
document provides checklists for items that should be considered for various
types of FIDL related changes. Note that not all elements of the checklist will
necessarily apply to every change, but they should at least be helpful for
developers to double check that their changes are complete.

## Change to the `fidlc` compiler

This checklist is for changes that affect FIDL itself, such as syntax or
semantics changes.

* Update documentation.
  * [Language reference][fidl-ref] and code snippets where applicable
  * [FIDL grammar][fidl-grammar]
  * [Bindings spec][bindings-spec] for guidance for how backends should generate
    code for this feature
  * [FIDL API rubric][api-rubric] for guidance on using this feature.
* Update associated tools.
  * The FIDL formatter/linter and their associated TreeVisitors.
  * [Editors] and syntax highlighting.
* Update FIDL files. For syntax changes, update existing FIDL source including:
  * Actual FIDL source files in tree (and out of tree, if required).
  * FIDL source specified as strings in test source code, such as the
    [compiler unit tests][compiler-test] and the [fidlgen end-to-end tests][fidlgen-tests].
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
    correct, and not just that parsing succeeds.
* For semantics changes:
  * Update the feature specific unittest, e.g. [`table_tests`][table-tests] when
    making a modification to tables, or create a new test file.

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
  changed
* Update the [bindings tutorials][bindings-tutorials] to reflect the current
  best practices and patterns.
* Update any relevant [guides].
  * For example, when changing the memory allocation APIs in LLCPP, the
    [LLCPP allocator guide][llcpp-allocators] should be updated.

<!-- xrefs -->
[editors]: /docs/development/languages/fidl/guides/editors.md
[fidlgen-lib]: /tools/fidl/lib/fidlgen
[fidl-codec]: /src/lib/fidl_codec
[fidlmerge]: /tools/fidl/fidlmerge
[bindings-refs]: /docs/reference/fidl/bindings/overview.md
[bindings-spec]: /docs/reference/fidl/language/bindings-spec.md
[fidl-ref]: /docs/reference/fidl/language/language.md
[fidl-grammar]: /docs/reference/fidl/language/grammar.md
[api-rubric]: /docs/concepts/api/fidl.md
[span-tests]: /zircon/system/utest/fidl-compiler/span_tests.cc
[table-tests]: /zircon/system/utest/fidl-compiler/table_tests.cc
[fidldoc]: /tools/fidl/fidldoc
[banjo]: /src/devices/tools/fidlgen_banjo
[measure-tape]: /tools/fidl/measure-tape
[gidl]: /tools/fidl/gidl
[kazoo]: /zircon/tools/kazoo
[bindings-tutorials]: /docs/development/languages/fidl/tutorials/overview.md
[llcpp-allocators]: /docs/development/languages/fidl/guides/llcpp-memory-ownership.md
[fidl-compiler]: /zircon/system/utest/fidl-compiler
[fidlgen-tests]: /tools/fidl/lib/fidlgentest
