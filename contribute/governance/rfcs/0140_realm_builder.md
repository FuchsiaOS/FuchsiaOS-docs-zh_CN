<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0140" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

[Realm builder][realm-builder-docs] is a library available in-tree in Rust and
C++ that enables users to programmatically assemble component
[realms][realm-docs], and makes it possible for these realms to contain local
components backed by in-process implementations. This library is implemented
with a sidecar child component named the Realm Builder Server that is included
in the manifest of any component using the library. This server hosts the
majority of the implementation, and the library communicates with it over FIDL.

This RFC outlines the main design of the FIDL API and client libraries of realm
builder, and proposes this API, along with the C++ client library and the
manifest and binary (as a pre-built) for the Realm Builder Server be published
in the Fuchsia SDK.

## Motivation

Integration testing of components is a very important task, which the component
framework team wishes to make as easy and enjoyable as possible. Unfortunately
many of the features of the component framework which provide great security and
isolation properties to production components end up complicating testing
scenarios. If a component is to be tested in different environments, then
separate component manifests for each environment must be maintained by hand. If
the test wishes to provide capabilities to a component under test, the inability
to dynamically declare capabilities and offer them to specific components within
a collection makes this challenging. If a component under test is to be
connected with a mock provider of a capability, then the mock must be
communicated with over FIDL which necessitates maintaining a test-specific FIDL
API for that communication.

Realm builder aims to significantly improve the experience of integration test
authors, and thus also the quality of integration tests. By implementing a
[resolver][resolver-docs] capability the realm builder library can create new
component manifests and provide them to the component framework at runtime. By
implementing a [runner][runner-docs] capability the realm builder library can
insert local components into these constructed realms, which are backed by
in-process implementations and thus can use in-process tools for coordinating
with logic in the test. Tasks that are common for integration test
authors, such as setting up storage capabilities or providing fake configuration
directories, can be automated and simplified.

This library has already seen adoption and successful application within the
Fuchsia tree, and by making it available through the SDK it can also be
leveraged by the many developers working outside of the Fuchsia tree.

## Stakeholders

Who has a stake in whether this RFC is accepted? (This section is optional but
encouraged.)

_Facilitator:_ hjfreyer@google.com

_Reviewers:_

- Yaneury Fermin (yaneury@google.com) - All
- Gary Bressler (geb@google.com) - All
- Peter Johnston (peterjohnston@google.com) - Functionality
- Jaeheon Yi (jaeheon@google.com) - Usability

_Consulted:_

List people who should review the RFC, but whose approval is not required.

_Socialization:_ This RFC is based on a library which has undergone significant
design changes and evolution thanks to feedback and improvements from various
teams which have adopted its use so far. Specifically the Netstack, Wlan,
Bluetooth, and SWD teams have incorporated the library into their integration
tests at this point.

## Design

### Overview

There are two critical pieces of realm builder: the client library, and the
Realm Builder Server. The developer exercises the client library to describe
what realm they would like constructed and then instructs the library to create
the realm, and the client library accomplishes these tasks by working together
with the Realm Builder Server over a FIDL connection.

This cooperation has some nice properties, such as making it easier to support
client libraries in different languages (the server can be reused between the
different client languages), but the split between the client and server is in
fact required for realm builder to be usable in integration testing scenarios.
The test runners, which exercise the various cases present in a test, consume
the component's outgoing directory handle and do not make it available to the
component itself. This makes it impossible to declare and provide any
capabilities from the test component, and any tasks requiring this (such as the
resolver and runner capabilities declared by realm builder) _must_ be moved into
a separate component.

Realm builder is designed such that the FIDL API that is used by the client
libraries looks as similar as possible (and in some areas directly maps to) the
API the client libraries make available to the developer.

Usage of the client libraries is encouraged over the raw FIDL bindings because
they can provide a better developer experience, and some tasks (such as managing
the state of local component implementations) would be tedious and require
significant boilerplate to handle without the client library.

### Realm Initialization

When a new realm is to be created, the client library establishes a new
connection with the Realm Builder Server.

```rust
let mut builder = RealmBuilder::new().await?;
```

When this struct is initialized it connects to the Realm Builder Server and uses
the `fuchsia.component.test.RealmBuilderFactory` protocol. This protocol is
simple: the `New` method is called to establish two new channels. One is used to
construct a new realm, and the other is used to finalize the changes.
Additionally, this call provides the Realm Builder Server with a handle to the
test's package directory, which will be used to load components referenced by
relative URL.

```
@discoverable
protocol RealmBuilderFactory {
    New(resource struct {
        pkg_dir_handle client_end:fuchsia.io.Directory;
        realm_server_end server_end:Realm;
        builder_server_end server_end:Builder;
    });
}
```

