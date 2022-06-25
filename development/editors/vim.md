# Helpful Vim tools for Fuchsia development

## Features

The [`fuchsia.vim`](/scripts/vim/fuchsia.vim) script sets up Vim to do the
following:

*   Set paths so that `:find` and `gf` know how to find files.
*   Enable FIDL syntax highlighting (using `/tools/fidl/editors/vim/`).
*   Integrate basic build system so that `:make` builds and populates the
    QuickFix window.
*   Configure [YouCompleteMe](/docs/development/editors/youcompleteme.md) (YCM)
    to provide error checking, code completion, and source navigation within the
    Fuchsia tree.

## Installation

The steps are:

1.  [Update your login script](#update-your-login-script).
1.  [Run the fx set command](#run-the-fx-set-command).
1.  [Update your Vim startup file](#update-your-vim-startup-file).
1.  [Restart Vim to configure YouCompleteMe](#restart-vim-to-configure-ycm).
1.  [Build a compilation database](#build-a-compilation-database).

### Update your login script {#update-your-login-script}

Add the following line to your startup script (typically `~/.bashrc`):

```shell
export FUCHSIA_DIR=<your_fuchsia_source_directory>
```

### Run the fx set command {#run-the-fx-set-command}

This command uses the format `fx set [PRODUCT].[BOARD]`. For example:

```shell
fx set core.x64
```

### Update your Vim startup file {#update-your-vim-startup-file}

If the following line exists in your `~/.vimrc` file, remove it:

```shell
filetype plugin indent on
```

Then add the following lines to your `~/.vimrc`:

```shell
if $FUCHSIA_DIR != ""
  source $FUCHSIA_DIR/scripts/vim/fuchsia.vim
endif
filetype plugin indent on
```

### Restart Vim to configure YouCompleteMe {#restart-vim-to-configure-ycm}

Note: If you haven't installed YouCompleteMe, see
[this installation guide](https://github.com/ycm-core/YouCompleteMe#installation){:.external}
to install YCM on your workstation.

To configure YouCompleteMe (YCM), you need to source the
[`fuchsia.vim`](/scripts/vim/fuchsia.vim) file.

Restart your Vim to run the `source $FUCHSIA_DIR/scripts/vim/fuchsia.vim`
command in your `~/.vimrc` file (see
[Update your Vim startup file](#update-yout-vim-startup-file)).

To verify that your YCM works, place the cursor on an identifier in a `.cc` or
`.h` file in Vim, then hit `Ctrl+]` to navigate to the definition of the
identifier.

## Auto-formatting

Google's [vim-codefmt](https://github.com/google/vim-codefmt) can auto-format
code on save. This can be installed using a package manager such as
[Vundle](https://github.com/VundleVim/Vundle.vim) or
[vim-plug](https://github.com/junegunn/vim-plug).

A plugin to format and syntax-highlight GN files is available
[separately](https://gn.googlesource.com/gn/+/refs/heads/master/misc/vim). The
following example `.vimrc` demonstrates how to turn on auto-formatting of GN
files using vim-plug and Fuchsia's prebuilt GN:

```
call plug#begin('~/.vim/plugged')
Plug 'google/vim-maktaba'
Plug 'google/vim-glaive'
Plug 'google/vim-codefmt'
Plug 'https://gn.googlesource.com/gn', { 'rtp': 'misc/vim' }
call plug#end()
call glaive#Install()

" Set gn path to the Fuchsia prebuilt.
let g:gn_path = systemlist('source ' . g:fuchsia_dir . '/tools/devshell/lib/vars.sh && echo $PREBUILT_GN')[0]
execute ':Glaive codefmt gn_executable="' . g:gn_path . '"'

augroup autoformat_gn
  autocmd!
  autocmd FileType gn AutoFormatBuffer gn
augroup END
```

For highlighting Rust, see its [language-specific
guide](/docs/development/languages/rust/editors.md#vim).
