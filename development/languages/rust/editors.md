# Rust Editor Configuration

[TOC]

## Intellij

为了设置Intellij，请参见[Intellij Rust网站](https://intellij-rust.github.io/).

设置完Intellij后，对需要做修改的GN目标运行`fx gen-cargo //garnet/foo/path/to/target:label`。 最后用Intellij打开相应的目录。


## VIM

参见 [`rust-lang/rust.vim`](https://github.com/rust-lang/rust.vim).

## Visual Studio Code

前文提到的VIM插件使用了RLS (Rust language server)。因此你需要首先
[安装 rustup](https://rustup.rs/) 并运行`rustup component add rls-preview rust-analysis rust-src`. 

安装RLS后, 需要安装[VSCode 插件](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

之后打开 File -> Preferences -> Settings (或输入 Ctrl+,). 并添加如下的设置：
`"rust.target": "x86_64-fuchsia"`.

最后，对需要做修改的GN目标运行`fx gen-cargo //garnet/foo/path/to/target:label`，之后在VSCode中打开相应的目录.
