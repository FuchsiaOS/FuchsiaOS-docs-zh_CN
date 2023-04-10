# Expose the driver capabilities

Driver components offer the features and services they provide to other drivers
and non-driver components through [capabilities][concepts-capabilities].
This enables Fuchsia's component framework to route those capabilities to the
target component when necessary. Drivers can also export their capabilities to
[devfs][concepts-devfs] to enable other components to discover them as file
nodes mounted in the component's namespace.

In this section, you'll expose the `qemu_edu` driver's factorial capabilities
and consume those from a component running elsewhere in the system.

## Create a new FIDL library

Create a new project directory in your Bazel workspace for a new FIDL library:

```posix-terminal
mkdir -p fuchsia-codelab/qemu_edu/fidl
```

After you complete this section, the project should have the following directory
structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/qemu_edu/fidl
                  |- BUILD.bazel
                  |- qemu_edu.fidl
```

Create the `qemu_edu/fidl/BUILD.bazel` file and add the following statement to
include the necessary build rules from the Fuchsia SDK:

`qemu_edu/fidl/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/fidl/BUILD.bazel" region_tag="imports" adjust_indentation="auto" %}
```

## Define a driver service protocol

The driver exposes the capabilities of the `edu` device using a custom FIDL
protocol. Add a new `qemu_edu/qemu_edu.fidl` file to your project workspace with
the following contents:

`qemu_edu/fidl/qemu_edu.fidl`:

```fidl
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/fidl/qemu_edu.fidl" region_tag="example_snippet" adjust_indentation="auto" %}

```

This FIDL protocol provides two methods to interact with the factorial
computation and liveness check hardware registers on the `edu` device.

Add the following build rules to the bottom of the project's build configuration
to compile the FIDL library and generate C++ bindings:

*   `fuchsia_fidl_library()`: Declares the `examples.qemuedu` FIDL
    library and describes the FIDL source files it includes.
*   `fuchsia_fidl_llcpp_library()`: Describes the generated
    [LLCPP (Low-Level C++) bindings][fidl-cpp-bindings] for components to
    interact with this FIDL library.

`qemu_edu/fidl/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/fidl/BUILD.bazel" region_tag="fidl_library" adjust_indentation="auto" %}
```

## Implement the driver service protocol

With the FIDL protocol defined, you'll need to update the driver to implement
and serve this protocol to other components.

After you complete this section, the project should have the following directory
structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/qemu_edu/drivers
                  |- BUILD.bazel
                  |- meta
                  |   |- qemu_edu.cml
                  |- edu_device.cc
                  |- edu_device.h
{{ '<strong>' }}                  |- edu_server.cc {{ '</strong>' }}
{{ '<strong>' }}                  |- edu_server.h {{ '</strong>' }}
                  |- qemu_edu.bind
                  |- qemu_edu.cc
                  |- qemu_edu.h
```

Create the new `qemu_edu/drivers/edu_server.h` file in your project directory
with the following contents to include the FIDL bindings for the
`examples.qemuedu` library and create a new `QemuEduServer` class to
implement the server end of the FIDL protocol:

`qemu_edu/drivers/edu_server.h`:

```cpp
#ifndef FUCHSIA_CODELAB_QEMU_EDU_SERVER_H_
#define FUCHSIA_CODELAB_QEMU_EDU_SERVER_H_

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_server.h" region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_server.h" region_tag="namespace_start" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_server.h" region_tag="fidl_server" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_server.h" region_tag="namespace_end" adjust_indentation="auto" %}

#endif  // FUCHSIA_CODELAB_QEMU_EDU_SERVER_H_

```

Create the new `qemu_edu/drivers/edu_server.cc` file in your project directory
with the following contents to implement the `examples.qemuedu/Device`
protocol methods and map them to the device resource methods:

`qemu_edu/drivers/edu_server.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_server.cc" region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_server.cc" region_tag="namespace_start" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_server.cc" region_tag="compute_factorial" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_server.cc" region_tag="liveness_check" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/edu_server.cc" region_tag="namespace_end" adjust_indentation="auto" %}

```

Update the driver's component manifest to declare and expose the FIDL protocol
as a capability:

`qemu_edu/drivers/meta/qemu_edu.cml`:

```json5
{
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/meta/qemu_edu.cml" region_tag="driver" %}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/meta/qemu_edu.cml" region_tag="use_capabilities" exclude_regexp="protocol" %}
{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/meta/qemu_edu.cml" region_tag="expose_capabilities" %}{{ '</strong>' }}
}
```

Update the driver's build configuration to depend on the FIDL bindings for the
new `examples.qemuedu` library:

`qemu_edu/drivers/BUILD.bazel`:

{% set build_bazel_snippet %}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/BUILD.bazel" region_tag="binary" adjust_indentation="auto" exclude_regexp="fuchsia\.device\.fs" highlight="13" %}
{% endset %}

```bazel
{{ build_bazel_snippet|replace("//src/qemu_edu","//fuchsia-codelab/qemu_edu")|trim() }}
```

## Export and serve the protocol

The `qemu_edu` driver makes the `examples.qemuedu/Device` protocol
discoverable to other components using devfs. To discover which driver services
are available in the system, a non-driver component would look up the device
filesystem (usually mounted to `/dev` in a component’s namespace) and scan for
the directories and files under this filesystem.

Driver manager can alias entries in devfs to a specific **device class** entry
(for example, `/dev/class/input`) when a matching **protocol ID** to a known
device class is provided. If a non-driver component does not know the exact path
of the driver service in devfs, but rather a specific type.
For this exercise, the `edu` device does not conform to a known class so you'll
configure this entry as an **unclassified device**.

Update the driver component's manifest to request the `fuchsia.devics.fs.Exporter`
capability:

`qemu_edu/drivers/meta/qemu_edu.cml`:

```json5
{
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/meta/qemu_edu.cml" region_tag="driver" %}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/meta/qemu_edu.cml" region_tag="use_capabilities" highlight="2" %}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/meta/qemu_edu.cml" region_tag="expose_capabilities" %}
}
```

Update the driver's `Start()` method to begin serving the `examples.qemuedu/Device` protocol
to a new devfs entry that matches the device node's topological path:

`qemu_edu/drivers/qemu_edu.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="compat_imports" adjust_indentation="auto" %}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="fidl_imports" adjust_indentation="auto" %}{{ '</strong>' }}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="start_method_start" adjust_indentation="auto" %}
  // ...

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="device_registers" %}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="serve_outgoing" %}{{ '</strong>' }}

{{ '<strong>' }}{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="devfs_export" %}{{ '</strong>' }}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="start_method_end" adjust_indentation="auto" %}
```

The `qemu_edu` driver's capabilities are now discoverable by other components.

## Rebuild the driver

Use the `bazel build` command to verify that the driver builds successfully with
your code changes:

```posix-terminal
bazel build //fuchsia-codelab/qemu_edu/drivers:pkg
```

Congratulations! You've successfully exposed FIDL services from a Fuchsia driver.

<!-- Reference links -->

[concepts-capabilities]: /concepts/components/v2/capabilities/README.md
[concepts-devfs]: /concepts/drivers/driver_communication.md#service_discovery_using_devfs
[fidl-cpp-bindings]: /development/languages/fidl/guides/c-family-comparison.md
