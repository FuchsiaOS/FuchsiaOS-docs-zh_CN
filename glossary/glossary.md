# Glossary

The purpose of this document is to provide short definitions for a collection of
technical terms used in Fuchsia.

When adding new definitions, follow these guidelines:

-   A definition should provide a high-level description of a term and in most
    cases should not be longer than two or three sentences.
-   When another non-trivial technical term needs to be employed as part of the
    description, consider adding a definition for that term and linking to it
    from the original definition.
-   A definition should be complemented by a list of links to more detailed
    documentation and related topics.

-   /docs/development/debugger/README.md)



## Terms

### **ABI** {#abi}

The [Application Binary Interface](/docs/concepts/system/abi/system.md) (ABI)
for a system is the binary-level interface to the system. Typically you don't
write software that uses the system ABI directly. Instead, you write software
against the system API. When the software is compiled, the binary artifact
created by the compiler interfaces with the system through the ABI. Changes to
the system ABI may require you to recompile your source code to account for the
changes in the ABI.

### **ABR** {#ABR}

Fuchsia uses a bootloader A/B/R slot logic. This logic is not required to boot
Zircon or run a basic Fuchsia system, but it is required to take full
advantage of Fuchsia's over-the-air (OTA) update features.

The term slot is used to refer to a set of partitions that update together. If
an update fails, the partitions fall back together. This works because there
are two copies of each partition on disk. By convention, the slots used for
normal updates are denoted A and B, and the corresponding partitions are
labeled with a suffix of "_a" or "_b". The slot used for recovery is denoted
R which is chosen when neither A nor B is bootable.

### **Agent** {#agent}

A [Modular](/docs/concepts/modular/overview.md) concept that is being
deprecated.

