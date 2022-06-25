# Testing best practices

As you write tests for Fuchsia, you want to make sure that you are familiarized
with the [testing principles][testing-principles] and the
[testing scope][test-scope] for writing tests.

## Desirable properties of tests

The properties below are generally good to have, provided that they don't
conflict with other goals of the test.

- **Isolated**: tests should be isolated from code, systems, and details that
  are outside the scope of the tests. Different test cases should be isolated
  from each other. On Fuchsia we use the isolation guarantees given by the
  Component Framework to isolate tests. A useful outcome of isolated tests is
  that tests can be run in parallel or in a different order and their result is
  the same.
- **Hermetic**: the result of a test is defined by the contents of the test. On
  Fuchsia, a unit test or an integration test is hermetic if its result is
  defined by the contents of the test’s package. If a test’s package hasn’t
  changed then it can be assumed that its behavior hasn’t changed, and it’s
  still passing or still failing. This property can be used to select which
  tests to run in order to validate a given change. In the absence of
  hermeticity guarantees, the next best alternative is to run all the tests,
  which is costly.
- **Reproducible**: re-running the same test should produce the same result.
  Isolation and hermeticity improve reproducibility. The larger the scope of the
  test, the more difficult it is for the test to be reproducible.
- **Proximity to the code under test**: tests should focus on a particular unit
  and test, and control for what’s not under test.
- **Resilient**: tests shouldn’t need to change when the code under test
  changes, unless the change is important to the code’s purpose. Tests that
  continue to work after benign changes to the code are more resilient. Tests
  that exercise the code’s public APIs or other forms of contracts tend to be
  more resilient. This also happens when you focus on testing behavior, not
  implementation.
- **Easy to troubleshoot**: when tests fail, they should produce clear
  actionable errors that identify the defect. Tests that isolate errors, or
  otherwise have a smaller scope, are usually easier to troubleshoot when they
  fail.
- **Fast**: tests are often part of the developer feedback loop. A faster
  feedback loop is a better feedback loop that makes developers more productive.
- **Reliable**: test failure should indicate a real defect with the code under
  test. Reliable tests give more confidence when they pass and produce fewer
  false failures that are costly to maintain.
- **Flexible**: the fewer constraints there are on running tests, the easier it
  is to run them. On Fuchsia we particularly appreciate if tests can run on
  emulators, when possible.

## Undesirable properties of tests

The properties below are generally bad to have. Depending on circumstances they
may be the downsides of a tradeoff that was made for the purpose of the test,
which as a whole is a net positive.

- **Flaky**: tests that produce false failures, then pass when they’re retried,
  are flaky. Flaky tests are costlier to maintain, slower to run due to retries,
  and provide lower confidence results.
- **Slow**: tests that take longer to run create less efficient feedback loops
  and make developers less productive. The bigger the scope of the test, the
  slower it is usually to run.
- **Difficult to troubleshoot**: tests that fail with errors that are not
  immediately actionable or otherwise don’t indicate the root cause of the
  failure are more difficult to troubleshoot. Developers have to look elsewhere
  other than the test failure itself, such as at system logs or internal system
  state, to troubleshoot the test failure.
- **Change detectors**: tests that are coupled too closely with implementation
  details that aren’t important for functionality will often fail when the code
  under test changes in ways that are benign to the external observer. Change
  detector tests are more costly to maintain.

## Test against interfaces and contracts

<span class="compare-better">Recommended</span>: Test using public APIs and
other interfaces and contracts offered to the client of the code under test.
These tests are more resilient to benign changes.

<span class="compare-worse">Not recommended</span>: Don’t test implementation
details that are not important to the client. Such tests often break when the
code under test changes in benign ways.

Further reading:

