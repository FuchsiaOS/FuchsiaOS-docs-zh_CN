# Driver runner

The driver runner is the runner responsible for launching
[components][glossary.component] that run in the driver host environment.

## Using the driver runner

To use the driver runner, the component's manifest must include a `program`
block similar to the following:

```json5 {:.devsite-disable-click-to-copy}
{
    program: {
        runner: "driver",
        binary: "driver/example.so",
        bind: "meta/bind/example.bindbc",
    }
}
```

A driver component's `program` block requires the following fields at a minimum:

-   `runner` – This field must be set to the string `driver`.
-   `binary` – The path to the driver's binary output in the component's
    package.
-   `bind` – The path to the compiled bind program in the component's package.

## Optional fields

In additional to the required fields, the driver runner accepts a set of
optional fields, which are used to specify metadata or configure the runtime
environment of the driver component.

### Colocation

If the `colocate` field is set to the string `true`, the driver will be put in
the same [driver host][driver-host] as its parent driver if possible. However
this is advisory. The [driver manager][driver-manager] may still put the driver
in a separate driver host, for instance, if the parent device has `MUST_ISOLATE`
set. In DFv1, a driver is always colocated if the parent device is a composite –
isolation may still be enforced by setting `MUST_ISOLATE` on the primary
fragment of the composite.

```json5 {:.devsite-disable-click-to-copy}
{
    program: {
        runner: "driver",
        binary: "driver/example.so",
        bind: "meta/bind/example.bindbc",
        {{ '<strong>' }}colocate: "true"{{ '</strong>' }}
    }
}
```

If the `colocate` field is not specified, its value defaults to the string
`false`.

### Default dispatcher options

The `default_dispatcher_opts` field provides the options which are used when
creating the driver's [default dispatcher][driver-dispatcher], for example:

```json5 {:.devsite-disable-click-to-copy}
{
    program: {
        runner: "driver",
        binary: "driver/example.so",
        bind: "meta/bind/example.bindbc",
        {{ '<strong>' }}default_dispatcher_opts: [ "allow_sync_calls" ]{{ '</strong>' }}
    }
}
```

The options in this field correspond to the flags defined in this
[`types.h`][dispatcher-flags] file. Today, the supported options are:

-   `allow_sync_calls`: This option indicates that the dispatcher may not
    share Zircon threads with other drivers. This setting allows the driver
    to make synchronous Banjo or FIDL calls on the dispatcher without
    deadlocking.

### Fallback

If the `fallback` field is set to the string `true`, this fallback driver will
only attempt to bind once all the base driver packages are indexed. Furthermore,
if this driver matches to a node and a non-fallback driver matches to the same
node, the non-fallback driver will bind to the node instead.

```json5 {:.devsite-disable-click-to-copy}
{
    program: {
        runner: "driver",
        binary: "driver/example.so",
        bind: "meta/bind/example.bindbc",
        {{ '<strong>' }}fallback: "true"{{ '</strong>' }}
    }
}
```

If the `fallback` field is not specified, its value defaults to the string
`false`.

### Device categories

The `device_categories` field provides metadata indicating the device categories
that the driver controls, for example:

```json5 {:.devsite-disable-click-to-copy}
{
    program: {
        runner: "driver",
        binary: "driver/example.so",
        bind: "meta/bind/example.bindbc",
        {{ '<strong>' }}device_categories: [
            { category: "board", subcategory: "i2c" },
            { category: "sensor", subcategory: "temperature" },
        ]{{ '</strong>' }}
    }
}
```

This metadata is used to determine the tests that the driver will undergo during
its certification process. See the full list of device categories and
subcategories in the [FHCP schema][fhcp-schema].

## Further reading

For more detailed explanation of how drivers are bound, see
[Driver binding][driver-binding].

<!-- Reference links -->

[glossary.component]: /glossary/README.md#component
[driver-host]: /concepts/drivers/driver_framework.md#driver_host
[driver-manager]: /concepts/drivers/driver_framework.md#driver_manager
[driver-dispatcher]: /concepts/drivers/driver-dispatcher-and-threads.md
[dispatcher-flags]: /sdk/lib/driver/runtime/include/lib/fdf/types.h
[fhcp-schema]: /build/drivers/FHCP.json
[driver-binding]: /concepts/drivers/driver_binding.md
