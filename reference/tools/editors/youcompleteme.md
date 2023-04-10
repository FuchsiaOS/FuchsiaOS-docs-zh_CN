# YouCompleteMe integration

[YouCompleteMe](http://ycm-core.github.io/YouCompleteMe/) is a semantic
code-completion engine. YouCompleteMe works natively with Vim but it can also be
integrated with other editors through [ycmd](https://github.com/Valloric/ycmd).

## Install YouCompleteMe in your editor

{% dynamic if user.is_googler %}

### [Googlers only]  gLinux

(This applies to anyone compiling on gLinux, even if editing over SSHFS on
MacOS) See the Google intranet specific instructions [go/ycm](http://go/ycm).

You'll also need to setup
`${FUCHSIA_DIR}/scripts/youcompleteme/default_settings.json` as the default
settings path in your editor, in order to disable the internal `use_clangd`
flag. If you want to use clangd, you can additionally edit that file to set
`use_clangd` to 1, and `clang_binary_path` to
`${FUCHSIA_DIR}/prebuilt/third_party/clang/${HOST_PLATFORM}/bin/clangd`.

{% dynamic else %}

See the [installation
guide](https://github.com/Valloric/YouCompleteMe#installation).

Note: Installing YCM on MacOS with Homebrew is not recommended because of
library compatibility errors. Use the official installation guide instead.

{% dynamic endif %}

## Generate compilation database

YouCompleteMe (and other tools like clang-tidy) require a [JSON compilation
database](https://clang.llvm.org/docs/JSONCompilationDatabase.html) that
specifies how each file is compiled. `fx` will automatically symlink the database,
`compile_commands.json`, from your current build directory to your source root.

If this database is not present, then Vim can be configured to fall back to the configuration
in [/scripts/youcompleteme/ycm_extra_conf.py](/scripts/youcompleteme/ycm_extra_conf.py). See
[Vim configuration](vim.md) for how to set this up.

## Use it

YouCompleteMe will use `compile_commands.json` to do code completion and find
symbol definitions/declarations. See your editor's YouCompleteMe docs for
details. The editor should pick up `compile_commands.json` file automatically.

See [Vim setup](vim.md) for instructions on configuring Vim for Fuchsia development.

## Other editors (ycmd)

You'll need to set the ycmd config option `global_ycm_extra_conf` to point to
`${FUCHSIA_DIR}/scripts/youcompleteme/ycm_extra_conf.py`.
Note you may need to manually replace `${FUCHSIA_DIR}` with the correct path.

Alternatively, you can create a `.ycm_extra_conf.py` symbolic link to let YCM
automatically find the config for any fuchsia repository:

```posix-terminal
ln -s $FUCHSIA_DIR/scripts/youcompleteme/ycm_extra_conf.py $FUCHSIA_DIR/.ycm_extra_conf.py
```

