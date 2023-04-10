<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0174" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC describes how graphical scaling operations are handled in
[Flatland][flatland]. Scenic, as Fuchsia's system compositor, can handle
upscaling or downscaling a Flatland instance's output through Flatland's
SetScale() method. Floating-point scale factors may be used, with a guarantee
that the resulting image avoids sub-pixel rendering. A Flatland instance only
knows about the device pixel ratio value. In addition to the design of scale,
this RFC defines some common terminology for referring to the pixel spaces.

## Motivation

Graphical scale operation is commonly needed for many UI features which
Flatland, as a complete system compositor, should offer. Three of them are
described in detail below.

### Upsample

Upsampling is increasing the rendered size of a Flatland instance or an Image.
As an example, consider magnification. Magnification can be done by asking
Scenic to upsample a Flatland client application's output.

### Downsample

Downsampling is decreasing the rendered size of a Flatland instance or an
Image. As an example, consider
[GNOME's Activities Overview](https://help.gnome.org/misc/release-notes/3.6/users-activities-overview.html.en){: .external}.
A graphical system shell may want to make a Flatland client application continue
rendering at the same resolution, but shrink their content output to a smaller
rectangle by asking Scenic.

### Device Pixel Ratio

Device pixel ratio is defined so that the display manufacturers can upscale the
resolutions of their devices while still being able to display content,
designed for lower resolutions, at the same size on the new screens. As an
example, 4K and 1080p screen versions of a laptop would look identical from a
distance, all buttons and layout being the same size physically. Only when you
get closer you would be able to tell that 4K is sharper, because it packs 4
times the physical pixels in the same physical space as 1080p.

## Stakeholders

_Facilitator:_ neelsa@google.com

_Reviewers:_

* Input: neelsa@google.com, quiche@google.com
* Accessibility: lucasradaelli@google.com
* Scenic: jaeheon@google.com, dworsham@google.com, jjosh@google.com

_Socialization:_

A detailed version of this design was reviewed internally with Scenic, Input
and Accessibility teams. Alternative solutions were also discussed.

## Glossary

* [Flatland][flatland]
  * Scenic's new 2D composition API.
* [Gfx][gfx]
  * Scenic's old 3D composition API that is deprecated.
* [View][view]
  * A visual region of graphical content.
  * It has a coordinate system, a bounding box, and a defined spatial
    relationship to its ancestors, via the View Tree.
* HiDPI
  * High dots per inch.
  * The industry term used for displays packing more pixels in the same
    physical area. For example, screens with 240 dpi or higher are considered
    HiDPI.
* PP
  * Physical pixels.
  * Display's physical pixel count. i.e. if we have 4K and FHD screen
    variants of the same laptop, 4K has 3840x2160 physical pixels and FHD has
    1920x1080 physical pixels, so icons will appear the same size on both
    laptop variants, just sharper on the HiDPI display
* DIP
  * Device independent pixels a.k.a. Logical display pixels a.k.a. density
    independent pixels.
  * Display's logical pixel count. i.e. if we have 4K and FHD screen variants
    of the same laptop, both 4K and FHD most likely have 1920x1080 device
    independent pixels.
* DPR
  * Device pixel ratio.
  * The ratio between the display's physical pixels and the display's device
    independent pixels.
* LP
  * Logical View pixels a.k.a. View's device independent pixels
  * View's logical pixel count. Affects the content layout.
* AP
  * Allocation pixels.
  * View's draw buffer allocation pixel count.
* PSS
  * Parent-set scales a.k.a. Accumulated scales.
  * All parent SetScale() values multiplied. Ratio between a View's DIPs to the
    display's DIPs.

## Design

To accommodate all the use cases described in the Motivation section, we define
a set of relationships that establishes what values may be set by a parent view
on their child (parent-defined), what values can be set only by Flatland
(system-defined), and what values may be observed by a child (child-observed).

__Device pixel ratio__ is system-defined uniformly for all views on a single
display. DPR is child-observed.

A child view's size is measured in __LPs__. This size is parent-defined and
child-observed. A view has _optimal resolution_ if the size and alignment of
the view's LPs precisely matches the display's __DIPs__ for the region used to
render the view. A view should act as if it has optimal resolution, where
LP=DIP in size and alignment. However, the optimal resolution is *not*
child-observed, and can be intentionally broken by a parent view (using Scale,
see below) to allow features such as Magnification or GNOME's Activities
Overview.

Scale is introduced as a floating-point value. It is parent-defined but not
child-observed! Parent-set scales for a child view is the multiplication of all
scales introduced by all ancestor views of that child. __PSS__ describes the
ratio from a child view's LPs to the display's DIPs. By hiding the PSS,
Flatland retains control over how many __physical pixels__ a child view
occupies on the display, without forcing the child to re-allocate or re-present
when upsampling or downsampling.

We make a further distinction with allocation pixels, which is the size of the
buffer used to present content to display. This is a client implementation
detail, but to remove confusion, we describe its relationship to other pixel
spaces:

* __HiDPI__-aware clients want to look sharp and can control the size of their
  content. Their __Allocation pixels__ is Logical View pixels multiplied with
  DPR.
* DPR-oblivious clients want to present content in just one size. They ignore
  DPR, and their Allocation pixels are equal to their Logical View pixels.
* Custom-allocation clients have content that enforces their Allocation pixels,
  i.e. video player or WebCanvas. These clients will define their own
  relationship between Allocation pixels and Logical View pixels.

Flatland Mappings:

* PP<sub>Display</sub> = DIP<sub>Display</sub>* DPR
* PP<sub>View</sub> = LP<sub>View</sub> * PSS * DPR
* AP<sub>View</sub> to LP<sub>View</sub> is actually the client's own business.
  * AP<sub>View</sub> = LP<sub>View</sub> * DPR is recommended for HiDPI-aware
    clients.

The pixel space relationships described above are illustrated in Figure 1.

**Figure 1 - Flatland** ![This figure presents the pixel space relationships.](
    resources/0174_scale_in_flatland/scale_diagram.svg "Figure 1")

The proposed changes are represented by the following FIDL snippet:

```
type LayoutInfo = table {
    /// The layout size of a View in logical pixels, defined by the parent's call to
    /// [`SetViewportProperties`]. Clients should re-layout their content when this value changes.
    1: logical_size fuchsia.math.SizeU;

    /// The ratio from physical display pixels to the display's device independent pixels.
    /// Clients should not necessarily re-layout their content when this value changes. Clients may
    /// accommodate by reallocating their Image buffers that are adjusted by [`device_pixel_ratio`].
    /// HiDPI-aware clients that want to avoid sampling artifacts should render onto a buffer with
    /// at least the size round([`logical_size`] * [`device_pixel_ratio`]).
    /// Note that rounding is not C-style float-to-int truncation. The floating-point product should
    /// be converted to the nearest integer.
    2: device_pixel_ratio fuchsia.math.VecF;
};

protocol Flatland {

/// Sets the scale on a Transform. The order of geometric attribute application is addressed
/// in the documentation for SetTranslation().
/// Note that if there is a Viewport set on this Transform, the child Flatland will not be notified
/// about the changes made here. This method should only be used if the intention is to upsample or
/// downsample the Viewport's output. Use [`SetViewportProperties`] if the intention is to resize or
/// relayout the Viewport.
SetScale(struct {
        transform_id TransformId;
        scale fuchsia.math.VecF;
    });
}
```

DPR is applied as a scale to each Image before Scenic renders or sends them to
the display. If a client wants to support HiDPI, they are expected to layout
their content using the given LP, but allocate larger buffers by using the
reported DPR to reverse the effects of Scenic's scaling.

PSS is not communicated the the children. That means upscaling through Flatland
may cause blurry artifacts. However, this is preferred to forcing the client to
re-allocate and re-present, for example because buffer size would grow by the
scale factor and risk OOMs.

We may end up with non-integer pixels with float scales. If we have a
consistent way of attaching to a nearby integer pixel value(round), we
shouldn't end up with artifacts.

Flatland clients may want to go from logical pixels to physical pixels and that
requires knowing PSS in addition to DPR. However, this is only necessary for
input, so this design pushes this conversion information to
[`fuchsia.ui.pointer`](/sdk/fidl/fuchsia.ui.pointer) API.

The clients may respond to DPR changes at their own pace using this API. They
will all be signaled about these changes at the same time, so we don't have a
cascading delay jank pattern.

The client should always expect to work with a single DPR value. When we
support multiple displays in the future, we can report a single DPR value
coming multiple displays, or the DPR value of the display they are on.

### Contrast with Legacy

Note that in Gfx(Legacy 3D API), DPR and PSS was multiplied into a single value
and reported to the child. Hence, scale manipulation caused allocation
side-effects in client code, causing OOMs and other unintended side effects,
including large-scale architectural workarounds to achieve effects like
Magnification and confusion around DPR. This design for Flatland diverges by
suggesting that the child only needs to know about DPR for graphics purposes.

Gfx Mappings:

* PP<sub>View</sub> = LP<sub>View</sub> * pixel_scale
  * pixel_scale = PSS * DPR
  * pixel_scale is the only metric Gfx returns.

## Implementation

This design involves changes in
[`fuchsia.ui.composition/Flatland`](/sdk/fidl/fuchsia.ui.composition/flatland.fidl)
API. The implementation will be done in three main steps:

* Remove the usage of `pixel_scale` field, which is being deprecated, in-tree
  and out-of-tree.
* Complete in-tree changes. Send DPR information based on what is reported from
  the display.
* Change in-tree and out-of-tree client code to make use of DPR information to
  adjust AP.

## Performance

This proposal reduces the memory usage of DPI-aware clients in comparison to
Gfx API. In Gfx, DPR and PSS was multiplied into a single value and reported to
the child. DPI-aware client would respond to the accumulation of all scales.
As an example, if they were scaled by 5 by their parent and DPR of 2 applied,
they would have received a `pixel_scale` of 10 and allocated a 10 times larger
buffer. This proposal distinguishes DPR and PSS, so the unnecesssary allocations
are gone.

This proposal reduces the expectations of re-allocate and re-layout from the
clients. We can implement smoother upsample and downsample operations, because
we no longer rely on the Flatland client to repond to scale changes.

## Security considerations

This proposal does not affect the security model of Flatland API. The API
guarantees around other graphical operations, such as clip limiting the input
receiving area of a View, still apply to the scaled content.

This proposal reduces the extent of information passed to a Flatland view. In
Gfx, DPR and PSS was multiplied into a single value and reported to the child.
This proposal only reports DPR information. A child Flatland instance does not
have an indication of how their parent Flatland instance decides to present
their content, scaled or not, and cannot react to scaling changes.

## Privacy considerations

This proposal suggests sending DPR information to Flatland clients. Device
Pixel Ratio is derived from the physical and technical properties of the
display unit. Although a very specific information, this property and ratio is
not unique and may be common across multiple hardware, so it isn't very useful
for fingerprinting. Furthermore, DPR is absolutely necessary for preparing
quality graphical output.

## Testing

Flatland API has been tested in a layered approach, which scale feature will
follow:

* Unit tests in Scenic codebase.
* UI Integration tests, in [/src/ui/tests](/src/ui/tests), that exercise the
  contracts around Flatland APIs.
* System tests that capture output pixels from devices with DPR values
  different than one.
* Runtimes such as Chromium and Flutter write integration tests to test against
  usage of SetScale() and different DPR values.

## Documentation

Some of the [Flatland][flatland] documentation will be updated following this
RFC to describe the HiDPI-aware client behavior and how scaling works.

## Drawbacks, alternatives, and unknowns

There isn't a significant cost for implementing this proposal. This proposal
suggests a DPR solution that will work no matter which display hardware is
used. It may also be extended for multi-display use cases.

There were a couple alternatives considered for this design.

* DPR was added as a configurable static value in some Chromium configurations.
  That is clearly not scalable across different applications.
* To avoid rounding issues, we considered only allowing integer scale factors.
  However, there are exceptions to this in some DPR configurations.
* We could fall into a slow sub-pixel rendering path when we have
  floating-point values. However, there isn't a compelling reason to do so, and
  it would hinder performance by not allowing performance passing buffers
  directly to the display.

## Prior art and references

* [RFC-0162:Flatland](/contribute/governance/rfcs/0162_flatland.md)
* [RFC-0147:View system](/contribute/governance/rfcs/0147_view_system.md)
* [RFC-0166:One UI stack](/contribute/governance/rfcs/0166_ui_stack.md)
* Flatland share some architectural similarities with the Wayland protocol,
which handles
[High density surfaces](https://wayland-book.com/surfaces-in-depth/hidpi.html){: .external}
in a similar way to the proposed design.

[flatland]: /sdk/fidl/fuchsia.ui.composition/flatland.fidl
[gfx]: /sdk/fidl/fuchsia.ui.scenic/session.fidl
[view]: /contribute/governance/rfcs/0147_view_system.md
