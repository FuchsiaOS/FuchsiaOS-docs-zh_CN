<!--
# Go

- [Logging](logging.md)
- [Readability reviews](readability_reviews.md)
-->

# Go 语言

- [Logging](logging.md)
- [Readability reviews](readability_reviews.md)

<!--
## Development setup

The layout of the Fuchsia checkout is not compatible out-of-the-box with
standard Go tooling like `go test` and `gopls` (the Go language server).

To set up your workspace for compatibility with Go tooling, run `fx setup-go`.
This will create all of the symlinks and other files necessary for Go tooling
and to work and be compatible with IDEs.

Note that this is only necessary if you care about IDE features and/or running
`go` commands directly. If you're happy without IDE features and with using `fx
set`, `fx build`, and `fx test` to work with Go, feel free to skip this setup.
-->

## 开发设置

Fuchsia checkout 的布局与标准 Go 工具（如“go test”和“gopls”（Go 语言服务器））不兼容。

要设置你的工作区以兼容 Go 工具，可以运行 “fx setup-go”。 这将创建 Go 工具所需的所有符号链接和其他文件，并与 IDE 兼容。

需注意，仅当你关心 IDE 的 features 或想要直接运行 `go` 命令时才需要这样做。 如果对没有 IDE features、而是使用 `fx set`、`fx build` 和 `fx test` 这类命令来开发的 Go 语言开发者，请跳过此设置。