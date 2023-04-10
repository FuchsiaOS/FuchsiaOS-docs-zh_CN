# CTF Testing Guidelines

### Follow Fuchsia's best practices.

* [Documentation Guide]
* [Testing Best Practices]
* The style guide appropriate for your [language][Languages]

### Test must use the ctf_* rule variants found in //sdk/ctf/build.

CTF tests target API and ABI available through SDKs.  Build support ensures that
tests only depend on API elements that are available via an SDK, or allowlisted
for use within the CTF.  All build targets must use the `ctf_` rule variants
found in `//sdk/ctf/build` instead of the standard fuchsia.git rules (i.e., use
`ctf_fuchsia_component`, `ctf_executable`, and so on). The allowlist for non-SDK
code can be found in `//sdk/ctf/build/allowed_ctf_deps.gni`.  Test authors who
believe they need an additional inclusion should file a bug in the
[CTF bug component].

### Tests may depend on anything released via the SDK.

Depending on software that is not released via the SDK makes a CTF test more
subject to failure due to unrelated changes in Fuchsia platform internals. We
make exceptions for dependencies on a case-by-case basis. (e.g., internal test
frameworks that would work just as well out of tree).


### Tests using unstable test runners must bring their own test runner.

Tests using unstable test runners, such as Rust tests, must bring their own test runner
via subpackages. See the [CTF Contributing Guide] for details and examples.

### Test should not have flaky dependencies.

Tests must not depend on things that may go away, are intermittently available
or are specific to a particular platform where the test isn't always run.
Examples of such things include internet servers and operating system specific
filepath.

### Tests must be implemented in `//sdk/ctf/tests`.

If this is a concern, please reach out to fuchsia-ctf-team@google.com

### Tests should serve as examples of how to use an API.

Informally, if an end developer were to see the test, and copy its usage of the API, the
test author would believe that developer would be using the API correctly. Tests should,
to the extent possible, not depend on undocumented, application-specific invariants.  In
the future, in the case of widespread use of undocumented usages outside of the Fuchsia
tree, we may need to support use cases that do not follow recommended usages.

### Tests should not have timeouts.

Instead, timeouts should be enforced by the test infrastructure.

### Tests should not be stress or performance tests.

We do not encourage developers to submit stress tests or performance tests to
the CTF. Such tests will be examined closely for coverage value.

### Tests should target one element of the platform surface area.

CTF tests typically target elements of the platform surface area directly. If one fails,
it should be clear which parts of the platform surface area were triggered, and what the
changed outcome is. As a result of this rule, typically, the amount of application logic
in a test is small.

### Tests should not sleep.

Sleeps are common causes of flakiness in tests, as timing can vary wildly between runs
depending on target hardware and system load.  We recommend that developers structure code
so that an explicit signal is sent when a given condition is met, rather than having part
of a test wait an arbitrary amount of time for other code to finish.

### Tests should avoid mocking or faking the internal state of the target device.

The intent of the CTF is to make sure the entire device behaves correctly, not to make
sure that a particular component behaves correctly in isolation.

### Tests should exercise edge cases as well as typical inputs and outputs.

Examples include values of 0 and MAXINT for integers, and null pointers for pointer values.

### Tests should restore the state of the system when the test has completed.

For example, a test that makes a system-wide change to set the background color
of text should reset the color to its original value at the end of the test.
This prevents tests from affecting one another.

[CTF Contributing Guide]: /docs/development/testing/ctf/contributing_tests.md
[CTF bug component]: https://bugs.fuchsia.dev/p/fuchsia/templates/detail?saved=1&template=Fuchsia%20Compatibility%20Test%20Suite%20%28CTS%29&ts=1627669234
[Documentation Guide]: /docs/contribute/docs/documentation-standards.md
[Languages]: /docs/development/languages/README.md
[Testing Best Practices]: /docs/contribute/testing/best-practices.md
