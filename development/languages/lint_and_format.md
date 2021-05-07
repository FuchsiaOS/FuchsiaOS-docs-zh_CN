# Overview

There are various tools at the disposal of Fuchsia developers for linting and formatting code. This
is a general overview of those tools for each language, as well as a description of how additional
lint checks should be added and the accuracy standards to which they should be held.

Note that this doesn't try to explain the specific language configurations for each linter and
formatter. Though the purpose of linting and formatting is to encourage and enforce recommendations
around style and best practices, each relevant language has its own guides that explain the
decisions made and the configurations enabled.

# Tooling Integration

The Fuchsia team provides two ways to format and lint code: subcommands in the developer-side fx
tool, and integrated Tricium analysis on uploaded changes. In addition, a subset of formatting and
linting is eligible to be directly included in the build, with strict limitations around accuracy.

## Developer tooling (IDEs and fx)

The primary developer tooling suite is the fx command and its subcommands. It provides two
subcommands relevant here: fx format-code and fx lint. Each runs the relevant tooling on a list of
files and prints the output to the terminal’s stdout/stderr. Running fx lint assumes that the
developer has already run fx build; if not, many of the linters will produce errors related to
missing files that are created by the build.

The list of files can be specified in one of three ways:

 - The list of files changed since the second-to-last Git commit, including committed, modified,
 and cached files (this is the default behavior)
 - A list of files passed in a comma-separated list to the --files flag
 - The list of files in the sources of the GN target passed to the --target flag

Formatting is done in-place. Linting is by default warn-only, but users can pass the --fix flag to
fx lint to automatically fix the errors for which the tools provide fixes.

Most editors will also integrate formatters and linters to allow developers to automatically
format-on-save or format-on-keybinding. In most cases, setup (if any) consists of pointing the IDE
at the relevant configuration file and Fuchsia-distributed tool binary.

## Integrated tooling (Tricium)

Tricium is a service that integrates with the Gerrit code review system to surface relevant
warnings in a way that does not block commits. It triggers on each patchset uploaded by a user with
commit access to the Fuchsia repository and runs two suites of tooling analysis.

The formatter analysis does a minimal checkout (no third_party, no prebuilts) and extracts the list
of changed files from the patch commit. It runs the relevant formatter based on file extension on
each file. If the produced formatted file differs from the file content in the uploaded patch,
Tricium posts a comment on the patch explaining how to run the appropriate formatter on the file.

The linter analysis does a full checkout and does a minimal build (to produce the necessary
configuration files and headers). It extracts the list of changed files from the patch commit and
then runs the relevant linter based on file extension. Machine-readable outputs are requested from
the linters, and if warnings are produced the output is then parsed and collected into comment
form. Tricium then comments on the appropriate line with the linter warning.

Tricium, where possible, only runs the tools on the changed lines in a commit, though not all
linters support this behavior. For the ones that do, this is so that existing but irrelevant lint
errors do not distract from the change itself and only directly relevant lints are surfaced.

Analysis results are often based on heuristics. As a result, they do from time to time produce
false positives. Fuchsia aims to support a high bar for these analyzers, with any analyzer with
greater than 10% error rates as measured by the metrics produced by the Tricium service being disabled.

New linters should generally be added to the existing Tricium recipes. Since checkout/build times
are by far the most costly in these builds (the analysis itself takes at best a few seconds, and at
worst a few minutes, while checkouts and/o builds can take much longer), it is more efficient from
both time and infrastructure resource perspectives to simply extend the existing builders. The
selection of which recipe to extend should be based on the amount of information needed, e.g. if
prebuilts/third_party code are not needed to run the analysis, the minimal checkout recipe should
be used.

## Build Integration
An alternative for linter checks that provide zero false-positive rates is to include them in the
build. Currently, the Fuchsia build runs the dartanalyzer in this capacity as a type checker.
Adding additional checks to this category is not encouraged unless it is certain that they do not
fire on false positives.

These checks are directly implemented in the build (generally as actions that run the relevant
script), and so will cause the whole build to fail if they catch errors. They also extend the build
time, and so should only be used in cases where they provide valuable and correct information to
the developer.

# Standards

## Formatters

Formatters should adhere to the relevant style guides, but whether the formatter’s output is the
source-of-truth for the style guide is left up to languages and their style arbiters. When a
formatter is changed in the upstream community (e.g. when the Rust community changes `rustfmt`),
the updated formatter will roll into Fuchsia with the toolchain. This doesn't happen often, but can
be the cause of conflicting formats between Tricium and local tooling until developers update to
use the new toolchain.

Generally, Fuchsia’s support for formatters is dependent on developers running the formatting
commands. The only automation is from Tricium, which will warn if a file differs from the
formatter’s output, but will not block the CL’s commit.

## Linters

Linters should generally provide useful and actionable comments to developers. Since they are often
heuristics-based, they can produce false positives, but any linter exceeding the 10% false positive
rate should be disabled. The process for adding a linter check is to file a bug requesting the new
check, outlining its value and the expected false positive rate. Removing a linter check can either
be done by filing a bug or submitting a patch with the requested configuration change.

