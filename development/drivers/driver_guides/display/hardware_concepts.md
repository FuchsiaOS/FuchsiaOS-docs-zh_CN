# What does a display controller do?

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

Display controllers (DCs) are responsible for compositing one or more source
images into a display output signal. Some DCs are capable of producing multiple
output signals in parallel, e.g. to drive multiple monitors. This document
briefly explains common hardware components to help you understand the often
brief and uncommented datasheets available for driver development.

The subset of hardware used to composite one output image is often called a
[pipe](#pipe). A pipe is made up of input [planes](#plane), color correction,
and blending/compositing stages.

Each plane has a source image with its own dimensions, position, [pixel
format](#pixel-format), [color space](#color-space), [gamma
correction](#gamma-correction) and blending behavior. The pipe must be
programmed to correctly decode, transform, color correct, and blend these planes
into a final display image.

[Encoders](#encoder) consume final display images and combine the raw pixel data
with [timing generators](#display-timing) for encoding into the right electrical
signal for a [connector port](#port) (DisplayPort, HDMI, DVI, etc.)

## Plane types {#plane}

Display
: A plane that is "screen-sized" and is blended normally.

Overlay
: An overlay is a plane that uses chroma-keyed transparency. A typical use case
  for this plane type is to render media controls on top of video playback --
  software punches a hole in the (largely) static content, allowing a hardware
  accelerated video decoder to dump contents into a second plane which appears
  beneath the overlay.

Sprite
: Nowadays, many driver developers use the term "sprite" to refer to any plane
  that is not intended as the main content. Historically, sprites were used to
  compactly encode repeated uses of a static image. This type was very popular
  in the very limited memory environments of 80s personal computers and 80s-90s
  video game consoles.

Cursor:
: Cursor planes are typically smaller and offer limited color space options.
  They are used to overlay cursors on static content. Most software manipulates
  this plane by adjusting its position and only very occasionally changing its
  source image.

## Color spaces {#color-space}

Color spaces consist of a color model, e.g. RGB or CMYK, and a mapping function
to convert values in that color space to a reference space such as CIELAB or
CIEXYZ.

The color spaces used in digital image processing and display can be roughly
divided into two types: physical/linear and
perceptual/[gamma-encoded](#gamma-correction). Colors in a linear space can be
combined and transformed in a physically accurate manner, e.g. doubling the
values means "twice as much light". Perceptual color spaces are meant to mimic
non-linear human visual perception, so doubling the values would instead mean
"twice as bright".

## Pixel formats {#pixel-format}

Pixel formats and color spaces are often confused, and this leads to color
accuracy bugs. Take the RGB_888_24 pixel format for example: each pixel is
formatted as a triple of (R, G, B) bytes, but you do not know whether that
triple is a point in the sRGB, Adobe RGB, or linear RGB color space. If two
values from different color spaces are combined (e.g. adding them), the
resulting pixel value may be physically incorrect.

A DC must be programmed to convert each plane's image into a linear color space,
composite them all, and convert the resulting image into a color space that is
appropriate for the output device. The HDMI spec mandates that monitors support
at least sRGB.

## Gamma correction {#gamma-correction}

For a detailed explanation, see [this excellent blog post by John
Novak][novak-gamma]{:.external}.

Most still digital images are in the sRGB color space, which is also the most
widely supported format for computer monitors. However, in order to correctly
composite such images for display on a monitor, the controller must convert
every plane image into a linear color space ("degamma"), blend them, and then
convert back ("regamma") to sRGB and a pixel format supported by the monitor.

## EDID {#edid}

Extended Display Identification Data (EDID) is a VESA metadata format for
display devices to describe their capabilities to a video source. Many displays
expose their color format & output capabilities via [EDID][edid]{:.external}.
The [//src/graphics/display/lib/edid][edid-lib]{:.external} can be used to parse this
information and adjust gamma & color-correction programming for higher fidelity
color.

Not all fields of EDID will apply to all display devices, e.g. projectors have
no physical dimensions.

## Pipes {#pipe}

Pipes are a common abstraction in the theory of operation for display
controllers. Each pipe is dedicated to producing a single output image to be
[encoded](#encoder) for transmission on a [port](#port). The final output of a
pipe is pixel data in an appropriate format and gamma-encoded for the target
displays. This separation of responsibilities allows drivers to support display
"mirroring" by using the same final image as the input to multiple encoder/port
pairs.

## Ports {#port}

Ports are physical connectors that can be used to attach one or more displays.
Example port types are [DisplayPort][display-port]{:.external},
[HDMI][hdmi]{:.external}, [DVI][dvi]{:.external} and [VGA][vga]{:.external}.
Some port specs allow multiple displays to be connected through daisychaining,
e.g. DisplayPort's Multi-stream Transport (MST).

## Encoders {#encoder}

Encoders transform the output images of a [pipe](#pipe) into signals to be
transmitted by one or more [ports](#port). For example, [HDMI][hdmi]{:.external}
connectors use [TMDS][tmds]{:.external} which is an 8b/10b encoding.

## Display timing {#display-timing}

Display timing is a collection of intervals that together describe how to encode
a signal for a specific panel. While these intervals are _related_ to the
display mode (resolution, color depth, refresh rate), they also include portions
of the signal that do not communicate pixel contents, e.g. the [vertical
blanking interval][vblank-interval]{:.external} that has been retained since the
early days of broadcast television. [This article][display-timing]{:.external}
explains the details of timing.

[display-port]: https://en.wikipedia.org/wiki/DisplayPort#Connectors_and_pin_configuration
[display-timing]: https://en.wikipedia.org/wiki/Raster_scan#video_timing
[dvi]: https://en.wikipedia.org/wiki/Digital_Visual_Interface#Connector
[edid]: https://en.wikipedia.org/wiki/Extended_Display_Identification_Data
[edid-lib]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/graphics/display/lib/edid
[hdmi]: https://en.wikipedia.org/wiki/HDMI#Connectors
[novak-gamma]: https://blog.johnnovak.net/2016/09/21/what-every-coder-should-know-about-gamma/
[tmds]: https://en.wikipedia.org/wiki/Transition-minimized_differential_signaling
[vblank-interval]: https://en.wikipedia.org/wiki/Vertical_blanking_interval
[vga]: https://en.wikipedia.org/wiki/VGA_connector
