<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0164" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Former API Design Document

This RFC was previously submitted as an API design document and converted to an
RFC afterwards when the API design doc template was deprecated.

## Goals and use cases

* What problem does this API or API feature solve? What would users of your API
  be able to accomplish?
  * This API encapsulates all test functionality (enumeration, execution,
    observation) into a FIDL protocol.
    * Allows Fuchsia to provide a common interface to enumerate and execute
      POSIX tests on Fuchsia using a unified UX.
    * Allows Fuchsia to run a test and extract standard structured output across
      various testing frameworks.
    * To run/execute and get status of a Cfv2 component, it needs to expose some
      capability. This API provides a nice interface to encapsulate test
      execution phases and expose it as a Fuchsia capability.
  * OOT tree customers need to build and run tests which are not present
    in-tree, so exposing this API to the SDK allows them to implement the
    protocol and bring their own runtime for testing.

## Design

* The actual code definition of the API, such as the FIDL definition of an
  interface.
  * [fuchsia.test.Suite][suite-protocol]
* A Gerrit change link that contains the code for your API:
  * <https://fuchsia-review.googlesource.com/c/fuchsia/+/357623>
* Design Requirements
  * **Run tests** for test suites that are commonly used in the Fuchsia tree
    today.
  * Represent test outcomes as **structured data**, rather than streams of
    characters to stdout, process return codes, and various side effects on the
    system.
  * **Avoid over-fitting to specific runtimes** and test frameworks.
  * It should be **easy to bring up support** for new languages and testing
    frameworks.
  * Help with CFv2 migration efforts
    * Support Cfv2 tests.
* Non-goals:
  * Redefine/reinvent testing. \
**BYOR** means that developers will bring their own concepts, frameworks, and
expectations. Our role is to _accommodate_.

## Unknowns

During early design, we researched prior art in protocols for communicating
between host-side controllers and test runners. We found the prior art to lean
heavily on assumptions specific to products or runtimes. For instance, Android
Jetpack and google3 Nitrogen assume Android APKs and JUnit semantics to describe
test classes, methods, annotations, results, errors etc’.

We informed our design from a general knowledge of language and test frameworks
(gtest, Rust, Dart, JUnit, pytest). We mentally checked that we can implement
clients and servers for these examples, and implemented working examples for C++
gtest, Rust, and Golang.

We don’t know what we don’t know, so in the future we expect to continue to
revise this protocol.

## Usability

This section answers the following questions regarding the usability of your
API:

* Are the semantics of your API intuitive from its signature(s)?
  * Yes
* Have you designed the appropriate extensions points to allow for the future
  evolution of your API?
  * Yes
* Does your API behave similarly to other Fuchsia APIs that do similar things?
  * No, there are no such Fuchsia APIs which can run a test on a granular level
    and return structured results.
* How does your API behave compared to similar APIs for other platforms?
  * N/A

**We demonstrated usability and generality by implementing end-to-end usage
examples in two languages, C++ and Rust**

## Testing

* How do you plan to test your API?
  * The implementation of this FIDL api has extensive tests in Rust and C++.
* If developers were to rely on your API feature, how would they test their
  code?
  * This implementation basically implements a mechanism to run tests.
    Developers should write
    * Unit test to test their internal code.
    * Integration tests to make sure their implementation works coherently with
      the framework.

## Performance considerations

* Does your API involve a large number of round-trips across a process or thread
  boundary?
  * No
* Does your API involve blocking on a remote process or thread?
  * No
* Does your API involve copying large amounts of data?
  * Yes, but that is achieved using sockets and iterators.
* How many queries per second (QPS) do you expect your API to receive?
  * Each test implements and exposes this protocol via test runner and is torn
    down once the test execution completes. For each test client number of
    queries depend on number of test cases and throughput.
* How much data do you expect a typical query to transport?
  * For enumeration API, it can transport data equivalent to max FIDL size per
    iteration. Rest of the APIs transport data in low KBs.

## Security considerations

* Does your API expose security-sensitive information?
  * No
* Does your API let its users manipulate security sensitive resources?
  * No
* Are the users of your API isolated from each other?
  * Yes (If implemented as per API design).
* Does your API respect an object-capability discipline?
  * Yes
* Does your API encourage your users to use your API securely?
  * Does your API encourage time-of-check to time-of-use (TOCTOU)
    vulnerabilities?
    * No, But it is not needed as we can only run tests using this API
  * Does your API clearly separate any control planes from any data planes?
    * Yes

## Privacy considerations

* Does your API expose privacy-sensitive information?
  * No
* Does your API involve any personally identifiable information?
  * No
* Does your API involve any device identifiers?
  * No
* Does your API provide users control over how information is shared?
  * No user information is shared.

**Tests aren’t interactive user programs. They don’t ship to consumers.
Developers invoke tests on devices that they fully own, or devices that are
loaned to them from a shared pool and are wiped before they’re returned to the
pool.**

## Drawbacks and alternatives

**We can continue using Cfv1 design for test execution, but that significantly
limits our ability to provide structured test results and migrate various tests
and corresponding production components over to Cfv2.**

[suite-protocol]: https://osscs.corp.google.com/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.test/suite.fidl
