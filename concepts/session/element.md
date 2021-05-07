# Elements {#elements}

*tl;dr:* Elements are components added to a session as a result of user
interaction, similar to "apps" on other platforms.

This document provides a brief conceptual overview of elements, element
annotations, and their relationship to [components][doc-component].

## Elements are components {#elements-are-components}

Elements are [components][doc-component] with key-value annotations. While any
component could hypothetically be instantiated as an element, elements
[contribute more directly](#element-ui) to the user experience of a product,
much like an "app" would on other platforms.

Since elements are components, they inherit all component properties, such as:

* Being identifiable by their component URLs, and having
[component manifests][doc-component-manifest].
* Participating in [capability routing][doc-capability-routing].

Elements are typically instantiated in [component
collections][doc-component-collection] within the session component.

## Elements provide a UI {#element-ui}

Elements contribute to the user experience of a product by providing a visual,
audible, or other discernible interaction with the user. The specific modes of
interaction available to, and expected from, an element, are defined by the
product and its configuration. The most common mode of output is graphical.

## Elements have annotations {#element-annotations}

Every element has an associated list of mutable key-value annotations. These
annotations are first set when the element instance is added to a product
experience at runtime. Element annotations are used by the product's "user
experience presenter" to control details of the element's presentation. For
example, a for graphical element's annotations may specify if the presentation
should hide system chrome, allow for certain types of gestures, and so on.

## Elements are portable across products {#elements-are-portable-across-products}

Elements are an abstraction intended to make it easier to interactively run
components on many different Fuchsia products and varying product
configurations. An element's primary interaction surface with a product are
defined by:

* The mechanism by which it presents itself (graphically using Scenic, through
  audio, and so on).
* The mechanism by which it receives user input (finger touches, gestures,
  keyboard, and so on).
* The semantic meaning associated with its various annotations.

Although none of these mechanisms are strictly defined by the Fuchsia platform,
some are standardized and intended to be re-used across all products built with
Fuchsia.

While the specific meanings of different annotation keys and their
respective values are not prescribed by the Fuchsia platform, some keys do
have recommended meanings with recommended standard schemas.

## Further reading {#further-reading}

* [fuchsia.element.Spec][sdk-element-spec]: a FIDL table that describes an element to add to a session.
* [Graphical Presenter][sdk-graphical-presenter]: a component role 
  for presenting elements using Fuchsia's graphical compositor,
  [Scenic][doc-scenic].

[doc-component]: /docs/concepts/components/v2/introduction.md
[doc-component-manifest]: /docs/concepts/components/v2/component_manifests.md
[doc-capability-routing]: /docs/concepts/components/v2/topology.md#capability-routing
[doc-scenic]: /docs/concepts/graphics/scenic/scenic.md
[doc-component-collection]: /docs/concepts/components/v2/realms.md#collections
[doc-graphical-presenter]: /docs/concepts/session/graphical_presenter.md
[sdk-graphical-presenter]: /sdk/fidl/fuchsia.element/graphical_presenter.fidl
[sdk-element-spec]: /sdk/fidl/fuchsia.element/element_manager.fidl
