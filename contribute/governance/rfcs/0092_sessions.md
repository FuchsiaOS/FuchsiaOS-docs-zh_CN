{% set rfcid = "RFC-0092" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }} - {{ rfc.title }}
<!-- *** DO NOT EDIT ABOVE THIS LINE -->

## Summary

This document describes the role and responsibilities of a Fuchsia session.

A session is a component. Each Fuchsia product builds its user experience within
a session. For example, the workstation session instantiates components to
handle input, set up the Scenic scene graph, and to render its UI.

The session component is offered all capabilities required to create the user
experience of a product, and is thus often the most privileged non-platform
component in a product. As such, the session component serves as a boundary
between the Fuchsia platform and a product built with Fuchsia. The
`session_manager` is the parent component of the session, and exposes debug
protocols that developers can use to control the session. Product variants
(e.g., `product_eng`, `product_userdebug`) may result in different flavors of
the same session.

The intention of this document is to ratify design decisions that have been made
since the `Modular Framework` was deprecated.

## Motivation

The Fuchsia platform aims to support many products. These products are composed
of product-specific and platform components. The product-specific components
must live somewhere in the component instance tree. The session is the root of
the product-specific component instance tree that drives the user experience.

The session serves as a boundary between the Fuchsia platform and the product.
As such, the session can be used to improve the product development cycle. For
example, the session can be destroyed and recreated to "restart" the product's
user experience.

## Design

This section outlines the current design for Fuchsia sessions, and highlights
where the current design is not the intended long-term design. The alternatives
section describes some potential long-term approaches at a high level.

### Startup

`core` is a non-executable component that provisions capabilities that are
consistent across products. `core` is the parent of `session_manager`.
`session_manager` is instantiated in response to `startup` connecting to
the `fuchsia.sessionmanager/Startup` protocol exposed by `session_manager`.

`session_manager` reads the initial boot session from a configuration file
located in its `/config/data`. It then instantiates the session as a child in a
dynamic `collection`. Although the session lives in a collection, at most one
session component exists in this collection. This means that all sessions
receive the same set of capabilities, as the set of offered capabilities doesn't
change from product to product. See [Session Manager
Configuration](#session-manager-configuration) for further discussion.

The session then instantiates its own, product-specific, component hierarchy.
Even for simple products, the session ends up with a lot of responsibilities.
For example, it configures the scene graph, wires up input, integrates
accessibility, and so on.

### Capabilities and components

The session uses the capabilities it is offered to instantiate a tree of
components required for the desired product experience. The session must be
offered all platform capabilities used within the session's component hierarchy.

The Fuchsia platform must consider the capabilities offered to the session
carefully even when the capabilities are specific to a single class of products.
The goal is to provide products the control and flexibility they need while
maintaining a trusted platform that is consistent across products.

For example, consider products with displays. The Fuchsia platform may decide to
offer the session the capability to interact with the display and then let the
session be responsible for instantiating a graphics subsystem (e.g., Scenic).
This gives the session a lot of flexibility and control, but it also makes it
more difficult for the Fuchsia platform to evolve: if the Fuchsia platform
offers the session the capabilities it needs to instantiate Scenic, it also
gives the session the ability to create its own Scenic replacement. If enough
products define their own graphics protocols, a "graphical component" will only
run on systems with the matching graphical protocols. In addition, the platform
can no longer provide guarantees about the scene graph, since any product has
the ability to circumvent them.

Scenic is currently instantiated as a "v1" component under `appmgr`. When Scenic
is migrated to "v2" (`component_manager`) component hierarchy, a decision will
need to be made about where in the component hierarchy it is instantiated.

To determine whether or not a component should be instantiated within the
session, consider the question: "should interaction with this capability be
consistent across products"? For example, a product should be able to choose
whether or not it needs graphics, but if it needs graphics it should use Scenic.
Similarly, a product should be able to decide which types of input devices it
supports, but it should use the input pipeline to receive events from said
devices. This implies that both Scenic and the input pipeline should be product
configurable but be instantiated outside the session, because the capabilities
those components require are privileged.

In the current architecture some of these components are planned to live within
the session for practical reasons (e.g., lifecycle management, lack of `.cml`
configurability, etc.). There are several ongoing efforts across Fuchsia which
introduce other mechanisms for platform configurability that will allow many of
these platform components to be instantiated outside the session again:

  * Drivers are already instantiated outside of the session yet are not static
    across all Fuchsia products.
  * Features defined in the core realm need to be toggled depending on security
    context, like enabling component instance ID enforcement for storage on user
    builds.
  * The temperature-logger component exists in the core realm today, but is only
    included on certain boards via a mechanism defined in [RFC-0089][rfc_0089].

Some examples of components that should clearly be instantiated within the
session:

  * Graphical presentation ("shell") components.
  * Components that contribute directly to the user experience (e.g., video
    player, terminal, etc.).

## Implementation

The `session_manager` component has been implemented and can be found in the
[session_manager][session_manager] directory.

The workstation product contains a session which can be found in the
[experiences][experiences] repository.

Other simple example sessions can be found in [examples][examples].

## Performance

The session is the first product owned component that gets instantiated, but
many platform components are instantiated before the session component.

## Security considerations

The `session_manager` offers the session all the capabilities it needs to
instantiate a product's user experience. All sessions receive the same set of
capabilities from `session_manager`. This set contains a wide range of
capabilities. However, the set of capabilities available to the session is
smaller, and more auditable, than the set that is offered to components that run
on product configurations that have not been migrated to use a session (i.e.,
products that use the `Modular Framework`).

It is important that the capabilities offered to the session go through security
review. It unlikely that the Fuchsia team will be able to perform thorough
security reviews of all sessions in the future. Thus the security of the
session, from the platform's perspective, relies heavily on the set of
capabilities offered to the session.

## Privacy considerations

`session_manager` only stores the URL of the current session, which may be
logged. This is an improvement over the `Modular Framework, which
manages components that are launched by the user.

Each session will require its own comprehensive privacy review.

## Testing

Product owners need tools to test and debug their sessions.

A testing framework that focuses on the needs of session developers will be
created. This framework will allow developers to, for example, test the setup of
their session's component hierarchy and verify the interactions between the
components in the session.

Developers will be able to interact with a running session via `ffx` commands.
For example, `ffx session restart` instructs the `session_manager` to destroy
and recreate the session.

## Documentation

Per [RFC-194][rfc_0194], conceptual documentation has been removed.

Each individual binary and library in `//src/session` also contains a README
explaining its purpose.

## Drawbacks, alternatives, and unknowns

### The platform as a child of the session

The relationship between the session and `core` could be reversed, such that
`core` is a child of the session. This would give the product full control over
how the system is configured at the cost of a stable platform foundation.
### Session manager configuration

The `session_manager` configuration currently only toggles which session
component to instantiate during boot. There is no variance in
`session_manager.cml`, and thus every session is offered the same set of
capabilities. Ideally the session manager configuration would minimize the
number of offered protocols.

## Prior art and references

[examples]: /src/session/examples
[experiences]: /src/experiences/session_shells/ermine/session
[session_manager]: /src/session/bin/session_manager
[rfc_0089]: /docs/contribute/governance/rfcs/0089_core_realm_variations.md
[rfc_0194]: /docs/contribute/governance/rfcs/0194_addendum_sessions.md