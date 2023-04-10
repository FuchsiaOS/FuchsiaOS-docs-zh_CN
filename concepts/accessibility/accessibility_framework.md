# Fuchsia Accessibility Framework

## Overview

Every modern operating system implements a different framework to support
assistive technology (e.g. IAccessible on Windows, NSAccessibility on OS X, ATK
on Linux, AccessibilityNodeInfo on Android). Specific accessibility features
such as screen readers are built on top of those frameworks. These frameworks
are typically responsible for understanding what is on screen at a given time
through a semantic tree, and providing APIs to mediate input and output to the
device so that accessibility services can implement different modes of
navigating the UI. There is a nice overview of this available in the [Chromium
public docs][chromium].

Some examples of accessibility features include: screen readers, switch access
control, braille devices, magnification, high contrast mode, color correction,
reduced motion, closed captioning, and more.

Fuchsia's accessibility framework is primarily implemented in the Accessibility
Manager. It supports the following accessibility features:

* [Screen reader][screenreader]
* [Magnifier][magnifier]
* [Color Correction and Color Inversion][colorcorrection]

## Accessibility Manager

Accessibility Manager has the following responsibilities:

* Manage accessibility settings
* Implement the [semantics API][semantics]
* Integrate with [Scenic][scenic] (fuchsia graphics renderer & touch handler) to
modify the display to support accessibility features such as magnification
* Understand the relationship between scenic views and clients (e.g. flutter)
* Mediate input and output (e.g. touch, audio) as needed by accessibility
services
* Mediate access to system services (e.g. TTS) needed for accessibility features

The Accessibility Manager is a part of the Fuchsia platform and runs on any
Fuchsia product with a screen. It is closely integrated with
[Scenic][scenic].

Accessibility manager code lives in
[/src/ui/a11y/bin/a11y_manager/](/src/ui/a11y/bin/a11y_manager/).

## Settings

Accessibility settings will be managed through
[SetUI](/docs/development/settings/introduction.md), which is responsible for
storing settings locally and notifying the Accessibility Manager when settings
change. This is done via the
[fuchsia.settings.accessibility](/sdk/fidl/fuchsia.settings/accessibility.fidl)
API. The accessibility manager is responsible for notifying other system
components of changes (e.g. scenic, flutter/chromium runtimes).

For additional information see [Accessibility Settings][a11ysettings].

## Semantic Tree Data

The Fuchsia UI is composed of views (surfaces drawn by Scenic). Views are
positioned within a view tree. Each view has a client responsible for the
content of the view, typically Flutter or Chromium. Each client knows how to
supply a semantic tree corresponding to the elements the user can currently view
or interact with. This is generally a simplified version of the current graphics
rendering tree which includes elements which are semantically meaningful to
users but eliminates things like buffer spaces and bounding boxes.

The API can be found in
[fuchsia.accessibility.semantics](/sdk/fidl/fuchsia.accessibility.semantics/).
This API allows a runtime to register with the Accessibility Manager to provide
semantic updates, and then call Update, Delete and Commit methods to send
information about its semantics. The Accessibility Manager is responsible for
implementing the server side of the semantics API.

For additional information see [Accessibility Semantics][semantics].

## Accessibility View

The accessibility manager owns a view in the Fuchsia [view
tree](/docs/development/graphics/scenic/concepts/view_ref.md).  This view allows
accessibility to intercept gestures, change graphics, inject input, and
participate in focus changes.

For more information see [Accessibility
View][a11yview].

## Input and Output

### Touch and keyboard events

The accessibility manager works with scenic to intercept user input events when
relevant accessibility features are enabled and implements a gesture arena to
handle user gestures.

For more information see [Accessibility
Input][a11yinput].

### Focus management

The accessibility manager has a notion of "accessibility focus", or which
semantic node is currently active. The accessibility manager is the
source-of-truth for which node has accessibility focus.  For more information
see [Accessibility
Focus][a11yfocus].

### Highlights

The accessibility manager works with scenic to draw a "highlight" around the
currently focused node.  For more information see [Accessibility
Highlights][highlights].

### Text to Speech

The Accessibility Manager will provide APIs for accessibility services to
trigger speech events.

For more information see
[TTS][tts].

## Testing the Accessibility Framework

Accessibility tests live in
[src/ui/a11y/lib/testing/](/src/ui/a11y/lib/testing/).

[chromium]: https://chromium.googlesource.com/chromium/src/+/lkgr/docs/accessibility/overview.md
[semantics]: https://fuchsia.dev/reference/fidl/fuchsia.accessibility.semantics
[screenreader]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=78636
[magnifier]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=78645
[colorcorrection]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=78644
[a11ysettings]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=78643
[a11yview]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=78640
[a11yinput]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=78638
[a11yfocus]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=78637
[highlights]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=78639
[tts]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=78642
[scenic]: /docs/concepts/ui/scenic/index.md