With the `Realm` and `Builder` channels created, clients may now add components
to the realm.


### Adding components to the realm

```
type ChildProperties = table {
    1: startup fuchsia.component.decl.StartupMode;
    2: environment fuchsia.component.name;
    3: on_terminate fuchsia.component.decl.OnTerminate;
};

protocol Realm {
    /// Adds the given component to the realm. If a component already
    /// exists at the given name, then an error will be returned.
    AddChild(struct {
        /// The name, relative to the realm's root, for the component that is
        /// being added.
        name fuchsia.component.name;

        /// The component's URL
        url fuchsia.url.Url;

        /// Additional properties for the component
        properties ChildProperties;
    }) -> () error RealmBuilderError;

    /// Modifies this realm to contain a legacy component. If a component
    /// already exists with the given name, then an error will be returned.
    /// When the component is launched, realm builder will reach out to appmgr
    /// to assist with launching the component, and the component will be able
    /// to utilize all of the features of the [legacy Component
    /// Framework](https://fuchsia.dev/fuchsia-src/concepts/components/v1). Note
    /// that _only_ protocol capabilities may be routed to this component.
    /// Capabilities of any other type (such as a directory) are unsupported for
    /// legacy components launched by realm builder, and this legacy component
    /// should instead use the legacy features to access things such as storage.
    AddLegacyChild(struct {
        /// The name, relative to the realm's root, for the component that is
        /// being added.
        name fuchsia.component.name;

        /// The component's legacy URL (commonly ends with `.cmx`)
        legacy_url fuchsia.url.Url;

        /// Additional properties for the component
        properties ChildProperties;
    }) -> () error RealmBuilderError;

    /// Modifies this realm to contain a component whose declaration is set to
    /// `decl. If a component already exists at the given name, then an error
    /// will be returned.
    AddChildFromDecl(struct {
        /// The name, relative to the realm's root, for the component that is
        /// being added.
        name fuchsia.component.name;

        /// The component's declaration
        decl fuchsia.component.decl.Component;

        /// Additional properties for the component
        properties ChildProperties;
    }) -> () error RealmBuilderError;

    // Other entries omitted
    ...
};
```

The client libraries will return objects that wrap the component's name, to
make it easy to provide the same name when wiring the realm together later in
a strongly typed fashion.

```rust
impl RealmBuilder {
    pub async fn add_child(
        &self,
        name: impl Into<String>,
        url: impl Into<String>,
        child_props: ChildProperties
    ) -> Result<ComponentName, Error> {
        ...
        return ComponentName { name: name.into() };
    }
}

struct ComponentName {
    name: String,
}

impl Into<String> for &ComponentName {
    fn into(input: &ComponentName) -> String {
        input.name.clone()
    }
}
```

```rust
// echo_server is a struct that contains the string "echo_server", which can
// be given to other functions later to reference this component
let echo_server = builder.add_child(
    "echo-server",
    "#meta/echo_server.cm",
    ChildProperties::new(),
).await?;

let echo_client = builder.add_legacy_child(
    "echo-client",
    "fuchsia-pkg://fuchsia.com/echo#meta/client.cmx",
    ChildProperties::new().eager(),
).await?;

let echo_client_2 = builder.add_child_from_decl(
    "echo-client-2",
    ComponentDecl {
        program: Some(ProgramDecl {
            runner: Some("elf".into()),
            info: Dictionary {
                entries: vec![
                    DictionaryEntry {
                        key: "binary".to_string(),
                        value: Some(Box::new(DictionaryValue::Str(
                            // This binary exists in the test package
                            "/bin/echo_client",
                        ))),
                    },
                ],
                ..Dictionary::EMPTY
            },
        }),
        uses: vec![
            UseDecl::Protocol(UseProtocolDecl {
                source: UseSource::Parent,
                source_name: EchoMarker::PROTOCOL_NAME,
                target_path: format!("/svc/{}", EchoMarker::PROTOCOL_NAME).into(),
                dependency_type: DependencyType::Strong,
            }),
        ],
        ..ComponentDecl::default()
    },
    ChildProperties::new().eager(),
).await?;
```

The Realm Builder Server maintains an internal tree structure of the components
in the realm. When `add_child` is used with an absolute (i.e. non-relative) URL,
the manifest for the parent's component is mutated to hold a `ChildDecl` with
the given URL. For all other ways to add a component, the component's manifest
is held in the server's tree structure and may be mutated before the realm is
created.

#### Adding components with local implementations {#local-components}

Clients may also add components to a realm whose implementation is provided by a
local routine. This enables users to have mock component implementations live in
the same file as the test logic, and to use in-process communication to
coordinate between these components and the test itself.

```
protocol Realm {
    /// Sets a component to have a new local component implementation. When this
    /// component should be started, the runner channel passed into `Build` will
    /// receive a start request for a component whose `ProgramDecl` contains the
    /// name for the component that is to be run under the key
    /// `LOCAL_COMPONENT_NAME`. If a component already exists at the given
    /// name, then an error will be returned.
    AddLocalChild(struct {
        /// The name, relative to the realm's root, for the component that is
        /// being added.
        child_name fuchsia.component.name;

        /// Additional properties for the child
        properties ChildProperties:optional;
    }) -> () error RealmBuilderError;

