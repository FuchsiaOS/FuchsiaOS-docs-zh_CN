# Testing principles

Testing is central to the
[continuous integration][continuous-integration]{:.external} processes that
Fuchsia contributors practice to sustain quality and velocity. Good tests become
an asset to the team, while bad tests can turn into a liability.

This document reviews topics related to testing on Fuchsia, and provides
references to additional resources. This document assumes that you are
familiarized with general software testing concepts.

Fuchsia platform testing should serve the goals of the project and align with
[Fuchsia's architectural principles][principles]:

- **[Simple][principles-simple]**: Simple tests are better than complex tests.
  Unit tests are preferred over integration, system, or manual tests. Testing
  solutions on Fuchsia should exercise the same mechanisms that exist in
  production.
- **[Secure][principles-secure]**: Tests are subject to the same security,
  isolation, and hermeticity mechanisms that apply to production software. Tests
  leverage those same mechanisms for their benefit. The security properties of
  the system are testable.
- **[Updatable][principles-updatable]**: It's important to test the stable
  interfaces between [components][glossary.component] and other parts of the
  stable [Fuchsia System Interface][fsi]. If components under tests change or
  are [entirely replaced][netstack3-roadmap], tests that only exercise
  interfaces between components should continue to work. Tests that are outside
  of Fuchsia tree should not assume implementation details of platform
  components.
- **[Performant][principles-performant]**: Tests on Fuchsia should be fast,
  reliable, and flexible. When tests run quickly, it is easier to iterate and
  expend fewer resources. Tests should not be flaky, or picky about how they run
  on Fuchsia. It’s easier to run tests on an emulator than on real hardware.

## How is testing different on Fuchsia?

### Operating systems are complex programs

Every domain of software development and testing brings unique challenges. There
are special problems and solutions to testing an operating system, as there are
for testing a mobile application or server software or a spacecraft.

For instance, tests for the [Zircon kernel][glossary.zircon] run in a special
runtime environment that makes as few assumptions as possible that the kernel is
functional, and can detect mistakes in low-level kernel code. Contrast this with
typical application testing, where the tests run on some operating system that
is assumed to be working.

### Isolation and hermeticity

The [Component Framework][cf] promotes Fuchsia’s security goals by running each
component in a sandbox environment that is strictly-defined in a
[component manifest][cf-manifests]. It then promotes Fuchsia’s updatability
goals by only allowing components to interconnect using those
[component capabilities][cf-capabilities] that are typed as updatable contracts.

These same mechanisms of isolation and hermeticity can also be used by tests as
a form of [dependency injection][wikipedia-dependency-injection]{:.external}.
For instance a component under test can be provided a test double for a
capability that it depends on by a test harness, making
[contract testing][contract-test]{:.external} easier.

### Multiple repositories

Fuchsia is a large project with many external dependencies, and builds against
hundreds of other source code repositories. Multi-repository development
introduces unique challenges to testing. A contributor working on the WebRTC
project published a [blog post][multi-repo-dev]{:.external} detailing many of
the problems and solutions that are also encountered when developing Fuchsia.

## Further reading

* [Testing scope][test-scope]
* [Testing best practices][best-practices]

[fsi]: /docs/concepts/packages/system.md
[netstack3-roadmap]: /docs/contribute/roadmap/2021/netstack3.md
[test-scope]: /docs/contribute/testing/scope.md
[best-practices]: /docs/contribute/testing/best-practices.md
[continuous-integration]: https://martinfowler.com/articles/continuousIntegration.html
[principles]: /docs/concepts/index.md
[principles-simple]: /docs/concepts/principles/simple.md
[principles-secure]: /docs/concepts/principles/secure.md
[principles-updatable]: /docs/concepts/principles/updatable.md
[principles-performant]: /docs/concepts/principles/performant.md
[glossary.component]: /docs/glossary/README.md#component
[glossary.zircon]: /docs/glossary/README.md#zircon
[cf]: /docs/concepts/components/v2/README.md
[cf-capabilities]: /docs/concepts/components/v2/capabilities/README.md
[cf-manifests]: /docs/concepts/components/v2/component_manifests.md
[wikipedia-dependency-injection]: https://en.m.wikipedia.org/wiki/Dependency_injection
[contract-test]: https://martinfowler.com/bliki/ContractTest.html
[multi-repo-dev]: https://testing.googleblog.com/2015/05/multi-repository-development.html
