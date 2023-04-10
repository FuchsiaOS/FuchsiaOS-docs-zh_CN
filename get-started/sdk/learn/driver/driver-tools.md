# Interact with the driver

Software on Fuchsia interacts with driver components through their exposed entries
in [devfs][concepts-devfs]. Once a client connects to the driver's devfs entry,
it receives an instance of the FIDL service representing that driver.

In this section, you'll create a new `eductl` executable that discovers and
interacts with the capabilities exposed by the `qemu_edu` driver.

## Create a new tools component

Create a new project directory in your Bazel workspace for a new binary tool:

```posix-terminal
mkdir -p fuchsia-codelab/qemu_edu/tools
```

After you complete this section, the project should have the following directory
structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/qemu_edu/tools
                  |- BUILD.bazel
                  |- eductl.cc
```

Create the `qemu_edu/tools/BUILD.bazel` file and add the following statement to
include the necessary build rules from the Fuchsia SDK:

`qemu_edu/tools/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/BUILD.bazel" region_tag="imports" adjust_indentation="auto" %}
```

Create a new `qemu_edu/tools/eductl.cc` file with the following code to set up a
basic command line executable:

`qemu_edu/tools/eductl.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/eductl.cc" region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/eductl.cc" region_tag="cli_helpers" adjust_indentation="auto" %}

int main(int argc, char* argv[]) {
  const char* cmd = basename(argv[0]);

  // ...

  return usage(cmd);
}

```

This executable supports two subcommands to execute the liveness check and
factorial computation.

Add the following new rules to the bottom of the project's build configuration
to build this new tool into a Fuchsia package:

`qemu_edu/tools/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/BUILD.bazel" region_tag="binary" adjust_indentation="auto" exclude_regexp="\/\/src\/qemu_edu" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/BUILD.bazel" region_tag="component" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/BUILD.bazel" region_tag="package" adjust_indentation="auto" %}
```

## Implement the client tool

When clients open a connection to an entry in devfs, they receive an instance of
the FIDL protocol being served by the driver. Add the following code to `eductl`
to open a connection to the `edu` device using its devfs path:

`qemu_edu/tools/eductl.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/eductl.cc" region_tag="imports" adjust_indentation="auto" %}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/eductl.cc" region_tag="fidl_imports" adjust_indentation="auto" %}{{ '</strong>' }}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/eductl.cc" region_tag="device_path" adjust_indentation="auto" %}{{ '</strong>' }}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/eductl.cc" region_tag="device_client" adjust_indentation="auto" %}{{ '</strong>' }}

// ...
```

Add `liveness_check()` and `compute_factorial()` functions to call methods using
the `examples.qemuedu/Device` FIDL protocol returned from `OpenDevice()`.
Finally, update the tool's `main()` function to call the appropriate device
function based on the argument passed on the command line:

`qemu_edu/tools/eductl.cc`:

```cpp
// ...

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/eductl.cc" region_tag="liveness_check" adjust_indentation="auto" %}{{ '</strong>' }}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/eductl.cc" region_tag="compute_factorial" adjust_indentation="auto" %}{{ '</strong>' }}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/eductl.cc" region_tag="main" adjust_indentation="auto" highlight="4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20" %}
```

Update the tool's build configuration to depend on the FIDL bindings for the
`examples.qemuedu` library:

`qemu_edu/tools/BUILD.bazel`:

{% set build_bazel_snippet %}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tools/BUILD.bazel" region_tag="binary" adjust_indentation="auto" highlight="7" %}
{% endset %}

```bazel
{{ build_bazel_snippet|replace("//src/qemu_edu","//fuchsia-codelab/qemu_edu")|trim() }}
```

<<_common/_restart_femu.md>>

## Reload the driver

Use the `bazel run` command to build and execute the driver component target:

```posix-terminal
bazel run //fuchsia-codelab/qemu_edu/drivers:pkg.component
```

## Run the tool

Use the `bazel run` command to build and execute the tool, passing the arguments
`fact 12` to compute the factorial of 12:

```posix-terminal
bazel run //fuchsia-codelab/qemu_edu/tools:pkg.eductl_tool -- fact 12
```

The `bazel run` command performs the following steps:

1.  Build the executable and package.
1.  Publish the package to a local package repository.
1.  Register the package repository with the target device.
1.  Use `ffx driver run-tool` to run the binary inside the `driver_playground`
    component.

The command prints output similar to the following with the computation result
the factorial:

```none {:.devsite-disable-click-to-copy}
Factorial(12) = 479001600
```

Congratulations! You've successfully connected to your driver's exposed services
from a separate client.

<!-- Reference links -->

[concepts-devfs]: /concepts/drivers/driver_communication.md#service_discovery_using_devfs