    // Other entries omitted
    ...
}
```

Note that this means that each client library must implement the logic necessary
for executing and managing the lifecycle of local tasks for each of these local
components. More details on this is provided later, under the [section titled
"realm creation, and local component implementations"](#creation).

As an example, this code adds an in-process implementation for an echo client.

```rust
let echo_client_3 = builder.add_local_child(
    "echo-client-3",
    move |handles: LocalComponentHandles| {
        Box::pin(async move {
            let echo_proxy = handles.connect_to_service::<EchoMarker>()?;
            echo_proxy.echo_string("hello, world!").await?;
            Ok(())
        })
    },
    ChildProperties::new().eager(),
).await?;
```

And likewise, this code adds an in-process implementation for an echo server.

```rust
let (send_echo_server_called, mut receive_echo_server_called) = mpsc::channel(1);
let echo_server_2 = builder.add_local_child(
    "echo-server-2",
    move |handles: LocalComponentHandles| {
        let mut send_echo_server_called = send_echo_server_called.clone();
        Box::pin(async move {
            let mut fs = fserver::ServiceFs::new();
            let mut tasks = vec![];

            let mut send_echo_server_called = send_echo_server_called.clone();
            fs.dir("svc").add_fidl_service(move |mut stream: fecho::EchoRequestStream| {
                let mut send_echo_server_called = send_echo_server_called.clone();
                tasks.push(fasync::Task::local(async move {
                    while let Some(fecho::EchoRequest::EchoString { value, responder }) =
                        stream.try_next().await.expect("failed to serve echo service")
                    {
                        responder.send(value.as_ref().map(|s| &**s)).expect("failed to send echo response");

                        // Use send_echo_server_called to report back that we successfully received a
                        // message and it aligned with our expectations
                        send_echo_server_called.send(()).await.expect("failed to send results");
                    }
                }));
            });

            // Run the ServiceFs on the outgoing directory handle from the mock handles
            fs.serve_connection(mock_handles.outgoing_dir.into_channel())?;
            fs.collect::<()>().await;
            Ok(())
        })
    },
    ChildProperties::new().eager(),
).await?;
```

### Connecting components together

Once components have been added to the realm, the realm needs to have routing
added to connect the added components to each other and expose things to the
test.

#### Manual component manifest manipulation

Capability routing can be added in a very manual fashion by using
`GetComponentDecl` to retrieve a component's manifest, mutating the manifest
locally, and then setting the new version of the manifest by using
`ReplaceComponentDecl`. Do note though that if the realm builder server was
built with a different version of the `fuchsia.component.decl` API then this
approach could risk the client accidentally omitting a field with which it is
unfamiliar in the manifest.

```
protocol Realm {
    /// Returns the the component decl for the given component. `name` must
    /// refer to a component that is one of the following:
    ///
    /// - A component with a local implementation
    /// - A legacy component
    /// - A component added with a relative URL
    /// - A descendent of a component added with a relative URL
    /// - An automatically generated realm (ex: the root)
    ///
    /// If the component was added to the realm with a modern (i.e. non-legacy),
    /// absolute (i.e. non-relative) URL, then an error will be returned, as
    /// realm builder is unable to retrieve or alter the declarations for these
    /// components.
    GetComponentDecl(struct {
        name fuchsia.component.name;
    }) -> (struct {
        component_decl fuchsia.component.decl.Component;
    }) error RealmBuilderError;

    /// Sets the the component decl for the given component. If the component
    /// was added to the realm with a modern (i.e. non-legacy), absolute (i.e.
    /// non-relative) URL, then an error will be returned, as realm builder is
    /// unable to retrieve or alter the declarations for these components.
    ReplaceComponentDecl(struct {
        name fuchsia.component.name;
        component_decl fuchsia.component.decl.Component;
    }) -> () error RealmBuilderError;

    // Other entries omitted
    ...
};
```

```rust
let echo_server = builder.add_child(
    "echo-server",
    "#meta/echo_server.cm",
).await?;
let mut echo_decl = builder.get_component_decl(&echo_server).await?;
echo_decl.offer.push(OfferDecl { ... });
builder.replace_component_decl(&echo_server, echo_decl).await?;

