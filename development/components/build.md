# Building components {#building-components}

This document demonstrates how to build and test a component, highlighting best
practices for defining packages, components, and their tests.

## Concepts {#concepts}

You should understand the following concepts before building a component:

**[Packages][glossary-package]** are the unit of software distribution on
Fuchsia. Packages are a collection of files with associated paths that are
relative to the base of the package. For instance, a package might contain an
ELF binary under the path `bin/hello_world`, and a JSON file under the path
`data/config.json`. Grouping files into a package is required in order to push
these files to the device.

**[Components][glossary-component]** are the unit of software execution on
Fuchsia. All software on Fuchsia except for the kernel image and usermode
bootstrap program is defined as a component.

A component is defined by a
**[component manifest][glossary-component-manifest]**. Components typically
include additional files, such as executables and data assets that they need at
runtime.

Developers must define their software in terms of packages and components,
whether for building production software or for writing their tests.

At runtime, **[Component instances][glossary-component-instance]** see the
contents of their package as read-only files under the path `/pkg`. Defining two
or more components in the same package doesn't grant each component access to
the other's capabilities. However it can guarantee to one component that the
other is available. Therefore if a component attempts to launch an instance of
another component, such as in an integration test, it can be beneficial to
package both components together.

Components are instantiated in a few ways, all of which somehow specify their
[URL][glossary-component-url]. Typically components are launched by specifying
their package names and path to their component manifest in the package, using
the **[`fuchsia-pkg://` scheme][glossary-fuchsia-pkg-url]**.

## GN templates

[GN][glossary-gn] is the meta-build system used by Fuchsia. Fuchsia extends GN
by defining templates. Templates provide a way to add to GN's built-in target
types. This section reviews various GN templates that can be used to define
packages, components, and their tests.

### Defining components, packages, and tests using GN templates {#defining}

Below is a hypothetical package containing one component that runs a C++
program and a data file. The example uses the following templates:

*   [`fuchsia_component.gni`](/build/components/fuchsia_component.gni)
*   [`fuchsia_package.gni`](/build/components/fuchsia_package.gni)

```gn
import("//build/components.gni")

executable("my_program") {
  sources = [ "my_program.cc" ]
}

fuchsia_component("my_component") {
  manifest = "meta/my_program.cmx"
  deps = [ ":my_program" ]
}

fuchsia_package("my_package") {
  deps = [ ":my_component" ]
}
```

The file `my_program.cmx` should include at least the following:

```json
{
    "program": {
        "binary": "bin/my_program"
    }
}
```

Note the following details:

*   This example imports `"//build/components.gni"`. This single import
    includes all templates related to packages, components, and testing them.
*   This example defines an `executable()`, which is used to build a C++
    program. This is for illustrative purposes - a component can launch all
    sorts of programs.
