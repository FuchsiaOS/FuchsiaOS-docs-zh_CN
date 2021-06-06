<!-- 
# Rust libraries
 -->
# Rust 库

<!-- 
This document explains what libraries are available for writing and reading Inspect data in Rust.
For specific documentation of each library, refer to the crate documentation linked on each section.
 -->
本文档讲述了在 Rust 中，有哪些库可用于写入和读取审视（Inspect）数据。对于每个库的特定文档，请参阅在各部分链接的 crate 文档。

<!-- 
## Libraries for writing Inspect
 -->
## 写入审视的库

### [`fuchsia-inspect`][fuchsia_inspect]

<!-- 
This is the core library. This library offers the core API for creating nodes, properties,
serving Inspect, etc. Internally it implements the buddy allocation algorithm described in
[Inspect vmo format][inspect_vmo_format].

For an introduction to Inspect concepts and the rust libraries, see the
[codelab][codelab].
 -->
这是核心库。该库为创建节点、创建属性、服务审视（Inspect）等提供了核心 API。内在地，它实现了在[审视 VMO 格式][inspect_vmo_format]中描述的伙伴分配算法（the buddy allocation algorithm）。

## [`inspect-runtime`][inspect_runtime]

<!-- 
This library enables components to serve Inspect and make it available to the
Archivist for collection.

It's intended to be used only by component binaries, not libraries. The reason for this
is that this library provides an `expect_includes` check to verify that component manifests are
configured to expose Inspect.

Avoid depending on `inspect-runtime` from libraries so that tests are not forced
to add the Inspect `client.shard.cml`.
 -->
该库启用了组件来服务审视，并使其可供归档器（the Archivist）收集。

它旨在仅由组件二进制文件使用，而非由库使用。其原因是该库提供了 `expect_includes`（期望包含）检查，以验证该组件清单配置为公开审视（expose Inspect）。

请避免依赖来自库的 `inspect-runtime`（审视时），以便测试不被强制添加 `client.shard.cml` 审视。

### [`fuchsia-inspect-contrib`][fuchsia_inspect_contrib]

<!-- 
This library is intended for contributions to the Inspect library from clients.
These are patterns that clients identify in their usage of Inspect that they can
generalize and share. It’s intended to be at a higher level than
`fuchsia-inspect`.
 -->
该库是用于来自客户端的针对审视库的贡献的。这些是客户端在使用审视时识别的模式，它们可以将其概括与共享。其旨在处于比 `fuchsia-inspect` 更高的层次。

### [`fuchsia-inspect-derive`][fuchsia_inspect_derive]

<!-- 
This library provides a convenient way to manage Inspect data in a Rust program through a
`#[derive(Inspect)]` procedural macro. This works at a higher level than `fuchsia-inspect`.
For more information on this library, see [Ergonomic Inspect][ergonomic_inspect].
 -->
该库提供了一个在 Rust 程序中通过 `#[derive(Inspect)]` 过程宏（procedural macro）管理审视数据的便捷方式。它工作在比 `fuchsia-inspect` 更高的层次上。要获取更多信息，请参阅[工效学审视][ergonomic_inspect]

<!-- 
## Libraries for reading Inspect
 -->
## 读取审视的库

<!-- 
These libraries are not specific to Inspect and are used for various kinds of diagnostics data.
 -->
这些库并非审视专用，它们用于各类诊断数据。

### [`diagnostics-hierarchy`][diagnostics_hierarchy]

<!-- 
This library includes the convenient macro `assert_data_tree` for testing as well as the
definition of the `DiagnosticsHierarchy`, which is not exclusive to Inspect and
is also used for logs and other diagnostics data sources.
 -->
该库包含了用于测试的便捷的 `assert_data_tree` 宏，以及 `DiagnosticsHierarchy`（诊断层次）的定义，它并非审视独有，而是也用于日志和其他诊断数据源。

### [`diagnostics-testing`][diagnostics_testing]

<!-- 
This library includes the convenient `EnvForDiagnostics` which is useful for testing Inspect
integration in Components v1.
 -->
该库包含便捷的 `EnvForDiagnostics`，它对于测试 Components v1 中的审视整合情况很有用。

### [`diagnostics-reader`][diagnostics_reader]

<!-- 
This library includes the convenient `ArchiveReader` which is useful for fetching Inspect
data from an archivist in a test or in production. It wraps the shared logic of
connecting to the `ArchiveAccessor` and fetching data from it.
 -->
该库包含了便捷的 `ArchiveReader`，在测试或生产中，它在取回来自归档器的审视数据方面很有用。它包装了连接至 `ArchiveAccessor` 并从中取回数据这一共享逻辑。

<!-- 
## Others
 -->
## 其他

### [`inspect_format`][inspect_format]

<!-- 
This library provides an API for reading and writing the blocks of the
[Inspect VMO format][inspect_vmo_format].
 -->
该库提供了用于读取和写入[审视 VMO 格式][inspect_vmo_format]块（block）的 API。


[codelab]: /docs/development/diagnostics/inspect/codelab/codelab.md#rust
[ergonomic_inspect]: /docs/development/languages/rust/ergonomic_inspect.md
[inspect_vmo_format]: /docs/reference/diagnostics/inspect/vmo-format.md
[inspect_format]: https://fuchsia-docs.firebaseapp.com/rust/inspect_format/index.html
[inspect_runtime]: https://fuchsia-docs.firebaseapp.com/rust/inspect_runtime/index.html
[fuchsia_inspect_derive]: https://fuchsia-docs.firebaseapp.com/rust/fuchsia_inspect_derive/index.html
[fuchsia_inspect]: https://fuchsia-docs.firebaseapp.com/rust/fuchsia_inspect/index.html
[fuchsia_inspect_contrib]: https://fuchsia-docs.firebaseapp.com/rust/fuchsia_inspect_contrib/index.html
[diagnostics_hierarchy]: https://fuchsia-docs.firebaseapp.com/rust/diagnostics_hierarchy/index.html
[diagnostics_reader]: https://fuchsia-docs.firebaseapp.com/rust/diagnostics_reader/index.html
[diagnostics_testing]: https://fuchsia-docs.firebaseapp.com/rust/diagnostics_testing/index.html
