<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0147" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC provides an overview of the Fuchsia View System: the set of APIs to
reason about and interact with visual regions ("Views") and their
lifecycle. This set of functionality is also commonly referred to on other
platforms as a windowing system. The scope of this RFC is limited to products
with a single display. Changes to accommodate headless devices and/or multiple
displays will be covered in future RFCs.

A View is a region of graphical content, and is the basic unit of graphics and
user interaction on Fuchsia. Views connect to form a View tree hierarchy, with a
distinguished root View. Fuchsia's graphics compositor, [Scenic][scenic],
renders graphical contents from each View in the View tree to produce output for
the display. Scenic and the [input pipeline][input-rfc] are responsible for
routing UI-targeted user input such as keyboard, mouse and touch to the correct
View.

The View System is a critical part of the Fuchsia platform's support for ["bring
your own runtime"][byor] and the mechanism through which product developers use
graphical content from multiple runtimes to build secure visual user experiences
on Fuchsia. Fuchsia's composition APIs (Flatland and GFX) are built on top of
the View System.

## Motivation

The goal of this RFC is to document and ratify the "state of the world" with
respect to the View System. Specifically, we'd like to ratify the following
decisions.

1. Components that wish to display graphics on Fuchsia must create a View using
   the View System. The only exception to this is single-window utility UIs
   such as Virtcon and the Recovery UI, which do not use windowing features and
   are designed run in resource-limited situations when Scenic is not
   running. These UIs talk directly to the display controller.
1. The View System is the only way for components to integrate with the Fuchsia
   platform's support for user touch, mouse, and keyboard and accessibility
   services.
1. The View System is agnostic about the composition strategy being used (GFX
   vs. Flatland), and provides a shared foundation for all compositor
   implementations on Fuchsia.

Modifying these decisions requires an additional RFC and/or an update to this
RFC.

Note: The the current Fuchsia implementation does contain exceptions to these
statements, because it is a work in progress. For example, there is a legacy API
that allows global access to pointer events. This RFC is a declaration of our
intent to remove these exceptions.

## Stakeholders

_Facilitator:_

hjfreyer@google.com

_Reviewers:_

This section expected to be updated during the review

* Graphics: reveman@google.com, jjosh@google.com, emircan@google.com
* Input: quiche@google.com
* Components: ypomortsev@google.com
* Security: kostyak@google.com
* General: wez@google.com

_Consulted:_

sanjayc@google.com, hjfreyer@google.com, jjosh@google.com, lindkvist@google.com,
geb@google.com

_Socialization:_

Draft doc was send to the Scenic and Input teams for discussion.

## Glossary

* [Scenic][scenic]
  * The graphical [compositing][compositor] component in the Fuchsia platform
  * The sole implementer of the View System APIs, including its tie-ins to the
    graphical composition APIs and HCI APIs.
* View
  * A visual region of graphical content.
  * It has a coordinate system, a bounding box, and a defined spatial
    relationship to its ancestors, via the View Tree.
* View Tree
  * The structure of Views on the system, connected by parent-child
    relationship. The root View of the View Tree is typically attached to the
    display.
* Display
  * A pixel-based output device; also called a screen. The View System
    currently supports one screen attached to the system at a time.
  * The screen is managed through a display controller.
* Display controller
  * Driver that manages screens attached to the computer.
  * Only talks to Scenic.
  * Is not available to out of tree clients.
* Scene Manager
  * A platform component that attaches the root View to the screen.
* Accessibility Manager
  * A platform component that implements accessibility APIs.
  * It has privileged access to the View System.
* System shell ("Sys UI", "System UI")
  * A component responsible for a product's user experience, for example
    Ermine. This component is typically specific to a particular product.
  * A system shell is responsible for [window management][window-manager],
    including focus handling and management of top-level Views, while the View
    system handles the [windowing system][window-system].
* Developer shell ("tiles", "present_view")
  * A system shell used for testing. Typically this allows a developer to
    launch a Fuchsia component in isolation.
* UI client
  * This is a generic way of referring to the owner of a View, used throughout
    this RFC.
  * Examples: A flutter app, chromium with multiple top-level Views.

## Design

### Definition of a View

A Fuchsia View is the basic unit of graphics and interaction on Fuchsia. A View
defines a visual region for displaying graphical content to the user. Not all
Views are visible at a given time. The graphical contents for each View are
supplied by a Fuchsia [component][component]. Platform support for managing
Views and compositing their contents onto the screen is implemented by Scenic.

Each View:

* Can incorporate the contents of another View (its child View), forming a *View
  tree*.
* Defines a coordinate system for placing child content.
* Has a bounding box that defines the visible (and optionally interactive) part
  of that View.
