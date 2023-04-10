{% set rfcid = "RFC-0094" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }} - {{ rfc.title }}
<!-- DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

This document proposes a plan for converting the Virtual Console (Virtcon)
from a C++ program with a custom low-level graphics library
(See [gfx](/zircon/system/ulib/gfx)) to a Rust based
[Carnelian](/src/lib/ui/carnelian) application. The Terminal application
code used for Workstation and Terminal products will be unified with Virtcon
in the process, and Virtcon will gain the ability to use advanced vector
graphics and scalable text with high quality anti-aliasing.

## Motivation

### Reducing complexity and code

Fuchsia is currently maintaining two terminal implementations:

1. Virtcon.
2. Terminal application for the Workstation product.

Virtcon implements its own limited software rendering library, while the
Terminal application is powered by Carnelian's advanced vector graphics
rendering backend. There is a large amount of code duplication between
these implementations and reducing that would be a net positive for the
platform. The legacy graphics library ([gfx](/zircon/system/ulib/gfx))
used by Virtcon today can be deleted and we can reduce the number of ways
that we interact with the display controller API when this transition has
completed.

### New features

Carnelian-based Virtcon will maintain support for flicker-free single
buffer mode rendering while introducing support for tear-free double
buffered output on devices with sufficient display driver support. This
improves the visual appearance and results in better integration with
some hardware (E.g. FEMU) where single buffer mode is not well supported.

#### Scalable text

Carnelian's support for efficient rendering of scalable vector graphics
(including text) makes it trivial to adjust the size of console text to
match the density of the display.

#### Splash screen

The transition to Carnelian will modernize Virtcon and enable splash screen
functionality that existing and future products will benefit from. For
example, an animation can be presented during boot and the _debuglog_ can
be hidden unless the user press a special key or an error occurs.
Carnelian's support for [rive](https://rive.app) animations provide a
seamless way to design and integrate UI assets, while maintaining low
resource usage and efficient rendering.

## Design

Virtcon is used for bringup and it is critical that it continues to be
functional for this use-case. Any design changes that prevent that will
not be accepted.

Carnelian allows applications to run without Scenic or Root Presenter and
has an efficient software renderer for situations where Vulkan is not
available. This is already in use by the [recovery UI](/src/recovery/system/)
and the additional steps needed to allow a Carnelian-based application to
replace Virtcon has already been completed. Carnelian applications
running without Scenic communicate directly with display and input drivers
in a similar way to legacy Virtcon.

Carnelian-based Virtcon will be designed to maximize code reuse with the
Terminal application.

Fuchsia developers should not notice a significant difference from this
transition. Hardware requirements and FIDL APIs used to interface with
Virtcon are not changing.

## Implementation

Carnelian-based Virtcon will be similar to the recovery UI application but
connect to the display controller as a _Virtcon_ client instead of as a
_Primary_ client (See the [display-controller API](/sdk/fidl/fuchsia.hardware.display/controller.fidl) for details about the difference).

Maximum code reuse with the Terminal application will be accomplished by
implementing all shared logic as rust libraries that both components can
use. In some cases, this means extracting Terminal code into libraries.

A Carnelian-based text grid will be developed to power both the Terminal
application and Virtcon. This text grid will take advantage of Carnelian's
support for partial screen updates to provide performance that match legacy
Virtcon.

## Performance

Performance and resource usage is a critical aspect of this project. It is
critical that performance and memory usage does not regress significantly
as part of this transition. Double buffering is expected to increase memory
usage but that feature should be optional.

There should be no significant difference in:

* CPU usage.
* Memory usage.
* Build time.

The binary size of Virtcon is expected to grow but should be kept less than
1MiB.

## Testing

Test coverage for Virtcon should be the same or improved. All new Rust code
is expected to have unit tests and existing integration tests should be
converted to the new version of Virtcon.

## Documentation

At this stage, we plan to document Carnelian-based Virtcon through this RFC,
and this [README.md](/src/bringup/bin/virtcon/README.md).
