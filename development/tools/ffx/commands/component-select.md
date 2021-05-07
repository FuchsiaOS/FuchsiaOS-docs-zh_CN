# Command select

The Remote Control Service offers an API for querying and connecting
to arbitrary FIDL services on the target.

Queries can match an arbitrary number of services on the system, and `select`
will output all of the matches, formatted according to their place in the
component topology.

In the special case of a query that uniquely matches a single service, RCS can
connect to that service and pass a handle to it to the host for FFX to use.
This is how the the [plugin system](/docs/development/tools/ffx/development/proxy-plugin.md) is able to create FIDL
proxies using the component selector mapping.

To query for services on a target, write a
[selector](https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#Selector) to
match the service(s) of interest.

### Selector syntax

Many `ffx` commands that interact with components or services take component
selectors as a parameter. Component selectors in `ffx` use the same syntax as
the component selectors in the
[diagnostics library](https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#Selector).

Here are some example selectors, all of which select the Remote Control Service:

```
core/remote-control:out:fuchsia.developer.remotecontrol.RemoteControl
core/*:out:fuchsia.developer.remotecontrol.RemoteControl
core/*:expose:fuchsia.developer.remotecontrol.RemoteControl
core/remote-control:out:*
core/*:expose:fuchsia.developer.remotecontrol.R*
```


#### Selector segments

There are three constituent segments of a selector:
`<component moniker>:<node selector>:<property selector>`. Each is discussed
below. The wildcard (`*`) is valid in each segment of a selector.

Consider this example:
  `core/remote-control:out:fuchsia.developer.remotecontrol.RemoteControl`

- `core/remote-control` is the
  [component moniker](/docs/concepts/components/v2/monikers.md).
  This uniquely specifies a path in the
  [component topology](/docs/concepts/components/v2/topology.md).

  Note: You can use `*` to wildcard a particular level in the topology, but *`*`
  is not recursive*. That is, `a/*/c` will match `a/b/c` but would not match
  `a/b/b2/c`.

- `out` is the node selector. In `ffx`, this must be one of the following
  values, which correspond to the routing terminology used in the component
  manifest and defined in
  [routing terminology documentation](/docs/concepts/components/v2/component_manifests.md#routing-terminology).
  - `out`: services offered by the component. Corresponds to `offer` in the
    component manifest.
  - `expose`: services exposed by the component. Corresponds to `expose` in the
    component manifest.
  - `in`: services depended upon by the component. Corresponds to `use` in the
    component manifest.
- `fuchsia.developer.remotecontrol.RemoteControl` is the property selector and
  is matched against fully-qualified FIDL service names in the routing
  directories matched by the node selector.

You may optionally omit the property selector: `core/remote-control:out` is
equivalent to `core/remote-control:out:*`.