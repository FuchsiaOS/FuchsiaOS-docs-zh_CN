# Troubleshooting components (Components v2) {#troubleshooting-components}

<<../../../concepts/components/_v2_banner.md>>

This document contains tips for troubleshooting the following kinds of problems
when using the [component framework][doc-intro]:

- [Error when trying to use a capability from the namespace](#troubleshoot-use)
- [Test does not start](#troubleshoot-test)

## Got an error when trying to use a capability from the namespace {#troubleshoot-use}

Sometimes, when connecting to a capability such as a [protocol][doc-protocol],
[service][doc-service], or [directory][doc-directory] in your
[namespace][glossary-namespace], the channel returns an error when you try to
use it. For example, consider the following snippet:

```rust
use fuchsia_component::client;
use log::info;
...
let echo = client::connect_to_protocol::<fidl_fuchsia_echo::EchoMarker>().expect("error connecting to echo");
if let Some(err) = echo.echo_string(Some("Hippos rule!")).await {
    info!("Echo failed: {}", err);
}
```

In this Rust example, the code connects to the `Echo` protocol in the namespace
through the usual means, by calling the `connect_to_protocol` API in the
`fuchsia_component` crate. This call should succeed as long as the protocol was
mapped into the component's namespace by a `use` declaration in the component's
[manifest][doc-manifests]:

```json5
use: [
    { protocol: "/svc/fuchsia.echo.Echo" },
    ...
],
```

However, when the `connect_to_protocol` call returns successfully, it does not
necessarily mean the protocol will be available. If it's not available, the
usual symptom is that a call to the protocol over the channel fails. The
snippet above checks for this and logs the error.

There are a few conditions that can cause these errors:

- [Channel was closed after connecting to a capability in the namespace](#troubleshoot-use-routing)
- [Component fails to start](#troubleshoot-use-start)
- [Component terminated or closed the channel](#troubleshoot-use-terminated)

### Channel was closed after connecting to a capability in the namespace {#troubleshoot-use-routing}

When a protocol or service is opened in the namespace, or a directory in the
namespace is used for the first time, component manager will perform
[capability routing][doc-routing] to find the source of the capability. It's
possible that routing will fail if one of the component manifests in the
routing path was configured incorrectly. For example, it's possible that an
offer or expose declaration is missing from some component in the path, or one
of the components in the chain could not be resolved.

There are a couple ways to check if a routing failure was the cause of channel
closure:

- Check for an [epitaph][doc-epitaphs] on the closed channel.
- Check the component manager logs with `fx log --only component_manager`

See [checking a closed channel](#troubleshoot-closed-channel) for details on
how to check if a channel was closed and get an epitaph if there was one.
Normally, the epitaph set for a routing failure is `ZX_ERR_UNAVAILABLE`.

For a more detailed description of the error, check the kernel debuglog. Look
for a message beginning with `ERROR: Failed to route` that contains the
requesting component's [moniker][doc-monikers]. This error should give you a
hint about what went wrong. Example:

```
 [component_manager] ERROR: Failed to route protocol `/svc/fuchsia.echo.Echo`
 from component `/core:0/echo_client:0`: A `use from realm` declaration was
 found at `/echo_client:0` for `/svc/fuchsia.echo.Echo`, but no matching
 `offer` declaration was found in the parent
```

Depending on where the component runs the log may be tagged as belonging to the
component, for example `[my_component]` instead of `[component_manager]`. For a
self-contained example of failed routing that demonstrates the content of this
section, refer to
[//examples/components/routing_failed][example-routing-failed].

### Component fails to start {#troubleshoot-use-start}

It's possible that the capability was [routed](#troubleshoot-use-routing)
successfully, but something went wrong when the [runner][doc-runners] tried to
start the component. Here's a couple ways this can happen:

- The [`program`][doc-manifests-program] declaration was misconfigured. For
  example, the binary's path was spelled incorrectly.
- The binary or some other resource needed to start the component was not
  included in its [package][doc-packages].

When this happens, the runner closes the channel with a `PEER_CLOSED` status,
with no epitaph.  See [checking a closed channel](#troubleshoot-closed-channel)
for details on how to check if a channel was closed and get an epitaph if there
was one.

Note that just from the state of the channel, it's impossible to distinguish
whether the runner failed to start the component, or the [component terminated
or closed the channel itself](#troubleshoot-use-terminated).

For a more detailed description of the error, check the logs. The log to check
depends on the runner:

- For the ELF runner, check the component manager logs with `fx log --only
  component_manager`
- For other runners, check the [logs][doc-logs] of the runner component. You
  can do this by running `fx log --tag <runner-name>`.

The form of the error message is runner-dependent. For the ELF runner, look for a message starting
with `ERROR: Failed to start component`:

```
[component_manager] ERROR: Failed to start component
`fuchsia-pkg://fuchsia.com/components-routing-failed-example#meta/echo_server_bad.cm`: unable to
load component with url
"fuchsia-pkg://fuchsia.com/components-routing-failed-example#meta/echo_server_bad.cm": error
loading executable: "reading object at "bin/routing_failed_echo_server_oops" failed: A FIDL
client's channel was closed: PEER_CLOSED"
```

In this case, the component failed to start because its binary was not present.

For an example of a component that failed to start due to a misconfigured
component manifest, refer to
[//examples/components/routing_failed][example-routing-failed].

### Component terminated or closed the channel {#troubleshoot-use-terminated}

If you have verified that [routing succeeded](#troubleshoot-routing) and the
[component started successfully](#troubleshoot-use-start), then the final
possibility is that the source component closed the channel itself. This can
happen while the component was running, or can be a side effect of the
component terminating.

If the component terminated because it crashed, you can look for a crash report
in `fx log` that starts like this:

```
[00177.191] 01775:02371> crashsvc: exception received, processing
[00177.191] 01775:02371> <== fatal : process echo_client.cm[21090] thread initial-thread[21092]
<stack trace follows...>
```

Note that you'll see name of the component manifest in the dump (this is
actually the process name).

If the component closed the channel itself, there's no universal way to debug
if this happened. You can look in the component's [logs][doc-logs], or in the
case of a protocol capability, search the source code for the name of the
source code in a language-appropriate format. For example, for the
`fuchsia.Echo` protocol in Rust, you might search for a `use` statement for
`fidl_fuchsia_echo`, then follow the identifier to where it's used.

The final possibility is that a component may have already been started by a
previous capability request, but has since terminated on its own.

### Checking if a channel was closed {#troubleshoot-closed-channel}

If a protocol channel was closed, you'll normally notice when trying to make a
call on it, if the call is awaited on. For example:

```rust
let res = echo.echo_string(Some("Hippos rule!")).await;
match res {
    Ok(_) => { info!("Call succeeded!"); }
    Err(fidl::Error::ClientChannelClosed { status, service_name } => {
        error!("Channel to service {} was closed with status: {}", service_name, status);
    }
    Err(e) => {
        error!("Unexpected error: {}", e);
    }
};
```

If the call doesn't return a value (i.e. it is a one-way method), you'll only
get an error if the channel was closed prior to the call. However, if your
protocol pipelines a call that does return a value, you can also check that:

```rust
let (echo_resp, echo_resp_svc) = fidl::endpoints::create_proxy();
let res = echo_async.echo_string_pipelined(Some("Hippos rule!"), echo_resp_svc);
match res {
    Ok(_) => {
        info!("EchoString succeeded!");
    }
    Err(fidl::Error::ClientChannelClosed { status, service_name } => {
        error!("Channel to service {} was closed with status: {}", service_name, status);
    }
    Err(e) => {
        error!("Unexpected error: {}", e);
    }
};
let res = echo_resp.get_result().await;
match res {
    Ok(_) => { info!("GetResult succeeded!"); }
    Err(fidl::Error::ClientChannelClosed { status, service_name } => {
        error!("Channel to service {} was closed with status: {}", service_name, status);
    }
    Err(e) => {
        error!("Unexpected error: {}", e);
    }
};
```

If `echo_resp` is closed, it's likely that's indirectly because `echo_async` was closed.

In the case of [routing failure](#troubleshoot-use-routing), component manager
sets an [epitaph][doc-epitaphs] on the channel that was opened through the
namespace.  You can get the epitaph on a closed channel as follows:

```rust
let stream = echo.take_event_stream();
match stream.next().await {
    Some(Err(fidl::Error::ClientChannelClosed { status, .. })) => {
        info!("Echo channel was closed with epitaph, probably due to \
              failed routing: {}", status);
    }
    Some(m) => {
        info!("Received message other than epitaph or peer closed: {:?}", m);
    }
    None => {
        info!("Component failed to start or Echo channel was closed by server");
    }
}
```

Note: in the `echo_async` example, the epitaph would be set on `echo_async`,
not `echo_resp`.

## Test does not start {#troubleshoot-test}

A Components v2 test is written using the [Test Runner Framework][doc-trf].
Sometimes, if one of the test components is configured incorrectly, this can
result in the test failing to run.

If this happens, you'll see an error like the following from `fx test`:

```
Test suite encountered error trying to run tests: getting test cases
Caused by:
    The test protocol was closed. This may mean `fuchsia.test.Suite` was not configured correctly.
    Refer to: https://fuchsia.dev/fuchsia-src/development/components/v2/troubleshooting#troubleshoot-test
```

Misconfigurations can happen in a few test-specific ways:

- [The test failed to expose `fuchsia.test.Suite` to test manager](#troubleshoot-test-root)
- [The test driver failed to expose `fuchsia.test.Suite` to the root](#troubleshoot-test-routing)
- [The test driver does not use a test runner](#troubleshoot-test-runner)

If you're still seeing the same error after trying the preceding solutions, consider following
[the troubleshooting steps for using capabilities](#troubleshoot-use). The troubleshooting steps may
help fix issues from routing the `fuchsia.test.Suite` capability in integration tests.

### The test failed to expose `fuchsia.test.Suite` to test manager {#troubleshoot-test-root}

This happens when the test root fails to expose `fuchsia.test.Suite` from the
[test root][doc-trf-root]. The simple fix is to add an `expose` declaration:

```json5
// test_root.cml
expose: [
    ...
    {
        protocol: "/svc/fuchsia.test.Suite",
        from: "self",  // If a child component is the test driver, put `from: "#driver"`
    },
],
```

### The test driver failed to expose `fuchsia.test.Suite` to the root {#troubleshoot-test-routing}

If the [test driver][doc-trf-driver] and [test root][doc-trf-root] are
different components, the test driver must also expose `fuchsia.test.Suite` to
its parent, the test root.

Make sure this is in the driver's CML:

```json5
// test_driver.cml
expose: [
    ...
    {
        protocol: "/svc/fuchsia.test.Suite",
        from: "self",
    },
],
```

If this is the problem, you can expect to see an error like this in the logs:

```
ERROR: Failed to route protocol `/svc/fuchsia.test.Suite` from component
`/test_manager:0/...`: An `expose from #driver` declaration was found at `/test_manager:0/...`
for `/svc/fuchsia.test.Suite`, but no matching `expose` declaration was found in the child
```

### The test driver does not use a test runner {#troubleshoot-test-runner}

The [test driver][doc-trf-driver] must use the appropriate [test
runner][doc-trf-runner] corresponding to the language and test framework the
test is written with. For example, the driver of a Rust test needs the
following declaration:

```json5
// test_driver.cml
include: [ "src/sys/test_runners/rust/default.shard.cml" ]
```


Also, if the test driver is a child of the [test root][trf-test-root], you need
to offer it to the driver:

```json5
// test_root.cml
offer: [
    {
        runner: "rust_test_runner",
        to: [ "#driver" ],
    },
],
```

[doc-directory]: /docs/concepts/components/v2/capabilities/directory.md
[doc-epitaphs]: /docs/reference/fidl/language/wire-format/README.md#epitaphs
[doc-trf-driver]: /docs/concepts/testing/v2/test_runner_framework.md#test-roles
[doc-trf-root]: /docs/concepts/testing/v2/test_runner_framework.md#tests-as-components
[doc-trf-runner]: /docs/concepts/testing/v2/test_runner_framework.md#test-runners
[doc-trf]: /docs/concepts/testing/v2/test_runner_framework.md
[doc-intro]: /docs/concepts/components/v2/introduction.md
[doc-logs]: /docs/concepts/diagnostics/logs/README.md
[doc-manifests-program]: /docs/concepts/components/v2/component_manifests.md#program
[doc-manifests]: /docs/concepts/components/v2/component_manifests.md
[doc-monikers]: /docs/concepts/components/v2/monikers.md
[doc-packages]: /docs/concepts/packages/package.md
[doc-protocol]: /docs/concepts/components/v2/capabilities/protocol.md
[doc-routing]: /docs/concepts/components/v2/component_manifests.md#routing
[doc-runners]: /docs/concepts/components/v2/capabilities/runners.md
[doc-service]: /docs/concepts/components/v2/capabilities/service.md
[example-routing-failed]: /examples/components/routing_failed/README.md
[glossary-components-v2]: /docs/glossary.md#components-v2
[glossary-namespace]: /docs/glossary.md#namespace
