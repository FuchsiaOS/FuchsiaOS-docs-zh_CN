{% set rfcid = "RFC-0096" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }} - {{ rfc.title }}
<!-- *** DO NOT EDIT ABOVE THIS LINE -->


## Summary

This RFC describes the target high-level architecture for delivering user input
events (keyboard, mouse, touch, etc.) on Fuchsia on systems with a graphical
user interface and multiple runtimes. Users provide input to the system through
a variety of methods/devices including keyboards, mouse, touch and buttons. This
RFC covers how input events on Fuchsia go from driver-level raw data to input
events that are dispatched to userspace software (e.g. a Flutter app). The
components described are under development as of publication.

## Motivation

To support Fuchsia's inclusive philosophy, Fuchsia input must
provide platform-level support to components built on top of different runtimes
that utilize different UI frameworks (e.g. Flutter, Chromium), as well as allow
product owners to customize input behavior. On other platforms these behaviors
are typically baked into a specific UI framework's implementation. Fuchsia
presents a number of unique challenges to separate platform input processing
from the specifics of a particular UI framework.

In the Fuchsia code, there are currently multiple input routing paths ([root
presenter][root-presenter], [input pipeline][input-pipeline]) with limited
public guidance about the preferred path and migration plans. This RFC outlines
the target architecture for user input routing on Fuchsia and provides guidance
about how to extend input handling for future use cases.

### Requirements

* _Security:_ The Fuchsia input stack makes it easy to build secure products.
  * Input events may contain sensitive data such as passwords and payment data,
    as well as information about when the user is active. All user input events
    should be treated as personally identifiable information and only dispatched
    to end user software by trusted system components.
  * User input can also be misused if routed incorrectly, for example by a
    malicious UI causing a user to click a button they did not intend to
    click ("clickjacking").
  * Whenever possible, the Fuchsia platform should encourage developers to build
    secure products by providing an API surface that makes it easy to understand
    and audit the flow of user input events through the system.
* _Correctness:_ Input events are delivered in accordance with user
  expectations.
  * The Fuchsia input system is responsible for interpreting input events in
    context and delivering them to the correct target component(s) currently
    running on the system. However, the definition of "correct" may vary
    depending on the event and product types.
  * Input event delivery should remain consistent even when the UI is animating
    or changing size.
  * Events may sometimes be dispatched to multiple components simultaneously
    (e.g. keyboard shortcuts).
* _Performance:_ User input is fast (enough).
  * While the exact latency requirements vary by input device and product type,
    input delivery is particularly latency-sensitive and should strive to be
    fast enough that users do not perceive lag.
  * The input architecture should avoid introducing unnecessary latency. In
    particular, care should be taken around process context switches to avoid
    unnecessary blocking calls.
