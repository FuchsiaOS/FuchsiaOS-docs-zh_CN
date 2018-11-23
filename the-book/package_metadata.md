# Fuchsia Package Metadata

The Fuchsia package format contains an extensive metadata directory. This
document describes the metadata extensions that are understood by Fuchsia
itself.

See [https://fuchsia.googlesource.com/garnet/+/master/go/src/pm#Structure-of-a-Fuchsia-Package] for
more information about where these files appear in a package.

## metadata

See [https://fuchsia.googlesource.com/garnet/+/master/go/src/pm#metadata]

## contents

See [https://fuchsia.googlesource.com/garnet/+/master/go/src/pm#contents]

## signature

See [https://fuchsia.googlesource.com/garnet/+/master/go/src/pm#signature]

## Component manifest

A component manifest (.cmx) is a JSON file with the file extension `.cmx`
located in the package’s `meta/` directory with information that declares
how to run the component and what resources it receives. In particular, the
component manifest describes how the component is sandboxed.

Here's a simple example of a cmx for an ELF binary component:

```
{
    "program": {
        "binary": "bin/app"
    },
    "sandbox": {
        "system": [ "data/sysmgr" ],
        "services": [ "fuchsia.sys.Launcher", "fuchsia.netstack.Netstack" ]
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

### program

The `program` property describes the resources to execute the component.

If [`runner`](#runner) is absent, the `program` property is a JSON object with
the following schema:

```
{
    "type": "object",
    "properties": {
        "binary": {
            "type": "string"
        }
    }
}
```

The `binary` property describes where in the package namespace to find the
binary to run the component.

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

### runner

`runner` is an optional property that names another component (or a package
that contains one) to which execution is to be delegated. The target component
must expose the [`Runner`][runner] service.

If `runner` is present, [`program`](#program) is a freeform string-string JSON
object interpreted as args to pass to the runner.

If `runner` is absent, it is assumed that `program.binary` is an ELF binary or
shell script.

The `runner` property is a JSON string.

### sandbox

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

The `dev` array contains a list of well-known device directories that are
provided to the component. For example, if the string `class/input` appears in
the `dev` array, then `/dev/class/input` will appear in the namespaces of components
loaded from the package. To whitelist access to a `misc` device, such as
`/dev/misc/sysinfo`, add the string `misc` to the `dev` array. Unfortunately,
whitelisting access to individual `misc` devices is not possible currently.

The `system` array contains a list of well-known paths within the system package
that are provided to the component. For example, if the string `bin` appears
in the `system` array, then `/system/bin` will appear in the namespaces of
components loaded from the package.

The `pkgfs` array contains a list of well-known paths within the pkgfs tree
that are provided to the component. For example, if the string `packages`
appears in the `pkgfs` array, then `/pkgfs/packages` will appear in the
namespaces of components loaded from the package, providing access to all
packages fully cached on the system.

The `services` array defines a whitelist of services from `/svc` that the
component may access. A typical component will require a number services from
`/svc` in order to play some useful role in the system. For example, if
`"services" = [ "fuchsia.sys.Launcher", "fuchsia.netstack.Netstack" ]`, the
component will have the ability to launch other components and access network
services. A component may declare any list of services in its `services`
whitelist, but it will only be able to access services present in its
[environment](../glossary.md#environment). This property should be defined by
all new components, and soon a migration will take place to convert all
components to define `services`.

The `features` array contains a list of well-known features that the package
wishes to use. Including a feature in this list is a request for the environment
in which the contents of the package execute to be given the resources required
to use that feature.

The set of currently known features are as follows:

- `introspection`, which requests access to introspect the system. The
  introspection namespace will be located at `/info_experimental`.

- `persistent-storage`, which requests access to persistent storage for the
  device, located in `/data` in the package's namespace. (Future work will
  likely make this access more fine-grained than just the `/data` directory.)

- `root-ssl-certificates`, which requests access to the root SSL certificates
  for the device. These certificates are provided in the `/config/ssl` directory
  in the package's namespace.

- `shell`, which requests access to the resources appropriate for an interactive
  command line. Typically, shells are granted access to all the resources
  available in the current environment. Currently, when a package requests the
  `shell` feature, the package loads shared libraries from `/system/lib` instead
  of from its own package. This behavior will probably change over time, but we
  do this currently so that programs that are run from the shell can find their
  shared libraries. The `shell` feature also implies the `root-ssl-certificates`
  feature.

- `shell-commands`, which requests access to the currently available shell
  binaries (note: not "installed", but "available"). Binaries are mapped into
  `/bin` in the requesters namespace. Running these commands may require the
  `fuchsia.process.Resolver` and `fuchsia.process.Launcher` services also
  be requested.

- `system-temp`, which requests access to the system temp directory, located at
  `/tmp` in the package's namespace. (Future work will likely remove access to
  the system temp directory in favor of a local temp directory for each
  process.)

- `vulkan`, which requests access to the resources required to use the Vulkan
  graphics interface. This adds layer configuration data in the `/config/vulkan`
  directory in the package's namespace.

- `deprecated-all-services`, which grants the component access to all services
  in its environment, instead of the whitelist in `services`. This feature is
  deprecated and should not be used for new components.

See [sandboxing.md](sandboxing.md) for more information about sandboxing.


[runner]: https://fuchsia.googlesource.com/garnet/+/master/public/fidl/fuchsia.sys/runner.fidl
