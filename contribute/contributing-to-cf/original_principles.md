# Original Principles

This document describes the design principles that have guided Component
Framework to date.

If a principles is included in this document, either Component Framework already
supports it, or it part of the original vision even if not fully realized in
today's system. This document aims to promote a common understanding of what the
principles are, not what they should be. The merit of these principles, or
suggestions for new principles that the framework wasn't originally designed to
provide, are out of scope for this document. Once the migration to Components v2
is complete, we expect to modify the principles to best match what has been
learned since Component Framework was first envisioned. Consider this document a
recording of historical context as a starting point for a process to evolve
them.

## Background

We define a *principle* as a fundamental statement about the properties of a
system that generally holds true. Principles are derived from the system's
design goals. Although generally true, under certain circumstances a principle
may not be perfectly satisfied, for example:

-   Tech debt that does not conform to modern standards.
-   Exotic use cases that require an exception to a general rule.
-   Developer hooks that provide more powerful or privileged access than what is
    appropriate in production.

There are a few strategies to improve the degree to which the system sustains a
principle, which are not mutually exclusive:

-   Have tools or runtime enforce invariants that make it impossible to violate
    the principle.
-   Design APIs that naturally encourage the user to interact with the system in
    a way that follows the principle.
-   Build higher-level frameworks that are consistent with the principle.
-   Practice diligence when making changes to the system and evaluate those
    changes against the principle.
-   Promote best practices through documentation and examples.

Some of the principles in this doc are specific to Component Framework. Others
are general Fuchsia principles that Component Framework also commits to.

## Principles

### Least privilege

Components should receive the minimum capabilities they need to perform their
role in the system and nothing more, in accordance with the
[Principle of Least Privilege][wiki-least-privilege].

Note: Component framework by itself cannot guarantee that components have least
privilege. The responsibility for this principle extends to platform and product
maintainers that use the component framework to assemble topologies.

Example: `sysmgr` offers an example of an anti-pattern in Components v1. The v1
APIs supported the ability to instantiate enclosed sub-realms provisioned with
their own set of capabilities. However, `sysmgr` defined a realm which contained
almost every system capability, making it very easy for a v1 component to
violate least privilege.

#### What's implemented today

