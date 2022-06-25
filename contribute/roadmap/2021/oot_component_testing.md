# Out-of-tree component testing support

- Project leads: shayba@google.com, crjohns@google.com
- Area: Testing

## Problem statement

### Missing platform surface for testing

Software developers targeting Fuchsia can write [components],
[build them][components-build], and [test them][components-test]. However these
critical developer journeys are fully and continuously tested only for
[in-tree][glossary.in-tree] developers, and not at all for
[out-of-tree][glossary.out-of-tree] developers.

Today there are several teams that develop and test components out-of-tree. We
sometimes refer to these teams as “partners”, because the Fuchsia team maintains
a close engagement with them. This arrangement is costly to maintain and
impossible to scale for the following reasons:

- Out-of-tree testing relies on deprecated platform features, protocols, and
  tools. Most notably: using the SSH protocol to issue commands on the target
  (such as with [`fx shell`][fx-shell] or [`fssh`][fssh]), [`dash`][dash] as a
  system interface, [`fx log`][fx-log] to collect system-wide logs during the
  test, and using the SCP protocol (such as with [`fx scp`][fx-scp]) to collect
  other test artifacts as side effects on a global mutable filesystem. These
  produce unreliable behavior that leads to flakes and is difficult to
  troubleshoot, which can at least be partially attributed to the brittle nature
  of text-based protocols. Furthermore the transport affords for a single
  character stream, a great fit for a system-wide log (e.g. serial log or
  syslog) but not for multiple log streams or large binary artifacts such as
  state dumps and screenshots from tests.

- Various text-based protocols that lack methods and practices for ensuring ABI
  stability or orchestrating ABI evolution.

- Tests written as [legacy components][legacy-components] aka CFv1 which enjoy a
  lesser degree of isolation, suffer from a greater degree of flakiness, and
  don’t benefit from [new testing tools][realm-builder].

- Bespoke, inconsistent rules and scripts for defining tests and capturing their
  results and associated diagnostics.

- Inconsistent support for various test frameworks. For instance, while all
  out-of-tree partners support C++ and GoogleTest, only some partners support
  Dart, and none support Rust despite its popularity for in-tree component
  development.

- Inconsistent support for additional instrumentation that adds more value to
  tests, such as [sanitizers] and [coverage].

- Some tools and source code is distributed to partners not via the
  [Fuchsia IDK][idk], the set of [SDK tools][sdk-tools], or specific frontends
  such as the [Fuchsia GN SDK][gn-sdk]. For instance the
  [`TestWithEnvironment`][testwithenvironment] helper class is
  [manually copied to the Flutter repository on GitHub][fxb-73984] to unblock
  integration testing needs.

To overcome these issues, the Fuchsia team has offered dedicated support to key
partners. This arrangement often produces tailored solutions that aren’t
portable between different customers. Furthermore, support for customer issues
often happens inside the customer’s source repository, which may be convenient
for the customer but doesn’t scale to supporting a general public audience of
developers.

We now have a breadth of experiences and observations from the wild to inform us
on how to create more generalized testing solutions. The time is right to take
these insights and build out platform support for these use cases, thus creating
a more capable SDK as well as reducing and removing bespoke solutions that carry
a fixed maintenance cost over a diminishing value proposition.

### Platform/Infra surface for testing

Fuchsia’s in-house testing infrastructure (aka “infra”) exhibits most of the
same problems listed above and is affected in similar ways to Fuchsia partners.
Since Fuchsia’s infra doesn’t continuously exercise platform solutions and SDK
tools for testing, there is a missed opportunity for continuous quality
assurance for said solutions and tools.

### A growing need for out-of-tree testing

There are several current and upcoming projects that are expected to increase
the scope of out-of-tree development & testing targeting Fuchsia. These include:

- *[Compatibility Test Suite][rfc-0015]* (CTS) tests will be able to run outside
  of Fuchsia’s in-tree build & test system, though their source code will be
  hosted on [fuchsia.git][fuchsia-git].

- *[Flutter-on-Fuchsia Velocity][fof-velocity]* expects to build and test a
  Flutter embedder on Fuchsia and a Flutter runner for components and their
  tests out-of-tree, with at least some integration tests being upstreamed to
  the Flutter project.

- *[Drivers as Components][dac]* will include a demonstration of a driver built
  and tested out-of-tree, to realize the promise of
  robust hardware support on Fuchsia] via
  [driver ABI stability][driver-abi-stability].

