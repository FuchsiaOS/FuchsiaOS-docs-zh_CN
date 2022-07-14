> **_ATTENTION:_** This document describes the behavior of Scenic's GFX API which is currently being replaced by the [Flatland API](/concepts/ui/scenic/flatland/index.md). Workstation currently uses Flatland only, and Smart Display will be converted to use Flatland as well. If working with Flatland, please refer to the [Flatland documentation](/concepts/ui/index.md).

# Life of a Pixel

A client requests a set of commands to be Presented as part of a future Scenic frame. A single Scenic frame
can have multiple client "Presents", where each Present represents a Session's update to the global scene graph. This
doc describes the architecture internal to Scenic for how a request becomes pixels.

The diagram below shows the steps a client Present follows when it is requested. Everything between the Scenic FIDL Boundary and the Vulkan driver is currently single-threaded and executes sequentially.

1. Client `Enqueue()`s a set of commands to change the contents of its part of the scene, and calls `Present2()` to commit them.
2. The `Present2()` request enters `scenic_impl::Session`,. `scenic_impl::Session` waits for any acquire fences to signal, as well as any previous `Present2()` calls whose fences haven't been reached yet. `scenic_impl::Session` then schedules an update for the targeted `presentation_time` with the `FrameScheduler`.
3. The `FrameScheduler` starts sleeps until there's just enough time to prepare a frame in time for the targeted presentation time. At that point the `FrameScheduler` wakes up and calls `SessionUpdater::UpdateSessions()` on all `SessionUpdaters`.
4. For each client Session, `GfxSystem` calls `ApplyScheduledUpdates()`, which applies the commands to the scene graph that were enqueued in step 1.
  Note: `GfxSystem` is a `SessionUpdater`.
5. Commands from a Session are applied to the global scene graph. The scene graph is in an inconsistent state ("dirty") at this time, and should not be read by other systems (i.e. input) until after the scene graph has been post-processed.
6. When all `SessionUpdaters` have successfully updated, the `FrameScheduler` is notified that the scene graph is dirty, and triggers a `RenderFrame()` call on the `FrameRenderer`.
7. To draw a frame, `gfx::Engine`'s renderer traverses the scene graph and creates `Escher::objects` for each element in the scene. The renderer then passes these objects to `Escher`, and calls `DrawFrame()`.
  Note: `gfx::Engine` is a `FrameRenderer`.
8. `Escher` interprets the scene graph objects as `vk::commands`, and sends these commands to the GPU.
9. The GPU processes the commands and sends the results to the display driver.
10. The display driver pushes the pixels to the screen.

![Image of the classes and calls a client Present request goes through to become a pixel on screen. This is a visual representation of the enumerated list above.](/development/graphics/scenic/meta/life_of_pixel.svg)
