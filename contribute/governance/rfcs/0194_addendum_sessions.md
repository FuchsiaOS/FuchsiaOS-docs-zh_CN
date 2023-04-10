<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0194" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

> NOTE: This RFC is an addendum to RFC-0092 and ratifies decisions that were
> left open in RFC-0092.

This document:

  * Deprecates the term "Session Framework"
  * Describes the current state of the Fuchsia platform as it relates
    to the legacy "Session Framework" concept
  * Defines the "session component" and its role in the Fuchsia platform
  * Begins to codify the role of Fuchsia Security in the review of capabilities
    routed to product components

From the legacy "Session Framework" concept, the following are retained:

  * The platform `session_manager` component
  * The "session component" itself as a child of `session_manager`
  * The "session component"'s role in the platform as a security boundary
  * The `ffx session` developer tools
  * The `fuchsia.element.GraphicalPresenter` protocol
  * The `fuchsia.element.Manager` protocol

The following are discarded or have their ownership transferred to specific
product owners:

  * The concept of a-la-carte platform-owned libraries/components for other
    processes, such as input pipeline management
  * The goal of solving the complexity explosion when crossing many runtimes
    (such as Flutter, Android, Chrome, etc) with many products under a _single
    umbrella under a single team_

The following are retained and continue to be owned by the platform but have
planned deprecations:

  * `fuchsia.element.Manager` protocol
  * `element_manager` component

The following open questions from the original RFC-0092 are answered:

  * What graphical capabilities are provided from the platform to the session?
  * What input capabilities are provided?
  * What capability classes are sensitive and will never be provided to a
    session?

## Motivation

When session components were introduced as a concept in RFC-0092, questions
about platform-provided capabilities to the session were largely open. Since
then, Fuchsia platform designers have learned enough to provide detail about
what capabilities are provided to all sessions and, importantly, what types of
capabilities will never be provided.

In its role as one of the significant security boundaries between platform
software and product software, the choice of which capabilities are provided to
the session component is of high importance to the Fuchsia platform designers.
Providing an overly broad set of capabilities to all sessions can lock the
platform out of providing key consistencies in behavior across all products.

Lastly, the legacy "Session Framework" concept has caused some developer
confusion. The concepts, some of which were not explicitly included in RFC-0092
but were in the Session Framework concept docs, originally included:

  * Providing client libraries and reusable components for use inside
    a session to solve a myriad of problems, including:
      - Window management
      - End-user app lifecycle
      - Input handling
      - Notifications
      - ... and so on
  * The definition of component roles and responsibilities, such as Element
    Proposer, for use inside a session.
  * Introduction of the "Element" concept as a new first-class usage of a
    component.

Due to the open-ended scope of "Session Framework the project", the term also
became open-ended and ambiguous.

## Stakeholders

_Facilitator_:

hjfreyer@google.com

_Reviewers_:

This section to be updated during the review.

Graphics: emircan@google.com, dworsham@google.com
Input: neelsa@google.com
Product: yaar@google.com
User Data Protection: jsankey@google.com
Identity: jsankey@google.com
Workstation: sanjayc@google.com

_Consulted_:

neelsa@google.com

_Socialization_:

Draft doc was sent to the Scenic, Input, Workstation, Trusted Platform Services
teams for discussion.

## Design

A "session component" is a component. Each Fuchsia product encapsulates its
user experience within a session component (and its descendent components). The
term "user experience" is used broadly: it encompasses traditional graphical
experiences with touch/mouse/keyboard interaction, as well as simpler
experiences with a single RGB LED and several hardware buttons, or those with
only network I/O.

Simple products may specify a session component with no children, while complex
products have many children of the session component.  For example, the
workstation session instantiates components to display the system user
interface, a command line shell, and a web browser as descendents of its session
component.

Today, a single session component is present in the component topology of any
Fuchsia product. In the future, multiple sessions running concurrently is
expected. The identity (URL) of the session component is chosen by the product
owner. The position within the topology is defined by the platform and cannot
be changed by product owners. Due to the nature of the component architecture,
the session component cannot learn of its position within the topology.

The session component is offered all platform-provided capabilities needed to
create the user experience of that product. The session component is one of the
most privileged non-platform components in the component topology.  For this
reason, the definition of the boundary between the platform and the session
component serves as an important security and control layer for Fuchsia.

### Startup & topology

The session component is a child of `session_manager.cml`, which is a child of
`core.cml`. `session_manager.cml` is an intermediary platform binary that
enables key behaviors:

* The URL of the session component is configurable without editing
  `core.cml` or any other platform `.cml` file
* On some products, the session component is started automatically
  on boot (by convention, this is used on user and userdebug builds)
* On others, the boot process can be instructed to interrupt before starting
  the session component by configuring an empty session URL for
  `session_manager` (by convention, this is used on "paused" variants of eng
  builds)
* On eng builds, the session component can be restarted or replaced by a
  different component at runtime

`session_manager.cml` configures the _definitive upper bound of capabilities
that can pass from the platform to the product_.

### Capabilities

