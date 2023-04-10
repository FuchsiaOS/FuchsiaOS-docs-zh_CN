{% set v1_banner %}
  <aside class="caution">
    <b>Caution:</b> Caution: This format is used with legacy components.
    If you are still using legacy components, consider
    <a href="/docs/contribute/open_projects/components/migration.md">migrating</a>
    to the modern component framework.
  </aside>
{% endset %}

# Build components

This document demonstrates how to build and test a component, highlighting best
practices for defining packages, components, and their tests.

## Concepts {#concepts}

You should understand the following concepts before building a component:

**[Packages][glossary.package]** are the unit of software distribution on
Fuchsia. Packages are a collection of files with associated paths that are
relative to the base of the package. For instance, a package might contain an
ELF binary under the path `bin/hello_world`, and a JSON file under the path
`data/config.json`. Grouping files into a package is required in order to push
these files to the device.

**[Components][glossary.component]** are the unit of software execution on
Fuchsia. All software on Fuchsia except for the kernel image and user mode
bootstrap program is defined as a component.

A component is defined by a
**[component manifest][glossary.component-manifest]**. Components typically
include additional files, such as executables and data assets that they need at
runtime.

Developers must define their software in terms of packages and components,
whether for building production software or for writing their tests.

At runtime, **[Component instances][glossary.component-instance]** see the
contents of their package as read-only files under the path `/pkg`. Defining two
or more components in the same package doesn't grant each component access to
the other's capabilities. However it can guarantee to one component that the
other is available. Therefore if a component attempts to launch an instance of
another component, such as in an integration test, it can be beneficial to
package both components together.