- *[Workstation out-of-tree][workstation-out-of-tree]* expects to test
  components out-of-tree that are written in a variety of languages and on a
  variety of virtual and physical devices.

- *Support for running existing tests on Fuchsia in [LLVM]{:.external} and
  [Rust]{:.external} projects* will require out-of-tree C++ and Rust testing
  support.

Currently these projects cannot be successfully completed as they depend on
missing support for out-of-tree testing.

## Solution statement

We will create a platform solution for testing that works exclusively based on
tools and protocols that are publicly available in the Fuchsia SDK.

### Host-side

We will use [FFX] as the entry point for out-of-tree testing. We will finish
developing [`ffx test`][ffx-test] to handle all host-side aspects of testing. We
will rely on the established FFX technologies and practices, such as
configuration management, target device discovery, and the [Overnet]
communication suite.

We will replace existing host tools with `ffx` tools. Tools such as
[`testrunner`][testrunner] and [Botanist][botanist] currently perform tasks in
Fuchsia CI/CQ - such as device discovery, device setup, test orchestration, test
artifact collection - that can be incrementally handed off to `ffx`. Some of
these handoffs will require building equivalent `ffx` plugins up to parity, for
instance bringing up `ffx test` support for running Bringup tests over a serial
connection. The payoff is that we'd get to continuously verify our work on modern
tools that are portable between in-tree and out-of-tree use cases using our
existing rich and robust corpus of in-tree tests and in-tree automation.

We will port aspects of working with [sanitizers] and [test coverage][coverage]
from tools that are only available in-tree such as [tefmocheck] and [covargs] to
[ffx plugins][ffx-plugin].

### Target-side

We will grow the [Test Runner Framework][trf] (TRF) to accommodate for the needs
of out-of-tree testing.

TRF includes an on-device Overnet daemon, a component to manage/schedule tests,
an isolated realm for hermetic testing, a selection of test runners that support
a variety of languages and frameworks for writing tests, and [FIDL] protocols to
connect all of the above. TRF supports both in-tree and out-of-tree testing
workflows. It replaces a test runtime that only worked in-tree and only
supported CFv1 components.

The priority customer for TRF so far has been in-tree testing, with success
measured in terms of the portion of tests that run on TRF. At the time of
writing more than 70% of Fuchsia in-tree tests have been migrated to TRF, with
modern (CFv2) tests running exclusively on TRF. By the end of 2021 we expect all
remaining tests except [ZBI tests][fx-run-zbi-test] to run under TRF, thanks to
an upcoming compatibility layer.

Once all component tests are migrated to TRF, we will turn down the legacy
target-side v1-only in-tree test runtime. This will allow us to focus on
improving the new testing runtime, improvements that will benefit both in-tree
and out-of-tree developers.

To improve the developer experience, out-of-tree developers will be able to
[include test runners from the SDK][rfc-0106]. Through this mechanism,
out-of-tree developers will have access to the existing inventory of test
runners - [gtest][runner-gtest], [rust][runner-rust], [Go][runner-go], and
arbitrary [ELF][runner-elf] binaries - as well as the upcoming Dart and Flutter
test runners. TRF will also provide the foundation for developing more advanced
testing strategies, such as [stress tests][stress-tests] and
[CTS tests][rfc-0015]. These testing strategies will be expressed as runners
that may also be provided in the SDK.

In addition, out-of-tree developers will be able to create and use test runners
of their own. This is thought to be feasible today, but it has not been
demonstrated yet. We should create a first out-of-tree test runner so we can
speak about this workflow with more confidence.

### Test execution control

We will finish the development and rollout of new protocols to control test
execution.

The new protocols are defined in [FIDL] form (allowing for ABI stability &
evolution, crucial for out-of-tree) and are natively carried by [Overnet]. The
new protocols don’t have specific knowledge of Fuchsia infra, therefore
sharpening the so-called platform/infra contract.

The new protocols allow for better layering and separation of concerns. For
instance the host side is responsible for test selection and requesting
execution on the target. The target-side test manager takes responsibility for
actually running the tests, as opposed to the legacy system in which a host tool
manually executes each test individually on a target device. The responsibility
of parallelizing test execution to maximize resource utilization is transferred
to the target, which is better equipped to handle this responsibility.

