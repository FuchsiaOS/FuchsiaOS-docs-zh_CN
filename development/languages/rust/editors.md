# Rust editor configuration

As there is no specific editor for Rust development on Fuchsia, `vim` and `VS Code` are the
most popular options. However, documentation for setting up any editor is welcome in this document.

## `rust-analyzer` setup {#rust-analyzer}

[rust-analyzer](https://rust-analyzer.github.io/) is a [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP)
implementation for Rust. This is the recommended workflow and will work with minimal editor setup.

`rust-analyzer` uses a file in the `out/` directory called `rust-project.json` that is
generated based on the build graph at `gn gen` time. A symlink to the `rust-project.json` is located
in the root directory of the Fuchsia tree.

The `rust-project.json` file format is currently unstable. Sometimes this can cause an
unexpected version mismatch where GN produces a `rust-project.json` that `rust-analyzer` is
not expecting, causing `rust-analyzer` to not work correctly.

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

### `rust-analyzer` VSCode extension (supported workflow)

Note:  This is the [`rust-analyzer`][vscode-rust-analyzer] extension, not the `rust` extension that
VSCode may recommend for Rust files.

You can install the `rust-analyzer` extension directly
[from the VSCode marketplace][vscode-rust-analyzer].

Once you have installed the rust-analyzer extension, add the following
configurations to your Workspace `settings.json` file:

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

### Enabling rustfmt in the `rust-analyzer` extension

The `rust-analyzer` extension relies on the `rustup` tool to choose the right toolchain for
invoking rustfmt, so you need to tell `rustup` about your Fuchsia checkout and its toolchain:

```shell
$ rustup toolchain link fuchsia-tools $FUCHSIA_DIR/prebuilt/third_party/rust_tools/<host os>
```

For instance, on linux, you would use `linux-x64`, or on MacOS you would use `mac-x64` (there should
only be one option in your Fuchsia directory, set up correctly for your host development system):

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

### Troubleshooting issues with `rust-analyzer`

If you notice that `rust-analyzer` is not working correctly, it could be due to a breaking
change in the `rust-project.json` file.  The first thing to try in this case is to `jiri update`,
rebase, and re-run `fx gen`, and see if the issue goes away:

```shell
$ jiri update
$ git rebase
$ fx gen
```

#### Downgrading {#downgrading-rust-analyzer}
If not, you may need to [manually downgrade rust-analyzer][vscode-downgrade] to the
[currently-supported version](#supported-rust-analyzer-version) if it's not the latest, or to a
previous version you were using if the currently supported version is listed as "latest".


## Vim

For basic support, instructions on [`rust-lang/rust.vim`](https://github.com/rust-lang/rust.vim).

For IDE support, see the vim section of the [rust-analyzer manual](https://rust-analyzer.github.io/manual.html#vimneovim).

If you use Tagbar, see [this post](https://users.rust-lang.org/t/taglist-like-vim-plugin-for-rust/21924/13)
for instructions on making it work better with Rust.

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

## Intellij (Custom code completion)

See instructions on [the Intellij Rust site](https://intellij-rust.github.io/).
Finally, follow [these steps][cargo-toml-gen] to generate a `Cargo.toml` file for use by Intellij.
Note that cargo-based workflows are more likely to break than rust-analyzer based ones.

[rust-analyzer-latest]: https://github.com/rust-analyzer/rust-analyzer/releases
[vscode-download]: https://code.visualstudio.com/Download
[vscode-update]:  https://vscode-docs.readthedocs.io/en/stable/supporting/howtoupdate/
[vscode-disable-telemetry]: https://code.visualstudio.com/docs/getstarted/telemetry#_disable-telemetry-reporting
[vscode-rust-analyzer]: https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer
[vscode-downgrade]: https://code.visualstudio.com/updates/v1_30#_install-previous-versions
[cargo-toml-gen]: /docs/development/languages/rust/cargo.md#cargo-toml-gen
[cargo-config-gen]: /docs/development/languages/rust/cargo.md#cargo-config-gen