- [Change-Detector Tests Considered Harmful](https://testing.googleblog.com/2015/01/testing-on-toilet-change-detector-tests.html){:.external}
- [Prefer Testing Public APIs Over Implementation-Detail Classes](https://testing.googleblog.com/2015/01/testing-on-toilet-prefer-testing-public.html){:.external}
- [Test Behavior, Not Implementation](https://testing.googleblog.com/2013/08/testing-on-toilet-test-behavior-not.html){:.external}
- [Testing State vs. Testing Interactions](https://testing.googleblog.com/2013/03/testing-on-toilet-testing-state-vs.html){:.external}

## Write readable test code

Consider readability as you write tests, the same as you do when you write the
code under test.

- A test is **complete** if the body of the test contains all the information
  you need to know in order to understand it.
- A test is **concise** if the test doesn’t contain any other distracting
  information.

<span class="compare-better">Recommended</span>: Write test cases that are
complete and concise. Prefer writing more test individual test cases, each with
a narrow focus on specific circumstances and concerns.

<span class="compare-worse">Not recommended</span>: Don’t combine multiple
scenarios into fewer test cases in order to produce shorter tests with fewer
test cases.

Further reading:

- [What Makes a Good Test?](https://testing.googleblog.com/2014/03/testing-on-toilet-what-makes-good-test.html){:.external}
- [Keep Tests Focused](https://testing.googleblog.com/2018/06/testing-on-toilet-keep-tests-focused.html){:.external}

## Write reproducible, deterministic tests

Tests should be deterministic, meaning every run of the test against the same
revision of code produces the same result. If not, the test may become costly
to maintain.

Threaded or time-dependent code, random number generators (RNGs), and
cross-component communication are common sources of nondeterminism.

<span class="compare-better">Recommended</span>: Use these tips to write
deterministic tests:

  - For time-dependent tests, use fake or mocked clocks to provide
    determinism. See [`fuchsia_async::Executor::new_with_fake_time`] and
    [fake-clock].
  - Threaded code must always use the proper synchronization primitives to
    avoid flakes. Whenever possible, prefer single-threaded tests.
  - Always provide a mechanism to inject seeds for RNGs and use them in
    tests.
  - Use mocks in component integration tests. See [Realm Builder][realm-builder].
  - When working with tests that are sensitive to flaky behavior,
    consider running tests multiple times to ensure that they consistently pass.
    You can use repeat flags, such as[`--gtest_repeat`][gtest_test_flags]{:.external}
    in GoogleTest and [`--test.count`][go_test_flags]{:.external} in Go, to do this.
    Aim for at least 100-1000 runs locally if your test is prone to flakes before
    merging.

<span class="compare-worse">Not recommended</span>: Never use `sleep` in tests
as a means of weak synchronization. You may use short sleeps when polling in a
loop, between loop iterations.

## Test doubles: stubs, mocks, fakes

Test doubles stand in for a real dependency of the code under test during a
test.

- A **stub** is a test double that returns a given value and contains no logic.
- A **mock** is a test double that has expectations about how it should be
  called. Mocks are useful for testing interactions.
- A **fake** is a lightweight implementation of the real object.

<span class="compare-better">Recommended</span>: Create fakes for code that you
own so that your clients can use that as a test double in their own tests. For
integration testing, consider making it possible to run an instance of your real
component in a test realm in isolation from the rest of the system, and document
this behavior.

<span class="compare-worse">Not recommended</span>: Don’t overuse mocks in your
tests, as you might create lower-quality tests that are less readable and more
costly to maintain while providing less confidence when they pass. Avoid mocking
dependencies that you don’t own.

Further reading:

- [Know Your Test Doubles](https://testing.googleblog.com/2013/07/testing-on-toilet-know-your-test-doubles.html){:.external}
- [Don’t Overuse Mocks](https://testing.googleblog.com/2013/05/testing-on-toilet-dont-overuse-mocks.html){:.external}
- [Don’t Mock Types You Don’t Own](https://testing.googleblog.com/2020/07/testing-on-toilet-dont-mock-types-you.html){:.external}

## Use end-to-end tests appropriately

<span class="compare-better">Recommended</span>: Use end-to-end tests to test
critical user journeys. Such tests should exercise the journey as a user, for
instance by automating user interactions and examining user interface state
changes.

<span class="compare-worse">Not recommended</span>: Don’t use end-to-end tests
to cover for missing tests at other layers or smaller scopes, since when those
end-to-end tests catch errors they will be very difficult to troubleshoot.

<span class="compare-better">Recommended</span>: Use end-to-end tests sparingly,
as part of a balanced testing strategy that leans more heavily on smaller-scoped
tests that run quickly and produce precise and actionable results.

<span class="compare-worse">Not recommended</span>: Don’t rely on end-to-end
tests in your development feedback cycle, because they typically take a long
time to run and often produce more flaky results than smaller-scoped tests.

Further reading:

- [Testing UI Logic? Follow the User!](https://testing.googleblog.com/2020/10/testing-on-toilet-testing-ui-logic.html){:.external}
- [Just Say No to More End-to-End Tests](https://testing.googleblog.com/2015/04/just-say-no-to-more-end-to-end-tests.html){:.external}
- [Test Flakiness - One of the main challenges of automated testing](https://testing.googleblog.com/2020/12/test-flakiness-one-of-main-challenges.html){:.external}
- [Test Flakiness - One of the main challenges of automated testing (Part II)](https://testing.googleblog.com/2021/03/test-flakiness-one-of-main-challenges.html){:.external}
- [Avoiding Flakey Tests](https://testing.googleblog.com/2008/04/tott-avoiding-flakey-tests.html){:.external}
- [Where do our flaky tests come from?](https://testing.googleblog.com/2017/04/where-do-our-flaky-tests-come-from.html){:.external}

[test-scope]: contribute/testing/scope.md
[testing-principles]: contribute/testing/principles.md
[audio-effects-example-tests]: /src/media/audio/examples/effects/test/audio_effects_example_tests.cc
[build-bringup]: development/build/build_system/bringup.md
[capabilities-protocol]: concepts/components/v2/capabilities/protocol.md
[cf]: concepts/components/v2/README.md
[cf-capabilities]: concepts/components/v2/capabilities/README.md
[cf-manifests]: concepts/components/v2/component_manifests.md
[channel]: reference/kernel_objects/channel.md
[continuous-integration]: https://martinfowler.com/articles/continuousIntegration.html
[contract-test]: https://martinfowler.com/bliki/ContractTest.html
[coverage-no-e2e]: contribute/testing/coverage.md#end-to-end_e2e_tests_exclusion
[cpuperf]: /garnet/bin/cpuperf/README.md
[create-e2e-test]: development/testing/create_a_new_end_to_end_test.md
[cts]: /sdk/cts/README.md
[dependency-injection]: https://en.m.wikipedia.org/wiki/Dependency_injection
[e2e-perf]: /src/tests/end_to_end/perf/README.md
[fidl]: concepts/fidl/overview.md
[fidl-benchmarks]: /src/tests/benchmarks/fidl/benchmark_suite/
[fidl-compatibility-tests]: /src/tests/fidl/compatibility/README.md
[fidl-wire-format]: reference/fidl/language/wire-format
[fonts-tests-integration]: /src/fonts/tests/integration/README.md
[fsi]: concepts/packages/system.md
[fuchsia.pkg.fontresolver]: https://fuchsia.dev/reference/fidl/fuchsia.pkg#FontResolver
[fuzzing]: development/testing/fuzzing/overview.md
[gidl]: /tools/fidl/gidl/README.md
[inspect]: development/diagnostics/inspect/README.md
[inspect-codelab]: development/diagnostics/inspect/codelab/codelab.md
[inspect-validator]: reference/diagnostics/inspect/validator/README.md
[inspect-vmo-format]: reference/diagnostics/inspect/vmo-format.md
[inspect-vmo-format-update]: reference/diagnostics/inspect/updating-vmo-format.md
[minfs]: concepts/filesystems/minfs.md
[minfs-stress]: /src/storage/stress-tests/minfs/
[multi-repo-dev]: https://testing.googleblog.com/2015/05/multi-repository-development.html
[netstack-benchmarks]: /src/connectivity/network/tests/benchmarks/README.md
[netstack3-roadmap]: contribute/roadmap/2021/netstack3.md
[practical-test-pyramid]: https://martinfowler.com/articles/practical-test-pyramid.html
[principles]: concepts/index.md
[principles-inclusive]: concepts/principles/inclusive.md
[principles-pragmatic]: concepts/principles/pragmatic.md
[principles-secure]: concepts/principles/secure.md
[principles-updatable]: concepts/principles/updatable.md
[reader-fuzzer]: /zircon/system/ulib/inspect/tests/reader_fuzzer.cc
[realm-builder]: development/testing/components/realm_builder.md
[run-e2e-test]: development/testing/run_an_end_to_end_test.md
[run-test-component]: development/run/run-test-component.md
[rust-stress-test-lib]: development/testing/rust_stress_test_library.md
[sanitizers]: contribute/testing/sanitizers.md
[sanitizers-supported-configs]: contribute/testing/sanitizers.md#supported_configurations
[screen-is-not-black]: /src/tests/end_to_end/screen_is_not_black/README.md
[stress-tests]: development/testing/stress_tests.md
[syscalls]: reference/syscalls/README.md
[test-coverage]: contribute/testing/coverage.md
[test-package-gn]: development/components/build.md#test-packages
[testing-integration]: development/testing/components/integration_testing.md
[testing-v2]: development/testing/components/README.md
[timer-slack]: concepts/kernel/timer_slack.md
[timer-tests]: /zircon/kernel/tests/timer_tests.cc
[timers-test]: https://fuchsia.googlesource.com/fuchsia/+/main/src/zircon/tests/timers/timers.cc
[userboot]: concepts/process/userboot.md
[utest-core]: /zircon/system/utest/core/README.md
[vdso]: concepts/kernel/vdso.md
[wikipedia-dependency-injection]: https://en.m.wikipedia.org/wiki/Dependency_injection
[`fuchsia_async::Executor::new_with_fake_time`]: https://fuchsia.googlesource.com/fuchsia/+/a874276/src/lib/fuchsia-async/src/executor.rs#345
[fake-clock]: https://fuchsia.googlesource.com/fuchsia/+/a874276/src/lib/fake-clock
[rust_65218]: https://github.com/rust-lang/rust/issues/65218
[go_test_flags]: https://golang.org/cmd/go/#hdr-Testing_flags
[gtest_test_flags]: https://github.com/google/googletest/blob/main/docs/advanced.md#repeating-the-tests