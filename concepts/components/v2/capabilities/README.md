# Capabilities

Components interact with one another through [capabilities][glossary.capability].
A capability combines access to a resource and a set of rights, providing a
access control and a means for interacting with the resource. Fuchsia
capabilities typically access underlying [kernel objects][glossary.kernel-object]
through [handles][glossary.handle] provided in the component's
[namespace][glossary.namespace].

A component can interact with the system and other components only through the
discoverable capabilities from its namespace and the few
[numbered handles][src-processargs] it receives.

## Capability routing {#routing}

Components declare new capabilities that they offer to the system and
capabilities provided by other components (or the framework) that they require
in their [component manifest][doc-component-manifest]. Component framework uses
these declarations to populate the namespace.

For capabilities to be available at runtime, there must also be a valid
[capability route][glossary.capability-routing] from the consuming component to
a provider. Since capabilities are most often routed through parent components
to their children, parent components play an important role in defining the
sandboxes for their child components.

Some capability types are routed to [environments][glossary.environment] rather
than individual component instances. Environments configure the behavior of
the framework for the realms where they are assigned. Capabilities routed to
environments are accessed and used by the framework. Component instances do not
have runtime access to the capabilities in their environment.

### Routing terminology {#routing-terminology}

Routing terminology divides into the following categories:

1.  Declarations of how capabilities are routed between the component, its
    parent, and its children:
    -   `offer`: Declares that the capability listed is made available to a
        [child component][doc-children] instance or a
        [child collection][doc-collections].
    -   `expose`: Declares that the capabilities listed are made available to
        the parent component or to the framework. It is valid to `expose` from
        `self` or from a child component.
1.  Declarations of capabilities consumed or provided by the component:
    -   `use`: For executable components, declares capabilities that this
        component requires in its [namespace][glossary.namespace] at runtime.
        Capabilities are routed from the `parent` unless otherwise specified,
        and each capability must have a valid route from its source.
    -   `capabilities`: Declares capabilities that this component provides.
        Capabilities that are offered or exposed from `self` must appear here.
        These capabilities often map to a node in the
        [outgoing directory][glossary.outgoing-directory].

## Capability types {#capability-types}

The following capabilities can be routed:

| type                                 | description                   | routed to                         |
| ------------------------------------ | ----------------------------- | --------------------------------- |
| [`protocol`][capability-protocol]    | A filesystem node that is     | components                        |
:                                      : used to open a channel backed :                                   :
:                                      : by a FIDL protocol.           :                                   :
| [`service`][capability-service]      | A filesystem directory that   | components                        |
:                                      : is used to open a channel to  :                                   :
:                                      : one of several service        :                                   :
:                                      : instances.                    :                                   :
| [`directory`][capability-directory]  | A filesystem directory.       | components                        |
:                                      :                               :                                   :
| [`storage`][capability-storage]      | A writable filesystem         | components                        |
:                                      : directory that is isolated to :                                   :
:                                      : the component using it.       :                                   :
| [`resolver`][capability-resolver]    | A capability that, when       | [environments][doc-environments]  |
:                                      : registered in an environment, :                                   :
:                                      : causes a component with a     :                                   :
:                                      : particular URL scheme to be   :                                   :
:                                      : resolved with that resolver.  :                                   :
| [`runner`][capability-runner]        | A capability that, when       | [environments][doc-environments]  |
:                                      : registered in an environment, :                                   :
:                                      : allows the framework to use   :                                   :
:                                      : that runner when starting     :                                   :
:                                      : components.                   :                                   :

## Examples {#examples}

Consider the following example that describes capability routing through the
component instance tree:

<br>![Capability routing example](/docs/concepts/components/v2/images/capability_routing_example.png)<br>

In this example:

-   The `echo` component instance provides the `fuchsia.Echo` protocol as one
    of its declared *capabilities*.
-   The `echo_tool` component instance requires the *use* of the
    `fuchsia.Echo` protocol capability.

Each intermediate component cooperates to explicitly route `fuchsia.Echo`
from `echo` to `echo_tool`:

1.  `echo` *exposes* `fuchsia.Echo` from `self` so the protocol is visible to
    its parent, `services`.
1.  `services` *exposes* `fuchsia.Echo` from its child `echo` to its parent,
    `shell`.
1.  `shell` *offers* `fuchsia.Echo` from its child `services` to another child,
    `tools`.
1.  `tools` *offers* `fuchsia.Echo` from `parent` to its child, `echo_tool`.

Component Framework grants the request from `echo_tool` to use `fuchsia.Echo`
because a valid route is found to a component providing that protocol capability.

For more information on how components connect to capabilities at runtime, see
[Life of a protocol open][doc-protocol-open].

[capability-protocol]: /docs/concepts/components/v2/capabilities/protocol.md
[capability-service]: /docs/concepts/components/v2/capabilities/service.md
[capability-directory]: /docs/concepts/components/v2/capabilities/directory.md
[capability-storage]: /docs/concepts/components/v2/capabilities/storage.md
[capability-resolver]: /docs/concepts/components/v2/capabilities/resolvers.md
[capability-runner]: /docs/concepts/components/v2/capabilities/runners.md
[doc-children]: /docs/concepts/components/v2/realms.md##child-component-instances
[doc-collections]: /docs/concepts/components/v2/realms.md#collections
[doc-component-manifest]: /docs/concepts/components/v2/component_manifests.md
[doc-environments]: /docs/concepts/components/v2/environments.md
[doc-outgoing-directory]: /docs/concepts/packages/system.md#outgoing_directory
[doc-protocol-open]: /docs/concepts/components/v2/capabilities/life_of_a_protocol_open.md
[doc-resolvers]: /docs/concepts/components/v2/capabilities/resolvers.md
[glossary.capability]: /docs/glossary#capability
[glossary.capability-routing]: /docs/glossary#capability-routing
[glossary.child]: /docs/glossary#child-component-instance
[glossary.component]: /docs/glossary#component
[glossary.environment]: /docs/glossary#environment
[glossary.handle]: /docs/glossary#handle
[glossary.kernel-object]: /docs/glossary#kernel-object
[glossary.namespace]: /docs/glossary#namespace
[glossary.outgoing-directory]: /docs/glossary/README.md#outgoing-directory
[glossary.parent]: /docs/glossary#parent-component-instance
[src-processargs]: /zircon/system/public/zircon/processargs.h