An agent is a role a [component](#component) can play to execute in the
background in the context of a [session](#session). Agents are always a
singleton within a session, and provide services to other components.

See [Agent concept docs](/docs/concepts/modular/agent.md) for more.

### **AppMgr** {#appmgr}

The Application Manager (AppMgr) is responsible for launching
[v1](#components-v1) [components](#component) and managing the namespaces in
which those components run. It is run by [component manager](#component-manager)
as a [v2](#components-v2) component.

### **Banjo** {#banjo}

Banjo is a language for defining protocols that are used to communicate between
[drivers](#driver). It is different from [FIDL](#fidl) in that it specifies an
ABI for drivers to use to call into each other, rather than an IPC protocol.

### **Binding** {#binding}

In Components v2, a [component instance](#component-instance) _binds_ to another
component instance when it connects to a capability provided by the other
component instance. This is the most common reason for a component to start.

-   [Documentation](/docs/concepts/components/v2/lifecycle.md#binding)

### **bootfs** {#bootfs}

The bootfs RAM disk contains the files needed early in the boot process when no
other filesystems are available. It is part of the [ZBI](#zircon-boot-image),
and is decompressed and served by [bootsvc](#bootsvc). After the early boot
process is complete, the bootfs is mounted at `/boot`.

-   [Documentation](/docs/concepts/booting/userboot.md#BOOTFS)

### **bootsvc** {#bootsvc}

`bootsvc` is the second process started in Fuchsia. It provides a filesystem
service for the [bootfs](#bootfs) and a loader service that loads programs from
the same bootfs. After starting these services, it loads the third program,
which defaults to `component_manager`.

-   [Documentation](/docs/concepts/booting/bootsvc.md)

### **Bus Driver** {#bus-driver}

A [driver](#driver) for a device that has multiple children. For example,
hardware interfaces like PCI specify a topology in which a single controller is
used to interface with multiple devices connected to it. In that situation, the
driver for the controller would be a bus driver.

### **Cache directory** {#cache-directory}

Similar to a [data directory](#data-directory), except that the contents of a
cache directory may be cleared by the system at any time, such as when the
device is under storage pressure. Canonically mapped to /cache in the component
instance’s [namespace](#namespace).

-   [Testing isolated cache storage](/docs/concepts/testing/testing_isolated_cache_storage.md).

### **Capability** {#capability}

A capability is a value that combines an *object reference* and a set of
*rights*. When a program has a capability it is conferred the privilege to
perform certain actions using that capability. A [handle](#handle) is a common
example for a capability.

### **Capability routing** {#capability-routing}

A way for one [component](#component-instance) to give
[capabilities](#capability) to another instance over the
[component instance tree](#component-instance-tree).
[Component manifests](#component-manifest) define how routing takes place, with
syntax for [service capabilities](#service-capability),
[directory capabilities](#directory-capability), and
[storage capabilities](#storage-capability).

Capability routing is a [components v2](#components-v2) concept.

### **Carnelian** {#Carnelian}

A user-interface framework for Fuchsia components written in Rust.

Further details are available in [the source](/src/lib/ui/carnelian/README.md)

#### expose {#expose}

A [component instance](#component-instance) may use the `expose`
[manifest](#component-manifest) keyword to indicate that it is making a
capability available to its parent to route. Parents may [offer](#offer) a
capability exposed by any of their children to their other children or to their
parent, but they cannot [use](#use) it themselves in order to avoid dependency
cycles.

#### offer {#offer}

A [component instance](#component-instance) may use the `offer`
[manifest](#component-manifest) keyword to route a capability that was
[exposed](#expose) to it to one of its children (other than the child that
exposed it).

#### use {#use}

A [component instance](#component-instance) may use the `use`
[manifest](#component-manifest) keyword to consume a capability that was
[offered](#offer) to it by its parent.

### **Channel** {#channel}

A channel is an IPC primitive provided by Zircon. It is a bidirectional,
datagram-like transport that can transfer small messages including
[Handles](#handle). [FIDL](#fidl) protocols typically use channels as their
underlying transport.

-   [Channel Overview](/docs/reference/kernel_objects/channel.md)
-   [Update Channel Usage Policy](/docs/contribute/governance/policy/update_channel_usage_policy.md)

### **Component** {#component}

A component is a unit of executable software on Fuchsia. Components support
[capability routing](#capability-routing), software composition, isolation
boundaries, continuity between executions, and introspection.

-   [Components](/docs/concepts/components/v2)

### **Component collection** {#component-collection}

A node in the [component instance tree](#component-instance-tree) whose children
are dynamically instantiated rather than statically defined in a
[component manifest](#component-manifest).

Component collection is a [components v2](#components-v2) concept.

### **Component declaration** {#component-declaration}

A component declaration is a [FIDL](#fidl) table ([fuchsia.sys2.ComponentDecl])
that includes information about a [component](#component)’s runtime
configuration, [capabilities](#capabilities) it [exposes](#expose),
[offers](#offer), and [uses](#use), and [facets](#component-manifest-facet).

Component declaration is a [components v2](#components-v2) concept.

[fuchsia.sys2.ComponentDecl]: /sdk/fidl/fuchsia.sys2/decls/component_decl.fidl

### **Component Framework** {#component-framework}

An application framework for declaring and managing [components](#component),
consisting of build tools, APIs, conventions, and system services.

-   [Components v1](#components-v1), [Components v2](#components-v2)

### **Component instance** {#component-instance}

One of possibly many instances of a particular [component](#component) at
runtime. A component instance has its own [environment](#environment) and
[lifecycle](#lifecycle) independent of other instances.

### **Component instance tree** {#component-instance-tree}

A tree structure that represents the runtime state of parent-child relationships
between [component instances](#component-instance). If instance A launches
instance B then in the tree A will be the parent of B. The component instance
tree is used to route capabilities such that parents can [offer](#offer)
capabilities to their children to [use](#use), and children can
[expose](#expose) capabilities for their parents to expose to their parents or
offer to other children.

Component instance tree is a [components v2](#components-v2) concept.

### **Component Manager** {#component-manager}

A system service that lets [component instances](#component-instance) manage
their children and [routes capabilities](#capability-routing) between them, thus
implementing the [component instance tree](#component-instance-tree). Component
Manager is the system service that implements the
[components v2](#components-v2) runtime.

### **Component Manifest** {#component-manifest}

In [Components v1](#components-v1), a component manifest is a JSON file with a
`.cmx` extension that contains information about a [component](#component)’s
runtime configuration, services and directories it receives in its
[namespace](#namespace), and [facets](#component-manifest-facet).

In [Components v2](#components-v2), a component manifest is a file with a `.cm`
extension, that encodes a [component declaration](#component-declaration).

-   [Component manifests v1](/docs/concepts/components/v1/component_manifests.md)
-   [Component manifests v2](/docs/concepts/components/v2/component_manifests.md)

### **Component Manifest Facet** {#component-manifest-facet}

Additional metadata that is carried in a
[component manifest](#component-manifest). This is an extension point to the
[component framework](#component-framework).

### **Component Instance Identifier** {#component-instance-id}

A unique, stable identifer for a component instance. The instance id is _the_
canonical identifier for a component instance. The component runtime uses this
to key a component's persistent resources, if it has any. While a component
instance's moniker may change, its instance ID remains the same.

Instance IDs are assigned to component instances using a
[component ID index](/docs/development/components/component_id_index.md).

### **Component URL** {#component-url}

A [URL][component-url] that identifies a component, most often used when
instantiating a component, for example
`fuchsia-pkg://fuchsia.com/stash#meta/stash_secure.cm`.

See also: [fuchsia-pkg URL](#fuchsia-pkg-url)

[component-url]: /docs/concepts/components/component_urls.md

### **Components v1** {#components-v1}

A shorthand for the [Component](#component) Architecture as first implemented on
Fuchsia. Includes a runtime as implemented by [appmgr](#appmgr) and
[sysmgr](#sysmgr), protocols and types as defined in [fuchsia.sys], build-time
tools such as [cmc], and IDK libraries such as [libsys] and [libsvc].

See also: [Components v2](#components-v2)

[fuchsia.sys]: /sdk/fidl/fuchsia.sys/
[cmc]: /tools/cmc
[libsys]: /sdk/lib/sys
[libsvc]: /sdk/lib/svc

### **Components v2** {#components-v2}

A shorthand for the [Component](#component) Architecture in its modern
implementation. Includes a runtime as implemented by
[component_manager](#component-manager), protocols and types as defined in
[fuchsia.sys2], and build-time tools such as [cmc].

See also: [Components v1](#components-v1)

[fuchsia.sys2]: /sdk/fidl/fuchsia.sys2/
[cmc]: /tools/cmc

### **Concurrent Device Driver** {#concurrent-device-driver}

A concurrent device driver is a [hardware driver](#hardware-driver) that
supports multiple concurrent operations. This may be, for example, through a
hardware command queue or multiple device channels. From the perspective of the
[core driver](#core-driver), the device has multiple pending operations, each of
which completes or fails independently. If the driven device can internally
parallelize an operation, but can only have one operation outstanding at a time,
it may be better implemented with a
[sequential device driver](#sequential-device-driver).

### **Core Driver** {#core-driver}

A core driver is a [driver](#driver) that implements the application-facing RPC
interface for a class of drivers (e.g. block drivers, ethernet drivers). It is
hardware-agnostic. It communicates with a [hardware driver](#hardware-driver)
through [banjo](#banjo) to service its requests.

### **Data directory** {#data-directory}

A private directory within which a [component instance](#component-instance) may
store data local to the device, canonically mapped to /data in the component
instance’s [namespace](#namespace).

### **Directory capability** {#directory-capability}

A [capability](#capability) that permits access to a filesystem directory by
adding it to the [namespace](#namespace) of the
[component instance](#component-instance) that [uses](#use) it. If multiple
[component instances](#component-instance) are offered the same directory
capability then they will have access to the same underlying filesystem
directory.

Directory capability is a [components v2](#components-v2) concept.

-   [Capability routing](#capability-routing)

### **Driver** {#driver}

A driver is a dynamic shared library that [Driver Manager](#drivermanager) can
load into a [Driver Host](#driverhost) and that enables, and controls one or
more devices.

-   [Reference](/docs/development/drivers/developer_guide/driver-development.md)
-   [Driver Sources](/src/devices)

### **Driver Host** {#driverhost}

A Driver Host is a process containing one or more device drivers. They are
created by the Driver Manager, as needed, to provide isolation between drivers
for stability and security.

### **Driver Manager** {#drivermanager}

The Driver Manager (formerly devmgr or devcoordinator) is responsible for
enumerating, loading, and managing the life cycle of device drivers.

### **Element** {#element}

A [component](#component) added to a [session](#session) dynamically through the
[FIDL](#fidl) protocol [`ElementManager`](#element-manager). In addition to the
properties common to all components, Elements are also annotated by Element
Proposers. Those annotations are shared with other components within the
session.

It is the session's responsibility to manage the lifecycle of elements.

Elements are a [Session Framework](#session-framework) concept.

-   [Elements conceptual documentation](/docs/concepts/session/element.md)

### **Element Annotation** {#element-annotation}

A [FIDL](#fidl) struct (see [`fuchsia.session.Annotation`]).

A collection of named attributes associated with an [element](#element).
Annotations are specified at element creation time by
[element proposers](#element-proposer) and remain mutable for the lifetime of
the element.

Element annotations are a [Session Framework](#session-framework) concept.

[`fuchsia.session.Annotation`]: https://fuchsia.dev/reference/fidl/fuchsia.session#Annotation

### **Element Manager** {#element-manager}

A [FIDL](#fidl) protocol (see [`fuchsia.session.ElementManager`]).

A protocol used to grant [session](#session) sub-components the capability to
ask for [elements](#element) to be added to the product experience at runtime.

Element manager is a [Session Framework](#session-framework) concept.

[`fuchsia.session.ElementManager`]: https://fuchsia.dev/reference/fidl/fuchsia.session#ElementManager

### **Element Proposer** {#element-proposer}

A way to refer to any [component](#component) that invokes the ProposeElement()
method on [`ElementManager`](#element-manager) to launch a specific
[element](#element) in a [session](#session).

Element Proposer is a [Session Framework](#session-framework) concept.

### **Environment** {#environment}

A container for a set of components, which provides a way to manage their
lifecycle and provision services for them. All components in an environment
receive access to (a subset of) the environment's services.

### **Escher** {#escher}

Graphics library for compositing user interface content. Its design is inspired
by modern real-time and physically based rendering techniques though we
anticipate most of the content it renders to have non-realistic or stylized
qualities suitable for user interfaces.

### **FAR** {#far}

The Fuchsia Archive Format is a container for files to be used by Zircon and
Fuchsia.

-   [FAR Spec](/docs/concepts/source_code/archive_format.md)

### **FBL** {#fbl}

FBL is the Fuchsia Base Library, which is shared between kernel and userspace.

-   [Zircon C++](/docs/development/languages/c-cpp/cxx.md)

### **FDF** {#fdf}

The Fuchsia Driver Framework is the documentation, APIs, and ABIs necessary to
build Zircon Device Drivers. Device drivers are implemented as ELF shared
libraries loaded by Zircon's Driver Manager.

-   [FDF Overview](/docs/concepts/drivers/fdf.md)
-   [FDF includes](/src/lib/ddk/include/ddk/)

### **fdio** {#fdio}

`fdio` is the Zircon IO Library. It provides the implementation of posix-style
open(), close(), read(), write(), select(), poll(), etc, against the RemoteIO
RPC protocol. These APIs are return- not-supported stubs in libc, and linking
against libfdio overrides these stubs with functional implementations.

-   [Source](/sdk/lib/fdio/)

### **FIDL** {#fidl}

The Fuchsia Interface Definition Language (FIDL) is a language for defining
protocols that are typically used over [channels](#channel). FIDL is programming
language agnostic and has bindings for many popular languages, including C, C++,
Dart, Go, and Rust. This approach lets system components written in a variety of
languages interact seamlessly.

-   [FIDL](/docs/development/languages/fidl/)

### **Flutter** {#flutter}

[Flutter](https://flutter.dev/) is a functional-reactive user interface
framework optimized for Fuchsia and is used by many system components. Flutter
also runs on a variety of other platforms, including Android and iOS. Fuchsia
itself does not require you to use any particular language or user interface
framework.

### **FTP** {#ftp}

[FIDL Tuning Proposal](/docs/contribute/governance/deprecated-ftp-process.md). An FTP is the
way developers can suggest changes to FIDL. After being written, an FTP goes
through a formal review process where it is either accepted or rejected.

### **Fuchsia API Surface** {#fuchsia-api-surface}

The Fuchsia API Surface is the combination of the
[Fuchsia System Interface](#fuchsia-system-interface) and the client libraries
included in the [Fuchsia IDK](#fuchsia-idk).

### **Fuchsia emulator (FEMU)** {#fuchsia-emulator}

The [Fuchsia emulator (FEMU)](/docs/concepts/emulator/index.md) is the default
emulator for Fuchsia. It allows you to test Fuchsia components and applications
without needing a Fuchsia device. FEMU is based on the
[Android Emulator (AEMU)](https://developer.android.com/studio/run/emulator),
which is a fork of [QEMU](https://www.qemu.org/).

### **Fuchsia Package** {#fuchsia-package}

A Fuchsia Package is a unit of software distribution. It is a collection of
files, such as manifests, metadata, zero or more executables (e.g.
[Components](#component)), and assets. Individual Fuchsia Packages can be
identified using [fuchsia-pkg URLs](#fuchsia-pkg-url).

### **fuchsia-pkg URL** {#fuchsia-pkg-url}

The [fuchsia-pkg URL](/docs/concepts/packages/package_url.md) scheme is a means
for referring to a repository, a package, or a package resource. The syntax is
`fuchsia-pkg://<repo-hostname>[/<pkg-name>][#<path>]]`. E.g., for the component
`echo_client_dart.cmx` published under the package `echo_dart`'s `meta`
directory, from the `fuchsia.com` repository, its URL is
`fuchsia-pkg://fuchsia.com/echo_dart#meta/echo_client_dart.cmx`.

### **Fuchsia Integrator Development Kit (IDK)** {#fuchsia-idk}

The Fuchsia IDK is a collection of libraries and tools that the Fuchsia project
provides to Fuchsia developers. Among other things, the Fuchsia IDK contains a
definition of the [Fuchsia System Interface](#fuchsia-system-interface) as well
as a number of client libraries. The IDK is targeted at development environment
integrators that add environment specific tooling specific to the build
environment to form a full SDK.

### **Fuchsia System Interface** {#fuchsia-system-interface}

The [Fuchsia System Interface](/docs/concepts/system/abi/system.md) is the
binary interface that the Fuchsia operating system presents to software it runs.
For example, the entry points into the vDSO as well as all the FIDL protocols
are part of the Fuchsia System Interface.

### **Fuchsia Volume Manager** {#fuchsia-volume-manager}

Fuchsia Volume Manager (FVM) is a partition manager providing dynamically
allocated groups of blocks known as slices into a virtual block address space.
The FVM partitions provide a block interface enabling filesystems to interact
with it in a manner largely consistent with a regular block device.

-   [Filesystems](/docs/concepts/filesystems/filesystems.md)

### **GN** {#gn}

GN is a meta-build system that generates build files so that Fuchsia can be
built with [Ninja](#ninja). GN is fast and comes with solid tools to manage and
explore dependencies. GN files, named `BUILD.gn`, are located all over the
repository.

-   [Language and operation](https://gn.googlesource.com/gn/+/HEAD/docs/language.md)
-   [Reference](https://gn.googlesource.com/gn/+/HEAD/docs/reference.md)
-   [Fuchsia build overview](/docs/concepts/build_system/fuchsia_build_system_overview.md)

### **GraphicalPresenter** {#graphical-presenter}

A [FIDL](#fidl) protocol (see [`fuchsia.element.GraphicalPresenter`]).

A `GraphicalPresenter` organizes and presents graphical views.

The presented views can be annotated with
[Element Annotations](#element-annotation) to communicate presentation
properties to the `GraphicalPresenter`. This protocol is used, for example, when
a [session component](#session) written in Rust wants to delegate presentation
logic to a child [component](#component) written in Flutter, or when a session
component that manages the lifecycle of elements delegates the presentation of
element views to a child component that implements `GraphicalPresenter`.

For more information, see the
[GraphicalPresenter concept documentation](/docs/concepts/session/graphical_presenter.md).

`GraphicalPresenter` is a [Session Framework](#session-framework) concept.

[`fuchsia.element.GraphicalPresenter`]: https://fuchsia.dev/reference/fidl/fuchsia.element#GraphicalPresenter

### **Handle** {#handle}

A Handle is how a userspace process refers to a [kernel object](#kernel-object).
They can be passed to other processes over [Channels](#channel).

-   [Reference](/docs/concepts/kernel/handles.md)

### **Hardware Driver** {#hardware-driver}

A hardware driver is a [driver](#driver) that controls a device. It receives
requests from its [core driver](#core-driver) and translates them into
hardware-specific operations. Hardware drivers strive to be as thin as possible.
They do not support RPC interfaces, ideally have no local worker threads (though
that is not a strict requirement), and some will have interrupt handling
threads. They may be further classified into
[sequential device drivers](#sequential-device-driver) and
[concurrent device drivers](#concurrent-device-driver).

### **Hub** {#hub}

The hub is a portal for tools to access detailed structural information about
component instances at runtime, such as their names, job and process ids, and
exposed capabilities.

-   [Hub](/docs/concepts/components/v2/hub.md)

### **Input pipeline client library** {#input-pipeline-client-library}

A client library available to [session](#session) authors to simplify the
consumption and routing of input events from physical hardware.

-   [Input client library](/docs/concepts/session/input.md)

Input pipeline is a [Session Framework](#session-framework) concept.

### **Input pipeline InputDeviceBinding** {#input-pipeline-input-device-binding}

A Rust trait in the
[input pipeline client library](#input-pipeline-client-library).

An `InputDeviceBinding` represents a connection to a physical input device (e.g.
mouse, keyboard) in an input pipeline. An `InputDeviceBinding` does the
following:

1.  Connects to an [`InputReport`](#input-report) file located at
    `/dev/class/input-report/XXX`.
2.  Generates [`InputEvent`s](#input-pipeline-input-event) from the
    `DeviceDescriptor` and incoming [`InputReport`s](#input-report).

The input pipeline creates and owns `InputDeviceBinding`s as new input
peripherals are connected to a device.

`InputDeviceBinding` is a [Session Framework](#session-framework) concept.

### **Input pipeline DeviceDescriptor** {#input-pipeline-device-descriptor}

A property of the Rust struct [`InputEvent`](#input-pipeline-input-event) of
type [`enum InputDeviceDescriptor`] and a [FIDL](#fidl) table (see
[`fuchsia.input.report.DeviceDescriptor`]).

An `InputDeviceDescriptor` describes the ranges of values a particular input
device can generate. For example, a `InputDeviceDescriptor::Keyboard` contains
the keys available on the keyboard, and a `InputDeviceDescriptor::Touch`
contains the maximum number of touch contacts and the range of `x-` and
`y-`values each contact can take on.

[`enum InputDeviceDescriptor`]: /src/session/lib/input/src/input_device.rs
[`fuchsia.input.report.DeviceDescriptor`]: https://fuchsia.dev/reference/fidl/fuchsia.input.report#DeviceDescriptor

`InputDeviceDescriptor` is a [Session Framework](#session-framework) concept.

### **Input pipeline InputDeviceEvent** {#input-pipeline-input-device-event}

A property of the Rust struct [`InputEvent`](#input-pipeline-input-event).

An `InputDeviceEvent` represents an input event from an input device.
`InputDeviceEvent`s contain more context than the raw
[`InputReport`s](#input-report) they are parsed from. For example,
`InputDeviceEvent::Keyboard` contains all the pressed keys, as well as the key's
phase (pressed, released, etc.).

`InputDeviceEvent` is a [Session Framework](#session-framework) concept.

### **Input pipeline InputEvent** {#input-pipeline-input-event}

A Rust struct in the
[input pipeline client library](#input-pipeline-client-library).

An event from an input device containing context (a `InputDeviceDescriptor`) and
state (e.g. phase and location of a button press). The input pipeline generates
`InputEvent`s from hardware signals.

`InputEvent` is a [Session Framework](#session-framework) concept.

### **Input pipeline InputHandler** {#input-pipeline-input-handler}

A Rust trait in the
[input pipeline client library](#input-pipeline-client-library).

An `InputHandler` represents a client of
[`InputEvent`s](#input-pipeline-input-event) in an input pipeline. When an
`InputHandler` receives an [`InputEvent`](#input-pipeline-input-event), it does
at least one of the following:

1.  Forwards the [`InputEvent`](#input-pipeline-input-event) to the relevant
    client component.
2.  Outputs a vector of [`InputEvent`s](#input-pipeline-input-event) for the
    next `InputHandler` to process.

`InputHandler` is a [Session Framework](#session-framework) concept.

### **InputReport** {#input-report}

A [FIDL](#fidl) struct (see [`fuchsia.ui.InputReport`]).

A stateless representation of an event from a physical input device. Zircon
generates `InputReport`s from HID Reports.

[`fuchsia.ui.InputReport`]: https://fuchsia.dev/reference/fidl/fuchsia.ui.input#InputReport

### **Jiri** {#jiri}

Jiri is a tool for multi-repo development. It is used to checkout the Fuchsia
codebase. It supports various subcommands, which makes it easy for developers to
manage their local checkouts.

-   [Reference](https://fuchsia.googlesource.com/jiri/+/HEAD/README.md)
-   [Sub commands](https://fuchsia.googlesource.com/jiri/+/HEAD/README.md#main-commands-are)
-   [Behaviour](https://fuchsia.googlesource.com/jiri/+/HEAD/behaviour.md)
-   [Tips and tricks](https://fuchsia.googlesource.com/jiri/+/HEAD/howdoi.md)

### **Job** {#job}

A Job is a [kernel object](#kernel-object) that groups a set of related
[processes](#process), their child processes, and their jobs (if any). Every
process in the system belongs to a job and all jobs form a single rooted tree.

-   [Job Overview](/docs/reference/kernel_objects/job.md)

### **Kernel Object** {#kernel-object}

A kernel object is a kernel data structure that is used to regulate access to
system resources such as memory, i/o, processor time and access to other
processes. Userspace can only reference kernel objects via [Handles](#handle).

-   [Reference](/docs/reference/kernel_objects/objects.md)

### **KOID** {#koid}

A Kernel Object Identifier.

-   [Kernel Object](#kernel-object)

### **LK** {#lk}

Little Kernel (LK) is the embedded kernel that formed the core of the Zircon
Kernel. LK is more microcontroller-centric and lacks support for MMUs,
userspace, system calls -- features that Zircon added.

-   [LK on Github](https://github.com/littlekernel/lk)

### **Module** {#module}

A [Modular](/docs/concepts/modular/overview.md) concept that is being
deprecated.

A module is a role a [component](#Component) can play to contribute UI to a user
experience container (story) within a Modular session. Any component that
exports a Scenic `ViewProvider` can be be used as a module.

See [Module concept docs](/docs/concepts/modular/module.md) for more.

### **Moniker** {#moniker}

A [moniker][moniker] identifies a specific component instance in the component
tree using a topological path.

A v1 component's moniker is defined as a tuple of (path to the component's
[realm](#realm), component URL).

A v2 component's moniker is defined as a path to the component instance in the
[component instance tree](#component-instance-tree).

[moniker]: /docs/concepts/components/v2/monikers.md

### **Musl** {#musl}

Fuchsia's standard C library (libc) is based on Musl Libc.

-   [Source](/zircon/third_party/ulib/musl/)
-   [Musl Homepage](https://www.musl-libc.org/)

### **Namespace** {#namespace}

A namespace is the composite hierarchy of files, directories, sockets,
[service](#service)s, and other named objects offered to components by their
[environment](#environment).

-   [Fuchsia Namespace Spec](/docs/concepts/process/namespaces.md)

### **Netstack** {#netstack}

An implementation of TCP, UDP, IP, and related networking protocols for Fuchsia.

### **Ninja** {#ninja}

Ninja is the build system executing Fuchsia builds. It is a small build system
with a strong emphasis on speed. Unlike other systems, Ninja files are not
supposed to be manually written but should be generated by other systems, such
as [GN](#gn) in Fuchsia.

-   [Manual](https://ninja-build.org/manual.html)
-   [Ninja rules in GN](https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#ninja_rules)
-   [Fuchsia build overview](/docs/concepts/build_system/fuchsia_build_system_overview.md)

### **OpaqueTest** {#opaque-test}

`OpaqueTest` is a Rust client-side library that sets up hermetic tests for a v2
component.

-   [Hermetic testing using OpaqueTest](/docs/concepts/components/v2/opaque_test.md)

### **Outgoing directory** {#outgoing-directory}

A file system directory where a [component](#component) may [expose](#expose)
capabilities for others to use.

### **Paver** {#paver}

A tool in Zircon that installs partition images to internal storage of a device.

-   [Guide for installing Fuchsia with paver](/docs/development/hardware/paving.md).

### **Platform Source Tree** {#platform-source-tree}

The Platform Source Tree is the open source code hosted on
fuchsia.googlesource.com, which comprises the source code for Fuchsia. A given
Fuchsia system can include additional software from outside the Platform Source
Tree by adding the appropriate [Fuchsia Package](#fuchsia-package).

### **Process** {#process}

A Process is a [kernel object](#kernel-object) that represents an instance of a
program as a set of instructions that are executed by one or more
[threads](#thread) together with a collection of [capabilities](#capability).
Every process is contained in a [job](#job).

-   [Process Overview](/docs/reference/kernel_objects/process.md)

### **Protocol** {#protocol}

In [FIDL](#fidl), a protocol groups methods and events to describe how one
process interacts with another.

In [components v1](#components-v1), a component may access a protocol (called a
"service" in v1) from its [environment](#environment) through its
[namespace](#namespace) by naming the protocol in its services whitelist.

In [components v2](#components-v2), a protocol is used and routed to other
components as a [protocol capability](#protocol-capability).

### **Protocol capability** {#protocol-capability}

A [capability](#capability) that permits communicating with a
[protocol](#protocol) over a [channel](#channel) using a specified [FIDL](#fidl)
protocol. The server end of the channel is held by the
[component instance](#component-instance) that provides the capability. The
client end of the channel is given to the
[component instance](#component-instance) that [uses](#use) the capability.

-   [Capability routing](#capability-routing)

Protocol capability is a [components v2](#components-v2) concept.

### **Realm** {#realm}

In [components v1](#components-v1), realm is synonymous to
[environment](#environment).

In [components v2](#components-v2), a realm is a subtree of component instances
in the [component instance tree](#component-instance-tree). It acts as a
container for component instances and capabilities in the subtree.

### **Runner** {#runner}

A [component](#component) that provides a runtime environment for other
components, e.g. the ELF runner, the Dart AOT runner, the Chromium web runner.

Every component needs a runner in order to launch. Components express their
dependency on a runner in the component's [declaration](#component-declaration).

When the [component framework](#component-framework) starts a component, it
first determines the capabilities that the component should receive, then asks
the component's runner to launch the component. The runner is responsible for
creating any necessary processes, loading executable code, initializing language
runtimes, handing control to the component's entry points, and terminating the
component when requested by the component framework.

-   [ELF runner](/docs/concepts/components/v2/elf_runner.md)
-   [Component runner](/docs/concepts/components/v2/capabilities/runners.md)

### **Scenic** {#scenic}

Scenic is a system service that composes graphical objects from multiple
processes into a shared scene graph. Scenic includes views, input, compositor,
and GPU services.

### **Sequential Device Driver** {#sequential-device-driver}

A sequential device driver is a [hardware driver](#hardware-driver) that will
only service a single request at a time. The [core driver](#core-driver)
synchronizes and serializes all requests.

### **Service** {#service}

In [FIDL](#fidl), a service groups [protocols](#protocol) to describe how one
process interacts with another.

Services can be used and provided to other components by
[routing](#capability-routing) [service capabilities](#service-capability).

-   [Service overview](/docs/concepts/components/v2/services.md)

### **Service capability** {#service-capability}

A [capability](#capability) that permits communicating with a
[service](#service) over a [channel](#channel) using a specified [FIDL](#fidl)
service. The server end of the channel is held by the
[component instance](#component-instance) that provides the capability. The
client end of the channel is given to the
[component instance](#component-instance) that [uses](#use) the capability.

-   [Capability routing](#capability-routing)

Service capability is a [components v2](#components-v2) concept.

### **Session** {#session}

A session is a [component](#component) that encapsulates a product’s user
experience. It is the first product-specific component started on boot after the
[Session Manager](#session-manager). Sessions typically utilize aspects of the
[Session Framework](#session-framework) during their development, in automated
testing, and at runtime. At runtime, there is only one session component, but it
can be composed of many sub-components. For example, the session for a graphical
product instantiates Scenic (graphics) as a child component.

Session is a [Session Framework](#session-framework) concept.

### **Session Framework** {#session-framework}

The session framework is a framework for building products on Fuchsia. The
framework provides software libraries, FIDL protocols, developer tools, and
standards that are composed to create a particular product’s user experience.

See the session framework
[conceptual documentation](/docs/concepts/session/introduction.md).

### **Session Launcher** {#session-launcher}

A [FIDL](#fidl) protocol (see [`fuchsia.session.Launcher`]).

A protocol, exposed by the [Session Manager](#session-manager), used to start or
restart [sessions](#session). It is available to developer tools such as
`session_control` to control session components at runtime.

Session Launcher is a [Session Framework](#session-framework) concept.

[`fuchsia.session.Launcher`]: https://fuchsia.dev/reference/fidl/fuchsia.session#Launcher

### **Session Manager** {#session-manager}

The platform component, started late in the Fuchsia boot sequence, that manages
the lifecycle of the [session](#session). The session manager defines the set of
system capabilities provided to sessions at runtime.

Session Manager is a [Session Framework](#session-framework) concept.

### **Storage capability** {#storage-capability}

A storage capability is a [capability](#capability) that allocates per-component
isolated storage for a designated purpose within a filesystem directory.
Multiple [component instances](#component-instance) may be given the same
storage capability, but underlying directories that are isolated from each other
will be allocated for each individual use. This is different from
[directory capabilities](#directory-capability), where a specific filesystem
directory is routed to a specific component instance.

Isolation is achieved because Fuchsia does not support
[dotdot](/docs/concepts/filesystems/dotdot.md).

There are three types of storage capabilities:

-   *data*: a directory is added to the [namespace](#namespace) of the
    [component instance](#component-instance) that [uses](#use) the capability.
    Acts as a [data directory](#data-directory).
-   *cache*: same as data, but acts as a [cache directory](#cache-directory).
-   *meta*: a directory is allocated to be used by component manager, where it
    will store metadata to enable features like persistent
    [component collections](#component-collection).

Storage capability is a [components v2](#components-v2) concept.

-   [Capability routing](#capability-routing)
-   [Storage capabilities](/docs/concepts/components/v2/capabilities/storage.md)

### **Thread** {#thread}

A Thread is a [kernel object](#kernel-object) that represents a time-shared CPU
execution context. Each thread is contained in a [process](#process).

-   [Thread Overview](/docs/reference/kernel_objects/thread.md)

### **userboot** {#userboot}

userboot is the first process started by the Zircon kernel. It is loaded from
the kernel image in the same way as the [vDSO](#virtual-dynamic-shared-object),
instead of being loaded from a filesystem. Its primary purpose is to load the
second process, [bootsvc](#bootsvc), from the [bootfs](#bootfs).

-   [Documentation](/docs/concepts/booting/userboot.md)

### **ViewController** {#view-controller}

A [FIDL](#fidl) protocol (see [`fuchsia.element.ViewController`]).

A `ViewController` represents a handle to a remote View that was launched by
calling `PresentView()` on a [`GraphicalPresenter`](#graphical-presenter). With
this handle, the caller (often, the session) can control the View's lifecycle. Closing the
`ViewController` should close the presented view, and allow the system to
reclaim its resources.

[`fuchsia.element.ViewController`]: https://fuchsia.dev/reference/fidl/fuchsia.element#ViewController

### **ViewHolderToken** {#view-holder-token}

A [FIDL](#fidl) struct (see [`fuchsia.ui.views.ViewHolderToken`]).

A `ViewHolderToken` uniquely identifies an attachment point for a View in the
global scene graph. Each `ViewHolderToken` has exactly one corresponding
[`ViewToken`](#view-token).

[`fuchsia.ui.views.ViewHolderToken`]: https://fuchsia.dev/reference/fidl/fuchsia.ui.views#ViewHolderToken

### **ViewRef** {#view-ref}

A [FIDL](#fidl) struct (see [`fuchsia.ui.views.ViewRef`]).

A `ViewRef` is a handle to a kernel object that identifies a unique View across
the system. Two `ViewRef`s to the same View have the same [KOID](#koid).

[`fuchsia.ui.views.ViewRef`]: https://fuchsia.dev/reference/fidl/fuchsia.ui.views#ViewRef

### **ViewSpec** {#view-spec}

A [FIDL](#fidl) table (see [`fuchsia.element.ViewSpec`]).

A `ViewSpec` is a description of a view to be presented by a
[`GraphicalPresenter`](#graphical-presenter).

[`fuchsia.element.ViewSpec`]: https://fuchsia.dev/reference/fidl/fuchsia.element#ViewSpec

### **ViewToken** {#view-token}

A [FIDL](#fidl) struct (see [`fuchsia.ui.views.ViewToken`]).

A `ViewToken` uniquely identifies a View, which is the root point for a subgraph
in the global scene graph. Each `ViewToken` has exactly one corresponding
[`ViewHolderToken`](#view-holder-token).

[`fuchsia.ui.views.ViewToken`]: https://fuchsia.dev/reference/fidl/fuchsia.ui.views#ViewToken

### **Virtual Dynamic Shared Object** {#virtual-dynamic-shared-object}

The Virtual Dynamic Shared Object (vDSO) is a Virtual Shared Library -- it is
provided by the [Zircon](#zircon) kernel and does not appear in the filesystem
or a package. It provides the Zircon System Call API/ABI to userspace processes
in the form of an ELF library that's "always there." In the Fuchsia IDK and
[Fuchsia Driver Framework](#fdf) it exists as `libzircon.so` for the purpose of
having something to pass to the linker representing the vDSO.

### **Virtual Memory Address Range** {#virtual-memory-address-range}

A Virtual Memory Address Range (VMAR) is a Zircon
[kernel object](#kernel-object) that controls where and how
[Virtual Memory Objects](#virtual-memory-object) may be mapped into the address
space of a process.

-   [VMAR Overview](/docs/reference/kernel_objects/vm_address_region.md)

### **Virtual Memory Object** {#virtual-memory-object}

A Virtual Memory Object (VMO) is a Zircon [kernel object](#kernel-object) that
represents a collection of pages (or the potential for pages) that may be read,
written, mapped into the address space of a process, or shared with another
process by passing a [Handle](#handle) over a [Channel](#channel).

-   [VMO Overview](/docs/reference/kernel_objects/vm_object.md)

### **Zircon Boot Image** {#zircon-boot-image}

A Zircon Boot Image (ZBI) contains everything needed during the boot process
before any drivers are working. This includes the kernel image and a
[RAM disk for the boot filesystem](#bootfs).

-   [ZBI header file](/zircon/system/public/zircon/boot/image.h)

### **Zedboot** {#zedboot}

Zedboot is a recovery image that is used to install and boot a full Fuchsia
system. Zedboot is actually an instance of the Zircon kernel with a minimal set
of drivers and services running used to bootstrap a complete Fuchsia system on a
target device. Upon startup, Zedboot listens on the network for instructions
from a bootserver that may instruct Zedboot to [install](#paver) a new OS. Upon
completing the installation Zedboot will reboot into the newly installed system.

### **Zircon** {#zircon}

Zircon is the [microkernel](https://en.wikipedia.org/wiki/Microkernel) and
lowest level userspace components (driver runtime environment, core drivers,
libc, etc) at the core of Fuchsia. In a traditional monolithic kernel, many of
the userspace components of Zircon would be part of the kernel itself.

-   [Zircon Documentation](/zircon/README.md)
-   [Zircon Concepts](/docs/concepts/kernel/concepts.md)
-   [Source](/zircon)

### **ZX** {#zx}

ZX is an abbreviation of "Zircon" used in Zircon C APIs/ABIs
(`zx_channel_create()`, `zx_handle_t`, `ZX_EVENT_SIGNALED`, etc) and libraries
(libzx in particular).

### **ZXDB** {#zxdb}

The native low-level system debugger.

-   [Reference](