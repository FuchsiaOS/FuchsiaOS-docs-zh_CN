# API 开发

本文档是与为Fuchsia API相关的文档的顶层入口。

## 这包含了什么

虽然此目录中的文档适用于所有Fuchsia API, 它将强制用于Fuchsia的面向公众的层面: 通过IDK发布面向开发者的Fuchsia api.  公众面对的API变化将由[API委员会][api-council] 审查，以保持这些准则的一致性.

## 标准

这个目录中的文档以标题的形式出现，它们是用于设计和构建api的协议。请注意，下面的列表并不完整:随着Fuchsia的发展，将添加更多的标准

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