*   This example defines a `fuchsia_component()`, which depends on the
    `executable()`. The component definition attaches a manifest, which
    references the executable by its packaged path: `bin/my_program`.
    For more details on the packaged path, see
    [finding paths for built executables](#finding-paths-for-built-executables).
*   The manifest must be either a `.cmx` file in [cmx format][cmx-format] or a
    `.cml` file in [cml format][cml-format].
*   The destination path for the manifest is not specified, but rather
    inferred from the component's name. In this example, the manifest path will
    be `meta/my_component.cmx`.
*   Both the component and package names are derived from their target names.
    They both take an optional `component_name` and `package_name` parameter
    respectively as an override. \
    In the example above, these names come together to form the URL for
    launching the component:
    `fuchsia-pkg://fuchsia.com/my_package#meta/my_component.cmx`.

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
     manifest = "meta/my_component.cmx"
     deps = [ ":bin" ]
   }

   fuchsia_package("my_package") {
     deps = [ ":my_component" ]
   }
   ```

   It's assumed that the file `meta/my_component.cmx`
   contains at least the following:

   ```json
   {
     "program": {
        "binary": "bin/my_program"
     }
   }
   ```

   * {Rust}

   ```gn
   import("//build/rust/rustc_binary.gni")
   import("//build/components.gni")

   rustc_binary("bin") {
     output_name = "my_program"
   }

   fuchsia_component("my_component") {
     manifest = "meta/my_component.cmx"
     deps = [ ":bin" ]
   }

   fuchsia_package("my_package") {
     deps = [ ":my_component" ]
   }
   ```

   It's assumed that the file `meta/my_component.cmx`
   contains at least the following:

   ```json
   {
     "program": {
        "binary": "bin/my_program"
     }
   }
   ```

   * {Go}

   ```gn
   import("//build/go/go_binary.gni")
   import("//build/components.gni")

   go_binary("bin") {
     output_name = "my_program"
   }

   fuchsia_component("my_component") {
     manifest = "meta/my_component.cmx"
     deps = [ ":bin" ]
   }

   fuchsia_package("my_package") {
     deps = [ ":my_component" ]
   }
   ```

   It's assumed that the file `meta/my_component.cmx`
   contains at least the following:

   ```json
   {
     "program": {
        "binary": "bin/my_program"
     }
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
     manifest = "meta/my_component.cmx"
     deps = [ ":lib" ]
   }

   fuchsia_package("my_package") {
     deps = [ ":my_component" ]
   }
   ```

   It's assumed that the file `meta/my_component.cmx`
   contains at least the following:

   ```json
   {
     "program": {
        "data": "data/my_component"
     }
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
     manifest = "meta/my_component.cmx"
     deps = [ ":lib" ]
   }

   fuchsia_package("my_package") {
     deps = [ ":my_component" ]
   }
   ```

   It's assumed that the file `meta/my_component.cmx`
   contains at least the following:

   ```json
   {
     "program": {
        "data": "data/my_component"
     }
   }
   ```

### Test packages {#test-packages}

Test packages are packages that contain at least one component that is
launched as a test. Test packages are defined using
[`fuchsia_test_package.gni`](/build/components/fuchsia_test_package.gni). This
template can be used to define all sorts of tests, although it's most useful for
integration tests -- tests where other components in addition to the test itself
participate in the test. See [below](#unit-tests) for templates that specialize
in unit testing.

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
  manifest = "meta/my_test.cmx"
  deps = [ ":my_test" ]
}

executable("my_program_under_test") {
  sources = [ "my_program_under_test.cc" ]
}

fuchsia_component("my_other_component_under_test") {
  manifest = "meta/my_component_under_test.cmx"
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

*   This example defines `"my_test_component"`, which is assumed to implement a
    test. Commonly this is done using some testing framework such as C++
    Googletest, Rust Cargo test, etc'.
*   To launch this test, you can use [`fx test`][fx-test].
*   The test is packaged with another component,
    `"my_other_component_under_test"`. Presumably this component participates in
    the test. For instance, the component under test might implement a protocol,
    and the test launches it and connects to it as a client while asserting
    correct behavior from the client's perspective. \
    Packaging the component under test together with the test component
    guarantees that the component under test is available for launch while
    the test is running, and built at the same version as the test. If
    this weren't the case, and instead the test assumed that the component under
    test was present in another package that's already installed on the target
    device, then the test would be exposed to side effects and version skew.
    Packaging the test with its dependencies makes it more hermetic.
*   Note the `environments` parameter. `fuchsia_test_package()` can optionally
    take [`test_spec.gni`](/build/testing/test_spec.gni) parameters to override
    the default testing behavior. In this example, this test is configured to
    run on VIM3 devices.
*   Finally, this example defines a `group()` to contain all the tests (which we
    have exactly one of). This is a [recommended practice][source-code-layout]
    for organizing targets across the source tree.

An important limitation of `fuchsia_test_package()` is that any
`test_component` targets must be defined in the same `BUILD.gn` file as the
test package target. This is due to a [limitation in GN][gn-get-target-outputs].

It's possible to work around this limitation with an indirection through
`fuchsia_test()`. In one `BUILD.gn` file, define:

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
  manifest = "meta/my_test.cmx"
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

This is slightly more verbose but achieves the same outcome.

#### Dart and Flutter tests

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
  manifest = "meta/my_flutter_test_component.cmx"
  sources = [ "foo_flutter_test.dart" ]
}

dart_test_component("my_dart_test_component") {
  testonly = true
  manifest = "meta/my_dart_test_component.cmx"
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

Since unit tests are very common, two simplified templates are provided:

* [`fuchsia_unittest_component.gni`](/build/components/fuchsia_unittest_component.gni) defines a
  component to be run as a test, with the option to automatically generate a basic component
  manifest, that must then be included in a package.
* [`fuchsia_unittest_package.gni`](/build/components/fuchsia_unittest_package.gni) defines a
  package with a single component to be run as a test, shorthand for a single
  `fuchsia_unittest_component()` target paired with a `fuchsia_test_package()`.

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
     manifest = "meta/my_test.cmx"
     deps = [ ":my_test" ]
   }
   ```

   * {Rust}

   ```gn
   import("//build/rust/rustc_test.gni")
   import("//build/components.gni")

   rustc_test("my_test") {}

   fuchsia_unittest_package("my_test") {
     manifest = "meta/my_test.cmx"
     deps = [ ":my_test" ]
   }
   ```

   * {Go}

   ```gn
   import("//build/go/go_test.gni")
   import("//build/components.gni")

   go_test("my_test") {}

   fuchsia_unittest_package("my_test") {
     manifest = "meta/my_test.cmx"
     deps = [ ":my_test" ]
   }
   ```

