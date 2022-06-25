# Build system policies

This document details design principles and specific technical decisions made
that relate to how the Fuchsia build should work.
These principles apply to all modes of using the Fuchsia build, for instance
whether by interactive engineering workflows or via automated systems such as
CI/CQ.

## Goals and priorities of the build

Like any system, the build is often subject to multiple conflicting
requirements. When there is a conflict, we generally seek to satisfy these
priorities by order of importance:

1. Meet customer requirements, as determined by Fuchsia technical leadership.
2. Ensure correctness: produce the desired outputs.
3. Promote maintainability: documentation, sound engineering processes.
4. Improve performance: perform the same builds at a lower cost.

## Desired properties of the build

The following are considered to be good properties for the build:

* Hermeticity - the build is self-contained and neither influences external
  software and configuration or is influenced by external software and
  configuration.
* Repeatability and reproducibility - two builds from the same source tree
  produce the same output or the same outcome deterministically.
  Reproducibility promotes security and auditing, and simplifies
  troubleshooting.
* Efficient - builds should only spend time doing work relevant to the build,
  and must aim to minimize the impact on both human and infrastructure costs.
* Portability - builds should produce consistent results across all supported
  host platforms.

These are ideals.
We aim to meet these ideals and measure our progress against these measures.

## Python scripts as build actions

Python scripts may be used as build actions.

Please follow the [Google style guide for Python][python-style].

Fuchsia currently uses Python 3.8. All Python sources are to begin with the
following:

```shell
#!/usr/bin/env python3.8
```

## Shell scripts as build actions

Shell scripts may be used as build actions.

Shell scripts are encouraged for tasks that can be expressed with a few simple
shell commands. For complex operations, other languages are preferred.

Please follow the [Google style guide for shell scripting][bash-style].
Please use [shellcheck] to find and correct common shell programming errors.

We prefer POSIX (aka Bourne) shell scripts for portability across wide set of
host platforms.
If you're maintaining an existing Bash script, please restrict the features
used to version 3.2, or consider rewriting the script as POSIX shell script.
To check whether your script is POSIX compliant, you can use:

```posix-terminal
shellcheck --shell=sh
```

Scripts that run on POSIX shell should begin with the following:

```shell
#!/bin/sh
```

Scripts that specifically require Bash should begin with the following:

```shell
#!/bin/bash
```

## Migrations

The build system can assist in performing migrations for such things as
compiler features, new tools, or proliferation of various best practices.
A legacy undesired behavior can often be expressed in terms of a dependency
on a `config()` that applies this behavior. The use of a legacy tool or
template to be replaced can be captured by a dependency on a `group()`
target.

### Commit to a plan

Efforts to improve code health are always welcome, but you should have a clear
plan to finish what you started before you begin. A half-done migration that's
run out of momentum could be worse than no migration at all.

### Establish a regression stop

Assume that the codebase doubles every 8 months, and work as early as you can
to prevent new instances of the legacy behavior from being introduced. By
establishing a regression stop, you are "passively" cleaning up the codebase as
governed by its doubling rate, i.e. every doubling period you will have
passively cleaned up half of the codebase.

Ensure that allowlists are guarded by `OWNERS` files, and that POCs for the
migration are listed as owners. Since owners are defined by file, it may be
preferable to subdivide allowlists to different `BUILD.gn` files. For instance,
`config()` targets related to Rust were pulled out to `//build/config/rust` to
better manage the `OWNERS` assignments.

### Document migration / cleanup steps

Publish a clear document explaining the nature of the migration, how to
participate, and how to perform related maintenance work. This allows your
migration effort to scale, and keeps any individual from becoming a roadblock to
ongoing migration efforts such as when they're overwhelmed with support requests
or otherwise unavailable to attend to questions.

Review [C++ implicit conversions][wconversion-project] as a positive example.

### Simplify and automate allowlist maintenance

Allowlists are easy to express as `visibility` lists for GN targets. This opens
the door to automated analysis, and makes changes that violate the allowlist
fail their builds quickly.

When allowlisting targets to use the legacy behavior that you're migrating away
from, make it easy for owners of those targets to make simple refactors such as
renaming individual targets within their directories by allowlisting base
directories, not individual targets.

Document the steps to regenerate and trim any allowlist, such that they can be
conducted by anyone.

See the example below:

```gn
group("foo_allowlist") {
  #  ________  _________  ________  ________
  # |\   ____\|\___   ___\\   __  \|\   __  \
  # \ \  \___|\|___ \  \_\ \  \|\  \ \  \|\  \
  #  \ \_____  \   \ \  \ \ \  \\\  \ \   ____\
  #   \|____|\  \   \ \  \ \ \  \\\  \ \  \___|
  #     ____\_\  \   \ \__\ \ \_______\ \__\
  #    |\_________\   \|__|  \|_______|\|__|
  #    \|_________|
  # This is an allowlist of targets that use the deprecated "foo" tool.
  # As of April 2021 we no longer use "foo". Users should migrate to the new
  # "bar" tool as described in this guide:
  # https://fuchsia.dev/...
  #
  # To regenerate:
  # fx gn refs $(fx get-build-dir) //path/to:foo_allowlist | sed 's|\(.*\):.*|"\1/*",|' | sort | uniq
  #
  # To trim:
  # scripts/gn/trim_visibility.py --target="//path/to:foo_allowlist"
  visibility = [
    "//src/project1/*",
    "//src/project2/*",
    ...
  ]
}
```

Then elsewhere, automatically add a dependency on the allowlisted target.

```gn
# Invoke the legacy foo tool.
# For new usage, please consider using the new bar tool instead!
# See:
# https://fuchsia.dev/...
# ...
template("foo") {
  action(target_name) {
    ...
    deps += [ "//build/foo:foo_allowlist" ]
  }
}
```

### Third party may be out of scope

Fuchsia uses a lot of third party code, that is code that is outside the scope
of the Fuchsia project. As a rule of thumb it's often fine to enter a blanket
allowlist for all third party code for opinionated changes or policy decisions.

```gn
group("bar_allowlist") {
  ...
  visibility = [
    "//third_party/*",
    ...
  ]
}
```

Depending on the nature of your change and the third party code in question,
it may be possible to make changes upstream. Use your best judgement.

[bash-style]: https://google.github.io/styleguide/shellguide.html
[python-style]: https://google.github.io/styleguide/pyguide.html
[shellcheck]: https://www.shellcheck.net/
[wconversion-project]: contribute/open_projects/cpp/wconversion.md
