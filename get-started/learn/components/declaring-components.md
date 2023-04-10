# Declaring components

<<../../_common/components/_declaring_intro.md>>

<<../../_common/components/_declaring_manifests.md>>

<<../../_common/components/_declaring_shards.md>>

<aside class="key-point">
To review the merged CML output with all includes resolved, run the
<code>fx cmc include</code> command with your manifest. For more details, see
the <a href="/docs/reference/tools/sdk/cmc.md"> reference documentation</a>.
</aside>

## Building components

The Fuchsia build system provides templates as GN imports in
[`//build/components.gni`](/build/components.gni) to build and package software
into Fuchsia components. Below is an example of a `BUILD.gn` file for a simple
C++ component:

```gn
{% verbatim %}
import("//build/components.gni")

executable("bin") {
  sources = [ "main.cc" ]
}

resource("my_file") {
  sources = [ "my_file.txt" ]
  outputs = [ "data/{{source_file_part}}" ]
}

fuchsia_component("hello-world-component") {
  component_name = "hello-world"
  deps = [
    ":bin",
    ":my_file",
  ]
  manifest = "meta/hello-world.cml"
}

fuchsia_package("hello-world") {
  package-name = "hello-world"
  deps = [
    ":hello-world-component",
  ]
}
{% endverbatim %}
```

This file contains the following main elements:

* `executable()`: Compiles the source code into a binary. This target varies
  depending on the programming language. For example, an `executable` target
  can be used for C++, `rustc_binary` for Rust, `go_binary` for Golang.
* `resource()`: Optional named collection of data files to copy as resources
  into another GN target. These files are accessible to the binary inside the
  component's namespace.
* `fuchsia_component()`: Collects the binary, component manifest, and additional
  resources together into a single target. This target compiles the manifest
  source into a component declaration using `cmc`.
* `fuchsia_package()`: Unit of distribution for components. Allows one or more
  components to be hosted in a package repository and included in the target
  device's package sets. This target generates the package metadata and builds
  the Fuchsia Archive (`.far`) file.

Packages can contain multiple components, listed as `deps` in the
`fuchsia_package()` template. You can simplify the build file for packages
containing only one component using the `fuchsia_package_with_single_component()`
template.

The following simplified `BUILD.gn` example is equivalent to to the previous
example:

```gn
{% verbatim %}
import("//build/components.gni")

executable("bin") {
  sources = [ "main.cc" ]
}

resource("my_file") {
  sources = [ "my_file.txt" ]
  outputs = [ "data/{{source_file_part}}" ]
}

fuchsia_package_with_single_component("hello-world") {
  manifest = "meta/hello-world.cml"
  deps = [
    ":bin",
    ":my_file",
  ]
}
{% endverbatim %}
```

Note: For more details on the GN syntax of the component build rules, see the
[components build reference](/docs/development/components/build.md).

## Exercise: Create a new component

In this exercise, you'll build and run a basic component that reads the program
arguments and echoes a greeting out the system log.

To begin, Create a project scaffold for a new component called `echo-args` in
the `//vendor/fuchsia-codelab` directory:

```posix-terminal
mkdir -p vendor/fuchsia-codelab/echo-args
```

Create the following file and directory structure in the new project directory:

* {Rust}

  ```none {:.devsite-disable-click-to-copy}
  //vendor/fuchsia-codelab/echo-args
                          |- BUILD.gn
                          |- meta
                          |   |- echo.cml
                          |
                          |- src
                              |- main.rs
  ```

  * `BUILD.gn`: GN build targets for the executable binaries, component, and
    package.
  * `meta/echo.cml`: Manifest declaring the component's executable and
    required capabilities.
  * `src/main.rs`: Source code for the Rust executable binary and unit tests.

* {C++}

  ```none {:.devsite-disable-click-to-copy}
  //vendor/fuchsia-codelab/echo-args
                          |- BUILD.gn
                          |- meta
                          |   |- echo.cml
                          |
                          |- echo_component.cc
                          |- echo_component.h
                          |- main.cc
  ```

  * `BUILD.gn`: GN build targets for the executable binaries, component, and
    package.
  * `meta/echo.cml`: Manifest declaring the component's executable and
    required capabilities.
  * `echo_component.cc`: Source code for the C++ component functionality.
  * `main.cc`: Source code for the C++ executable binary main entry point.

### Add program arguments

The component manifest file defines the attributes of the component's executable,
including program arguments, and the component's capabilities.
Add the following contents to `meta/echo.cml`:

