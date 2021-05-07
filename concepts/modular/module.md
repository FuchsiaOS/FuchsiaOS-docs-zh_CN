# Modules

Note: The Modular framework is being deprecated in favor of
the [Session Framework](/docs/concepts/session/introduction.md).

A `Module` is a component that displays UI in a Modular session. Modules are
added to a story (a container for Modules). Multiple modules can be added to a
single story.

## Environment

A module is given access to two services provided by the modular framework in
its incoming namespace:

*   `fuchsia.modular.ComponentContext`, which gives the agent access to
    functionality that is shared across components run under the modular
    framework (e.g. modules, shells, agents).
*   `fuchsia.modular.ModuleContext`, which gives modules access to module
    specific functionality.

A module is expected to provide three services to the modular framework in its
outgoing namespace:

*   `fuchsia.ui.app.ViewProvider`, which is used to display the module's UI.
*   `fuchsia.modular.Lifecycle`, which allows the framework to signal the module
    to terminate gracefully.

## Lifecycle

A module's lifecycle is bound to the lifecycle of the story it is part of. In
addition, a given module can have multiple running instances in a single story.

## Communication Mechanisms

Modules can acquire services from agents by connecting to those services
from the module's incoming directory.