Like any component, the parent of the session component defines the session
component's sandbox. Below is an incomplete list of the superset of
capabilities that are available to the session component from the platform.
Depending on the product's configuration of the platform, some capabilities
will not be available at runtime.

Product configuration, and thus the set of capabilites actually available to
a given session component on a given product, is accomplished today through
adding or removing build-time package dependencies on a set of 
[product package labels](/docs/get-started/learn/build/product-packages.md).

#### System lifecycle control

The session component can:

* Instruct the platform to restart the session component.
* Request that device hardware reboot or suspend.
* Initiate a device factory reset.

#### Graphics

For products that have a graphical user interface:

The session component is offered the necessary capabilites to specify a single
[View](/docs/concepts/ui/scenic/views.md#view) to act as the root View of
the user experience. The choice of View can change over a session's lifecycle.
For example, when interaction with a session is locked due to inactivity, the
root view may swap to a lock screen.

Additionally, the session can
[embed sub-views](/docs/concepts/ui/scenic/views.md#viewport) in its root
View for the purposes of delegating to additional software. The ability to
embed sub-views is not unique to sessions: it is a property of Fuchsia's
[system compositor](/docs/concepts/ui/scenic/index.md).

Sessions are not _required_ to specify a View. For example: headless sessions
running on headless devices would not specify a View.

The following sensitive capabilities are explicitly *not* provided to sessions:

* Low-level control over display hardware. Fuchsia ensures that UI composition
  happens through the system compositor that provides a minimum level of
  performance and features (such as accessibility overlays), across all
  products.

#### Input

The session component is offered capabilities that allow it to observe input
events. They include routing of key events to focused Views in a View
hierarchy, routing of mouse and touch events, and the ability to register for
notification of keyboard shortcuts. Additional input capabilities may be
required in the future to support interaction with other input devices, both
virtual and physical.

The following sensitive capabilities are explicitly *not* provided to sessions:

* Low-level access to HID input events.

#### System settings

The session can access and manage system setting such as the software update channel and active WiFi network, including:

* Input policy, such as keyboard keymaps and auto-repeat rates
* Display policy, such as light sensor modes
* Internationalization
* Software update channel
* Hardware settings, such as WLAN (WiFi), Bluetooth, or Camera settings.

#### Persistent data

The session is provided with the necessary capabilities to supply encrypted
storage to components inside the session. These include encrypted device
storage capabilities and the [account management](/sdk/fidl/fuchsia.identity.account/account_manager.fidl)
service. The account management service may be used to perform authentication
and access account encrypted storage.

## Implementation

### Update documentation

* Delete the Session Framework concept documentation.
* Add "session component" to the glossary.

### Restrict session capability routing

The `session_manager.cml` component contains the definitive list of capabilities
offered to sessions. Select capabilities that pose a security or privacy risk
will result in a platform build failure if included in `session_manager.cml`.

The list of disallowed capabilities will be generated and maintained in
collaboration with the Fuchsia Security team. Candidates include:

* `fuchsia.hardware.display.controller.*`
* `/dev/class/input-report`

## Security considerations

### Overview

The capabilities the platform delivers to the session component constrain the
session component's abilities. This has important implications: a session
component that is granted the ability to launch third party software as a child
of itself could learn a lot about those instances of the software. For example:
it owns persistent storage for its child instances and could be configured to
read from and write to that storage. It can also learn the software's identity
(URL), among other information.

### Auditing responsibilities

The platform `session_manager.cml` component configures the upper bound of all
capabilities possible to be granted from the platform to the session component.
It is the responsibility of the Fuchsia security team to audit this superset
and ensure it is suitable for all possible products. _Any changes to these
capabilities must go through a security review_.

Product owners are responsible for the security properties of capability
routing once on the product-side of the session component boundary: any routing
decisions made within the session component or its children are not visible
to the Fuchsia platform security team.

Some auditing and enforcement tools exist today. These include:

* [`fx scrutiny`](https://fuchsia.dev/reference/tools/fx/cmd/scrutiny)
* Routing allow-lists which can vary on a per-product basis

Improved auditing and enforcement tools are needed to support product-specific
security teams. It is the intent of the Component Framework team to improve
both `scrutiny` and routing allow-list mechanism for better recursion, making
it easier to apply the tools to component sub-topologies other than the
platform root. However, no specific plans are available at this time.

For now, Security team will be auto-CC'ed on CLs that change either
`session_manager.cml` or the `BUILD.gn` file responsible for compiling the
`.cml` file.

## Privacy considerations

The privacy implications of having a session component are similar to those for
security. While Fuchsia platform can issue guidelines and best practices to
session owners, it has no mechanism to enforce policy other than restricting
the capabilities (and their respective backing implemenations) provided to the
session component.

## Documentation

The existing Session Framework documentation will be updated or removed to align
with the content in this document.

## Drawbacks, alternatives, and unknowns

No alternatives aside from the default of "do nothing". The risk therein
includes the perpetuation of an outdated concept ("Session Framework") that is
causing confusion, and does not currently have a dedicated team to reduce the
ambiguity or solve technical issues.

## Prior art and references

* [RFC-0092 - Sessions](/docs/contribute/governance/rfcs/0092_sessions.md)
