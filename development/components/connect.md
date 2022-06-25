# Connect components

This document demonstrates how to connect components together using capabilities
and additional tools for parent components to manage their children.

## Concepts

You should understand the following concepts before continuing with this guide:

*   The Component Framework assembles the
    [namespace][glossary.namespace] for a component using
    [component declarations][glossary.component-declaration] that describe the
    [capabilities][glossary.capability] the component requires to function.
    The capabilities the component exposes to others are assembled into an
    [exposed directory][glossary.exposed-directory].
*   Every component receives a handle to the server end of a
    [`Directory`][fidl-fuchsia.io.Directory] channel called the
    [outgoing directory][glossary.outgoing-directory].
    The component's executable makes discoverable any capabilities that it provides
    through this directory.
*   At runtime, the
    [component instance tree][glossary.component-instance-tree] connects individual
    [component instances][glossary.component-instance] together in a hierarchy of
    parent and child relationships. The component instance tree and the capability
    routes over that tree are collectively referred to as the
    [component topology][glossary.component-topology].
*   Parent components declare child components either statically in their
    [component manifest][glossary.component-manifest] or dynamically using a
    [component collection][glossary.component-collection]. A collection is a container
    for dynamic children that may be created and destroyed at runtime using the
    `fuchsia.component.Realm` framework protocol.

For more details on these concepts, see [Realms][doc-realms] and
[Capabilities][doc-capabilities].

## Connecting capabilities {#capabilities}

Note: For a complete example using routed capabilities, see
[`//examples/components/routing`][example-routing].

Components interact with each other through their capabilities. Capabilities implemented
in a component need to be declared in that component's manifest and routed through its
parent/child components. Other components that use that capability also need to declare
their use in their manifests. This capability routing describes which component should act
as the provider for any given client. Once the proper components are identified, the
component manager initiates connections between components.

### Provide a capability implementation {#provide-capability}

Components that implement a capability must declare the implementation in their
component manifest using a [`capabilities`][cml-capabilities] declaration.

See the following example that declares a FIDL [protocol capability][doc-protocol] in the
providing component's manifest:

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_server/meta/echo_server.cml" region_tag="example_snippet" adjust_indentation="auto" highlight="4,5,6,7" %}
```

At runtime, the provider component provides an implementation of the capability by serving
it through the outgoing directory using the [fuchsia.io][fidl-fuchsia.io] protocol.
The generated FIDL bindings wrap this handle and enable the provider to begin receiving
incoming requests:

* {Rust}

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_server/src/main.rs" region_tag="main_body" adjust_indentation="auto" highlight="1,2,3,4,8,14,15,16,21,22,23,24,25,26,27,28" %}

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_server/src/main.rs" region_tag="handler" adjust_indentation="auto" highlight="1,2,3,4,5,6,7" %}
  ```

* {C++}

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_server/main.cc" region_tag="handler" adjust_indentation="auto" highlight="1,2,3,4,5,6" %}

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_server/main.cc" region_tag="main_body" adjust_indentation="auto" highlight="3,9,10,11,12,13,14,15,16,17" %}
  ```

### Connect to routed capabilities {#connect-routes}

Client components request capabilities in their component manifest with a [`use`][cml-use]
declaration.

See the following example of a client component's manifest that uses the FIDL protocol provided
by the previous component:

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_client/meta/echo_client.cml" region_tag="example_snippet" adjust_indentation="auto" highlight="4,5,6,7" %}
```

At runtime, the client component connects to the capability handles populated in its namespace
using the [fuchsia.io][fidl-fuchsia.io] protocol. The Fuchsia component library works
with the generated FIDL bindings to wrap these handles and provide a structured interface
for communicating over the channel:

* {Rust}

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_client/src/main.rs" region_tag="main_body" adjust_indentation="auto" highlight="7,8,9,10,11,12,13,14" %}
  ```

* {C++}

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/cpp/echo_client/main.cc" region_tag="main_body" adjust_indentation="auto" highlight="2,3,4,5,7,8,10" %}
  ```

