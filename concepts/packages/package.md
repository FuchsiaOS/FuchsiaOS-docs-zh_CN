<!-- # Fuchsia packages

A Fuchsia package is a namespace hierarchy, which distributes and organizes files,
and then provides one or more programs, components or services for a Fuchsia system.
A Fuchsia package is a term representing a unit of distribution, though unlike many
other package systems, that unit is composed of parts and is not a single binary `BLOB`. -->

# Fuchsia 包

Fuchsia 包是具有层次结构的命名空间，它负责组织与分发文件，并为 Fuchsia 系统提供一个或多个程序、组件或者服务。Fuchsia 包是一个代表分发单元的术语，尽管与许多其他包系统不同，但是该单元是由许多部分组成的，而不是单个二进制「BLOB」。

<!-- Note: For more information on components, see
[Introduction to the Fuchsia component framework](/docs/concepts/components/v2/README.md). -->

注意：关于组件的更多信息，请查阅 [Fuchsia 组件框架的简介](/docs/concepts/components/v2/README.md)。

<!-- Packages are downloaded from the Fuchsia package server in `BLOB`s. The Fuchsia
package server is a HTTP(s) server. These `BLOB`s are uniquely defined by a Merkle
root. A `BLOB` is named after its content, using the
[Fuchsia Merkle Root](merkleroot.md) algorithm. If two `BLOB`s have the same content,
they have the same name. As a result, each `BLOB` has a unique identifier and is
written to persistent storage using this Merkle
root as a key. This process is done to eliminate duplication of `BLOB`s that
may exist between packages. For example, a shared library which exists in
multiple packages is only stored once on the device. -->

包以 `BLOB` 的形式从 Fuchsia 包服务器上被下载下来。包服务器是 HTTP（s） 服务器。这些 `BLOB` 使用墨克根作为唯一标识。`BLOB` 先被定义内容，然后才根据 [Fuchsia 墨克根](merkleroot.md) 算法为其命名。如果两个 `BLOB` 具有相同的内容，那么它们的名字也必定相同。因此，每一个 `BLOB` 都有它唯一的标识，并以其墨克根作为键被写入持久化存储中。这样做是为了消除不同包之间 `BLOB` 的重复，例如，一个被多个包使用的共享库只会在设备中保存一份。

<!-- The package server can serve as a root of trust and validates the authenticity
of each package. -->

包服务器可以作为信任链的起点并能验证每个包的真实性。

