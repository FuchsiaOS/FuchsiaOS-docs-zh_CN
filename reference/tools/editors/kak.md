# Kakoune for Fuchsia development

[Kakoune]{: .external} is a modal text editor inspired by vim. See
[why-kakoune]{: .external} for why you might want to use it over vim or any
other editor or IDE.

## Line numbers

Add the following to `kakrc` to enable relative line numbers (note that this is
equivalent to vim when both `number` and `relativenumber` are set, meaning that
the absolute line number is shown for the line the primary cursor is on).

```none
add-highlighter global/ number-lines -relative
```

Remove `-relative` for absolute line numbers.

## Plugin manager

[plug.kak]{: .external} is a plugin manager inspired by `vim-plug`. It is the
recommended way to install and manage plugins. See the Github README for
installation and usage instructions.

The basic workflow to add a plugin is:

1. Add configuration to `kakrc` which names and configures the plugin you want
   to install.
1. Restart Kakoune in order to reload `kakrc`.
1. Run `:plug-install` to install the plugin.

## Indentation and alignment

A recommended setup for indentation and alignment is to use a combination of
built-in support for [editorconfig]{: .external} and the
[smarttab.kak]{: .external} plugin. Naturally, this relies on a `.editorconfig`
configuration file being present at or above the root of the Fuchsia
checkout. Add the following lines to `kakrc` and then install the
`smarttab.kak` plugin:

```none
plug "andreyorst/smarttab.kak" defer smarttab %{
    set-option global softtabstop 4
}

hook global BufOpenFile .* %{
    editorconfig-load
    autoconfigtab
}
hook global BufNewFile .* %{
    editorconfig-load
    autoconfigtab
}
```

Note that backspace always removes one indentation worth of spaces in
indentation context, and the value of `softtabstop` only affects how many
spaces are deleted in alignment context. A value of 4 is the recommended
default; other alternatives are:

- `%opt(indentwidth)` to use the value of `indentwidth`, or
- a very large number such that all alignment spaces are deleted with a single
  input of backspace.

## LSP client

[kak-lsp]{: .external} is a LSP (Language Server Protocol) client written
in Rust. See the Github README for installation instructions.

The default [kak-lsp.toml]{: .external} runs `clangd` for C and C++, `gopls`
for Go, and `rustup which rust-analyzer` for Rust. Edit the TOML config file
to make any necessary changes.

At minimum, the following configuration line should be added to enter
LSP mode through which all LSP functionality can be accessed.

```none
map global user l %{:enter-user-mode lsp<ret>} -docstring "LSP mode"
```

## Filetype-specific configuration

### FIDL

The first release version to include FIDL support is v2022.08.11. Run
`kak -version` to verify that you are running this or a more recent version. If
you are running an older version, the alternatives are:

1. Compile from source; or
1. Save a copy of [fidl.kak]{: .external} anywhere and source it from `kakrc`.
   For example, if it is saved next to `kakrc`, it can be sourced by:

   ```none
   source "%val(config)/fidl.kak"
   ```

   This works because `%val(config)` expands to the path where `kakrc` can be
   found.

FIDL filetype support includes syntax highlighting and indentation. The only
indentation behavior present by the auto-formatter that is not implemented
is alignment of colons for table and union fields.

[Kakoune]: https://kakoune.org/
[why-kakoune]: https://kakoune.org/why-kakoune/why-kakoune.html
[editorconfig]: https://github.com/mawww/kakoune/blob/master/rc/detection/editorconfig.kak
[smarttab.kak]: https://github.com/andreyorst/smarttab.kak
[plug.kak]: https://github.com/andreyorst/plug.kak
[kak-lsp]: https://github.com/kak-lsp/kak-lsp
[kak-lsp.toml]: https://github.com/kak-lsp/kak-lsp/blob/2a981ad1b02bb4d8a8fe992586f76263c0b133d3/kak-lsp.toml
[fidl.kak]: https://github.com/mawww/kakoune/blob/f3cb2e434004a718d1225cb0d74c694e66a7248b/rc/filetype/fidl.kak