### Route capabilities {#route-capability}

Components may only access capabilities routed to them. Capabilities can originate
from anywhere in the component topology as long as a valid capability route exists
as a chain of the following declarations from the capability provider to any
consumers:

*   [`expose`][cml-expose]: Routes a capability up to the component's parent.
*   [`offer`][cml-offer]: Routes a capability down to one of the component's children.

To connect capability providers with components requesting those capabilities,
do the following:

1.  Add an `offer` or `expose` declaration to the capability provider component:

    ```json5
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/rust/echo_server/meta/echo_server.cml" region_tag="example_snippet" adjust_indentation="auto" highlight="8,9,10,11,12,13" %}
    ```

1.  For each intermediate component in the component instance tree, include additional
    `expose` and `offer` declarations until you reach the consuming component containing
    a `use` declaration:

    ```json5
    {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/meta/echo_realm.cml" region_tag="example_snippet" highlight="13,14,15,16,17,18,19,21,22,23,24,25,26,27,28,29,30" %}
    ```

## Managing child components {#children}

Note: For a complete example using child components, see
[`//examples/components/lifecycle`][example-lifecycle].

Components can interact with each other from anywhere in the component topology through
capabilities as long as a valid capability route exists between them. There are additional
methods that enable parent components to interact with their direct children.

The following example component declares a single static child named `lifecycle` and
a collection named `echo` where additional child components may be created at runtime:

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/meta/manager.cml" region_tag="example_snippet" adjust_indentation="auto" %}
```

Notice that a collection behaves like a static child instance in the parent component's
manifest — you can give it a name and offer specific capabilities to it. All child
components in the collection may access the set of capabilities offered to it.

### Start child components {#start-child}

The Component Framework provides the [`fuchsia.component.Binder`][fidl-Binder] protocol
for parent components to explicitly start a child that may not expose any other capabilities.
Since this capability is provided by the framework, child components only need to expose it
from their component manifest:

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/meta/lifecycle.cml" region_tag="example_snippet" adjust_indentation="auto" %}
```

### Create dynamic children {#create-child}

To create a new child component at runtime, use the [`fuchsia.component.Realm`][fidl-Realm]
protocol to create the component inside of an existing collection. Call the
[`CreateChild`][fidl-Realm.CreateChild] method with the following parameters:

*   [`CollectionRef`][fidl-decl.CollectionRef]: Describes the collection where the component
    will be added.
*   [`Child`][fidl-decl.Child]: Component declaration, including its name and component URL.

* {Rust}

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/src/manager.rs" region_tag="imports" adjust_indentation="auto" %}

  // ...

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/src/manager.rs" region_tag="create_child" adjust_indentation="auto" %}
  ```

* {C++}

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/cpp/manager.cc" region_tag="imports" adjust_indentation="auto" %}

  // ...

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/cpp/manager.cc" region_tag="create_child" adjust_indentation="auto" %}
  ```

### Connect to child capabilities {#connect-child}

Because the parent of a dynamic component is not known at build time, its exposed capabilities
cannot be named in capability routes expressed in the component manifest.

To connect with the capabilities exposed by a dynamic child instance:

1.  Use the [`fuchsia.component.Realm`][fidl-Realm] protocol to open the child's
    exposed directory. Call the [`OpenExposedDir`][fidl-Realm.OpenExposedDir]
    method with the child component's name and the collection name:

    * {Rust}

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/src/manager.rs" region_tag="imports" adjust_indentation="auto" %}

      // ...
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/src/manager.rs" region_tag="connect_child" adjust_indentation="auto" %}
      ```

    * {C++}

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/cpp/manager.cc" region_tag="imports" adjust_indentation="auto" %}

      // ...

      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/cpp/manager.cc" region_tag="connect_child" adjust_indentation="auto" %}
      ```

2.  Connect to the child's exposed capabilities using the exposed directory handle
    as the root:

    * {Rust}

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/src/manager.rs" region_tag="echo_send" adjust_indentation="auto" %}
      ```

    * {C++}

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/cpp/manager.cc" region_tag="echo_send" adjust_indentation="auto" %}
      ```

