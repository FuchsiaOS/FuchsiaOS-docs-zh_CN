# Run Clang static analysis

Static analysis is a way of analyzing source code without
executing it. One of its applications is to find
[code smells](https://en.wikipedia.org/wiki/Code_smell) and bugs.

Fuchsia uses Clang as its compiler. Clang has several tools
to analyze the code statically. Fuchsia enables a large set
of useful warning messages and compiles with warnings as errors.

## Prerequisites

Before you do static analysis, make sure you have the following:

* Toolchain: You can either use the prebuilt toolchain or compile compile a toolchain. For
  information on compiling a toolchain, see the [toolchain guide][toolchain].
  Note: This guide assumes that you are using the prebuilt toolchain.
* Compilation database: You need the compilation database to use `clang-tidy` and Clang static
  analyzer. It is created at the root of your build directory automatically by `fx set`.
## Clang tidy

There is a more detailed guide available [here][lint].

## Clang static analyzer

### Prerequisites

Install `scan-build-py`:

```
pip install scan-build --user
```

You might get a warning that `~/.local/bin` is not part of the `PATH`. Either
add it to your `PATH` environment variable or install `scan-build` globally (without the `--user` flag).

### Run

From your Fuchsia directory, run the Clang static analyzer:

```
analyze-build --cdb compile_commands.json --use-analyzer path/to/checkout/prebuilt/third_party/clang/linux-x64/bin/clang --output path/to/output
```

### View the results

View the results of Clang static analyzer with Chrome:

```
chrome path/to/output/scan-build-date-hash/index.html
```

## Resources

* [Clang Tidy](https://clang.llvm.org/extra/clang-tidy/)
* [Clang Static Analyzer](https://clang.llvm.org/docs/ClangStaticAnalyzer.html)

[toolchain]: development/build/toolchain.md
[lint]: development/languages/c-cpp/lint.md
