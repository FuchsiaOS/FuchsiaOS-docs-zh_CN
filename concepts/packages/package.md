<!--
# Fuchsia packages
 -->
# Fuchsia 包

<!--
A Fuchsia package is a hierarchical collection of files that provides one or more programs,
components or services to a Fuchsia system.  A Fuchsia package is a term representing a
unit of distribution, though unlike many other package systems, that unit is composed of
parts and is not a single binary `BLOB`.
 -->
Fuchsia 包（package）是文件的分层集合，为 Fuchsia 系统提供一个或多个程序、组件或服务。Fuchsia 包是一个表示分发单元的术语，与许多其他包系统不同，单元由多个部分组成，而不是单个二进制 `BLOB`。

<!--
Note: For more information on components, see
[Introduction to the Fuchsia component framework](/concepts/components/v2/introduction.md).
 -->
注：要获取关于组件的更多信息，请参阅 [Fuchsia 组件框架介绍](/concepts/components/v2/introduction.md)。

<!--
Some packages are present on a Fuchsia system at startup, and
additional packages can be downloaded from a Fuchsia package server in `BLOB`s.
The Fuchsia package server is an HTTP(S) server. These `BLOB`s are uniquely defined by a Merkle
root. A `BLOB` is named after its content, using the
[Fuchsia Merkle Root](merkleroot.md) algorithm. If two `BLOB`s have the same content,
they have the same name. As a result, each `BLOB` has a unique identifier and is
written to persistent storage using this Merkle
root as a key. This process is done to eliminate duplication of `BLOB`s that
may exist between packages. For example, a shared library which exists in
multiple packages is only stored once on the device.
 -->
一些包在启动时出现在 Fuchsia 系统上，其他包可以从 `BLOB` 中的 Fuchsia 包服务器下载。Fuchsia 包服务器是一个 HTTP(S) 服务器。这些 `BLOB` 由 Merkle 根唯一定义。`BLOB` 根据 [Fuchsia Merkle 根](merkleroot.md)算法按内容命名。如果两个 `BLOB` 具有相同的内容，则它们拥有相同的名称。因此，每个 `BLOB` 都有唯一的标识符，并使用此 Merkle 根作为密钥写入持久存储。执行此过程是为了消除包之间可能存在的 `BLOB` 重复。例如，存在于多个包中的共享库仅在设备上存储一次。

<!--
The package server serves as a root of trust as it validates the authenticity of
each package.
 -->
包服务器因其验证每个包的真实性而作为信任根。

<!--
Packages can also declare dependencies on named
[subpackage][glossary.subpackage]s, creating a hierarchy of nested packages.
Build rules link a package with the build target of each subpackage. At build
time, the package build tool records the subpackages in the parent package's
metadata, mapping each subpackage name to its package hash (the `BLOB` id that
identifies the subpackage). This ensures the list of subpackages and the
internals of each subpackage cannot change without also changing the Merkle
(package hash) of the parent.
 -->
包还可以声明对具名[子包][glossary.subpackage]（subpackage）的依赖，创建嵌套包的层次结构。构建规则将包与每个子包的构建目标链接起来。在构建时，包构建工具在父包的元数据中记录子包，将每个子包名称映射到其包哈希（标识子包的 `BLOB` ID）。这就确保了子包列表和每个子包的内部结构不会在未更改父包 Merkle（包哈希）的情况下发生更改。

<!--
_Subpackages enable:_
 -->
**子包带来了：**

<!--
* Encapsulated dependencies (packages are inherently "package trees")
* Isolated `/pkg` directories (grouped components don't need to merge their
  files, libraries, and metadata into a single shared namespace)
* Assured dependency resolution (system and build tools ensure subpackages
  always "travel with" their packages)
 -->
* 封装的依赖关系（包本质上是“包树”）
* 隔离的 `/pkg` 目录（分组的组件不需要将其文件、库和元数据合并到单个共享命名空间中）
* 有保证的依赖解析（系统和构建工具确保子包始终“随同”它们的包）

