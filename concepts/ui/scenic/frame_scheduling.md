# Frame scheduling

Frame scheduling is how Scenic makes decisions around when to apply client
updates and draw frames. Scenic's frame scheduler is shared across Gfx and
Flatland. This section contains high level concepts related to how Scenic makes
scheduling decisions, and how clients can respond.

### Presentation requests

A "presentation request" is equal to one client call to Present(). It is a
request to apply all updates to the scene made since the last call to Present()
and make them visible to the display (i.e. "present" them).

## The frame scheduling queue {#frame-scheduling-queue}

The frame scheduling queue is how Scenic internally tracks presentation
requests. The frame scheduling queue is a set of queues, one for each individual
presentation client. Scenic pulls presentation requests from all individual
queues and combines them as they reach their requested presentation times.

Every time a client calls Present() Scenic places the presentation request into
the frame scheduling queue when all associated acquire fences have been
signaled.

## The frame scheduling process {#frame-scheduling-process}

When the queue is non-empty Scenic looks at the first item on each client's
presentation queue and selects the earliest requested presentation time. It uses
that time along with vsync timing information from the display to determine when
to wake up. Scenic tries to wake in time to to produce the next frame at a vsync
as close as possible to, but no earlier than, the requested presentation time.
Scenic then goes to sleep waiting for the calculated wake-up time.

When Scenic wakes up it collects the next request for each client from the frame
scheduling queue, where the requested presentation time meets the calculated
presentation time. Scenic then applies the updates associated with each
individual request.

This wake-up time is sometimes referred to as the "latch point". After the latch
point is reached, the presentation requests for the current frame are "latched",
and any requests arriving after the latch point will be deferred to a later
frame. Both predicted latch points before presentation and actual latch points
after presentation are communicated to clients through the Present() API to
enable client-side low-latency frame scheduling.

Scenic may reset the currently targeted latch point for an earlier one if a
presentation request with an earlier requested presentation time should arrive
before the currently targeted latch point is reached.

After applying all relevant updates Scenic renders the next frame and then waits
for the display to return the vsync signal, at which point it wakes up and
signals all clients whose requests were handled that the frame has been
presented.

If there are remaining requests in the presentation queue, Scenic finds the next
request on the queue, calculates the a new wake-up time and waits, continuing
the cycle.

## Squashing {#squashing}

An "unsquashable" presentation request from a client is guaranteed to be shown
for at least one vsync-interval.

Squashing is the process by how Scenic may combine multiple subsequent
presentation requests into a single frame. It has the effect of reducing latency
in cases where frames are either delayed on Scenic's side or are produced too
fast on the client side. The consequence of squashing is that a squashed
presentation request does not appear on screen for even a single frame.

A presentation request is by default "squashable", and may be marked by the
client as "unsquashable" in the Present() call.

If a presentation request is marked as "squashable" then Scenic will, when
applying the update, look at the next request in the queue for that client to
see if its requested presentation time also meets the calculated presentation
time for this latch point. If it does Scenic may apply the updates for that
request as well, "squashing" the updates into a single frame.

The "squashable" property is similar to Vulkan's
[VK_PRESENT_MODE_FIFO_KHR and VK_PRESENT_MODE_MAILBOX_KHR][1] swapchain
presentation modes, except it's applied on a per-frame basis rather than at
setup time.

[1]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html