Only linters that are guaranteed to not produce false positives should be implemented in the build
itself. These should be enforced by both local builds and by CQ, so that there are no surprises
when developers attempt to submit their code.

# Language Tools

Each supported language provides a formatter and optionally linters. This section describes the
integration of these tools into the Fuchsia workflow. While the formatters tend to be
straightforward, the tooling is a bit complex in how the linters are integrated. In most cases,
developers do not need to understand the internals of `fx` and Tricium.

All commands are assumed to be run from the root of a Fuchsia checkout.

## C/C++

C/C++ code uses [`clang-format`](https://clang.llvm.org/docs/ClangFormat.html) and [`clang-tidy`](https://clang.llvm.org/extra/clang-tidy/). These are distributed as prebuilts from the Clang
toolchain. Both use root-level configuration files (`.clang-format` and `.clang-tidy`,
respectively). Developers should not create additional configuration files at a lower level, as
this will cause disagreements in the tree.

`clang-format` is run on source files as follows:

```sh
prebuilt/third_party/clang/$HOST_PLATFORM/bin/clang-format \
-i \
-style=file \
-fallback-style=Google \
-sort-includes \
$FILES
```

Before you run `clang-tidy`, you must:

* Build the set of generated headers.
  The `clang-tidy` tool partially compiles the source code and most C and C++ code in Fuchsia
  includes headers generated as part of the build.

Once the compilation database and generated headers are present, you can run the `run-clang-tidy.py`
script to start the `clang-tidy` tool. The script handles handles parallelization and deduplication
of errors, which is necessary when the same header is included in multiple source files. When you
use this script, you must also pass the `clang-tidy` and `clang-apply-replacements` binaries from
the distributed Fuchsia toolchain to make sure the correct ones are used.

```sh
export CLANG_TOOLCHAIN_PREFIX=prebuilt/third_party/clang/$HOST_PLATFORM
$CLANG_TOOLCHAIN_PREFIX/share/clang/run-clang-tidy.py \
  -clang-tidy-binary $CLANG_TOOLCHAIN_PREFIX/bin/clang-tidy \
  -clang-apply-replacements-binary $CLANG_TOOLCHAIN_PREFIX/bin/clang-apply-replacements \
  $FILES
```

An optional `-fix` flag can be added to automatically apply fixes. This is available in the
developer-side tooling.

## Rust

Rust code uses [`rustfmt`](https://github.com/rust-lang/rustfmt) and [`clippy`](https://github.com/rust-lang/rust-clippy). These are distributed as prebuilts from the Rust toolchain. The
formatter has a root-level configuration file (`rustfmt.toml`).

`rustfmt` runs on source files as follows:

```sh
prebuilt/third_party/rust_tools/${HOST_PLATFORM}/bin/rustfmt \
--config-path=rustfmt.toml \
--unstable-features \
--skip-children \
$FILES
```

TODO(fxbug.dev/27311): Document clippy once implementation details are finalized.

## Go

Go code uses [`gofmt`](https://golang.org/cmd/gofmt/) and [`go vet`](https://golang.org/cmd/vet/). These are built as part of the Go toolchain build, and also
distributed in the Go host toolchain prebuilts.

`gofmt` runs on source files as follows:

```sh
prebuilt/third_party/go/$HOST_PLATFORM/bin/gofmt -s -w $FILES
```

TODO(fxbug.dev/27310): Document go vet once implementation details are finalized.

## Dart

Dart uses [`dartfmt`](https://github.com/dart-lang/dart_style) and [`dartanalyzer`](https://github.com/dart-lang/sdk/tree/HEAD/pkg/analyzer_cli). These are distributed as prebuilts from the Dart toolchain. The
`dartanalyzer` is run as part of the build rather than as a check, as it performs type-checking and
other assertive checks.

`dartfmt` runs on source files as follows:

```sh
prebuilt/third_party/dart/${HOST_PLATFORM}/bin/dartfmt -w $FILES
```

The `dartanalyzer` is run as part of the build, triggered when the [`dart_library`](/build/dart/dart_library.gni) GN template is
invoked. The [invocation](/build/dart/gen_analyzer_invocation.py) is:

```sh
prebuilt/third_party/dart/${HOST_PLATFORM}/bin/dartanalyzer \
  --packages=$DOT_PACKAGES_FILE \
  --dart-sdk=prebuilt/third_party/dart/${HOST_PLATFORM} \
  --fatal-warnings \
  --fatal-hints \
  --fatal-lints \
  --options=$PACKAGE_ROOT/analysis_options \
  $FILES
  ```

## FIDL

FIDL code uses the `fidl-format` and `fidl-lint` tools. These are built as host tools from in-tree.
Before running either the `zircon/tools` target must be built so that the binaries exist.

`fidl-format` runs on source files as follows:

```sh
$ZIRCON_BUILD_DIR/tools/fidl-format -i $FILES
```

`fidl-lint` runs on source files as follows:

```sh
$ZIRCON_BUILD_DIR/tools/fidl-lint $FILES
```

## GN

GN files use the [`gn format`](https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#cmd_format) subcommand. There is not a linter. This is distributed as part of the GN
prebuilt.

It runs on source files as follows:

```sh
prebuilt/third_party/gn/$HOST_PLATFORM/gn format <files>
```