let mut root_decl = builder.get_component_decl(RealmBuilder::root()).await?;
root_decl.offer.push(OfferDecl { ... });
builder.replace_component_decl(builder.root(), root_decl).await?;
```

#### Capability routing

A very common task for a developer using realm builder is to make a capability
available to a component. An approach for doing this is shown above, wherein the
relevant component manifests (specifically the manifests for the realm along
with any legacy or local children) are fetched, mutated, and then sent back to
the realm builder server, but this approach has some undesirable properties:

- Multiple targets cannot be specified in an `OfferDecl`.
- Multiple capabilities cannot be specified in an `OfferDecl` or `ExposeDecl`.
- The component declarations that the Realm Builder Server synthesizes for local
  and legacy components must also be updated to align with capabilities they
  should consume and capabilities they should provide, so adding offers or
  exposes to the root component is insufficient for routing capabilities when
  dealing with these components.

To assist developers with moving capabilities around their realms, the
`AddRoute` function exists.

```
protocol Realm {
    AddRoute(table {
        1: capabilities vector<RouteCapability>;
        2: from fuchsia.component.decl.Ref;
        3: to vector<fuchsia.component.decl.Ref>;
    }) -> () error RealmBuilderError;

    // Other entries omitted
    ...
};

type Parent = struct {};
type Debug = struct {};
type Framework = struct {
    scope string:fuchsia.component.MAX_PATH_LENGTH;
};

type RouteCapability = flexible union {
    1. protocol RouteCapabilityProtocol;
    2. directory RouteCapailityDirectory;

    // Routes for all the other capability types
    ...
};

type RouteCapabilityProtocol = table {
    1: name fuchsia.component.name;
    2: as fuchsia.component.name; // optional
    3: type_ fuchsia.component.decl.DependencyType; // optional
};

type RouteCapabilityDirectory = table {
    1: name fuchsia.component.name;
    2: as fuchsia.component.name; // optional
    3: type_ fuchsia.component.decl.DependencyType; // optional
    4: rights fuchsia.io.Rights; // optional
    5: subdir string:fuchsia.component.MAX_PATH_LENGTH; // optional
};
```

This function allows developers to specify a set of capabilities that should be
routed from a single source to a set of targets. This allows developers to
express the same information that would be present in a set of `OfferDecl` and
`ExposeDecl`, but far more succinctly. Additionally, when this recipe is used
any legacy or local components that are sources or targets will be updated to
declare, expose, and/or use the capability as appropriate.

Some examples of what this looks like in practice:

```rust
// Offer the LogSink protocol to components echo_server and echo_client
builder.add_route(
    vec![RouteCapability::protocol_from_marker::<LogSinkMarker>()],
    Ref::parent(),
    vec![&echo_server, &echo_client],
)).await?;
```

```rust
// Offer two different sub directories of dev to components echo_server and
// echo_client
builder.add_route(
    vec![
        RouteCapability::Directory(RouteCapabilityDirectory {
            name: "dev",
            as: "dev-class-input",
            subdir: "class/input",
            ..RouteCapabilityDirectory::default()
        }),
        RouteCapability::Directory(RouteCapabilityDirectory {
            name: "dev",
            as: "dev-class-block",
            subdir: "class/block",
            ..RouteCapabilityDirectory::default()
        }),
    ],
    Ref::parent(),
    vec![&echo_server, &echo_client],
).await?;
```

```rust
// Expose protocol fuchsia.test.Suite as fuchsia.test.Suite2 from component
// echo_client
builder.add_route(
    capabilities: vec![RouteCapability::Protocol(RouteCapabilityProtocol {
        name: "fuchsia.test.Suite",
        as: "fuchsia.test.Suite2",
        ..RouteCapabilityProtocol::default()
    })],
    from: &echo_client,
    to: vec![Ref::parent()],
).await?;
```

#### Read-only directory stub {#read-only-directory-stub}

Many components utilize `config-data`, which is a read-only directory holding
configuration data that is made available to components under the `config-data`
name. Providing each constructed realm with a stub for this directory is
possible using local component implementations, but this requires significant
boilerplate.

To make this pattern easier, along with any other uses for a read-only
directory, the Realm Builder Server can be provided with the contents of a
read-only directory and it will provide a directory capability holding these
contents to a component in the realm. This is done by automatically inserting a
"built-in" component into the realm, which is just like a component with a local
implementation in that it is a real component with a real component declaration,
but its implementation is provided by the Realm Builder Server itself instead of
asking the realm builder client to provide an implementation.

```
protocol Realm {
    ReadOnlyDirectory(table {
        1: name fuchsia.component.name;
        2: directory_name: string:fuchsia.component.MAX_NAME_LENGTH;
        3: directory_contents: vector<DirectoryEntry>:MAX_DIR_ENTRIES;
    }) -> () error RealmBuilderError;

