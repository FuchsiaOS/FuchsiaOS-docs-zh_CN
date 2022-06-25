# Component manifests (Components v1)

<<../_v1_banner.md>>

A component manifest (.cmx) is a JSON file with the file extension `.cmx`.
Component manifests are often located in a package’s `meta/` directory. The
manifest contains information that declares how to run the component and what
resources it receives. In particular, the component manifest describes how
the component is sandboxed.

Here's a simple example of a cmx for an ELF binary component:

```
{
    "include": [
        "syslog/client.shard.cmx"
    ],
    "program": {
        "binary": "bin/example_app",
        "args": [ "--example", "args" ]
    },
    "sandbox": {
        "system": [ "data/sysmgr" ],
        "services": [
            "fuchsia.posix.socket.Provider",
            "fuchsia.sys.Launcher"
        ]
    }
}
```

And one for a flutter/dart component:

```
{
    "program": {
        "data": "data/simple_flutter"
    },
    "runner": "flutter_jit_runner"
}
```

## include

The optional `include` property describes zero or more other component manifest
files (or shards) to be merged into this component manifest.

In the example given above, the component manifest is including contents from a
file provided by the `syslog` library, thus ensuring that the component
functions correctly at runtime if it attempts to write to `syslog`. By convention
such files end with `.shard.cmx`.

Include paths prepended with `//` are relative to the source root of the Fuchsia
checkout. However, include paths not prepended with `//`, as in the example
above, are resolved from Fuchsia SDK libraries (`//sdk/lib`) that export
component manifest shards.

For reference, inside the Fuchsia checkout these two include paths are
equivalent:

* `syslog/client.shard.cmx`
* `//sdk/lib/syslog/client.shard.cmx`

You can review the outcome of merging any and all includes into a component
manifest file by invoking the following command:

Note: The `fx` command below is for developers working in a fuchsia source
checkout environment.

```sh
fx cmc include {{ "<var>" }}cmx_file{{ "</var>" }} --includeroot $FUCHSIA_DIR --includepath $FUCHSIA_DIR/sdk/lib
```

Includes are transitive, meaning that shards can have their own includes.

Include paths can have diamond dependencies. For instance this is valid:
A includes B, A includes C, B includes D, C includes D.
In this case A will transitively include B, C, D.

Include paths cannot have cycles. For instance this is invalid:
A includes B, B includes A.
A cycle such as the above will result in a compile-time error.

## program

The `program` property describes the resources to execute the component.

