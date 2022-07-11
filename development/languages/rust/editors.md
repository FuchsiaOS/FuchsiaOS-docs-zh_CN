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

To setup `rust-analyzer` on VS Code, see the VS Code guide [Installing extensions][vscode-extension-guide].

## Vim

Install [`rust-lang/rust.vim`](https://github.com/rust-lang/rust.vim), which also integrates with:

*   [Tagbar](https://github.com/preservim/tagbar): universal-ctags is strongly recommended, but
    other ctags implementations are also supported.

*   [Syntastic](https://github.com/vim-syntastic/syntastic): `rust.vim` automatically registers
    `cargo` as a syntax checker for Rust. Yet it will fail unless you [set up cargo][cargo-setup].
    If you want to disable the integration (or use `rust-analyzer` to check the code, e.g. via
    YouCompleteMe), add the following to your `~/.vimrc`:

    ```
    let g:syntastic_rust_checkers = []
    ```

*   Auto formatting: to run `rustfmt` on save (disabled by default), add the following to `~./vimrc`:

    ```
    let g:rustfmt_command = '{{ "<var>" }}FUCHSIA_DIR{{ "</var>" }}/prebuilt/third_party/rust/{{ "<var>" }}HOST_OS{{ "</var>" }}/bin/rustfmt'
    let g:rustfmt_autosave = 1
    ```

For IDE support, see the vim section of the [rust-analyzer
manual](https://rust-analyzer.github.io/manual.html#vimneovim). For YouCompleteMe, add
`--rust-completer` (or `--all`) if you are installing from source, and it should work out of the
box. You can also specify the path to a standalone `rust-analyzer` in your `~/.vimrc` (this is not
guaranteed to work due to version compatibility); here is an example assuming it is installed to
`~/.local/bin/rust-analyzer` (note the intentional absence of `/bin` in the path):

```
let g:ycm_rust_toolchain_root = $HOME . '/.local'
```

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
using are Fuchsia versions of those.  If you haven't already,
[install rustup](https://rustup.rs/). Then, from your Fuchsia root, type:

```posix-terminal
rustup toolchain link fuchsia {{ '<var>' }}FUCHSIA_DIR{{ '</var>' }}/prebuilt/third_party/rust/{{ '<var>' }}HOST_OS{{ '</var>' }}
rustup default fuchsia
```

Finally, follow the steps to [generate a `Cargo.toml`][cargo-toml-gen] for the GN target that you
want to work on.

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

Follow the [instructions][cargo-setup] to generate `Cargo.toml` and `.cargo/config`, which will also
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
[vscode-extension-guide]: /development/editors/vscode/extensions.md#rust-analyzer
[vscode-download]: https://code.visualstudio.com/Download
[vscode-update]:  https://vscode-docs.readthedocs.io/en/stable/supporting/howtoupdate/
[vscode-disable-telemetry]: https://code.visualstudio.com/docs/getstarted/telemetry#_disable-telemetry-reporting
[vscode-rust-analyzer]: https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer
[vscode-downgrade]: https://code.visualstudio.com/updates/v1_30#_install-previous-versions
[cargo-setup]: /development/languages/rust/cargo.md
[cargo-toml-gen]: /development/languages/rust/cargo.md#cargo-toml-gen