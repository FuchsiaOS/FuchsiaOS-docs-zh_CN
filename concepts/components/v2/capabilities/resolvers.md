# Component resolvers (Components v2)

<<../../_v2_banner.md>>

A component resolver is a protocol-backed [capability][glossary-capability] responsible for
resolving a URL to a component manifest.

Component resolver capabilities are registered with an [`environment`][environment] for a
particular URL scheme (http, fuchsia-pkg, etc) and are used by the component manager on behalf of a
component to resolve its children.

## Registering a component resolver {#registering}

Component resolvers are registered with [`environments`][environment] to resolve a particular URL
scheme. If the environment extends from a parent environment, and the same scheme is registered in
both parent and child environments, the child registration takes precedence.

Note: For more information on the environments section of the component manifest, see
[environments](/docs/concepts/components/v2/component_manifests.md#environments).

```json5
environments: [
    {
        name: "my-environ",
        extends: "realm",
        resolvers: [
            {
                resolver: "my-resolver",
                scheme: "my-scheme",
                from: "parent",
            }
        ],
    }
```

An environment must be assigned to a child in order for the registered resolver to take effect.
That child’s URL, and any descendents that do not override their environment
(see [`environment`][environment]), will be resolved with the registered resolver, if the URL
scheme matches.

Note: For more information on the children section of the component manifest, see
[children](/docs/concepts/components/v2/component_manifests.md#children).

```json5
children: [
    {
        name: "my-child",
        url: "my-scheme://myhost.com/my-path",
        environment: "#my-environ"
    },
]
```

## Implementing a component resolver

A component resolver can be implemented by

- Implementing the [`fuchsia.sys2.ComponentResolver`] FIDL protocol.
- Declaring and [routing] a `resolver` capability backed by this protocol.

When the component manager is asked to resolve a component URL, it finds the component resolver
registered to the URL’s scheme and asks it to resolve the URL over the
[`fuchsia.sys2.ComponentResolver`] FIDL protocol.

If resolution succeeds, the component resolver must return a [`ComponentDecl`], the FIDL
representation of a [component manifest][component-manifest]. If the component being resolved has
an associated package, the component resolver should also return a [`fuchsia.io.Directory`] handle
that points to the package directory.

Before registering a resolver with an environment, it must be created and routed to the
environment.

Note: For more information on the capabilities section of the component manifest, see
[capabilities](/docs/concepts/components/v2/component_manifests.md#capabilities).

```json5
capabilities: [
    {
        resolver: "my-resolver",
        path: "/svc/fuchsia.sys2.ComponentResolver",
    },
],
expose: [
    {
        resolver: "my-resolver",
        from: "self",
    }
]
```

`resolver` capabilities are different from [`protocol`] capabilities in that they cannot be used
directly by a component. They can only be registered with an environment.
See [registering a component resolver](#registering).

## Built-in boot resolver

The component manager provides a built-in component resolver called `boot-resolver`, which is
registered to the `fuchsia-boot` scheme in component manager's built-in environment.

This resolver can be routed, and the built-in environment can be extended.
See [`environments`][environment].

[environment]: ../environments.md
[`fuchsia.sys2.ComponentResolver`]: /sdk/fidl/fuchsia.sys2/runtime/component_resolver.fidl
[`ComponentDecl`]: /sdk/fidl/fuchsia.sys2/decls/component_decl.fidl
[component-manifest]: ../component_manifests.md
[`fuchsia.io.Directory`]: /sdk/fidl/fuchsia.io/io.fidl
[`protocol`]: protocol.md
[glossary-capability]: /docs/glossary.md#capability
[routing]: ../component_manifests.md#capability-routing
