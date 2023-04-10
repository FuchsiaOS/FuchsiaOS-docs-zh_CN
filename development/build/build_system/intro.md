# Introduction to GN

This is an introduction to GN's terms and way of thinking. This should be
sufficient background to get your bearings in GN and how it's used in Fuchsia.
GN (and the Fuchsia build) are more complicated than the below will discuss, but
the average developer will not need to understand most of it on a deeper level.

The GN documentation pages [QuickStart] and [Language] give more detailed
background on GN, and [Reference] has the full language documentation.  Use
the `gn help` command to print out the reference interactively for individual
topics.  [Ninja] has its own documentation as well.

In the Fuchsia checkout after running `jiri update`, the commands
`fx gn` and `fx ninja` provide access to the prebuilt binaries.

[Ninja]: https://ninja-build.org/manual.html
[QuickStart]: https://gn.googlesource.com/gn/+/HEAD/docs/quick_start.md
[Language]: https://gn.googlesource.com/gn/+/HEAD/docs/language.md
[Reference]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md

## Two-phase operation: `gn` and `ninja`

Unlike `make`, `gn` is only ever half the story.  It's in the name: GN stands
for Generate [Ninja].  There's a division of responsibilities between the
tools that corresponds to a separation of running the build into two steps:

1. `gn gen` takes all the configuration choices and makes all the decisions.
   All it really does is generate the `.ninja` files in the build directory.
   This step only has to be done by hand when you change the configuration or
   completely nuke the build directory.  In general, it only needs to be done
   when the GN files change, and in incremental builds it happens
   automatically if the GN files or configuration change.

1. `ninja` runs the commands to compile and link, etc.  It handles incremental
   builds and parallelism.  This is the step you do every time you've changed
   a source file, like running `make`.  GN automatically emits rules to
   re-generate the Ninja files by running `gn gen` again when a relevant
   `BUILD.gn` file (or some other relevant files) has changed, so for most
   changes after the first time you've built, `ninja` does it all.

Ninja is very simple compared to something like GNU `make`.  It just compares
times and runs commands and its input files are written by machines, not
humans.  However, it builds in some useful things that we bend over backward
to accomplish in `make`:

 - Rebuild each file when the command line changes.  Command lines will only
   really change when GN runs again.  But after that, Ninja is smart about
   incremental builds re-doing commands for files that have changed and not
   re-running commands that haven't changed.
 - Handle compiler-generated dependency files.  Ninja knows about the makefile
   subset that compilers emit in `.d` files and consumes them directly when
   directed to by GN.
 - Run with `-j$(getconf _NPROCESSORS_ONLN)` by default.  You can pass `-j1`
   to serialize or `-j1024` when using Goma, but out of the box it does the
   parallelism you usually want.
 - Prevent interleaved `stdout`/`stderr` output from parallel jobs.  Ninja
   buffers the output so that error messages don't get garbled by spew from
   multiple processes.
 - Support terse/verbose command output.  By default, Ninja emits short
   `Kbuild`-style messages for each command it runs, in a wordy-progress-meter
   style.  The -v switch is like V=1 in `Kbuild`, to show each actual command.

GN was developed as part of the Chromium project to replace older build
systems.  Fuchsia inherited it from them, and it is now used across the tree as
the primary build system.

## Build directories and `args.gn`

Ninja always runs in the build directory.  All commands Ninja runs are run from
the root of the build directory.  The common thing is `ninja -C build-dir`.

Neither GN nor Ninja cares what build directory you use.  It's common practice
to use a subdirectory of the source directory, and since file paths are
usually rebased to be relative to the build directory, the file names given to
the compiler will have a whole lot of `../` in them if you put your build
directory elsewhere; but it should work.  It's long been common practice in
Chromium (predating GN itself) to use `out/_something_` in the source
directory, and Fuchsia inherited that default.  But nothing cares what build
directory names you choose, though the `out` subdirectory is in the top-level
`.gitignore` file for Fuchsia.

The basic command is `gn gen build-dir`.  This creates `build-dir/` if needed,
and populates it with Ninja files for the current configuration.  If
`build-dir/args.gn` exists, then `gn gen` will read that file to set GN build
arguments (see below).  `args.gn` is a file in GN syntax that can assign values
to GN build arguments that override any hardcoded defaults.  This means just
repeating `gn gen build-dir` preserves what you did last time.

