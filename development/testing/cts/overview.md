# Fuchsia Compatibility Test Suite

The Fuchsia Compatibility Test Suite (CTS) is a suite of tests designed
to detect compatibility changes between two different versions of the
Fuchsia platform surface.  To learn how it works, and to get started on adding
CTS tests for your area, please see the links below.

## Fuchsia CTS Definition
* [CTS Overview][overview]: Background, motivation and goals for building the
Fuchsia CTS.
* [CTS RFC][rfc15]: Requirements, design and implementation strategy.
* [FAQ][faq]: Frequently asked questions.

## Contributing to the CTS
* [Contributing Guide][contributing]: One-stop shop with everything you need
to know about contributing to the Fuchsia CTS.  Start here!  Below are a few
examples of CTS tests in action.
* Code Examples
  * [Hello World \[c++\]][hello c++]: A barebones example CTS test written in
C++.
  * [Hello World \[rust\]][hello rust]: A barebones example CTS test written
in Rust.
  * [fuchsia.diagnostics][diag]: An example real CTS test running in
production, protecting the fuchsia.diagnostics FIDL from compatibility issues.

## CTS Test Coverage

Note: TODO: Dashboards are currently internal-only

[overview]: /docs/development/testing/cts/compatibility_testing.md
[rfc15]: /docs/contribute/governance/rfcs/0015_cts.md
[faq]: /docs/development/testing/cts/faq.md
[contributing]: /docs/development/testing/cts/contributing_tests.md
[hello c++]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/sdk/cts/examples/hello_world/
[hello rust]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/sdk/cts/examples/rust/
[diag]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/sdk/cts/tests/fidl/fuchsia.diagnostics/
[cts team]: https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=Fuchsia+Compatibility+Test+Suite+%28CTS%29
