# Out-of-tree system testing support

- Project leads: shayba@google.com, ananthak@google.com, fsamuel@google.com
- Project partners: borthakur@google.com, jasoncampbell@google.com,
  lite@google.com, ppi@google.com
- Area: Testing

## Problem statement

The needs of Fuchsia platform and product developers for out-of-tree system
testing are currently either unmet, or are met with the wrong tools, resulting
in missing test coverage or low-quality tests.

Fuchsia-based product development, which happens outside of the Fuchsia source
tree (out-of-tree, or OOT), leans heavily on unsandboxed system tests that
exercise the Fuchsia platform outside of the platform’s strictly defined
contracts. This is done because platform components mostly don’t offer supported
ways to perform the instrumentation that’s necessary to meet the needs of
product testing. Test authors have been able to find ways around these
limitations, but these solutions lead to low-quality tests.

Subjecting the development process of the Fuchsia platform to OOT tests that are
able to circumvent platform contracts threatens Fuchsia’s ability to be
[updatable][fuchsia-updatable].

### What are system tests?

System testing is testing that’s conducted on a complete assembled system in
order to validate that the system meets certain requirements. In a
[practical test pyramid][practical-test-pyramid]{:.external}, system tests
complement unit tests and integration tests to fill the testing gaps that can
only be addressed by observing a full system under test.

System tests are sometimes also referred to as:

- End-to-end (or e2e) tests, especially if the scope of the system test exceeds
  a particular target device to also include, for instance, a remote server over
  a network interface, or a controlling host machine.

- Critical User Journey (CUJ) tests, especially when the test is expressed in
  terms of simulated and automated user inputs and outputs, for instance by
  injecting input events such as button presses and comparing UI state summaries
  or screenshots to an expected test outcome.

In addition these tests can be further instrumented to produce additional value,
expressed as:

- Performance tests, where in addition to exercising a product CUJ the test
  harness also collects performance information such as timing, traces, FPS
  statistics etc’.

- Longevity tests, where the same CUJ is exercised in a tight loop and the
  system is monitored for signs of stress such as resource leaks (RAM, handles,
  etc’) or crashes.

