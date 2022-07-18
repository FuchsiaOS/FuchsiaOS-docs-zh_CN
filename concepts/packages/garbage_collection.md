<!-- # Garbage collection -->

# 垃圾回收

<!-- ## Static vs dynamic indexing -->

## 静态与动态索引

<!-- Static indexing is used for base packages. At `pkgfs` startup, base packages are
pre-populated in `/pkgfs/packages` based on the `static_packages` index located in
`/system/data/static_packages`. These static packages are then used to bootstrap
the system, so that core components like `pkg_resolver`, `pkg_cache`, `netstack`,
`sysmgr` can be started. -->

基本包使用静态索引。当 `pkgfs` 启动时，基于位于 `/system/data/static_packages` 中的  `static_packages` 索引，基本包会被预填充到 `/pkgfs/packages` 当中。这些静态包之后会被用来启动系统，诸如 `pkg_resolver`, `pkg_cache`, `netstack`,
`sysmgr` 之类的核心组件将会被启动。

<!-- `pkgfs` doesn't maintain state across reboots but the base package set is
guaranteed to always be present. Base packages cannot be deleted. -->

重启之后 `pkgfs` 会被重置，但是基本包会始终存在（基本包不能被删除）。

<!-- The dynamic index stores a mapping of all ephemerally fetched packages. `pkgfs`
will pre-populate the dynamic index with any present packages (i.e. `meta.far`
and all `BLOB`s resolved) listed in `/system/data/cache_packages`. In memory, the dynamic
index has the most recently resolved version of a package with the same name
by keying on the `$name/$variant` of the package. `pkgfs` then "forgets" about
the old version of the package. The old version of the package is still present
in the system but no longer referenced. The dynamic index is then used to implement
garbage collection. -->

动态索引会保存一个所有临时获取的包的映射。`pkgfs` 会使用 `/system/data/cache_packages` 中存在的所有包（例如 `meta.far` 和 `BLOB` 相关的）来预填充动态索引。通过使用包的 `$name/$variant` 作为键，动态索引能始终指向一个同名包的最新版本。`pkgfs` 会忽略该包较旧的版本。旧版本的包在系统中仍然存在，但是无法被引用。因此，动态索引能够用来实现垃圾回收机制。

<!-- ## How to garbage collect -->

## 如何实现垃圾回收

<!-- There is no notion of installing a package in fuchsia and likewise no notion of
deleting a package. Rather, garbage collection can be thought of as a means to
reclaim space. Garbage collection can be triggered manually by running `pkgctl gc`
or it can be triggered by the `system-updater`. The implementation of garbage
collection uses the [`fuchsia.space/Manager` protocol](https://fuchsia.dev/reference/fidl/fuchsia.space?hl=en#fuchsia.space/Manager.Gc). The `system-updater` trigger
happens twice; once before a system update and once after fetching the [update package](update_pkg.md). -->

在 Fuchsia 中没有安装一个包或删除一个包的概念。准确地说，垃圾回收可以被当作是回收空间的一种手段。垃圾回收可以通过 `pkgctl gc` 来手动触发，也可以被 `system-updater` 触发。垃圾回收的实现可参考 [`fuchsia.space/Manager` protocol](https://fuchsia.dev/reference/fidl/fuchsia.space?hl=en#fuchsia.space/Manager.Gc)。通过 `system-updater` 会在两种情况下触发垃圾回收，一是系统升级之前，二是在获取到 [更新包](update_pkg.md)之后。

<!-- The `pkgfs` garbage collector currently uses set differences to determine which
packages are live packages. A package is considered live if any of the following
is true: -->

`pkgfs` 的垃圾回收机制目前通过排除法来决定哪个包才是有效的包。满足以下任一条件，则该包为有效包：

<!-- * A package is a base package in the static index.
* A package is in the process of being updated (by tracking the `meta.far` merkle
  root and any missing `BLOB`s until they’ve been fully resolved).
* A package is the most recently resolved version of an ephemeral package according to its `meta` or `package` in the dynamic index. -->

* 该包为静态索引中的基本包
* 该包正在升级中（通过追踪  `meta.far` 和任意的缺失的 `BLOB` 的墨克根（merkle root），直到它们被完全解析）
* 该包是某个临时包的最新版本（根据其在动态索引中的 `meta` or `package` 来判断）

<!-- When garbage collection runs, it deletes every `BLOB` in `blobfs` that is not referenced
by a live package. -->

运行垃圾回收时，它会删除未被有效包引用的 `blobfs` 中的所有 `BLOB`。

<!-- ## Known issues -->

## 已知问题

<!-- Existing garbage collection implementation is suboptimal. -->

目前的垃圾回收机制并非最优，仍存在如下缺陷。

<!-- * An old version of an ephemeral package that is open can be garbage
collected. This may lead the garbage collector to erase a package out
from under a component. -->

* 正在使用的旧版本包也会被回收。这会导致某个组件下的包被当作垃圾清除掉。

<!-- * If `system-updater` fails to download a new package, the garbage collector
 protects both the base package and the most recent package version, which leads
 to duplicate copies of every package. If this happens, you should reboot the
 Fuchsia device to clear the list of activated packages. -->

 * 如果 `system-updater` 没能成功下载新的包，垃圾回收器会同时保留新包和旧版本的包，这导致每个包都有重复的副本。如果发生这样的情况，需要重启设备来清除被激活的包的列表。
