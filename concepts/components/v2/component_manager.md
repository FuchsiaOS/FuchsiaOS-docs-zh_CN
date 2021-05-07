# Component manager (Components v2)

<<../_v2_banner.md>>

The component manager is the heart of the component framework. It maintains the
[component topology][doc-topology], manages
[component lifecycle][doc-lifecycle], provides components with the
[capabilities][doc-capabilities] they require at runtime, and keeps them
isolated from one another.

## Booting the system

The component manager is responsible for starting most processes in the system.
It is one of the first processes created when the system boots and it is one
of the last processes destroyed when the system shuts down.

The component manager coordinates the execution of all components, beginning
with the root component that is launched at boot. The root component then
asks the component manager to start other components such as the device
manager, filesystems, network stack, and other essential services.

## Intermediation

The component manager intermediates all introductions between components at
runtime.

For example, when a component connects to a [service][doc-services], the
component manager validates the request, uses
[capability routing][doc-capability-routing] to find the component that exposes
the desired service, starts it if needed, establishes a direct connection
between the client and the service, and continues to monitor the relationship so
that the client and service are held accountable for their behavior.

The component manager has a highly privileged role in the system. Through
intermediation, it makes many critical decisions for system security and
stability.

## Framework capabilities

The component manager offers a variety of framework capabilities to components.
Components use these capabilities to interact with their environment with the
help of the component manager.

## Framework extensions

The component manager supports a variety of framework extensions that
components can implement to integrate new functionality with their environment.

- [Runners][doc-runners]: Integrate programming language runtimes and
  application frameworks.
- [Resolvers][doc-resolvers]: Integrate software delivery systems.

[doc-capabilities]: /docs/concepts/components/v2/capabilities
[doc-capability-routing]: /docs/concepts/components/v2/topology.md#capability-routing
[doc-hub]: /docs/concepts/components/v2/hub.md
[doc-lifecycle]: lifecycle.md
[doc-realms]: /docs/concepts/components/v2/realms.md
[doc-resolvers]: /docs/concepts/components/v2/capabilities/resolvers.md
[doc-runners]: /docs/concepts/components/v2/capabilities/runners.md
[doc-topology]: /docs/concepts/components/v2/topology.md
[doc-services]: /docs/concepts/components/v2/services.md