There is also an entire category of system tests that exercise platform
functionality and are developed in-tree. These are [out of scope](#out-of-scope)
for this document.

### System testing challenges on Fuchsia today

#### Non-hermetic legacy (v1) component tests

Fuchsia’s component framework offers [extensive testing support][v2-testing]
with a high degree of isolation between the test environment and the rest of the
system. Legacy (CFv1) testing is still supported to test legacy components that
are not yet [migrated][v2-migration] to CFv2. Though the majority of production
components are still CFv1, most tests (>70%) use the CFv2 test framework since
it’s more reliable and offers some new features for developers.

For legacy reasons the v1 test runtime is not hermetic in many ways, for
instance by
[allowing access to certain real system services][v1-test-system-services]. As a
result, inadvertently many CFv1 tests actually behave as system tests. These
tests suffer from multiple issues, including:

- Test failures can be difficult to troubleshoot, since the full scope of the
  test is very broad or not strictly defined.

- Tests can be influenced by outside state, or leave outside state as side
  effects. As a result, these tests can cross-talk with other such system tests,
  causing for instance failures that don’t reproduce (“flakes”) or that
  reproduce under unstable conditions, for instance by re-running the same tests
  but in a different order.

- Tests that assume implementation details of other system components beyond
  their stated contract.

- The component testing framework is designed to be used to write isolated
  hermetic unit tests and integration tests. In such tests, any cause of failure
  can only come from within the test realm. Due to this expectation of
  isolation, it’s also expected that any two tests can be run concurrently or in
  any sequence without affecting their results. Using deprecated CFv1 features
  to break the testing sandbox and write system tests breaks those guarantees
  and creates difficult troubleshooting situations. Many authors of said tests
  don’t realize that they are in fact system tests.

#### OOT CUJ tests that breach the Fuchsia System Interface

For OOT software, the intended and supported way to interact with Fuchsia
platform is via the [Fuchsia System Interface][fsi] (FSI). However there exist
tools for OOT developers today that allow test authors to sidestep this
interface and to violate platform-product contracts.

[Scripting Layer for Fuchsia (SL4F)][sl4f] is a system automation framework that
was developed to write comprehensive system tests.

SL4F is inspired by [Scripting Layer for Android (SL4A)][sl4a]. It was
originally intended for in-tree platform system tests. Particularly SL4F is
useful for porting such things as the
[Android Comms Test Suite (ACTS)][acts]{:.external} tests, which speak the same
underlying JSON-RPC/HTTPS protocol to drive the target device. This arrangement
has been very useful for Fuchsia connectivity testing.

As a system automation framework, SL4F can also be used to test CUJs. For
instance SL4F powers a platform CUJ tests that ensure that
[the device doesn't boot into a black screen][e2e-not-black-screen].

However, SL4F was not designed for OOT testing. Interacting with SL4F is done
over protocols that are outside the FSI, and don't offer the same evolution
mechanisms as are offered by FIDL for instance. Extending the automation
capabilities of SL4F can be done by introducing new facades, but all facades
must be developed and built in-tree. Therefore when testing CUJs for products
that are defined and developed OOT, using SL4F gives rise to some of the issues
listed further below.

Another common mechanism that allows for tests that are too invasive is the use
of [SSH] to gain a remote shell and to copy files between the host and the
target. This is not to be confused with the use of SSH as a tunneling protocol,
which is useful for instance as a transport for Overnet.

Engineering builds of Fuchsia today include an SSH daemon that runs with
unsandboxed access to global namespaces and serves the [`dash`][dash] shell to
the client. The same daemon also allows [SCP] functionality for a similar degree
of read/write access to global namespaces, for instance global mutable storage.
All too often, this serves as a way around the FSI, allowing test authors to
breach the supported interfaces for platform components by observing or changing
their state in mutable storage, and in doing so relying on platform
implementation details such as names of Fuchsia base packages.

SSH and SCP access are hooked up for easy use by test authors for instance using
the SL4F client library in Dart.

##### Brittle testing patterns

SL4F offers test authors many ways to manipulate and observe system state. Some
of those mechanisms bypass platform-product contracts and necessary abstraction
layers. It’s not strictly necessary to use these mechanisms when writing SL4F
tests, and not all tests necessarily use them. But the welcoming presence of
these invites many anti-patterns into our inventory of SL4F tests.

Notably, these patterns were developed out of necessity, since the platform
failed to offer robust alternatives to meet testing needs. We list these
patterns below not to criticize the platform developers or the test authors, but
for the purpose of understanding and categorizing what is now our technical debt
that we must pay back.

###### Observing state via non-contracts

Code under test may emit information about its state to the system log or via
[Inspect]. These are useful tools for collecting diagnostics for instance into a
[snapshot]. However they’re not designed to be contracts. FIDL is used on
Fuchsia to define strongly-typed contracts that can be stable and have evolution
mechanisms such as binary-compatible changes to the wire format and versioning,
which allows uncoordinated clients and servers to exchange FIDL messages. FIDL
was designed for this purpose with great care, whereas free-text logs and
Inspect were not.

Some specific example that are useful to note

*Logs in tests*: some longevity tests use logs that are annotated at a severity
level of “error” or higher to identify that the product came under unexpected
stress during the test. Unfortunately, an “error” message emitted during the
test’s execution is often actually benign from the test author’s perspective.
Therefore longevity test authors inevitably come to maintain an allowlist of
logged error messages.

*Inspect in tests*: some test drivers read Inspect information from components
that they drive to observe the state of these components. Since Inspect is typed
data, and can be acquired as a single coherent snapshot, it’s a useful tool for
diagnosing the component’s current state by the component’s author. However when
used as a contract between platform components and product tests, it makes for a
brittle ABI and often causes breakages. These breakages are difficult to
troubleshoot since they may happen weeks after the underlying platform change
landed, when an SDK roll is attempted.

###### Manipulating state via non-contracts

SL4F tests are afforded a lot of control over the host, including the ability to
execute arbitrary SSH commands in an unsandboxed shell (i.e. over global
namespaces) and full read/write access to global immutable and mutable storage.
Some key examples:

*Killing and restarting processes*: this is often used in setup and teardown
routines in tests. The intent is positive - the test wants to clear any prior
state and start fresh. However the platform components under test were not
necessarily tested to be robust to being restarted multiple times or in a
different sequence, which often results in flaky behavior.

Another problem with this approach is that by having OOT tests kill platform
processes, the process name - which is not part of the platform’s contract -
becomes a contract. Such violations of intended interfaces and contracts make
platform refactors more difficult, and make OOT tests more brittle.

*Manipulating mutable storage*: this obvious sandboxing violation is commonly
used for instance to inject user credentials as a setup step for certain CUJ
tests. Rather than operate an intended interface for credential injection, the
state is injected ahead of the time that the code under test reads said state.
If the timing isn’t right, the test fails. If cleanup is unsuccessful,
subsequent tests could fail as they’re exposed to test cross-talk.

Another use case for global mutable filesystem access is the use of a global
`/tmp` storage directory as a staging area for test results, artifacts, and
diagnostics - for instance performance traces that were collected during the
test. Again this is an opportunity for tests to fail to clean up state, or to
affect each other via cross-talk, or to inject false or flaky behaviors by other
means.

Isolated storage directories for different component instances are managed on
the same partition, as it's expensive and inflexible to create individual
partitions for different components. Isolation is achieved by creating a certain
directory layout on the shared partition. The directory layout reflects platform
implementation details such as the component topology or how Component Manager
translates that topology into filesystem parts. These are, once again, platform
implementation details that can and will change over time, and should not be
exposed to OOT tests.

###### Outcomes

This unruly mix of open-box testing and closed-box testing in the same test
produces low-quality tests - tests that set irrelevant expectations, and that
orchestrate and observe state in ways that the software under test was not
designed to support.

The present situation evolved out of a long-standing neglect for system testing
tools as a Fuchsia platform offering. Be that as it may, we found ourselves with
many exotic tests that don’t fit into existing categories and defy testing best
practices.

Worse still, the platform implementation details that these tests have come to
rely on are expressed in terms that were not designed to allow for evolution.
For instance much of the FSI is precisely defined in terms of FIDL. FIDL is
designed to allow platform developers to make
[ABI-compatible changes][fidl-abi-compatibility], or to detect when the existing
ABI is broken by a given change. FIDL gives developers many ways to change types
and protocols without breaking the ABI, or to introduce a breakage in a
controlled manner: stable protocol method ordinals, flexible tables, versioning
etc.

Contrast this with the use of process names as a contract, such as for the
purpose of having a test kill a process. The name for a process that's
implemented by a platform component is never part of the FSI, in part because
there is no affordance for evolution - any change is a breaking change, there is
no affordance for versioning, and rarely is it possible to keep both an old and
a new process running for the purpose of a temporary compatibitliy window. The
same applies to global filesystem paths, internal file formats, free text logs,
and other such non-contracts.

##### Missing OOT support

SL4F clients can automate the system by speaking to an SL4F daemon that’s
included in the platform image that’s distributed to OOT testers via Fuchsia SDK
rolls. The daemon can perform different tasks to orchestrate and inspect system
state that are grouped into “facades”. This aspect of the system generally works
well in that it’s a stable contract and there are reasonable means to evolve it
over time.

However, developing facades can only be done in-tree, meaning that OOT there is
currently no possibility to extend the system automation capabilities. This is
not a surprising property of SL4F - it was simply not designed to be used by OOT
clients. This is seen for instance in the handy
[developer guide][new-e2e-test-guide], which references fuchsia.git paths and
`fx` commands.

#### Separate stack

SL4F has solutions for configuration, device discovery, host-target network
transport protocols, some mechanisms for extensibility, and means of delivering
client libraries and tools to SDK customers.

The same problems have also been solved by `ffx`, in arguably better ways. When
developing [`ffx` plugins][ffx-plugins], if the host-side plugin requires a
target-side collaborating component to exert control over the target then a
[FIDL proxy][ffx-proxy-plugin] can be developed. Communication between the host
and the target is then defined in terms of FIDL, which - unlike JSON-RPC over
HTTP - can be part of the FSI, and can evolve as a contract with the rest of the
platform and the SDK. This is in part because
[maintaining ABI compatibility][fidl-abi-compatibility] with FIDL is easier than
with untyped JSON contracts where the schema lives only in code.

Relatedly, the `ffx` stack has a better
[understanding of Fuchsia components][ffx-components], for instance knowing when
and how to start components that are needed for a test. It also solves problems
that the SL4F stack doesn’t even attend to, such as host-target authentication.
This allows using `ffx` in `userdebug` build types, whereas SL4F is only allowed
in `eng` builds.

#### Other custom test harnesses

Additional testing solutions exist OOT that are not listed above. These cover
the full spectrum of testing, though they’re largely orchestrated as system
tests because they don’t utilize the platform’s own isolation mechanisms for
testing.

Some OOT partners share the same set of testing solutions and tools, partially
and inconsistently. This has resulted in growing tech debt.

## Solution statement

Fuchsia platform teams will create robust system testing solutions in-tree and
OOT. Platform teams will work with product teams to understand product testing
needs, meet them, and assist with any migrations that are driven by product
teams.

The new solutions will embrace the sandboxing capabilities of the
[Component Framework][cf] and [`ffx` plugins][ffx-plugins] to:

1. Make it possible to write great tests OOT, including great system tests.
1. Make it more difficult (towards impossible) to write tests OOT that violate
   Fuchsia platform’s strictly-defined and supported contracts.

Specifically:

### Promote componentized unit and integration testing where applicable

Where OOT tests could be reimplemented as components that run in test realms to
produce hermetic unit tests or integration tests, we will reimplement them as
such. In order to enable and promote a healthier testing pyramid, we will bring
support for OOT component testing in parity with in-tree component testing.

### Reimplement all non-hermetic CFv1 tests

All existing legacy CFv1 tests that violate hermeticity by accessing real system
services will be reimplemented in other terms that satisfy the same or greater
testing requirements. Whether these tests are reimplemented as hermetic CFv2
component tests, or as new system tests, or in another form, is up to the owners
of the code under test.

Where platform testing support is missing or insufficient, for instance CFv1
component owners are blocked from migrating to CFv2, the relevant platform teams
will partner with the test owners to create the necessary platform solutions.

### Create a new system testing platform solution for product owners

Fuchsia platform teams will develop a new solution for
[product owners][glossary.product-owner] to write system tests that meets the
following criteria:

1. System tests can be developed and executed OOT.

1. There is precisely one way to invoke these tests and collect their results,
   ensuring that the same workflows are followed and the same consistent
   outcomes are achieved everywhere, whether the tests are exercised in local
   developer workflows and regardless of the developer environment, or whether
   the same tests are exercised in some CI/CQ automation regardless of what
   automation solution is used.

1. Details of the platform that are not part of an intended contract, such as
   what’s defined in the [Fuchsia System Interface][fsi], are not exposed to OOT
   system test developers. To enable the required level of sandboxing, tests and
   testing frameworks will be migrated to CFv2 if they weren't already.

1. The system testing framework shares as much of the stack with `ffx` as is
   relevant. This includes such things as configuration, host tools and client
   library distribution mechanisms, versioning, configuration, target device
   discovery, host-target authentication, host-target transport, and mechanisms
   for remote control.

1. Owners of platform system components can, and will, extend the system testing
   framework to meet the testing needs of product developers. In doing so,
   platform developers will create maintainable ABIs and establish long-term
   ownership for these testability contracts.

### Reduce and eliminate legacy solutions

Platform and product teams will work together to eliminate the use of previous
solutions and align on the new blessed system testing framework everywhere.
Legacy testing solutions will not be used long term, except where exceptional
incentives exist to continue using them, such as for cross-platform test suite
compatibility where desired.

### Prioritization

Work may be prioritized as the teams involved see fit. However we recommend
prioritizing work to first untangle the tests that are most expensive to own and
maintain, such as to attack the head of the technical debt curve first.

Regardless of specific choice of tasks, work on modernizing system testing will
be considered a high priority task when considering what actions can be taken to
pay down technical debt.

## Dependencies

To close platform testability gaps and develop and roll out the new frameworks,
work is required across multiple platform teams including Component Framework,
Testing Architecture, SDK Tools, EngProd Testing, and EngProd Infra.

In addition, work is required across all product partner teams and several
platform component owners to help discover testability gaps and to perform
migrations to new testing frameworks and solutions.

## Risks and mitigations

System tests are not scoped to specific components or packaged, but are scoped
to the entire system under test. Therefore it's not strictly defined what code
is actually being tested. It’s possible that by migrating a system test from any
one framework to another, some beneficial test coverage that was achieved as a
fringe benefit of system testing will be lost.

Some system tests have a very large scope, which sets a very high bar for their
reimplementations. As a useful prefactor step, it may be beneficial to downscope
or split up some of these tests.

Some existing system tests don’t have a dedicated long-term owner. Such
abandonware can be difficult to work with, and some migration work will
inevitably fall on different shoulders.

Aligning on modern solutions, especially OOT, requires work on OOT and
product-side CFv2 migrations. The CFv2 migration has been making great progress,
but so far the focus has been on system components, and migrations of OOT or
product-side components has not yet begun. It’s reasonable to expect unknown
blockers.

Paying tech debt, as always, competes with other team priorities. Leadership
alignment is required to effectively execute this transition.

## Not in scope {#out-of-scope}

### In-tree system testing

Tests that are developed in-tree are out of scope for this document. Although
there is potential windfall from the work described above to favor in-tree
system testing, the problem statement for in-tree testing is sufficiently
different such that it is not discussed here. For instance in-tree tests can
operate below the Fuchsia platform interfaces without compromising on Fuchsia’s
updatability principles and goals.

### Component tests

For component tests, that is tests that are expressed as a set of one or more
components that implement a unit test or an integration test, see the
[roadmap document for out-of-tree component testing support][oot-component-testing].

### Special portability requirements

There are unique circumstances where a pre-existing test suite needs to be run
on Fuchsia to demonstrate compatibility or compliance with another
implementation, or to benchmark against it. In such cases the pre-existing tests
must run unmodified or else compatibility cannot be demonstrated with
confidence. This in turn essentially defines a specification for the device-side
test automation system.

One common solution to this problem is to port the pre-existing test framework
to Fuchsia. For instance, the Fuchsia LLVM Toolchain and Fuchsia Rust Toolchain
teams are planning to port the LLVM and Rust test frameworks respectively so
they can run tests from upstream on Fuchsia. This has the benefit of upgrading
Fuchsia to a higher tier of toolchain support for those external projects
respectively.

Another common solution is to re-implement the test framework against the same
specification. For instance, the Fuchsia Connectivity team implemented SL4F to
the JSON-RPC/HTTPS specification of SL4A so that they could run ACTS tests on
Fuchsia. This has the benefit of bringing a large corpus of useful tests to
Fuchsia, as well as demonstrating compatibility which is critical in the
connectivity domain.

These are fine solutions to this unique problem space, provided that they are
exercised with the right rationale and used in a disciplined way. As a counter
example, if we ported the LLVM testing framework to Fuchsia and then wrote new
tests using this framework that are outside the scope of demonstrating
compatibility or sharing tests with a partner project, that would require
additional justification or otherwise be discouraged by the platform team.

[acts]: https://android.googlesource.com/platform/tools/test/connectivity/+/HEAD/acts
[cf]: /concepts/components/v2/README.md
[dash]: /zircon/third_party/uapp/dash/
[e2e-not-black-screen]: /src/tests/end_to_end/screen_is_not_black/
[ffx-components]: /development/tools/ffx/getting-started.md#interacting_with_components
[ffx-plugins]: /development/tools/ffx/development/plugins.md
[ffx-proxy-plugin]: /development/tools/ffx/development/plugins.md#fidl-proxy
[fidl-abi-compatibility]: /development/languages/fidl/guides/compatibility/README.md
[fsi]: /concepts/packages/system.md
[fuchsia-updatable]: /concepts/principles/updatable.md
[glossary.product-owner]: /glossary/README.md#product-owner
[inspect]: /development/diagnostics/inspect/README.md
[new-e2e-test-guide]: /development/testing/create_a_new_end_to_end_test.md
[oot-component-testing]: /contribute/roadmap/2021/oot_component_testing.md
[practical-test-pyramid]: https://martinfowler.com/articles/practical-test-pyramid.html
[scp]: https://fuchsia.dev/reference/tools/fx/cmd/scp
[sl4a]: https://android.googlesource.com/platform/external/sl4a/
[sl4f]: /development/drivers/concepts/driver_development/sl4f.md
[snapshot]: https://fuchsia.dev/reference/tools/fx/cmd/snapshot
[ssh]: /development/idk/documentation/ssh.md
[v1-test-system-services]: /concepts/testing/v1_test_component.md#additional_system_services
[v2-migration]: /contribute/open_projects/components/migration.md
[v2-testing]: /development/testing/components/README.md
