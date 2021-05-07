# Design principles (Components v2)

<<../_v2_banner.md>>

This document offers a few significant design principles of component framework.

### Least privilege {#privilege}

Components should receive the minimum capabilities they need to perform their
role in the system and nothing more, in accordance with the
[Principle of Least Privilege][wiki-least-privilege].

Some components may be entrusted with privileges that could be harmful if
misused.  To mitigate this potential harm, the component architecture provides
mechanisms to grant components the minimum set of capabilities they require,
such as sandboxing, role-based access control, and isolation policies.

## Isolation {#isolation}

Components may possess sensitive information and privileges and the component
framework is responsible for protecting their
[confidentiality, integrity, and availability][wiki-infosec]. To do so,
the component framework using various mechanisms to isolate components and
prevent them from interfering with one another undesirable.

Here are some examples:

- Process isolation: Some runners execute components in separate processes
  to prevent them from accessing each other's memory and handles.
- Storage isolation: Components cannot directly access each other's
  [isolated persistent storage][doc-storage] and are typically not granted
  access to shared mutable storage either.

## Accountability {#accountability}

System resources are finite. There's only so much memory, disk, or CPU time
available on a computing device. The component framework keeps track of how
resources are used by components to ensure they are being used efficiently
and that they can be reclaimed when no longer required or when they are more
urgently needed for other purposes if the system is oversubscribed.

Resources must be used for a reason.

For example, every running process must belong to at least one component
instance whose capabilities are currently in use, were recently of use, or will
soon be of use; any outliers are considered to be running for no reason and are
promptly stopped.

Similarly, the system may terminate processes if they exceed the resource
constraints of the components that are responsible for them.

Here are some more examples of accountability:

- Every component exists for a reason: Parent component instances are
  responsible for determining the existence of their children by destroying
  children that are no longer of use. Parents also play a role in setting
  resource constraints for their children.
- Every component runs for a reason: The component framework starts
  component instances when they have work to do, such as in response to
  incoming service requests from other components, and stops them when the
  demand is gone (or has lesser priority than other demands that contend for
  the same resources).
- Metrics: The component framework provides mechanisms for diagnostics tools
  to audit resource usage by components over time.

As a general rule, every resource in the system must be accounted for in
some way so the system can ensure they are being used effectively.

## The illusion of continuity {#continuity}

The component framework offers mechanisms to preserve the illusion of
continuity: the user should generally not be concerned about restarting their
software because it will automatically resume right where they left off,
even when they reboot or replace their devices.

The fidelity of the illusion depends on how well the following properties
are preserved across restarts:

- State: Preserving the user-visible state of component instances.
- Capabilities: Preserving the rights granted to component instances.
- Structure: Preserving the relationships between collaborating component
  instances such that they can reestablish communication as required.
- Behavior: Preserving the runtime behavior of component instances.

In practice, the illusion is imperfect. The system cannot guarantee faithful
reproduction in the presence of software upgrades, non-determinism, bugs,
faults, and external dependencies on network services.

While it might seem simpler to keep components running forever, eventually the
system will run out of resources so it needs a way to balance its working
set size by stopping less essential components at a moment's notice.

## No ambient authority {#no-ambient-authority}

A system exhibits ambient authority when a program can obtain access to an
object simply by providing the object's name to a service in its environment,
such as the kernel. It may be difficult to ensure that objects are being
accessed appropriately because there is no chain of evidence to describe
how the object names were transmitted.

In contrast, with [capability][glossary-capability]-based security, a program
must possess a capability object to obtain access to an object. Capabilities
can be transferred but cannot be forged, enabling delegation of access to
occur safely without ambient authorities.

The component framework avoids introducing ambient authorities.

- Parent components can declare children and their children can declare their
  own children but parent components have no direct awareness of the existence
  of any of their grandchildren in the [component topology][doc-topology].
- When components request to use capabilities, they have no control over where
  those capabilities come from because they have no way to refer to other
  components outside of themselves. Components must trust their parents to have
  offered them capabilities from a trustworthy source.

However, some of the component framework's development diagnostics and debugging
features do introduce ambient authorities that are scoped to the realms of
interest.

- The [hub][doc-hub] allows its client to traverse part of component topology
  rooted at a particular realm, meaning that it can observe and access the
  services of all components within that realm. The hub capability is only
  granted to authorized components such as developer tools.

[doc-hub]: hub.md
[doc-storage]: capabilities/storage.md
[doc-topology]: topology.md
[glossary-capability]: /docs/glossary.md#capability
[wiki-infosec]: https://en.wikipedia.org/wiki/Information_security
[wiki-least-privilege]: https://en.wikipedia.org/wiki/Principle_of_least_privilege
