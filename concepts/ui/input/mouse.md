# Mouse

This guide describes a very high-level event plumbing and policy knobs for
conveying a mouse device's events to a User Interface (UI) client. For a more
general overview of input, beyond mouse, see the
[User Input
architecture](/docs/contribute/governance/rfcs/0096_user_input_arch.md) RFC.

## Background

A [mouse device](https://en.wikipedia.org/wiki/Computer_mouse){:.external} typically has a
motion sensor, a number of buttons, and a scroll wheel. Many variations and
configurations exist, but this page discusses just the basics.

When the user moves the mouse along a flat surface, its physical motion is used
to control the motion of a cursor on a display. When the user clicks a button,
its state change (down/up) is used to drive interactions in a UI. When the user
moves the scroll wheel, its motion is used to drive motion of graphical content
in a UI.

## Protocols and event flow

A mouse event starts from a mouse device and flows up the stack, until it
reaches a UI client, which reacts to a mouse event. A UI client is typically
implemented on top of a UI framework, so when a mouse event reaches this level,
it gets exposed in a framework-specific way.

![Mouse event flow](images/mouse-event-flow.png)

### Device

A mouse device connects to a "host" (your computer), and speaks the USB HID
protocol. The protocol specifics are defined in a
[HID usage table](https://www.usb.org/hid){:.external}. The host waits to be
[interrupted](https://wiki.osdev.org/USB_Human_Interface_Devices){:.external} for HID
reports from the device. The mouse sits inert, until there is something to
report to the host, such as motion, button press, or scrolling.

### Input driver

On the host side, Fuchsia's input driver stack reads the mouse's HID reports and
translates them into a FIDL
[`MouseInputReport`](https://fuchsia/dev/reference/fidl/fuchsia.input.report#MouseInputReport).
This translation improves ergonomics for the upper layers of the Fuchsia
platform: the components in the upper layers can process events in FIDL, instead
of HID-specific data formats.

Input driver allows higher-level components to read reports from each device
over a Zircon channel. Each device is initially surfaced as a file in
`/dev/class/input-report`.

### Input pipeline

The Input Pipeline component attaches to a mouse device by discovery in the
`/dev/class/input-report`
[directory](/docs/concepts/components/v2/capabilities/directory.md). Only platform
components have access to this sensitive capability.

This component provides a centralized location for routing mouse events and for
implementing policy decisions.

Physical mouse motion is typically interpreted as cursor motion, and this motion
data is sent to the Scenic component via a sensitive
[pointer injector](https://fuchsia/dev/reference/fidl/fuchsia.ui.pointerinjector#Device) API.
Button events and scroll wheel events also travel on the same injector channel.

The Input Pipeline implements the following centralized policy decisions:

*   Choosing which [view
    tree](/docs/contribute/governance/rfcs/0147_view_system.md)
    to target for mouse event injection.
*   Defining how to interpret a mouse's physical motion. For example, a mouse's
    physical motion could be set up to control the movement of the cursor, or
    instead of cursor control, reported to UI clients as relative motion data.
*   Defining the mouse's motion resolution.
*   Defining the button priority order (primary, secondary, etc). For example,
    reordering priority allows users to make a mouse's right-side button the
    primary button.
*   Defining the scroll wheel's travel (per detent) and direction.

Generally, these fields may be extended in the future, to support a richer or
more precise interpretation of the user's intentions for mouse usage.

### Scenic

The Scenic component forwards injected mouse events to UI clients, following
policy defined by the injector (Input Pipeline). Scenic performs hit testing in
the view tree to determine which UI client should receive mouse events.

When a mouse target is identified, Scenic dispatches mouse events to that UI
client over the
[`MouseSource`](https://fuchsia/dev/reference/fidl/fuchsia.ui.pointer#MouseSource) API. UI
clients listen for mouse events with a
[hanging-get](/docs/development/api/fidl.md#hanging-get) FIDL pattern.

### UI clients and UI frameworks

A UI client can freely make use of the mouse event it received from Scenic. It
can read cursor motion, button events, and scroll events as part of creating a
user experience.

If the UI client is implemented on top of a UI framework, such as Flutter or
Chromium, the framework code is responsible for implementing Fuchsia-facing code
to receive mouse events. It then must surface these mouse events in a
framework-specific way to the UI client implemented on top of that framework.
For example, in the Flutter framework, mouse events are surfaced to Flutter
programs via the
[`PointerEvent`](https://api.flutter.dev/flutter/gestures/PointerEvent-class.html)
Dart class.
