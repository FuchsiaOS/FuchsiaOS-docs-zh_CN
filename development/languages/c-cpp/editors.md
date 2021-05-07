# C++ Editor/IDE Setup

This page lists various community-contributed plugins for different editors.
Note that none of these plugins are officially supported, but some users
have reported good experiences.

## CLion

Follow the **Compilation Database** instructions below to create the
appropriate project description file in the fuchsia root directory.

Then in CLion choose *Import Project from Sources* and select the
fuchsia root directory.

### CLion Performance Tweaks

To improve performance you can try some or all of the following. They
are only suggestions, we recommend checking with directly with JetBrains
at <https://intellij-support.jetbrains.com/hc> to be sure what works
best for your environment.

#### Exclude Directories

To speed up indexing time you can exclude directories you are not
working with. You can do that in the Project View by
right-clicking each directory and choosing
*Mark directory as->Excluded*. Note the affected configuration is stored
in `<project>/.idea/misc.xml`

See
[Control Source, Library, and Exclude Directories \- Help \| CLion](https://www.jetbrains.com/help/clion/controlling-source-library-and-exclude-directories.html)
for more information.

#### Unregister Git Repositories

The fuchsia source tree has a fair number of git repositories. Scanning
them can use CPU cycles for CLion. You can unregister the git
repositories you are not working on under
*File -> Settings -> Version Control*. They will still be listed there
so you can add them back later if needed.

#### Tune JVM Options and Platform Properties

See
[Tuning CLion \- Help \| CLion](https://www.jetbrains.com/help/clion/tuning-the-ide.html)
for general tips on tweaking CLion JVM Options and Platform Properties.
As that link suggests, contact CLion support for instructions
regarding the options and values that might help you with whatever issue
you are trying to solve.

### Compilation Database

A [Compilation
Database](https://clang.llvm.org/docs/JSONCompilationDatabase.html) file,
`compile_commands.json`, will be created automatically by `fx` within your
current build directory as well as automatically symlinked to your source root.

Note that this file is only intended to help the IDE find and parse
the source files. Building should still be done with `fx build`.

Note: There is an ongoing issue where CLion shows compiler errors for a few
hundred files in the Fuchsia source code. Other files should work
fine in CLion.

## Vim

See [Helpful Vim tools for Fuchsia development](/scripts/vim/README.md).

## Visual Studio Code (VS Code) {#visual-studio-code}

See more
[VS Code configuration recommendations for Fuchsia contributors](/docs/development/editors/vscode.md).

### clangd

Install
[vscode-clangd](https://marketplace.visualstudio.com/items?itemName=llvm-vs-code-extensions.vscode-clangd).
Disable the default C/C++ extension if you have it installed.

In settings, add:

```
"clangd.path": "<absolute path to fuchsia root directory>/prebuilt/third_party/clang/<platform>/bin/clangd",
```

Note: The path to clangd does need to be absolute.

Finally, follow the **Compilation Database** instructions below to
generate the `compile_commands.json` in the fuchsia root directory. Then
reload VS Code to enjoy the results.

You may also benefit from enabling background indexing and clang-tidy using the following settings:

```
"clangd.arguments": [
    "--clang-tidy",
    "--background-index"
]
```

Further details on clangd setup can be found [here](https://clang.llvm.org/extra/clangd/Installation.html).
