# Declaring components

<<../../../_common/components/_declaring_intro.md>>

<<../../../_common/components/_declaring_manifests.md>>

<<../../../_common/components/_declaring_shards.md>>

## Building components

The Fuchsia SDK system provides Bazel rules to build and package software
into Fuchsia components. The
[Fuchsia SDK environment](/docs/get-started/sdk/index.md#clone-the-sdk-samples-repository)
makes these rules available within a
[Bazel workspace](https://bazel.build/concepts/build-ref#workspace){:.external}
directory.

Within the Bazel workspace, you declare Fuchsia packages and components as
[Bazel targets](https://bazel.build/concepts/build-ref#targets){:.external} within
a [Bazel package](https://bazel.build/concepts/build-ref#packages){:.external},
described by a `BUILD.bazel` file.

Below is an example of a `BUILD.bazel` file for a simple C++ component:

```bazel
# Build rules provided by the Fuchsia SDK
load(
    "fuchsia_cc_binary",
    "fuchsia_component",
    "fuchsia_component_manifest",
    "fuchsia_package",
)

fuchsia_cc_binary(
    name = "hello_world",
    srcs = [
        "hello_world.cc",
    ],
)

fuchsia_component_manifest(
    name = "manifest",
    src = "meta/hello_world.cml",
)

fuchsia_component(
    name = "component",
    manifest = ":manifest",
    deps = [":hello_world"],
)

fuchsia_package(
    name = "pkg",
    package_name = "hello_world",
    visibility = ["//visibility:public"],
    deps = [
        ":component",
    ],
)
```

This file contains the following main elements:

* `fuchsia_cc_binary()`: Compiles the C++ source code into a binary, including
  any necessary library dependencies.
* `fuchsia_component_manifest()`: Compiles the component manifest source file
  (`.cml`) into a binary component declaration using `cmc`.
* `fuchsia_component()`: Collects the binary, component manifest, and additional
  resources together into a single target.
* `fuchsia_package()`: Unit of distribution for components. Allows one or more
  components to be hosted in a package repository and included in the target
  device's package sets. This target generates the package metadata and builds
  the Fuchsia Archive (`.far`) file.

## Exercise: Create a new component

In this exercise, you'll build and run a basic component that reads the program
arguments and echoes a greeting out the system log.

To begin, create a new project directory in your Bazel workspace for a new
component called `echo`:

```posix-terminal
mkdir -p fuchsia-codelab/echo
```

This component project should have the following directory structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/echo
                  |- BUILD.bazel
                  |- meta
                  |   |- echo.cml
                  |
                  |- echo_component.cc
                  |- echo_component.h
                  |- echo_unittest.cc
                  |- main.cc
```

* `BUILD.bazel`: Bazel build targets for the executable binaries, component, and
  package.
* `meta/echo.cml`: Manifest declaring the component's executable and
  required capabilities.
* `echo_component.cc`: Source code for the C++ component functionality.
* `echo_unittest.cc`: Source code for the C++ unit tests.
* `main.cc`: Source code for the C++ executable binary main entry point.

### Add program arguments

The component manifest file defines the attributes of the component's executable,
including program arguments, and the component's capabilities.
Add the following contents to `meta/echo.cml`:

`echo/meta/echo.cml`:

```json5
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/echo/meta/echo.cml" region_tag="manifest" adjust_indentation="auto" %}
```

### Log the arguments

Open the `main.cc` source file for the main executable and add the following
import statements:

`echo/main.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/echo/main.cc" region_tag="imports" adjust_indentation="auto" %}

#include "echo_component.h"
```

Add the following code for the the `main()` function:

`echo/main.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/echo/main.cc" region_tag="main" adjust_indentation="auto" %}
```

This code reads the program arguments and passes them to a function called
`greeting()` to generate a response for the syslog entry.

Add the following code to implement the `greeting()` function:

`echo/echo_component.h`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/echo/echo_component.h" region_tag="greeting" adjust_indentation="auto" %}
```

`echo/echo_component.cc`:

```cpp
#include "echo_component.h"

{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/echo/echo_component.cc" region_tag="greeting" adjust_indentation="auto" %}
```

This function creates a simple string from the list of provided arguments based
on the length of the list.

<aside class="key-point">
  <b>Logging and standard streams</b>
  <p>Fuchsia has two main logging buffers; the system log (<code>syslog</code>)
  and kernel's debug log (<code>klog</code>). By default, components do not have
  stream handles for stdout and stderr available to record log messages from your
  code. Instead, you must use one of Fuchsia's logging libraries or redirect these
  streams to a Fuchsia log buffer.</p>

  <p>For more details on logging from your code, see
  <a href="/docs/development/diagnostics/logs/recording.md">Recording Logs</a>.</p>
</aside>

### Add to the build configuration

Add the following to your `BUILD.bazel` file to include the new component in the
build configuration:

`BUILD.bazel`:

```bazel
load(
    "@rules_fuchsia//fuchsia:defs.bzl",
    "fuchsia_cc_binary",
    "fuchsia_component",
    "fuchsia_component_manifest",
    "fuchsia_package",
)

{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/echo/BUILD.bazel" region_tag="echo" adjust_indentation="auto" %}
```

Run `bazel build` and verify that the build completes successfully:

```posix-terminal
bazel build --config=fuchsia_x64 //fuchsia-codelab/echo:pkg \
    --publish_to=$HOME/.package_repos/sdk-samples
```

In the next section, you'll integrate this component into the build and test the
output in the system log.
