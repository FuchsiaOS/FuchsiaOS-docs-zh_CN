# Flatland timeline

In a typical frame, the order of events when using the Flatland API goes like
this[^1]:

1.  The client calls `Flatland::Present()`.
2.  Scenic waits until `PresentArgs::requested_presentation_time` has been
    reached and `PresentArgs::server_wait_fences` have been signaled.
    *   `requested_presentation_time` is the earliest possible time the present
        may appear on the display.
    *   `server_wait_fences` is a set of zircon events ("fences") which must all
        be signaled before the content submitted with the `Present()` call may
        appear on the display.
3.  Scenic composites the content from all clients this frame and sends it off
    to the display.
4.  Scenic sends `Flatland::->OnNextFrameBegin()` to the client.
5.  The client starts creating their next frame.
6.  Once the image has actually been displayed Scenic sends
    `Flatland::->OnFramePresented()` to the client.
7.  Repeat from 1.

This document explains what each of these signals mean.

[^1]: The exact order of these events is not guaranteed. For example
    `->OnNextFrameBegin()` may happen after `->OnFramePresented()`, or the
    client may call `Present()` again before receiving `->OnFramePresented()`.
    For most clients these edge cases should not be a concern. They should
    just use the signal from `->OnNextFrameBegin()` to draw their next frame
    and use `->OnNextFramePresented()` to collect data, in which case there
    should be no conflicts.

## Present() {#present}

As clients make Flatland API calls, those operations are not actually applied
right away. Instead these updates are bundled and committed atomically when
`Present()` is called. The set of updates between two calls to `Present()` is
referred to in this document as a `present`.

When calling `Present()` the client passes a table of arguments, `PresentArgs`,
which contain arguments for how the `Present()` call should be handed. The most
important arguments for deciding when the present appears on the display are the
`requested_presentation_time` and `server_wait_fences`.

For more information on how `Present()` calls are handled, see
[frame scheduling](/docs/concepts/ui/scenic/frame_scheduling.md).

## ->OnNextFrameBegin() {#on-next-frame-begin}

`->OnNextFrameBegin()` is sent as a hint to the client for when they should
begin creating their next frame. It does not explicitly say anything about the
state of Scenic, except that Scenic thinks resource contention should be
relatively low. `OnNextFrameBegin()` is only sent after the client have made
previous `Present()` calls, and only when the client has present credits
remaining.

## ->OnFramePresented() {#on-frame-presented}

`->OnFramePresented()` is sent when one or more presents have actually appeared
on the display (i.e. after Scenic receives the VSync signal). It provides two
main pieces of information:

*   Timing feedback to the client so they can evaluate how their frame
    scheduling strategy is working (latency, dropped frames, etc).
*   It is signal for *when the effects of a present has updated the state of the
    [ViewTree](/docs/concepts/ui/scenic/views.md#view-tree))*. After
    `->OnFramePresented()` Scenic guarantees that any subsequent calls to APIs
    that interact with the `ViewTree` (such as those in
    `Flatland::ViewBoundProtocols`) will act as if the previous present has been
    applied (though more presents may have been applied afterwards).
