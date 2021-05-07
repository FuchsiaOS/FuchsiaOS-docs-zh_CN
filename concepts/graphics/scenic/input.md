# Scenic Input System

This document describes how RootPresenter and Scenic process visually-related
input, such as touch, mouse, and keyboard. We'll work roughly bottom up through
the layers of abstraction, from device to gesture.

Other inputs, such as buttons, audio, and video, are out of scope for this
document.

## Major Entities and Their High-Level Role

Drivers
: gives us inputs as InputReport FIDL tables

RootPresenter
: routes touch and mouse input from Drivers to Scenic

Scenic
: routes touch and mouse input from RootPresenter to UI Clients

Keyboard Subsystem
: routes and transforms text input to UI Clients

UI Client
: consumes inputs from Scenic and Keyboard Subsystem to drive UI

## Drivers Send Structured Input

Input Drivers provide access to input peripherals through the file system under
`/dev/class/input-report`. These are presented as structured FIDL tables that
describe the device and the reports that will be sent.

## RootPresenter Transforms and Routes Inputs

Generally, the RootPresenter is the singleton process that has detailed and
specific knowledge about the entire device, such as details about the display,
peripherals, sensors, etc. It takes care of device management details, such as
reading out InputReports reports from Drivers, and packages them into FIDL structs for
consumption by Scenic or other entities.

It also instructs Scenic to create the top-level (or "root") elements of the
scene graph, and vends the
[Presenter API](/sdk/fidl/fuchsia.ui.policy/presenter.fidl)
that UI clients use to attach their visual content to the scene graph.

The general transformation for an input event through RootPresenter is from the
Driver, to
[`InputReport`](/sdk/fidl/fuchsia.ui.input/input_reports.fidl),
to
[`InputEvent`](/sdk/fidl/fuchsia.ui.input/input_events.fidl).
The `InputEvent` is sent to Scenic.

### Implementation

The
[`InputReader` library](/src/ui/lib/input_report_reader/)
is the code responsible for actually monitoring `/dev/class/input-report` for new
peripherals, and reacting to new reports from existing peripherals. It forwards
new events for processing to other parts of RootPresenter.
More information on `InputReader` can be found
[here](/src/ui/lib/input_report_reader/README.md).

For each new peripheral (an input device), `InputReader` assigns a new
`InputInterpreter` object that reads the InputReport descriptor report for a single
input device, and performs bookkeeping by pushing a
[`DeviceDescriptor`](/sdk/fidl/fuchsia.ui.input/input_reports.fidl)
and its designated event forwarding channel, an
[`InputDevice`](/sdk/fidl/fuchsia.ui.input/input_device_registry.fidl#17),
to the
[`InputDeviceRegistry`](/sdk/fidl/fuchsia.ui.input/input_device_registry.fidl#12)
FIDL protocol. (The `InputDeviceRegistry` protocol also enables programmatic
input injection from outside RootPresenter.) The `InputDeviceRegistry` protocol
is vended by RootPresenter, and in addition to bookkeeping (details below),
informs each `Presentation` about the new peripheral.

For each new event, `InputInterpreter` reads a `fuchsia.input.report:InputReport`,
transforms it into a `fuchsia.ui.input:InputReport`, and forwards it on
`InputDevice::DispatchReport`.

The
[implementation of `DispatchReport`](/src/lib/ui/input/input_device_impl.h)
forwards the `InputReport` to the registered `InputDeviceImpl::Listener`,
typically the RootPresenter itself. In turn, the `InputReport` is forwarded to
the active `Presentation`.

For internal bookkeeping, each `Presentation` keeps a mapping of `InputDevice`
ID to an associated `DeviceState`. The `DeviceState` is used to create a little
persistent state for each peripheral, e.g., keeping track of a mouse device's
DOWN/MOVE/UP state. In `Presentation`, the `InputReport` is routed to its
relevant `DeviceState`, where it is transformed into an appropriate
`InputEvent`, and is sent to the `OnEventCallback` that was registered at the
`DeviceState`'s constructor (when the peripheral was first added).

The `InputEvent` is now handled by RootPresenter's `OnEvent` callback. It looks
for global hooks, displays a mouse cursor, adjusts for predetermined screen
rotation, and finally enqueues the `InputEvent` as an `InputCmd` to Scenic.

### Sensor Inputs

Sensor HID reports are handled in an analogous fashion. Some differences are:

*   Sensors typically don't have state to manage, so they have no `DeviceState`.
*   The `InputReport` is typically enough for plumbing out to clients.
*   Interfaces for sensor data is vended by RootPresenter itself; this may
    change in the future.

## Scenic Routes Pointer Inputs to UI Clients

In contrast to RootPresenter, Scenic has less knowledge about the device.
Instead of knowing about peripherals, it receives `InputEvent` FIDL structs from
RootPresenter. Generally, it owns and manages the large-scale visual elements
that each UI client creates (the scene graph), and handles mouse and touch input
(hereafter: "pointer input") dispatch to each UI client.

Root Presenter performs pointer event injections into Scenic over the
[`fuchsia.ui.pointerinjector`] protocol, which is a privileged capability.

Pointer events, such as touch, typically follow an ADD &rarr; DOWN &rarr; MOVE\*
&rarr; UP &rarr; REMOVE state sequence, encoded as
[`PointerEventPhase`](/sdk/fidl/fuchsia.ui.input/input_events.fidl).

On ADD, we identify the set of potential clients by performing a hit test, and
forward this event to these clients. To associate future touch events by the
same finger to the same clients, we track the set of clients for that particular
finger. Parallel dispatch is used to enable gesture disambiguation (TBD), where
the touch events should eventually be owned by a single client.

On DOWN, we send a `FocusEvent` to the single client that is "on top". We also
send a `FocusEvent` with `focused=false` to the previously focused client.

On MOVE and UP, we merely forward them to existing clients.

On REMOVE, we forward it to existing clients, and then remove the tracking
association.

For an overview of pointer coordinate mapping, see [Ray Casting and Hit Testing](view_bounds.md#ray-casting-and-hit-testing).

## Keyboard subsystem

The Keyboard Subsystem, implemented in [ime_service.cmx](/src/ui/bin/ime/src/main.rs), receives [`KeyEvent`s](/sdk/fidl/fuchsia.ui.input3/events.fidl), and routes them to UI clients.

The Keyboard Subsystem routes key events based on which client(s) have requested
such events, and whether or not those clients have received focus.