<!--
For more information on packaging components with their dependencies using
Subpackages, see [Subpackaging components].
 -->
要获取关于使用子包打包组件及其依赖项的更多信息，请参阅[子包化组件][Subpackaging components]。

<!--
Note: To understand how components and packages work together in Fuchsia,
see [Fuchsia's software model](/concepts/software_model.md).
 -->
注意：要了解组件和包在 Fuchsia 中的协同工作方式，请参阅 [Fuchsia 的软件模型](/concepts/software_model.md)。

<!--
## Types of packages
 -->
## 包的类型

<!--
The packages that comprise the Fuchsia operating system are categorized into
three groups, which affect how they are managed:
 -->
组成 Fuchsia 操作系统的软件包分为三种，且影响其管理方式：

<!--
Note: To understand how these packages work in a Fuchsia build, see
[Dependency sets](/development/build/build_system/boards_and_products.md#dependency_sets)
 -->
注意：要了解这些包在 Fuchsia 构建中的工作方式，请参阅[依赖集](/development/build/build_system/boards_and_products.md#dependency_sets)

<!--
* [Base packages](#base-packages)
* [Cached packages](#cached-packages)
* [Universe packages](#universe-packages)
 -->
* [基础包](#base-packages)（base package）
* [缓存包](#cached-packages)（cached package）
* [宇宙包](#universe-packages)（universe package）

<!--
### Base packages {#base-packages}
 -->
### 基础包 {#base-packages}

<!--
Note: Base packages are part of the system assembly process.
There is no way to determine if a package is a base package.
 -->
注意：基础包是系统组装过程的一部分。无法确定一个包是否为基础包。

<!--
These are the packages that are part of the foundation of the Fuchsia
operating system and are considered critical for security and the system.
Resolving a package which is in base on a running Fuchsia system always
returns the version that is on the device, and not a new version which
may exist on a package server. However, base packages can be updated as part of
the [OTA process](/concepts/packages/ota.md).
 -->
这些软件包是 Fuchsia 操作系统基础的一部分，被认为对安全性和系统至关重要。在正在运行的 Fuchsia 系统中解析基础包总是返回设备所使用的版本，而不是包服务器上可能存在的新版本。但是，基础包可以作为 [OTA 流程](/concepts/packages/ota.md) 的一部分进行更新。

<!--
Since these packages are immutable for the runtime of a
system, these packages must be updated with
[`fx ota`](https://fuchsia.dev/reference/tools/fx/cmd/ota) which triggers an
over-the-air (OTA) update.
 -->
由于这些包在系统运行时是不可变的，因此必须使用 [`fx ota`](https://fuchsia.dev/reference/tools/fx/cmd/ota) 更新这些包，这会触发空中（over-the-air，OTA）更新。

<!--
### Cached packages {#cached-packages}
 -->
### 缓存包 {#cached-packages}

<!--
These are packages on the device which are not part of base. These
packages exist when the device is flashed or paved, so these packages
are usable if the device boots without a network connection. Cached packages
are updated during the resolution process if a different package is available
on the package server. These packages are not updated during a system update,
but are updated ephermerally.
 -->
这些是设备上不属于基础包的包。这些包在设备刷入或铺设时存在，因此这些包在设备无网络连接引导时可用。如果包服务器上有不同的包可用，缓存包将在解析过程中更新。这些软件包不会在系统更新期间进行更新，而是临时更新。

<!--
Fuchsia can also evict cached packages on a running system to free up
resources based on runtime resource demands.
 -->
Fuchsia 还可以回收正在运行的系统上的缓存包，以根据运行时资源需求释放资源。

<!--
### Universe packages {#universe-packages}
 -->
### 宇宙包 {#universe-packages}

<!--
These are packages that exist on the package server, but not on the device.
 -->
这些包存在于包服务器上，但非设备上。

<!--
## Structure of a package {#structure-of-a-package}
 -->
## 包的结构 {#structure-of-a-package}

<!--
In most cases, a package in Fuchsia is a collection of `BLOB`s, which at a
minimum contains one content-addressed `BLOB` named [`meta.far`](#meta-far).
 -->
大多数情况下，Fuchsia 中的包是 `BLOB` 的集合，其中至少包含一个名为 [`meta.far`](#meta-far) 的内容寻址 `BLOB`。

<!--
Note: For more information on the Fuchsia archive format (FAR), see
[Fuchsia archive format (FAR)](/development/source_code/archive_format.md).
 -->
注意：要获取关于 Fuchsia 归档格式（Fuchsia archive format，FAR）的更多信息，请参阅 [Fuchsia 归档格式（FAR）](/development/source_code/archive_format.md)。

<!--
In Fuchsia, you build a package with the `ffx package build` command or the
legacy `pm` tool, which both exist in the `//tools/` directory of the
Fuchsia IDK.
 -->
在 Fuchsia 中，您可以使用 `ffx package build` 命令或旧版 `pm` 工具构建包，两种工具都在 Fuchsia IDK 的 `//tools/` 目录中。

<!--
Essentially, a package is a tree of zero or more content-addressed items.
A package contains the following:
 -->
本质上，包是零个及以上内容寻址项的树。一个包包含以下内容：

<!--
* [`meta.far`](#meta-far)
* [`BLOB`s outside of `meta/`](#outside-blobs)
 -->
* [`meta.far`](#meta-far)
* [`meta/` 外 `BLOB`](#outside-blobs)

### `meta.far` {#meta-far}

<!--
Note: For more information on the Fuchsia archive format (FAR), see
[Fuchsia archive format (FAR)](/development/source_code/archive_format.md).
 -->
注意：要获取关于 Fuchsia 归档格式（FAR）的更多信息，请参阅 [Fuchsia 归档格式（FAR）](/development/source_code/archive_format.md)。

<!--
The package metadata archive, `meta.far`, contains metadata about
a package, presented as the `meta/` directory. `meta.far` has a
[merkleroot](merkleroot.md) which in practical terms is also known as the
merkleroot of a package.
 -->
包元数据的归档 `meta.far` 包含有关包的元数据，显示为 `meta/` 目录。`meta.far` 有一个 [merkle 根](merkleroot.md)（merkleroot），实际上也称为包的 merkleroot。

<!--
The `meta/` directory of a package contains at minimum two files:
 -->
包的 `meta/` 目录至少包含两个文件：

* `meta/package`

<!--
   The package identity file. This is a JSON file that contains the name and
   version of the package.
 -->
   包标识文件。这是一个包含包的名称和版本的 JSON 文件。

* `meta/contents`

<!--
   The contents file. This file is created by the `ffx package build` command,
   (or the legacy `pm update` and `pm build` commands). This file maps the
   user-facing file names of a package to the Merkle root of those files.
 -->
   内容文件。此文件由 `ffx package build` 命令（或旧版 `pm update` 和 `pm build` 命令）创建。该文件将包的面向用户文件名映射到这些文件的 Merkle 根。

<!--
If the package declares subpackages, the `meta/` directory also contains:
 -->
如果包声明了子包，那么 `meta/` 目录还包含：

* `meta/fuchsia.pkg/subpackages`

<!--
   The subpackages file. This is a JSON file that contains the name and version
   of each declared subpackage. From the perspective of the parent package, the
   subpackage name is used as a relative package URL when resolving the
   subpackage.
 -->
   子包文件。这是一个 JSON 文件，其中包含每个声明的子包的名称和版本。从父包的角度来看，子包名称在解析子包时用作相对包网址。

<!--
   Package build tools traverse subpackage references (declared through build
   dependency declarations and package manifest files that reference other
   package manifest files for each subpackage) to compute the version (package
   hash) of each subpackage and generate the `subpackages` file.
 -->
   包构建工具遍历子包引用（通过构建依赖声明项和引用了每个子包包清单文件的包清单文件）来计算每个子包的版本（包哈希）并生成 `subpackages` 文件。

<!--
Additionally, the `meta/` directory can contain files such as a component manifest.
For more information on component manifests, see
[Component manifests](/concepts/components/v2/component_manifests.md).
 -->
此外，`meta/` 目录可以包含组件清单等文件。要获取关于组件清单的更多信息，请参阅[组件清单](/concepts/components/v2/component_manifests.md)。

<!--
### `BLOB`s outside of `meta/` {#outside-blobs}
 -->
### `meta/` 外 `BLOB` {#outside-blobs}

<!--
Most files of a package exist outside of the `meta/`directory and each are a `BLOB`.
 -->
包的大多数文件都在 `meta/` 目录之外，每个文件都是一个 `BLOB`。

<!--
For example, these files can be like the following:
 -->
例如，这些文件可以如下所示：

* `bin/foo`
* `lib/libfdio.so`
* `data/mydata.db`

<!--
## Identification of a package
 -->
## 包标识

<!--
Every package in Fuchsia is identified by a `package-url`.
 -->
Fuchsia 中的每个包都由 `package-url` 标识。

<!--
Note: For more information about [Fuchsia package URLs](/concepts/packages/package_url.md).
 -->
注意：要获取更多信息，请参阅 [Fuchsia 包网址](/concepts/packages/package_url.md)。

<!--
### Absolute package URLs
 -->
### 绝对包网址

<!--
An absolute Fuchsia package URL identifies a system-addressable package, without
requiring any additional context, and looks like the following:
 -->
绝对 Fuchsia 包网址标识系统可寻址的包，不需要任何额外上下文，形如：

```
fuchsia-pkg://{{ '<var>' }}repository{{ '</var>' }}/{{ '<var>' }}package-name{{ '</var>' }}?hash={{ '<var>' }}package-hash{{ '</var>' }}#{{ '<var>' }}resource-path{{ '</var>' }}
```

<!--
Fuchsia has different intereprations of `fuchsia-pkg` URL depending on which parts of the URL are
present.
 -->
Fuchsia 对 `fuchsia-pkg` 网址有不同的解释，具体取决于出现了网址的哪些部分。

<!--
 * If the repository, package, and resource parts are present, then the URL
   identifies the indicated resource within the package.
 * If only the repository and package parts are present, then the URL identifies
   the indicated package itself.
 * If only the repository parts are present, then the URL identifies the
   indicated repository itself.
 -->
 * 如果仓库、包和资源部分出现，那么该网址标识指示的包内资源。
 * 如果只有仓库和包部分出现，那么该网址标识指示的包本身。
 * 如果仅出现仓库部分，则该网址标识指示的仓库本身。

<!--
The package parts can express varying degrees of specificity. At minimum the
package name must be present, optionally followed by the package hash.
 -->
包部件可以表达不同程度的特异性。至少必须存在包名称，其后可选择性地附加包哈希。

<!--
If the package hash is missing, the package resolver fetches the resources
from the newest revision of the package variant available to the client.
 -->
如果包哈希丢失，包解析器从客户端可用的包变体的最新版本中获取资源。

<!--
### Relative package URLs
 -->
### 相对包网址

<!--
A relative Fuchsia package URL identifies a subpackage given previously loaded
package (or subpackage) as "context". The repository and parent package are
implicit, and the subpackage name is used to look up the package hash in the
parent package's `"meta/fuchsia.pkg/subpackages"` file. (The package hash
cannot be overridden). A relative package URL looks like the following:
 -->
相对 Fuchsia 包网址以先前加载的包（或子包）为“上下文”对子包进行标识。仓库和父包是隐式的，子包名称用于在父包的 `meta/fuchsia.pkg/subpackages` 文件中查找包哈希。（包哈希不能覆盖。）相对包网址形如：

```
{{ '<var>' }}package-name{{ '</var>' }}#{{ '<var>' }}resource-path{{ '</var>' }}
```

<!--
As with absolute package URLs, the resource path may or may not be included.
 -->
与绝对包网址一样，资源路径可能包含也可能不包含。

[Subpackaging components]: /concepts/components/v2/subpackaging.md
[glossary.subpackage]: /glossary/README.md#subpackage
