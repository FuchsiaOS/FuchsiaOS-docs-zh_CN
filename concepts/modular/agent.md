# Agents

Note: The Modular framework is being deprecated in favor of
the [Session Framework](/docs/concepts/session/introduction.md).

An `Agent` is a singleton-per-session component without any graphical UI.

Agents can provide services to other modular components.

Any modular component can connect to an agent and access its services (including
modules, shells, and other agents).

## Environment

An agent is given access to two services provided by the modular framework in
its incoming namespace:

*   `fuchsia.modular.ComponentContext`, which gives the agent access to
    functionality that is shared across components run under the modular
    framework (e.g. modules, shells, agents).
*   `fuchsia.modular.AgentContext`, which gives agents access to agent specific
    functionality.

An agent is expected to provide the following services to the modular framework in its
outgoing namespace:

*   `fuchsia.modular.Lifecycle` allows the framework to signal the agent
    to terminate gracefully.

## Lifecycle

Agents are launched when a client requests a service that is provided by the agent.

"Session agents" and "startup agents", specified in a (Modular
config)[guide/config.md], are started eagerly during session startup.

## Exposing FIDL services

Agents can expose services to other components. They do so by publishing
these services in their outgoing directory. Other components (agents, shells, modules)
can connect to these services through their incoming directory (aka. namespace)
like any other service, provided a mapping in the `agent_service_index`
for the requested service exists.

See [Guide to configuring the Modular Framework](guide/config.md) for information on
the `agent_service_index`.