The manifest file `meta/my_test.cmx` may look like this:

```json
{
    "program": {
        "binary": "bin/my_test"
    }
}
```

The above is a minimal valid manifest file for this test. In practice a test
might require additional capabilities, to be specified in its manifest.

The launch URL for the test is
`fuchsia-pkg://fuchsia.com/my_test#meta/my_test.cmx`. It can be launched using
`fx test` followed by the launch URL, or followed by the GN target name.

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

   rustc_test("rot13_test") {}

   fuchsia_unittest_package("rot13_test") {
     deps = [ ":rot13_test" ]
   }
   ```

   * {Go}

   ```gn
   import("//build/go/go_test.gni")
   import("//build/components.gni")

   go_test("rot13_test") {}

   fuchsia_unittest_package("rot13_test") {
     deps = [ ":rot13_test" ]
   }
   ```

It's also possible to generate multiple unit test components and include them in
a single package.

```gn
import("//build/components.gni")

fuchsia_unittest_component("rot13_encrypt_test") {
  ...
}

fuchsia_unittest_component("rot13_decrypt_test") {
  ...
}

fuchsia_test_package("rot13_tests") {
  test_components = [
    ":rot13_encrypt_test",
    ":rot13_decrypt_test",
  ]
}
```

The generated component manifest file can be found as follows:

<pre class="prettyprint">
<code class="devsite-terminal">fx gn outputs out/default <var>unittest target</var>_generated_manifest</code>
</pre>

To print it directly:

<pre class="prettyprint">
<code class="devsite-terminal">fx build && cat out/default/$(fx gn outputs out/default <var>unittest target</var>_generated_manifest)</code>
</pre>

Note that `fx gn outputs` prints an output path, but the file at the path
may not exist or may be stale if you haven't built.

To launch the test:

```bash
# By launch URL
fx test fuchsia-pkg://fuchsia.com/rot13_test#meta/rot13_test.cmx
# By GN target name
fx test rot13_test
```

See also: [`fx test`][fx-test]

You can generate a [CFv2 test component][v2-test-component] by specifying:

```gn
import("//build/components.gni")

fuchsia_unittest_package("rot13_test") {
  v2 = true
  ...
}
```

Or:

```gn
import("//build/components.gni")

fuchsia_unittest_component("rot13_encrypt_test") {
  v2 = true
  ...
}

fuchsia_unittest_component("rot13_decrypt_test") {
  v2 = true
  ...
}

fuchsia_test_package("rot13_tests") {
  test_components = [
    ":rot13_encrypt_test",
    ":rot13_decrypt_test",
  ]
}
```

#### Multiple unit tests in a single package

The `fuchsia_unittest_component()` rule can be used instead of
`fuchsia_unittest_package()` to include multiple components in a single
`fuchsia_test_package()`. This can be useful to easily run all the test components
in a single package, e.g. with `fx test <package_name>`, rather than needing to
type many separate package names.

The example below creates a single test package `rot13_tests` that contains two
separate test components, `rot13_decoder_test` and `rot13_encoder_test`.
Both tests can be run using `fx test rot13_tests`, or individual tests can be run
using either `fx test rot13_decoder_test` or the full URL
`fx test fuchsia-pkg://fuchsia.com/rot13_tests#meta/rot13_decoder_test.cmx`.

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
     manifest = "meta/rot13.cmx"
     deps = [ ":rot13_encoder_decoder" ]
   }
   ```

   * {Rust}

   ```gn
   import("//build/rust/rustc_binary.gni")
   import("//build/components.gni")

   rustc_binary("rot13_encoder_decoder") {
   }

   fuchsia_package_with_single_component("rot13") {
     manifest = "meta/rot13.cmx"
     deps = [ ":rot13_encoder_decoder" ]
   }
   ```

   * {Go}

   ```gn
   import("//build/go/go_binary.gni")
   import("//build/components.gni")

   go_binary("rot13_encoder_decoder") {
   }

   fuchsia_component("rot13") {
     manifest = "meta/rot13.cmx"
     deps = [ ":rot13_encoder_decoder" ]
   }
   ```

## Test-driven development

The `fx smoke-test` command automatically detects all tests that are known to
the build system as affected by changes in your checkout. Try the following:

<pre class="prettyprint">
<code class="devsite-terminal">fx -i smoke-test --verbose</code>
</pre>

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
`resource()` and `resource_group()`.

### Example: fonts

{# Disable variable substition to avoid {{ being interpreted by the template engine #}
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

{# Re-enable variable substition #}
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

{# Disable variable substition to avoid {{ being interpreted by the template engine #}
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

{# Re-enable variable substition #}
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

The underlying behavior of `resource()` and `resource_group()` is identical.
You are free to choose whichever one you prefer.

## Component manifest includes {#component-manifest-includes}

As shown above, component declarations have an associated [component
manifest][glossary-component-manifest]. The component manifest supports
"include" syntax, which allows referencing one or more files where additional
contents for the component manifest may be merged from. This is conceptually
similar for instance to `#include` directives in the C programming language.
These included files are also known as component manifest shards.

