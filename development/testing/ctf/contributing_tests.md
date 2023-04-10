# Contributing Tests to CTF

<!--
TODO(113454): Turn this doc into an index of guides for writing CTF tests
for different plasa elements (fidls, C++ libraries, tools, etc...)
-->

This guide will walk you through the process of writing a CTF test for a FIDL API.

{% set fidl_library = "fuchsia.examples" %}
{% set fidl_service = "fuchsia.examples.Echo" %}

To test a FIDL API, we write a test that uses the API's client bindings in the SDK to
interact with the FIDL service. For this guide we'll be testing an example FIDL service
`{{ fidl_service }}` from the library `{{ fidl_library }}`. Throughout this guide, you
can replace the library and service with your own values to match your use case.

The test will consist of two components: The first is a __test driver binary__ which
implements the core test logic, and is the part of your test that will be released in CTF.
The second is a __test realm__ which provides the capabilities and dependencies that we
want to test, is always built from the sources at HEAD, and is not released as part of CTF.

## Requirements

* Tests must be written in C, C++, or Rust.
* Tests must only depend on APIs, ABIs, and tools in partner facing SDKs.

Note: Exceptions for dependencies are made by updating the allowlist [here][allow list].

Note: The examples in this guide use synchronous FIDL clients, but this is not a requirement.
Test authors can choose to use synchronous or asynchronous clients.

If you are writing a CTF test for a FIDL service whose `fidl` target is not in the partner
SDK category, please see this section.

## Concepts

* [Components]
* [Component Manifests]
* [Packages]
* [Test Components]
* [Subpackages]

## Steps

### 1. Setup

First create a directory for the test. The directory name should match the name of the
FIDL library. You can copy these commands to generate some scaffolding:

{% set test_root = "sdk/ctf/tests/fidl/" + fidl_library %}
{% set test_component_name = fidl_service|lower + "_test" %}

  * {C/C++}

    ```sh
    mkdir {{ test_root }}
    mkdir {{ test_root }}/meta/
    touch {{ test_root }}/meta/{{ test_component_name }}.cml
    touch {{ test_root }}/BUILD.gn
    touch {{ test_root }}/main.cc
    ```

  * {Rust}

    ```sh
    mkdir {{ test_root }}
    mkdir {{ test_root }}/meta/
    # Rust tests need an additional component to offer the subpackaged test runner.
    touch {{ test_root }}/meta/{{ test_component_name }}_root.cml
    touch {{ test_root }}/meta/{{ test_component_name }}.cml
    touch {{ test_root }}/BUILD.gn
    touch {{ test_root }}/main.rs
    ```

### 2. Create the test realm

The test realm is a component whose sole purpose is to `expose` the FIDL API that we want to test.
The test realm component is always built from the HEAD of the current Fuchsia branch. This is what
makes the CTF test a compatibility test: The test and the FIDL capability it's testing are built at
different versions.

For convenience, the test realm component can be defined anywhere in the source tree but we prefer
if all realms are defined in `//sdk/ctf/test_realm/BUILD.gn.`

To create the realm, add contents like the following to `//sdk/ctf/test_realm/BUILD.gn`:

```build
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/ctf/test_realm/BUILD.gn" region_tag="example" adjust_indentation="auto" %}
```

{% set test_realm_package = fidl_library + "_test_realm" %}

Then create the test realm's component manifest at `//sdk/ctf/test_realm/meta/{{ test_realm_package }}.cml`:

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/ctf/test_realm/meta/fuchsia.example_test_realm.cml" region_tag="example" adjust_indentation="auto" %}
```

Finally, add the realm's label to the list in `//sdk/ctf/build/ctf_test_realms.gni`.
This will cause the build to include the test realm as a [subpackage][Subpackages] of your
test. We'll explain this in more detail, later.

### 3. Write the BUILD.gn file

Add contents like the following to `//{{ test_root }}/BUILD.gn` to define an
executable, test driver component, and package for your test. Be sure to add
the `:tests` target as a dependency of `//sdk/ctf/tests/fidl:tests` in order
to include the test in the build graph for CI and CQ builds.

  * {C/C++}

    ```build
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/ctf/tests/examples/fidl/fuchsia.examples/cc/BUILD.gn" region_tag="build" adjust_indentation="auto" %}
    ```

  * {Rust}

    ```build
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/ctf/tests/examples/fidl/fuchsia.examples/rust/BUILD.gn" region_tag="build" adjust_indentation="auto" %}
    ```

