Vulkan Development
==================

## Runtime dependencies

The magma driver and libraries should already be built into a complete Fuchsia
image. The correct driver will be built and loaded based on the
[board](/docs/concepts/build_system/boards_and_products.md) that is selected
when building.

Include the following in your component manifest to enable access to the Vulkan driver:

```json
{
   "include": [
      "src/lib/vulkan/application.shard.cmx"
   ],
   ...
}
```

A [test component](/docs/concepts/testing/v1_test_component.md) should instead have
these lines in its .cmx:

```json
{
   "include": [
      "src/lib/vulkan/test-application.shard.cmx"
   ],
   ...
}
```

### Out of tree runtime dependencies
An application that is not in the Fuchsia tree or which otherwise can't
include the file above must include these features and services in its .cmx
file:

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
allow the client driver to report [trace events](/docs/concepts/tracing/README.md).
`fuchsia.logger.LogSink` is also
recommended to allow logs from the client driver to appear in the [system
log](/docs/development/diagnostics/logs/viewing.md).

A [test component](/docs/concepts/testing/v1_test_component.md) must also have
these lines in its .cmx:

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

For details on the magma customization, refer to the
[vkcube](/src/graphics/examples/vkcube) example.

## Interaction with the graphics console

The magma display driver supports toggling ownership between the main display owner, and the graphics console.

Currently, on system startup the gfxconsole owns the display.

When a Vulkan application starts, it will take over the display.

To toggle display ownership between the Vulkan app and the gfxconsole, press alt-esc.

## Reporting issues

Keep an eye on the system log for various types of graphics driver specific issues, and file tickets on the Magma project.
The driver should kill the connection corresponding to the context that was executing when these issues occurred; but otherwise should handle this failure gracefully.

If nothing works afterward, please file that as an issue as well.

### Gpu fault

Looks something like the following. This can happen due to user error or driver bug. Please make sure your app has no validation layer issues.

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

## Demo

The magma build includes a spinning cube demo 'vkcube', which you can copy over to your Fuchsia system and execute via `netruncmd`.