Some dependencies, such as libraries, assume that dependent components have
certain capabilities available to them at runtime.
Practically this could mean that the code in question assumes that its
dependents include a certain file in their component manifests. For instance,
the [C++ Syslog library][cpp-syslog] makes such an assumption.

Target owners can declare that dependent components must include one or more
files in their component manifest. For example we have the hypothetical file
`//sdk/lib/fonts/BUILD.gn` below:

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
    "client.shard.cmx",
    "client.shard.cml",
  ]
}
```

It is possible (and recommended) to provide both `.cmx` and `.cml` includes.
Dependent manifests are required to include the expected files with the
matching extension.

   * {.cmx}

   ```json
   {
       "include": [
           "sdk/lib/fonts/client.shard.cmx"
       ]
       ...
   }
   ```

   * {.cml}

   ```json5
   {
       include: [
           "sdk/lib/fonts/client.shard.cml",
       ]
       ...
   }
   ```

Include paths are resolved relative to the source root.
Transitive includes (includes of includes) are allowed.
Cycles are not allowed.

By convention, component manifest shard files are named with the suffix
`.shard.cmx` or `.shard.cml`.

When naming your shards, don't repeat yourself in relation to the full path.
In the example above it would have been repetitive to name the shard
`fonts.shard.cml` because then the full path would have been
`sdk/lib/fonts/fonts.shard.cml`, which is repetitive. Instead the file is
named `client.shard.cml`, to indicate that it is to be used by clients of the
SDK library for fonts.

## Troubleshooting {#troubleshooting}

### Listing the contents of a package {#listing-the-contents-of-a-package}

Packages are described by a package manifest, which is a text file where every
line follows this structure:

```
<packaged-path>=<source-file-path>
```

To find the package manifest for a `fuchsia_package()` or
`fuchsia_test_package()` target, use the following command:

<pre class="prettyprint">
<code class="devsite-terminal">fx gn outputs out/default <var>package target</var>_manifest</code>
</pre>

The package target is a fully-qualified target name, i.e. in the form
`//path/to/your:target`.

Combine this with another command to print the package manifest:

<pre class="prettyprint">
<code class="devsite-terminal">cat out/default/$(fx gn outputs out/default <var>package target</var>_manifest)</code>
</pre>

See also:

*   [Working with packages][working-with-packages]
*   [pm]

### Finding paths for built executables {#finding-paths-for-built-executables}

Executable programs can be built with various language-specific templates such
as `executable()`, `rustc_binary()`, `go_binary()` etc'. These templates are
responsible for specifying where in a package their output binaries should be
included. The details vary by runtime and toolchain configuration.

*   Typically the path is `bin/` followed by the target's name.
*   Typically if an `output_name` or `name` is specified, it overrides the
    target name.

Some rudimentary examples are given below:

   * {C++}

   ```gn
   # Binary is packaged as `bin/rot13_encode`
   executable("rot13_encode") {
     sources = [ "main.cc" ]
   }
   ```

   * {Rust}

   ```gn
   # Binary is packaged as `bin/rot13_encode`
   rustc_binary("rot13_encode") {}
   ```

   * {Go}

   ```gn
   # Binary is packaged as `bin/rot13_encode`
   go_binary("rot13_encode") {}
   ```

In order to reference an executable in a component manifest, the author needs
to know its packaged path.