<!-- Note: To understand how components and packages work together in Fuchsia,
see [Fuchsia's software model](/docs/concepts/software_model.md). -->

注意：要理解 Fuchsia 中组件与包的工作方式，请查阅 [Fuchsia 软件模型](/docs/concepts/software_model.md)。

## Types of packages

The packages that comprise the Fuchsia operating system are categorized into
three groups, which affect how they are managed:

## 包的种类

组成 Fuchsia 操作系统的软件包被分为三组，分组影响着它们是如何被管理的：

<!-- Note: To understand how these packages work in a Fuchsia build, see
[Dependency sets](/docs/concepts/build_system/boards_and_products.md#dependency_sets) -->

注意：要理解这些包是如何在 Fuchsia 构建中发挥作用的，请查阅 [依赖集合](/docs/concepts/build_system/boards_and_products.md#dependency_sets)

<!-- * [Base packages](#base-packages)
* [Cached packages](#cached-packages)
* [Universe packages](#universe-packages) -->

* [基本包](#base-packages)
* [缓存包](#cached-packages)
* [太空包](#universe-packages)

<!-- ### Base packages {#base-packages}

Note: Base packages are part of the system assembly process.
There is no way to determine if a package is a base package. -->

### 基本包 {#base-packages}

注意：基本包是系统组建过程中的一部分。但是无法确定一个包是否是基本包。

<!-- These are the packages that are part of the foundation of the Fuchsia
operating system and are considered critical for security and the system.
Resolving a package which is in base on a running Fuchsia system always
returns the version that is on the device, and not a new version which
may exist on a package server. However, base packages can be updated as part of
the [OTA process](/docs/concepts/packages/ota.md). -->

这些包是 Fuchsia 操作系统的基础部分，对系统和系统安全起重要作用。解析正在运行中的系统的基本包总是返回目前设备上的版本而不是包服务器上的较新（可能）版本。但是，基本包仍能通过 [OTA 过程](/docs/concepts/packages/ota.md) 得到更新。

<!-- Since these packages are immutable for the runtime of a
system, these packages must be updated with
[`fx ota`](https://fuchsia.dev/reference/tools/fx/cmd/ota) which triggers an
over-the-air (OTA) update. -->

由于这些包在系统运行期间无法修改，这些包必须通过 [`fx ota`](https://fuchsia.dev/reference/tools/fx/cmd/ota) 命令来开启 over-the-air （OTA） 更新。

### Cached packages {#cached-packages}

<!-- These are packages on the device which are not part of base. These
packages exist when the device is flashed or paved, so these packages
are usable if the device boots without a network connection. Cached packages
are updated during the resolution process if a different package is available
on the package server. These packages are not updated during a system update,
but are updated ephermerally. -->

### 缓存包 {#cached-packages}

缓存包是设备中那些不是基本包的包。这些包存在于在设备被刷写或铺设时，因此这些包在设备断网启动时非常起用。如果包服务器上存在与之不同的包，缓存包会在解析过程中被更细。这些包不会在系统更新时更新，而是临时更新。

<!-- Fuchsia can also evict cached packages on a running system to free up
resources based on runtime resource demands. -->

Fuchsia 也会在系统运行中根据运行时的资源需求删除缓存包来释放资源。

<!-- ### Universe packages {#universe-packages}

These are packages that exist on the package server, but not on the device. -->

### 太空包 {#universe-packages}

太空包是指存在于包服务器上的包，而不是存在于设备上的包。

<!-- ## Structure of a package

In most cases, a package in Fuchsia is a collection of `BLOB`s, which at a
minimum contains one content-addressed `BLOB` named [`meta.far`](#meta-far). -->

## 包的结构

在许多场合中，Fuchsia 中的一个包是众多 `BLOB` 的一个集合，至少包含一个名为 [`meta.far`](#meta-far) 内容寻址的 `BLOB`。

<!-- Note: For more information on the Fuchsia archive format (FAR), see
[Fuchsia archive format (FAR)](/docs/concepts/source_code/archive_format.md). -->

注意：请查阅 [Fuchsia 归档格式（FAR）](/docs/concepts/source_code/archive_format.md) 了解更多关于 Fuchsia 归档格式的信息。

<!-- In Fuchsia, you build a package with the `pm` tool, which exists in the
`//tools/` directory of the Fuchsia GN SDK. -->

在 Fuchsia 中，可通过 Fuchsia GN SDK 的 `//tools/` 目录下的 `pm` 工具构建一个包。

Essentially, a package is a tree of zero or more content-addressed items.
A package contains the following:

* [`meta.far`](#meta-far)
* [`BLOB`s outside of `meta/`](#outside-blobs)

从本质上讲，包是一棵包含零个或多个内容寻址项的树。  
一个包含有如下内容：

* [`meta.far`](#meta-far)
* [`BLOB`s outside of `meta/`](#outside-blobs)

<!-- ### `meta.far` {#meta-far}

Note: For more information on the Fuchsia archive format (FAR), see
[Fuchsia archive format (FAR)](/docs/concepts/source_code/archive_format.md). -->

### `meta.far` {#meta-far}

注意：请查阅 [Fuchsia 归档格式（FAR）](/docs/concepts/source_code/archive_format.md) 了解更多关于 Fuchsia 归档格式的信息。

<!-- The package metadata archive, `meta.far`, contains metadata about
a package, presented as the `meta/` directory. `meta.far` has a
[merkleroot](merkleroot.md) which in practical terms is also known as the
merkleroot of a package. -->

包的元数据归档，`meta.far`，包含着包的元数据，存在于 `meta/` 目录下。在实践中，`meta.far` 的 [墨克根](merkleroot.md) 也被认为就是包的墨克根。

<!-- The `meta/` directory of a package contains at minimum two files:

* `meta/package`

   The package identity file. This is a JSON file that contains the name and
   version of the package.

* `meta/contents`

   The contents file. This file is created by the `pm update` tool, which is executed
   with the `pm build` tool. This file maps the user-facing file names of a
   package to the Merkle root of those files. -->

包的 `meta/` 目录下至少包含以下两个文件：

* `meta/package`

   包的标识文件。这是一个含有包名和包版本号的 JSON 文件。

* `meta/contents`

   内容文件。这个文件是 `pm update` 调用 `pm build` 工具创建的。这个文件将包面向用户的文件名映射为这些文件的 Merkle 根。

<!-- Additionally, the `meta/` directory can contain files such as a component manifest.
For more information on component manifests, see
[Component manifests](/docs/concepts/components/v2/component_manifests.md). -->

此外，`meta/` 目录可以包含组件清单（manifest）等文件。关于组件清单的更多信息，请查阅 [组件清单](/docs/concepts/components/v2/component_manifests.md)。

<!-- ### `BLOB`s outside of `meta/` {#outside-blobs}

Most files of a package exist outside of the `meta/`directory and each are a `BLOB`.

For example, these files can be like the following:

* `bin/foo`
* `lib/libfdio.so`
* `data/mydata.db` -->

### `meta/` 外部的 `BLOB` {#outside-blobs}

一个包的大多数文件都存在于 `meta/` 目录外部，并且每个文件也都是 `BLOB`。

例如，这些文件可能是如下形式：

* `bin/foo`
* `lib/libfdio.so`
* `data/mydata.db`

<!-- ## Identification of a package

Every package in Fuchsia is identified by a `package-url`.

Note: For more information about [Fuchsia package URLs](/docs/concepts/packages/package_url.md). -->

## 包的标识

Fuchsia 中的每个包都使用 `package-url` 标识。  
注意：点击查看更多关于 [Fuchsia 包 URLs](/docs/concepts/packages/package_url.md) 的信息。

<!-- A Fuchsia package URL looks like the following: -->

包的 URL 的形式如下所示：

```
fuchsia-pkg://{{ '<var>' }}repository{{ '</var>' }}/{{ '<var>' }}package-name{{ '</var>' }}?hash={{ '<var>' }}package-hash{{ '</var>' }}#{{ '<var>' }}resource-path{{ '</var>' }}
```

<!-- Fuchsia has different intereprations of `fuchsia-pkg` URL depending on which parts of the URL are
present.

 * If the repository, package, and resource parts are present, then the URL
   identifies the indicated resource within the package.
 * If only the repository and package parts are present, then the URL identifies
   the indicated package itself.
 * If only the repository parts are present, then the URL identifies the
   indicated repository itself. -->

取决于 URL 中出现了哪些字段，Fuchsia 对 `fuchsia-pkg` 的 URL 有不同的解释。

* 如果出现了 repository、package 和 resource，则该 URL 标识包中的指定资源。
* 如果只出现 repository 和 package，则该 URL 标识被指定的包本身。
* 如果只出现 repository，则该 URL 标识被指定的 repository 本身。

<!-- The package parts can express varying degrees of specificity. At minimum the
package name must be present, optionally followed by the package hash.

If the package hash is missing, the package resolver fetches the resources
from the newest revision of the package variant available to the client. -->

URL 各字段可以表达不同程度的特异性。但必须至少提供包名，其后是否跟上包的哈希值则是可选的。  

如果包的哈希值缺失，包解析器将从客户机可用的包变体的最新版本中获取该资源。
