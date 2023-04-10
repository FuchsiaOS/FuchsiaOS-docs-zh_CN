{% set rfcid = "RFC-0041" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-041.

## Summary

Introduce the notion of a service &mdash; a collection of protocols, where
there may be one or more instances of the collection.

## Motivation

Today, within the component framework, a service is defined as a single
protocol, and only one instance of that protocol may exist in the
namespace of a process under `/svc`.
This prevents us from describing more complex relationships:

*   A service that is expressed in two different forms, depending on the
    consumer &mdash; e.g., when there are two different versions of the
    protocol, like `FontProvider` and `FontProviderV2`
*   A service that is split in two, in order to grant features based on
    levels of access &mdash; e.g., regular access versus administrative
    access, like `Directory` and `DirectoryAdmin`, where the latter provides
    privileged access
*   A service that is comprised of many different protocols for use by
    different consumers &mdash; e.g., like `Power` for power management, and
    `Ethernet` for network stacks
*   A service that has multiple instances &mdash; e.g., multiple audio
    devices offering `AudioRenderer`, or multiple printers exposing `Printer`

Providing this flexibility allows a service to be more clearly expressed,
without resorting to the use of workarounds like [service
hubs](/docs/development/api/fidl.md#service_hubs).
With that flexibility, we can define devices as services.
Concretely, we plan to evolve `/svc/`**`$Protocol`**
which implies "only one protocol per process namespace" to:

```fidl
/svc/$Service/$Instance/$Member
```

Which instead introduces two additional indirections: a service (e.g.,
printer, ethernet), and an instance (e.g., default, deskjet_by_desk,
e80::d189:3247:5fb6:5808).
A path to a protocol will then consist of the following parts:

*   `$Service` &mdash; the fully-qualified type of the service, as
    declared in FIDL
*   `$Instance` &mdash; the name of an instance of the service, where
    "default" is used by convention to indicate the preferred (or only)
    instance made available
*   `$Member` &mdash; a service member name, as declared in FIDL, where
    the declared type of that member indicates the intended protocol

## Design

### Flavours of Services

Let's first consider various flavours of service we aim to support:

* A single, unique protocol: **ONE** instance, **ONE** protocol:

  ```
  /svc/fuchsia.Scheduler/default/profile_provider
  ```

* A composite of multiple protocols: **ONE** instance, **MANY** protocols:

  ```
  /svc/fuchsia.Time/default/network
                        .../rough
  ```

* Multiple instances of a service, with a single protocol: **MANY** instances, **ONE** protocol:

  ```
  /svc/fuchsia.hardware.Block/0/device
                          .../1/device
  ```

* Multiple instances, with different sets of protocols: **MANY** instances, **MANY** protocols:

  ```
  /svc/fuchsia.Wlan/ff:ee:dd:cc:bb:aa/device
                                  .../power
                .../00:11:22:33:44:55/access_point
                                  .../power
  ```

### Language

To introduce the notion of a service to FIDL and support the various
flavours, we will make the following changes to the FIDL language:

1. Add a `service` keyword.
2. Remove the `Discoverable` attribute.

The `service` keyword will allow us to write a service declaration, which
we can use to define a set of protocols as members of a service.
For example, we can declare the different flavours of service as follows:

* A single, unique protocol: **ONE** instance, **ONE** protocol:

  ```
  service Scheduler {
    fuchsia.scheduler.ProfileProvider profile_provider;
  };
  ```

* A composite of multiple protocols: **ONE** instance, **MANY** protocols:

  ```
  service Time {
    fuchsia.time.Provider network;
    fuchsia.time.Provider rough;
  };
  ```

* Multiple instances of a service, with a single protocol: **MANY** instances, **ONE** protocol:

  ```
  service Block {
    fuchsia.hardware.block.Device device;
  };
  ```

* Multiple instances, with different sets of protocols: **MANY** instances, **MANY** protocols

  ```
  service Wlan {
    fuchsia.hardware.ethernet.Device device;
    fuchsia.wlan.AccessPoint access_point;
    fuchsia.hardware.Power power;
  };
  ```

A service declaration may have multiple members that use the same
protocol, but each member declaration must use a different identifier.
See "a composite of multiple protocols" above.

When an instance of a service may contain a different set of protocols
from another instance, the service declaration declares all possible
protocols that may be present in any instance.
See "multiple instances, with different sets of protocols" above.

A service declaration makes no mention of the names of specific instances
of a service or the URI of the components that offer the service, this is
left to the purview of the component framework based on component manifest
declarations and use of its APIs at runtime.

### Language bindings

Language bindings will be modified to make connecting to a service more
convenient.
Specifically, they will become more service-oriented, for example:

* Connect to the "default" instance of a service, with a single protocol: **ONE** instance, **ONE** protocol:
  * C++:

    ```cpp
    Scheduler scheduler = Scheduler::Open();
    ProfileProviderPtr profile_provider;
    scheduler.profile_provider().Connect(profile_provider.NewRequest());
    ```

  * Rust:

    ```rust
    let scheduler = open_service::<Scheduler>();
    let profile_provider: ProfileProviderProxy = scheduler.profile_provider();
    ```

* Connect to the "default" instance of a service, with multiple protocols: **ONE** instance, **MANY** protocols:
  * C++:

    ```cpp
    Time time = Time::Open();
    ProviderPtr network;
    time.network().Connect(&network);
    ProviderPtr rough;
    time.rough().Connect(&rough);
    ```

  * Rust:

    ```rust
    let time = open_service::<Time>();
    let network = time.network();
    let rough = time.rough();
    ```

* Connect to multiple instances of a service, with a single protocol: **MANY** instances, **ONE** protocol:
  * C++:

    ```cpp
    Block block_0 = Block::OpenInstance("0");
    DevicePtr device_0;
    block_0.device().Connect(&device_0);

    Block block_1 = Block::OpenInstance("1");
    DevicePtr device_1;
    block_1.device().Connect(&device_1);
    ```

  * Rust:

    ```rust
    let block_0 = open_service_instance::<Block>("0");
    let device_0 = block_0.device();
    let block_1 = open_service_instance::<Block>("1");
    let device_1 = block_1.device();
    ```

* Connect to multiple instances of a service, with multiple protocols: **MANY** instances, **MANY** protocols:
  * C++:

    ```cpp
    Wlan wlan_a = Wlan::OpenInstance("ff:ee:dd:cc:bb:aa");
    DevicePtr device;
    wlan_a.device().Connect(&device);
    Power power_a;
    wlan_a.power().Connect(&power_a);

    Wlan wlan_b = Wlan::OpenInstance("00:11:22:33:44:55");
    AccessPoint access_point;
    wlan_b.access_point().Connect(&access_point);
    Power power_b;
    wlan_b.power().Connect(&power_b);
    ```

  * Rust:

    ```rust
    let wlan_a = open_service_instance::<Wlan>("ff:ee:dd:cc:bb:aa");
    let device = wlan_a.device();
    let power_a = wlan_a.power();

    let wlan_b = open_service_instance::<Wlan>("00:11:22:33:44:55");
    let access_point = wlan_b.access_point();
    let power_b = wlan_b.power();
    ```

The following illustrates the proposed function signatures.

Note that the `Open()` and `OpenInstance()` methods also accept an
optional parameter to specify the namespace.
By default, the process's global namespace will be used (can be retrieved
using [fdio_ns_get_installed]).

```c++
// Generated code.
namespace my_library {
class MyService final {
public:
  // Opens the "default" instance of the service.
  //
  // |ns| the namespace within which to open the service or nullptr to use
  // the process's "global" namespace as defined by |fdio_ns_get_installed()|.
  static MyService Open(fdio_ns_t* ns = nullptr) {
    return OpenInstance(fidl::kDefaultInstanceName, ns);
  }

  // Opens the specified instance of the service.
  //
  // |name| the name of the instance, must not be nullptr
  // |ns| the namespace within which to open the service or nullptr to use
  // the process's "global" namespace as defined by |fdio_ns_get_installed()|.
  static MyService OpenInstance(const std::string& instance_name,
                                fdio_ns_t* ns = nullptr);

  // Opens the instance of the service located within the specified directory.
  static MyService OpenAt(zxio_t* directory);
  static MyService OpenAt(fuchsia::io::DirectoryPtr directory);

  // Opens a directory of available service instances.
  //
  // |ns| the namespace within which to open the service or nullptr to use
  // the process's "global" namespace as defined by |fdio_ns_get_installed()|.
  static fidl::ServiceDirectory<MyService> OpenDirectory(fdio_ns_t* ns = nullptr) {
    return fidl::ServiceDirectory<MyService>::Open(ns);
  }

  // Gets a connector for service member "foo".
  fidl::ServiceConnector<MyService, MyProtocol> foo() const;

  // Gets a connector for service member "bar".
  fidl::ServiceConnector<MyService, MyProtocol> bar() const;

  /* more stuff like constructors, destructors, etc... */
}
```

And the bindings code:

```c++
/// FIDL bindings code.
namespace fidl {
constexpr char[] kDefaultInstanceName = "default";

// Connects to a particular protocol offered by a service.
template <typename Service, typename Protocol>
class ServiceConnector final {
public:
   zx_status_t Connect(InterfaceRequest<Protocol> request);
};

// A directory of available service instances.
template <typename Service>
class ServiceDirectory final {
public:
  // Opens a directory of available service instances.
  //
  // |ns| the namespace within which to open the service or nullptr to use
  // the process's "global" namespace as defined by |fdio_ns_get_installed()|.
  static ServiceDirectory Open(fdio_ns_t* ns = nullptr);

  // Gets the underlying directory.
  zxio_t* directory() const;

  // Gets a list of all available instances of the service.
  std::vector<std::string> ListInstances();

  // Opens an instance of the service.
  Service OpenInstance(const std::string& name);

  // Begins watching for services to be added or removed.
  //
  // Invokes the provided |callback| to report all currently available services
  // then reports incremental changes.  The callback must outlive the returned
  // |Watcher| object.
  //
  // The watch ends when the returned |Watcher| object is destroyed.
  [[nodiscard]] Watcher Watch(WatchCallback* callback,
                              async_dispatcher_t* dispatcher = nullptr);

  // Keeps watch.
  //
  // This object has RAII semantics.  The watch ends once the watcher has
  // been destroyed.
  class Watcher final {
  public:
    // Ends the watch.
    ~Watcher();
  };

  // Callback invoked when service instances are added or removed.
  class WatchCallback {
  public:
    virtual void OnInstanceAdded(std::string name) = 0;
    virtual void OnInstanceRemoved(std::string name) = 0;
    virtual void OnError(zx_status_t error) = 0;
  };
}
```

Language bindings will further expand upon these by offering convenient
methods of iterating through instances of a service, and watching for new
instances to become available.

### Service Evolution

To evolve a service, we can add new protocols to it over time.
In order to maintain source compatibility, existing protocols should not
be removed, otherwise source compatibility may be broken as users may
depend on the code generated from the service by language bindings.

As all protocols within a service are effectively optional, they may or
may not be provided at runtime and components should be built for that
eventuality, it simplifies the set of problems we face when evolving a
service:

*   Adding a protocol member to a service can be done at any time
*   Removing a protocol member should be avoided (for source compatibility)
*   Renaming a protocol member involves adding a new protocol member, and
    leaving the existing protocol member

To evolve a service itself, we have a similar set of restrictions.
A service is not guaranteed to exist within a component's namespace, and a
service can be visible at multiple different locations within a namespace,
therefore:

*   Adding a service can be done at any time
*   Removing a service should be avoided (for source compatibility)
*   Renaming a service involves duplicating a service and using a new
    name, whilst keeping the original copy of the service (for source
    compatibility)

### Possible Extensions

We expect `service` instances to eventually become 'first class' and be
allowed to be part of messages, just like `protocol P` handles can be
passed around as `P` or `request<P>`.
This might take the form of something like `service_instance<S>` for a
`service S`.
We will make sure that this extension is possible, without putting working
behind it today.

We leave the door open to (and plan on) expanding the kinds of members
possible beyond solely allowing protocols.
For instance, we may want to have a VMO (`handle<vmo>`) exposed by a service:

```fidl
service DesignedService {
    ...
    handle<vmo>:readonly logo; // gif87a
};
```

## Implementation strategy

This proposal should be implemented in phases, so as not to break existing
code.

##### _Phase 1_

1. Modify component_manager, so that components v2 supports the new
   directory schema for services.
2. Modify appmgr and sysmgr, so that components v1 supports the new
   directory schema for services.

##### _Phase 2_

1. Add support for service declarations.
2. Modify the language bindings to generate services.

##### _Phase 3_

1. For all protocols that have a `Discoverable` attribute, create
   appropriate service declarations.
   > Note: at this stage, we should verify that there are no name
   > collisions possible between the old and new directory schemas for services.
2. Migrate all source code to use services.

##### _Phase 4_

1. Remove all `Discoverable` attributes from FIDL files.
2. Remove support for `Discoverable` from FIDL and the language bindings.
3. Remove support for the old directory schema from component_manager,
   appmgr, and sysmgr.

## Documentation and examples

We would need to expand the [FIDL tutorial] to explain the use of service
declarations, and how they interact with protocols.
We would then explain the different structures of a service: singleton vs
multi-instance, and how the language bindings can be used.

### Glossary

A **protocol declaration** describes a set of messages that may be sent or
received over a channel and their binary representation.

A **service declaration** describes a capability that is offered as a unit
by a service provider.
It consists of a service name and zero-or-more named member protocols that
clients use to interact with the capability.

The same protocol may appear more than once as a member of a service
declaration, with the member's name indicating the intended interpretation
of a protocol:

```fidl
service Foo {
    fuchsia.io.File logs;
    fuchsia.io.File journal;
};
```

A **component declaration** describes a unit of executable software,
including the location of the component's binaries and the capabilities
(such as services) that it intends to **use**, **expose**, or **offer** to
other components.

This information is typically encoded as a **component manifest file**
within a package:

```json
// frobinator.cml
{
    "uses": [{ "service": "fuchsia.log.LogSink" }],
    "exposes": [{ "service": "fuchsia.frobinator.Frobber" }],
    "offers": [{
        "service": "fuchsia.log.LogSink",
        "from": "realm",
        "to": [ "#child" ]
    }],
    "program": { "binary": ... }
    "children": { "child": ... }
}
```

A **service instance** is a capability that conforms to a given service
declaration.
On Fuchsia, it is represented as a directory.
Other systems may use different service discovery mechanisms.

A **component instance** is a particular instance of a component with its
own private sandbox.
At runtime, it uses service instances offered by other components through
opening directories in its **incoming namespace**.
Conversely, it exposes its own service instances to other components by
presenting them in its **outgoing directory**.
The **component manager** acts as a broker for service discovery.

*   A component instance is often (but not always) one-to-one with a
    **process**.
*   Component runners can often run multiple component instances within
    the same process each with its _own_ incoming namespace.

### Idiomatic Use of Services

## Backwards compatibility

This proposal will deprecate, and eventually remove the `Discoverable`
attribute from FIDL.

There are no changes to the wire format.

If you are introducing a new data type or language feature, consider what
changes you would expect users to make to FIDL definitions without
breaking users of the generated code.
If your feature places any new [source compatibility](/docs/contribute/governance/rfcs/0024_mandatory_source_compatibility.md)
restrictions on the generated language bindings, list those here.

## Performance

This should have no impact on IPC performance when connecting to the
default instance of a service, or an instance known _a priori_.

To connect to a different instance, where the instance ID is not known
_a priori_, will require the user to list the service's directory and locate
the instance before connecting.

There will be a minimal impact on build and binary size, as service
definitions must be generated by backends for particular language bindings.

## Security

This proposal will allow us to enforce more fine-grained access control,
as we can split a service into separate protocols with different access
rights.

This proposal has no other effect on security.

## Testing

Unit tests in the compiler, and changes to the compatibility test suite to
check that protocols contained within services can be connected to.

## Drawbacks, alternatives, and unknowns

The following questions are explored:

* [Why do service declarations belong in FIDL?](#q1)
* [What is the difference between a protocol, a service, and a
  component?](#q2)
* [Is the proposed flat topology for service instances sufficiently
  expressive?](#q3)
* [How should we extend services over time?](#q4)
* [If a component instance wishes to expose multiple services that relate
  to a single underlying logical resource, how is that expressed?](#q5)

### Q1: Why do service declarations belong in FIDL? {#q1}

#### Response

*   We use FIDL to describe Fuchsia's system API including the protocols
    that components exchange.
*   The same protocols may be used in many ways depending on the situation.
    Representing the various uses of these protocols as services makes it
    easier for developers to access the right set of protocols for each
    situation.
*   FIDL already provides language bindings that can readily be extended
    to provide developers a consistent and convenient way to access these
    services.

#### Discussion

* [ianloic] But what about component manifests?  Why not use FIDL to
  describe those too?
* [jeffbrown] component manifests describe concepts that go well beyond IPC
  concerns
* [abdulla] describing services in component manifests would lead to
  duplication of the description of those services
* [ianloic] could we generate the skeleton of a component from its manifest?
* [drees] putting service declarations in FIDL is imposing a specific
  structure, does this make sense on other platforms?
* [jeffbrown] we want declarations of services to be external to components
  because they need to be shared between components, it is the point of
  agreement for service exchange
* [ianloic] service declarations for overnet likely to be similar
* [pascallouis] Is it is good to start simple based on what we know we need
  now. We can adapt later as needed.
* [pascallouis] FIDL is Fuchsia first so it makes sense to introduce
  features that only make sense in that context given the information we
  have today but that over time could be generalized for other contexts
* [dustingreen] what about a separate file?
* [pascallouis] those files would be very small and lonely, opportunities
  for static type checking if we keep them in FIDL, seems low risk to move
  it later if needed

### Q2: What is the difference between a protocol, a service, and a component? {#q2}

#### Response

*   A **protocol declaration** describes a set of messages that may be
    sent or received over a channel and their binary representation.
*   A **service declaration** describes a capability that is offered as a
    unit by a service provider.
    It consists of a service name and zero-or-more named member protocols
    that clients use to interact with the capability.
    *   The same protocol may appear more than once as a member of a
    service declaration; the member's name indicates the intended
    interpretation of a protocol.
        *   e.g., `service Foo { fuchsia.io.File logs; fuchsia.io.File
            journal; };`
*   A **component declaration** describes a unit of executable software,
    including the location of the component's binaries and the capabilities
    (such as services) that it intends to **use**, **expose**, or **offer** to
    other components.
    *   This information is typically encoded as a **component manifest
        file** within a package.
        Example:

        ```
        // frobinator.cml
        {
            "uses": [{ "service": "fuchsia.log.LogSink" }],
            "exposes": [{ "service": "fuchsia.frobinator.Frobber" }],
            "offers": [{ "service": "fuchsia.log.LogSink",
                         "from": "realm", "to": [ "#child" ]}],
            "program": { "binary": ... }
            "children": { "child": ... }
        }
        ```

*   A **service instance** is a capability that conforms to a given
    service declaration.
    On Fuchsia, it is represented as a directory.
    Other systems may use different service discovery mechanisms.
*   A **component instance** is a particular instance of a component with
    its own private sandbox.
    At runtime, it uses service instances offered by other components
    through opening directories in its **incoming namespace**.
    Conversely, it exposes its own service instances to other components
    by presenting them in its **outgoing directory**.
    The **component manager** acts as a broker for service discovery.
    *   A component instance is often (but not always) one-to-one with a
        **process**.
    *   Component runners can often run multiple component instances
        within the same process each with its _own_ incoming namespace.

#### Discussion

* [ianloic] what guidance should we offer for choosing protocol composition
  vs. service declarations?
* [abdulla] protocol composition indicates that the protocol themselves are
  highly related vs. service is indicating that a set of capabilities
  (possibly unrelated) are being jointly offered
* [pascallouis] compose multiplexes protocols over a single channel so has
  implications for message ordering vs. individual protocols of a service
  have different channels
* [jeffbrown] can delegate in different places, not related, composition
  doesn't get you this functionality, services allow "discovery" at runtime,
  e.g. listing which protocols are available

### Q3: Is the proposed flat topology for service instances sufficiently expressive? {#q3}

#### Response

*   A flat topology is easy to use because there is no need to recursively
    traverse paths to locate all instances.
    This impacts both ease of use and performance.
*   A flat topology can be just as expressive as a hierarchical topology
    when relevant information is encoded in the instance names, e.g.,
    `/svc/fuchsia.Ethernet/`**rack.5,port.9**`/packet_receiver`.
*   Services can be accessed from different locations using **Open()**,
    **Open(namespace)**, and **OpenAt(directory)**.
    In other words, not all services need to come from `/svc" in the
    process's global namespace.
    This allows for the creation of arbitrary service topologies, if
    necessary.

### Q4: How should we extend services over time? {#q4}

#### Response

*   We can add new members to existing service declarations.
    Adding a new member doesn't break source or binary-compatibility
    because each member is effectively optional (attempting to connect to the
    protocol is an operation that can fail).
*   We can remove existing members from service declarations.
    Removing (or renaming) an existing member may break source and binary
    compatibility and may require a careful migration plan to mitigate adverse
    impact.
*   The service's documentation should provide clear expectations for how
    the service is intended to be used or implemented, particularly when such
    usage is not obvious, e.g., explain what features of the service are
    deprecated and slated for removal.
*   Anticipated pattern for versioning: add new members to a service as
    protocols evolve.
    Protocol enumeration (listing directories) allows clients to discover
    what is supported.
    Example:
    *   In version 1...

        ``` fidl
        service Fonts {
            FontProvider provider;
        };

        protocol FontProvider {
            GimmeDaFont(string font_name) -> (fuchsia.mem.Buffer ttf);
        };
        ```

    *   In version 2, an incremental update...

        ```fidl
        service Fonts {
            FontProvider provider;
            FontProvider2 provider2;
        };

        protocol FontProvider2 {
            compose FontProvider;
            GetDefaultFontByFamily(string family) -> (string family);
        };
        ```

    *   In version 3, a complete redesign...

        ```fidl
        service Fonts {
            [Deprecated]
            FontProvider provider;
            [Deprecated]
            FontProvider provider2;
            TypefaceChooser typeface_chooser;
        }

        protocol TypefaceChooser {
            GetTypeface(TypefaceCriteria criteria);
        };

        table TypefaceCriteria {
            1: Family family;
            2: Style style;
            3: int weight;
        };
        ```

<!-- must be on the same line -->
### Q5: If a component instance wishes to expose multiple services that relate to a single underlying logical resource, how is that expressed? {#q5}

#### Response

*   A component would define multiple services that are exposed through
    its component manifest.
    Example:

    ```json
    // frobinator.cml
    {
        ...
        "exposes": [
            { "service": "fuchsia.frobinator.Fooer" },
            { "service": "fuchsia.frobinator.Barer" },
        ],
        ...
    }
    ```

*   The component would then implement these services on top of the single
    underlying resource, but users of these services need not be aware of that
    fact.

<!-- xrefs -->
[fdio_ns_get_installed]: /sdk/lib/fdio/include/lib/fdio/namespace.h#70
[FIDL tutorial]: /docs/development/languages/fidl/tutorials/overview.md