One way to find the packaged path for an executable is to make sure that the
target that builds the executable is in a package's `deps`, then follow
[listing the contents of a package](#listing-the-contents-of-a-package).
The executable is listed among the contents of the package.

### Finding a [component's launch URL][glossary-component-url]

Component URLs follow this pattern:

```none
fuchsia-pkg://fuchsia.com/<package-name>#meta/<component-name>.<extension>
```

*   `<package-name>`: specified as `package_name` on the package target, which
    defaults to the target name.
*   `<component-name>`: specified as `component_name` on the component target,
    which defaults to the target name.
*   `<extension>`: based on the [component
    manifest][glossary-component-manifest] - `cmx` for cmx files, `cm` for cml
    files.

## Migrating from legacy build rules {#legacy-package-migration}

The examples below demonstrate migration scenarios from the legacy
[`package()`](/build/package.gni) template to the new
[`fuchsia_package()`](/build/components/fuchsia_package.gni) & friends.

### Simple `package()` example

This example is adapted from
[`//src/sys/component_index/BUILD.gn`](/src/sys/component_index/BUILD.gn).

* {Pre-migration}

  ```gn
  import("//build/config.gni")
  import("//build/package.gni")                              # <1>
  import("//build/rust/rustc_binary.gni")
  import("//build/test/test_package.gni")                    # <1>

  rustc_binary("component_index_bin") {                      # <2>
    name = "component_index"
    # Generate a ":bin_test" target for unit tests
    with_unit_tests = true
    edition = "2018"
    deps = [ ... ]
  }

  package("component_index") {                               # <3>
    deps = [ ":component_index_bin" ]

    binaries = [
      {
        name = "component_index"
      },
    ]

    meta = [
      {
        path = rebase_path("meta/component_index.cmx")       # <4>
        dest = "component_index.cmx"                         # <4>
      },
    ]

    resources = [ ... ]
  }

  test_package("component_index_tests") {                    # <5>
    deps = [ ":component_index_bin_test" ]

    tests = [
      {
        name = "component_index_bin_test"                    # <5>
        dest = "component_index_tests"                       # <5>
      },
    ]
  }
  ```

* {Post-migration}

  ```gn
  import("//build/config.gni")
  import("//build/rust/rustc_binary.gni")
  import("//build/components.gni")                   # <1>

  rustc_binary("component_index_bin") {                      # <2>
    name = "component_index"
    # Generate a ":bin_test" target for unit tests
    with_unit_tests = true
    edition = "2018"
    deps = [ ... ]
  }

  fuchsia_package_with_single_component("component_index") { # <3>
    manifest = "meta/component_index.cmx"                    # <4>
    deps = [ ":component_index_bin" ]
  }

  fuchsia_unittest_package("component_index_tests") {        # <5>
    deps = [ ":component_index_bin_test" ]
  }
  ```

The following key elements are called out in the code example above:

> 1.  Necessary imports are replaced by `//build/components.gni`.
> 2.  Targets that generate executables or data files are not expected to change
>     in a migration.
> 3.  The original `package()` declaration contains a single component manifest
>     (listed under `meta`). The `fuchsia_package_with_single_component()`
>     template can replace this, referencing the same manifest file.
> 4.  Under `package()`, the manifest is given a specific destination
>     (`"component_index.cmx"`) to place it in the final package. With the new
>     templates, the manifest is renamed according to the **target name**.
>     As a result, the launch URL for the component remains the same.
> 5.  For a simple test package that does not contain multiple test components,
>     the `fuchsia_unittest_package()` template replaces `test_package()`. A
>     basic test component manifest is automatically generated and
>     `meta/component_index_tests.cmx` is no longer needed.

### Complex `package()` example

This example is adapted from
[`//src/fonts/BUILD.gn`](/src/fonts/BUILD.gn).

* {Pre-migration}

  ```gn
  import("//build/package.gni")                            # <1>
  import("//build/rust/rustc_binary.gni")
  import("//build/test/test_package.gni")                  # <1>
  import("//src/fonts/build/fonts.gni")

  rustc_binary("font_provider") {                          # <2>
    name = "font_provider"
    # Generate a ":bin_test" target for unit tests
    with_unit_tests = true
    edition = "2018"

    deps = [ ... ]
    sources = [ ... ]
  }

  package("pkg") {
    package_name = "fonts"

    deps = [ ":font_provider" ]

    binaries = [
      {
        name = "font_provider"
      },
    ]
    meta = [                                               # <3>
      {
        path = rebase_path("meta/fonts.cmx")               # <3>
        dest = "fonts.cmx"                                 # <4>
      },
      {
        path = rebase_path("meta/fonts.cml")               # <3>
        dest = "fonts.cm"                                  # <4>
      },
    ]
  }

  test_package("font_provider_unit_tests") {
    deps = [ ":font_provider_test" ]

    v2_tests = [
      {
        name = "font_provider_bin_test"                    # <4>
      },
    ]
  }
  ```

* {Post-migration}

  ```gn
  import("//build/rust/rustc_binary.gni")
  import("//src/fonts/build/fonts.gni")
  import("//build/components.gni")                         # <1>

  rustc_binary("font_provider") {                          # <2>
    name = "font_provider"
    # Generate a ":bin_test" target for unit tests
    with_unit_tests = true
    edition = "2018"

    deps = [ ... ]
    sources = [ ... ]
  }

  fuchsia_component("font_provider_cm") {                  # <3>
    manifest = "meta/fonts.cml"
    component_name = "fonts"                               # <4>
    deps = [ ":font_provider" ]
  }

  fuchsia_component("font_provider_cmx") {                 # <3>
    manifest = "meta/fonts.cmx"
    component_name = "fonts"                               # <4>
    deps = [ ":font_provider" ]
  }

  fuchsia_package("pkg") {
    package_name = "fonts"
    deps = [
      ":font_provider_cm",                                 # <3>
      ":font_provider_cmx",                                # <3>
    ]
  }

  fuchsia_component("font_provider_unit_tests_cmp") {
    testonly = true
    manifest = "meta/font_provider_bin_test.cml"
    component_name = "font_provider_bin_test"              # <4>
    deps = [ ":font_provider_test" ]
  }

  fuchsia_test_package("font_provider_unit_tests") {
    test_components = [ ":font_provider_unit_tests_cmp" ]
  }
  ```

The following key elements are called out in the code example above:

> 1.  Necessary imports are replaced by `//build/components.gni`.
> 2.  Targets that generate executables or data files are not expected to change
>     in a migration.
> 3.  If a `package()` includes multiple distinct components using the `meta`
>     field, each one must be broken out into a separate `fuchsia_component()`
>     and collected together in the `fuchsia_package()` using `deps`.
> 4.  Each `fuchsia_component()` uses the `component_name` field to map the
>     manifest destination in the final package. Without this, they are placed
>     according to the **target name**, which affects the launch URL of the
>     component.
>     This is true for both `fuchsia_package()` and `fuchsia_test_package()`.

Note: The new build templates allow targets that produce files, such as
`executable()`, to decide which files they produce and where the targets place
these files. This may affect the packaged path to binaries in your manifest or
test definition after migrating. If you encounter build-time errors you are
unable to resolve, see [Troubleshooting](#troubleshooting).

### Test package considerations

The example below highlights some key differences between the legacy
[`test_package()`](/build/test/test_package.gni) template and the new
[`fuchsia_test_package()`](/build/components/fuchsia_test_package.gni).

* {Pre-migration}

  ```gn
  import("//build/package.gni")                            # <1>
  import("//build/test/test_package.gni")                  # <1>

  executable("foo_bin_test") { ... }

  test_package("foo_tests") {                              # <1>
    deps = [ ":foo_bin_test" ]                             # <2>

    tests = [                                              # <3>
      {
        name = "foo_test"                                  # <2>
        log_settings = {
          max_severity = "ERROR"
        }
      }
    ]
  }
  ```

* {Post-migration}

  ```gn
  import("//build/components.gni")                         # <1>

  executable("foo_bin_test") { ... }

  fuchsia_component("foo_test") {                          # <2>
    testonly = true
    manifest = "meta/foo_test.cmx"
    deps = [ ":foo_bin_test" ]
  }

  fuchsia_test_package("foo_tests") {                      # <1>
    test_components = [ ":foo_test" ]                      # <2>

    test_specs = {                                         # <3>
      log_settings = {
        max_severity = "ERROR"
      }
    }
  }
  ```

The following key elements are called out in the code example above:

> 1.  Replace necessary imports with `//build/components.gni` and rename
>     `test_package()` to `fuchsia_test_package()`.
> 2.  Create a `fuchsia_component()` to encapsulate the test components previously
>     added with the `tests` field. Reference the components in the package with
>     the new `test_components` field.
>
>     Note: A `test_package()` typically sets the packaged path for binaries to
>     `test/`, while the new build rules let the executables define this and they
>     typically use `bin/`. This may affect the packaged path to binaries in your
>     test definition after migrating. If you encounter build-time errors you are
>     unable to resolve, see [Troubleshooting](#troubleshooting).
>
> 3.  Both template families support test specifications, such as restricting to
>     specific [test environments][test-environments] or
>     [restricting log severity][restrict-log-severity].
>
>     Note: With the new templates, the `test_specs` apply to all tests in the package.
>     See [test packages](#test-packages) for more examples.

### Remove legacy allowlist

The `deprecated_package` group in [`//build/BUILD.gn`](/build/BUILD.gn) contains
an allowlist of build files still using the legacy `package()` template.
Once you have successfully migrated your build files to the new templates,
remove the affected lines from the group. Removing the allowlist entries prevents
future changes from re-introducing uses of the legacy templates.

For example, if you migrated the files under [`//src/fonts`](/src/fonts) to the
new templates, you would find and remove all the related files paths in
[`//build/BUILD.gn`](/build/BUILD.gn):

```gn
group("deprecated_package") {
  ...
  visibility += [
    ...
    "//src/fonts/*",
    "//src/fonts/char_set/*",
    "//src/fonts/font_info/*",
    "//src/fonts/manifest/*",
    "//src/fonts/offset_string/*",
    "//src/fonts/tests/integration/*",
    "//src/fonts/tests/smoke/*",
    ...
  ]
}
```

### Legacy features

The following special attributes are supported by the legacy `package()` template:

*   `binaries`
*   `drivers`
*   `libraries`
*   `loadable_modules`

These are used with special syntax, which determines how the files that certain
targets produce are packaged.
For instance the `libraries` attribute installs resources in a special `lib/` directory,
`drivers` are installed in `drivers/`, etc'.
The legacy syntax looks like this:

```
package("my_driver_package") {
  deps = [ ":my_driver" ]

  drivers = [
    {
      name = "my_driver.so"
    },
  ]
}
```

This special treatment is not necessary with the new templates. Simply add the
necessary target to `deps = [ ... ]` and the packaging is done automatically.

```
fuchsia_component("my_driver_component") {
  deps = [ ":my_driver" ]
  ...
}

fuchsia_package("my_driver_package") {
  deps = [ ":my_driver_component" ]
  ...
}
```

Additionally, legacy `package()` supports the `resources` attribute. This is
replaced by adding a dependency on a `resource()` target.
See also:

*   [Listing the contents of a package](#listing-the-contents-of-a-package).
*   [Additional packaged resources](#additional-packaged-resources).

### Renaming files

The legacy `package()` template allowed developers to rename certain files that
are included in their package. For example, below we see an executable being
built and then renamed before it's packaged so that it's packaged under the path
`bin/foo_bin`.

```gn
import("//build/package.gni")

executable("bin") {
  ...
}

package("foo_pkg") {
  deps = [ ":bin" ]
  binaries = [
    {
      name = "bin"
      dest = "foo_bin"
    }
  ]
  meta = [
    {
      path = "meta/foo_bin.cmx"
      dest = "foo.cmx"
    }
  ]
}
```

The new templates allow targets that produce files, such as `executable()`
above, to decide which files they produce and where they're placed. This is
important because some targets produce multiple files, or might produce
different files based on the build configuration (for instance if building
for a different target architecture). In order to control the paths of
packaged files, developers should work with the templates for the targets
that produce those files. For instance:

```gn
import("//build/components.gni")

executable("bin") {
  output_name = "foo_bin"
  ...
}

fuchsia_component("foo_cmp") {
  deps = [ ":bin" ]
  manifest = "meta/foo_bin.cmx"
}

fuchsia_package("foo_pkg") {
  deps = [ ":foo_cmp" ]
}
```

### Shell binaries

The legacy `package()` template allowed developers to make a particular binary
in the package available to `fx shell`.

```gn
import("//build/package.gni")

# `fx shell echo Hello World` prints "Hello World"
executable("bin") {
  output_name = "echo"
  ...
}

package("echo") {
  binaries = [
    {
      name = "echo"
      dest = "echo"
      shell = true
    }
  ]
  deps = [ ":bin" ]
}
```

The new templates support this feature as follows:

```gn
import("//build/components.gni")

# `fx shell echo Hello World` prints "Hello World"
executable("bin") {
  output_name = "echo"
  ...
}

fuchsia_shell_package("echo") {
  deps = [ ":bin" ]
}
```

Note that in the `package()` example the binary is explicitly named "echo",
which is the same name that's used for its intrinsic name
(`output_name = "echo"`). The new templates don't have this renaming behavior,
and instead let the target that produces the binary (`executable()` in this
case) decide the file name, as determined by the `output_name` specified (or the
executable target's name if `output_name` isn't specified).

This feature was left out intentionally.
Moving forward the use of legacy shell tools is discouraged.

### Go `grand_unified_binary`

"Grand unified binary" (GUB) is a single binary that merges together multiple Go
programs. The entry point to the combined program can identify which sub-program
the caller intends to run based on the filename of the invocation (`argv[0]`).
Therefore in order to include GUB in your package and invoke a sub-program the
common practice is to rename the binary.

The legacy `package()` template allowed developers to accomplish this as shown
below:

```gn
import("//build/go/go_library.gni")
import("//build/package.gni")

go_library("my_tool") {
  ...
}

package("tools") {
  deps = [
    "//src/go/grand_unified_binary",
  ]
  binaries = [
    {
      name = "my_tool"
      source = "grand_unified_binary"
    }
  ]
}
```

The new templates support this feature as follows:

```gn
import("//build/go/go_library.gni")
import("//src/go/grand_unified_binary/gub.gni")
import("//build/components.gni")

go_library("my_tool") {
  ...
}

grand_unified_binary("bin") {
  output_name = "my_tool"
}

fuchsia_package("tools") {
  deps = [ ":bin" ]
}
```

### Legacy component index (aka `fx run my_package`)

The legacy `package()` template supported a short-form syntax for launching legacy
v1 components in the legacy sys shell.

```gn
import("//build/package.gni")

executable("bin") {
  output_name = "echo"
  sources = [ "echo.cc" ]
}

package("echo") {
  deps = [ ":bin" ]
  binaries = [
    {
      name = "echo"
    },
  ]
  meta = [
    {
      path = "meta/echo.cmx"
      dest = "echo.cmx"
    },
  ]
}
```

```posix-terminal
fx run echo Hello World
```

This is also known as the [Component Index][component-index].

The new templates don't support this feature out of the box, but you can use the
full launch URL.

```posix-terminal
fx run fuchsia-pkg://fuchsia.com/echo#meta/echo.cmx Hello World
```

The plan is to deprecate the legacy shell and the legacy component index along
with it, but there is currently no concrete timeline for this deprecation. If
you'd like to keep the old behavior, you can do so with this special syntax:

```gn
import("//build/components.gni")
import("//src/sys/component_index/component_index.gni")

executable("bin") {
  output_name = "echo"
  sources = [ "echo.cc" ]
}

add_to_component_index("component_index") {
  package_name = "echo"
  manifest = "meta/echo.cmx"
}

fuchsia_package_with_single_component("echo") {
  deps = [
    ":bin",
    ":component_index",
  ]
  manifest = "meta/echo.cmx"
}
```

### Other unsupported features

Note that some features of `package()` are unsupported moving forward. If your
package depends on them then at this time it cannot be migrated to the new
templates. These unsupported features include:

*   Marking a test as disabled. Instead, change the test source code to mark it
    as disabled, or comment out the disabled test component from the build file.
*   `__deprecated_system_image`: the legacy approach to including a package in
    the system image is not supported moving forward. A solution is being
    prepared and will be available later in 2021.
    Nearly all existing uses of this legacy feature are done via the
    `driver_package()` wrapper, which currently cannot be migrated.

[cpp-syslog]: /docs/development/languages/c-cpp/logging.md#component_manifest_dependency
[cml-format]: /docs/concepts/components/v2/component_manifests.md
[cmx-format]: /docs/concepts/components/v1/component_manifests.md
[component-index]: /src/sys/component_index/component_index.gni
[executable]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#func_executable
[fx-test]: https://www.fuchsia.dev/reference/tools/fx/cmd/test.md
[fxb-55739]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=55739
[glossary-component]: /docs/glossary.md#component
[glossary-component-instance]: /docs/glossary.md#component-instance
[glossary-component-manifest]: /docs/glossary.md#component-manifest
[glossary-component-url]: /docs/glossary.md#component-url
[glossary-fuchsia-pkg-url]: /docs/glossary.md#fuchsia-pkg-url
[glossary-gn]: /docs/glossary.md#gn
[glossary-package]: /docs/glossary.md#fuchsia-package
[gn-get-target-outputs]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#func_get_target_outputs
[pm]: /src/sys/pkg/bin/pm/README.md
[restrict-log-severity]: /docs/concepts/testing/logs.md#restricting_log_severity
[rustc-binary]: /build/rust/rustc_binary.gni
[rustc-test]: /build/rust/rustc_test.gni
[source-code-layout]: /docs/concepts/source_code/layout.md
[source-expansion-placeholders]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#placeholders
[test-environments]: /docs/concepts/testing/environments.md
[v2-test-component]: /docs/concepts/testing/v2/v2_test_component.md
[working-with-packages]: /docs/development/idk/documentation/packages.md
