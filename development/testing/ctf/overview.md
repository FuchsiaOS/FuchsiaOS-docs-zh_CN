# Compatibility Tests for Fuchsia

The Compatibility Tests for Fuchsia (CTF) is a suite of tests designed
to detect compatibility changes between two different versions of the
Fuchsia platform surface.  To learn how it works, and to get started on adding
CTF tests for your area, please see the links below.

## Fuchsia CTF Definition

* [CTF Overview][overview]: Background, motivation and goals for building the
Fuchsia CTF.
* [CTF RFC][rfc15]: Requirements, design and implementation strategy.
* [FAQ][faq]: Frequently asked questions.

## Contributing to the CTF

* [Contributing Guide][contributing]: One-stop shop with everything you need
to know about contributing to the Fuchsia CTF. Start here!

* Example FIDL Tests:
  * [Rust Echo Test][rust_echo_test]
  * [C++ Echo Test][cpp_echo_test]
  * For more examples, see [//sdk/ctf/tests/fidl][existing_tests]

## CTF Test Coverage

Note: TODO: Dashboards are currently internal-only

[overview]: /docs/development/testing/ctf/compatibility_testing.md
[rfc15]: /docs/contribute/governance/rfcs/0015_cts.md
[faq]: /docs/development/testing/ctf/faq.md
[contributing]: /docs/development/testing/ctf/contributing_tests.md
[cpp_echo_test]: /sdk/ctf/tests/examples/fidl/fuchsia.examples/cc/
[rust_echo_test]: /sdk/ctf/tests/examples/fidl/fuchsia.examples/rust/
[existing_tests]: /sdk/ctf/tests/fidl/