    // Other entries omitted
    ...
};

const MAX_DIR_ENTRIES uint32 = 1024;

type DirectoryEntry = struct {
    path string:fuchsia.component.MAX_PATH_LENGTH;
    contents fuchsia.mem.Data;
};
```

```rust
builder.read_only_directory(
    &echo_server,
    "config-data".into(),
    vec![
        DirectoryEntry("file1", b"{ \"config_key\": \"config_value\" }"),
        DirectoryEntry("dir1/file2", b"{ \"foo\": \"bar\" }"),
    ],
).await?;
```

#### Mutable storage

Many components utilize mutable storage, and it can be advantageous for a test
to be able to view and mutate a component's storage before, during, and after
they have run. The isolation properties of the storage capabilities offered to
the tests realm prevents a test from accessing another component's storage,
which makes it unsuitable for this application. A test could host a mutable
directory itself to provide to a component under test through a local component
implementation, but this has a significant amount of boilerplate.

To make this pattern easier, the Realm Builder Server can be asked to host a
storage capability for a child component in the realm, and to provide a protocol
with which the component's storage may be accessed. This is done by adding a
"built-in" component to the realm to provide the storage capability, as is
described under the [read-only directory stub](#read-only-directory-stub)
function.

```
protocol Realm {
    HostStorage(table {
        1: name fuchsia.component.name;
        2: storage_name string:fuchsia.component.MAX_NAME_LENGTH;

        /// If set, will be connected to the component's isolated storage
        /// directory and can be immediately used (even before the realm is
        /// created).
        3: directory_server_end server_end:fuchsia.io.Directory;
    }) -> () error RealmBuilderError;

    // Other entries omitted
    ...
};
```

```rust
let component_storage_proxy = builder.host_storage(
    &echo_server,
    "data",
).await?;
let file_proxy = fuchsia_fs::open_file(
    &component_storage_proxy,
    "config-file.json",
    fio::OpenFlags::RIGHT_WRITABLE|fio::OpenFlags::CREATE,
)?;
fuchsia_fs::write_file(&file_proxy, "{ \"foo\": \"bar\"}").await?;
let realm_instance = builder.create().await?;
```

In the above examples, `component_storage_proxy` can be used to read and write
files in the component's storage both before and after the realm is created.
This proxy will be closed once the realm is destroyed however, so if the test
wishes to access this storage after the component has stopped then the test
should manually stop the component by accessing the component's [lifecycle
controller][lifecycle-controller].

#### Program declaration manipulation

Sometimes the contents of the `program` section of a component's manifest needs
to be altered, and using `GetComponentDecl` and `ReplaceComponentDecl` requires
a great amount of boilerplate to manipulate the contents of a program
declaration.

```rust
let mut echo_client_decl = builder.get_component_decl(&echo_client).await?;
for entry in echo_client_decl.program.as_mut().unwrap().info.entries.as_mut().unwrap() {
    if entry.key.as_str() == "args" {
        entry.value = Some(Box::new(fdata::DictionaryValue::StrVec(vec![
            "Whales".to_string(),
            "rule!".to_string(),
        ])));
    }
}
builder.replace_component_decl(&echo_client, echo_client_decl).await?;
```

To address this, the `MutateProgramDecl` function can assist.

```
protocol Realm {
    MutateProgramDecl(table {
        1: name fuchsia.component.name;
        2: field_name string:fuchsia.component.MAX_NAME_LENGTH;
        3: mutation ProgramFieldMutation;
    }) -> () error RealmBuilderError;

    // Other entries omitted
    ...
};

type ProgramFieldMutation = flexible union {
    /// Sets the field to the given string. Overwrites any pre-existing value
    /// for this field.
    1: set_value string:fuchsia.data.MAX_VALUE_LENGTH

    /// Sets the field to the given vector. Overwrites any pre-existing value
    /// for this field.
    2: set_vector vector<string:fuchsia.data.MAX_VALUE_LENGTH>:fuchsia.data.MAX_NUM_VALUE_ITEMS;

    /// Appends the given values to the field. If the field is not already a
    /// vector, it will be converted into one before the append is applied (a
    /// single value turns into a singleton vector, a missing field turns into
    /// an empty vector).
    3: append_to_vector vector<string:fuchsia.data.MAX_VALUE_LENGTH>:fuchsia.data.MAX_NUM_VALUE_ITEMS;
};
```

```rust
builder.mutate_program_decl(
    &echo_client,
    "args",
    ProgramFieldMutation::SetToVector(
        vec!["Whales".to_string(), "Rule".to_string()],
    ),
).await?;
```

### Working with child realms

For any use case that calls for constructing a realm with descendants beyond
direct children, a child realm can be opened with the `AddChildRealm` call.

```
protocol Realm {
    AddChildRealm(struct {
        name fuchsia.component.name;
        properties ChildProperties:optional;
    }) -> (child_realm client_end:Realm) error RealmBuilderError;