If [`runner`](#runner) is absent, the `program` property is a JSON object with
the following schema:

```
{
    "type": "object",
    "properties": {
        "binary": {
            "type": "string"
        },
        "args": {
            "type": "array",
            "items": {
                "type": "string"
            },
        },
        "env_vars": {
            "type": "array",
            "items": {
                "type": "string"
            },
        },
    }
}
```

The `binary` property describes where in the package namespace to find the
binary to run the component, and the optional `args` property contains the
string arguments to be provided to the process. The optional `env_vars`
property specifies environment variables to provide to the binary where
each element in the array uses the format `"VAR=VALUE"`, for example
`"RUST_BACKTRACE=1"`.

If [`runner`](#runner) is present, `program` is a freeform string-string JSON
object interpreted as args to pass to the runner.

For instance, for a flutter/dart component, its format is:

```
{
    "type": "object",
    "properties": {
        "data": {
            "type": "string"
        }
    }
}
```

Where `data` should describe the location of the flutter/dart binaries. By
default, it is under `data/<component-name>`.

## runner

`runner` is an optional property that names another component (or a package
that contains one) to which execution is to be delegated. The target component
must expose the [`Runner`][runner] service.

If `runner` is present, [`program`](#program) is a freeform string-string JSON
object interpreted as args to pass to the runner.

If `runner` is absent, it is assumed that `program.binary` is an ELF binary or
shell script.

The `runner` property is a JSON string.

## facets

`facets` is an optional property that contains free-form JSON about the
component. Facets can be consumed by things on the system to acquire additional
metadata about a component.

The schema for `facets` is:

```
{
    "type": "object"
}
```

As an example of a facet, the `fuchsia.test` field is used to convey what
additional services should be
[injected into testing environments][test-components].

## sandbox

The `sandbox` property controls the environment in which the component
executes. Specifically, the property controls which directories the component
can access during execution.

The `sandbox` property is a JSON object with the following schema:

```
{
    "type": "object",
    "properties": {
        "dev": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "services": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "system": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "pkgfs": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "features": {
            "type": "array",
            "items": {
                "type": "string"
            }
        }
    }
}
```

All items must be valid paths in canonical form as defined by fuchsia.io/Path.

The `dev` array contains a list of well-known device directories that are
provided to the component. For example, if the string `class/input` appears in
the `dev` array, then `/dev/class/input` will appear in the namespaces of components
loaded from the package. To allow access to a `misc` device, add the string `misc`
to the `dev` array. Allowing access to individual `misc` devices is not possible.

The `system` array contains a list of well-known paths within the system package
that are provided to the component. For example, if the string `bin` appears
in the `system` array, then `/system/bin` will appear in the namespaces of
components loaded from the package.

The `pkgfs` array contains a list of well-known paths within the pkgfs tree
that are provided to the component. For example, if the string `versions`
appears in the `pkgfs` array, then `/pkgfs/versions` will appear in the
namespaces of components loaded from the package, providing access to all
packages fully cached on the system.

The `services` array defines a list of services from `/svc` that the
component may access. A typical component will require a number services from
`/svc` in order to play some useful role in the system. For example, if
`"services" = [ "fuchsia.posix.socket.Provider", "fuchsia.sys.Launcher" ]`, the
component will have the ability to launch other components and access network
services. A component may declare any list of services in its `services`,
but it will only be able to access services present in its
[environment](glossary/README.md#environment). This property should be defined by
all new components, and soon a migration will take place to convert all
components to define `services`.

The `features` array contains a list of well-known features that the package
wishes to use. Including a feature in this list is a request for the environment
in which the contents of the package execute to be given the resources required
to use that feature.

The set of currently known features are as follows:

- `config-data`, which will provide any configuration data available to the
  package this component is in that was provided in the [config-data](development/components/data.md)
  package on the system.

- `introspection`, which requests access to introspect the system. The
  introspection namespace will be located at `/info_experimental`.

- `isolated-persistent-storage`, which requests access to persistent storage for
  the device, located in `/data` in the package's namespace. This storage is
  isolated from the storage provided to other components.

- `isolated-cache-storage`, which requests access to persistent storage for the
  device, located in `/cache` in the package's namespace. This storage is
  isolated from the storage provided to other components. Unlike
  `isolated-persistent-storage`, items placed in the storage provided by this
  feature will be deleted by the system to reclaim space when disk usage is
  nearing capacity.

- `isolated-temp`, which requests that a temp directory be installed into the
  component's namespace at `/tmp`. This is isolated from the system temp and
  the temp directories of other component instances. This directory is backed by
  an in-memory filesystem, and is thus cleared on device reboots.

- `root-ssl-certificates`, which requests access to the root SSL certificates
  for the device. These certificates are provided in the `/config/ssl` directory
  in the package's namespace.

- `hub`, which shows information about the component instance's realm and its
  children in a [directory structure][hub].

- `deprecated-shell`, which requests access to the resources appropriate for an
  interactive command line. Typically, shells are granted access to all the
  resources available in the current environment. The `deprecated-shell` feature
  also implies the `root-ssl-certificates` and `hub` features.
  As the name suggests, this feature is to be removed. Current uses of this
  feature are explicitly allowlisted, and new uses are discouraged.

- `shell-commands`, which requests access to the currently available shell
  binaries (note: not "installed", but "available"). Binaries are mapped into
  `/bin` in the requesters namespace. Running these commands may require the
  `fuchsia.process.Resolver` and `fuchsia.process.Launcher` services also
  be requested.

- `vulkan`, which requests access to the resources required to use the Vulkan
  graphics interface. This adds layer configuration data in the `/config/vulkan`
  directory in the package's namespace.

- `deprecated-ambient-replace-as-executable`, which provides legacy support for
  using the invalid handle with replace_as_executable.

- `durable-data`, which requests access to the read-write durable partition for
  the device and places it at `/durable` in the component's namespace. This
  partition is for storing persistent data that will survive a factory reset,
  and is only to be used for specific, approved use cases.

See [sandboxing](concepts/process/sandboxing.md) for more information about sandboxing.

[hub]: concepts/components/v1/hub.md
[runner]: /sdk/fidl/fuchsia.sys/runner.fidl
[test-components]: concepts/testing/v1_test_component.md
