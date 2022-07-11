# Fuchsia Namespaces

Namespaces are the backbone of file access and service discovery in Fuchsia.

## Definition

A namespace is a composite hierarchy of files, directories, sockets, services,
devices, and other named objects provided to a component by its
environment.

Let's unpack that a little bit.

**Objects are named**: The namespace contains _objects_ which can be enumerated
and accessed by name, much like listing a directory or opening a file.

**Composite hierarchy**: The namespace is a _tree_ of objects that has been
assembled by _combining_ together subtrees of objects from other namespaces
into a composite structure where each part has been assigned a path prefix
by convention.

**Namespace per component**: Every component receives its own namespace
tailored to meet its own needs.  It can also publish objects of its own
to be included in other namespaces.

**Constructed by the environment**: The environment, which instantiates a
component, is responsible for constructing an appropriate namespace for that
component within that scope.

Namespaces can also be created and used independently from components although
this document focuses on typical component-bound usage.

## Namespaces in Action

You have probably already spent some time exploring a Fuchsia namespace;
they are everywhere.  If you type `ls /` at a command-line shell prompt
you will see a list of some of the objects that are accessible from the
shell's namespace.

Unlike other operating systems, Fuchsia does not have a "root filesystem".
As described earlier, namespaces are defined per-component rather than
globally or per-process.

This has some interesting implications:

- There is no global "root" namespace.
- There is no concept of "running in a chroot-ed environment" because every
  component [effectively has its own private "root"](/concepts/filesystems/dotdot.md).
- Components receive namespaces tailored to their specific needs.
- Object paths may not be meaningful across namespace boundaries.
- A process may have access to several distinct namespaces at once.
- The mechanisms used to control access to files can also be used to control
  access to services and other named objects on a per-component basis.

## Objects

The items within a namespace are called objects.  They come in various flavors,
including:

- Files: objects that contain binary data
- Directories: objects that contain other objects
- Sockets: objects that establish connections when opened, like named pipes
- Services: objects that provide FIDL services when opened
- Devices: objects that provide access to hardware resources

### Accessing Objects

