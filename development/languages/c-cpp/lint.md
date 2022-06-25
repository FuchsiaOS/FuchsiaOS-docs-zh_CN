# Lint

We use clang-tidy to lint C++ code and aim to keep the repository warning-clean.
The linter is configured in the root level `.clang-tidy` file. Developers
should not create additional configuration files at a lower level, as
this will cause disagreements in the tree.

## How to lint

`fx lint` is a Fuchsia script that wraps language-specific linters in a common
command line interface. It gathers a list of files, based on the options you
specify, separates them by matching linter, and executes each required linter.
`clang-tidy` is used for C and C++ files.

Without any other arguments, `fx lint` lints the files in your
most recent git commit, and passes them through the linter:

```
fx lint
```

To restrict linting to C++, add a double-dash (--) followed by the
file pattern(s) to match, such as:

```
fx lint -- '*.cc' '*.cpp'
```

To run a specific GN target through the linter, use:

```
fx lint --target=<target>
```

In order to lint all files under the current working directory, add `--all`.
Running `fx lint --all` from the top-level `fuchsia` directory is generally not
recommended, and will likely take several hours to complete. Be certain you
`cd` to the best top level directory for your analysis requirements. For example:

```
(cd <your/subdir>; fx lint --all -- '*.cc')
```

You can also add `--fix` in order to automatically generate fixes for some (but
not all) of the warnings.

Additional options and examples are documented in the tool itself. For the most up
to date documentation on `fx lint`, including examples, run:

```
fx lint --help
```

## Suppressing warnings

Any warning can be suppressed by adding a `// NOLINT(<check_name>)` or a
`// NOLINTNEXTLINE(<check_name>)` comment to the offending line. It is also
possible to disable the check entirely within the repository by editing the
root level `.clang-tidy` file.

## Checks

There are a number of check categories enabled, and specific checks within them
have been disabled for the reasons below. The list of enabled check categories
is as follows:

 - `bugprone-*`
 - `clang-diagnostic-*`
 - `google-*`
 - `misc-*`
 - `modernize-`
 - `performance-*`
 - `readability-*`

This list tracks the reasons for which we disabled in particular [checks]:

 - `clang-diagnostic-unused-command-line-argument` - ninja-generated compilation
    database contains the linker argument, which ends up unused and triggers
    this warning for every file
 - `misc-noexcept*` - Fuchsia doesn't use C++ exceptions
 - `misc-non-private-member-variables-in-classes` - We don't allow classes/structs
   with a mix of private and public members, but all public is fine.
 - `modernize-deprecated-headers` - Fuchsia uses old-style C headers
 - `modernize-use-nodiscard` - Not generally used in the Fuchsia codebase
 - `modernize-raw-string-literal` - the check was suggesting to convert `\xFF`
    literals, which we'd rather keep in the escaped form.
 - `modernize-return-braced-init-list` - concerns about readability of returning
    braced initialization list for constructor arguments, prefer to use a
    constructor explicitly
 - `modernize-use-emplace` - enabled the IgnoreImplicitConstructors option to
   comply with [Abseil Tip of the Week #112](https://abseil.io/tips/112).
 - `modernize-use-equals-delete` - flagging all gtest TEST_F
 - `modernize-use-trailing-return-type` - Fuchsia C++ code typically uses the
   `int foo()` style of defining functions, and not the `auto foo() -> int`
   style as recommended by this check.
 - `readability-implicit-bool-conversion` - Fuchsia C++ code commonly uses implicit
   bool cast of pointers and numbers
 - `readability-isolate-declaration` - Zircon code commonly uses paired declarations.
 - `readability-uppercase-literal-suffix` - Fuchsia C++ code chooses not to impose
   a style on this.

# Static analysis

Strictly speaking it is not linting, but the Clang static analyzer can do
deep analysis to find bugs. See [Static analysis][static_analysis] for details.

[static_analysis]: /docs/development/build/static_analysis.md
[checks]: https://clang.llvm.org/extra/clang-tidy/checks/list.html
