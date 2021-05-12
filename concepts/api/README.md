<!--
# API Development

This document is a top-level entry point to documentation related to developing
APIs for Fuchsia.
-->
# API 开发

本文档是与为 Fuchsia API 相关的文档的顶层入口。
<!--
## What this covers

Although the documentation in this directory applies to all Fuchsia APIs, it
will be enforced for the _public facing surface area_ of Fuchsia: the Fuchsia
APIs that are surfaced to developers via IDK releases.  All public facing API
changes will be reviewed by the [API Council][api-council] for consistency with
these guidelines.
-->
## 这包含了什么

虽然此目录中的文档适用于所有 Fuchsia API, 它将强制用于 Fuchsia 的面向公众的层面: 通过IDK发布面向开发者的 Fuchsia api.  公众面对的 API 变化将由[API 委员会][api-council] 审查，以保持这些准则的一致性.
<!--
## Rubrics

The documentation in this directory comes in the form of _rubrics_, which are
established protocols for how to design and build APIs.  Note that the list
below is not complete: as Fuchsia evolves, more rubrics will be added.
-->
## 标准

这个目录中的文档以标题的形式出现，它们是用于设计和构建 api 的协议。请注意，下面的列表并不完整:随着 Fuchsia 的发展，将添加更多的标准
<!--

 * [API Documentation](documentation.md)
 * [CLI and GUI tools](tools.md)
 * Languages
   * [C API Readability](c.md)
   * [Dart API Readability](dart.md)
   * [FIDL Style][fidl-style]
   * [FIDL API][fidl-api]
 * Domain-specific areas
   * [Zircon System Interface](system.md)
   * [Fuchsia Device Interface](device_interfaces.md)
-->
 * [API 文档](documentation.md)
 * [CLI 和 GUI 工具](tools.md)
 * 语言
   * [C API 可读性](c.md)
   * [Dart API 可读性](dart.md)
   * [FIDL 风格][fidl-style]
   * [FIDL API][fidl-api]
 * 特定领域
   * [Zircon 系统接口](system.md)
   * [Fuchsia 设备接口](device_interfaces.md)

<!-- xrefs -->
[api-council]: /docs/contribute/governance/api_council.md
[fidl-style]: /docs/development/languages/fidl/guides/style.md
[fidl-api]: /docs/concepts/api/fidl.md
