# Codelab: QEMU edu driver

Drivers provide software interfaces for communicating with hardware (or virtual)
devices that are embedded in or connected to a system. In Fuchsia, drivers are
user-space [components][concepts-components] running within the system's
[component topology][concepts-component-topology], and the
[driver framework][concepts-driver-framework] builds on the concepts and tools
provided by the component framework. Drivers interact with each other and
non-driver components using [capabilities][concepts-capabilities] and connections
established over FIDL.

In this codelab, you'll build a Fuchsia driver component that targets a virtual
device built into [QEMU][qemu]{:.external} (which underpins the
[Fuchsia emulator][guide-fuchsia-emulator]) named `edu`, which is an educational
device for writing drivers that computes the factorial of a given integer.
You'll also explore how to interact with drivers using the tools provided with
the Fuchsia SDK.

Note: For more details on the driver framework, see the
[Driver Concepts][concepts-drivers].

## Prerequisites

*   [Fuchsia SDK fundamentals][guide-fundamentals]
*   [Get started with driver development][driver-get-started]

## What you'll learn

*   Discovering and binding to device nodes
*   Registering and loading a new device driver
*   Interacting with a running device driver

## What you'll need

*   An x64-based machine running Linux or macOS

    Note: While you can use an x64-based (Intel) macOS machine for this get-started
    flow, you might run into issues. To help us improve, please
    [file a bug][sdk-bug]{:.external} if you discover issues on macOS.

*   A configured [Fuchsia SDK environment][driver-get-started]
*   [Emulator product bundle][driver-product-bundle]
    for `workstation_eng.qemu-x64`

## Before you begin

As part of the prerequisites, you created a **Bazel workspace** in the
`fuchsia-drivers/` directory. This is the directory that contains a
`WORKSPACE.bazel` file and it represents the root of the workspace. Throughout
the codelab, this root directory is referred to using the `//` prefix. For
example, the path `//fuchsia-codelab` represents a directory named
`fuchsia-codelab` at the root of the Bazel workspace.

Note: For more details on the Bazel build system and its terminology, see
[Bazel core concepts][bazel-concepts]{:.external}.

The Bazel workspace is also pre-configured with development tools provided by
the Fuchsia SDK in the `tools/` directory. This codelab assumes you are using
the SDK tools from within your Bazel workspace, so consider updating your `PATH`
to include the SDK's `tools/` directory or create a temporary alias using the
following commands:

```posix-terminal
alias ffx=tools/ffx
```

```posix-terminal
alias bazel=tools/bazel
```

<!-- Reference links -->

[bazel-concepts]: https://bazel.build/concepts/build-ref
[concepts-capabilities]: /docs/concepts/components/v2/capabilities/README.md
[concepts-component-topology]: /docs/concepts/components/v2/topology.md
[concepts-components]: /docs/concepts/components/v2/introduction.md
[concepts-driver-framework]: /docs/concepts/drivers/driver_framework.md
[concepts-drivers]: /docs/concepts/drivers/README.md
[driver-get-started]: /docs/get-started/sdk/get-started-with-driver.md
[driver-product-bundle]: /docs/get-started/sdk/get-started-with-driver.md#start-the-emulator
[guide-fuchsia-emulator]: /docs/development/sdk/ffx/start-the-fuchsia-emulator.md
[guide-fundamentals]: /docs/get-started/sdk/learn/README.md
[qemu]: https://www.qemu.org/
[sdk-bug]: https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=Bazel
