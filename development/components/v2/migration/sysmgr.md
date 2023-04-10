# Sysmgr configuration

sysmgr is one of the two major pieces of Components v1 (appmgr being the other).
It is responsible for hosting the `sys` [realm](/glossary/README.md#realm)
that contains `global` system services.

At runtime, sysmgr loads all files present under /config/data in its namespace
and parses them using the format described below. This directory is provided to
sysmgr because it uses the 'config-data' feature in its component manifest,
sysmgr.cmx. For more details, see the docs on
[the config-data feature](/development/components/data.md).

sysmgr services are registered with a GN `config_data` target that is defined
and included in [product configurations](/products/README.md) to include an
extra file in sysmgr's /config/data directory.

For example:

```gn
config_data("my_service_config") {
  for_pkg = "sysmgr"
  sources = "my_service.config"
}
```

And then in the appropriate `product.gni`:

```gn
base_package_labels += [
  ...
  "//path/to:my_service_config",
  ...
]
```

## sysmgr configuration format

sysmgr's configuration files are in JSON format. Each config file should have a
single top-level JSON object, and the following keys are supported:

*   `services`
*   `startup_services`
*   `apps`

The contents of all sysmgr config files are read from sysmgr's /config/data
directory and merged at runtime to form sysmgr's overall configuration.

### `services`

This is the most common configuration key and where most of sysmgr's
configuration comes from. Each entry in the `services` map consists of a service
name and the component URL which provides it. Optionally, command line arguments
can be provided to the component by using an array instead of a string value.

```json
{
  "services": {
    "fuchsia.foo.Service": "fuchsia-pkg://fuchsia.com/foo#meta/foo.cmx",
    "fuchsia.bar.Service": [
        "fuchsia-pkg://fuchsia.com/bar#meta/bar.cmx", "arg1", "arg2", "arg3"
    ]
  }
}
```

The combined `services` map across all config files defines the list of services
that are available in the `sys` realm and that are available for other
components running in the `sys` realm to request in their component manifests.
It is not possible to add services to the `sys` realm in any other way; the list
of services is fixed at realm creation time.

Components in the `services` map are started lazily as the services they provide
are connected to. In other words, in the example above `foo.cmx` will not be
started until another component attempts to connect to `fuchsia.foo.Service`. If
your component needs to be started eagerly, see `startup_services` or `apps`
below.

### `startup_services` and `apps`

Both of these keys perform similar functions; they cause sysmgr to eagerly
launch components right after it creates the `sys` realm. The key difference is
that `startup_services` should be used to eagerly launch a component which is
providing services to the `sys` realm (i.e. there is an entry in the `services`
map that uses the component), whereas `apps` should be used to eagerly launch a
component which does not provide any services to `sys`.

For example, the following configuration would cause sysmgr to eagerly launch
the component that provides `fuchsia.foo.Service` (the same as if another
component attempted to connect to the service) as well as an instance of
`bar.cmx` and `baz.cmx` with the given arguments:

```json
{
  "services": {
    "fuchsia.foo.Service": "fuchsia-pkg://fuchsia.com/foo#meta/foo.cmx"
  },
  "startup_services": [
    "fuchsia.foo.Service"
  ],
  "apps": [
    "fuchsia-pkg://fuchsia.com/bar#meta/bar.cmx",
    [ "fuchsia-pkg://fuchsia.com/baz#meta/baz.cmx", "arg1", "arg2", "arg3" ]
  ]
}
```

It is important to not mix up usage of `startup_services` and `apps`. Using
`apps` instead of `startup_services` can result in sysmgr starting two separate
instances of your component, which is likely unintended. For example, the
example below would result in two instances of foo.cmx, one started eagerly and
the other started lazily when another component connects to
`fuchsia.foo.Service`:

```json
// WARNING: This is an example of what NOT to do.
{
  "services": {
    "fuchsia.foo.Service": "fuchsia-pkg://fuchsia.com/foo#meta/foo.cmx"
  }
  "apps": [
    "fuchsia-pkg://fuchsia.com/foo#meta/foo.cmx"
  ]
}
```