You can also add `--args=...` to gn gen or use the `gn args` command to
configure your build arguments.  The `gn args` command gives you a way to run
your $EDITOR on the `args.gn` file, and upon exiting the editor the command
will re-run `gn gen` for you with the new arguments.  You can also just edit
`args.gn` any time, and the next Ninja run will re-generate the build files.

Args can also be set using the `fx set` command, which invokes `gn gen`. For
example to set `foxtrot` to ' `true` via `fx set`:

```sh
$ fx set <your configuration> --args 'foxtrot=true'
```

See [GN Build Arguments](/docs/gen/build_arguments.md), for details.

## GN syntax and formatting

GN syntax is whitespace-insensitive. `x=1 y=2` is the same as:

```gn
x = 1
y = 2
```

However, there is *one true indentation and formatting style* for GN code.  The
`gn format` command reformats syntactically valid GN code into the canonical
style.  There is editor syntax support for Emacs and Vim.  Canonical formatting
will be enforced by Tricium and mass reformatting will be done.  If you don't
like the formatting, file bugs or make a change in upstream GN and if it lands
we'll mass reformat everyone to conform to the new one truth.

## Source paths and GN labels

GN uses POSIX-style paths (always in represented as strings) both for files and
to refer to GN-defined entities.  Paths can be relative, which means relative
to the directory containing the `BUILD.gn` file where the path string appears.
They can also be "source-absolute", meaning relative to the root of the source
tree.  Source-absolute paths begin with `//` in GN.

When source paths are eventually used in commands, they are translated into
OS-appropriate paths that are either absolute or relative to the build
directory (where commands run).

Predefined variables are used in source path contexts to locate parts of the
build directory:

 - `$root_build_dir` is the build directory itself
 - `$root_out_dir` is the subdirectory for the current toolchain (see below)
   - This is where all "top-level" targets go.  In many GN builds, all
     executables and libraries go here.
 - `$target_out_dir` is the subdirectory of `$root_out_dir` for files built by
   targets in the current `BUILD.gn` file.  This is where the object files go.
 - `$target_gen_dir` is a corresponding place recommended to put generated code
 - `$root_gen_dir` is a place for generated code needed outside this
   subdirectory

GN labels are how we refer to things defined in a `BUILD.gn` file.  They are
based on source paths, and always appear inside GN strings.  The full syntax of
a GN label is `"dir:name"` where the `dir` part is a source path that names the
particular `BUILD.gn` file.  The `name` refers to a target defined in that file
with `target_type("name") { ... }`.  As a shorthand, you can define a target
with the same name as its directory.  The label `"//path/to/dir"` with no `:`
part is a shorthand for `"//path/to/dir:dir"`.  This is the most common case.

## Dependency graph and `BUILD.gn` files

Everything in GN is rooted in the dependency graph.  There is one root
`BUILD.gn` file.  The only way other `BUILD.gn` files are even read is if there
is a dependency on a label in that directory.

There are no wildcards.  Every target must be named as a dependency of some
other target to get built.  You can give individual targets on the `ninja`
command line to explicitly get them built.  Otherwise they must be in the graph
from the `//:default` target (named `default` in the root `BUILD.gn` file).

There is a generic meta-target type called `group()` that doesn't correspond to
a file produced by the build but is rather a way to structure your dependency
graph nicely.  Top-level targets like `default` are usually groups.  You can
have a group for all the drivers for a piece of hardware, a group for all the
binaries in a use case, etc.

When some code uses something at runtime (a data file, another executable,
etc.)  but doesn't use it as a direct input at build time, that file belongs in
the `data_deps` list of target that uses it.  That will also be enough to get
the thing into the BOOTFS image at its appointed place.

Targets can also be labeled with `testonly = true` to indicate that the target
contains tests. GN prevents targets that are not `testonly` from depending on
targets that are, allowing for some level of control over where test binaries
end up.

Building image files is driven from one or more `zbi()` targets.  This will
make a ZBI by building and using the ZBI host tool. Targets can be placed in
this image by existing within its dependency graph, and so you can give it
dependencies on the kernel and any drivers or executables you want in the
image.