    ...
}
```

This call is similar to calling `AddChildFromDecl` with an empty component
declaration, with the key difference being that the call returns a new
`Realm` channel. The client may use this channel to add children to the
child realm, and manipulate them and the child realm in the same way that the
root realm may be.

For example, a child named `foo` with a child itself named `bar` may be added to
the realm, and a capability from `bar` routed to the parent of `foo` with the
following example:

```rust
let foo = builder.add_child_realm("foo", ChildProperties::new()).await?;
let bar = builder.add_local_child(
    "bar",
    move |handles: MockHandles| { ... },
    ChildProperties::new(),
).await?;
foo.add_route(
    vec![RouteCapability::protocol_from_marker::<FooBarMarker>()],
    &bar,
    vec![Ref::parent()],
).await?;
builder.add_route(
    vec![RouteCapability::protocol_from_marker::<FooBarMarker>()],
    &foo,
    vec![Ref::parent()],
).await?;
```

### Realm creation, and local component implementations {#creation}

When realm setup is complete, the realm may be created by calling `Build` on the
`Builder` channel that was returned by the `RealmBuilderFactory.New` call. Once
`Build` is called, no further mutations to the realm are possible using any
`Realm` channels for this realm, and thus the static components in the realm may
no longer be altered. Dynamic components (as in, in a [collection][collection])
can still be instantiated/run/destroyed in the realm as per usual Component
Framework semantics. The Realm Builder Server will return a URL which can be
given to `fuchsia.component/Realm.CreateChild` to create the realm.

```
@discoverable
protocol Builder {
    /// Assembles the realm being constructed and returns the URL for the root
    /// component in the realm, which may then be used to create a new component
    /// in any collection where fuchsia-test-component is properly set up.
    Build(struct {
        runner client_end:fuchsia.component.runner.ComponentRunner;
    }) -> (struct {
        root_component_url string:fuchsia.component.types.MAX_URL_LENGTH;
    }) error RealmBuilderError;
};
```

The `Build` function takes the client end for a component runner, and the
component using realm builder is expected to host a component runner for the
components backed by local (i.e. in-process) component implementations located
in the realm. This function returns a component URL which should be used by the
client in a `fuchsia.component/Realm.CreateChild` call to create the constructed
realm. This child should be placed in a collection that has the realm builder
resolver and runner capabilities available in the environment. Such a collection
is included in the realm builder [shard][manifest-shards] (a partial component
manifest that users are required to merge into their component manifest) and may
be used by the client, but any other collection with an appropriately configured
environment would also work.

Once the realm is created component manager will send start requests for local
components to the Realm Builder Server, who will then proxy these start requests
to the runner channel for the realm the local component exists in.

This is the part where using a client library instead of the direct FIDL
bindings significantly saves on boilerplate, as this can be handled
automatically.

Each start request will include a `ProgramDecl` which contains the key
`LOCAL_COMPONENT_NAME`. The value for this key will be one of the names given
to the `AddLocalChild` function. The local routine associated with the component
added using that function should then begin execution and be given access to the
handles from the start request.

Each start request also contains a component controller channel, which should be
serviced to receive notification from component manager when the local component
should stop execution. Once a stop notification is received, the client library
can either immediately stop execution of the local component's task, or it can
notify it that it is being instructed to stop and give it an opportunity to do
so cleanly.


### Realm destruction

When the realm's purpose has been fulfilled, the realm is destroyed with
`fuchsia.component/Realm.DestroyChild`. This causes component manager to stop
the realm in an orderly fashion, terminating clients before their dependencies,
just like any other realm. Note that this is a regular FIDL call from the test
to Component Manager; realm destruction is not specific to realm builder.

Once the `DestroyChild` call returns the realm has been destroyed, and all
components within the realm have stopped execution.


## Implementation

Realm builder is used extensively throughout the fuchsia.git repository, and
will soon be picking up users behind the SDK. All breaking changes detailed here
will thus be introduced through a soft migration, and then additive changes (ex:
the read only directory function) can be introduced with ease after the soft
migration is complete.

## Performance

The realm builder library is only suitable for use in test scenarios, and thus
the API is written to favor usability over performance. Each function call is
synchronous, and can return an error if input was invalid or other issues were
encountered. The lack of pipelining means that errors are reported in direct
response to the input that caused them, for the price of slower test setup.

## Ergonomics

Ergonomics is an important objective for realm builder, and both the client
libraries and the raw FIDL bindings should be easy to use. The advanced
component mutation functions aim to provide compelling features that are easy to
use for developers, and each function in the API is synchronous so that errors
can be returned as close to the point of origin as possible.

The client libraries themselves are also explicitly designed to provide improved
ergonomics over using the raw FIDL bindings. An example comparing the two can be
found under ["clients use the FIDL API directly"](#direct-fidl-bindings)

## Backwards Compatibility

The API described here has some breaking changes with the current realm builder
implementation. These changes aim to make it easy to evolve realm builder's
functionality in the future without breaking changes, as things like adding new
functions are simple additions to the API.

## Security considerations

The Realm builder client libraries are marked as `testonly`, and thus are always
used within the constrained test environment along with the test code. Due to
this, realm builder can not access any resources or capabilities not already
made available to tests. Thus there are no security concerns for this RFC.

## Privacy considerations

No user or otherwise private data is made available through realm builder, nor
can realm builder even access such data. Thus there are no privacy concerns for
this RFC.

## Testing

Realm builder has unit and integration tests already written for it, which will
be adapted to accommodate the new design. Additionally there are a sizeable
number of tests in-tree that will be modified to use the new realm builder
design, and these will provide additional confidence in the changes.

## Documentation

Realm builder has documentation [located in //docs][realm-builder-docs]. This
documentation will be updated and expanded upon to detail the new design and
functionality.

## Drawbacks, alternatives, and unknowns

### Deep realm support {#deep-realm-support}

The functions detailed throughout this RFC have clients pass in names
whenever a component in the realm is referenced. These are defined as "child
names", and are the names that will appear verbatim in the generated
component manifests.

An alternative approach would be to accept "relative names", where components
deeper in the realm than the top-level children could, for example, be
referenced by a `AddRoute` call in order to connect components further away in
the instance tree than direct descendants or siblings.

### Reducing server-side complexity

Before the C++ library was added the API was much simpler. The bulk of the logic
lived in the client library, with the realm builder server component being as
slim as possible. This was also a suitable arrangement, but it would have
resulted in significantly higher costs to port the library to other languages,
and thus the current approach was adopted.

### Clients use the FIDL API directly {#direct-fidl-bindings}

One alternative to this design would be to have clients connect to and exercise
the realm builder FIDL bindings directly, instead of relying on a client library
to do so for them. We would still want to maintain and recommend libraries for
the runner related tasks that are needed once `Build` is called, as these tasks
would result in significant boilerplate for clients.

This would result in this example Rust code...

```rust
let builder = RealmBuilder::new().await?;
let echo_server = builder.add_child(
    "echo-server",
    "#meta/echo_server.cm",
    ChildProperties::new(),
).await?;
let echo_client = builder.add_legacy_child(
    "echo-client",
    "fuchsia-pkg://fuchsia.com/echo#meta/client.cmx",
    ChildProperties::new().eager(),
).await?;
builder.add_route(
    vec![RouteCapability::protocol_from_marker::<EchoMarker>()],
    &echo_server,
    vec![&echo_client],
)).await?;
let realm_instance = builder.build().await?;
```

...looking something like this:

```rust
let rb_factory_proxy = connect_to_service::<RealmBuilderFactoryMarker>().await?;
let pkg_dir_proxy = fuchsia_fs::open_directory_in_namespace(
    "/pkg",
    fuchsia_fs::OpenFlags::RIGHT_READABLE | fuchsia_fs::OpenFlags::RIGHT_EXECUTABLE,
)?;
let pkg_dir_client_end =
    ClientEnd::from(pkg_dir_proxy.into_channel().unwrap().into_zx_channel());
