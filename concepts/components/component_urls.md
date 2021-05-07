# Component URLs {#component-urls}

A component URL is a [URL][wiki-url] that locates a component, including its
declaration, program, and assets. In [Components v2][glossary-components-v2],
component URLs are resolved to
[component declarations][doc-manifests-declaration] by a
[resolver][doc-resolvers].

## Usage

The primary use of component URLs is to identify a component in the definition
of a component instance, as part of a
[child declaration][doc-manifests-children].

You should not use component URLs to identify component *instances*, as multiple
component instances can share the same URL. For that purpose, instead use
[monikers][doc-monikers].

## Format

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

```
fuchsia-pkg://<repo-hostname>[/<pkg-name>[/<pkg-variant>][?hash=<pkg-hash>][#<path-to-manifest>]]
```

Example:

```
fuchsia-pkg://fuchsia.com/stash#meta/stash_secure.cm
```

### fuchsia-boot {#fuchsia-boot}

A `fuchsia-boot` component URL locates a component in the system boot image.
This scheme is used to identify components that must be resolved during early
boot before a [`fuchsia-pkg`](#fuchsia-pkg) [resolver][doc-resolvers] is
available. It has the following format:

```
fuchsia-boot:///<path-to-manifest-in-bootfs>
```

Example:

```
fuchsia-boot:///#meta/driver_manager.cm
```

### http(s) {#http}

An `http(s)` component URL identifies a web page as a component. Such a
component could be executed as a web page in a web [runner][doc-runners], for
example.

Example:

```
https://en.wikipedia.org/wiki/Hippos
```

[doc-manifests]: v2/component_manifests.md
[doc-manifests-children]: v2/component_manifests.md#children
[doc-manifests-declaration]: v2/component_manifests.md#component-declaration
[doc-monikers]: v2/monikers.md
[doc-resolvers]: v2/capabilities/resolvers.md
[doc-package]: /docs/concepts/packages/package.md
[doc-package-url]: /docs/concepts/packages/package_url.md
[doc-package-url-resource-path]: /docs/concepts/packages/package_url.md#resource-paths
[glossary-components-v2]: /docs/glossary.md#components-v2
[doc-runners]: v2/capabilities/runners.md
[wiki-url]: https://en.wikipedia.org/wiki/URL
[rfc-uri-scheme]: https://tools.ietf.org/html/rfc3986#section-3.1
