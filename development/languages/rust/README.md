# Rust

## 构建目标

有两种GN构建目标可供Rust项目使用：

- [`rustc_library`][target-library-rustc] 定义一个可供其他目标使用的库
- [`rustc_binary`][target-binary-rustc] 定义一个可执行输出

[garnet/examples/rust][rust-examples]目录下有一些作为示例的Rust包，它们同时使用了上述的目标和[Rust FIDL][fidl-tutorial].

由于这些GN目标在正常的Fuchsia构建过程中并不是由Cargo构建的，因此它们不要求有`Cargo.toml` 文件。但是仍然可以对这些目标运行
`fx gen-cargo path/from/fuchsia/root/to/target:label`来生成一个`Cargo.toml`文件。为了做到这一点，首先需要有一个包含这些目标，且成功完成的Fuchsia构建。
生成`Cargo.toml`对于以下情况十分有用：1. 与IDE（如Intellij或VSCode）集成；2. 使用[`fargo`][fargo]工具，在不进行完整的GN构建时，对本目标进行构建和测试。

你也可以使用`fx`在已链接的设备上运行单元测试。具体的命令为`fx run-test {package name}_{bin or lib}_test_rustc`。
注意：在相应包的`BUILD.gn`中需要有`with_unit_tests = true`。

## 代码风格

我们现在尚没有一个Rust的风格指南，但在提交前你应该运行[`rustfmt`][rustfmt-install]。除了一些[自定义设置][rustfmt-toml]外，我们基本完全使用`rustfmt`的默认设置。

## 用自定义工具链构建

如果你想试着用自己构建的rustc或cargo进行构建，你可以通过`fx set`的`rustc_prefix`进行设置，如：

```
fx set x64 --release --args "rustc_prefix=\"/path/to/bin/dir\""
```

## 交流渠道

[rust@fuchsia.com]邮件列表有公开的讨论。对于Googler请访问[go/fuchsia-rust-googlers].

## 现有的Fuchsia Rust库

- [crates 列表](crates.md)
- [为crates自动生成的文档](https://fuchsia-docs.firebaseapp.com)

## 更多信息

- [编辑器设置](editors.md)
- [管理第三方依赖](third_party.md)
- [Unsafe代码](unsafe.md)
- [不稳定的语言特性](unstable.md)
- [Rust FIDL教程][fidl-tutorial]
- [Syslog API](syslog.md)

[target-library-rustc]: https://fuchsia.googlesource.com/build/+/master/rust/rustc_library.gni "Rust library"
[target-binary-rustc]: https://fuchsia.googlesource.com/build/+/master/rust/rustc_binary.gni "Rust binary"
[rust-examples]: https://fuchsia.googlesource.com/garnet/+/master/examples/rust/
[fargo]: https://fuchsia.googlesource.com/fargo
[rustfmt-install]: https://github.com/rust-lang-nursery/rustfmt#quick-start
[rustfmt-toml]: https://fuchsia.googlesource.com/garnet/+/master/rustfmt.toml
[fidl-tutorial]: https://fuchsia.googlesource.com/docs/+/HEAD/development/languages/fidl/tutorial/README.md#server-in-rust
[rust@fuchsia.com]: https://groups.google.com/a/fuchsia.com/forum/#!forum/rust-fuchsia
[go/fuchsia-rust-googlers]: https://goto.google.com/fuchsia-rust-googlers