let (realm_proxy, realm_server_end) = create_proxy::<RealmMarker>?().await?;
let (builder_proxy, builder_server_end) = create_proxy::<BuilderMarker>?().await?;
rb_factory_proxy.new(
    pkg_dir_client_end,
    realm_server_end,
    builder_server_end,
)?;

realm_proxy.add_child(
    &"echo-server".to_string(),
    &"#meta/echo_server.cm".to_string(),
    &mut ChildProperties::EMPTY,
).await??;

realm_proxy.add_child(
    &"echo-client".to_string(),
    &"fuchsia-pkg://fuchsia.com/echo#meta/client.cmx".to_string(),
    &mut ChildProperties {
        startup: fsys::StartupMode::Eager,
        ..ChildProperties::EMPTY
    },
).await??;

realm_proxy.add_route(
    &mut CapabilityRoute {
        capabilities: vec![
            RouteCapability::Protocol(RouteCapabilityProtocol {
                name: EchoMarker::PROTOCOL_NAME.to_string(),
                ..RouteCapabilityProtocol::EMPTY
            }),
        ],
        from: Ref::Child(ChildRef {
            name: "echo-server".to_string(),
            collection: None,
        }),
        to: vec![Ref::Child(ChildRef {
            name: "echo-client".to_string(),
            collection: None,
        })],
        ..CapabilityRoute::EMPTY
    },
).await??;

