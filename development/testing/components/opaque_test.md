# [DEPRECATED] Hermetic testing using OpaqueTest

Warning: OpaqueTest is deprecated. Refer to [Component testing][testing-overview]
for information about testing v2 components using the Test Runner Framework.

## Motivation

The OpaqueTest framework enables an integration test to observe or
influence the behavior of component manager without depending on its internal
libraries.

Creating dependencies on component manager's internal libraries is problematic
for a number of reasons:

- A test can set up component manager inconsistently with how component manager
is normally started.
- A test can modify component manager’s behavior in arbitrary ways.
- Changes to component manager may require changing the test.

## Features

To test the behavior of a v2 component, OpaqueTest lets you:

- Start component manager in a hermetic environment.
- Communicate with component manager using only FIDL and the [hub][concepts-hub].
- Access the hub of the root component.
- Wait for events to occur in component manager.
- Halt a component manager task on an event.
- Inject or mock out capabilities.
- Interpose between a client and a service.

## Minimum requirements

For the OpaqueTest framework to function correctly, the test cmx manifest
must specify (at minimum) the following features and services:

```json
{
    "include": [ "syslog/client.shard.cmx" ],
    "sandbox": {
        "features": [
            "hub"
        ],
        "services": [
            "fuchsia.process.Launcher",
            "fuchsia.sys.Environment",
            "fuchsia.sys.Launcher"
        ]
    }
}
```

These services and features ensure that OpaqueTest can set up a hermetic
environment and launch component manager.

## Usage

In the simplest case, a test can be started as follows:

```rust
let test = OpaqueTest::default("fuchsia-pkg://fuchsia.com/foo#meta/root.cm").await?;
```

By the end of this statement:

- A component manager instance has been created in a hermetic environment.
- The root component is specified by the given URL.
- Component manager is waiting to be unblocked by the `EventSource`.
- The root [component manifest][concepts-manifest] (`root.cm`) has been resolved.
- No component has been started.
- Component manager’s outgoing directory is serving:
  - The hub of the root component at `$out/hub`.
  - The `EventSource` FIDL service at
    `$out/svc/fuchsia.sys2.EventSource`.
