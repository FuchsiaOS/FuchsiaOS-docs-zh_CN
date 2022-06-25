# Migrate test components

To migrate your test components, follow these steps:

1.  [Migrate the test manifest](#create-test-manifest)
1.  [Update test dependencies](#update-dependencies)
1.  [Migrate component features](#features)
1.  [Verify the migrated tests](#verify-tests)

## Migrate the test manifest {#create-test-manifest}

Find the GN build rules for the tests that exercise your component.
Typically this is a [`fuchsia_test_package()`](#test-package) or
[`fuchsia_unittest_package()`](#unittest-package).

### Unit test packages {#unittest-package}

The preferred practice for tests declared with a `fuchsia_unittest_package()`
build rule is to use the [generated manifest][unit-test-manifests] provided by
the Fuchsia build system.

To allow the GN target to generate your manifest, remove the `manifest`
attribute from the `fuchsia_unittest_package()`:

```gn
fuchsia_unittest_package("my_component_tests") {
  {{ '<strike>' }}manifest = "meta/my_component_test.cmx"{{ '</strike>' }}
  deps = [ ":my_component_test" ]
}
```

Your test package is now able to execute using Components v2 and the
Test Runner Framework.

### Test packages {#test-package}

Consider the following example test component manifest:

```json
// my_component_test.cmx
{
    "include": [
        "syslog/client.shard.cmx"
    ],
    "program": {
        "binary": "bin/my_component_test"
    }
}
```

To migrate this test to the Test Runner Framework, do the following:

1.  Create a CML file that points to the test binary that includes the
    appropriate [test runner][trf-test-runners]:

    Note: See the [available test runners][trf-provided-test-runners] provided
    by the framework.

    ```json5
    // my_component_test.cml
    {
        include: [
            // Select the appropriate test runner shard here:
            // rust, gtest, go, etc.
            "//src/sys/test_runners/rust/default.shard.cml",
            // Enable system logging
            "syslog/client.shard.cml",
        ],
        program: {
            binary: "bin/my_component_test",
        }
    }
    ```

1.  Locate the GN build rule for your test component referenced by the
    `fuchsia_test_package()`:

    ```gn
    fuchsia_component("my_component_test") {
      testonly = true
      manifest = "meta/my_component_test.cmx"
      deps = [ ":bin_test" ]
    }

    fuchsia_test_package("my_component_tests") {
      deps = [ ":my_component_test" ]
    }
    ```

1.  Update your test component's build rule to reference the new CML file:

    ```gn
    fuchsia_component("my_component_test") {
      testonly = true
      {{ '<strong>' }}manifest = "meta/my_component_test.cml"{{ '</strong>' }}
      deps = [ ":bin_test" ]
    }

    fuchsia_test_package("my_component_tests") {
      deps = [ ":my_component_test" ]
    }
    ```

## Update test dependencies {#update-dependencies}

A test may include or depend on components that are separate from the test
component. Here are some things to look for:

-   Does your test have a CMX with [`fuchsia.test facets`][fuchsia-test-facets],
    such as `injected-services` or `system-services`?
-   Does your test create environments in-process? If so, does it create a
    separate environment for each test case?

Note: The Test Runner Framework executes tests within a realm that enforces
hermetic component resolution, which means that test components must resolve
dependencies from within their own package.
For more details, see [hermetic component resolution][hermetic-resolution].

The migration procedure varies depending on the testing framework features in
your v1 component:

-   [Test depends on system services](#system-services): The test has a CMX that
    contains [`system-services`][system-services] test facets.
-   [Test depend on injected services](#injected-services): The test has a CMX that
    contains [`injected-services`][fuchsia-test-facets] test facets.

Note: For more details on the services and capabilities provided to components
by the Test Runner Framework, see the
[test manager documentation][trf-test-manager].

### System service dependencies {#system-services}

For tests that use [`system-services`][system-services] test facets, consider if
they can be converted to [injected services](#injected-services) instead.
Injecting services is the preferred method because it promotes hermetic test
behavior.

For certain non-hermetic tests, the Test Runner Framework provides the test
realm with the following services:

| Service                             | Description                           |
| ----------------------------------- | ------------------------------------- |
| `fuchsia.scheduler.ProfileProvider` | Profile provider for scheduler        |
| `fuchsia.sysmem.Allocator`          | Allocates system memory buffers       |
| `fuchsia.tracing.provider.Registry` | Register to trace provider            |
| `fuchsia.vulkan.loader.Loader`      | Vulkan library provider               |
| `fuchsia.sys.Loader`                | CFv1 loader service to help with      |
:                                     : migration.                            :
| `fuchsia.sys.Environment`           | CFv1 environment service to help with |
:                                     : migration.                            :

Consider the following example test component that uses a single system service,
`fuchsia.sysmem.Allocator`:

```json
// my_component_test.cmx
{
    "facets": {
        "fuchsia.test": {
            "system-services": [
                "fuchsia.sysmem.Allocator"
            ]
        }
    },
    "program": {
        "binary": "bin/my_component_test"
    },
    "sandbox": {
        "services": [
            "fuchsia.sysmem.Allocator"
        ]
    }
}
```

To migrate this test to the Test Runner Framework, declare each available system
service with the other [required services](#required-services) in your test
component manifest. Since this test uses the `fuchsia.sysmem.Allocator`
system capability, it also needs to be marked with `type: "system"` as shown
below.

```json5
// my_component_test.cml

{
    include: [
        // Select the appropriate test runner shard here:
        // rust, gtest, go, etc.
        "//src/sys/test_runners/rust/default.shard.cml",
    ],
    program: {
        binary: "bin/my_component_test",
    },
    {{ '<strong>' }}facets: {
        "fuchsia.test": {
            type: "system"
        },
    },
    use: [
        {
            protocol: [ "fuchsia.sysmem.Allocator" ],
        },
    ],{{ '</strong>' }}
}
```

### Injected service dependencies {#injected-services}

For tests that use other [fuchsia.test facets][fuchsia-test-facets], such as
`injected-services`, your test component manifest must declare each dependent
component and route the provided capabilities to the test component.

In the following example, suppose there's a single injected service,
`fuchsia.pkg.FontResolver`:

```json
// my_component_test.cmx
{
    "facets": {
        "fuchsia.test": {
            "injected-services": {
                "fuchsia.pkg.FontResolver":
                    "fuchsia-pkg://fuchsia.com/font_provider_test#meta/mock_font_resolver.cmx"
            }
        }
    },
    "program": {
        "binary": "bin/my_component_test"
    },
    "sandbox": {
        "services": [
            "fuchsia.pkg.FontResolver"
        ]
    }
}
```

To migrate this test to the Test Runner Framework, do the following:

1.  Create a CML file for the test component that points to the test binary and
    includes the appropriate [test runner][trf-test-runners]:

    Note: See [test runners][trf-provided-test-runners] that are provided by the
    framework.

    ```json5
    // my_component_test.cml (test component)
    {
        include: [
            // Select the appropriate test runner shard here:
            // rust, gtest, go, etc.
            "//src/sys/test_runners/rust/default.shard.cml",
        ],
        program: {
            // Binary containing tests
            binary: "bin/font_provider_test",
        },
        use: [
            ...
        ],
    }
    ```

1.  Ensure each component providing capabilities to this test has a CML manifest
    file. If this manifest does not already exist, consider creating it at this
    point. You can also temporarily wrap a legacy (CMX) provider component using
    the `cmx_runner` in your migrated test.

    * {CML provider}

        ```json5
        // mock_font_resolver.cml (capability provider)
        {
            program: {
                runner: "elf",
                binary: "bin/mock_font_resolver",
            },
            use: [
                //  mock_font_resolver's dependencies.
                {
                    protocol: [ "fuchsia.proto.SomeProtocol" ],
                },
            ],
            capabilities: [
                {
                    protocol: [ "fuchsia.pkg.FontResolver" ],
                },
            ],
            expose: [
                {
                    protocol: "fuchsia.pkg.FontResolver",
                    from: "self",
                },
            ],
        }
        ```

    * {CMX provider}

        ```json5
        // mock_font_resolver.cml (capability provider)
        {
            include: [
                // Use `cmx_runner` to wrap the component.
                "//src/sys/test_manager/cmx_runner/default.shard.cml",
                "syslog/client.shard.cml",
            ],
            program: {
                // wrap v1 component
                legacy_url: "fuchsia-pkg://fuchsia.com/font_provider_test#meta/mock_font_resolver.cmx",
            },
            use: [
                // if `mock_font_resolver.cmx` depends on some other protocol.
                {
                    protocol: [ "fuchsia.proto.SomeProtocol" ],
                },
            ],
            // expose capability provided by mock component.
            capabilities: [
                {
                    protocol: [ "fuchsia.pkg.FontResolver" ],
                },
            ],
            expose: [
                {
                    protocol: "fuchsia.pkg.FontResolver",
                    from: "self",
                },
            ],
        }
        ```

        Note: Component manifests wrapping a legacy component can only `use`
        protocol capabilities. The `.cmx` file of the legacy component defines
        the remaining non-protocol capabilities (`isolated-tmp`, `/dev`, etc).
        These capabilities will come directly from the system and can't be mocked
        or forwarded from the test to legacy components.

    Note: The CML files for the capability providers can be distributed in the
    same package that contained the v1 test. Follow the same instructions in
    [Migrate the component manifest][migrate-component-manifest] that you used
    to package your component.

1.  Add the capability provider(s) as children of the test component, and route
    the capabilities from each provider.

    * {CML provider}

        ```json5
        // my_component_test.cml (test component)
        {
            ...

            // Add capability providers
            children: [
                {
                    name: "font_resolver",
                    url: "#meta/mock_font_resolver.cm",
                },
            ],
            // Route capabilities to the test
            use: [
                {
                    protocol: [ "fuchsia.pkg.FontResolver" ],
                    from: "#font_resolver",
                },
            ],
            offer: [
                {
                    // offer dependencies to mock font provider.
                    protocol: [ "fuchsia.proto.SomeProtocol" ],
                    from: "#some_other_child",
                },
            ],
        }
        ```

    * {CMX provider}

        ```json5
        // my_component_test.cml (test component)
        {
            include: [
                // Required for wrapped CMX components
                "sys/testing/hermetic-tier-2-test.shard.cml",
            ],
            ...

            // Add capability providers
            children: [
                {
                    name: "font_resolver",
                    url: "#meta/mock_font_resolver.cm",
                },
            ],
            // Route capabilities to the test
            use: [
                {
                    protocol: [ "fuchsia.pkg.FontResolver" ],
                    from: "#font_resolver",
                },
            ],
            offer: [
                {
                    // offer dependencies to mock font provider.
                    protocol: [ "fuchsia.proto.SomeProtocol" ],
                    from: "#some_other_child",
                },
            ],
        }
        ```

1.  Package the test component and capability provider(s) together into a
    single hermetic `fuchsia_test_package()`:

    * {CML provider}

        ```gn
        # Test component
        fuchsia_component("my_component_test") {
          testonly = true
          manifest = "meta/my_component_test.cml"
          deps = [ ":bin_test" ]
        }

        fuchsia_component("mock_font_resolver") {
          testonly = true
          manifest = "meta/mock_font_resolver.cml"
          deps = [ ":mock_font_resolver_bin" ]
        }

        # Hermetic test package
        fuchsia_test_package("my_component_tests") {
          test_components = [ ":my_component_test" ]
          deps = [ ":mock_font_resolver" ]
        }
        ```

    * {CMX provider}

        ```gn
        # Test component
        fuchsia_component("my_component_test") {
          testonly = true
          manifest = "meta/my_component_test.cml"
          deps = [ ":bin_test" ]
        }

        fuchsia_component("mock_font_resolver") {
          testonly = true
          manifest = "meta/mock_font_resolver.cml"
          deps = [ {{ '<var label="legacy_component">"//path/to/legacy(v1)_component"</var>' }} ]
        }

        # Hermetic test package
        fuchsia_test_package("my_component_tests") {
          test_components = [ ":my_component_test" ]
          deps = [ ":mock_font_resolver" ]
        }
        ```

For more details on providing external capabilities to tests, see
[Integration testing topologies][integration-test].

## `TestWithEnvironment`

The legacy Component Framework provided a C++ library named
`TestWithEnvironment` that allowed the construction of an isolated environment
within a test. It was often used to serve injected services that are either
implemented in-process or by other components in the test.

Consider migrating legacy tests relying on this functionality to Realm Builder.
Realm Builder creates isolated environments similar to those constructed by
`TestWithEnvironment`. For more details on Realm Builder, see the
[developer guide][realm-builder].

The remainder of this section covers migrating `TestWithEnvironment` use cases
to Realm Builder.

### Test setup

Realm Builder does not provide its own test fixture, so tests that depend on
`TextWithEnvironmentFixture` should migrate to a more generic test fixture,
such as `gtest::RealLoopFixture`.

```cpp
class RealmBuilderTest : public gtest::RealLoopFixture {};
```

During the setup phase of your test, use `RealmBuilder::Create()` to initialize a
realm instance. After populating the realm with components and routes, call
`Build()` to construct and start the realm instance.

```cpp
TEST_F(RealmBuilderTest, RoutesProtocolFromChild) {
      auto realm_builder = RealmBuilder::Create();

      // Configure the realm.
      // ...

      auto realm = realm_builder.Build(dispatcher());

      // Use the constructed realm to assert properties on the components
      // under test.
      // ...
}
```

### Add components to a realm

When using `TestWithEnvironment`, services are specified before creation of the
`EnclosingEnvironment`. After the environment is created, the test may include
components using `CreateComponent()` or `CreateComponentFromUrl()`.
Consider the following example:

```cpp
std::unique_ptr<EnvironmentServices> services = CreateServices();
// Add services to the environment
// ...
auto test_env = CreateNewEnclosingEnvironment("test_env", std::move(services));

// Create additional components in the environment
test_env_->CreateComponentFromUrl(
      "fuchsia-pkg://fuchsia.com/example-package#meta/example.cmx");
```

Realm Bulder supports constructing realms that contain both legacy and CML
components simultaneously. However, all components must be added to the realm
_before_ it is created. Once a realm is created its contents are immutable.

To add a CML component with Realm Builder, use `AddChild()`:

```cpp
realm_builder->AddChild("example_component", "#meta/example_component.cm");
```

To include a legacy component in the same realm, use `AddLegacyChild()`:

```cpp
realm_builder->AddLegacyChild(
    "example_legacy_component",
    "fuchsia-pkg://fuchsia.com/example-package#meta/example.cmx");
```

Realm Builder allows you to provide additional options for each new child
component. The following example marks the child as [`eager`][cml-children] when
adding it to the realm, indicating the component should start automatically with
its parent:

```cpp
realm_builder->AddChild(
    "example_eager_component",
    "#meta/example_eager.cm",
    ChildOptions{.startup_mode = StartupMode::EAGER});
```

### Connect components together

When using `TestWithEnvironment`, the `EnclosedEnvironment` inherits all
services from the parent environment by default. Tests can configure additional
services in their nested environment using the `EnvironmentServices` instance.
It is not necessary to route these services from the components providing them
to the test environment.

With Realm Builder, tests must explicitly route all capabilities between
the components in the realm and the parent using `AddRoute()`.
The following example makes the `fuchsia.logger.LogSink` protocol available
from the parent to `example_component` and `example_legacy_component` in the
realm:

```cpp
realm_builder->AddRoute(
    Route{.capabilities = {Protocol{"fuchsia.logger.LogSink"}},
          .source = ParentRef(),
          .targets = {
              ChildRef{"example_component"},
              ChildRef{"example_legacy_component"}}});
```

Note: All components should be added to the realm _before_ adding routes.

To route additional capabilities between child components within the realm or
back to the parent, simply adjust the `source` and `target` properties.

```cpp
// Route fuchsia.examples.Example from one child to another
realm_builder->AddRoute(
    Route{.capabilities = {Protocol{"fuchsia.examples.Example"}},
          .source = ChildRef{"example_component"},
          .targets = {ChildRef{"example_legacy_component"}}});

//Route fuchsia.examples.Example2 up to the parent
realm_builder->AddRoute(
    Route{.capabilities = {Protocol{"fuchsia.examples.Example2"}},
          .source = ChildRef{"example_legacy_component"},
          .targets = {ParentRef{}}});
```

### Implement protocols

The `EnvironmentServices` connected to `TestWithEnvironment` can be implemented
anywhere, including within the test component itself. The test runner framework
in Components v2 does not allow test components to offer capabilities they
implement directly to components in the test realm. Instead the test component
can create _local components_ using Realm Builder.

Local components are implemented in-process by local objects. When these functions
are added to the realm under construction, they become a valid `source` or
`target` for capability routes. Once the realm is created, Realm Builder invokes
these functions as dedicated components.

The following example implements a mock for the `fuchsia.example.Echo` protocol:

```cpp
class LocalEchoServer : public test::placeholders::Echo, public LocalComponent {
 public:
  explicit LocalEchoServer(fit::closure quit_loop, async_dispatcher_t* dispatcher)
      : quit_loop_(std::move(quit_loop)), dispatcher_(dispatcher), called_(false) {}

  void EchoString(::fidl::StringPtr value, EchoStringCallback callback) override {
    callback(std::move(value));
    called_ = true;
    quit_loop_();
  }

  void Start(std::unique_ptr<LocalComponentHandles> handles) override {
    handles_ = std::move(handles);
    ASSERT_EQ(handles_->outgoing()->AddPublicService(bindings_.GetHandler(this, dispatcher_)),
              ZX_OK);
  }

  bool WasCalled() const { return called_; }

 private:
  fit::closure quit_loop_;
  async_dispatcher_t* dispatcher_;
  fidl::BindingSet<test::placeholders::Echo> bindings_;
  bool called_;
  std::unique_ptr<LocalComponentHandles> handles_;
};
```

You can use Realm Builder to instantiate this class as a local component to
provide `fuchsia.example.Echo` to the realm and handle requests:

```cpp
LocalEchoServer local_echo_server(QuitLoopClosure(), dispatcher());
realm_builder.AddLocalChild(kEchoServer, &local_echo_server);
```

Since the test has direct access to the local component object, you can inspect
it to determine state. In the above example, the test could assert the value of
`LocalEchoServer::WasCalled()` to indicate whether the FIDL protocol method was
accessed.

## Migrate component features {#features}

Explore the following sections for additional migration guidance on
specific features your test components may support:

-   [Component sandbox features](features.md)
-   [Diagnostics capabilities](diagnostics.md)
-   [Other common situations](common.md)

## Verify the migrated tests {#verify-tests}

Verify that your migrated tests are passing successfully using Components v2.

1.  Build the target for your test package:

    ```posix-terminal
    fx build
    ```

1.  Verify your tests successfully pass with the test:

    ```posix-terminal
    fx test my_component_tests
    ```

    Note: If tools or scripts invoke your tests component using
    `fx shell run-test-component`, migrate this usage to
    `fx shell run-test-suite` or `ffx test run`.

If your test doesn't run correctly or doesn't start at all, try following the
advice in [Troubleshooting test components][troubleshooting-tests].

[cml-children]: https://fuchsia.dev/reference/cml#children
[example-package-rule]: https://fuchsia.googlesource.com/fuchsia/+/cd29e692c5bfdb0979161e52572f847069e10e2f/src/fonts/BUILD.gn
[fuchsia-test-facets]: /docs/concepts/testing/v1_test_component.md
[hermetic-resolution]: /docs/development/testing/components/test_runner_framework.md#hermetic_component_resolution
[integration-test]: /docs/development/testing/components/integration_testing.md
[migrate-component-manifest]: /docs/development/components/v2/migration/components.md#create-component-manifest
[system-services]: /docs/concepts/testing/v1_test_component.md#services
[trf-provided-test-runners]: /src/sys/test_runners
[trf-test-manager]: /docs/development/testing/components/test_runner_framework.md#the_test_manager
[trf-test-runners]: /docs/development/testing/components/test_runner_framework.md#test-runners
[troubleshooting-tests]: /docs/development/testing/components/test_runner_framework.md#troubleshooting
[unit-test-manifests]: /docs/development/components/build.md#unit-tests
[realm-builder]: /docs/development/testing/components/realm_builder.md