* _Customizability:_ The system should support different user input behavior for
  different products built on the Fuchsia platform.
  * Specifically, the product [session][session] should have the
    capability to customize input behavior. However, the initial implementation
    proposed in this RFC will only meet a subset of the eventual requirements
    for customization. (See [Input pipeline
    customization](#input-pipeline-customization).)
  * Some products will require keyboard and mouse support while others focus
    primarily on interaction via touch and buttons.
  * The Fuchsia platform should provide hooks for product-specific
    customization of input behavior (for example, differing interpretation of
    button events depending on the system context).
  * Fuchsia should allow products to map input events to different types as
    needed, for example reinterpreting touch events as mouse events for
    compatibility with software that does not support touch gestures.
* _Extensibility:_ Fuchsia can add support for new input modalities.
  * While products with substantially different input needs may require changes
    to the platform, it should be possible to add support for new input methods
    without substantially re-writing the existing input stack.

### Motivating examples

These use cases are in no way comprehensive, but provide some insight into the
types of behavior this architecture ought to support.

* Correctly interpret touchscreen gestures when UIs from multiple frameworks are
  onscreen.
* Route keyboard input with the appropriate layout info applied when multiple
  text boxes (backed by different frameworks) are onscreen.
* Allow a button combination (e.g. volume button up and down together) to
  trigger factory reset in addition to the usual function of those buttons.
* Suppress input on a laptop touchscreen when the device is asleep or closed.
* Interpret laptop touchpad gestures (pinch zoom, two finger scroll).

### Background and terminology

* _Human interface device (HID)_ – a device that allows input and output to
  users such as a keyboard, mouse, touchscreen or consumer control
  (button). Typically this refers to a device that uses the [USB-HID][HID]
  specification.
* _Input event_ — a single user input event such as a key press, mouse move
  action, or touch event. As they are processed events may be annotated with
  additional information depending on the current context.
* _Pointer event_ – a user input event that corresponds to a position on a
  screen, such as a touch or mouse event. Pointer events are a subset of input
  events.
* _Event stream_ - a set of related input events that typically take place
  close together in time.
* _Input handler_ — a piece of software that executes a single stage of input
  processing. An input handler takes input events as input and also emits input
  events. It may modify the events or communicate with other parts of the
  system.
* _Input pipeline algorithm_ – an algorithm that involves chaining together a
  series of input handlers to process Fuchsia input. This acts as the policy
  layer for input on Fuchsia.
* _Input pipeline implementation_ - an implementation of input pipeline
  algorithm. In practice this is a component in fuchsia.git responsible for
  routing driver-level user input events to the rest of the system.
* _[Scenic][scenic]_ – the Fuchsia graphics engine. Scenic is also responsible
  for routing pointer events.
* _[Global scene graph][scenegraph]_ – a tree of graphical content rendered by
  Scenic.
* _[View][views]_ - a subspace within the scene graph. A view typically
  corresponds to a region on screen, although this region is not necessarily
  rectangular.
* _[ViewRef][viewref]_ - an [event pair][event-pair] associated with a
  particular view. This is used to identify the view across multiple Fuchsia
  components.

## Architecture overview

This design provides an overview of the overall flow of user input events
on Fuchsia but does not cover all of the particular details for each type of
input. Many of those details are addressed in vertical-specific designs linked
from the appropriate sections below.

![bottom to top: dev/class/input-report -> input pipeline -> {scenic OR a11y
manager OR IME manager OR media buttons listener} -> {Chromium OR flutter OR
carnelian} -> {View 1 OR view 2 OR view3 OR view 4} On the side: Product Session
component -> {input pipeline OR root scene component} Root scene Component ->
Scenic inside scenic: focus manager, scene graph (with a visual tree
representation) inside input pipeline:
bind -> pointer event handler -> a11y shortcut handler -> media buttons handler
-> assorted other handlers -> IME handler ->
fallback](resources/0096_user_input_arch/input_pipeline.png)

Note: This diagram shows a sample implementation of the input pipeline for a
product that uses touch and mouse events, media button events, and keyboard
events. It does not include all possible input handlers or system services. For
example, keyboard shortcuts, which will be present on most products with a
physical keyboard, are not shown. For additional detail please refer to the
[input verticals](#verticals) section below. The input pipeline will contain
different handlers on different products.

Events are dispatched according to a number of interrelated rules, including but
not limited to:

* Product policy for that event type (e.g. all volume events are dispatched to
  settings)
* The event's on screen location (e.g. for touch or mouse)
* The currently focused view (e.g. for text entry)

The [input pipeline](#input-pipeline) is a Fuchsia component that is responsible
for managing these rules and their interactions, and for routing user input
events to the appropriate system services along with the information needed to
process those events. It implements the input pipeline algorithm (a system of
chained input handlers each performing a discrete step in the device input
processing). This component acts as the policy layer for input on Fuchsia. The
input pipeline ingests events directly from the driver layer via the [input
drivers][drivers], as well as synthetic events from testing and
software-generated events such as those from a virtual keyboard.

As events move through the input pipeline they progress from global, low-level
events (no semantic meaning, but sensitive data) to scoped, processed data with
local meaning that can be interpreted by end-user software (gestures,
characters, etc). Driver-level input events come in two broad categories:

* Pointer events which correspond to a particular location on screen.
* Button and switch events.

This could be expanded in the future. The USB-HID usage table is [vast and
contains multitudes][HID], including pages for flight simulators, exercise
equipment, and virtual reality devices.

Driver-level input events may be transformed into:

* Touch gestures.
* Mouse events (including scrolling).
* Text input events.
* Semantic accessibility actions.
* Button events (e.g. volume change, camera on/off).

Typically each event is dispatched as follows:

```none
Driver -> Input Pipeline -> UI System Component -> UI Framework-> UI View
```

* The input pipeline component contains a number of internal phases called Input
  Handlers. The input pipeline acts as the policy layer for input, and may vary
  by product type. (See [Input Pipeline
  Customization](#input-pipeline-customization) below.)
* UI System components include Scenic, the Accessibility Manager, and the IME
  manager but will likely expand over time as we add features to the
  platform. These components are part of the Fuchsia platform and live in
  fuchsia.git. These components are product agnostic.
* UI System Components are responsible for deciding which view or views should
  receive an event and passing that information through to the UI framework
  runner for dispatch. UI system components use information from Scenic
  (typically the [focus chain](#focus)) to determine which view should receive
  events.
* UI System Components may also consume events.
* UI Frameworks currently include Flutter, Chromium, and Carnelian. We expect to
  support additional UI frameworks/runtimes in the future. These may be used
  across a variety of product types but are not part of the Fuchsia platform.
* UI Views are graphical components that own a corresponding screen region
  called a view. (See [views and event routing](#views-and-event-routing)
  below.)
* Media button events do not always follow this pattern. The final destination
  for media buttons may be the settings service rather than a UI view.

In some cases events may go through additional stages before final delivery.
For example, a touch event might be dispatched through Scenic to a virtual
keyboard component, which generates a synthetic key press that is then
dispatched through the input pipeline.

The input pipeline sends pointer events to Scenic, which is responsible for
dispatching the event to the correct runtime instance. This allows Scenic to
maintain a globally consistent understanding of where things are on screen,
and avoid race conditions during animations. (See [Routing Graphical
Events](#routing-graphical-events) below.)


## Design

### Source of input events
Input events typically enter the system through the [input drivers][drivers] or
through a Fuchsia component that acts as a controller for an input device
(e.g. the bt-host component for Bluetooth HID devices). In many cases these
start as Human Interface Device (HID) events, but there are some
exceptions. Drivers and controllers emit each event as a
[fuchsia.ui.input/InputReport][input-report]. In general input reports are then
consumed and transformed by the input pipeline. However, in recovery mode they
are sent directly to the terminal component implemented using
[Carnelian](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/lib/ui/carnelian/)
without any intermediary processing.

Input events for testing are generated differently. (See [testing](#testing)
below.)

Note: [`fuchsia.ui.input/InputReport`][input-report] can also send
[OutputReports][output-report] back to devices to change their state. This can
be simple action in response to an input event (e.g. setting a capslock LED) or
a more complex configuration of device state such as mouse DPI or even firmware
updates. Simple cases may be handled in the input pipeline (see [Input
handlers](#input-handlers) below). Complex cases may be considered a form of
configuration change and will be addressed in a subsequent RFC.

### Input pipeline {#input-pipeline}
The [input pipeline][input-pipeline] component is a Fuchsia system component. It
implements the input pipeline algorithm. The input pipeline component acts like
the policy layer in the input stack. It is responsible for determining which
event types are supported on a particular device and how input events should be
dispatched, and for managing input devices.

The input pipeline algorithm is composed of:

* A bind phase that transforms a stream of raw InputReports (which contain no
  system state) into a stream of [InputEvents][glossary.InputEvent] (which may contain
  system state) to be passed through the pipeline.
* A series of [InputHandlers][glossary.InputHandler] that may modify and/or consume
  these InputEvents, possibly by dispatching them to other components.
* (optionally) A fallback phase that processes unhandled InputEvents.

The bind phase typically augments InputReports with information about the
device state. Examples:

* An InputReport containing relative mouse movement becomes an InputEvent with
  screen-relative coordinates.
* A stream of key press InputReports becomes a stream of "key down" and "key up"
  InputEvents. (The driver typically only reports which keys are down at a given
  moment in time rather than separate up and down events.) The "key up" may be
  inferred from the absence of a key in a later InputReport.

The product's use of the input pipeline may in theory be configured by the
Fuchsia product [session component][glossary.session-component]. However, the
input pipeline does not run inside the session realm. The input pipeline is a
more privileged component than the session and makes use of a number of
capabilities that we do not currently wish to expose to session components.

Input pipeline implementation(s) are provided to product owners as part of the
Fuchsia platform. As of the publication fo this RFC, these implementations are
written in Rust and the input handlers are classes that implement a shared
InputHandler Rust trait, although this will likely change in the future to allow
for optimization and extensibility.

Currently the platform supplies two input pipeline implementations - one that is
optimized for devices that rely primarily on touchscreen interaction, and
another oriented around a product with a mouse and keyboard. This limited degree
of customization will likely be extended in the future. (See [Input pipeline
customization](#input-pipeline-customization) below.)

#### Input handlers {#input-handlers}

[Input Handlers][glossary.InputHandler] represent a series of stages for input
processing. Input handlers are the primary mechanism that allows product owners
to customize input handling related to product state. We also sometimes refer to
this as product policy for input events. Input handlers may

* Augment an event by adding contextual information (for example, the active
  keyboard layout, or which keys were held down at the same time as a mouse
  click).
* Alter events based on their surrounding event stream (e.g. input smoothing,
  touchpad gestures).
* Handle an event either within the handler or by sending it to a service
  exposed by a UI system component, which is in turn responsible for dispatching
  that event to the appropriate set of UI views. The target view or views are
  determined by the current view focus in Scenic.  (See [Focus](#focus) below.)
* Send [OutputReports][output-report] to control input device state
  (e.g. setting the capslock LED).

In the short term, we define a Rust trait `InputHandler` that must be
implemented by every input handler. Longer term, this could be replaced by a
FIDL interface. Each handler must have the ability to consume an input event and
should output a (possibly empty) vector of input events.

```rust
#[async_trait]
pub trait InputHandler: Send {
    /// Returns a vector of InputEvents after handling `input_event`.
    ///
    /// # Parameters
    /// `input_event`: The InputEvent to be handled.
    async fn handle_input_event(
        &mut self,
        input_event: input_device::InputEvent,
    ) -> Vec<input_device::InputEvent>;
}
```

Example input handlers that future products built on Fuchsia might need (some
are quite fanciful):

* _Accessibility Input Handler_: Sends events to a11y manager when a relevant
  accessibility feature (such as switch navigation or a screen reader using
  keyboard shortcuts) is enabled.
* _Shortcut Handler_: Determines whether keyboard events match an active
  keyboard shortcut (may call out to another component).
* _Locale Handler_: Applies information about the active locale for the current
  view.
* _Pointer Event Dispatch Handler_: Sends touch/mouse/stylus events to Scenic
  for dispatch to views.
* _Media Button Handler_: Routes media buttons to the settings service (or
  elsewhere).
* _Keyboard Layout Handler_: Annotates keyboard events with the currently active
  keyboard layout.
* _TouchPad Gesture Handler_: touchpad gestures and forwards equivalent mouse
  events.
* _Input Smoothing Handler_: Reduces jitter by averaging events.
* _Focus Handler_: Annotates keyboard events with the ViewRef for the currently
  focused view.
* _Sleep Mode Handler_: Suppresses input when device is asleep.
* _Multitouch Waffle Iron Magic Button Handler_: Interprets touch gesture events
  as media buttons.

In many cases input handlers will dispatch events to Fuchsia system services
such as Scenic, the Accessibility Manager, the IME Manager, and so on. In this
case, the event will be marked as "handled" and propagate as such through the
remainder of the pipeline. (See [event stream
consistency](#event-stream-consistency) below.)

#### Input pipeline customization {#input-pipeline-customization}
The input pipeline is instantiated by specifying the handlers to be included and
the order in which they should appear. This makes adding handlers, reordering
handlers, or instantiating a modified pipeline very lightweight. An example in
Rust:

```rust
async fn input_handlers(
 ...
 ...
) -> Vec<Box<dyn InputHandler>> {
    let mut handlers: Vec<Box<dyn InputHandler>> = vec![];
    // Shortcut needs to go before IME.
    add_shortcut_handler(&mut handlers).await;
    add_ime(&mut handlers).await;
    add_touch_handler(..., &mut handlers).await;
    add_mouse_handler(..., &mut handlers).await;
    handlers
}
```

In 2021, the platform provides two input pipeline implementations, each of which
is implemented as a component in fuchsia.git able to take advantage of
privileged APIs not published in the fuchsia SDK. These implementations share
many input handlers (they run the same code) and differ primarily in terms of
which input modes they support. One implementation is optimized for devices that
rely primarily on touchscreen interaction, and another oriented around a product
with a mouse and keyboard. The product-specific code is limited to the setup
code that determines handlers to instantiate.

In the short term, limited configurability can be granted to sessions by
exposing a capability (likely in the form of a FIDL API for configuration
mediated through the session framework) to allow the session to determine which
input modes it wishes to enable. This API would allow configuration on a
per-product basis and would take effect at session startup time (i.e. when the
product user experience is configured). There is no intention to support "on the
fly" reconfiguration of the pipeline during the course of normal product
operation.

As Fuchsia expands to support additional product categories, the set of input
handlers and the different product configurations will likely expand, and it may
become necessary to expose a capability that allows the session to either pick
from a variety of pipeline configurations or directly configure which input
pipeline stages should be present and in what order. Some input handlers may be
product-specific and need to live outside of fuchsia.git.

Each product should be able to use an input pipeline that is tailored to its
particular needs, composed of input handlers needed for that product's supported
input behaviors. The decision of how exactly to do this is out of scope for this
RFC. Ideally, any such configuration solution would take advantage of
[structured configuration][config-roadmap] mechanisms provided by the platform
rather than inventing something specific to input, and should take pains to
expose the minimum possible configuration surface required to meet product
needs.

### Views and event routing {#views-and-event-routing}

The Fuchsia UI may contain graphics from multiple runtimes on screen at the same
time. The UI is organized into a [global scene graph][scenegraph], owned by
Scenic, which contains the graphical content that makes up the UI as well as the
information needed to render it. A runtime can contribute user-visible content
to the scene by adding it to a Scenic resource called a [view][views], which
is installed in the scene graph. Typically a view corresponds to a region on the
screen, although these regions are not necessarily rectangular and not all
views are visible at a given time. Because each view is a Scenic-local resource
and not suitable for referencing in other components, we associate a kernel
object, called a [ViewRef][viewref], with each view.

Views are organized hierarchically, with a child view only able to affect screen
real estate within the bounds of its parent view, i.e. a strict containment
model." When a view is the parent of another view in the graph, the parent view
retains certain control over its child, in particular as it pertains to content
placement and event routing.

Note: All user input is potentially sensitive data, since it can include
passwords, payment details and other personally identifying information. It is
particularly important that user input is routed only to the view that matches
user expectations and the Fuchsia security model, and not to something that is
offscreen, disallowed by the user, or otherwise unexpected.

#### View Tree
The diagram below illustrates the structure at the root of the scene graph.

![root -> accessibility view -> sysUI -> {some view, some other view, yet
another view}](resources/0096_user_input_arch/scene_graph.png)

* The view associated with the accessibility manager allows the accessibility
  manager to intercept input events required for accessibility services and
  change focus, as well as drawing "on top" of the rest of the UI when
  necessary.
* The "sysUI" (system UI) view is the parent of all the other views. This view
  is typically used to implement "system gestures" (such as a swipe on the edge
  of the screen to change apps) and system keyboard shortcuts. It is also used
  for many non-input aspects of the UI.

#### Routing graphical events {#routing-graphical-events}
For user events that correspond to a particular on screen location (e.g. touch,
mouse, stylus), input events are routed through Scenic before dispatch to the
runtime instance associated with each view. Advantages of this approach include

* _Scene graph isolation_: Scenic is the only component with a complete
  understanding of which view is where on screen. We do not allow other
  components to look up what is on screen at a given point (also called
  "hit testing").
* _Global consistency_: Because Scenic is the source of truth for where a given
  view is on screen at a certain time, having Scenic dispatch events avoids
  problems where a view changes size, location or disappears between the time
  where an input event occurred and when it was delivered. This is particularly
  important for products where there may be frequent changes while a user is
  interacting with the device. Note that Scenic also owns the focus chain, which
  is used to keep dispatch consistent for non-pointer events.
* _Parallel dispatch (gesture disambiguation)_: When views overlap, there may be
  multiple views interested in a particular event stream. For example, a product
  may wish to implement a "system gesture" such as a swipe to dismiss an
  application, while the application itself may interpret that gesture
  differently. Scenic is responsible for determining which views should receive
  a given event and mediating between them through a process called gesture
  disambiguation. In this mode, streams of touch events are dispatched in
  parallel to multiple views, which is then resolved in a "gesture arena" to
  determine which view will ultimately consume the event stream.

#### Focus {#focus}
In order to determine which piece of currently-running software should receive a
given input event, we need a notion of view focus. This is roughly the view
which is currently "active" and ready to receive events. However, in practice
multiple views may be interested in each event.

View focus is determined through the [Focus Chain][focuschain] (a vector of
ViewRefs corresponding to the focused view and its ancestors in the view
tree). The focus chain is owned by Scenic (because Scenic manages views) but
other components may request changes to the current focus, for example in
response to an accessibility action or a keyboard shortcut. The input pipeline
is responsible for monitoring changes in the focus chain and provides the
information to input handlers, which may in turn forward events to the correct
client component/view. The terminal element of the focus chain corresponds to
the currently focused view.

Some handlers will be interested in a singular focused view, while others will
be interested in the whole focus chain. For example:

* Keyboard events that don't trigger a keyboard shortcut are typically routed to
  the currently focused view.
* The accessibility manager uses the focused view to determine what is currently
  "active" to the screen reader when it is enabled.
* The keyboard shortcut manager is interested in the entire focus chain, any of
  which may have registered a keyboard shortcut and associated priority.

Focus events move through the input pipeline as a special type of InputEvent and
are strictly ordered with other input events.

#### Event stream consistency {#event-stream-consistency}
An event stream is a set of related InputEvents that typically take place
close together in time. For example:

* Keyboard: 'a' KEY DOWN -> 'a' KEY UP
* Mouse: HOVER -> HOVER -> BUTTON_DOWN -> BUTTON_UP
* Touch: FINGER DOWN -> MOVE -> MOVE -> MOVE -> FINGER UP

The system must ensure consistency in the event stream at each stage of the
pipeline and for each view. This means that if an input handler consumes an
event by routing it to a system service, it should send an appropriate "handled"
event to subsequent input handlers so they may notify any clients.

This is also true when focus changes while processing an event stream. From a
client standpoint, every "key down" must be matched with a corresponding "key
up", or a "cancel" event if focus changed or the input device disconnected. The
same is true for mouse clicks and touch event streams. The input pipeline is
responsible for marking events as "handled" and propagating them through the
input pipeline to ensure stream consistency. System services are responsible for
notifying views when a stream is cancelled.

### Performance

#### Acceptable Latency
User input is time sensitive. Latency (meaning, the time from when an event
occurs to the time the UI responds) should ideally be as low as possible, though
users' tolerance for delay varies by input type. Users may experience degraded
performance (can they accomplish a task) and satisfaction at latencies as low as
10ms in some cases. User experience starts to degrade noticeably above 100ms and
may be unacceptable over 300ms. Direct manipulation (e.g. a stylus drawing on a
screen) is especially sensitive to delays and may require prediction of events
to supply an acceptable user experience.  (See this [latency
paper][latencyreference] for context.)

Because the input system described in this RFC runs at a lower level than a
runtime and the UI built on top of it, the user will experience the system input
latency as well as any latency caused by the time to process and render a
response to the input event. Thus, the input system should strive to be as fast
as possible to leave as much of the "latency budget" as possible to applications
and runtimes.

Additionally, consistent timing matters. Even when average event delivery time
is low, high variability in event timing can degrade the user experience, so
it's important to look at the latency distribution as well as averages.

#### Improving Performance

The best way to reduce latency in the input architecture is by minimizing
unnecessary process context switches. Every entry and exit of the kernel,
typically requiring running the scheduler, introduces variability into an
event's timing.

In the future we may explore running multiple components in the graphics and
input stack (e.g. scenic and input pipeline) as separate components within a
single process to further reduce process hops. We also may revisit the choice of
Rust as an implementation language for the input pipeline if we find that this
incurs extra latency.

The introduction of gesture disambiguation (also called parallel dispatch) for
touch events has the potential to introduce additional delays while waiting for
interested components to respond to a given event stream. In order for this
algorithm to be performant, clients must cooperate and respond promptly to input
events. The system will need some mechanism for specifying and enforcing client
latency expectations (SLAs). This will be elaborated in a future design.

### Internationalization and Input Context
Every view has its own input context, which includes the active [input
method][inputmethod] ("active keyboard"). The input context is distinct from the
information in
[fuchsia.intl.Profile](https://fuchsia.dev/reference/fidl/fuchsia.intl#Profile)
which contains the user's preferred locales and affects the UI
presentation. However, the user's locale settings may affect which input
methods/keyboard layouts are available. See [the Fuchsia internationalization
documentation][i18n] for more information on Fuchsia internationalization.

The system should permit different views to have different active input
methods. For example, a user may write an email in one language while chatting
in another language. Products may choose to enforce a single system-wide locale
or active input method, but the architecture must support a distinct input
context for each view. Exactly how to store these settings will likely be
decided in the future.

In addition to routing events to the right view, the input pipeline (and
associated system components like IME Manager) will use the input context for
that view when interpreting input events. For example, the input pipeline would
annotate a physical keyboard event with information about the keyboard layout
that was active in the input context in which the event occurred. As with focus
changes, changes to the active keyboard layout should considered Input Events
and processed sequentially with other events to avoid race conditions with
changing state.

### Accessibility
In order to make Fuchsia devices accessible to users regardless of ability, the
Fuchsia accessibility framework provides a number of accessibility features that
alter how the users interact with the device. In particular, this enables:

* A magnifier that "zooms in" on some or all of the UI.
* A screen reader that allows blind or low vision users to explore and interact
  without visual input via a "semantic tree" corresponding to the current UI.

When one or both of these features is enabled, the accessibility manager needs
to intercept input events via the input pipeline and interpret them as commands
to the currently active accessibility feature. These commands can use multiple
input modalities depending on the type of device. For example, a workstation
screen reader would operate primarily using keyboard shortcuts, while a screen
reader on a touchscreen device might use a series of taps and swipes. Depending
on which accessibility features are enabled, the accessibility manager may
decide to consume only some events (as in the case of the magnifier, which
consumes some gestures but allows others to pass through to the UI) or all
events (as is the case of the screen reader, which translates events to semantic
actions).

The accessibility manager maintains a connection to each view that allows it to
examine, describe and interact with the UI element in that view via the
semantics API, `fuchsia.accessibility.semantics`. For example a "double tap"
with the screen reader enabled is typically passed through to a view as a
semantic "default action" on the currently selected semantic node.

### Security considerations
The input pipeline and associated system components make use of a number of
privileged APIs that are not published in the SDK. By requiring sessions to use
the input pipeline for their input handling, the platform is able to limit the
capabilities available to external software.

It is also important to consider UI redress attacks such as
clickjacking. Misdirected input events can be used to grant privileges without
the user's consent (e.g. routing a click to a button on a malicious website).
While this is difficult to fully prevent at the platform level, the input
architecture must ensure the graphic events are delivered only to the correct UI
components and that it is easy for product owners to understand the flow of
input events through the system.

### Privacy considerations
Changes to the input pipeline should go through privacy review, as access to
user input could allow attackers to create a keylogger or other malicious
software. With the exception of recovery mode, the input pipeline should be the
only component allowed user input events directly from the driver to prevent
this.

### Testing {#testing}

#### Testing the input pipeline
Platform input behavior should be verified with hermetic integration tests
corresponding to supported input features (e.g. touch input, keyboard shortcuts)
independent of the product code that uses these features. Tests should use
minimal graphical components in each supported runtime to verify the relevant
functionality. Ensuring feature stability independent of a particular product is
important for allowing developers to build products out-of-tree.

Hermetic testing presents a number of challenges that are out of scope for this
RFC.

#### Testing everything else
End-to-end tests rely heavily on synthetic input events to fake user interaction
in a reproducible way for tests. While most UI frameworks include some way to
inject events (e.g. the Flutter Driver), this is insufficient to test any
situation involving multiple runtimes. This means that it is important for
Fuchsia to supply appropriate APIs for creating fake input. This is accomplished
via SL4F and the Fuchsia input synthesis library, which inserts events into the
input pipeline via a dedicated injection API. This API should only be available
in developer builds and never in production builds as it allows injection of
arbitrary input.

### Input verticals {#verticals}
Support for each different type of input device adds significant complexity
beyond the high-level architecture described here. These details will be
addressed in subsequent RFCs. Major input verticals include:

* Physical keyboards (both Bluetooth and USB)
* Virtual or on screen keyboards
* Mouse
* Touch
* Trackpads

## Documentation
The contents of this RFC should be added to the Fuchsia public documentation
along with implementation details.

## Alternatives considered

This section includes a subset of the (extensive) alternatives discussed.

### Evolve root presenter into a pipeline

Historically, Scenic was responsible for dispatching all user input events,
including keyboard events which have no graphical/location component. This is
currently used in some product configurations, although keyboard events have
been removed. The existing input handling code in [root
presenter][root-presenter] could be extended to handle additional
use-cases. However, this code lacks test coverage and would require a
substantial re-write to give the desired properties of consistency and
configurability, and to remove unnecessary coupling between input handling and
the graphics API.

### Product-specific Scenic
Because input (especially pointer-based input) is intimately related to
graphics, one option explore would have routed input processing through
[Scenic][scenic], the Fuchsia graphics engine. This architecture would be a
radical departure from the current state. In this version, the compositor is
factored out of Scenic and functions in "immediate mode" meaning it must draw
any time the window manager makes a change. Scenic becomes the window-manager
which is assumed to be a product-specific component that will require a
different implementation for each product category. Input is routed through this
component.

While this would behave well in any single product, it would require any input
customization for an individual product to be baked into the graphics
engine. This could mean that a specialized implementation of Scenic is required
for each new product type. This approach might be a valuable optimization in the
future but was deemed too heavyweight for the current use-cases.

[HID]: http://www.freebsddiary.org/APC/usb_hid_usages.php
[views]: /docs/development/graphics/scenic/concepts/view_ref.md
[scenic]: /docs/concepts/ui/scenic/index.md
[scenegraph]: /docs/concepts/ui/scenic/index.md#scenes
[viewref]: /docs/development/graphics/scenic/concepts/view_ref.md
[focuschain]: /docs/development/graphics/scenic/concepts/focus_chain.md
[latencyreference]: https://www-user.tu-chemnitz.de/~attig/Attig-Rauh-Franke-Krems_2017_LatencyGuidelines.pdf
[inputmethod]: https://en.wikipedia.org/wiki/Input_method
[i18n]: /docs/development/internationalization/README.md
[root-presenter]: /src/ui/bin/root_presenter/presentation.cc
[input-pipeline]: /src/ui/bin/input-pipeline/
[drivers]: /docs/development/drivers/concepts/driver_architectures/input_drivers/input.md
[input-report]: https://fuchsia.dev/reference/fidl/fuchsia.input.report
[glossary.InputEvent]: /docs/glossary/README.md#inputevent
[glossary.InputHandler]: /docs/glossary/README.md#inputhandler
[glossary.session-component]: /docs/glossary#session-component
[output-report]: https://fuchsia.dev/reference/fidl/fuchsia.input.report#fuchsia.input.report/InputDevice.SendOutputReport
[input-roadmap]: /docs/contribute/roadmap/2020/overview.md#implementing_accessibility_and_input_improvements
[config-roadmap]: /docs/contribute/roadmap/2021/structured_configuration.md
[event-pair]: /docs/reference/kernel_objects/eventpair.md