- The state of the hub reflects the following:
  - Static children of the root component should be visible.
  - Grandchildren of the root component should not be visible (because they
  haven't been resolved yet).
  - There should be no `exec` directories for any component.

Use the `EventSource` FIDL service to subscribe to events and unblock
the component manager. The following example shows you how to use the
`EventSource` service:

```rust
let event_source = test.connect_to_event_source().await?;
let event_stream = event_source.subscribe(vec![Stopped::TYPE]).await?;
event_source.start_component_tree().await?;
```

By the end of this code block:

- An `event_stream` has been created that receives `Stopped` events.
- Component manager’s execution has begun.
- The root component (and its eager children, if any) will be started soon.

## Custom tests

In some cases, you may want to customize `OpaqueTest::default`.
Use an `OpaqueTestBuilder` to specify:

- The component manager manifest to be used for the test.

- Additional directories to be created in component manager's namespace.

- A file descriptor to redirect output from components.

- The configuration file to be used by component manager.

- Additional command-line args to be used by component manager.

```rust
let test = OpaqueTestBuilder::new("fuchsia-boot:///#meta/root.cm")
    .component_manager_url("fuchsia-pkg://fuchsia.com/base_resolver_test#meta/component_manager_without_loader.cmx")
    .add_dir_handle("/boot", pkg_channel.into())
    .build()
    .await?;
```

## EventSource

An `EventSource` is used to subscribe to system events sent by component manager.

Since the `EventSource` is built on top of system events:

- A subscription can only be set on a system event.
- It supports all system events in component manager.
- It can be scoped down to a [realm][concepts-realm] of the component hierarchy.
- It follows the component manager’s rules of event propagation (i.e - an
event dispatched at a child realm is also dispatched to its parent).

Note: When component manager is in [debug mode](#debug-mode), an `EventSource`
is installed at the root. Hence it receives events from all components.

For reliable state verification, a test must be able to:

- Expect or wait for various events to occur in component manager.
- Halt the component manager task that is processing the event.

The workflow for an `EventSource` looks something like this:

```rust
// Create an EventSource using ::new_sync() or use the source
// provided by OpaqueTest
let test = OpaqueTest::default("fuchsia-pkg://fuchsia.com/foo#meta/root.cm").await?;

// Get an event stream of the `Started` event.
let event_source = test.connect_to_event_source().await?;
let event_stream = event_source.subscribe(vec![Started::TYPE]).await?;

// Unblock component manager.
event_source.start_component_tree().await;

// Wait for an event
let event = EventMatcher::ok().expect_match::<Started>(&mut event_stream).await;

// Verify state
...

// Resume from event
event.resume().await?;
```

Note: Subscribing to an event stream after the component tree has been started is racy.
`start_component_tree()` consumes the `EventSource` object to prevent future subscriptions.

Calling `resume()` on an event unblocks component manager and allows it to proceed with the
event dispatch.

Note: It is not strictly necessary to invoke `resume()` on an event. When the event object
goes out of scope, `resume()` is called implicitly.

### Scoping of events

The `EventSource` FIDL protocol can be requested by any component instance within the
component topology and is served by the component manager.

Events are capailities themselves so they have to be routed as well. Refer
to [event capabilities][event-capabilities] for more details on this.

A component instance can request a scoped `EventSource` in its manifest
file as follows:

```json5
{
    program: {
        binary: "bin/client",
    },
    use: [
        {
            protocol: [
                "fuchsia.sys2.EventSource",
            ],
            from: "framework"
        },
        {
          event: [ "started", "stopped" ],
          from: "framework",
        }
    ],
}
```

Note: To receive asynchronous events, use the `EventSource` FIDL protocol instead.
Asynchronous events do not block component manager and the events do not have `resume()`
methods on them.

Another component can pass along its scope of system events by passing along the
`EventSource` capability through the conventional routing operations `offer`,
`expose` and `use`.

If a component requests a `EventSource` then its children cannot start until it explicitly
calls `start_component_tree()`.

### Additional functionality

With complex component hierarchies, event propagation is hard to predict and
may even be non-deterministic due to the asynchronous nature of component
manager. To deal with these cases, `EventSource` offers the following additional
functionality:

- [Multiple event streams](#multiple-event-streams)
- [Discardable event streams](#discardable-event-streams)
- [Event sequences](#event-sequences)
- [Capability injection](#capability-injection)
- [Capability interposition](#capability-interposition)
- [Event logs](#event-logs)

#### Multiple event streams {#multiple-event-streams}

It is possible to register multiple event streams, each listening to their own set
of events:

```rust
// Started and CapabilityRouted events can be interleaved,
// so use different event streams.
let start_event_stream = event_source.subscribe(vec![Started::TYPE]).await?;
let route_event_stream =
    event_source.subscribe(vec![CapabilityRouted::TYPE]).await?;

// Unblock component manager
event_source.start_component_tree().await;

// Expect 5 components to start
for _ in 1..=5 {
    let event = EventMatcher::ok().expect_match::<Started>(&mut start_event_stream).await;
    event.resume().await?;
}

// Expect a CapabilityRouted event from ./foo:0
let event = EventMatcher::ok()
    .moniker("./foo:0")
    .expect_match::<CapabilityRouted>(&mut route_event_stream)
    .await;
event.resume().await?;
```

#### Discardable event streams {#discardable-event-streams}

It is possible to listen for specific events and then discard the event stream,
causing future events to be ignored:

```rust
// Subscribe to Stopped events
let stop_event_stream = event_source.subscribe(vec![Stopped::TYPE]).await?;

{
    // Temporarily subscribe to CapabilityRouted events
    let route_event_stream = event_source.subscribe(vec![CapabilityRouted::TYPE]).await?;

    // Expect a CapabilityRouted event from ./bar:0
    let event = EventMatcher::ok().moniker("./bar:0").expect_match::<CapabilityRouted>(&mut route_event_stream).await;
    println!("/bar:0 used capability -> {}", event.capability_id);
    event.resume().await?;
}

// At this point, the test does not care about CapabilityRouted events, so the
// event stream can be dropped. If the event stream were left instantiated,
// component manager would halt on future CapabilityRouted events.

// Expect a Stopped event
let event = EventMatcher::ok().expect_match::<Stopped>(&mut stop_event_stream).await?;
println!("{} was stopped!", event.target_moniker);
event.resume().await?;
```

#### Event sequences {#event-sequences}

When writing tests, it is useful to expect events to occur in some order.
Event Sequences allow writers to verify ordering of events:

```rust
// This test expects the following events to occur:
// 1. the two trigger components stop in any order
// 2. the parent component stops
// 3. the two trigger components are destroyed in any order
// 4. the parent component is destroyed
let expectation = EventSequence::new()
    .all_of(
        vec![
            EventMatcher::ok().r#type(Stopped::TYPE).moniker("./coll:parent:1/trigger_a:0"),
            EventMatcher::ok().r#type(Stopped::TYPE).moniker("./coll:parent:1/trigger_b:0"),
        ],
        Ordering::Unordered,
    )
    .then(EventMatcher::ok().r#type(Stopped::TYPE).moniker("./coll:parent:1"))
    .all_of(
        vec![
            EventMatcher::ok().r#type(Destroyed::TYPE).moniker("./coll:parent:1/trigger_a:0"),
            EventMatcher::ok().r#type(Destroyed::TYPE).moniker("./coll:parent:1/trigger_b:0"),
        ],
        Ordering::Unordered,
    )
    .then(EventMatcher::ok().r#type(Destroyed::TYPE).moniker("./coll:parent:1"))
    .subscribe_and_expect(&mut event_source)
    .await
    .unwrap();

// Start the component tree
event_source.start_component_tree().await;

// Wait for the event sequence to occur
expectation.await.unwrap();
```

#### Capability injection {#capability-injection}

Several tests need to mock out capabilities that a component connects to in the test.
Sometimes, tests may wish to communicate with components directly. The simplest way to do
this is to implement an `Injector`.

```rust
/// Client <---> EchoCapability
/// EchoCapability implements the Echo protocol and responds to clients.
struct EchoCapability;

#[async_trait]
impl Injector for EchoCapability {
    type Marker = fecho::EchoMarker;

    async fn serve(self: Arc<Self>, mut request_stream: fecho::EchoRequestStream) {
        // Start listening to requests from client
        while let Some(Ok(fecho::EchoRequest::EchoString { value: Some(input), responder })) =
            request_stream.next().await
        {
            // Respond to the client with the echo string.
            responder.send(Some(&input)).expect("failed to send echo response");
        }
    }
}
```

Injectors can automatically install themselves on `CapabilityRouted` events.

```rust
let echo_capability: Arc<EchoCapability> = EchoCapability::new();

// Inject the Echo capability when /foo:0 successfully connects to the Echo service
echo_capability.inject(&event_source, EventMatcher::ok().moniker("/foo:0")).await;

event_source.start_component_tree().await?;
```

#### Capability interposition {#capability-interposition}

Tests may want to silently observe or mutate messages between a client
and service. It is possible to interpose a capability and manipulate the traffic
over the channel. Consider an interposer for an Echo service that mutates the input from
the client before sending it to the service:

```rust
/// Client <---> EchoInterposer <---> Echo service
/// The EchoInterposer copies all echo responses from the service
/// and sends them over an mpsc::Channel to the test.
struct EchoInterposer;

#[async_trait]
impl Interposer for EchoInterposer {
    type Marker = fecho::EchoMarker;

    async fn interpose(
        self: Arc<Self>,
        mut from_client: fecho::EchoRequestStream,
        to_service: fecho::EchoProxy,
    ) {
        // Start listening to requests from client
        while let Some(Ok(fecho::EchoRequest::EchoString { value: Some(input), responder })) =
            from_client.next().await
        {
            // Copy the response from the service and send it to the test
            let modified_input = format!("{} Let there be chaos!", input);

            // Forward the request to the service and get a response
            let out = to_service
                .echo_string(Some(&modified_input))
                .await
                .expect("echo_string failed")
                .expect("echo_string got empty result");

            // Respond to the client with the response from the service
            responder.send(Some(out.as_str())).expect("failed to send echo response");
        }
    }
}
```

Interposers can automatically install themselves on `CapabilityRouted` events.

```rust
let interposer = EchoInterposer::new();

// Interpose the Echo capability when any component successfully connects to the Echo service
echo_interposer.interpose(&event_source, EventMatcher::ok()).await;

event_source.start_component_tree().await?;
```

#### Event logs {#event-logs}

It is possible to record events of certain types asynchronously and flush them at a later
point in time:

```rust
let event_stream = event_source.subscribe(vec![Destroyed::TYPE]).await?;
let event_log = EventLog::record_events(&mut event_source, vec![Started::NAME]).await?;
event_source.start_component_tree().await;

// Wait for the component to be destroyed
let event = EventMatcher::ok().expect_match::<Destroyed>(&mut event_stream).await;
event.resume().await?;

// Flush events from the log
let events = event_log.flush().await;

// Verify that the 3 components were started in the correct order
assert_eq!(events, vec![
    RecordedEvent {
        event_type: Started::TYPE,
        target_moniker: "./".to_string()
    },
    RecordedEvent {
        event_type: Started::TYPE,
        target_moniker: "./foo:0".to_string()
    },
    RecordedEvent {
        event_type: Started::TYPE,
        target_moniker: "./foo:0/bar:0".to_string()
    }
]);
```

Note: Recording of events will continue until the `EventLog` object goes out
of scope.

## Debug Mode {#debug-mode}

Both `OpaqueTest` and `EventSource` rely on component manager’s
debug mode.

Component manager's startup behavior can be configued via the `--config` flag
which accepts a path to a JSON configuration file.

To start component manager in debug mode, use the `--config` flag to pass in a
JSON configuration file that has `debug` set to `true`. This is exactly what
`OpaqueTest::default` does.

When component manager is in debug mode, it does the following:

1. Creates the root realm and built-in services.

1. Creates the hub and the `EventSource`.

1. Serves the following from component manager's outgoing directory:

   - The hub of the root component at `$out/hub`.

   - The `EventSource` FIDL service at
   `$out/svc/fuchsia.sys2.EventSource`.

1. Waits to be unblocked by the `EventSource` FIDL service.

1. Starts up the root component (including any eager children).

[concepts-hub]: /docs/concepts/components/v2/hub.md
[concepts-manifest]: /docs/concepts/components/v2/component_manifests.md
[concepts-realm]: /docs/concepts/components/v2/realms.md
[event-capabilities]: /docs/concepts/components/v2/capabilities/event.md
[testing-overview]: /docs/development/testing/components/README.md