### Destroy dynamic children {#destroy-child}

When the dynamic child is no longer needed, use the [`fuchsia.component.Realm`][fidl-Realm]
protocol to destroy the component instance. Call the [`DestroyChild`][fidl-Realm.DestroyChild]
method with a [`ChildRef`][fidl-decl.ChildRef] representing the child inside the collection.

* {Rust}

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/src/manager.rs" region_tag="imports" adjust_indentation="auto" %}

  // ...

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/src/manager.rs" region_tag="destroy_child" adjust_indentation="auto" %}
  ```

* {C++}

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/cpp/manager.cc" region_tag="imports" adjust_indentation="auto" %}

  // ...

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/cpp/manager.cc" region_tag="destroy_child" adjust_indentation="auto" %}
  ```

This causes the component to stop if it is currently running. To handle this event in
your component, see [listen for stop events](#lifecycle-notifications).

## Controlling component lifecycle {#lifecycle}

The Component Framework provides features to modify and interact with various parts of
the component lifecycle.

For more details on lifecycle concepts, see [Component lifecycle][doc-lifecycle].

### Lifecycle notifications {#lifecycle-notifications}

The ELF runner notifies components of lifecycle events using the
[`fuchsia.process.lifecycle.Lifecycle`][fidl-Lifecycle] protocol.

To listen for stop notifications in your child component:

1.  Subscribe to the [lifecycle event][elf-lifecycle] in your component manifest:

    * {Rust}

      ```json5
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/meta/lifecycle.cml" region_tag="lifecycle_event" adjust_indentation="auto" highlight="9,10" %}
      ```

    * {C++}

      ```json5
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/cpp/meta/lifecycle.cml" region_tag="lifecycle_event" adjust_indentation="auto" highlight="9,10" %}
      ```

1.  Register a lifecycle handler using the startup handle provided by the runner:

    * {Rust}

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/src/lifecycle.rs" region_tag="imports" adjust_indentation="auto" %}

      // ...

      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/src/lifecycle.rs" region_tag="lifecycle_handler" adjust_indentation="auto" %}
      ```

    * {C++}

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/cpp/lifecycle.cc" region_tag="imports" adjust_indentation="auto" %}

      // ...

      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/cpp/lifecycle.cc" region_tag="lifecycle_handler" adjust_indentation="auto" %}
      ```

### Start with parent {#eager}

[Component manifests][doc-manifests] let you mark a child as [`eager`][cml-children],
which causes the component framework to implicitly start that child with the parent.

If the eager child fails to start for any reason (such as a missing component),
component manager exhibits the following behavior:

-   If the parent is not the root component, the parent will start but the
    component that bound to it will observe a dropped connection (just like any
    other failed binding).
-   If the parent is the root component, component manager will crash, with an
    error message like:

    ```none {:.devsite-disable-click-to-copy}
    [component_manager] ERROR: Failed to route protocol `fuchsia.appmgr.Startup` with target component `/startup`:
    failed to resolve "fuchsia-pkg://fuchsia.com/your_component#meta/your_component.cm":
    package not found: remote resolver responded with PackageNotFound
    ```

Components marked as `eager` can cause system crashes when they are not present if their
ancestors are also marked `eager` up to the root component. This is important because
many build configurations create system images containing a subset of the available components.
To avoid this problem, declare these components using [**core realm shards**][core-shard] to
ensure they can be safely excluded from test builds and product images.

An `eager` component should also be in the same [package set][doc-package-set] as its parent
since the component will be started at the same time as its parent. Typically, `eager`
components should be in the product's base package set.

To determine if your package is in the base package set, run the following command:

```posix-terminal
fx list-packages --verbose {{ '<var label="package name">my-package</var>' }}
```

This command outputs a list of the package sets where the matching package is found.
For example, `system-update-checker` is in the `base` and `universe` package sets:

```none {:.devsite-disable-click-to-copy}
$ fx list-packages --verbose system-update-checker
system-update-checker [base universe]
```

You can also look at all the packages in the base package set using the `--base` option:

```posix-terminal
fx list-packages --base
```

### Reboot on terminate {#reboot-on-terminate}

[Component manifests][doc-manifests] let you control the termination policy of your component
using [`on_terminate`][cml-children]. Components with the "reboot-on-terminate" policy set
cause the system to gracefully reboot if the component terminates for any reason (including
successful exit).

Note: This is a special feature intended for use only by system components deemed critical
to the system's function. Therefore, its use is governed by a security policy allowlist.
If you believe you need this option, please reach out to the
[Component Framework team][cf-dev-list].

To enable this feature, do the following:

1.  Mark the child as `on_terminate: reboot` in the parent's component manifest:

    ```json5
    // core.cml
    {
        children: [
            ...
            {
                name: "system-update-checker",
                url: "fuchsia-pkg://fuchsia.com/system-update-checker#meta/system-update-checker.cm",
                startup: "eager",
                {{ '<strong>' }}on_terminate: "reboot",{{ '</strong>' }}
            },
        ],
    }
    ```

1.  Add the component's moniker to component manager's security policy allowlist at
    [`//src/security/policy/component_manager_policy.json5`][src-security-policy]:

    ```json5
    // //src/security/policy/component_manager_policy.json5
    {
        security_policy: {
            ...
            child_policy: {
                reboot_on_terminate: [
                    ...
                    "/core/system-update-checker",
                ],
            },
        },
    }
    ```

## Troubleshooting {#troubleshooting}

This section contains common issues you may encounter trying to `use` and connect to
capabilities from your component.

When component connections fail, the underlying FIDL channel closes. FIDL protocol
bindings return an error status if the channel was closed. Consider the following example:

* {Rust}

  ```rust
  let echo = connect_to_protocol::<EchoMarker>()
      .context("Failed to connect to echo service")?;
  let res = echo.echo_string(Some("Hippos rule!")).await;
  match res {
      Ok(_) => { info!("Call succeeded!"); }
      {{ '<strong>' }}Err(fidl::Error::ClientChannelClosed { status, service_name } => { {{ '</strong>' }}
          {{ '<strong>' }}error!("Channel to service {} was closed with status: {}", service_name, status); {{ '</strong>' }}
      {{ '<strong>' }}} {{ '</strong>' }}
      Err(e) => {
          error!("Unexpected error: {}", e);
      }
  };
  ```

* {C++}

  ```cpp
  fuchsia::examples::EchoPtr echo_proxy;
  auto context = sys::ComponentContext::Create();
  context->svc()->Connect(echo_proxy.NewRequest());

  {{ '<strong>' }}// Sets an error handler that will be called if an error causes the underlying {{ '</strong>' }}
  {{ '<strong>' }}// channel to be closed. {{ '</strong>' }}
  {{ '<strong>' }}echo_proxy.set_error_handler([&loop](zx_status_t status) { {{ '</strong>' }}
    {{ '<strong>' }}printf("Channel was closed with status: %d\n", status); {{ '</strong>' }}
    {{ '<strong>' }}// ... {{ '</strong>' }}
  {{ '<strong>' }}}); {{ '</strong>' }}

  echo_proxy->EchoString("Hippos rule!", [&](std::string response) {
    // ...
  });
  ```

Note: If the protocol method doesn't return a value (such as a one-way method),
the error status is only set if the channel was closed prior to the method call.

To determine the underlying cause of a channel closure, you can inspect the optional
[epitaph][doc-epitaphs] set on the channel. To retrieve the epitaph on a closed channel,
do the following:

* {Rust}

  ```rust
  let stream = echo.take_event_stream();
  match stream.next().await {
      Some(Err(fidl::Error::ClientChannelClosed { status, .. })) => {
          info!("Channel was closed with epitaph: {}", status);
      }
      Some(m) => {
          info!("Received message other than epitaph or peer closed: {:?}", m);
      }
      None => {
          info!("Component failed to start or channel was closed by server");
      }
  }
  ```

* {C++}

  ```cpp
  echo_proxy.set_error_handler([&loop](zx_status_t status) {
    // If an Epitaph was present on the channel, its error value will be passed as
    // the parameter.
    printf("Channel was closed with epitaph: %d\n", status);
  });
  ```

### Capability routing failed {#troubleshoot-use-routing}

Component manager performs [capability routing][doc-capabilities] to find the source
of a given capability once your component attempts to access the capability. Routing
can fail if one of the component manifests in the routing path is configured incorrectly.
For example, an `offer` or `expose` declaration is missing from some component in the path,
or one of the components in the chain could not be resolved.

Do the following to check if a routing failure was the cause of channel closure:

*   Check the component logs with `ffx log` for a message beginning with `Failed to route`
    that explains where the routing chain failed. For example:

    ```none {:.devsite-disable-click-to-copy}
    [echo_client][][W] Failed to route protocol
    `fidl.examples.routing.echo.Echo` with target component `/core/ffx-laboratory:echo_realm/echo_client`:
    A `use from parent` declaration was found at `/core/ffx-laboratory:echo_realm/echo_client`
    for `fidl.examples.routing.echo.Echo`, but no matching `offer` declaration was found in the parent
    ```

*   Check for an [epitaph on the closed channel](#troubleshooting).
    Normally, the epitaph set for a routing failure is `ZX_ERR_UNAVAILABLE`:

    ```none {:.devsite-disable-click-to-copy}
    [echo_client][][I] Connecting to Echo protocol failed with error
    "A FIDL client's channel to the service fidl.examples.routing.echo.Echo was closed: UNAVAILABLE"
    ```

For a self-contained example of failed capability routing, see
[`//examples/components/routing_failed`][example-routing-failed].

### Component failed to start {#troubleshoot-use-start}

You may encounter an error if capability routing was successful, but an issue occurred
resolving or starting the component. The form of the error message depends on the
[component runner][doc-runners]:

*   For the ELF runner, check the component manager logs with
    `ffx log --filter component_manager`. Look for a message starting with
    `Failed to start component`. For example:

    ```none {:.devsite-disable-click-to-copy}
    [component_manager] WARN: Failed to start component
    `fuchsia-pkg://fuchsia.com/components-routing-failed-example#meta/echo_server_bad.cm`:
    unable to load component with url "fuchsia-pkg://fuchsia.com/components-routing-failed-example#meta/echo_server_bad.cm":
    error loading executable: "reading object at \"bin/routing_failed_echo_server_oops\" failed:
    A FIDL client's channel to the service (anonymous) File was closed: PEER_CLOSED"
    ```

*   For other runners, check the [logs][doc-logs] of the runner component. You
    can do this by running the following command:

    ```posix-terminal
    ffx log --tags {{ '<var label="component runner">runner-name</var>' }}
    ```

To address the issue, verify the following:

*   The [`program`][cml-program] declaration in your component manifest is configured properly.
    For example, verify that the binary's path is spelled correctly.
*   The binary itself and all other resource needed to start the component are included in the
    [package][doc-packages].

For an example of a component that failed to start due to a misconfigured
component manifest, see [`//examples/components/routing_failed`][example-routing-failed].

### Component terminated or closed the channel {#troubleshoot-use-terminated}

If you have verified that routing succeeded and the component started successfully,
you may be experiencing an issue where the source component closed the channel itself.
This can happen while the component was running, or can be a side effect of the
component terminating.

If the component terminated because it crashed, you can look for a crash report
in `ffx log` that contains the component name in the dump:

```none {:.devsite-disable-click-to-copy}
[33860.645][klog][klog][I] crashsvc: exception received, processing
[33860.645][klog][klog][I] <== fatal : process echo_client.cm[21090] thread initial-thread[21092]
<stack trace follows...>
```

If the source component closed the channel itself, here are some tips to further troubleshoot
the cause:

*   Refer to the source component's [logs][doc-logs] for error messages.
*   Use `ffx debug fidl` to examine the FIDL connection traffic with
    [`fidlcat`][doc-fidlcat] for errors or unexpected behavior.

[cf-dev-list]: https://groups.google.com/a/fuchsia.dev/g/component-framework-dev
[core-shard]: /src/sys/core/README.md
[cml-capabilities]: https://fuchsia.dev/reference/cml#capabilities
[cml-children]: https://fuchsia.dev/reference/cml#children
[cml-expose]: https://fuchsia.dev/reference/cml#expose
[cml-offer]: https://fuchsia.dev/reference/cml#offer
[cml-program]: https://fuchsia.dev/reference/cml#program
[cml-use]: https://fuchsia.dev/reference/cml#use
[doc-capabilities]: /docs/concepts/components/v2/capabilities/README.md
[doc-epitaphs]: /docs/reference/fidl/language/wire-format/README.md#epitaph_control_message_ordinal_0xffffffff
[doc-fidlcat]: /docs/development/monitoring/fidlcat/README.md
[doc-lifecycle]: /docs/concepts/components/v2/lifecycle.md
[doc-logs]: /docs/concepts/components/diagnostics/logs/README.md
[doc-manifests]: /docs/concepts/components/v2/component_manifests.md
[doc-packages]: /docs/concepts/packages/package.md
[doc-package-set]: /docs/concepts/packages/package.md#types_of_packages
[doc-protocol]: /docs/concepts/components/v2/capabilities/protocol.md
[doc-realms]: /docs/concepts/components/v2/realms.md
[doc-runners]: /docs/concepts/components/v2/capabilities/runners.md
[elf-lifecycle]: /docs/concepts/components/v2/elf_runner.md#lifecycle
[example-lifecycle]: /examples/components/lifecycle/
[example-routing]: /examples/components/routing/
[example-routing-failed]: /examples/components/routing_failed/
[fidl-Binder]: https://fuchsia.dev/reference/fidl/fuchsia.component#Binder
[fidl-decl.Child]: https://fuchsia.dev/reference/fidl/fuchsia.component.decl/#Child
[fidl-decl.ChildRef]: https://fuchsia.dev/reference/fidl/fuchsia.component.decl/#ChildRef
[fidl-decl.CollectionRef]: https://fuchsia.dev/reference/fidl/fuchsia.component.decl/#CollectionRef
[fidl-fuchsia.io]: https://fuchsia.dev/reference/fidl/fuchsia.io
[fidl-fuchsia.io.Directory]: https://fuchsia.dev/reference/fidl/fuchsia.io#Directory
[fidl-Lifecycle]: https://fuchsia.dev/reference/fidl/fuchsia.process.lifecycle#Lifecycle
[fidl-Realm]: https://fuchsia.dev/reference/fidl/fuchsia.component#Realm
[fidl-Realm.CreateChild]: https://fuchsia.dev/reference/fidl/fuchsia.component#Realm.CreateChild
[fidl-Realm.DestroyChild]: https://fuchsia.dev/reference/fidl/fuchsia.component#Realm.DestroyChild
[fidl-Realm.OpenExposedDir]: https://fuchsia.dev/reference/fidl/fuchsia.component#Realm.OpenExposedDir
[glossary.capability]: /docs/glossary/README.md#capability
[glossary.component-collection]: /docs/glossary/README.md#component-collection
[glossary.component-declaration]: /docs/glossary/README.md#component-declaration
[glossary.component-instance]: /docs/glossary/README.md#component-instance
[glossary.component-instance-tree]: /docs/glossary/README.md#component-instance-tree
[glossary.component-manifest]: /docs/glossary/README.md#component-manifest
[glossary.component-topology]: /docs/glossary/README.md#component-topology
[glossary.exposed-directory]: /docs/glossary/README.md#exposed-directory
[glossary.namespace]: /docs/glossary/README.md#namespace
[glossary.outgoing-directory]: /docs/glossary/README.md#outgoing-directory
[src-security-policy]: /src/security/policy/component_manager_policy.json5
