## Component lifecycle

Component instances are created and destroyed when they are added and removed
in the component topology. This can happen in one of two ways:

* **Statically**: The instance is declared in the component manifest as a child
  of another component in the tree. Static components are only created and
  destroyed when an update changes the component topology.
* **Dynamically**: The instance is added or removed in a component `collection`
  at runtime using the `fuchsia.component.Realm` protocol. Dynamic components are
  destroyed on system shutdown.

Once a component is destroyed, the framework removes its persistent state
(such as local storage).

The framework starts a component instance when another component attempts to
open a channel to it — known as **binding**. Binding happens **implicitly** when
connecting to a capability exposed by the component. Binding to a component that
is already started connects to the currently running instance.

<aside class="key-point">
Components are initially <strong>stopped</strong> when they are created. A
component must be successfully <strong>resolved</strong> by a component resolver
before it can <strong>start</strong>.
</aside>

Components may stop themselves by exiting the program (as defined by the
component's `runner`), or the framework may stop the component as part of
system shutdown. Before being destroyed, the framework moves components to a
**shutdown** state to indicate that it cannot be started again.

![Diagram showing how components have two distinct states: instance and
execution. Together, these states describe the "component lifecycle."]
(/docs/get-started/images/components/component-lifecycle.png){: width="662"}

Note: For more details on component states and execution, see
[component lifecycle](/docs/concepts/components/v2/lifecycle.md).