Note that getting targets defined in Ninja files is at the granularity of
`BUILD.gn` files, though the dependency graph from default or any other target
is at the granularity of an individual target.  So having some target in the
`BUILD.gn` file in the graph from default makes all targets in that file (and
toolchain, see below) available as targets on the Ninja command line even
though they are not built by default.

## More Advanced Concepts

### GN expression language and GN scopes

GN is a simple, dynamically-typed, imperative language whose sole purpose at
the end of the day is to produce declarative Ninja rules.  Everything revolves
around scopes, which is both the lexical binding construct of the language and
a data type.

GN values can take any of several types:

 - Boolean, either `true` or `false`
 - Integer, signed with normal decimal syntax; not used much
 - String, always in "double-quotes" (note below about `$` expansion)
 - Scope, in curly braces:  `{ ... }`; see below.
 - List of values, in square brackets: `[ 1, true, "foo", { x=1 y=2 } ]` is a
   list of four elements.

Values are dynamically-typed and there is no kind of implicit type coercion,
but there is never type-checking as such.  Values of different types never
compare as equal, but it's not an error to compare them.

String literals expand simple `$var` or `${var}` expressions inside the
double-quotes.  This is an immediate expansion: `x${var}y` is the same as `x +
var + y` when var is a string.  In this way, any value can be rendered as a
pretty-printed string.

Identifiers made up of alphanumerics and underscores can populate a scope via
assignment operators.  Imperative assignment with `=` and modification via `+=`
are really all the GN language does (there are also some special ways to have
side effects like `print()`, used for debugging; and `write_file()`, used
sparingly).

Each file is internally represented as a scope, and there is no global scope.
Shared "globals" can be defined in a `.gni` file and imported where they are
used (`import("//path/to/something.gni")`).  Each `.gni file is processed once
per toolchain (see below for information about toolchains), and the resulting
scope is copied into the importing file scope.

Target declarations introduce a sub-scope:

```gn
foo = true
executable("target") {
  foo = 12
}
# Outside the target, foo == true
```

GN is very strict in diagnosing errors when a variable is defined but never
used within a scope.  The scope inside a target acts like a keyword argument
list for the target with checking that the argument names were spelled
correctly.  The target-defining code can also use `assert()` to diagnose an
error if a required argument was omitted.

A value can also be a scope.  Then it's acting like a struct when you use it:
`value.member`.  But a scope is always a block of GN code that executes to
yield its set of names and values:

```gn
foo = {
  x = global_tuning + 42
  if (some_global && other_thing == "foobar") {
    y = 2
  }
}
```

This always defines `foo.x` but only sometimes defines `foo.y`.

### GN toolchains

GN has a concept called a "toolchain".  This will all be happening behind the
scenes and developers shouldn't need to deal with it directly, but it helps
to understand the mechanism.

This is what encapsulates the compilers and default compilation switches.  It's
also the only real way to get the same things compiled twice in different
ways. In Fuchsia there will be several toolchains:

 - Host
 - Vanilla userland (compiled with default `-fPIE`)
 - Shared libraries in userland (compiled with `-fPIC`)
 - `userboot`
 - Kernel
 - Kernel physical-address mode for ARM64 (compiled with `-mstrict-align`)
 - Multiboot for x86 (compiled with `-m32`)
 - UEFI for Gigaboot
 - Toolchains are also used in the ["variants"
   scheme](/docs/gen/build_arguments.md#known_variants) that is how we allow selectively
   enabling ASan or the like for parts of userland.

Each toolchain is identified by a GN label.  The full syntax for target labels
is actually `//path/to/dir:name(//path/to/toolchain/label)`.  Usually the
toolchain is omitted and this is expanded to `label($current_toolchain)`,
i.e. label references are usually within the same toolchain.

All the GN files are instantiated separately in each toolchain.  Each toolchain
can set global variables differently, so GN code can use tests like `if
(is_kernel)` or `if (current_toolchain == some_toolchain)` to behave
differently in different contexts.  This way the GN code stays with the source
it describes, but it can still do different subsets of shared sources for
kernel and user, etc.