Lastly, the new protocols are not based on a character stream (SSH). This allows
for information to flow both ways with multiple streams of instructions,
results, and diagnostics, as well as large test inputs and outputs that may be
in binary format.

### Test results

Test results will no longer be constrained to a suite-level pass/fail outcome,
but will be enumerated in fine detail and with structured format. Diagnostics
collected during the test, such as logs captured from the test realm throughout
the duration of the test, will be organized in a manner associated with the
tests that produced them. There will be standard support for other artifacts
from tests, such as profiles collected during test runtime or large test outputs
such as screenshots taken during the test. The schema for the result format will
be published in the SDK to support processing by out-of-tree tools.

### Documentation

Developer guides such as [this][troubleshooting-run-test-component] will be
reviewed, edited, and simplified to be useful to out-of-tree developers. In-tree
and out-of-tree testing workflows will be unified such that these guides won’t
have information specific to Fuchsia in-tree/out-of-tree developers, or separate
sections for such different audiences.

A new onboarding guide detailing the "Journey of Testing" will be developed.
This guide will provide an entrypoint to developers who need to ensure their
code is properly tested with both unit and integration tests. The goals of this
guide are to 1) aid developers in making the correct choice about which type of
test to write, and 2) quickly bring developers to a point where their tests are
prepared to take advantages of more advanced testing strategies (such as
[CTS][rfc-0015] and [Stress Testing][stress-tests]).

### Virtualization support

Virtual targets such as emulators are very useful and popular for testing.
Fuchsia currently offers to [download a distribution of qemu][qemu-download]
that has been tested to work with Fuchsia. However there are additional tools
for working with virtualized targets, such as [`fx qemu`][fx-qemu] and
[`fx gce`][fx-gce], that are only available in-tree.

We will map the gap between in-tree and out-of-tree support for running Fuchsia
on virtualized targets, and address it to close testing workflow gaps as needed.

## Dependencies

- The FFX tool and associated stack.
- The Fuchsia IDK, and any SDK frontends used by out-of-tree developers.
- The ongoing [Components v2 migration][cfv2-migration] project as it enables
  out-of-tree component development.
- Making [RealmBuilder][realm-builder] available out-of-tree.
- Exposing [RealmBuilder][realm-builder] via the SDK. This includes the
  underlying protocol, and at least one client library.
- Extending RealmBuilder to [support additional
  languages][realm-builder-languages].

## Risks and mitigations

At the time of writing, the most risky dependency is the
[Components v2 migration][cfv2-migration] effort. This is a critical dependency
since out-of-tree components are v2 components moving forward, and the
[Test Runner Framework][trf] assumes that tests are v2 components. The v2
migration project is a multi-year migration to pay back multi-year tech debt.
Ongoing work [over the past year][cfv2-roadmap-2020] exposed a long tail of
issues that need to be addressed.

The risk is primarily of overloading the Component Framework team with work
that’s needed to unblock many other teams. To mitigate this risk, we are
distributing and sharing this work across multiple stakeholder teams.

Specifically the work to introduce
[more language support to RealmBuilder][realm-builder-languages] can easily be
distributed, since the underlying protocols that RealmBuilder client libraries
are using are already stable. Though there may be centralized prerequisite tasks
such as API polish.

## Not in scope

### End-to-end testing (aka system testing)

This proposal focuses on component testing, which takes the form of unit testing
a single component or integration testing that spans multiple components. System
tests, also known as end-to-end (or e2e) tests, don't exercise specific
component instances but rather the entire system under test. As such they differ
in many ways from component tests such as in developer needs and use cases, and
in the platform's ability to offer isolation between e2e tests.

The current popular solution for e2e tests is
[SL4F (Scripting Layer for Fuchsia)][sl4f].
The implementation includes a target-side daemon that is included in
some Fuchsia images and can perform a documented set of system automation tasks,
and a [client library written in Dart][sl4f-client] that is available in the
Fuchsia SDK.

Additionally, there exist CFv1 component tests that are arguably system tests.
This is possible because the legacy CFv1 test runtime allows
[accessing real system services][v1-test-system-services], as a legacy
compromise that is intentionally not supported in CFv2 testing. For the purpose
of this discussion we consider such strictly non-hermetic CFv1 component tests
to be system tests as well.

Current challenges in scaling e2e test development include:

- Adding facades to SL4F requires changing platform code and redistributing
  Fuchsia system images. Out-of-tree developers cannot extend SL4F's
  capabilities to automate a system.

