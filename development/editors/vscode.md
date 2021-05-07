# VS Code Configuration

## General configurations

### Remote workspaces

You may want to configure [remote workspaces](https://code.visualstudio.com/docs/remote/ssh#_connect-to-a-remote-host).

### Speed up automatic file reloading
VS Code watches external file changes. It automatically reloads the lastest stored file if it does not have a working copy that conflicts. Watching and detecting changes, however, can take long time. The larger the code base, the longer it takes to detect the file change. Excluding some directories from the search space improves the speed.

Follow the menu Code -> Preferences -> Text Editor -> File -> Add Pattern
Add a directory pattern you want to exclude the search from. Alternatively one can directly modify `settings.json` and add exclude pattern similar to below

```
    "files.watcherExclude": {
        "**/.DS_Store": true,
        "**/.cipd": true,
        "**/.clang-format": true,
        "**/.clang-tidy": true,
        "**/.dir-locals.el": true,
        "**/.git": true,
        "**/.gitattributes": true,
        "**/.gitignore": true,
        "**/.hg": true,
        "**/.idea": true,
        "**/.jiri_manifest": true,
        "**/.jiri_root": true,
        "**/.ssh": true,
        "**/.style.yapf": true,
        "**/.svn": true,
        "**/AUTHORS": true,
        "**/CMakeLists.txt": true,
        "**/CODE_OF_CONDUCT.md": true,
        "**/CONTRIBUTING.md": true,
        "**/CVS": true,
        "**/LICENSE": true,
        "**/PATENTS": true,
        "**/buildtools": true,
        "**/examples": true,
        "**/garnet/test_data": true,
        "**/garnet/third_party": true,
        "**/out": true,
        "**/prebuilt": true,
        "**/rustfmt.toml": true,
        "**/src/chromium": true,
        "**/topaz": true,
        "**/zircon/experimental": true,
        "**/zircon/prebuilt": true,
        "**/zircon/third_party": true,
    },
```

### Useful Fuchsia-specific extensions

- [FIDL Language Support](https://marketplace.visualstudio.com/items?itemName=fuchsia-authors.language-fidl){:.external}
  Provides syntax support and LSP-based language features
  [FIDL](/docs/development/languages/fidl/README.md).
- [Fuchsia.git Helper](https://marketplace.visualstudio.com/items?itemName=jwing.fuchsia-git-helper){:.external}
  Adds an "Open in..." option to the editor context menus.
- [FuchsiAware](https://marketplace.visualstudio.com/items?itemName=RichKadel.fuchsiaware){:.external}
  Assists with browsing Fuchsia artifacts, such as by linking from component URLs to component
  manifests.

## Language specifics
Each language may require extra configuration. See more for

* [Rust](/docs/development/languages/rust/editors.md#visual-studio-code)
* [Dart](/docs/development/languages/dart/ides.md#visual-studio-code)
* [C/C++](/docs/development/languages/c-cpp/editors.md#visual-studio-code)
* [FIDL](/docs/development/languages/fidl/guides/editors.md#visual-studio-code)