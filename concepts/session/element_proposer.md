# Element Proposer

An Element Proposer is a _component role_ within the
[session][doc-session] describing a component that requests
[elements][doc-element] to be added to the session through the
[fuchsia.element.Manager][sdk-element-manager] API.

## Element Proposers are components {#proposers-are-components}

Element Proposers are [components][doc-component] within a
[session][doc-session]. They are distinct from other components within the same
session by their role:

> Element Proposers use the [fuchsia.element.Manager][sdk-element-manager]
> protocol to add [elements][doc-element] to the session.

## Element Proposers drive product UI {#proposers-drive-product-ui}

Element Proposers can react to environmental inputs, including the following:

* Accepting direct keyboard input from the user
* Listening to requests from a mobile device on the same network
* Using predictive algorithms based on past user behavior

They then respond to these inputs by adding elements to the session.

A fully featured product contains multiple Element Proposers.

## Element Proposers are portable across products {#proposers-are-portable}

Like the elements they propose, Element Proposers are reusable across products
and product configurations. Including an Element Proposer is accomplished by
including it in the [component topology][doc-component-topology] for the
session.

[doc-session]: /concepts/session/introduction.md
[doc-element]: /concepts/session/element.md
[doc-component]: /concepts/components/v2/introduction.md
[doc-component-topology]: /concepts/components/v2/topology.md
[sdk-element-manager]: /sdk/fidl/fuchsia.element/element_manager.fidl