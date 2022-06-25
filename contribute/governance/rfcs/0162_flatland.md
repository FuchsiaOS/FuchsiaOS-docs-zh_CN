<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0162" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Former API Design Document

This RFC was previously submitted as an API design document and converted to an
RFC afterwards when the API design doc template was deprecated.

## Summary

This document proposes a 2D API for Fuchsia graphics clients. Flatland offers a
display-controller-like functionality to clients where the resources are defined
in a 2D world.

## Goals and use cases

Scenic currently provides graphics clients with a 3D API under the
`fuchsia.ui.gfx` namespace (the "gfx api"). This 3D API provides clients with a
scene model similar to video game engines or other 3D graphics programs. Drawing
order is handled by Z depth and opacity is handled via alpha blending based on
depth. Unfortunately the gfx API is no longer suitable for the demands being
placed on Scenic, both from a product standpoint and a performance standpoint.
Features like group opacity are impossible, because how to handle shadows for a
group of translucent content is unclear.

From the product standpoint, our current customers are 2D
products. There is no concept of depth and draw order. They are simply in the
order that different batches of draw geometry are submitted in. Because there is
no depth, transparency effects are also dictated by draw order. The "2D" clients
(via Flutter, Chromium, and session framework) have to do extra work to resolve
the mismatch between Scenic's 3D scene representation and the 2D
representation experienced by the user.

From the performance standpoint, modern Video Display Controller (VDC) hardware
provides acceleration features such as multiple display planes and hardware
overlays that Scenic would like to leverage in the future to lower power
consumption and GPU usage. The hardware operates in a strictly 2D paradigm
however, and only understands rectangular layers that can be positioned in X/Y.
Scenic's current 3D API allows and encourages clients to submit content that
doesn't fit into this paradigm and frustrates optimization attempts.

By exposing a true 2D API we intend to make our 2D clients life better:

* 2D API matches the 2D client's expectations more closely.
* Less GPU usage by delegating work to the VDC where possible.
* A more lightweight Scenic optimized for 2D rectangular layers only.

## Design

