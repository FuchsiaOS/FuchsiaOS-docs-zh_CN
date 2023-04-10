Vulkan Development
==================

## Runtime dependencies

The magma driver and libraries should already be built into a complete Fuchsia
image. The correct driver will be built and loaded based on the
[board](/docs/development/build/build_system/boards_and_products.md) that is selected
when building.

### Component manifest

Include the following to enable access to the Vulkan driver:

- {CML}

  For components declared using a [.cml][cml] manifest:

  ```json5
  {
    include: [
      "vulkan/client.shard.cml"
    ],
    use: [
        { protocol: "fuchsia.media.ProfileProvider" },
    ],
    ...
  }
  ```

The `fuchsia.media.ProfileProvider` capability optional, but recommended.

### Required capability routes

A component that uses Vulkan must have these FIDL services routed to it:

- `fuchsia.sysmem.Allocator`
- `fuchsia.vulkan.loader.Loader`
- `fuchsia.tracing.provider.Registry`
- `fuchsia.logger.LogSink`
- `fuchsia.media.ProfileProvider` - optional, but strongly recommended; this should only be left out
if there are security concerns about the use of deadline threads in the Vulkan ICD. If not
specified, the Vulkan ICD will use default thread priorities for internal threads, which may cause
suboptimal performance.  Not included in `vulkan/client.shard.cml`, so it must be `use`d manually.

Test components can receive these capabilities by being placed into a
[non-hermetic realm](/docs/development/testing/components/test_component.md#legacy_non-hermetic_tests):

- For `vulkan-test` include the `//src/lib/vulkan/vulkan-test.shard.cml` shard
- For `system-test` include the `//src/sys/test_manager/system-test.shard.cml` shard

Test components can use the [vulkan_envs][vulkan_envs]
[environment][environment] to ensure they're run on all buildbots with Vulkan
support.

## Buildtime dependencies
### In-tree builds

In-tree code should depend on `//src/lib/vulkan` to be able to include the vulkan headers and link against `libvulkan.so`.

Other useful targets:

- `//src/lib/vulkan:vulkan_validation_layers`: Needed to be able to enable the [Vulkan validation layers][validation-layers].

### SDK clients

Code using the [Bazel SDK][bazel-sdk] should depend on `@fuchsia_sdk//pkg/vulkan`.

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

[cml]: /docs/concepts/components/v2/component_manifests.md
[environment]: /docs/contribute/testing/environments.md
[vulkan_envs]: /src/lib/vulkan/vulkan.gni
[bazel-sdk]: /docs/get-started/sdk/index.md
[validation-layers]: https://github.com/KhronosGroup/Vulkan-ValidationLayers
