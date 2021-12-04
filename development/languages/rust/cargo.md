<!--
# Using cargo on Fuchsia

Many tools in the Rust ecosystem assume you are using cargo. For tools like
this, which don't have a counterpart in our build, there is a utility for
generating `Cargo.toml` files. You can run cargo itself (`cargo check` for
instance) using it.

This functionality is maintained by volunteers. GN and Cargo have some design
mismatches that may result in the generated `Cargo.toml` files not working at
all or needing manual tweaks. Because of this, cargo in the Fuchsia tree is
**not** officially supported; things may break from time to time.
-->

# 在 Fuchsia 上使用 cargo

Rust 生态系统中的许多工具认为你在使用 cargo 。这些工具，在我们的构建中没有对应的，倒是有一个公用程序来生成 `Cargo.toml` 文件。你可以运行 cargo （例如 `cargo check`）来使用它。

这个功能由志愿者维护。 GN 和 Cargo 有一些不互相匹配的设计，可能导致生成的 `Cargo.toml` 文件完全不能工作或者需要手动调整。因此，cargo 在 Fuchsia 树里是不受官方支持的; 可能时不时会中断。

<!--
### Generating `Cargo.toml` files {#cargo-toml-gen}

In order to generate the cargo files based on the build graph of GN, add `--cargo-toml-gen` to
the `fx set` command or the `//build/rust:cargo_toml_gen` target to `$OUT_DIR/args.gn`'s
`universe_package_labels`. This adds a few seconds to `gn gen`. Make sure to run
a full `fx build` after updating the setting:

```sh
fx set PRODUCT.BOARD --cargo-toml-gen <other fx args>
fx build
```
-->

### 生成 `Cargo.toml` 文件 {#cargo-toml-gen}

为了生成基于 GN 构建图的 cargo 文件，要把`--cargo-toml-gen` 参数添加到 `fx set` 命令中或者是 `//build/rust:cargo_toml_gen` 目标指向 `$OUT_DIR/args.gn` 的 `universe_package_labels`。这会让 `gn gen` 多增加几秒钟。确保在更新设置之后完整运行一次 `fx build`。

```sh
fx set PRODUCT.BOARD --cargo-toml-gen <other fx args>
fx build
```

<!--
**Note:** If a `Cargo.toml` is required to complete an `fx build`, such as in the
case `cbindgen` needs to be run to generate new C bindings for a Rust crate, you may
need to use the `//build/rust:cargo_toml_gen` build target instead. This target will only
build the `Cargo.toml` files.

Most editors require the `Cargo.toml` file to be in a location that is adjacent to
the `src/` directory. Symlinks to these files can be generated using the following
commands, where `//garnet/foo/path/to/target:label` is the GN target that you want
to work on:

```sh
fx gen-cargo garnet/foo/path/to/target:some_label
```
-->

**注意：** 如果一个 `fx build` 需要一个 `Cargo.toml` 来完成，比如在一个例子里， `cbindgen` 需要为了一个 Rust crate 而运行以产生新的 C 绑定，你可能需要使用 `//build/rust:cargo_toml_gen` 构建目标来替代。这个目标将只构建 `Cargo.toml` 文件。

大多数编辑器都要求一个靠近 `src/` 目录的 `Cargo.toml` 文件。可以使用下边的命令来产生这些文件的符号链接，`//garnet/foo/path/to/target:label` 是你期望的 GN 目标工作的路径：

```sh
fx gen-cargo garnet/foo/path/to/target:some_label
```

<!--
**Note:** The above will not work for `rustc_staticlib` targets, e.g. Rust
crates used to generate C bindings generally use the `rustc_staticlib` target
template. For `rustc_staticlib` targets, you should use the following command instead.

```sh
fx gen-cargo garnet/foo/path/to/target:_some_label_rustc_static
```

Note that this label must point to a [`rustc_...` GN template](README.md#build)
(not a Fuchsia package or other GN target). For example:

```
rustc_binary("some_label") {
   ...
}
```
-->

**注意：** 上边的说明不适用于 `rustc_staticlib` 目标，例如，用于生成 C 绑定的 Rust crates 一般都使用了 `rustc_staticlib` 目标模板。对于 `rustc_staticlib` 目标，你应该使用下面的命令来替代。

```sh
fx gen-cargo garnet/foo/path/to/target:_some_label_rustc_static
```

要注意这个标签必须指向一个 [`rustc_...` GN template](README.md#build)（而非一个 Fuchsia 包或者其它 GN 目标）。例如：

```
rustc_binary("some_label") {
   ...
}
```

<!--
### Generating .cargo/config files {#cargo-config-gen}

Some plugins require a `.cargo/config` file to allow cargo to operate correctly for Fuchsia
(e.g. to run `cargo check`). To easily generate this file, use the [`fargo`][fargo] tool.

1. [Install rustup](https://rustup.rs/)
2. Configure `rustup` to use the Fuchsia Rust toolchain by running:

    ```sh
    rustup toolchain link fuchsia $($FUCHSIA_DIR/scripts/youcompleteme/paths.py VSCODE_RUST_TOOLCHAIN)
    rustup default fuchsia
    ```

3. Clone and install the `fargo` tool within your `$FUCHSIA_DIR` by following the
[getting started instructions][fargo] for fargo.
4. Create your config:

    ```sh
    cd $FUCHSIA_DIR && fargo write-config
    # Note the caveats about changing architecture in the fargo readme
    # https://fuchsia.googlesource.com/fargo/#creating-a-cargo_config
    ```
-->

### 生成 .cargo/config 文件 {#cargo-config-gen}

一些插件要求有一个 `.cargo/config` 文件来允许 cargo 正确的在 Fuchsia 上进行操作（例如，运行 `cargo check`）。简易生成这个文件的方式是使用 [`fargo`][fargo] 工具。

1. [安装 rustup](https://rustup.rs/)
2. 运行下边的脚本命令配置 `rustup` 使用 Fuchsia Rust 工具链：

    ```sh
    rustup toolchain link fuchsia $($FUCHSIA_DIR/scripts/youcompleteme/paths.py VSCODE_RUST_TOOLCHAIN)
    rustup default fuchsia
    ```

3. 参照 fargo [入门说明][fargo]，在你的 `$FUCHSIA_DIR` 目录中克隆并安装 `fargo` 工具。
4. 创建你的配置：

    ```sh
    cd $FUCHSIA_DIR && fargo write-config
    # 要注意 fargo readme 中关于架构改变的注意事项
    # https://fuchsia.googlesource.com/fargo/#creating-a-cargo_config
    ```


[fargo]: https://fuchsia.googlesource.com/fargo/