- SL4F does not rely on ffx, but rather uses its own transport layer, protocols,
  target discovery, and configuration. These differences add to the ongoing
  maintenance cost and introduce inconsistencies and friction in the developer
  experience.

- Only a Dart client library is provided.

- A [helpful developer guide][sl4f-guide] is provided, but instructions are
  given for developing SL4F tests in-tree only.

Since system tests are so uniquely different than component tests, this topic is
covered in a [separate roadmap document][oot-system-testing].

[botanist]: /tools/botanist/
[cfv2-migration]: contribute/open_projects/components/migration.md
[cfv2-roadmap-2020]: contribute/roadmap/2020/overview.md#components_v2
[components]: concepts/components/v2/introduction.md
[components-build]: development/components/build.md
[components-test]: development/testing/components/README.md
[covargs]: /tools/debug/covargs/
[coverage]: contribute/testing/coverage.md
[dac]: contribute/roadmap/2020/overview.md#implementing_drivers_as_components
[dash]: /zircon/third_party/uapp/dash/
[driver-abi-stability]: contribute/roadmap/2021/stable_driver_runtime.md
[ffx]: development/tools/ffx/getting-started.md
[ffx-plugin]: development/tools/ffx/development/plugins.md
[ffx-test]: https://fuchsia.dev/reference/tools/sdk/ffx.md#test
[fidl]: /docs//concepts/fidl/overview.md
[fof-velocity]: contribute/roadmap/2021/flutter_on_fuchsia_velocity.md
[fssh]: https://fuchsia.dev/reference/tools/sdk/fssh.md
[fuchsia-git]: https://fuchsia.googlesource.com/fuchsia
[fx-gce]: https://fuchsia.dev/reference/tools/fx/cmd/gce
[fx-log]: https://fuchsia.dev/reference/tools/fx/cmd/log
[fx-qemu]: https://fuchsia.dev/reference/tools/fx/cmd/qemu
[fx-run-zbi-test]: https://fuchsia.dev/reference/tools/fx/cmd/run-zbi-test
[fx-scp]: https://fuchsia.dev/reference/tools/fx/cmd/scp
[fx-shell]: https://fuchsia.dev/reference/tools/fx/cmd/shell
[fxb-73984]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=73984
[glossary.in-tree]: glossary/README.md#in-tree
[glossary.out-of-tree]: glossary/README.md#out-of-tree
[gn-sdk]: development/idk/gn/README.md
[idk]: development/idk/README.md
[legacy-components]: concepts/components/v1/README.md
[llvm]: https://llvm.org/
[oot-system-testing]: contribute/roadmap/2021/oot_system_testing.md
[overnet]: /src/connectivity/overnet/README.md
[qemu-download]: development/idk/download.md#qemu
[realm-builder]: development/testing/components/realm_builder.md
[realm-builder-languages]: development/testing/components/realm_builder.md#language-feature-matrix
[rfc-0015]: contribute/governance/rfcs/0015_cts.md
[rfc-0106]: contribute/governance/rfcs/0106_manifest_includes_in_sdk.md
[runner-elf]: development/testing/components/test_runner_framework.md#elf-test-runner
[runner-go]: development/testing/components/test_runner_framework.md#gotest-runner
[runner-gtest]: development/testing/components/test_runner_framework.md#gtest-runner
[runner-rust]: development/testing/components/test_runner_framework.md#rust-runner
[rust]: https://www.rust-lang.org/
[sanitizers]: contribute/testing/sanitizers.md
[sdk-tools]: https://fuchsia.dev/reference/tools/sdk/README.md
[sl4f]: development/drivers/concepts/driver_development/sl4f.md
[sl4f-client]: /sdk/testing/sl4f/client/
[sl4f-guide]: development/testing/create_a_new_end_to_end_test.md
[stress-tests]: development/testing/stress_tests.md
[tefmocheck]: /tools/testing/tefmocheck/README.md
[testrunner]: /tools/testing/testrunner/README.md
[testwithenvironment]: /sdk/lib/sys/cpp/testing/test_with_environment.h
[trf]: development/testing/components/test_runner_framework.md
[troubleshooting-run-test-component]: development/testing/components/test_runner_framework.md#troubleshooting
[v1-test-system-services]: concepts/testing/v1_test_component.md#additional_system_services
[workstation-out-of-tree]: contribute/roadmap/2021/workstation_out_of_tree.md
