# Layout of Product and Package Directories

Each [layer](/development/source_code/layers.md) of the Fuchsia source tree
contains a top-level directory called `packages` containing all the
[build packages](packages.md) for that layer. The present document describes
the packages that are common to all of the layers. Each layer will introduce
additional packages specific to that layer.

## Directory map

In the diagram below, "pkg" refers to Fuchsia packages, the unit of installation
in Fuchsia.

```
//<layer>/products
    default          # default build configuration for this layer
                     # by convention, default preinstalls development tools,
                     # and makes all prod packages available.
//<layer>/packages
    <layer>          # all production pkg up to this layer
    buildbot         # all pkg declared at this layer; used by CQ/CI
    default          # monolith packages for daily development at this layer
    preinstall       # devtools for daily development at this layer
    kitchen_sink     # all pkg up to this layer
    all              # grab bag of every pkg in this layer
    prod/            # pkg that can be picked up in production
    tests/           # correctness tests (target & host)
    tools/           # dev tools not for prod (target & host)
    benchmarks/      # performance tests
    examples/        # pkg demonstrating features offered by this layer
  * experimental/    # pkg not quite ready for prod
  * config/          # config files for the system (e.g. what to boot into)
    sdk/             # SDK definitions
    ...              # each layer will also define additional packages
```

## Cross-layer dependencies

- `<layer>(N)` depends on `<layer>(N-1)` and adds all the production artifacts
  of (N)
  - this defines a pure production build
- `buildbot(N)` depends on `<layer>(N-1)` and adds all artifacts of (N)
  - this defines a build suitable for verifying the integrity of (N)
- `kitchen_sink(N)` depends on `kitchen_sink(N-1)` and adds all artifacts of (N)
  - this defines a build suitable for developing (N) as well as its dependencies

## Inner-layer dependencies

Most directories in a `packages` directory contain a special `all` package which
aggregates all packages in this directory. Every `all` package should roll up to
the root `all` package, thereby creating a convenient shortcut to build "all
packages in the layer".
Note that the directories that do not require aggregation are marked with `*` in
the diagram above.

## Disabling packages

Some packages might need to get (temporarily) disabled as refactorings occur in
the Fuchsia codebase. In order to disable a package `<layer>/<type>/foo`, move
it under `<layer>/<type>/disabled/foo` and remove it from `<layer>/<type>/all`.
Note that this does not apply to packages in directories that do not require
aggregation, as these packages are strictly opt-in already.

## Verification

The [`//scripts/packages/verify_layer`][verify-layer] tool is used to verify
that a layer's `packages` and `products` directory's structure matches the
description in the present document.

Note that only package files are allowed in such a directory, with the exception
of `README.md` files for documentation purposes.

[verify-layer]: https://fuchsia.googlesource.com/scripts/+/master/packages/README.md
