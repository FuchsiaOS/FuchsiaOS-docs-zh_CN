<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0210" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This document describes how we will route the ability to create virtual machines
into a product session. By association, this document will also describe the set
of virtualization features and APIs that are included in the Fuchsia platform
and what components are provided by a product.

## Motivation

All virtualization components are currently provided by the Fuchsia core
platform today, including:

* Components for emulating the virtual machine and virtual devices
* Packages that provide specific guest Operating System binaries
* Components that integrate guest-specific features with products

This has some notable downsides. By placing all of the guest-specific components
in the platform we are making it impossible for products to integrate
virtualization in product specific ways. Additionally many guests will retain
state and hold user data and we need to ensure that this user data is encrypted
and protected along with all other user data. This document describes a design
which is product agnostic, but much of the design will consider requirements of
the workstation product as a motivating example.

An explicit goal of this proposal is to enable virtualization to be provided as
a Fuchsia platform feature. This means that we will expose a set of capabilities
to the product session that allows products to utilize the mechanism of
virtualization without needing to route all the low-level privileged
capabilities that virtualizations needs into the session itself. This is, in
part, based  the guidance in [RFC-0092 -
Sessions](/docs/contribute/governance/rfcs/0092_sessions.md) and [RFC-0194 -
Addendum: Sessions](/docs/contribute/governance/rfcs/0194_addendum_sessions.md).
For more details on this see [Security
Considerations](#security_considerations).


## Stakeholders

Who has a stake in whether this RFC is accepted? (This section is optional but
encouraged.)

_Facilitator:_

The person appointed by FEC to shepherd this RFC through the RFC process.

_Reviewers:_

- abdulla
- alerad
- dahastin
- jsankey
- aaronwood
- ypomortsev

_Consulted:_

List people who should review the RFC, but whose approval is not required.

_Socialization:_

A draft of this plan was socialized within stakeholders from Virtualization,
Component Framework, and Security.

## Glossary

* Virtual Machine Manager (vmm): A component that is responsible for emulating
  and driving a virtual machine.
* Virtual Device: A component responsible for emulating a single device (ex:
  block, network, etc).
* vsock: A paravirtualized socket device that allows for 0-configuration socket
  connections between a guest and host. Commonly used as a control plane for
  guests.

## Background

Virtual machines on Fuchsia are driven by the vmm (virtual machine manager)
component. The `vmm` component itself manages several sub-components which
implement virtual devices (ex: `virtio_block.cm`, `virtio_net.cm`, etc), however
this document will refer to all of the entire group of components as simply the
`vmm`. The `vmm` will provide the virtual machine with basic resources (memory,
vcpus) and virtual devices. The `vmm` component is guest-agnostic and does not
have any guest-OS specific logic. Each `vmm` component instance is managed by a
`GuestManager` component specific to the guest-OS, for example
`TerminaGuestManager`. The `GuestManager`. contains higher level logic for the
guest lifecycle and any guest-specific services or functionality. The
`GuestManager` is responsible for providing all the boot resources (kernel,
ramdisk, block devices, etc) as well as guest configuration, such as which
devices to provide, how many virtual-CPUs or how much memory to provide.

Each guest operating system supported on Fuchsia is currently modeled entirely
using static component routes. For example, the `TerminaGuestManager` component
(the guest that powers the Linux Terminal on workstation) has a static child
`vmm` component provided by a core shard. There are routes that allow the Linux
Terminal component in the session to connect to the `TerminaGuestManager` in the
core realm, and routes that allow the guest to create windows in the graphical
shell (using `virtio-wayland`).

![Diagram showing current capability routing][image.static_routing]

### Storage

The `GuestManager` components are responsible for opening all the files and
devices that are needed by the `vmm`; `vmm` itself only receives handles to
files, block devices, or other stateful capabilities. The `TerminaGuestManager`
will initialize the stateful partition by creating a file in a directory
provided by the CFv2 `data` storage capability. The other read-only stateful
partitions are provided by opening the blobfs files for the images included in
the `TerminaGuestManager` package itself.

Because the `TerminaGuestManager` is a component in the core realm, the `data`
storage capability that will be provided to it is also sourced from the core
realm. This implies that on products such as workstation where there is a
separate account volume encrypted based on user authentication factors, the
guest data storage will not be located on this volume, even though it may
contain sensitive user data. Additionally, if we imagine a multi-user system,
having a single component in the core realm would mean a single `vmm` instance
and data would be shared across all accounts, which is not a tenable solution.

## Design

Keeping the `vmm` component in core is desirable since it requires multiple,
privileged capabilities. Additionally, since the `vmm` provides the mechanism to
launch virtual machines there is already no product-specific policy or behavior
in the `vmm`. Extracting the `GuestManager` components from core is desirable
because these are product-specific packages which bring the logic and guest
operating systems that are only relevant to a specific product. By moving those
components out of `core` we can give the product the freedom to customize how
virtualization is integrated in a product-specific way without needing to handle
the low level details of virtual machine emulation.

We will introduce a new core shard that allows sessions to create `vmm`
component instances in a CFv2 collection, but any state must be provided and
managed by the client. We will then provide the ability to launch a virtual
machine using capability routing instead of using a `vmm.cm` a static child of
each `GuestManager` component. This allows us to move the `GuestManager`
components into the session without also requiring that the vmm itself be moved
into the session. In other words, we keep the `vmm`, which is product and guest
agnostic in `core` and we move the product-specific `GuestManagers` into the
session.

To do this, we will introduce a new `VmmLauncher` component to handle creating
`vmm` components. This component will expose the same `GuestLifecycle` protocol
that is exposed by the `vmm`. The difference is that instead of implementing
`GuestLifecycle` directly, `VmmLauncher` will create a new `vmm` instance for
each `GuestLifecycle` connection and forward the server end of the FIDL channel
to the new component. The `GuestLifecycle` protocol is used to initialize the VM
and then start execution.

![Diagram showing proposed capability routing][image.vmm_launcher_routing]

```fidl
/// The guest control plane allows for creating, starting, and stopping the guest.
protocol GuestLifecycle {
    /// Create a VMM configured with the provided config. This instantiates all
    /// devices and loads the kernel without starting the VCPU or device dispatch
    /// loops.
    ///
    /// `Create` must not be called after a call to `Run` until `Stop` is called.
    /// Once a guest has been stopped with a call to `Stop`, then `Create` may be
    /// called again to re-initialize the guest.
    Create(resource struct {
        guest_config GuestConfig;
    }) -> (struct {}) error GuestError;

    /// Binds to the Guest protocol for an initialized guest.
    ///
    /// This operation must be called between `Create` and `Stop`, otherwise
    /// the provided channel will be immediately closed.
    Bind(resource struct {
        guest server_end:Guest;
    }) -> (struct {}) error GuestError;

    /// Start the VCPU and device dispatch loops. This will not return until the
    /// dispatch loops exit. On a clean shutdown (either guest or client initiated)
    /// this will return success.
    ///
    /// If forced to stop by the guest manager calling stop, a SHUTDOWN_FORCED
    /// error will be returned. This will also return any runtime error that forces
    /// the guest to stop.
    ///
    /// `Run` must only be called after a call to `Create`. Once a guest is stopped with
    /// a call to `Stop`, then `Run` may not be called again until `Create` is called
    /// to re-initialize the guest. Notably, we do not support calling `Stop`
    /// and then `Run` directly; the call to `Create` after `Stop` is a requirement.
    Run() -> (struct {}) error GuestError;

    /// Stop a running VMM. Returns once the dispatch loops have stopped. After
    /// Stop returns, `Create` and then `Run` can be called again.
    Stop() -> ();
};

type GuestError = strict enum {
    /// Catch all VMM error.
    INTERNAL_ERROR = 1;

    /// A device endpoint was requested via the guest client API, but the device isn't enabled.
    DEVICE_NOT_PRESENT = 2;

    /// The config failed VMM validation for reasons such as a missing required field.
    BAD_CONFIG = 3;

    /// The VMM failed to initialize the guest object, usually due to capability routing issues
    /// or memory layout problems.
    GUEST_INITIALIZATION_FAILURE = 4;

    /// The VMM failed to initialize a device.
    DEVICE_INITIALIZATION_FAILURE = 5;

    /// The VMM failed to start a device, usually because the device component returned a failure.
    DEVICE_START_FAILURE = 6;

    /// Two or more devices have attempted to register overlapping memory ranges.
    DEVICE_MEMORY_OVERLAP = 7;

    /// Failed to connect to a required service. Check the routing in the manifest.
    FAILED_SERVICE_CONNECT = 8;

    /// Failed to add a public service.
    DUPLICATE_PUBLIC_SERVICES = 9;

    /// General error when loading the guest kernel.
    KERNEL_LOAD_FAILURE = 10;

    /// Error when starting a VCPU.
    VCPU_START_FAILURE = 11;

    /// A VCPU encountered a fatal error while running.
    VCPU_RUNTIME_FAILURE = 12;

    /// The VMM was asked to run before it was created.
    NOT_CREATED = 13;

    /// A VMM is already running. The VMM must be stopped and a new VMM must be created before it
    /// can be run again.
    ALREADY_RUNNING = 14;

    /// A running VMM was forced to stop by the VMM controller.
    CONTROLLER_FORCED_HALT = 15;
};
```

The set of parameters exposed to the client for configuration are included in
the `GuestConfig` FIDL table. This allows for configuring the machine shape
(vcpu and memory) and which virtual devices are available.

```fidl
type GuestConfig = resource table {
    /// Type of kernel to load.
    1: kernel_type KernelType;
    /// File to load the kernel from.
    2: kernel client_end:fuchsia.io.File;
    /// File to load the initial RAM disk from.
    3: ramdisk client_end:fuchsia.io.File;
    /// File to load the dtb overlay for a Linux kernel from.
    4: dtb_overlay client_end:fuchsia.io.File;
    /// Kernel command-line to use.
    5: cmdline string:MAX;
    /// Additional kernel command-lines to append to the main command-line.
    6: cmdline_add vector<string:MAX>:MAX;
    /// The number of CPUs to provide to a guest.
    7: cpus uint8;
    /// Amount of guest memory required, in bytes. This value may be rounded up
    /// depending on the system configuration.
    8: guest_memory uint64;
    /// A list of block devices to give a guest. Cannot be changed from the
    /// command-line.
    9: block_devices vector<BlockSpec>:MAX_BLOCK_DEVICES;
    /// A list of specifications for network devices.
   10: net_devices vector<NetSpec>:MAX_NET_DEVICES;
    /// Optional virtio-wl device.
   11: wayland_device WaylandDevice;
    /// Optional virtio-magma device.
   12: magma_device MagmaDevice;
    /// Whether to add a default network device.
   13: default_net bool;
    /// Enable virtio-balloon.
   14: virtio_balloon bool;
    /// Enable virtio-console.
   15: virtio_console bool;
    /// Enable virtio-gpu.
   16: virtio_gpu bool;
    /// Enable virtio-rng.
   17: virtio_rng bool;
    /// Enable virtio-vsock.
   18: virtio_vsock bool;
    /// Enable virtio-sound.
   19: virtio_sound bool;
    /// Enable input streams (capture) for virtio-sound.
   20: virtio_sound_input bool;
    /// Host ports to listen for guest initiated vsock connections on. This can be
    /// used for simplicity if a Listener is known at config creation time, or if a
    /// Listener must be available at the moment of guest creation for timing
    /// reasons.
    /// To add a Listener after a guest starts, see HostVsockEndpoint::Listen.
   21: vsock_listeners vector<Listener>:MAX;
};
```

The `Guest` protocol provides access to runtime services from the VM.

```fidl
/// A `Guest` provides access to services of a guest instance.
protocol Guest {
    /// Get a guest console.
    ///
    /// The details regarding what output is produced and what input is accepted
    /// are determined by each guest, but will typically be a read/write socket
    /// with a shell.
    ///
    /// Returns ZX_ERR_UNAVAILABLE if the guest has no configured console.
    GetConsole() -> (resource struct {
        socket zx.handle:SOCKET;
    }) error zx.status;

    /// Get the socket for low-level guest debug logs.
    ///
    /// The details regarding what output is produced and what input is accepted
    /// are determined by each guest, but will typically be a read-only socket
    /// with the guest kernel's serial logs.
    GetSerial() -> (resource struct {
        socket zx.handle:SOCKET;
    }) error zx.status;

    /// Get the vsock endpoint for the guest.
    ///
    /// This endpoint can be used to register listeners for guest initiated
    /// connections, and to initiate connections from a client. If listeners need
    /// to be registered before the guest starts so that they are immediately
    /// available, set them via the `GuestConfig` instead of using this protocol.
    ///
    /// Returns error VSOCK_NOT_PRESENT if the guest was started without a vsock
    /// device.
    GetHostVsockEndpoint(resource struct {
        endpoint server_end:HostVsockEndpoint;
    }) -> (struct {}) error GuestError;

    /// Get the balloon controller endpoint for the guest.
    ///
    /// Returns error BALLOON_NOT_PRESENT if the guest was started without a
    /// balloon device.
    GetBalloonController(resource struct {
        controller server_end:BalloonController;
    }) -> (struct {}) error GuestError;
};
```

## Implementation

We will implement `VmmLauncher` with a component in core, called
`vmm_launcher.cm`. This component will manage a CFv2 collection that it will
launch `vmm` components into. The `GuestManager` components will then be updated
to access the `GuestLifecycle` capability from their `"parent"` instead of
through a static child. `vmm_launcher.cm` will be provided using a core-shard at
`/core/virtualization` in the CFv2 component hierarchy. We choose to expose this
as a `virtualization` shard since this shard would be used by any product that
supports virtualization.

We will then expose the `GuestLifecycle` protocol to the product session and
update the workstation product session to include `TerminaGuestManager`.

### Tools, Diagnostics, and Developer Ergonomics

We need to maintain the ability to run guest operating systems on the core
product. This is done today using additional core-shards that are added manually
to the build using GN arguments. Each core shard provides support for a single
guest. Nothing in this proposal requires changes to how this works, however it
vastly simplifies component routing since the vmm and the associated devices are
responsible for the majority of the capability routes into the guest manager
shards.

For future work we could change how we launch `GuestManager` components for
development purposes. Simple guests, like `debian` or `zircon` don't require any
capabilities aside from `GuestLifecycle`, so we could support launching these
components into a collection of their own to reduce the number of custom
core-realm shards are needed to support virtualization workflows.

## Security considerations

The `vmm` needs some privileged capabilities to properly operate (ex: the
`HypervisorResource`, some `/dev` directories). This design avoids exposing
these privileged capabilities to the session.

One capability difference with this design is before we offered a protocol that
represented a single VM instance (ex:
`fuchsia.virtualization.TerminaGuestManager`) and now we will offer a capability
to create an arbitrary number VM instances using the
`fuchsia.virtualization.GuestManager` protocol. This capability is exposed
through the `vmm` component only and we will not allow the session to launch
arbitrary components into the collection that contains the `vmm`s instances.

Any guests created by a session will be stopped whenever their `GuestLifecycle`
channel has closed to ensure no guests continue running after the user session
has stopped.

To avoid leaking state between `vmm` instances, mutable directory capabilities
will be dynamically routed to `vmm` instances from the corresponding
GuestManager component associated with each instance, rather than, for example,
statically routing a shared directory capability. This also allows for sessions
to manage and protect stateful VM data in a consistent way.

## Documentation

Details of how products consume virtualization will be documented in a new
section of the virtualization documentation on fuchsia.dev. This will include
details on the set of FIDL protocols that the product session can consume and
any best practices.

## Testing

As part of this change all the existing integration tests will be converted to
use the `vmm_launcher` to demonstrate correct functionality. We will also
explore possible ways to fuzz test the `vmm_launcher` to verify that `vmm`
instances are not routed shared, stateful capabilities.

## Alternatives Considered

### Remove the need for a VmmLauncher component

`vmm_launcher.cm` as proposed in this document doesn't do anything
interesting, it just creates components into a collection in response to
in-bound connection requests. This is something that could be handled by the
Component Framework directly which would allow us to remove this component. For
example, to support this the Component Framework could provide a way to create a
homogeneous collection of components. Instead of creating a dynamic child into
these collections, components would be auto-started whenever they were connected
to.

Adding a bespoke component to do this is not a large amount of work, but if this
is a common pattern it may make sense to consider adding this to the Component
Framework directly.

### Add an explicit VmmLauncher protocol

Instead of implicitly creating new `vmm` components in response to a connection
to the existing `GuestLifecycle` protocol, we could create a new, named protocol
to use for the `VmmLauncher`. This design would be appropriate if we find the
need to expand the API footprint to include more functionality than what is
currently proposed.

```fidl
library fuchsia.virtualization;

@discoverable
protocol VmmLauncher {
    /// Launches a new, uninitialized virtual machine. The bound
    /// `GuestLifecycle` can be used to initialize the VM and start or stop it.
    ///
    /// The VM will be stopped and the component destroyed when the
    ///`GuestLifecycle` channel is closed.
    Launch(resource struct {
        lifecycle server_end:GuestLifecycle;
    });

    /// ... other functionality ...
};
```

Since we don't require any additional functionality yet we will prefer to just
use the existing set of protocols instead.

[image.vmm_launcher_routing]: resources/0210_virtualization_capability_routing/vmm_launcher_routing.png
[image.static_routing]: resources/0210_virtualization_capability_routing/static_routing.png