* May be connected and disconnected from the Scenic View Tree. (Unconnected
  Views may exist, but do not have their content rendered to the screen, nor do
  they receive input.)

Any graphical composition API supported by Scenic (such as
[fuchsia.ui.composition][flatland] and [fuchsia.ui.scenic][gfx]) must provide
methods to create a View and manage its lifecycle. Any future composition APIs
must also operate on top of the View System. Fuchsia platform HCI APIs (such as
[pointer](/sdk/fidl/fuchsia.ui.pointer) and
[keyboard](/sdk/fidl/fuchsia.ui.input3/keyboard.fidl)) use Views to route
input. (See [Input](#input) below for details.)

### View Tree {#view-tree}

There is one global View Tree, which is owned and implemented by Scenic. The
View Tree has a distinguished root View that connects to the screen. Scenic is
the only Fuchsia component that directly manipulates all Views and their
positions.

Each View in a View Tree has bounds defined in its own coordinate system, and a
position and orientation expressed in the parent View's coordinate
system. Together, these determine the regions where graphical content is
ultimately visible on-screen, and regions which are responsive to user input.

#### Child Views

A View can embed additional graphical content from another View by creating an
empty placeholder for it in its coordinate system; the placeholder is called a
[viewport](/sdk/fidl/fuchsia.ui.composition/flatland.fidl). The two Views form a
parent-child relationship in the View Tree, where the parent View's Viewport
embeds a child View's graphical content.  To establish the relationship, the
parent and child must provide matching
[tokens](/sdk/fidl/fuchsia.ui.views/flatland_tokens.fidl) when creating their
View and Viewport. These tokens are implemented as kernel objects, and are not
clonable. The parent and child may obtain these matching tokens in
a variety of ways, which are external to the View System.

Attaching or detaching a parent View from the View Tree also attaches/detaches
its child View(s). Connections between Views within a subtree remain even when a
subtree is detached from the global View Tree.

Note: The View System provides parent Views with the ability to restrict where
their descendent Views can render or receive user input ("clipping") for
security reasons (for example, preventing clickjacking).

#### View Isolation

While a View can embed another View, the View System does not give a View access
to the graphical content of any other View. This isolation guarantee forms one
of the bases of [View security](#view-security).

### Scenic maps Views to the screen

Scenic is the sole component that talks to the display controller. It takes the
graphical content of the entire View Tree, along with the relationships encoded
in the tree, creates a single image (possibly with multiple layers). It then
programs the hardware to display the final on-screen image.

### View ownership

The contents in a View are supplied by a single FIDL channel to a graphical
composition API, which is implemented by Scenic. Each client endpoint of a FIDL
channel to Scenic is termed a "UI client", and a UI client can create at most
one View.

Views may come from a variety of components including user-facing components
(e.g. a browser), the system UI, as well as components that are part of the
Fuchsia platform, such as the Accessibility Manager.  A component may create
multiple channels and thus multiple Views. Consequently, a single component may
vend both a parent View and that View's child View in some cases.

In order to combine graphics from multiple components, developers should create
a View or Views in each component and use child View and the View Tree to
combine them. It is important to note that the [View Tree hierarchy](#view-tree)
does not need to match the component instance hierarchy. In many cases, these
structures intentionally look quite different.

#### Runtimes

For components written in a higher level language (like Dart or Javascript) the
runner implementation is typically responsible for creating and managing the
View. Developers working in these languages may be unaware of many of the
details of the underlying OS; it is the runner's responsibility to translate the
View lifecycle into language-specific or runtime-specific mechanisms.

### Window Management vs. Window System vs. Composition

Fuchsia separates three functions that are sometimes combined on other
platforms.

* [Window management][window-manager] incorporates product-specific policy and
  specific choices about how windows behave. On Fuchsia, this is the
  responsibility of the system UI and lives entirely outside of the platform.
* The [windowing system][window-system] refers to low-level window
  management. On Fuchsia this is handled by the View System.
* [Composition][compositor] refers to combining images from multiple sources to
  produce images for display. This is currently also a responsibility of the
  Scenic component, although this could change in the future.

The choice to keep the windowing system and composition separate provides a
fast-path for graphical presentation and allows for an efficient
implementation. While both the View System and the compositor(s) are currently
implemented primarily within Scenic, they are separate both in terms of APIs and
code. This separation makes it possible for the View System to support multiple
composition strategies (Flatland and GFX).

By making window management a product-level concern we keep policy logic outside
of the platform, ensuring a clean separation between platform and product.

### Input {#input}

The View System is the primary mechanism through which the Fuchsia platform
determines how to route user input, such as pointer events or keyboard
events. Fuchsia's user input APIs are designed such that each channel is scoped
to a particular View. Input will be routed to a View based on either the current
View focus (in the case of keyboard events) and/or to the View or Views
corresponding to the location of the input event (as in the case of touch or
mouse events). A View may _only_ receive user input when conntected to the View
Tree.

Multiple Views may participate in input processing for the same
event. The window manager for a particular product may configure this routing
based on product policy, for example by granting global access to mouse events
to the system UI. For additional details see [User Input
Architecture][input-rfc]. This system allows seamless user experiences across
Views from multiple runtimes, including the ability to [disambiguate touch
gestures](https://www.tdcommons.org/dpubs_series/3872/).

One important implication of this is that components outside of the Fuchsia
platform do not get direct access to input events from the driver. They must
receive this information mediated by the View System.


#### View Focus

At any given time the View Tree has one distinguished View, called the *focused*
View. The focused View is typically the View that the user expects will receive
user input. Fuchsia's input subsystems rely on View focus as a means of
determining where to route input. For additional information see the [Focus
Chain documentation][focus_chain]. A parent View owner may control View focus
within its subtree.

#### ViewRef

The platform uses tokens called
[ViewRefs][viewref] to identify and communicate
about Views. A ViewRef is a unique reference to a particular View that remains
unique until system reboot. It is implemented as a kernel object, the handle
of which can be freely duplicated and sent to other components over arbitrary
protocols.

The View System APIs make heavy use of ViewRefs to provides a stable
cross-component reference for each View. ViewRefs are also used to signal
lifecycle events about the associated View. This allows other components inside
and outside of the platform to communicate about Views, their lifecycles, and to
route user input and accessibility protocols to Views.

Note: ViewRefs are distinct from ViewTokens, ViewHolderTokens,
ViewCreationTokens, etc. View*Tokens are used to associate parent and child
Views during View creation and installation.

## Implementation

Scenic is the source of authority for the View System because the View Tree and
core View management APIs are implemented in Scenic. Each client uses its
ViewRef to register to receive user input and accessibility events for the
associated View. Scenic currently supports two graphical composition APIs:
fuchsia.ui.scenic (legacy) and fuchsia.ui.composition (in development). The View
System is independent of, but works in close conjunction, with each.

API implementation details are out of scope for this RFC but the relevant APIs
can be found below. It's worth noting that these APIs evolved incrementally over
multiple years. Some aspects may be simplified in the future.

### Platform components involved in the View System

* Scenic
* Input Pipeline
* Scene Manager (or Root Presenter on legacy systems)
* Accessibility Manager
* Text Manager

###  APIs for View management

* [fuchsia.ui.views](/sdk/fidl/fuchsia.ui.views)
  * View refs, View tokens definitions
  * View focus management
* [fuchsia.ui.composition](/sdk/fidl/fuchsia.ui.composition)
  * Create a View, move a View, resize a View, connect/disconnect a View
* [fuchsia.ui.policy](/sdk/fidl/fuchsia.ui.policy)
  * Connect System UI View to root View
* [fuchsia.ui.app.ViewProvider](/sdk/fidl/fuchsia.ui.app/view_provider.fidl)
  * Surface View creation from a component
* [fuchsia.web.Frame](/sdk/fidl/fuchsia.web/frame.fidl)
  * Create web client Views

### APIs for user input

* [fuchsia.ui.pointer](/sdk/fidl/fuchsia.ui.pointer)
  * Pointer events
* [fuchsia.ui.input3](/sdk/fidl/fuchsia.ui.input3)
  * Keyboard input
* [fuchsia.ui.shortcut](/sdk/fidl/fuchsia.ui.shortcut)
  * Keyboard shortcuts

### APIs for accessibility

* [fuchsia.accessibility.semantics](/sdk/fidl/fuchsia.accessibility.semantics)
  * Allows a View to send semantic information for accessibility
* [fuchsia.ui.annotation](/sdk/fidl/fuchsia.ui.annotation)
  * Allows accessibility to highlight the focused UI component

## Performance
Many aspects of the View System influence graphics and input performance. We
describe some important aspects below.

* *All runtimes are equal*: Fuchsia's principle of [bring your own
  runtime][byor] means that all runtimes on Fuchsia use the same event
  routing, and no runtime gets preferential treatment.
* *Render once*: Scenic makes it possible to combine graphical content for
  different Views without necessarily re-rendering, because it knows view
  positions and can correctly forward graphical content to the display driver.
* *Efficient dispatch*: Scenic exposes focus information, allowing non-graphical
  user input (e.g. keyboard events) to be dispatched directly from the
  component(s) involved in processing it.

## Security and Privacy considerations {#view-security}

The View System makes several guarantees for securely composing graphical
content on Fuchsia, and these guarantees form the foundation for a secure
UX. However, the View System alone cannot guarantee that every UX is secure -
only that the View System respects its own security guarantees.

The View System attempts to enable secure UX by:

* Delegating sole responsibility for owning and manipulating the View Tree to
  Scenic.
* Delegating sole responsibility for displaying graphical content on the screen
  to Scenic.
* Disallowing UI clients from manipulating another View without a matching
  Viewport or ViewRef.
* Disallowing UI clients from using the View System to inspect the contents of
  any other UI clients' Views.
* Only allowing a UI client to inject input into its View subtree.
* Only allowing a View to gain focus and receive input when connected to the
  view tree.

UI security relies on the guarantees provided by Component Framework with
respect to capability routing. The details of View security will be addressed in
future RFCs.

## Testing

The View System is implemented at multiple layers of abstraction, so it is
tested using a layered approach.

Within fuchsia.git:

* Unit tests of the View Tree and View lifecycle, in the Scenic component's
  codebase.
* UI Integration tests, in [/src/ui/tests](/src/ui/tests), that exercise the
  contracts around View System APIs.
  * These APIs are vended and used by platform components like Scenic, Scene
    Manager, Input Pipeline, Accessibility Manager, and Text Manager.
* The fuchsia.ui.observation.test APIs are used within in-tree integration tests
  to inspect View System behavior.

Out of tree:

* Runtimes such as Chromium and Flutter write integration tests to exercise View
  System APIs
* Products write end-to-end tests that indirectly exercise the View System.

## Documentation

This RFC provides a high-level overview of the View System's role and
responsibilities. Docs and possibly additional RFCs will be written to address
details.

Some of the [Scenic documentation][scenic] will also need to
be updated in light of this RFC and upcoming changes for Flatland.

## Drawbacks, alternatives, and unknowns

### Known Limitations

The View System currently lacks a mechanism for synchronizing updates between
multiple Views.

The View System is partially responsible for routing capabilities to runtimes
that expose Views (using ViewRefs). This has some conceptual overlap with
capability routing via Fuchsia's Component Framework, but is currently entirely
separate. In the future, it may be advantageous to support View capabilities in
the Component Framework.

The current APIs only allow View owners to create child Views. This isn't ideal
for something like an application that wishes to create multiple top-level
windows.

Because the View System APIs evolved organically over multiple years with many
different contributors, the cognitive burden on runtime developers is currently
quite high. This should be mitigated in the future by better documentation and
examples, and possible API simplification.

### Alternatives

The View System architecture makes a number of architectural choices.

* The View System is required for any product with graphics.
  * Alternatively, Scenic could be optional and products could include their own
    compositor. This would require exposing many low-level APIs out of tree.
  * Advantages of current approach:
    * Homogeneous behavior (including accessibility and input) across runtimes
    * Consistent security guarantees
    * Display API can evolve without breaking clients
  * Disadvantages of current approach:
    * Less customization for products
    * Complexity for runtime integrations
  * This could change in the future _if needed by a product building on
    Fuchsia._
* The View System is distributed across several components including Scenic and
  the Input Pipeline.
  * Alternatively, we could have built the entire View System into Scenic.
  * Advantages of current approach:
    * Separation of concerns: graphical
composition, window management, product policy are all separate.
    * Allows high-performance graphics path via delegated composition
    * UI-related components can evolve independently
    * Allows graphics to keep working even if a sub-system (e.g. text input)
crashes
  * Cons of current approach:
    * Multiple processes can add latency with IPC
    * Added coordination/synchronization complexity between components

## Prior art and references

* The View System and Scenic share some architectural similarities with the [X
Windows System](https://en.wikipedia.org/wiki/X_Window_System).
* Other aspects have more in common with the [Wayland
protocol](https://wayland.freedesktop.org/architecture.html).
* The [input architecture RFC][input-rfc] describes details of how the View
System interacts with Fuchsia's user-input subsystems.

[input-rfc]: /contribute/governance/rfcs/0096_user_input_arch.md
[byor]: /contribute/governance/rfcs/0082_starnix.md
[component]: /concepts/components/v2/README.md
[scenic]: /concepts/ui/scenic/index.md
[viewref]: /development/graphics/scenic/concepts/view_ref.md
[flatland]: /sdk/fidl/fuchsia.ui.composition/flatland.fidl
[gfx]: /sdk/fidl/fuchsia.ui.scenic
[compositor]: https://en.wikipedia.org/wiki/Compositing_window_manager
[window-manager]: https://en.wikipedia.org/wiki/Window_manager
[window-system]: https://en.wikipedia.org/wiki/Windowing_system
[focus_chain]: /development/graphics/scenic/concepts/focus_chain.md
