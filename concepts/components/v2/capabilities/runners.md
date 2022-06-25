# Component Runners

<<../../_v2_banner.md>>

Component runners extend the component framework through an
[environment][glossary.environment] to provide a runtime for launching new
component instances.

Component manager launches components by sending a request containing
[`ComponentStartInfo`][fidl-runner] to the appropriate runner using the
[`fuchsia.component.runner.ComponentRunner`][fidl-runner] protocol.
The `ComponentStartInfo` contains details about the component's executable and
its [namespace][glossary.namespace]. The runner manages the component's
execution within the supported runtime.

After starting the component, component manager uses the
[`fuchsia.component.runner.ComponentController`][fidl-runner] protocol provided
in the [`Start`][fidl-runner] request to send execution actions to the runner,
such as stopping the component. The runner chooses how to interpret these
commands as appropriate to the component runtime.

## Providing runner capabilities {#provide}

To provide a runner capability, a component must declare a `runner`
capability, whose `path` designates a FIDL protocol implementing
[`fuchsia.component.runner.ComponentRunner`][fidl-runner] served from the
component's [outgoing directory][glossary.outgoing-directory].

```json5
{
    capabilities: [
        {
            runner: "web",
            path: "/svc/fuchsia.component.runner.ComponentRunner",
        },
    ],
}
```

Component manager sends `ComponentRunner/Start` requests to this protocol.
Each request includes a [`ComponentController`][fidl-controller] channel which
the runner should serve to handle lifecycle events for the component.

## Routing runner capabilities {#route}

Components route runner capabilities by [exposing](#expose) them to their
parent and [offering](#offer) them to their children.

For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].

### Exposing {#expose}

Exposing a runner capability gives the component's parent access to that
capability:

```json5
{
    expose: [
        {
            runner: "web",
            from: "self",
        },
    ],
}
```

You may optionally specify:

* [`as`](#renaming)

### Offering {#offer}

Offering a runner capability gives a child component access to that
capability:

```json5
{
    offer: [
        {
            runner: "web",
            from: "self",
            to: [ "#child-a" ],
        },
    ],
}
```

You may optionally specify:

* [`as`](#renaming)

## Registering a component runner {#register}

Component runners are made available to components through their
[environment][environment]. To register a new runner within an environment,
add a new entry to the `runners` section of the `environments` declaration:

```json5
environments: [
    {
        name: "my-environ",
        extends: "realm",
        runners: [
            {
                runner: "web",
                from: "parent",
            },
        ],
    },
]
```

You may optionally specify:

* [`as`](#renaming)

For more details on how to apply environments to components, see the
[environments documentation][environment].

## Selecting a runner

A component specifies the appropriate runner for execution using the `program`
section of its manifest. The `program` section designates the `runner` as well
as any runner-specific options. The runner must be [registered](#register) in
the component's environment.

For example, a component which runs as a web page might have a `program` like
the following:

```json5
program: {
    runner: "web",
    mode: "incognito",
},
```

When the component manager attempts to launch this component, it will send a
request to the provider of the `web` runner to start it.

## Renaming runners {#renaming}

You may `expose`, `offer`, or [register](#register) the runner capability under
a different name using the `as` parameter:

```json5
{
    expose: [
        {
            runner: "web",
            from: "#chromium",
            as: "web-chromium",
        },
    ],
}
```

## Framework runners {#framework}

Component framework provides the following built-in component runners:

-   [ELF runner][elf-runner]: Runs binaries compiled to the ELF file format.

```json5
{
    program: {
        runner: "elf",
        binary: "bin/example",
    },
}
```

[glossary.environment]: /docs/glossary/README.md#environment
[glossary.namespace]: /docs/glossary/README.md#namespace
[glossary.outgoing-directory]: /docs/glossary/README.md#outgoing-directory
[capability-routing]: /docs/concepts/components/v2/capabilities/README.md#routing
[elf-runner]: /docs/concepts/components/v2/elf_runner.md
[environment]: /docs/concepts/components/v2/environments.md
[fidl-directory]: /sdk/fidl/fuchsia.io/directory.fidl
[fidl-runner]: https://fuchsia.dev/reference/fidl/fuchsia.component.runner#ComponentRunner
[fidl-controller]: https://fuchsia.dev/reference/fidl/fuchsia.component.runner#ComponentController
