<!--
# Component URLs {#component-urls}

A component URL is a [URL][wiki-url] that locates a component, including its
declaration, program, and assets. In [Components v2][glossary-components-v2],
component URLs are resolved to
[component declarations][doc-manifests-declaration] by a
[resolver][doc-resolvers].
-->

# 组件 URL {#component-urls}
组件 URL 是定位组件的 [URL][wiki-url]，包括它的 声明、程序和资产。
在 [Components v2][glossary-components-v2] 中，
组件 URL 被一个[解析器][doc-resolvers]解析为[组件声明][doc-manifests-declaration] 。

<!--
## Usage

The primary use of component URLs is to identify a component in the definition
of a component instance, as part of a
[child declaration][doc-manifests-children].

You should not use component URLs to identify component *instances*, as multiple
component instances can share the same URL. For that purpose, instead use
[monikers][doc-monikers].
-->
## 用法

组件 URL 的主要用途是标识组件实例定义中的组件部分，作为[子声明][doc-manifests-children]的一部分。

您不应使用组件 URL 来标识组件 *实例*，因为多个 组件实例可以共享相同的 URL。为此，请改用[绰号][doc-monikers]。

<!--
## Format

A component URL can, in principle, have any [scheme][rfc-uri-scheme]. Some
common schemes you may encounter are:

-   [fuchsia-pkg](#fuchsia-pkg)
-   [fuchsia-boot](#fuchsia-boot)
-   [http(s)](#http)

-->

## 形式
原则上，组件 URL 可以具有任何 [方案][rfc-uri-scheme]。 您可能会遇到的一些常见方案是：

-   [fuchsia-pkg](#fuchsia-pkg)
-   [fuchsia-boot](#fuchsia-boot)
-   [http(s)](#http)

<!--
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
-->

### fuchsia-pkg {#fuchsia-pkg}
`fuchsia-pkg` 组件 URL 是一个 [package URL][doc-package-url]，它是在 [Fuchsia package][doc-package] 中分发的组件。

它的格式与 [packaeg URL][doc-package-url] 相同，
具有相对于定位 [组件清单] [doc-manifests]的包根目录的[资源路径][doc-package-url-resource-path] 。
此路径通常采用以下形式
`meta/<manifest_name>.cm`。
```
fuchsia-pkg://<repo-hostname>[/<pkg-name>[/<pkg-variant>][?hash=<pkg-hash>][#<path-to-manifest>]]
```

示例:

```
fuchsia-pkg://fuchsia.com/stash#meta/stash_secure.cm
```

<!--
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
-->

### fuchsia-boot {#fuchsia-boot}
`fuchsia-boot` 组件 URL 在系统引导映像中定位一个组件。 此方案用于识别在[`fuchsia-pkg`](#fuchsia-pkg) [resolver][doc-resolvers] 可用之前必须在早期引导期间解析的组件。 它具有以下格式：
```
fuchsia-boot:///<path-to-manifest-in-bootfs>
```

示例:

```
fuchsia-boot:///#meta/driver_manager.cm
```
<!--
### http(s) {#http}

An `http(s)` component URL identifies a web page as a component. Such a
component could be executed as a web page in a web [runner][doc-runners], for
example.

Example:

```
https://en.wikipedia.org/wiki/Hippos
```
-->

### http(s) {#http}

http(s) 组件 URL 将网页标识为组件。 例如，这样的组件可以在 web  [runner][doc-runners] 中作为网页执行。
示例:

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