The component framework provides capability routing as the main form of access
control. It requires every capability[*](#framework-exception) to be explicitly
declared in a route thereby granting the capability to children. By employing
capability routing, a parent component defines the sandboxes for its children.

#### The vision that wasn't implemented

There was originally an idea to complement capability routing with role-based
access control. This idea never got past early brainstorming however.

There was an intent for component manager to support flexible isolation
policies, which are another mechanism to control privilege. More in
[Isolation](#isolation) below.

## No ambient authority

A system exhibits [ambient authority][wiki-ambient-authority] when a program can
perform an operation on an object without being granted access to the object
explicitly, e.g. by referring to it by name or number. It may be difficult to
ensure that the program has a legitimate reason to access the object because it
can generate the name by itself.

In contrast, with [capability-based security][wiki-capabilities], a program can
only perform an operation on an object if it has a capability to access that
object. Capabilities can be transferred but cannot be forged, enabling
delegation of access to occur safely without ambient authorities.

Fuchsia, and likewise component framework, should operate
[without ambient authorities][docs-principles-secure].

#### What's implemented today

Since capabilities are only[*](#framework-exception) provided by the parent,
there is no global namespace from which to acquire capabilities or operate on
objects. Example: what looks to a component like its POSIX filesystem is derived
from the parent's declaration of the component's sandbox.

Component framework, except in the case of a few privileged
APIs,[*][#peer-exception] does not reveal to components the identity of their
peers. This prevents components from misusing peer identity to build ambient
authorities.

Framework capabilities are a form of limited ambient authority because any
component can access them. However, this ambient authority is safe because the
scope of these capabilities is limited to the component's own
realm.[*](#framework-exception)

#### The vision that wasn't implemented

Component framework [environments][docs-environments], which allow a parent to
provision capabilities that are inherited by default, are a compromise between
security and ergonomics that create a sort of ambient authority. While
components have the ability to override the environment passed to a child, most
of the time they don't and unlike regular capability routing, the component
doesn't have insight into the capabilities that constitute the environment. In
particular, environments are used to provision runners and resolvers. This
points to a possible gap in the framework for a way to route capabilities both
securely and ergonomically.

### Universality

Components are the [foundational building blocks][docs-intro-components] of
Fuchsia software. *Universality* is the principle that all userspace software
should run as components.

There is a strong and weak form of this principle. The weak form is that all
software should run within components. The strong form is that programs that are
part of a larger subsystem ought to be components and not processes.
Furthermore, individual programs may decide to break submodules into components.

#### What's implemented today

As of April 2022, a large chunk of the system is running as v2 components.

Most user experience components (running under Modular) are still v1 as of April
2022, but there is an active project to migrate them to v2.

Drivers and filesystems are processes, but there are plans to turn them into
components.

#### The vision that wasn't implemented

It is not clear if the strong form of the principle will be applied everywhere.
For example:

-   Processes inside Chromium are just processes, not components. It's unclear
    whether they should be components however.
-   Shell programs are processes today, and we don't know if they will become
    components.

We never developed formal guidance for when it is appropriate to represent a
program as a component vs. a process.

### Independence

There may be multiple instances of the same (or different) components running
simultaneously on a Fuchsia device. Component instances are independent along
the following dimensions:

-   Identity: Each instance is distinguishable from other instances.
-   Sandbox: Each instance has its own namespace tailored just for that
    instance.
-   Storage: Each instance has its own private storage location, assuming it
    needs storage (such as for its grants, mailboxes, data, caches, and other
    state).
-   Lifecycle: Each instance can be independently started or stopped.
-   Children: Each instance has its own list of children and is responsible for
    configuring the environment in which its children will run.

#### What's implemented today

It's possible to create multiple instances of a component. Component instances
carry their own state, children and lifecycle.

Today's system has a few concepts which map to "component identity", used in
different contexts:

-   A component's absolute [moniker][docs-monikers]. Used for capability
    allowlist.
-   A component's relative [moniker][docs-monikers]. Supplied in a component
    event's payload, used by `archivist` and `test_manager` to identify clients.
-   The [instance id][docs-storage-index]. Used for persistent storage. This
    allows a component's topological position to change without losing storage.
-   The component's [URL][docs-url]. Used when resolving a component and
    sometimes as a proxy for component instance identity (by cobalt for
    example).

#### The vision that wasn't implemented

Grants (persisted capabilities) and mailboxes (basically a message bus between
components that don't require the sender and receiver to be running) were not
implemented.

The question of what "component identity" means is not fully resolved. As
mentioned in the section above, component framework has various notions of
identity which are not consistent between each other. For example, under what
circumstances is a component considered the "same" when it is
updated/moved/recreated?

### Isolation

Components may possess sensitive information and privileges and the component
framework is responsible for protecting their
[confidentiality, integrity, and availability][wiki-info-security]. To do so,
the component framework uses various mechanisms to isolate components and
prevent them from interfering with one another. In particular:

-   The component framework should prevent components from reading or modifying
    each other's internal state.
-   The component framework should not reveal to components the true identity of
    their peers. It should either provide local identity (e.g. child name),
    obfuscated identity, or no identifying information at all.

#### What's implemented today

Because components interact through capabilities, they do not know who is on the
other end.

The component framework, in collaboration with runners, supports various
mechanisms for isolating state between components, which it either defines
itself or uses from Zircon:

-   Process isolation: Some runners execute components in separate processes to
    prevent them from accessing each other's memory and handles.
-   ELF runner implements process isolation: each component receives its own
    process and job.
    -   However, not all runners provide the same isolation guarantees. Dart
        runner, where multiple component instances share the same process, would
        provide weak memory isolation guarantees.
-   Storage isolation: Components cannot directly access each other's isolated
    persistent storage and are typically not granted access to shared mutable
    storage either.
-   Memory isolation: Components cannot directly access each other's private
    memory (though they may decide to exchange [shared VMOs][docs-vmo]).

Component framework does not reveal the identity of a component's
peers.[*](#peer-exception)

#### The vision that wasn't implemented

There was originally an idea that the component framework would define a type of
container, called a "compartment", to act as a runtime isolation boundary. This
would count as one of the basic component relationships. Configuring the
compartment boundaries would let product owners make tradeoffs between safety
and performance.

We've achieved the key performance benefits of colocating components in
processes through custom solutions for specific use cases (devhost, fshost), but
nothing that is generally usable at the component framework level. At one point,
it was believed that Flutter would present an important use case because Flutter
programs can benefit from collocation (so that each Flutter program doesn't have
to bring along its own runtime). However, this turned out to be difficult to
achieve due the lack of a stable ABI in Flutter and Dart, and there wasn't a use
case that strongly called for it.

There is a long-standing idea that component framework could give components an
obfuscated token they could use to locally identify their peers. This was called
"obfuscated monikers": component manager could take the relative
[moniker][docs-monikers] between components, hash it (perhaps with a nonce), and
present the client with the hash. Since this is derived from the relative
moniker, it would inherently be instance-specific.

### Accountability

System resources are finite. There's only so much memory, disk, or CPU time
available on a computing device. The component framework should keep track of
how resources are used by components to ensure they are being used efficiently
and that they can be reclaimed when no longer required or when they are more
urgently needed for other purposes if the system is oversubscribed.

Resources must be used for a reason. As a general rule, every resource in the
system must be accounted for in some way so the system can ensure they are being
used effectively.

Every component exists for a reason. Parent component instances are responsible
for determining the existence of their children by destroying children that are
no longer of use. Parents also play a role in setting resource constraints for
their children.

Every component runs for a reason. The component framework starts component
instances when they have work to do, such as in response to incoming service
requests from other components, and stops them when the demand is gone (or has
lesser priority than other demands that contend for the same resources).

#### What's implemented today

The component framework's current support for accountability is basic.

The component framework requires that every component belongs to a parent
component.

Component manager starts component instances in response to a request (with the
exception of single-run components).Component manager never proactively stops
components except when they are destroyed or during system shutdown.

Component manager reclaims resources of destroyed dynamic components, erasing
their storage in the background after the component is deleted.

Component framework supports an [`eager`][docs-eager] option that causes a
component instance to be started automatically. There is substantial uncertainty
about how this feature fits into the vision for accountability. There is no
clear 'reason' associated with the startup of eager components, and component
manager never restarts eager components. On the other hand, from a UX
perspective it's a simple, convenient way to run 'daemon'-type components, and
we haven't come up with a better solution yet.

#### The vision that wasn't implemented

There was originally a much larger vision for accountability.

Every running process must belong to at least one component instance whose
capabilities are currently in use, were recently of use, or will soon be of use;
any outliers are considered to be running for no reason and are promptly
stopped.

"Resources must be used for a reason" is currently true for
[isolated storage][docs-storage], but that's about it. There was a vision for a
resource management system where each resource in use on the system would be
attributed to a particular component instance. This attribution could be used to
expose metrics for diagnostics, enforce resource limits, and balance load.
Examples of resources we might track include storage, memory, CPU, GPU, power,
or bandwidth. It's likely we'll implement this, or a subset of it, in the
future.

"Every component runs for a reason" is only partially achieved today. Most
components are started in response to a request to access one of their exposed
capabilities. However, component manager makes no effort to proactively
terminate components that are not in use or are consuming too many resources. In
particular, measuring when a component is "not in use" is known to be a hard
problem because component manager only brokers the introduction phase of service
discovery -- once the connection between client and provider has been
established, it gets out of the way.

There were originally plans to build a "deferred communication" framework. This
would grant the ability for a component to dispatch a message or work item which
is delivered to the receiving component at a later time, relaxing constraints on
when components need to run and giving the component runtime more leeway to
start and stop them. In particular, the following systems were proposed:

-   Work scheduler: Schedule work to be processed later only once certain
    conditions are met.
-   Mailboxes: Allow components to post messages to a "mailbox" which holds on
    to them until the receiving component is ready to process them.

## The illusion of continuity

The component framework should offer mechanisms to preserve the illusion of
continuity: the user should generally not be concerned about restarting their
software because it will automatically resume right where they left off, even
when they reboot or replace their devices.

The fidelity of the illusion depends on how well the following properties are
preserved across restarts:

-   State: Preserving the user-visible state of component instances.
-   Capabilities: Preserving the rights granted to component instances.
-   Structure: Preserving the relationships between collaborating component
    instances such that they can reestablish communication as required.
-   Behavior: Preserving the runtime behavior of component instances.

In practice, the illusion is imperfect. The system cannot guarantee faithful
reproduction in the presence of software upgrades, non-determinism, bugs,
faults, and external dependencies on network services.

While it might seem simpler to keep components running forever, eventually the
system will run out of resources so it needs a way to balance its working set
size by stopping less essential components at a moment's notice (see
[Accountability](#accountability)).

#### What's implemented today

In general, components continue to exist even when they stop running. Compare
this with how processes work.

The capabilities routed to, from, or through a component remain consistent if
the component is restarted. However, the *connections* to those capabilities are
not preserved across restarts. Depending on the capability, a new instance might
not act the same way as the old one, or it might not be possible to get multiple
instances.

Component framework supports [persistent storage][docs-storage] for static
component instances. The component's storage can be preserved even if its
topological position changes.

#### The vision that wasn't implemented

There are many components in the system not tolerant to restarts. Many of these
can be found by searching for components that use the `reboot_on_terminate`
feature.

There are no standard design patterns for how to build a component that is able
to recover its state when it restarts, or for what to do when one of its
dependencies becomes unavailable.

There are open questions about how restart policy for components should be
configured. Relatedly, there are questions about when and how to reestablish
connections between components when the server is restarted.

Components have no way to persist capabilities (also known as "grants"). If they
are restarted, they must re-acquire them. Components also have no way to persist
messages they have received or defer dispatch of messages to a later time.
However, there was an idea for a message queue architecture called "mailboxes"
which would have supported this.

If a component subscribed to events and died while some unprocessed events were
in its queue, it will lose those events.

Components do not support suspend/hibernate.

### Prefer declarative over imperative APIs

This principle could have also been called "prefer static over dynamic".

The component framework instills a general preference for APIs that are static,
declarative, and assembly-time over those that are dynamic, imperative, and
runtime-based.

This is not to say that *all* component framework APIs are declarative -- a
completely static system wouldn't be very useful! However, the general rule is
that if an aspect of a component's definition or behavior could be described
statically, it should be.

Being declarative offers the following advantages:

-   Accessibility: The system’s structure and security policy are accessible to
    developers and security specialists. In principle, they can also be
    described to end-users by various means. e.g. “Module X wants to access the
    microphone…”
-   Alignment: The security boundaries are clearly marked and align with
    well-established architectural abstractions. Here we place them at component
    instance boundaries.
-   Auditability: Declarations can be conveniently reviewed by humans.
    Centralizing the most sensitive declarations (e.g. role-based access control
    policy) helps a lot too.
-   Testability: The predicted and actual result of modifying declarations can
    easily be evaluated.
-   Immutability: Authoritative declarations can be baked into the system’s
    chain of trust and verified.

#### What's implemented today

The vision for declarative APIs is implemented through [CML][docs-manifests].
CML statically describes a component's inputs and outputs (capability routes),
composition (children), and execution information.

Combined together, the component manifests in a Fuchsia build form a "component
instance tree" that can be explored with host-side tooling (`scrutiny`) or even
just by inspecting the source files. There is also a `verify routes` plugin for
`scrutiny` run automatically on CQ, that verifies all routes in the static
topology are intact.[*](#declarative-exception)

[Security policy allowlists][src-security-policy] are another part of the
declarative API.

Some parts of the component framework API are imperative, but only when there is
a good reason for them to be. Examples include: collections, dynamic offers,
RealmBuilder, and service aggregation from collections.

#### The vision that wasn't implemented

The dynamic parts of the component framework API are not as thoroughly
developed. Historically, a lot of these questions were delegated to the session
framework, but session framework has since been sunset. Overall, current
products don't demand much in the way of dynamic component configuration.
However, this may change if and when Fuchsia embraces third party or more open
products like workstations.

Component framework APIs tend to be either mostly static or mostly dynamic;
there is not much in between (service aggregation is an exception). In some
cases it could be useful to have APIs that are principally static but delegate
some aspects to runtime, or that are principally dynamic but are constrained by
a statically described "upper bound".

### Sandboxing

Component instances have no awareness of where the services in their
[sandbox][wiki-sandbox] actually come from. They perceive a subjective reality
defined by their sandbox. It follows that component instances should not be able
to distinguish whether they are running within a "test" sandbox or a "real"
sandbox unless they are provided some means of external attestation,
notwithstanding the possibility of covert side-channels.

Parents have a significant degree of authority over their children:

-   Although a parent component instance knows the [URL][docs-url] of each of
    its children, the reverse is not true.
-   When components request to use capabilities, they have no control over where
    those capabilities come from. Components must trust their parents to have
    offered them capabilities from a trustworthy source. Sandboxing supports a
    property called "recursive symmetry". This is the idea that a subtree of the
    component topology can be considered isomorphic to a full component
    topology, if we take the root of the subtree as the root of the full
    topology it's isomorphic to. In particular, it should be possible to run a
    separate copy of the system in a subtree of the component topology without
    breaking things.

A corollary of this principle is that component manager should have *no global
singletons*.

#### What's implemented today

A large part of the vision of sandboxing was implemented. When you define a
subtree it behaves a lot like a full topology:

-   Because components don't have access to information about their parent, it
    "can't tell" what part of the tree it's in. For instance, a component
    doesn't have a way to distinguish whether it's running in a test or
    production environment (unless it somehow deduces this fact through its
    interactions with the capabilities offered to it).
-   All capabilities with a `framework` source are scoped. For example, if you
    request the `hub from framework`, you'll get a hub rooted at that component.
-   Component framework offers a number of [built-in capabilities][src-builtin],
    but these capabilities can be shimmed simply by replacing them with a
    capability from another source. This includes runners and resolvers.
-   When you subscribe to an event scoped to the root of the subtree, the
    event's [moniker][docs-monikers] is relative to the root.
-   It's possible to run a nested instance of archivist. Selectors sent to this
    archivist are relative to the event the archivist subscribed to.

#### The vision that wasn't implemented

Some parts of the component framework API use absolute monikers:

-   [Security policy allowlists][src-security-policy]
-   [Storage id index][docs-storage-index]
-   [Route exceptions allowlist][src-route-exceptions]

The system shutdown API, although it could conceivably be scoped to a subtree,
always shuts down the entire tree.

There was an idea that you could run `ffx component relative` to a subtree,
although this hasn't been implemented.

A related idea, which was never implemented, is that it should be possible to
run a nested instance of component manager in the tree, and it should be
possible to "compose" the subtree under the nested component manager with the
parent tree.

In its most powerful form, recursive symmetry could support "lifting" a subtree
of a topology to run on a different device, with all persisted state intact.

## Encapsulation

Encapsulation, as in OOP, refers to the concealment of a component's internal
structure or data from a containing component. In particular, this means
component instances should only have direct awareness of their child components
(components they instantiated themselves) but not about their children’s
descendants and not about their own ancestors.

### What's implemented today

Components are, faithful, faithful to encapsulation. Parents have access to the
identities and exposed capabilities of their children, but not to their
grandchildren, barring some privileged APIs.[*](#peer-exception)

As discussed in [Sandboxing](#sandboxing), there are no safeguards against
malicious parents that would offer their children a compromised capability. This
is by design; in capability based systems, it's normal for parents to dominate
their children.

#### The vision that wasn't implemented

Normally, a child's internal state is isolated from its parent, just like any
other two component instances. However, there are clever ways a parent can
circumvent this. For example, a parent could inject a pseudo ELF runner that
behaves mostly like an ELF runner, but it injects a thread into the component's
process that exfiltrates the component's private memory.

## Loose coupling

Loose coupling makes it easier to evolve components over time. The component
architecture abstracts away most component implementation details (such as the
programming language used to implement components) behind common IPC protocols
and data formats.

When a component uses a capability, it should explicitly declare the constraints
it needs that capability to satisfy. As long as a capability provider satisfies
these constraints, it should be possible to substitute one implementation for
another. This property is called *substitutability*. Examples of such
constraints are:

-   FIDL protocol interface
-   [ABI version][rfc-versions]
-   [Component URL][docs-url], when a component uses a capability from a child

When components request capabilities by name from their namespace, the choice of
which implementation to use resides in the component’s ancestors since they set
up the environment for the component. This "call by meaning" approach to
capability discovery makes the system more dynamic and configurable than if
components explicitly requested to be bound to specific implementations of these
capabilities (although sometimes they may).

#### What's implemented today

Capabilities are the inputs and outputs of a component. To a large extent, the
interactions between components can be described in terms of the capabilities
routed between them, and in this sense components depend on each others'
interfaces, not their implementations.

However, when a component instantiates a child, it chooses the child component
by specifying a URL. In this case the component might expect a particular
implementation of the child. This is a form of tighter coupling, although there
is a degree of freedom because the URL is resolved relative to a resolver, which
makes the ultimate decision about what component to resolve the URL to.

Component framework hides the identity of peers, which supports
substitutability.

#### The vision that wasn't implemented

Components depend on capabilities only by name, without any version information.
This can make components tightly coupled by version. Platform ABI version will
solve a part of this problem.

It's not clear to what extent dependencies by component URLs agree with this
principle.

## Updatability

Components can be updated independently of other components.

Component binaries and assets can be fetched just in time, cached, and removed
when no longer of proximate use, freeing up storage for other components.

Software packages are signed to verify their authenticity and integrity, making
it safe to retrieve them again from any available source, including from other
Fuchsia devices.

#### What's implemented today

In eng builds, when a component is relaunched, its runtime information, i.e.
package, binary, and namespace, is updated to the latest version from the
package server. (However, otherwise its manifest is not updated.)

A component's runtime assets (binary and package) are discarded when the
component terminates.

#### The vision that wasn't implemented

There is an [RFC][rfc-eager-updates] approved for eager updates that will make
it possible to update packages outside of an OTA. However, some work remains to
integrate this update flow with the component framework.

Component manager never evicts the cached copy of a manifest. In eng builds,
this can lead to inconsistency between the component's runtime state and its
children or capability routes.

## Usability

The essentials of the component architecture should be easy for developers to
learn and apply.

The component architecture offers a relatively small number of general-purpose
primitives which effectively cover the needs of software composition for all of
Fuchsia’s architectural layers, from device drivers to third-party end-user
apps. All components use the same primitives though they may receive different
capabilities due to their respective roles.

The component architecture also eschews making assumptions about
product-specific requirements, such as whether the product has a user interface
or how it works. This way, we don’t have to reinvent the wheel for each new
use-case that comes along.

The component framework has a responsibility to make it easy for users to get
the most out of the component framework. Here are some ways of doing this:

-   Design APIs so they naturally guide users in the right direction.
-   Publish accessible, thorough, and up to date documentation.
-   Provide paradigmatic examples that have relevance to real user problems.

#### What's implemented today

We have reference documentation under
[//concepts/components][docs-concepts] and
[//development/components][docs-development]. There is a
[Components getting started guide][docs-get-started].

We have some basic examples under [//examples][examples].

#### The vision that wasn't implemented

The general sentiment is that many component framework APIs aren't as elegant or
user friendly as they could be.

The developer experience of iterating on a component and validating correctness
outside of tests leaves much to be desired. We have several ideas for
runtime-based component exploration tools that we've not yet pursued.

We have much more documentation to write, especially howto-style docs. In
addition, some of the existing documentation could benefit from some love. The
examples we have are fairly basic and limited to C++ and rust. We could probably
benefit from more sophisticated or realistic examples.

## Gaps in the vision

The component framework provides abstractions over parts of Zircon but these
abstractions are different than Zircon and they do not capture all the features
that Zircon provides. For example:

-   Zircon has policy controls for construction of all types of objects. The
    component framework has plumbing for only a subset of these such as process
    object creation.
-   Zircon has types of capabilities (e.g. [eventpairs][docs-eventpair],
    [sockets][docs-socket]) that the component framework does not support.
-   Compent framework capabilities are always "factories" you can use to get a
    Zircon object. There is no way to route a singleton Zircon object.

The component framework does little to promote compatibility with software
written with a traditional program model. This was probably intentional, but
it's likely we'll decide to incorporate some of these features in the future.

## Appendix

### Framework capabilities {#framework-exception}

The framework (through component manager) gives access to some capabilities not
specifically granted by the parent, for example:

-   [`fuchsia.component.Realm`][fidl-realm], which allows a component to control
    the lifecycle of its children.
-   Component-scoped event streams, which allow components to receive lifecycle
    events about children. - Access to its own package through `/pkg`, which
    every component gets even without having to request it
-   [`hub`][docs-hub], which allows its client to traverse part of component
    topology rooted at a particular realm, meaning that it can observe and
    access the services of all components within that realm.

Nevertheless, thanks to the invariant that framework capabilities never provide
access to capabilities from the containing environment, these capabilities do
not violate [least privilege](#least-privilege),
[no ambient authority](#no-ambient-authority), or
[encapsulation](#encapsulation).

### APIs that expose peer info {#peer-exception}

There are some privileged APIs (hub, realm-scoped event streams) that expose
internal information about a component, such as its relative moniker, URL, or
outgoing directory. However, these APIs are locked down and only usable by
non-production or specially privileged components like `archivist` or
`debug_data`.

### Limitations of static validation of the topology {#declarative-exception}

There is a limit to how much of the topology can be statically validated. When
the exploration reaches a collection, in general it has to stop because the
contents of a collection are runtime determined.

[docs-concepts]: /concepts/components/v2
[docs-development]: /development/components/build.md
[docs-eager]: /development/components/connect.md#eager
[docs-environments]: /concepts/components/v2/environments.md
[docs-eventpair]: /reference/kernel_objects/eventpair.md
[docs-get-started]: /get-started/learn/components
[docs-hub]: /concepts/components/v2/hub.md
[docs-intro-components]: /concepts/components/v2/introduction.md#components
[docs-manifests]: /concepts/components/v2/component_manifests.md
[docs-monikers]: /concepts/components/v2/identifiers.md#monikers
[docs-principles-secure]: /concepts/principles/secure.md
[docs-socket]: /reference/kernel_objects/socket.md
[docs-storage-index]: /development/components/component_id_index.md
[docs-storage]: /concepts/components/v2/capabilities/storage.md
[docs-url]: /concepts/components/v2/identifiers.md#component-urls
[docs-vmo]: /reference/kernel_objects/vm_object.md
[examples]: /examples
[fidl-realm]: /sdk/fidl/fuchsia.component/realm.fidl
[rfc-eager-updates]: /contribute/governance/rfcs/0145_eager_package_updates.md
[rfc-versions]: /contribute/governance/rfcs/0002_platform_versioning.md
[src-builtin]: /src/sys/component_manager/configs/bootfs_config.json5
[src-route-exceptions]: /src/security/policy/build/verify_routes_exceptions_allowlist.json5
[src-security-policy]: /src/security/policy/component_manager_policy.json5
[wiki-ambient-authority]: https://en.wikipedia.org/wiki/Ambient_authority
[wiki-capabilities]: https://en.wikipedia.org/wiki/Capability-based_security
[wiki-info-security]: https://en.wikipedia.org/wiki/Information_security
[wiki-least-privilege]: https://en.wikipedia.org/wiki/Principle_of_least_privilege
[wiki-sandbox]: https://en.wikipedia.org/wiki/Sandbox_(computer_security)
