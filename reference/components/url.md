# Component URLs

A [component URL][glossary.component-url] is a URL that identifies a component.

This section describes the syntax used for displaying URLs to users.

## URL Format

A component URL can, in principle, have any [scheme][rfc-uri-scheme]. Some
common schemes you may encounter are:

-   [fuchsia-pkg](#fuchsia-pkg)
-   [fuchsia-boot](#fuchsia-boot)
-   [http(s)](#http)

### fuchsia-pkg {#fuchsia-pkg}

A `fuchsia-pkg` component URL is a [package URL][doc-package-url] that locates a
component distributed in a [Fuchsia package][doc-package].

It has the same format as [package URL][doc-package-url], with a
[resource path][doc-package-url-resource-path] relative to the package root that
locates a [component manifest][doc-manifests]. This path is usually of the form
`meta/<manifest_name>.cm`.

```none {:.devsite-disable-click-to-copy}
fuchsia-pkg://<repo-hostname>[/<pkg-name>[/<pkg-variant>][?hash=<pkg-hash>][#<path-to-manifest>]]
```

Example:

```none {:.devsite-disable-click-to-copy}
fuchsia-pkg://fuchsia.com/stash#meta/stash_secure.cm
```

### fuchsia-boot {#fuchsia-boot}

A `fuchsia-boot` component URL locates a component in the system boot image.
This scheme is used to identify components that must be resolved during early
boot before a [`fuchsia-pkg`](#fuchsia-pkg) [resolver][doc-resolvers] is
available. It has the following format:

```none {:.devsite-disable-click-to-copy}
fuchsia-boot:///<path-to-manifest-in-bootfs>
```

Example:

```none {:.devsite-disable-click-to-copy}
fuchsia-boot:///#meta/driver_manager.cm
```

### http(s) {#http}

An `http(s)` component URL identifies a web page as a component. Such a
component could be executed as a web page in a web [runner][doc-runners], for
example.

Example:

```none {:.devsite-disable-click-to-copy}
https://en.wikipedia.org/wiki/Hippos
```

## Relative URLs {#relative}

The Component Framework supports a subset of relative URLs (Relative URLs are
defined in [URL RFC 3986][url-rfc-3986]). Specifically, Component Framework
supports relative path URLs (plus a *URL fragment* with the path to the
component manifest) to subpackaged components, and fragment-only URLs.

The path to the component manifest is the only content allowed in a relative
component URL fragment (`#meta/<component>.cm`). If the fragment contains any
other content, or if a relative component includes URL query parameters
(`?key=value`), the component will not resolve.

### Relative path URLs to subpackaged components

A relative path URL is resolved at runtime based on a known "context". For
components resolving a child component by relative URL, the context is a
resolver-supplied value associated with the parent component. The URL `path` is
interpreted as the name of one of the declared
[subpackage][glossary.subpackage]s of the parent component's package.

Relative subpackage path URLs start with a
[relative package URL][doc-relative-package-url] and have the following format:

```none {:.devsite-disable-click-to-copy}
<subpackage-path>#<path-to-manifest>
```

Example:

```none {:.devsite-disable-click-to-copy}
child#meta/default.cm
```

For more information on subpackages and subpackaged components, see
the documentation on [Fuchsia Subpackages][doc-subpackages].

### Relative fragment-only URLs {#relative-fragment-only}

A relative fragment-only URL is resolved at runtime based on the URL of the
parent component's [package][doc-package]. Fragment-only URLs have the following
format:

```none {:.devsite-disable-click-to-copy}
#<path-to-manifest>
```

Example:

```none {:.devsite-disable-click-to-copy}
#meta/child.cm
```

For a `fuchsia-pkg` parent component with the following URL:

```none {:.devsite-disable-click-to-copy}
fuchsia-pkg://fuchsia.com/package#meta/component.cm
```

The relative URL resolves to:

```none {:.devsite-disable-click-to-copy}
fuchsia-pkg://fuchsia.com/package#meta/child.cm
```

[glossary.component-url]: /glossary/README.md#component-url
[glossary.subpackage]: /glossary/README.md#subpackage
[doc-manifests]: /concepts/components/v2/component_manifests.md
[doc-package]: /concepts/packages/package.md
[doc-package-url]: /concepts/packages/package_url.md
[doc-package-url-resource-path]: /concepts/packages/package_url.md#resource-paths
[doc-relative-package-url]: /concepts/packages/package.md#relative-package-urls
[doc-resolvers]: /concepts/components/v2/capabilities/resolvers.md
[doc-runners]: /concepts/components/v2/capabilities/runners.md
[doc-subpackages]: /concepts/components/v2/subpackaging.md
[rfc-uri-scheme]: https://tools.ietf.org/html/rfc3986#section-3.1
[url-rfc-3986]: https://datatracker.ietf.org/doc/html/rfc3986#section-4.2