// We omit the component runner because there are no local components in this
// realm.
let root_component_url = builder_proxy.build(None).await?;
let realm_instance =
    ScopedInstance::new("collection-name", root_component_url).await?;
```

This alternative is not being chosen due to the higher boilerplate and more
verbose function calls.

### Implicit containing realm

According to the proposal outlined in the "design" section, capabilities
implemented in the component using realm builder that are to be accessed by
components in the constructed realm are made available to the realm through a
"local component". Once the local component is added to the realm, capabilities
may be routed from it to their clients.

This indirection is likely to be unexpected by new users. If a capability is
implemented in the constructed realm's parent, then it's a reasonable guess that
the added capability route would have a source of `parent`.

To enable routes to have a source of `parent` for capabilities implemented in
the constructed realm's parent, realm builder could insert a new component into
the realm above the user-controlled root component. This component would hold
capability implementations, and any routes that come from `parent` and are _not_
implemented in this component would result in an offer with a source of `parent`
being added to the implicitly inserted component.

For example, this code...

```rust
let builder = RealmBuilder::new().await?;
let echo_client = builder.add_child(
    "echo-client",
    "#meta/echo_client.cm",
    ChildProperties::new().eager(),
).await?;
builder.add_local_capability(
    RouteCapability::protocol_from_marker::<EchoMarker>(),
    |stream: EchoRequestStream| { echo_server_implementation(stream).boxed() },
).await?;
builder.add_route(
    vec![RouteCapability::protocol_from_marker::<EchoMarker>()],
    Ref::parent(),
    vec![&echo_client],
)).await?;
builder.add_route(
    vec![RouteCapability::protocol_from_marker::<LogSinkMarker>()],
    Ref::parent(),
    vec![&echo_client],
)).await?;
let realm_instance = builder.build().await?;
```

...would result in this realm structure...

```
implicit_local_component
          |
         root
          |
     echo_server
```

...and the manifest for `implicit_local_component` would include the following:

```
{
    offer: [
        {
            protocol: "fuchsia.logger.LogSink",
            from: "parent",
            to: "root",
        },
        {
            protocol: "fuchsia.example.Echo",
            from: "self",
            to: "root",
        },
    ],
    capabilities: [
        {
            protocol: "fuchsia.example.Echo",
        },
    ],
}
```

This alternative is not being chosen at this time, as it's unnecessary to
achieve the benefits outlined in the "motivation" section.

### Offer and expose instead of a single route definition

The API for `AddRoute` proposed above does not require the user to differentiate
between offers and exposes. The parent may be freely referenced as a source or a
target to a route. This deviates from the CML and CM formats, and splitting the
`AddRoute` call into `AddOffer` and `AddExpose` would make the realm builder
APIs more conceptually consistent with the rest of the component framework.

```
protocol Realm {
    AddOffer(table {
        1: capabilities vector<RouteCapability>;
        2: from fuchsia.component.decl.Ref;
        // An error will be returned if `to` contains `Ref::Parent`
        3: to vector<fuchsia.component.decl.Ref>;
    }) -> () error RealmBuilderError;

    AddExpose(table {
        1: capabilities vector<RouteCapability>;
        // An error will be returned if `from` contains `Ref::Parent`
        2: from fuchsia.component.decl.Ref;
        3: to fuchsia.component.decl.Ref;
    }) -> () error RealmBuilderError;

    // Other entries omitted
    ...
};
```

This alternative is not being chosen at this time, as the current API will allow
easier exploration in the future of [deep realm support](#deep-realm-support),
this approach will allow us to gather more data on the utility of a combined
offer/expose API, and this approach results in a slightly simpler API.

[realm-builder-docs]: development/testing/components/realm_builder.md
[realm-docs]: concepts/components/v2/realms.md
[resolver-docs]: concepts/components/v2/capabilities/resolvers.md
[runner-docs]: concepts/components/v2/capabilities/runners.md
[test-manager-lib-rs]: /src/sys/test_manager/src/lib.rs
[lifecycle-controller]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#LifecycleController
[collection]: concepts/components/v2/realms.md#collections
[manifest-shards]: development/components/build.md#component-manifest-shards