### 4. Implement the test driver

This component implements the core logic for your test.

First we need to create the component manifest. Add contents like the following
to `{{ test_component_name }}.cml`:

  * {C/C++}

    ```json5
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/ctf/tests/examples/fidl/fuchsia.examples/cc/meta/fuchsia.examples.echo_test.cml" region_tag="example" adjust_indentation="auto" %}
    ```

  * {Rust}

    This test root component includes `//sdk/ctf/meta/rust.shard.cml`, which defines the rust test runner as a subpackage. The `echo_test` component must be started in the `subpackaged-runner-env`.

    ```json5
    // echo_test_root.cml

    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/ctf/tests/examples/fidl/fuchsia.examples/rust/meta/fuchsia.examples.echo_test_root.cml" region_tag="example" adjust_indentation="auto" %}
    ```

    The test component. The test realm is offered from the test root rather than the test itself.

    ```json5
    // echo_test.cml

    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/ctf/tests/examples/fidl/fuchsia.examples/rust/meta/fuchsia.examples.echo_test.cml" region_tag="example" adjust_indentation="auto" %}
    ```

The package URL `{{ test_realm_package }}#meta/default.cm` loads the test realm we created
earlier from a subpackage. Every time this test runs it receives a new version of this
subpackage which is built from the current commit, regardless of whether the test itself
is built from the current commit or obtained as a prebuilt.

Next we need to implement the executable. Add contents like the following to the test's
source file:

  * {C/C++}

    ```C++
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/ctf/tests/examples/fidl/fuchsia.examples/cc/main.cc" region_tag="example" adjust_indentation="auto" %}
    ```

  * {Rust}

    ```rust
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/ctf/tests/examples/fidl/fuchsia.examples/rust/main.rs" region_tag="example" adjust_indentation="auto" %}
    ```

### 5. Running the test

These instructions require you to open several terminal tabs.
Follow the insructions for each tab, from left to right:

  * {Build Fuchsia}

    ```devsite-terminal
    fx set core.x64 --with //{{ test_root }}:tests
    fx build
    ```

  * {Run the emulator}

    ```devsite-terminal
    ffx emu start --headless
    ```

  * {Serve packages}

    ```devsite-terminal
    fx serve
    ```

  * {Stream logs}

    ```devsite-terminal
    ffx log
    ```

  * {Run the tests}

    ```devsite-terminal
    fx test -v {{ test_component_name }}
    # -v enables verbose output.
    ```

If you need additional help debugging at this step, please reach out to fuchsia-ctf-team@google.com.

### 6. Submit the changes

If the tests pass, send your changes for review. After submission the tests willji
automatically be included in the next CTF release when the next milestone branch
is cut.

## Testing experimental FIDL APIs

You can follow this guide to write a CTF test for a FIDL API that is not in the partner
SDK category, but you must not release the test in CTF until the API has been added to the
partner category. To prevent the test from being released, set `release_in_ctf = false` on
the test package:

```build
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/ctf/tests/examples/experimental_fidl/BUILD.gn" region_tag="norelease_example" adjust_indentation="auto" %}
```

## See Also

The FAQ sections about [retiring tests] and [disabling tests].

[Component Manifests]: /concepts/components/v2/component_manifests.md
[Components]: /concepts/components/v2
[Fuchsia language policy]: /contribute/governance/policy/programming_languages.md
[Packages]: /concepts/packages/package.md
[Start the Fuchsia Emulator]: /get-started/set_up_femu.md
[Test Components]: /development/testing/components/test_component.md
[file a bug]: https://bugs.fuchsia.dev/p/fuchsia/issues/list?q=component%3ADeveloperExperience%3ECTS
[relative component URL]: /reference/components/url.md#relative
[CTF bug component]: https://bugs.fuchsia.dev/p/fuchsia/templates/detail?saved=1&template=Fuchsia%20Compatibility%20Test%20Suite%20%28CTS%29&ts=1627669234
[disabling tests]: /development/testing/ctf/faq.md#disable-a-test
[retiring tests]: /development/testing/ctf/faq.md#retire-a-test
[allow list]: /sdk/ctf/build/internal/allowed_ctf_deps.gni
[Subpackages]: /concepts/components/v2/subpackaging.md