Components are instantiated in a few ways, all of which somehow specify their
[URL][glossary.component-url]. Typically components are launched by specifying
their package names and path to their component manifest in the package, using
the [<code>fuchsia-pkg://</code> scheme][glossary.fuchsia-pkg-url].

## Component manifests {#component-manifests}

A component manifest is a file that encodes a component declaration, usually
distributed as part of a package. The binary format is a persisted FIDL file
containing the component declaration. The manifest declares information about
the component's program binary and required capabilities.

Below is an example manifest file for a simple "Hello, World" component:

```json5
{
    // Information about the program to run.
    program: {
        // Use the built-in ELF runner to run platform-specific binaries.
        runner: "elf",
        // The binary to run for this component.
        binary: "bin/hello",
        // Program arguments
        args: [
            "Hello",
            "World!",
        ],
    },

    // Capabilities used by this component.
    use: [
        { protocol: "fuchsia.logger.LogSink" },
    ],
}
```

### Manifest shards {#component-manifest-shards}

Some collections of capabilities represent use case requirements that are common
to many components in the system, such as logging. To simplify including these
capabilities in your components, the Component Framework supports abstracting
them into **manifest shards** that can be included in your main manifest file.
This is conceptually similar to `#include` directives in the C programming
language.

Note: By convention, component manifest shard files are named with `.shard` in
the file suffix.

Below is an equivalent manifest to the previous example, with the logging
capability replaced by a manifest shard `include`:

```json5
{
    // Include capabilities for the syslog library
    include: [ "syslog/client.shard.cml" ],

    // Information about the program to run.
    program: {
        // Use the built-in ELF runner to run platform-specific binaries.
        runner: "elf",
        // The binary to run for this component.
        binary: "bin/hello-world",
        // Program arguments
        args: [
            "Hello",
            "World!",
        ],
    },
}
```

#### Relative paths

Include paths that begin with `"//"` are relative to the root of the source tree
that you are working in. For include paths that don't begin with `"//"`, the
build system will attempt to resolve them from the Fuchsia SDK.

#### Inter-shard dependencies

If one manifest shard adds a child to the manifest and a second manifest shard
adds a second child which depends on the first, then the offer declaration from
the first child to the second child will cause a manifest validation error if
the second shard is ever included in a manifest without the first shard, since
the offer will reference a non-existent child.

```json
// echo_server.shard.cml
{
    children: [ {
        name: "echo_server",
        url: "fuchsia-pkg://fuchsia.com/echo_server#meta/echo_server.cm",
    } ],
}
```

```json
// echo_client.shard.cml
{
    children: [
        {
            name: "echo_client",
            url: "fuchsia-pkg://fuchsia.com/echo_client#meta/echo_client.cm",
        }
    ],
    offer: [ {
        // This offer will cause manifest validation to fail if
        // `echo_client.shard.cml` is included in a manifest without
        // `echo_server.shard.cml`.
        protocol: "fuchsia.examples.Echo",
        from: "echo_server",
        to: "echo_client",
    } ],
}
```

To address this, the `source_availability` field on an offer can be set to
inform manifest compilation that it's acceptable for an offer source to be
missing. When set to `unknown`, then the following will happen to the offer
declaration:

- If the `from` source exists: the availability is set to `required`.
- If the `from` source does not exist: the availability is set to `optional` and
  the source of the offer is rewritten to `void`.

```json
// echo_client.shard.cml
{
    children: [
        {
            name: "echo_client",
            url: "fuchsia-pkg://fuchsia.com/echo_client#meta/echo_client.cm",
        }
    ],
    offer: [
        {
            // If `echo_server.shard.cml` is included in this manifest, then
            // `echo_client` can access the `fuchsia.examples.Echo` protocol from
            // it.
            //
            // If `echo_server.shard.cml` is not included in this manifest, then
            // `echo_client` will be offered the protocol with a source of
            // `void` and `availability == optional`. `echo_client` must consume
            // the capability optionally to not fail route validation.
            protocol: "fuchsia.examples.Echo",
            from: "echo_server",
            to: "echo_client",
            source_availability: "unknown",
        }
    ],
}
```

### Client library includes {#component-manifest-includes}

As shown above, the component manifest supports "include" syntax, which allows
referencing one or more manifest shards as source for additional manifest content.
Some dependencies, such as libraries, assume that dependent components have
certain capabilities available to them at runtime. For instance,
the [C++ Syslog library][cpp-syslog] makes such an assumption.

If you are building a client library, you can declare these required dependencies
using `expect_includes` in your `BUILD.gn` file. For example, consider the
following hypothetical file `//sdk/lib/fonts/BUILD.gn` below:

```gn
import("//tools/cmc/build/expect_includes.gni")

# Client library for components that want to use fonts
source_set("font_provider_client") {
  sources = [
    "font_provider_client.cc",
    ...
  ]
  deps = [
    ":font_provider_client_includes",
    ...
  ]
}

expect_includes("font_provider_client_includes") {
  includes = [
    "client.shard.cml",
  ]
}
```

This sets a build time requirement for dependent manifests to include the
expected manifest shards:

```json5
{
    include: [
        "//sdk/lib/fonts/client.shard.cml",
    ]
    ...
}
```

Include paths are resolved relative to the source root.
Transitive includes (includes of includes) are allowed.
Cycles are not allowed.

When naming your shards, don't repeat yourself in relation to the full path.
In the example above it would have been repetitive to name the shard
`fonts.shard.cml` because then the full path would have been
`sdk/lib/fonts/fonts.shard.cml`, which is repetitive. Instead the file is
named `client.shard.cml`, to indicate that it is to be used by clients of the
SDK library for fonts.

## Component package GN templates {#component-package}

[GN][glossary.gn] is the meta-build system used by Fuchsia. Fuchsia extends GN
by defining templates. Templates provide a way to add to GN's built-in target
types.

Fuchsia defines the following GN templates to define packages and components:

*   [`fuchsia_component.gni`](/build/components/fuchsia_component.gni)
*   [`fuchsia_package.gni`](/build/components/fuchsia_package.gni)

Below is a hypothetical package containing one component that runs a C++
program:

```gn
import("//build/components.gni")

executable("my_program") {
  sources = [ "my_program.cc" ]
}

fuchsia_component("my_component") {
  manifest = "meta/my_program.cml"
  deps = [ ":my_program" ]
}

fuchsia_package("my_package") {
  deps = [ ":my_component" ]
}
```

Note the following details:

*   Import `"//build/components.gni"` to access all templates related to
    packages, components, and tests.
*   The `fuchsia_component()` template declares the component. It depends on the
    program binary (in this case, `executable()`) and requires a `manifest`
    pointing to the component manifest file.
*   Both the component and package names are derived from their target names.
    In the example above, these names come together to form the URL for
    launching the component:
    `fuchsia-pkg://fuchsia.com/my_package#meta/my_component.cm`.

    Note: Targets support an optional `component_name` or `package_name`
    parameter to override the default behavior.

### Language-specific component examples {#language-specific-component-examples}

Below you'll find basic examples for defining a package with a single component
that launches a program in a variety of commonly used languages. The referenced
source files and component manifest are assumed to be present in the specified
paths.

   * {C++}

   ```gn
   import("//build/components.gni")

   executable("bin") {
     output_name = "my_program"
     sources = [ "main.cc" ]
   }

   fuchsia_component("my_component") {
     manifest = "meta/my_component.cml"
     deps = [ ":bin" ]
   }

   fuchsia_package("my_package") {
     deps = [ ":my_component" ]
   }
   ```

   * {Rust}

   ```gn
   import("//build/rust/rustc_binary.gni")
   import("//build/components.gni")

   rustc_binary("bin") {
     output_name = "my_program"
     sources = [ "src/main.rs" ]
   }

   fuchsia_component("my_component") {
     manifest = "meta/my_component.cml"
     deps = [ ":bin" ]
   }

   fuchsia_package("my_package") {
     deps = [ ":my_component" ]
   }
   ```

   * {Go}

   ```gn
   import("//build/go/go_binary.gni")
   import("//build/components.gni")

   go_binary("bin") {
     output_name = "my_program"
     sources = [ "main.go" ]
   }

   fuchsia_component("my_component") {
     manifest = "meta/my_component.cml"
     deps = [ ":bin" ]
   }

   fuchsia_package("my_package") {
     deps = [ ":my_component" ]
   }
   ```

   * {Dart}

   ```gn
   import("//build/dart/dart_component.gni")
   import("//build/dart/dart_library.gni")
   import("//build/components.gni")

   dart_library("lib") {
     package_name = "my_lib"
     sources = [ "main.dart" ]
   }

   dart_component("my_component") {
     manifest = "meta/my_component.cml"
     deps = [ ":lib" ]
   }

   fuchsia_package("my_package") {
     deps = [ ":my_component" ]
   }
   ```

   * {Flutter}

   ```gn
   import("//build/dart/dart_library.gni")
   import("//build/flutter/flutter_component.gni")
   import("//build/components.gni")

   dart_library("lib") {
     package_name = "my_lib"
     sources = [ "main.dart" ]
   }

   flutter_component("my_component") {
     manifest = "meta/my_component.cml"
     deps = [ ":lib" ]
   }

   fuchsia_package("my_package") {
     deps = [ ":my_component" ]
   }
   ```

### Packages with a single component {#packages-with-single-component}

Packages are units of distribution. It is beneficial to define multiple
components in the same package if you need to guarantee that several
components are always co-present, or if you'd like to be able to update
several components at once (by updating a single package).

This pattern is also commonly used to create hermetic integration tests.
For instance an integration test between two components where one is a client
of a service implemented in another component would include both the client
and server components.

However, you may often define a package that only requires a single component.
In those cases, you can use the `fuchsia_package_with_single_component()`
template as a convenience. This template fuses together `fuchsia_package()` and
`fuchsia_component()`.

   * {C++}

   ```gn
   import("//build/components.gni")

   executable("rot13_encoder_decoder") {
     sources = [ "rot13_encoder_decoder.cc" ]
   }

   fuchsia_package_with_single_component("rot13") {
     manifest = "meta/rot13.cml"
     deps = [ ":rot13_encoder_decoder" ]
   }
   ```

   * {Rust}

   ```gn
   import("//build/rust/rustc_binary.gni")
   import("//build/components.gni")

   rustc_binary("rot13_encoder_decoder") {
     sources = [ "src/rot13_encoder_decoder.rs" ]
   }

   fuchsia_package_with_single_component("rot13") {
     manifest = "meta/rot13.cml"
     deps = [ ":rot13_encoder_decoder" ]
   }
   ```

   * {Go}

   ```gn
   import("//build/go/go_binary.gni")
   import("//build/components.gni")

   go_binary("rot13_encoder_decoder") {
     sources = [ "rot13_encoder_decoder.go" ]
   }

   fuchsia_package_with_single_component("rot13") {
     manifest = "meta/rot13.cml"
     deps = [ ":rot13_encoder_decoder" ]
   }
   ```

## Test package GN templates {#test-packages}

Test packages are packages that contain at least one component that is
launched as a test. Test packages are defined using
[`fuchsia_test_package.gni`](/build/components/fuchsia_test_package.gni). This
template can be used to define all sorts of tests, although it's most useful for
integration tests -- tests where other components in addition to the test itself
participate in the test. See [unit tests](#unit-tests) for templates that
specialize in unit testing.

```gn
import("//build/components.gni")

executable("my_test") {
  sources = [ "my_test.cc" ]
  testonly = true
  deps = [
    "//src/lib/fxl/test:gtest_main",
    "//third_party/googletest:gtest",
  ]
}

fuchsia_component("my_test_component") {
  testonly = true
  manifest = "meta/my_test.cml"
  deps = [ ":my_test" ]
}

executable("my_program_under_test") {
  sources = [ "my_program_under_test.cc" ]
}

fuchsia_component("my_other_component_under_test") {
  manifest = "meta/my_component_under_test.cml"
  deps = [ ":my_program_under_test" ]
}

fuchsia_test_package("my_integration_test") {
  test_components = [ ":my_test_component" ]
  deps = [ ":my_other_component_under_test" ]
  test_specs = {
    environments = [ vim3_env ]
  }
}

group("tests") {
  deps = [ ":my_integration_test" ]
  testonly = true
}
```

Note the following details:

*   This example defines `"my_test_component"`, which is assumed to implement
    tests written using some common testing framework such as C++ Googletest,
    Rust Cargo test, etc.
*   The test is packaged with a dependent component,
    `"my_other_component_under_test"`. This could be a mock service provider
    required by the test component or another component the test needs to invoke.
    Packaging these components together guarantees that the dependent component
    is available to launch while the test is running, and built at the same
    version as the test.
*   The `environments` parameter enables `fuchsia_test_package()` to optionally
    take [`test_spec.gni`](/build/testing/test_spec.gni) parameters and override
    the default testing behavior. In this example, this test is configured to
    run on VIM3 devices.
*   Finally, this example defines a `group()` to contain all the tests (which we
    have exactly one of). This is a [recommended practice][source-code-layout]
    for organizing targets across the source tree.

Due to a [limitation in GN][gn-get-target-outputs], any `test_component` targets
in your `fuchsia_test_package()` must be defined in the same `BUILD.gn` file as
the test package target. You can work around this behavior with an indirection
through `fuchsia_test()`.

In one `BUILD.gn` file, define:

```gn
# Let this be //foo/BUILD.gn
import("//build/components.gni")

executable("my_test") {
  sources = [ "my_test.cc" ]
  testonly = true
  deps = [
    "//src/lib/fxl/test:gtest_main",
    "//third_party/googletest:gtest",
  ]
}

fuchsia_component("my_test_component") {
  testonly = true
  manifest = "meta/my_test.cml"
  deps = [ ":my_test" ]
}

fuchsia_test("my_test_component_test") {
  package = "//bar:my_test_package"
  component = ":my_test_component"
}

group("tests") {
  testonly = true
  deps = [ ":my_test_component_test" ]
}
```

Then elsewhere, you can add the `fuchsia_component()` target to the `deps` of a
`fuchsia_package()` target.

```gn
# Let this be //bar/BUILD.gn
import("//build/components.gni")

fuchsia_package("my_test_package") {
  testonly = true
  deps = [ "//foo:my_test_component" ]
}
```

### Dart and Flutter tests

Dart and Flutter tests differ slightly in that they need to be built with a
`flutter_test_component()`, which collects all of the test mains into a single
main invocation. The `flutter_test_component()` can then be used by the
`fuchsia_test_package()`.

```gn
import("//build/dart/dart_test_component.gni")
import("//build/flutter/flutter_test_component.gni")
import("//build/components.gni")

flutter_test_component("my_flutter_test_component") {
  testonly = true
  manifest = "meta/my_flutter_test_component.cml"
  sources = [ "foo_flutter_test.dart" ]
}

dart_test_component("my_dart_test_component") {
  testonly = true
  manifest = "meta/my_dart_test_component.cml"
  sources = [ "foo_dart_test.dart" ]
}

fuchsia_test("my_test_component_test") {
  test_components = [
    ":my_dart_test_component",
    ":my_flutter_test_component"
  ]
}
```

### Unit tests {#unit-tests}

Since unit tests are very common, the build system provides two simplified
GN templates:

* [`fuchsia_unittest_component.gni`](/build/components/fuchsia_unittest_component.gni)
  defines a component to be run as a test, with the option to automatically
  generate a basic component manifest, that must then be included in a package.
* [`fuchsia_unittest_package.gni`](/build/components/fuchsia_unittest_package.gni)
  defines a package with a single component to be run as a test, shorthand for
  a single `fuchsia_unittest_component()` target paired with a
  `fuchsia_test_package()`.

#### Unit tests with manifests {#unit-tests-with-manifests}

The examples below demonstrate building a test executable and defining a
package and component for the test.

   * {C++}

   ```gn
   import("//build/components.gni")

   executable("my_test") {
     sources = [ "test.cc" ]
     deps = [
       "//src/lib/fxl/test:gtest_main",
       "//third_party/googletest:gtest",
     ]
     testonly = true
   }

   fuchsia_unittest_package("my_test") {
     manifest = "meta/my_test.cml"
     deps = [ ":my_test" ]
   }
   ```

   * {Rust}

   ```gn
   import("//build/rust/rustc_test.gni")
   import("//build/components.gni")

   rustc_test("my_test") {
     sources = [ "test.rs" ]
     testonly = true
   }

   fuchsia_unittest_package("my_test") {
     manifest = "meta/my_test.cml"
     deps = [ ":my_test" ]
   }
   ```

   * {Go}

   ```gn
   import("//build/go/go_test.gni")
   import("//build/components.gni")

   go_test("my_test") {
     sources = [ "test.go" ]
     testonly = true
   }

   fuchsia_unittest_package("my_test") {
     manifest = "meta/my_test.cml"
     deps = [ ":my_test" ]
   }
   ```

Launch the test component using `fx test` with either the GN target name
or the full component URL:

   * {GN Target}

   ```posix-terminal
   fx test my_test
   ```

   * {Component URL}

   ```posix-terminal
   fx test fuchsia-pkg://fuchsia.com/my_test#meta/my_test.cm
   ```

#### Unit tests with _generated_ manifests

The examples above specify a manifest for the test. However, it's possible for
unit tests to not require any particular capabilities.

Below is an example for a test that performs ROT13 encryption and decryption.
The algorithm under test is pure logic that can be tested in complete
isolation. If we were to write a manifest for these tests, it would only
contain the test binary to be executed. In such cases, we can simply specify
the test executable path, and the template generates the trivial manifest
for us.

   * {C++}

   ```gn
   import("//build/components.gni")

   executable("rot13_test") {
     sources = [ "rot13_test.cc" ]
     deps = [
       "//src/lib/fxl/test:gtest_main",
       "//third_party/googletest:gtest",
     ]
     testonly = true
   }

   fuchsia_unittest_package("rot13_test") {
     deps = [ ":rot13_test" ]
   }
   ```

   * {Rust}

   ```gn
   import("//build/rust/rustc_test.gni")
   import("//build/components.gni")

   rustc_test("rot13_test") {
     sources = [ "rot13_test.rs" ]
     testonly = true
   }

   fuchsia_unittest_package("rot13_test") {
     deps = [ ":rot13_test" ]
   }
   ```

   * {Go}

   ```gn
   import("//build/go/go_test.gni")
   import("//build/components.gni")

   go_test("rot13_test") {
     sources = [ "rot13_test.go" ]
     testonly = true
   }

   fuchsia_unittest_package("rot13_test") {
     deps = [ ":rot13_test" ]
   }
   ```

The generated component manifest file can be found with the following command:

```posix-terminal
fx gn outputs $(fx get-build-dir) {{ '<var>//some/path/to/build/file:unittest target</var>' }}_component_generated_manifest
```

To print it directly:

```posix-terminal
fx build && cat $(fx get-build-dir)/$(fx gn outputs $(fx get-build-dir) {{ '<var>//some/path/to/build/file:unittest target</var>' }}_component_generated_manifest)
```

Note: `fx gn outputs` prints an output path, but the file at the path
may not exist or may be stale if you haven't built.

Launch the test component using `fx test` with either the GN target name
or the full component URL:

   * {GN Target}

   ```posix-terminal
   fx test rot13_test
   ```

   * {Component URL}

   ```posix-terminal
   fx test fuchsia-pkg://fuchsia.com/rot13_test#meta/rot13_test.cm
   ```

#### Multiple unit tests in a single package

To package multiple unit testing components together, use the
`fuchsia_unittest_component()` rule instead of `fuchsia_unittest_package()`,
collecting them together in a`fuchsia_test_package()`. This enables you to run
all the test components in a single package with `fx test <package_name>` rather
than executing them individually.

The example below creates a single test package `rot13_tests` that contains two
separate test components, `rot13_decoder_test` and `rot13_encoder_test`.

   * {C++}

   ```gn
   import("//build/components.gni")

   executable("rot13_decoder_bin_test") {}

   executable("rot13_encoder_bin_test") {}

   fuchsia_unittest_component("rot13_decoder_test") {
     deps = [ ":rot13_decoder_bin_test" ]
   }

   fuchsia_unittest_component("rot13_encoder_test") {
     deps = [ ":rot13_encoder_bin_test" ]
   }

   fuchsia_test_package("rot13_tests") {
     test_components = [
       ":rot13_decoder_test",
       ":rot13_encoder_test",
     ]
   }
   ```

   * {Rust}

   ```gn
   import("//build/rust/rustc_test.gni")
   import("//build/components.gni")

   rustc_test("rot13_decoder_bin_test") {}

   rustc_test("rot13_encoder_bin_test") {}

   fuchsia_unittest_component("rot13_decoder_test") {
     deps = [ ":rot13_decoder_bin_test" ]
   }

   fuchsia_unittest_component("rot13_encoder_test") {
     deps = [ ":rot13_encoder_bin_test" ]
   }

   fuchsia_test_package("rot13_tests") {
     test_components = [
       ":rot13_decoder_test",
       ":rot13_encoder_test",
     ]
   }
   ```

   * {Go}

   ```gn
   import("//build/go/go_test.gni")
   import("//build/components.gni")

   go_test("rot13_decoder_test") {}

   go_test("rot13_encoder_test") {}

   fuchsia_unittest_component("rot13_decoder_test") {
     deps = [ ":rot13_decoder_bin_test" ]
   }

   fuchsia_unittest_component("rot13_encoder_test") {
     deps = [ ":rot13_encoder_bin_test" ]
   }

   fuchsia_test_package("rot13_tests") {
     test_components = [
       ":rot13_decoder_test",
       ":rot13_encoder_test",
     ]
   }
   ```

Launch all test components inside the package using `fx test` with simply the
GN target name:

```posix-terminal
fx test rot13_tests
```

### Test-driven development

The `fx smoke-test` command automatically detects all tests that are known to
the build system as affected by changes in your checkout. Try the following:

```posix-terminal
fx -i smoke-test --verbose
```

In the command above, `--verbose` prints which tests `fx smoke-test` thinks
are affected by your change, and `-i` automatically repeats this command
every time you save your changes. For test-driven development, try launching
this command in a separate shell and watching your code rebuild and retest as
you're working on it.

`fx smoke-test` works best with hermetic test packages. A test package is
hermetic if the package contains all the dependencies of any tests in it.
That is to say, any code changes that affect the outcome of this test should
require rebuilding that test's package as well.

## Additional packaged resources {#additional-packaged-resources}

In the examples above we've demonstrated that a `deps` path from a package to a
target that produces an executable ensures that the executable is included in
the package.

Sometimes there is the need to include additional files. Below we demonstrate
the use of two [`resource.gni`](/build/dist/resource.gni) templates,
`resource()`, `resource_group()`, and `resource_tree()`.

### Example: fonts

{# Disable variable substitution to avoid {{ being interpreted by the template engine #}
{% verbatim %}

```gn
import("//build/components.gni")

resource("roboto_family") {
  sources = [
    "Roboto-Black.ttf",
    "Roboto-Bold.ttf",
    "Roboto-Light.ttf",
    "Roboto-Medium.ttf",
    "Roboto-Regular.ttf",
    "Roboto-Thin.ttf",
  ]
  outputs = [ "data/fonts/{{source_file_part}}" ]
}

fuchsia_component("text_viewer") {
  ...
  deps = [
    ":roboto_family",
    ...
  ]
}
```

{# Re-enable variable substitution #}
{% endverbatim %}

In the example above, six files are provided to be packaged under `data/fonts/`,
producing the paths `data/fonts/Roboto-Black.ttf`,
`data/fonts/Roboto-Bold.ttf`, etc'. The format for `destination` accepts [GN
source expansion placeholders][source-expansion-placeholders].

Then, a text viewer component is defined to depend on the fonts. In this
example, the text viewer implementation renders text with Roboto fonts. The
component can read the given fonts in its sandbox under the path
`/pkg/data/fonts/...`.

### Example: integration test with golden data

In this example we define a hypothetical service that minifies JSON files. The
service is said to receive a buffer containing JSON text, and returns a buffer
containing the same JSON data but with less whitespace. We present an
integration test where a test component acts as the client of the minifier
component, and compares the result for a given JSON file to be minified against
a known good result (or a "golden file").

{# Disable variable substitution to avoid {{ being interpreted by the template engine #}
{% verbatim %}

```gn
import("//build/components.gni")

fuchsia_component("minifier_component") {
  ...
}

fuchsia_package("minifier_package") {
  ...
}

resource("testdata") {
  sources = [
    "testdata/input.json",
    "testdata/input_minified.json",
  ]
  outputs = [ "data/{{source_file_part}}" ]
}

fuchsia_component("minifier_test_client") {
  testonly = true
  deps = [
    ":testdata",
    ...
  ]
  ...
}

fuchsia_test_package("minifier_integration_test") {
  test_components = [ ":minifier_test_client" ]
  deps = [ ":minifier_component" ]
}
```

{# Re-enable variable substitution #}
{% endverbatim %}

Note that we place the `resource()` dependency on the test component. From the
build system's perspective the resource dependency could have been placed on
the test package and the same outcome would have been produced by the build.
However, it is a better practice to put dependencies on the targets that need
them. This way we could reuse the same test component target in a different
test package, for instance to test against a different minifier component, and
the test component would work the same.

### Example: using `resource_group()`

In the examples above all the paths conformed to a certain structure such that
we could specify a single output pattern for multiple files and even leverage
[GN source expansion placeholders][source-expansion-placeholders]. In this next
example we are required to rename different files to different destination
paths for packaging.

```gn
import("//build/components.gni")

resource_group("favorite_recipes") {
  files = [
    {
      source = "//recipes/spaghetti_bolognese.txt"
      dest = "data/pasta/spaghetti_bolognese.txt"
    },
    {
      source = "//recipes/creamy_carbonara.txt"
      dest = "data/pasta/carbonara.txt"
    },
    {
      source = "//recipes/creme_brulee.txt"
      dest = "data/dessert/creme_brulee.txt"
    },
    ...
  ]
}
```

Our sources are all in a single directory, but are to be packaged in different
directories, some even under different names. To express this same relationship
we might need as many `resource()` targets as we have files. Situations like
this call for the use of `resource_group()` instead, as shown above.

### Example: using `resource_tree()`

Mapping each source file to a destination file path using `resource_group()` can
be cumbersome for larger file sets. `resource_tree()` offers a way to map a
directory tree of source files to an identical hierarchy under a destation
directory in the package. The following example copies the subdirectory
`default_repo_files/` to the package directory `repo/` (using the `sources` list
to ensure only the explicitly listed files are included).

```gn
import("//build/components.gni")

resource_tree("default-repo") {
  sources_root = "default_repo_files"
  sources = [
    "keys/root.json",
    "keys/snapshot.json",
    "keys/targets.json",
    "keys/timestamp.json",
    "repository/1.root.json",
    "repository/1.snapshot.json",
    "repository/1.targets.json",
    "repository/root.json",
    "repository/snapshot.json",
    "repository/targets.json",
    "repository/timestamp.json",
  ]
  dest_dir = "repo"
}
```

The underlying behavior of `resource()`, `resource_group()`, and
`resource_tree()` is identical. You are free to choose whichever one you prefer.

Note: see more information in [provide data files to components][provide-data].

## Restricted features {#restricted-features}

When a new component manifest feature is under active development, or a feature
is intended for a narrow audience, the Component Framework team may wish to
restrict who may use the feature. The CML compiler (`cmc`) controls access to
these restricted features through an opt-in property in your component build
rule.

In order to use an restricted feature, add the `restricted_features` property:

```gn
fuchsia_component("my-component") {
  manifest = "meta/my-component.cml"
  # This component opts-in to the restricted "allow_long_names" feature.
  restricted_features = [ "allow_long_names" ]
  deps = [ ... ]
}
```

Use of restricted features are restricted to an allowlist.
You must add your component to the allowlist for the feature in
[`//tools/cmc/build/restricted_features/BUILD.gn`][allowlist].

## Troubleshooting {#troubleshooting}

This section contains common issues you may encounter while building your components.

### Missing shard includes {#troubleshoot-build-include}

The `check_includes` action fails the build with the following error if your
[component manifest][glossary.component-manifest] is missing an `include` for a
required [manifest shard](#component-manifest-shards):

```none {:.devsite-disable-click-to-copy}
Error at ../../examples/components/echo_server/meta/echo_server.cml:
"../../examples/components/echo_server/meta/echo_server.cml" must include "../../sdk/lib/inspect/client.shard.cml".
```

This occurs when a library in your component's dependency chain has an
[`expect_includes`](#component-manifest-includes) requirement and the required
`include` was not found in your component manifest. Consider the following example
using [Inspect][doc-inspect]:

* {C++}

  1.  Your component depends on `//sdk/lib/sys/inspect/cpp`:

      ```gn {:.devsite-disable-click-to-copy}
      executable("bin") {
        output_name = "echo_server_cpp"
        sources = [ "main.cc" ]

        deps = [
          "//examples/components/routing/fidl:echo",
          "//sdk/lib/sys/cpp",
          {{ '<strong>' }}# This library requires "inspect/client.shard.cml" {{ '</strong>' }}
          {{ '<strong>' }}"//sdk/lib/sys/inspect/cpp", {{ '</strong>' }}
          "//zircon/system/ulib/async-loop:async-loop-cpp",
          "//zircon/system/ulib/async-loop:async-loop-default",
        ]
      }
      ```

  1.  [`//sdk/lib/sys/inspect/cpp`][src-inspect-cpp] depends on
      [`//sdk/lib/inspect:client_includes`][src-inspect-include], which is an
      `expect_includes()` rule.

* {Rust}

  1.  Your component depends on `//src/lib/diagnostics/inspect/runtime/rust`:

      ```gn
      rustc_binary("echo_server") {
        edition = "2021"
        deps = [
          "//examples/components/routing/fidl:echo_rust",
          {{ '<strong>' }}# This library requires "inspect/client.shard.cml" {{ '</strong>' }}
          {{ '<strong>' }}"//src/lib/diagnostics/inspect/runtime/rust", {{ '</strong>' }}
          "//src/lib/diagnostics/inspect/rust",
          "//src/lib/fuchsia",
          "//src/lib/fuchsia-component",
          "//third_party/rust_crates:anyhow",
          "//third_party/rust_crates:futures",
        ]

        sources = [ "src/main.rs" ]
      }
      ```

  1.  [`//src/lib/diagnostics/inspect/runtime/rust`][src-inspect-rust] depends on
      [`//sdk/lib/inspect:client_includes`][src-inspect-include], which is an
      `expect_includes()` rule.

To address the issue, add the missing `include` in your component manifest. For example:

```json5 {:.devsite-disable-click-to-copy}
{
    include: [
        {{ '<strong>' }}// Add this required include {{ '</strong>' }}
        {{ '<strong>' }}"inspect/client.shard.cml", {{ '</strong>' }}

        // Enable logging
        "syslog/client.shard.cml",
    ],

    // ...
}
```

For additional detail on the source of the required includes, you can use the `gn path`
command to explore the dependency path:

```posix-terminal
fx gn path $(fx get-build-dir) {{ '<var>my-component</var>' }} {{ '<var>expect_includes target</var>' }} --with-data
```

Note: You can find the `expect_includes()` target by searching for `BUILD.gn` files that
reference the missing shard by filename.

The command prints output similar to the following, showing the path to the library that
required the include:

* {C++}

  ```none {:.devsite-disable-click-to-copy}
  $ fx gn path $(fx get-build-dir) //examples/components/routing/cpp/echo_server //sdk/lib/inspect:client_includes --with-data
  //examples/components/echo_server:bin --[private]-->
  //sdk/lib/sys/inspect/cpp:cpp --[data]-->
  //sdk/lib/inspect:client_includes
  ```

* {Rust}

  ```none {:.devsite-disable-click-to-copy}
  $ fx gn path $(fx get-build-dir) //examples/components/routing/rust/echo_server //sdk/lib/inspect:client_includes --with-data
  //examples/components/routing/rust/echo_server:bin --[public]-->
  //examples/components/routing/rust/echo_server:bin.actual --[private]-->
  //src/lib/diagnostics/inspect/runtime/rust:rust --[public]-->
  //src/lib/diagnostics/inspect/runtime/rust:lib --[public]-->
  //src/lib/diagnostics/inspect/runtime/rust:lib.actual --[private]-->
  //sdk/lib/inspect:client_includes
  ```

### Failed to validate manifest {#troubleshoot-build-references}

The `cmc_validate_references` action fails the build with the following error if your
[component manifest][glossary.component-manifest] contains references to resources that
cannot be found in the component's package:

```none {:.devsite-disable-click-to-copy}
Error found in: //examples/components/echo/rust:rust-component_cmc_validate_references(//build/toolchain/fuchsia:x64)
	Failed to validate manifest: "obj/examples/components/echo/rust/cml/rust-component_manifest_compile/echo_rust.cm"
program.binary=bin/echo_example_oops but bin/echo_example_oops is not provided by deps!

Did you mean bin/echo_example?

Try any of the following:
...
```

This occurs when the `binary` field in your component manifest [`program`][cml-program] block
references a file path that is not present in your `fuchsia_package()`.

To address the issue, verify the following:

1.  Reference paths in your component manifest are entered correctly.

    ```json5 {:.devsite-disable-click-to-copy}
    {
        // ...

        // Information about the program to run.
        program: {
            // Use the built-in ELF runner.
            runner: "elf",

            {{ '<strong>' }}// The binary to run for this component. {{ '</strong>' }}
            {{ '<strong>' }}binary: "bin/echo_example_oops", {{ '</strong>' }}
        },
    }
    ```

1.  The component executable target is part of the `deps` chain connected to your
    `fuchsia_package()`:

    * {C++}

      ```gn {:.devsite-disable-click-to-copy}
      executable("bin") {
        output_name = "echo_example"
        sources = [ "main.cc" ]

        deps = [ ... ]
      }

      {{ '<strong>' }}# Neither the component or package depend on ":bin" {{ '</strong>' }}
      fuchsia_component("component") {
        manifest = "meta/echo_example.cml"
        {{ '<strong>' }}deps = [] {{ '</strong>' }}
      }

      fuchsia_package("package") {
        package_name = "echo_example"
        {{ '<strong>' }}deps = [ ":component" ] {{ '</strong>' }}
      }
      ```

    * {Rust}

      ```gn
      rustc_binary("echo_example") {
        edition = "2021"
        sources = [ "src/main.rs" ]

        deps = [ ... ]
      }

      {{ '<strong>' }}# Neither the component or package depend on ":echo_example" {{ '</strong>' }}
      fuchsia_component("component") {
        manifest = "meta/echo_example.cml"
        {{ '<strong>' }}deps = [] {{ '</strong>' }}
      }

      fuchsia_package("package") {
        package_name = "echo_example"
        {{ '<strong>' }}deps = [ ":component" ] {{ '</strong>' }}
      }
      ```

### Static capability analyzer {#troubleshoot-build-analyzer}

The Scrutiny static analyzer fails the build with the following error when it is unable
to verify each the [capability routes][glossary.capability-routing] in the
[component topology][glossary.component-topology]:

```none {:.devsite-disable-click-to-copy}
Static Capability Flow Analysis Error:
The route verifier failed to verify all capability routes in this build.
...

Verification Errors:
[
  {
    "capability_type": "directory",
    "results": { ... }
  },
  {
    "capability_type": "protocol",
    "results": { ... }
  },
]
```

This occurs when the analyze cannot successfully trace a capability route from its source
to the component requesting the capability through a valid chain of [`expose`][cml-expose]
and [`offer`][cml-offer] component manifest declarations.

In the following example, the error occurs due to the component `/core/echo` requesting to
`use` the `fuchsia.logger.LogSink` protocol without a corresponding `offer` for that capability
from the parent:

```none {:.devsite-disable-click-to-copy}
"errors": [
  {
    "using_node": "/core/echo",
    "capability": "fuchsia.logger.LogSink",
    "error": {
      "error": {
        "analyzer_model_error": {
          "routing_error": {
            "use_from_parent_not_found": {
              "moniker": {
                "path": [
                  {
                    "name": "core",
                    "collection": null,
                    "rep": "core"
                  },
                  {
                    "name": "echo",
                    "collection": null,
                    "rep": "echo"
                  }
                ]
              },
              "capability_id": "fuchsia.logger.LogSink"
            }
          }
        }
      },
      "message": "A `use from parent` declaration was found at `/core/echo` for `fuchsia.logger.LogSink`, but no matching `offer` declaration was found in the parent"
    }
  }
]
```

To address this issue explore the error details provided in the build failure to discover the
source of the routing error, then add or correct the invalid declarations in the routing chain.
In the previous example error, an `offer` should be added in the parent component's manifest:

```json5
{
    // ...

    children: [
        // ...
        {
            name: "echo",
            url: "echo#meta/default.cm",
        },
    ],
    offer: [
        // ...
        {{ '<strong>' }}{ {{ '</strong>' }}
            {{ '<strong>' }}protocol: "fuchsia.logger.LogSink", {{ '</strong>' }}
            {{ '<strong>' }}from: "parent", {{ '</strong>' }}
            {{ '<strong>' }}to: "#echo", {{ '</strong>' }}
        {{ '<strong>' }}}, {{ '</strong>' }}
    ],
}
```

For more details on building capability routes, see [Connect components][doc-connect].

[allowlist]: /tools/cmc/build/restricted_features/BUILD.gn
[cml-expose]: https://fuchsia.dev/reference/cml#expose
[cml-offer]: https://fuchsia.dev/reference/cml#offer
[cml-program]: https://fuchsia.dev/reference/cml#program
[components-migration]: /docs/contribute/open_projects/components/migration.md
[cpp-syslog]: /docs/development/languages/c-cpp/logging.md#component_manifest_dependency
[doc-connect]: /docs/development/components/connect.md
[doc-inspect]: /docs/development/diagnostics/inspect/README.md
[executable]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#func_executable
[ffx-scrutiny]: https://fuchsia.dev/reference/tools/sdk/ffx#scrutiny
[fx-test]: https://fuchsia.dev/reference/tools/fx/cmd/test.md
[fxb-55739]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=55739
[glossary.capability-routing]: /docs/glossary/README.md#capability-routing
[glossary.component]: /docs/glossary/README.md#component
[glossary.component-instance]: /docs/glossary/README.md#component-instance
[glossary.component-manifest]: /docs/glossary/README.md#component-manifest
[glossary.component-topology]: /docs/glossary/README.md#component-topology
[glossary.component-url]: /docs/glossary/README.md#component-url
[glossary.fuchsia-pkg-url]: /docs/glossary/README.md#fuchsia-pkg-url
[glossary.gn]: /docs/glossary/README.md#gn
[glossary.package]: /docs/glossary/README.md#fuchsia-package
[gn-get-target-outputs]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#func_get_target_outputs
[provide-data]: /docs/development/components/data.md
[rustc-binary]: /build/rust/rustc_binary.gni
[rustc-test]: /build/rust/rustc_test.gni
[source-code-layout]: /docs/development/source_code/layout.md
[source-expansion-placeholders]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#placeholders
[src-inspect-cpp]: /sdk/lib/sys/inspect/cpp/BUILD.gn
[src-inspect-include]: /sdk/lib/inspect/BUILD.gn
[src-inspect-rust]: /src/lib/diagnostics/inspect/runtime/rust/BUILD.gn
[test-environments]: /docs/contribute/testing/environments.md
[v2-test-component]: /docs/development/testing/components/test_component.md
[working-with-packages]: /docs/development/idk/documentation/packages.md
