Vulkan Development
==================

## Runtime dependencies

The magma driver and libraries should already be built into a complete Fuchsia
image. The correct driver will be built and loaded based on the
[board](/development/build/build_system/boards_and_products.md) that is selected
when building.

### Add Vulkan driver support

Include the following to enable access to the Vulkan driver:

- {CML}

  For components declared using a [.cml][cml] manifest:

  ```json5
  {
    include: [
      "vulkan/client.shard.cml"
    ],
    ...
  }
  ```

- {CMX}

  For components declared using a [.cmx][cmx] manifest:

  ```json
  {
    "include": [
        "//src/lib/vulkan/application.shard.cmx"
    ],
    ...
  }
  ```

  A [test component](/concepts/testing/v1_test_component.md) should instead
  have these lines in its `.cmx` file:

  ```json
  {
    "include": [
        "//src/lib/vulkan/test-application.shard.cmx"
    ],
    ...
  }
  ```

### Out of tree runtime dependencies

For components built outside the Fuchsia tree or otherwise can't include the
above shards, include the following:

- {CML}

  For components declared using a [.cml][cml] manifest:

  ```json5
  {
    include: [
      "vulkan/client.shard.cml"
    ],
    ...
  }
  ```

- {CMX}

  For components declared using a [.cmx][cmx] manifest:

  ```json
  {
    "sandbox": {
        "features": [
          "vulkan"
        ],
        "services": [
          "fuchsia.sysmem.Allocator",
          "fuchsia.vulkan.loader.Loader"
        ]
    },
    ...
  }
  ```

  The `fuchsia.tracing.provider.Registry` service may optionally be included to
  allow the client driver to report [trace events](/concepts/kernel/tracing-system.md).
  `fuchsia.logger.LogSink` is also
  recommended to allow logs from the client driver to appear in the [system
  log](/development/diagnostics/logs/viewing.md).

  A [test component](/concepts/testing/v1_test_component.md) must also have
  these lines in its `.cmx` file:

  ```json
  {
    "facets": {
        "fuchsia.test": {
          "system-services": [
              "fuchsia.sysmem.Allocator",
              "fuchsia.vulkan.loader.Loader"
          ]
        }
      },
      ...
  }
  ```

### Required capabilities

A component that uses Vulkan must have these FIDL services routed to it:

* `fuchsia.sysmem.Allocator`
* `fuchsia.vulkan.loader.Loader`
* `fuchsia.tracing.provider.Registry`
* `fuchsia.logger.LogSink`

Test components can receive these capabilities by being placed into a
[non-hermetic realm](/development/testing/components/test_component.md#legacy_non-hermetic_tests):

* For `vulkan-test` include the `//src/lib/vulkan/vulkan-test.shard.cml` shard
* For `system-test` include the `//src/sys/test_manager/system-test.shard.cml` shard

## Buildtime dependencies

In order for your project to access the Vulkan headers, and to link against the Vulkan loader libvulkan.so, add the following GN dependency:

`//src/lib/vulkan`

## Rendering onscreen

There are two options for displaying your rendered output:

1. The system compositor

   See Scenic documentation for details.

2. Directly to the display

   This method is not compatible with a system that has a system compositor.

You can use a custom version of the [WSI swapchain](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#_wsi_swapchain){: .external}.

For details on the Fuchsia customization, refer to the
[vkcube](/src/graphics/examples/vkcube) example.  Note that swapchain support for option 1 has been removed.

## Reporting issues

Keep an eye on the system log for various types of graphics driver specific issues, and file tickets on the Magma project.
The driver should kill the connection corresponding to the context that was executing when these issues occurred; but otherwise should handle this failure gracefully.

If nothing works afterward, please file that as an issue as well.

### Gpu fault

Looks something like the following. This can happen due to user error or driver bug. Please make sure your app has no Vulkan validation layer issues.

If you believe your app is innocent, please file a Magma ticket and include at least this portion of the log, plus ideally a recipe to repro:

```
> [WARNING] GPU fault detected
> ---- device dump begin ----
> RELEASE build
> Device id: 0x1916
> RENDER_COMMAND_STREAMER
> sequence_number 0x1003
> active head pointer: 0x1f328
> ENGINE FAULT DETECTED
> engine 0x0 src 0x3 type 0x0 gpu_address 0x1000000
> mapping cache footprint 11.9 MB cap 190.0 MB
> ---- device dump end ----
> [WARNING] resetting render engine
```

### Gpu hang

If a command buffer fails to complete within a certain amount of time, the gpu driver should detect the condition and treat it as if a fault occurred.

Again, may be an application error or driver bug. If you believe your app is innocent, please file a Magma ticket and include at least this portion of the log, plus ideally a recipe to repro:

```
> [WARNING] Suspected GPU hang: last submitted sequence number 0x1007 master_interrupt_control 0x80000000
> ---- device dump begin ----
> DEBUG build
> Device id: 0x1916
> RENDER_COMMAND_STREAMER
> sequence_number 0x1006
> active head pointer: 0x20
> No engine faults detected.
> mapping cache footprint 0.0 MB cap 0.0 MB
> ---- device dump end ----
> [WARNING] resetting render engine
```

[cml]: /concepts/components/v2/component_manifests.md
[cmx]: /concepts/components/v1/component_manifests.md
