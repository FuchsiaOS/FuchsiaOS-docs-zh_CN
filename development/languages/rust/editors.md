<!--
# Rust editor configuration

As there is no specific editor for Rust development on Fuchsia, `vim` and `VS Code` are the
most popular options. However, documentation for setting up any editor is welcome in this document.
-->

# Rust 编辑器配置
在 Fuchsia 上做 Rust 开发并没有特定的编辑器，`vim` 和 `VS Code` 是最主流的选择。然而，任何编辑器的设置文档在这里都是受欢迎的。

<!--
## `rust-analyzer` setup {#rust-analyzer}

[rust-analyzer](https://rust-analyzer.github.io/) is a [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP)
implementation for Rust. This is the recommended workflow and will work with minimal editor setup.

`rust-analyzer` uses a file in the `out/` directory called `rust-project.json` that is
generated based on the build graph at `gn gen` time. A symlink to the `rust-project.json` is located
in the root directory of the Fuchsia tree.

The `rust-project.json` file format is currently unstable. Sometimes this can cause an
unexpected version mismatch where GN produces a `rust-project.json` that `rust-analyzer` is
not expecting, causing `rust-analyzer` to not work correctly.
-->

## `rust-analyzer` 设置 {#rust-analyzer}

[rust-analyzer](https://rust-analyzer.github.io/) 是一个为 Rust 实现的[语言服务器协议](https://microsoft.github.io/language-server-protocol/) (LSP)。这是推荐的工作流，仅需最小的编辑设置就可以工作。

`rust-analyzer` 在 `out/` 目录下使用了一个叫做 `rust-project.json` 的文件，这个文件在 `gn gen` 时基于构建图生成。指向 `rust-project.json` 的符号链接位于 Fuchsia 树的根目录下。

`rust-project.json` 文件格式目前尚未稳定。由此有时会导致一个版本差异的意外发生，在 GN 产生的 `rust-project.json` 并非 `rust-analyzer` 所期望的文件时，会导致 `rust-analyzer` 工作异常。

<!--
### Currently supported version {#supported-rust-analyzer-version}

Currently, use [the **latest version** of `rust-analyzer`][rust-analyzer-latest].

## Visual Studio Code {#visual-studio-code}

To use `rust-analyzer` with VSCode, use the latest stable version of
VSCode since `rust-analyzer` frequently depends on recent language server features.
VSCode can be downloaded from the
[official VSCode website][vscode-download].
It is recommended to:

* Keep automatic updates turned on (not available for Linux, see these
  [update instructions][vscode-update]).
* If you are working on confidential code,
  [disable telemetry reporting][vscode-disable-telemetry] as a precaution.
-->

### 当前支持的版本
{#supported-rust-analyzer-version}

当前，请使用 [**最新版本**的 `rust-analyzer`][rust-analyzer-latest]。

## Visual Studio Code {#visual-studio-code}

要在 VSCode 中使用 `rust-analyzer`，请使用最新稳定版本的 VSCode，因为 `rust-analyzer` 频繁依赖于新近的语言服务器功能。
VSCode 可以从 [VSCode 官方网站][vscode-download]上下载。
推荐的用法：
* 保持开启自动升级功能（不适用于 Linux 平台，请查看[升级介绍][vscode-update]）。
* 如果你的编码工作涉密，要提前注意[禁止遥测报告][vscode-disable-telemetry]。

<!--
### `rust-analyzer` VSCode extension (supported workflow)

Note:  This is the [`rust-analyzer`][vscode-rust-analyzer] extension, not the `rust` extension that
VSCode may recommend for Rust files.

You can install the `rust-analyzer` extension directly
[from the VSCode marketplace][vscode-rust-analyzer].

Once you have installed the rust-analyzer extension, add the following
configurations to your Workspace `settings.json` file:
-->

### `rust-analyzer` VSCode 扩展（支持工作流）

注意：这是 `rust-analyzer` 扩展，而不是 VSCode 可能推荐的针对 Rust 文件的 `rust` 扩展。

你可以[从 VSCode 市场][vscode-rust-analyzer]直接安装`rust-analyzer` 扩展。一旦你安装了 `rust-analyzer` 扩展，把下边的配置添加到你工作空间的 `settings.json` 文件中：

<!--
Note: To access the VS Code Workspace settings, click the **View** menu, then **Command Palette**,
and select `Preferences: Open Workspace Settings (JSON)`.

```javascript
{
  // disable cargo check on save
  "rust-analyzer.checkOnSave.enable": false,
  "rust-analyzer.checkOnSave.allTargets": false,
}
```

In addition, the following settings may provide a smoother experience:

```javascript
{
  // optional: only show summary docs for functions (keeps tooltips small)
  "rust-analyzer.callInfo.full": false,
  // optional: don't activate parameterHints automatically
  "editor.parameterHints.enabled": false,
}
```
-->
注意：要访问 VS Code 的工作空间设置，点击 **查看(View)** 菜单，然后选择 **命令面板(Command Palette)**，再选择 `首选项：打开工作区设置(Preferences: Open Workspace Settings (JSON))`。
```javascript
{
  // 禁止在保存时执行 cargo check
  "rust-analyzer.checkOnSave.enable": false,
  "rust-analyzer.checkOnSave.allTargets": false,
}
```
此外，下边的设置可能提供一个更平顺的体验：

```javascript
{
  // 可选的: 只为方法显示摘要文档 (保持工具提示小形化)
  "rust-analyzer.callInfo.full": false,
  // 可选的: 不自动激活参数提示
  "editor.parameterHints.enabled": false,
}
```

<!--
### Enabling rustfmt in the `rust-analyzer` extension

The `rust-analyzer` extension relies on the `rustup` tool to choose the right toolchain for
invoking rustfmt, so you need to tell `rustup` about your Fuchsia checkout and its toolchain:

```shell
$ rustup toolchain link fuchsia-tools $FUCHSIA_DIR/prebuilt/third_party/rust_tools/<host os>
```

For instance, on linux, you would use `linux-x64`, or on MacOS you would use `mac-x64` (there should
only be one option in your Fuchsia directory, set up correctly for your host development system):
-->

### 在 `rust-analyzer` 扩展中使能 rustfmt

`rust-analyzer` 扩展依赖于 `rustup` 工具以选择正确的工具链来调用 rustfmt，所以你需要告诉 `rustup` 你检出的 Fuchsia 和它的工具链：

```shell
$ rustup toolchain link fuchsia-tools $FUCHSIA_DIR/prebuilt/third_party/rust_tools/<host os>
```

例如，在 linux 系统上，你会用到 `linux-x64`，或者在 MacOS 系统上你会用到 `mac-x64` （在你的 Fuchsia 目录中只会有一个选择，请在你的开发主机系统中正确设置）：

<!--
```shell
$ rustup toolchain link fuchsia-tools $FUCHSIA_DIR/prebuilt/third_party/rust_tools/linux-x64/
```

Having done that, the `rust-analyzer` extension can be configured to use this toolchain and
the Fuchsia `rustfmt.toml`.  Open Workspace settings as above, and add:

```javascript
{
    // use fuchsia-tools toolchain and fuchsia's rules for rustfmt:
    "rust-analyzer.rustfmt.extraArgs": [
        "+fuchsia-tools",
        "--config-path=<path to $FUCHSIA_DIR>/rustfmt.toml"
    ],
}
```
-->

```shell
$ rustup toolchain link fuchsia-tools $FUCHSIA_DIR/prebuilt/third_party/rust_tools/linux-x64/
```

完成了上边的步骤，`rust-analyzer` 扩展将可以配置使用这个工具链和 Fuchsia 的 `rustfmt.toml`。就像上边提到的一样打开工作区设置，添加以下内容：

```javascript
{
    // 使用 fuchsia-tools 工具链和 fuchsia 的 rustfmt 规则
    "rust-analyzer.rustfmt.extraArgs": [
        "+fuchsia-tools",
        "--config-path=<path to $FUCHSIA_DIR>/rustfmt.toml"
    ],
}
```

<!--
### A note on `rust-analyzer` and symlinked Fuchsia directories

If your Fuchsia workspace is symlinked from elsewhere (such as another mountpoint), the
`rust-analyzer` extension may not be able to properly locate the files for analysis as they are
opened in VSCode.

`rust-analyzer`, and the `rust-project.json` file, contain _absolute_ paths to your source files.
As such, if you open the Fuchsia directory via the symlink'd location, it will not match those
absolute paths, and the `rust-analyzer` VSCode extension will not be able to align the opened files
in VSCode with the files that the `rust-analyzer` LSP is parsing and analyzing.

Instead, open the actual path to the Fuchsia source in VSCode, so that the LSP and the editor see
the same paths to all source files.
-->

### 一个关于 `rust-analyzer` 和 Fuchsia 目录符号链接的注意事项

如果你的 Fuchsia 工作区是由别处通过符号链接而来（比如其它挂载点），`rust-analyzer` 扩展可能无法正确定位在VSCode中已打开的文件以进行分析。

`rust-analyzer`，以及 `rust-project.json` 文件，包含了你的源码文件的 _绝对_ 路径。
因此，如果你是通过符号链接定位来打开 Fuchsia 目录，将与那些绝对路径不符，VSCode 的 `rust-analyzer` 扩展将不能让已在 VSCode 中打开的文件与 `rust-analyzer` LSP 正在解析和分析中的文件进行对齐。

相反的，在 VSCode 中打开实际路径的 Fuchsia 源码，这样 LSP 和编辑器就能看到相同路径的所有源文件。

<!--
### Troubleshooting issues with `rust-analyzer`

If you notice that `rust-analyzer` is not working correctly, it could be due to a breaking
change in the `rust-project.json` file.  The first thing to try in this case is to `jiri update`,
rebase, and re-run `fx gen`, and see if the issue goes away:

```shell
$ jiri update
$ git rebase
$ fx gen
```
-->

### `rust-analyzer` 问题排故

如果你注意到 `rust-analyzer` 不能正确的工作，原因可能是 `rust-project.json` 发生了变化。对于这个问题，可以首先尝试的是 `jiri update`，rebase，然后重新运行 `fx-gen`，再看看问题是否已消失：

```shell
$ jiri update
$ git rebase
$ fx gen
```

<!--
#### Downgrading {#downgrading-rust-analyzer}
If not, you may need to [manually downgrade rust-analyzer][vscode-downgrade] to the
[currently-supported version](#supported-rust-analyzer-version) if it's not the latest, or to a
previous version you were using if the currently supported version is listed as "latest".
-->

#### 降级 {#downgrading-rust-analyzer}
如果不行，你可能需要[手动降级 rust-analyzer][vscode-downgrade] 到[当前支持的版本](#supported-rust-analyzer-version)，如果它不是最新的，或者是降级到你之前使用的，被列为“最新”的当前支持版本的前一个版本。

<!--
## Vim

For basic support, instructions on [`rust-lang/rust.vim`](https://github.com/rust-lang/rust.vim).

For IDE support, see the vim section of the [rust-analyzer manual](https://rust-analyzer.github.io/manual.html#vimneovim).

If you use Tagbar, see [this post](https://users.rust-lang.org/t/taglist-like-vim-plugin-for-rust/21924/13)
for instructions on making it work better with Rust.
-->

## Vim

基础的支持，在 [`rust-lang/rust.vim`](https://github.com/rust-lang/rust.vim) 上有介绍。

IDE 支持，请查看 [rust-analyzer 手册](https://rust-analyzer.github.io/manual.html#vimneovim) 中的 vim 段落。

如果你使用标签栏，请查看[这个帖子](https://users.rust-lang.org/t/taglist-like-vim-plugin-for-rust/21924/13)中关于更好的使用Rust的介绍。

<!--
## emacs

### Completion

See the [rust-analyzer manual](https://rust-analyzer.github.io/manual.html#emacs) for instructions.

### Check on save

You will be using [flycheck](https://www.flycheck.org/en/latest/) to compile
your Rust files when you save them.  flycheck will parse those outputs and
highlight errors.  You'll also use
[flycheck-rust](https://github.com/flycheck/flycheck-rust) so that it will
compile with cargo and not with rustc.  Both are available from
[melpa](https://melpa.org/#/).

Note that this workflow is based on cargo, which is more likely to break than
rust-analyzer based workflows.

If you don't yet have melpa, follow the instructions
[here](https://melpa.org/#/getting-started).

Install `flycheck` and `flycheck-rust` in `M-x list-packages`.  Type `i`
to queue for installation what you are missing and then `x` to execute.

Next, make sure that flycheck-rust is run at startup.  Put this in your `.emacs` files:

```elisp
(with-eval-after-load 'rust-mode
  (add-hook 'flycheck-mode-hook #'flycheck-rust-setup))
```
-->

## emacs

### 完整介绍

查看 [rust-analyzer 手册](https://rust-analyzer.github.io/manual.html#emacs) 获取相关介绍。

### 存档检查
当你保存你的 Rust 文件时，你将使用 [flycheck](https://www.flycheck.org/en/latest/) 来编译它们。flycheck 将解析编译输出并把错误高亮显示。你也可以使用 [flycheck-rust](https://github.com/flycheck/flycheck-rust)，这样就可以用 cargo 编译取代 rustc。两者都可从 [melpa](https://melpa.org/#/) 上获取。

要注意这个工作流是基于 cargo，比基于 rust-analyzer 的工作流更容易损坏。

如果你还没有 melpa ，按照 [这里](https://melpa.org/#/getting-started) 的说明。

在 `M-x 包列表` 里安装 `flycheck` 和 `flycheck-rust` 。输入 `i` 来顺序安装缺失的包，然后输入 `x` 来执行。

下一步，确保 flycheck-rust 在启动的时候就运行。把下边的内容添加到你的 `.emacs` 文件中：

```elisp
(with-eval-after-load 'rust-mode
  (add-hook 'flycheck-mode-hook #'flycheck-rust-setup))
```


<!--
You'll want cargo to run "check" and not "test" so set
`flycheck-rust-check-tests` to `nil`.  You can do this by typing `C-h v
flycheck-rust-check-tests<RET>` and then customizing the variable in the normal
way.

Now, you'll want to make sure that the default `cargo` and `rustc` that you are
using are Fuchsia versions of those.  From your fuchsia root, type:

```elisp
rustup toolchain link fuchsia $PWD/prebuilt/third_party/rust/linux-x64 && rustup default fuchsia
```

Finally, follow the steps at the top of this page to generate a `Cargo.toml` for the GN target
that you want to work on.

You can [read about](http://www.flycheck.org/en/latest/user/error-reports.html)
adjusting flycheck to display your errors as you like.  Type `C-h v
flycheck-highlighting-mode<RET>` and customize it.  Also customize `C-h v
flycheck-indiation-mode<RET>`.

Now restart emacs and try it out.
-->

你将想要 cargo 来运行 "check" 而不是 "test"，所以要把 `flycheck-rust-check-tests` 设置到 `nil`。你可以输入 `C-h v
flycheck-rust-check-tests<RET>` 来达到这个目的，并且可以以常规方式自定义变量。
至此，你将需要确保你使用的默认的 `cargo` 和 `rustc` 都是 Fuchsia 上的版本。在你的 fuchsia 根目录下启动命令行，输入：

```elisp
rustup toolchain link fuchsia $PWD/prebuilt/third_party/rust/linux-x64 && rustup default fuchsia
```

最后，跟从本文开始提到的步骤为你的 GN 目标生成一个 `Cargo.toml`。

你可以 [参阅](http://www.flycheck.org/en/latest/user/error-reports.html) 调整 flycheck 以你喜欢的方式来显示你的错误。输入 `C-h v
flycheck-highlighting-mode<RET>` 并定制它。当然也可以定制 `C-h v
flycheck-indiation-mode<RET>` 。

现在可以重启 emacs 并试试看。

<!--
### Test and debug

To test that it works, you can run `M-x flycheck-compile` and see the
command-line that flycheck is using to check syntax.  It ought to look like one
of these depending on whether you are in a lib or bin:

```sh
cargo check --lib --message-format\=json
cargo check --bin recovery_netstack --message-format\=json
```

If it runs `rustc` instead of `cargo`, that's because you didn't `fx gen-cargo`.

Note that it might report errors on the first line of the current file.  Those are
actually errors from a different file.  The error's comment will name the
problematic file.
-->

### 测试和调试

要测试它正常工作，你可以运行 `M-x flycheck-compile` 然后查看命令行可以看到 flycheck 正在检查语法。它起来应该像是那些依赖于你是否处于 lib 或 bin 目录中：

```sh
cargo check --lib --message-format\=json
cargo check --bin recovery_netstack --message-format\=json
```

如果它运行的是 `rustc` 而不是 `cargo` ，那是因为你没有设置 `fx gen-cargo` 。

要注意的是它可以在当前文件的第一行报错。那些实际上的错误来自不同的文件。这些错误的注释里会提到这些有问题的文件。

<!--
## Sublime Text {#sublime-text}

### Using Rust-Enhanced for syntax checking

Follow the steps above to [generate a `Cargo.toml` file][cargo-toml-gen] and also
the steps to [generate a `cargo/config` file][cargo-config-gen], which will also
setup `cargo` to use the Fuchsia Rust toolchain.

Then, install the [Rust Enhanced](https://packagecontrol.io/packages/Rust%20Enhanced) plugin.
Now, you should have syntax checking on save and be able to run `cargo check` from the
context menu / command palette. Thanks to `fargo`, some tests also appear to run OK, but this
hasn't been thoroughly tested.

### Using a language server for intellisense / hover tooltips / go-to-definition
-->

## Sublime Text {#sublime-text}

### 使用 Rust-Enhanced 进行语法检查

依照上边的步骤来 [生成一个 `Cargo.toml` 文件][cargo-toml-gen]，还有就是 [生成一个 `cargo/config` 文件][cargo-config-gen]的步骤，将会设置 `cargo` 使用 Fuchsia Rust 工具链。

然后，安装 [Rust Enhanced](https://packagecontrol.io/packages/Rust%20Enhanced) 插件。
现在，你的 Sublime Text 应该可以在保存时进行语法检查了并且可以从上下文菜单 / 命令面板 (context menu / command palette) 运行 `cargo check` 。感谢 `fargo` ，一些测试看起来运行正常，但是没有进行过彻底的测试。

### 为 智能感知 / 悬浮工具提示 / 转到定义 使用语言服务器

<!--
#### Setup

First, install the [LSP package](https://github.com/sublimelsp/LSP) for Sublime. Then,
follow  the [rust-analyzer setup instructions](https://rust-analyzer.github.io/manual.html#sublime-text-3)
for Sublime.

#### Usage

In order for the language server to work, you need to open a folder that contains a `Cargo.toml`
as the root of your Sublime project. There are two ways you can do this:

1. Open a new Sublime window for the folder that contains the `Cargo.toml` (e.g.
`garnet/foo/path/to/target`)
2. Or, go to the top menu bar -> Project -> Add Folder to Project. This will keep all your files
inside one Sublime window, and works even if you have the broader `fuchsia` folder also open.

You may need to restart Sublime after these steps.
-->

#### 设置

首先，为 Sublime 安装 [LSP 包](https://github.com/sublimelsp/LSP)。然后，按照 [rust-analyzer 设置说明](https://rust-analyzer.github.io/manual.html#sublime-text-3) 设置 Sublime 。

#### 使用

为了让语言服务器工作，你需要打开包含 `Cargo.toml` 文件的目录来作为你的 Sublime 工程的根目录。有两种方式：

1. 为包含 `Cargo.toml` 文件的目录（比如 `garnet/foo/path/to/target`）打开一个新的 Sublime 窗口
2. 或者，从顶部的菜单栏 -> 工程(Project) -> 添加文件夹到工程(Add Folder to Project)。这样你的所有文件都会在一个 Sublime 窗口里，即使你打开的是更宽泛的 `fuchsia` 文件夹。

这些设置完成之后你可能需要重启 Sumblime 。


<!--
## Intellij (Custom code completion)

See instructions on [the Intellij Rust site](https://intellij-rust.github.io/).
Finally, follow [these steps][cargo-toml-gen] to generate a `Cargo.toml` file for use by Intellij.
Note that cargo-based workflows are more likely to break than rust-analyzer based ones.
-->

## Intellij （自定义代码补全）

查看 [Intellij Rust 网站](https://intellij-rust.github.io/) 上的介绍。
最后，按照 [这些步骤][cargo-toml-gen] 生成一个用于 Intellij 的 `Cargo.toml` 文件。
要注意的是基于 cargo 的工作流比基于 rust-analyzer 的工作流更容易损坏。

[rust-analyzer-latest]: https://github.com/rust-analyzer/rust-analyzer/releases
[vscode-download]: https://code.visualstudio.com/Download
[vscode-update]:  https://vscode-docs.readthedocs.io/en/stable/supporting/howtoupdate/
[vscode-disable-telemetry]: https://code.visualstudio.com/docs/getstarted/telemetry#_disable-telemetry-reporting
[vscode-rust-analyzer]: https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer
[vscode-downgrade]: https://code.visualstudio.com/updates/v1_30#_install-previous-versions
[cargo-toml-gen]: /docs/development/languages/rust/cargo.md#cargo-toml-gen
[cargo-config-gen]: /docs/development/languages/rust/cargo.md#cargo-config-gen