To access an object within a namespace, you must already have another object
in your possession.  A component typically receives channel handles for
objects in the scope of its namespace during
[Namespace Transfer](#namespace_transfer).

You can also create new objects out of thin air by implementing the
appropriate FIDL protocols.

Given an object's channel, you can open a channel for one of its sub-objects
by sending it a FIDL message that includes an object relative path expression
which identifies the desired sub-object.  This is much like opening files
in a directory.

Notice that you can only access objects that are reachable from the ones
you already have access to.  There is no ambient authority.

We will now define how object names and paths are constructed.

### Object Names

An object name is a locally unique label by which an object can be located
within a container (such as a directory).  Note that the name is a property
of the container's table of sub-objects rather than a property of the object
itself.

For example, `cat` designates a furry object located within some unspecified
recipient of an `Open()` request.

Objects are fundamentally nameless but they may be called many names by others.

Object names are represented as binary octet strings (arbitrary sequences
of bytes) subject to the following constraints:

- Minimum length of 1 byte.
- Maximum length of 255 bytes.
- Does not contain NULs (zero-valued bytes).
- Does not contain `/`.
- Does not equal `.` or `..`.
- Always compared using byte-for-byte equality (implies case-sensitive).

Object names are valid arguments to a container's `Open()` method.
See [FIDL Protocols](/concepts/fidl/overview.md).

It is intended that object names be encoded and interpreted as human-readable
sequences of UTF-8 graphic characters, however this property is not enforced
by the namespace itself.

Consequently clients are responsible for deciding how to present names
which contain invalid, undisplayable, or ambiguous character sequences to
the user.

<!-- _TODO(jeffbrown): Document a specific strategy for how to present names._ -->

### Object Relative Path Expressions

An object relative path expression is an object name or a `/`-delimited
sequence of object names designating a sequence of nested objects to be
traversed in order to locate an object within a container (such as a
directory).

For example, `house/box/cat` designates a furry object located within its
containing object called `box` located within its containing object called
`house` located within some unspecified recipient of an `Open()` request.

An object relative path expression always traverses deeper into the namespace.
Notably, the namespace does not directly support upwards traversal out of
containers (e.g. via `..`) but this feature may be partially emulated by
clients (see below).

Object relative path expressions have the following additional constraints:

- Minimum length of 1 byte.
- Maximum length of 4095 bytes.
- Does not begin or end with `/`.
- All segments are valid object names.
- Always compared using byte-for-byte equality (implies case-sensitive).

Object relative path expressions are valid arguments to a container's `Open()`
method.  See [FIDL Protocols](/concepts/fidl/overview.md).

### Client Interpreted Path Expressions

A client interpreted path expression is a generalization of object relative
path expressions, but includes optional features that may be emulated
by client code to enhance compatibility with programs that expect a rooted
file-like interface.

Technically these features are beyond the scope of the Fuchsia namespace
protocol itself but they are often used so we describe them here.

- A client may designate one of its namespaces to function as its "root".
  This namespace is denoted `/`.
- A client may construct paths relative to its designated root namespace
  by prepending a single `/`.
- A client may construct paths that traverse upwards from containers using
  `..` path segments by folding segments together (assuming the container's
  path is known) through a process known as client-side "canonicalization".
- These features may be combined together.

For example, `/places/house/box/../sofa/cat` designates a furry object
located at `places/house/sofa/cat` within some client designated "root"
container.

Client interpreted path expressions that contain these optional features
are not valid arguments to a container's `Open()` method; they must be
translated by the client prior to communicating with the namespace.
See [FIDL Protocols](/concepts/fidl/overview.md).

For example, `fdio` implements client-side interpretation of `..` paths
in file manipulation APIs such as `open()`, `stat()`, `unlink()`, etc.

## Namespace Transfer

When a component is instantiated in an environment (e.g. its process is
started), it receives a table that maps one or more namespace path prefixes
to object handles.

The path prefixes in the table encode the intended significance of their
associated objects by convention.  For example, the `pkg` prefix should
be associated with a directory object, which contains the component's own
binaries and assets.

More on this in the next section.

## Namespace Conventions

This section describes the conventional layout of namespaces for typical
components running on Fuchsia.

The precise contents and organization of a component's namespace varies
greatly depending on the component's role, type, identity, scope,
relation to other components, and rights. See [Sandboxing](sandboxing.md) for
information about how namespaces are used to create sandboxes for components.

_For more information about the namespace your component can expect to
receive from its environment, please consult the documentation related to
the component type you are implementing._

### Typical Objects

There are some typical objects that a component namespace might contain:

- Read-only executables and assets from the component's package.
- Private local persistent storage.
- Private temporary storage.
- Services offered to the component by the system, the component framework,
  or by the client that started it.
- Device nodes (for drivers and privileged components).
- Configuration information.

### Typical Directory Structure

- `pkg/`: the contents of the current program's package as they were when the
  package was signed
  - `bin/`: executable binaries within the package
  - `lib/`: shared libraries within the package
  - `data/`: data, such as assets, within the package
- `data/`: local persistent storage (read-write, private to the package)
- `tmp/`: temporary storage (read-write, private to the package)
- `svc/`: protocols and services offered to a component
  - `fuchsia.process.Launcher`: launch processes
  - `fuchsia.logger.Log`: log messages
  - `vendor.topic.Interface`: service defined by a _vendor_
- `dev/`: device tree (relevant portions visible to privileged components as needed)
  - `class/`, ...
- `hub/`: introspect the system, see [Hub][hub] (privileged components only)
- `config/`: configuration data for the component

## Namespace Participants

Here is some more information about a few abstractions that interact with
and support the Fuchsia namespace protocol.

### Filesystems

Filesystems make files available in namespaces.

A filesystem is simply a component that publishes file-like objects
from someone else's namespace.

### Protocols

[Protocols][glossary.protocol] are accessed through namespaces.

A **protocol handle** is a channel object backed by an implementation of a
FIDL protocol, which can be discovered using the namespace.
A protocol name corresponds to a path within the `/svc` branch of the namespace
from which a component can access the implementation.

For example, the default Fuchsia logging protocol is `fuchsia.logger.Log` and
its path in the namespace is `/svc/fuchsia.logger.Log`.

For more details on protocols and components,
see [protocol capabilities][protocol-capabilities].

### Services

[Services][glossary.service] are accessed through namespaces.

A **service handle** is a directory that contains related FIDL protocols.
Implementations of those protocols can be discovered using the namespace.
A service name corresponds to a path within the `/svc` branch of the namespace
from which a component can access the implementation.

For more details on services and components,
see [service capabilities][service-capabilities].

### Components

Components consume and extend namespaces.

A component is an executable program object that has been instantiated
within some environment and given a namespace.

A component participates in the Fuchsia namespace in two ways:

1.  It can **use** objects from the namespace provided to it, such as
    incoming protocols and services or its own package contents.
2.  It can **publish** objects through its outgoing directory in the form of a
    namespace to other components.

### Environments

Environments construct namespaces.

An environment is a container of components.  Each environment is responsible
for _constructing_ the namespace for its components.

The environment decides what objects a component may access and how the
component's request for services by name will be bound to specific
implementations.

### Configuration

Components may have different kinds of configuration data exposed to them
depending on the features listed in their
[component manifest](/concepts/components/v1/component_manifests.md)
which are exposed as files in the `/config` namespace entry. These are
defined by the feature set of the component.

[glossary.protocol]: /glossary/README.md#protocol
[glossary.service]: /glossary/README.md#service
[hub]: /concepts/components/v2/hub.md
[protocol-capabilities]: /concepts/components/v2/capabilities/protocol.md
[service-capabilities]: /concepts/components/v2/capabilities/service.md
