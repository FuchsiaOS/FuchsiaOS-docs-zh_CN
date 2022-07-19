# Component resolvers

<<../../_v2_banner.md>>

Component resolvers extend the component framework through an
[environment][glossary.environment] to resolve a
[component URL][glossary.component-url] into a component.

Component manager resolves component URLs by finding a resolver that supports a
matching URL scheme in the relevant environment and sending a request using the
[`fuchsia.component.resolution.Resolver`][fidl-resolver] protocol.

If resolution succeeds, the component resolver returns a
[`ComponentDecl`][fidl-decl], the FIDL representation of a
[component manifest][component-manifest]. If the component being resolved has
an associated package, the component resolver also returns a
[`fuchsia.io.Directory`][fidl-directory] handle for the package directory.

## Providing resolver capabilities {#provide}

To provide a resolver capability, a component must declare a `resolver`
capability, whose `path` designates a FIDL protocol implementing
[`fuchsia.component.resolution.Resolver`][fidl-resolver] served from the component's
[outgoing directory][glossary.outgoing-directory].

```json5
{
    capabilities: [
        {
            resolver: "my_resolver",
            path: "/svc/fuchsia.component.resolution.Resolver",
        },
    ],
}
```

Component manager submits requests to resolve a component URL to this protocol.

## Routing resolver capabilities {#route}

Components route resolver capabilities by [exposing](#expose) them to their
parent and [offering](#offer) them to their children.

For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].

### Exposing {#expose}

Exposing a resolver capability gives the component's parent access to that
capability:

```json5
{
    expose: [
        {
            resolver: "my_resolver",
            from: "self",
        },
    ],
}
```

### Offering {#offer}

Offering a resolver capability gives a child component access to that
capability:

```json5
{
    offer: [
        {
            resolver: "my_resolver",
            from: "self",
            to: [ "#child-a" ],
        },
    ],
}
```

## Registering a component resolver {#register}

Component resolvers are made available to components through their
[environment][environment]. To register a new resolver within an environment,
add a new entry to the `resolvers` section of the `environments` declaration:

```json5
environments: [
    {
        name: "my-environ",
        extends: "realm",
        resolvers: [
            {
                resolver: "my_resolver",
                scheme: "my-scheme",
                from: "parent",
            }
        ],
    },
]
```

The registered resolver will be used to resolve component URLs whose URL scheme
matches the provided `scheme`.

For more details on how to apply environments to components, see the
[environments documentation][environment].

## Framework resolvers {#framework}

Component framework provides the following built-in component resolvers to
support standard Fuchsia URL schemes:

| Resolver            | URL scheme                 |
| ------------------- | -------------------------- |
| `boot_resolver`     | [`fuchsia-boot`][url-boot] |
| `base_resolver`     | [`fuchsia-pkg`][url-pkg]   |
| `universe_resolver` | [`fuchsia-pkg`][url-pkg]   |

[glossary.component-url]: /glossary/README.md#component-url
[glossary.environment]: /glossary/README.md#environment
[glossary.outgoing-directory]: /glossary/README.md#outgoing-directory
[capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[component-manifest]: /concepts/components/v2/component_manifests.md
[environment]: /concepts/components/v2/environments.md
[fidl-resolver]: /sdk/fidl/fuchsia.component.resolution/resolver.fidl
[fidl-decl]: /sdk/fidl/fuchsia.component.decl/component.fidl
[fidl-directory]: /sdk/fidl/fuchsia.io/directory.fidl
[url-boot]: /reference/components/url.md#fuchsia-boot
[url-pkg]: /reference/components/url.md#fuchsia-pkg
