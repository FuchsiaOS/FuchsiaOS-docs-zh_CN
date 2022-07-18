
<!--
# FIDL Rust Crates

FIDL is the primary mechanism for structured IPC within Fuchsia. The easiest
way to use FIDL from Rust is by generating a "FIDL crate" from a FIDL library
and then importing it from your Rust library or binary.

See [the FIDL Rust bindings](/docs/reference/fidl/bindings/rust-bindings.md) to
understand how different FIDL constructs map into their Rust equivalents, and
[the FIDL Rust tutorials][tutorials] for examples on using the Rust
bindings.
-->

# FIDL Rust Crates

FIDL 是在 Fuchsia 中实现结构化 IPC 的基本机制。从 Rust 使用 FIDL 最简便的途径是从一个 FIDL 库生成一个 "FIDL crate"，然后从你的 Rust 库或二进制应用中导入它。

查看 [FIDL Rust 绑定](/reference/fidl/bindings/rust-bindings.md) 以理解不同的 FIDL 构造如何映射到它们的 Rust 等价物，以及从 [FIDL Rust 教程][tutorials] 获取使用 Rust 绑定的示例。

<!--
## Build Instructions {#build}


When a [GN `fidl` rule](/build/fidl/fidl.gni) is defined for a FIDL library,
a correspoding FIDL Rust crate is automatically generated under
the original target name appended with `-rustc`. Transitive dependencies on
other FIDL libraries are resolved automatically.
For example, given the declaration:

```gn
# //src/tictactoe/BUILD.gn

fidl("games.tictactoe") { ... }
```
-->

## 构建指南 {#build}

当一个 [GN `fidl` 规则](/build/fidl/fidl.gni) 被定义给一个 FIDL 库，就会相应地自动生成一个 FIDL Rust crate，名称为原始目标名称加上 `-rustc`。其它 FIDL 库的传递依赖会自动解析。
例如，给定以下声明：

```gn
# //src/tictactoe/BUILD.gn

fidl("games.tictactoe") { ... }
```

<!--
The FIDL crate target is
`//src/tictactoe:games.tictactoe-rustc`. To use the FIDL crate,
add the target to the `deps` field of the
[`rustc_*` build rule](/docs/development/languages/rust/README.md#build)
for your Rust crate. For example:

```gn
rustc_binary("tictactoe") {
  # ...
  deps = ["//src/tictactoe:games.tictactoe-rustc"]
}
```
-->

得到的 FIDL crate 目标为 `//src/tictactoe:games.tictactoe-rustc`。要使用这个 FIDL crate, 在你的 Rust crate 里，把目标添加到 [`rustc_*` 构建规则](/development/languages/rust/README.md#build)的 `deps` 域中。例如：

```gn
rustc_binary("tictactoe") {
  # ...
  deps = ["//src/tictactoe:games.tictactoe-rustc"]
}
```

<!--
The Rust crate will be named `fidl_games_tictactoe` and its items can now be
imported:

```rust
use fidl_games_tictactoe::BOARD_SIZE;
```

In the Fuchsia tree, frequently used FIDL crates are often aliased to
a shorter name for brevity, like so:

```rust
use fidl_fuchsia_io2 as fio2;
use fidl_fuchsia_data as fdata;
```
-->

这个 Rust crate 将被命名为 `fidl_games_tictactoe`，它的组件现在可以被导入：

```rust
use fidl_games_tictactoe::BOARD_SIZE;
```

在 Fuchsia 树中，频繁使用的 FIDL crates 经常别名为更短的名称以达到简洁的目的，就像这样：

```rust
use fidl_fuchsia_io2 as fio2;
use fidl_fuchsia_data as fdata;
```

<!--
## Generated Documentation {#documentation}

Documentation in HTML format can be automatically
generated for a FIDL crate using the `fx rustdoc` command. For example:

```bash
fx rustdoc //src/tictactoe:games.tictactoe-rustc --open
```

FIDL crates in the public Fuchsia source tree are published in the
[Fuchsia Rust API reference](https://fuchsia-docs.firebaseapp.com/rust/).
-->

## 生成文档 {#documentation}

一个 FIDL crate 的 HTML 格式文档可以使用 `fx rustdoc` 命令自动生成。例如：

```bash
fx rustdoc //src/tictactoe:games.tictactoe-rustc --open
```

在公开的 Fuchsia 源码树中， FIDL crates 都发布在 [Fuchsia Rust API 参考](https://fuchsia-docs.firebaseapp.com/rust/)中。

<!--
## Generated Rust Code {#code}

To manually inspect the generated Rust code for a FIDL crate, the Rust
source files are available under the `BUILD_DIR/fidling/gen` (refer to the
[Generated code guide][generated-code] for an example). Note that
the FIDL crate must first have been built (e.g. using `fx build`).
-->

## 生成 Rust 代码 {#code}

要手动检查给一个 FIDL crate 生成的 Rust 代码，可以在 `BUILD_DIR/fidling/gen` 目录下找到相关的 Rust 源代码（参考 [代码生成指南][generated-code] 获取示例）。要注意必须先构建相应的 FIDL crate （例如，使用 `fx build`）。

<!-- xrefs -->
[generated-code]: /development/languages/fidl/guides/generated-code.md#rust
[tutorials]: /development/languages/fidl/tutorials/rust