Our suggestion is to write a 2D protocol from scratch that doesn't reuse parts
of [`fuchsia.ui.gfx`](https://fuchsia.dev/reference/fidl/fuchsia.ui.gfx).

The current state of the proposed 2D API is in the
`fuchsia.ui.scenic.internal.flatland` library being submitted as part of this
review. Please take a look at and drop comments.

Here are explanations for some high level decisions made around Flatland API:

* Flatland closely follows and provides functionality similar to the display
  controller API defined on
  [`fuchsia.hardware.display/Controller`](https://fuchsia.dev/reference/fidl/fuchsia.hardware.display#Controller).
  * The most performant scenario is when the Flatland implementation in Scenic
    passes its resources to the display without compositing. Therefore, there
    are common resources with the display controller API that are defined in the
    same manner. I.e., zx.handle:EVENT that are used as fences that signal when
    it is safe to access resources, are transported across these protocols and
    carry the same meaning and purpose.
* Flatland aims to offer deterministic CPU cost to the clients. There is one
  deadline-scheduled render thread. Each Flatland session runs on their own
  dispatchers, which are on their own threads in the current configuration.
  * There may be multiple Flatland sessions where each is a channel speaking to
    Flatland for the purpose of rendering to a rectangular layer on the display.
    These sessions may not affect each other's presentation or performance flow.
* Image allocations are enforced to be done through Scenic's Allocator protocol
  defined under
  [`fuchsia.ui.composition/Allocator`](https://fuchsia.dev/reference/fidl/fuchsia.ui.composition#Allocator).
  * All Image usages are zero-copy as the client and Scenic agrees on the
    allocation and formats before allocation happens using Sysmem.
  * Allocator allows Images to be used across multiple Flatland sessions.
* Flatland does not offer a command union pattern. Present() call is the marker
  for processing the enqueued commands, which are individual methods.
  * Flatland strictly enforces limits on how many times the clients may call
    Present(). This is communicated through the response OnPresentProcessed().
    This is used as a throttling mechanism.
  * Flatland may return an Error in OnPresentProcessed() callback to inform
    clients about an illegal operation. The Flatland channel is closed following
    this.
* Flatland expects clients to define and keep track of [unique resource
  identifiers](/docs/development/api/fidl.md#client-assigned_identifiers). Type
  safety is enforced by the structs defined around these identifiers, such as
  TransformId and ContentId.
* Flatland uses hanging-gets to notify clients about changes in the link
  structure or properties. These are emitted through a separate protocol.

## Unknowns

Flatland design might evolve in some areas to better fit the clients needs.

* The clients may expect the compositor to handle some 2D effects that are not
  handled by the display hardware, such as blur. These operations are more
  expensive. We are planning to make this distinction clear by decoupling these
  two categories of operations. The current status of Flatland only supports the
  operations that can be handled by the display.
* Present and feedback flow evolves around our clients’ feedback. There were
  three iterations on gfx API's presentation flow before defining the current
  Flatland flow. Therefore, PresentArgs is defined as a table to allow some
  flexibility for the future changes.
  * We focus on the low-latency client and the high-throughput client in our
    design and presentation feedback. OnPresentProcessed() notifies the client
    when it is a good time to start work for the next frame. OnFramePresented()
    informs the advanced client about the time when the presented frame made it
    to the screen.

## Usability

Please take a look at out in-tree tests on
[flatland_unittest](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/ui/scenic/lib/flatland/tests/flatland_unittest.cc)
to find an extensive set of examples of Flatland API usage.

Below is a figure that explains how a connected graph that links multiple
Flatland sessions would work. This is a more complex but common use case.

**Figure 1 - Flatland** ![This figure presents the linking between Flatland
APIs.](resources/0162_flatland/figure_1.svg "Figure 1")

## Testing

Flatland is currently defined as an internal API and testing is provided by
in-tree unit tests. Since there is no other way to run Flatland code other than
tests, we have been extensively covering every code with an automated test. We
are planning to maintain this test coverage and quality.

We are planning to eventually convert every graphics example to Flatland. We do
not have any clients that strictly need to live in a 3D world. We are currently
working on having a Flatland presenter to be the basis for the addition and
migration of in-tree integration tests
[fxbug.dev/76315](http://fxbug.dev/76315). This will prepare us towards the
future migration of some critical external clients, such as Flutter and
Chromium.

Flatland is designed to allow running business logic without dependency on
hardware capabilities, such as Vulkan and display. Vulkan can be swapped by the
null renderer implementation, which skips compositing. Display can be swapped
the existing fake display implementation.

Additional Flatland API integration tests will be provided under [Compatibility
Test Suite](/docs/contribute/governance/rfcs/0015_cts.md).

## Performance considerations

Flatland has been designed to allow many clients to operate at the same time
without affecting each other’s work.

* Each method is a one-way FIDL method representing a client's command to render
  to a rectangular portion of the display. Present() signals the end of the
  command sequence coming from the client and the start of Scenic's processing
  for the next display update.
* Most clients make at least one call every vsync when actively updating their
  content. That is ~16ms for 60 fps display scenario. In each vsync interval:
  * Clients may make N calls to modify scene graph followed by one Present()
    call.
  * Flatland emits OnPresentProcessed() to inform the client that the operation
    has been queued and the client should begin producing its next frame.
    Currently, our clients have no idea when it's the ideal time to begin work
    and rely on hardcoded offsets to avoid overlaps and OnPresentProcessed()
    fixes that. This feedback also gives hints about the ideal future Present()
    call to the client, as well as informing them about their Present()
    allowance.
  * The client may ignore the timing and hints given in OnPresentProcessed() and
    keep presenting according to their internal clock. This is still valid, but
    there is no guarantee on avoiding resource contention.
  * Flatland emits OnFramePresented() to inform the client that the content has
    actually been displayed on the screen. This feedback is necessary for
    synchronization, such as audio/video, for the advanced client.
* Flatland stops a malicious client from queuing too many Present() calls by
  explicitly defining `num_presents_returned`. The client may not call Present()
  more times than it is allowed by OnPresentProcessed(). Each client starts with
  one present allowance.
* Remember that Flatland runs each channel connection on its own dispatcher.
  These feedback mechanisms are asynchronous.

## Security considerations

Flatland users are isolated from each other. Each of them connect to Scenic
through their own channels. [Unique resource
identifiers](/docs/development/api/fidl.md#client-assigned_identifiers) are defined
only within the scope of their channel. They can only target the portion of the
screen that is defined by their parent Flatland session.

Each error case causes Flatland channel closure. In these error states, it is
not clear what should be drawn on the screen, so we don’t see any point to allow
the client to keep presenting.

## Privacy considerations

Flatland does not expose any device identifiers or privacy-sensitive
information. The client does not interact with the hardware directly.

Flatland allows the client to set an identifiable debug string through
SetDebugName(). This is used as a prefix when printing detailed system logs
about errors to help the client distinguish what is theirs. The client has full
control over what to set here, and if nothing is set, there is no prefix in
system logs.

## Drawbacks and alternatives

What we learned from the existing 3D api under `fuchsia.ui.gfx` is the basis for
the decision made for the 2D API of Flatland. We took into account all the user
feedback, bugs and lessons when making design decisions.

* We could opt for the evolution of 2D API under the existing 3D API. However,
  there are some fundamental differences that would make this unnecessarily
  complicated for the client as well as the implementation.
* Flatland could use a command union pattern like `fuchsia.ui.gfx` did.
  Currently, each Flatland command is mapped as a FIDL method. This decision was
  made because of the negatives observed with the existing command pattern in 3D
  API. We had to provide and maintain wrappers in different languages for the
  clients. However, there are some negatives about mapping each comment as a
  method. This design prevents batching until [support for multiple messages in
  single
  write](/docs/contribute/governance/rfcs/0010_channel_iovec.md#support_for_multiple_messages_in_single_write)
  becomes available. However, we don’t expect the clients to manipulate the
  scene graph often and don’t consider this costly.

## Future work

There are some areas that we plan to work on for improving Flatland API:

* Each Flatland instance will have a ViewRef associated with it, and methods to
  grab a ViewRef of yourself and your children. See
  [fxbug.dev/79630](http://fxbug.dev/79630).
* A "factory function" to bind input protocols to a specific Flatland instance,
  thus limiting their scope to the instance's view sub-tree. See
  [fxbug.dev/79661](http://fxbug.dev/79661).
* Size and metrics flow one way, from parent to child, but the parent (and the
  mediating server) doesn't know which frame those size and metrics take effect,
  except for the very first frame. This has knock-on effects: user can see
  imperfect frames, and other APIs are exposed to latency in client logic. See
  [fxbug.dev/76440](http://fxbug.dev/76440).
* Synchronizing Present() across multiple instances is considered but not yet
  solved. See [fxbug.dev/79673](http://fxbug.dev/79673).
* Flatland will live under fuchsia.ui.composition namespace along with its
  dependencies. fuchsia.scenic.allocation and fuchsia.scenic.scheduling are also
  moving under there. See [fxbug.dev/78648](http://fxbug.dev/78648).
