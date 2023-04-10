# Editors for developing Fuchsia

Fuchsia recommends that you use an IDE (integrated development environment)
to develop Fuchsia and to build software on Fuchsia. An IDE consolidates
multiple tools in a single GUI to help you develop.


## VS Code {#vs-code}

To get started with VS Code, install [VS Code][vs-code-download]{: .external}.

### Configuration

Once you have installed VS Code, you should configure the IDE. Depending on
your development setup, consider the following guides:

* [Configuring remote workspaces][remote-workspaces]: This guide is recommended
  if you are developing on a virtual machine, container, or an environment with
  a running SSH server.
* [Configuring file reloading][file-reloading]: This guide is recommended if
  you are developing Fuchsia in the source tree. As Fuchsia has a large code
  base, you may want to exclude some directories from being watched for file
  changes.

### Extensions

VS Code supports a large amount of extensions which can help you customize
your IDE. Fuchsia has developed several extensions that are specific for
developing the Fuchsia platform and for developing on Fuchsia with the SDK.

* [Fuchsia extension for VS Code][fuchsia-dev-ext]: This extension integrates key
  Fuchsia-specific functionality into VS Code such as connecting, debugging,
  analyzing logs for Fuchsia devices, and functionality to help you edit and
  debug code as you develop for Fuchsia.
* [Additional Fuchsia extensions][fuchsia-source-ext]: This guide lists
  additional Fuchsia extensions that may help you as you contribute to
  Fuchsia.

## Other editors

These guides describe configurations and best practices of other editors and IDE
configurations for Fuchsia development.

### YouCompleteMe integration

[YouCompleteMe](http://ycm-core.github.io/YouCompleteMe/) is a semantic
code-completion engine. YouCompleteMe works natively with Vim but it can also be
integrated with other editors through [ycmd](https://github.com/Valloric/ycmd).

For installation and usage information see [YouCompleteMe integration][youcompleteme-editor]

### Vim configuration

The [`fuchsia.vim`](/scripts/vim/fuchsia.vim) script sets up Vim to do the
following:

*   Set paths so that `:find` and `gf` know how to find files.
*   Enable FIDL syntax highlighting (using `/tools/fidl/editors/vim/`).
*   Integrate basic build system so that `:make` builds and populates the
    QuickFix window.
*   Configure [YouCompleteMe][youcompleteme-editor] (YCM)
    to provide error checking, code completion, and source navigation within the
    Fuchsia tree.

For installation and usage information see [Vim tools for Fuchsia development][vim-editor]

### Sublime Text Configuration

Each language may have extra configuration. See more for

* [Rust][rust-sublime]
* [FIDL][FIDL-sublime]

### Kakoune configuration

For installation and usage information see
[Kakoune for Fuchsia development][#kakoune].

[vs-code-download]: https://code.visualstudio.com/docs/setup/setup-overview
[remote-workspaces]: /docs/reference/tools/editors/vscode/remote-workspaces.md
[file-reloading]: /docs/reference/tools/editors/vscode/file-reloading.md
[sdk-fundamentals]: /docs/get-started/sdk/learn/README.md
[source-fundamentals]: /docs/get-started/learn/README.md
[fuchsia-dev-ext]: /docs/reference/tools/editors/vscode/fuchsia-ext-install.md
[ffx-ref]: https://fuchsia.dev/reference/tools/sdk/ffx
[fuchsia-source-ext]: /docs/reference/tools/editors/vscode/extensions.md
[rust-sublime]: /docs/development/languages/rust/editors.md#sublime-text
[FIDL-sublime]: /docs/development/languages/fidl/guides/editors.md#sublime
[vim-editor]: /docs/reference/tools/editors/vim.md 
[youcompleteme-editor]: /docs/reference/tools/editors/youcompleteme.md
[kakoune]: /docs/reference/tools/editors/kak.md