{% set cml_rust %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/rust/meta/echo.cml" region_tag="manifest" adjust_indentation="auto" %}
{% endset %}

{% set cml_cpp %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/cpp/meta/echo.cml" region_tag="manifest" adjust_indentation="auto" %}
{% endset %}

* {Rust}

  `echo-args/meta/echo.cml`:

  ```json5
  {{ cml_rust|replace("echo_example_rust","echo-args")|trim() }}
  ```

* {C++}

  `echo-args/meta/echo.cml`:

  ```json5
  {{ cml_cpp|replace("echo_example_cpp","echo-args")|trim() }}
  ```

### Log the arguments

Open the source file for the main executable and add the following import statements:

* {Rust}

  `echo-args/src/main.rs`:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/rust/src/main.rs" region_tag="imports" adjust_indentation="auto" %}
  ```

* {C++}

  `echo-args/main.cc`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/cpp/main.cc" region_tag="imports" adjust_indentation="auto" %}

  #include "vendor/fuchsia-codelab/echo-args/echo_component.h"
  ```

Add the following code to implement the `main()` function:

* {Rust}

  `echo-args/src/main.rs`:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/rust/src/main.rs" region_tag="main" adjust_indentation="auto" %}
  ```

  <aside class="key-point">
  The <code>fuchsia::main</code> attribute removes some common boilerplate
  for component execution in Rust, such as initializing logging or async execution
  behavior.
  </aside>

* {C++}

  `echo-args/main.cc`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/cpp/main.cc" region_tag="main" adjust_indentation="auto" %}
  ```

This code reads the program arguments and passes them to a function called
`greeting()` to generate a response for the syslog entry.

Add the following code to implement the `greeting()` function:

* {Rust}

  `echo-args/src/main.rs`:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/rust/src/main.rs" region_tag="greeting" adjust_indentation="auto" %}
  ```

* {C++}

  `echo-args/echo_component.h`:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/cpp/echo_component.h" region_tag="greeting" adjust_indentation="auto" %}
  ```

  `echo-args/echo_component.cc`:

  ```cpp
  #include "vendor/fuchsia-codelab/echo-args/echo_component.h"

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/cpp/echo_component.cc" region_tag="greeting" adjust_indentation="auto" %}
  ```

This function creates a simple string from the list of provided arguments based
on the length of the list.

<aside class="key-point">
  <b>Logging and standard streams</b>
  <p>Fuchsia has two main logging buffers; the system log (<code>syslog</code>)
  and the kernel's debug log (<code>klog</code>). By default, components only
  have stream handles for stdout and stderr available to record log messages
  from your code if they have access to the system log. Fuchsia also has logging
  libraries available for greater control over metadata and format of
  messages.</p>

  <p>For more details on logging from your code, see
  <a href="/docs/development/diagnostics/logs/recording.md">Recording Logs</a>.</p>
</aside>

### Add to the build configuration

Update the program's dependencies in your `BUILD.gn` file:

{% set gn_rust_binary %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/rust/BUILD.gn" region_tag="executable" adjust_indentation="auto" %}
{% endset %}

{% set gn_rust_component %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/rust/BUILD.gn" region_tag="component" adjust_indentation="auto" %}
{% endset %}

{% set gn_cpp_binary %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/cpp/BUILD.gn" region_tag="executable" adjust_indentation="auto" %}
{% endset %}

{% set gn_cpp_component %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/cpp/BUILD.gn" region_tag="component" adjust_indentation="auto" %}
{% endset %}

* {Rust}

  `echo-args/BUILD.gn`:

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/rust/BUILD.gn" region_tag="imports" adjust_indentation="auto" %}

  group("echo-args") {
    testonly = true
    deps = [
      ":package",
    ]
  }

  {{ gn_rust_binary|replace("echo_example_rust","echo-args")|trim() }}

  {{ gn_rust_component|replace("echo_rust","echo-args")|trim() }}

  fuchsia_package("package") {
    package_name = "echo-args"
    deps = [ ":component" ]
  }
  ```

* {C++}

  `echo-args/BUILD.gn`:

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/echo/cpp/BUILD.gn" region_tag="imports" adjust_indentation="auto" %}

  group("echo-args") {
    testonly = true
    deps = [
      ":package",
    ]
  }

  {{ gn_cpp_binary|replace("echo_example_cpp","echo-args")|trim() }}

  {{ gn_cpp_component|replace("echo_cpp","echo-args")|trim() }}

  fuchsia_package("package") {
    package_name = "echo-args"
    deps = [ ":component" ]
  }
  ```

Add your new component to the build configuration:

```posix-terminal
fx set workstation_eng.qemu-x64 --with //vendor/fuchsia-codelab/echo-args
```

Run `fx build` and verify that the build completes successfully:

```posix-terminal
fx build
```

In the next section, you'll integrate this component into the build and test the
output in the system log.
