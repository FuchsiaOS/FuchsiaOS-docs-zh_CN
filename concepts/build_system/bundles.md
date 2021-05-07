# Bundles

Bundles are GN group labels that provide common major groups of features.
They can be included into one of the [dependency
sets](boards_and_products.md#dependency-sets).

When using the `fx set` command, bundles are most commonly added to the
`universe` dependency set by use of the `--with` flag. See [fx build
configuration][fx-build-config] for more information.

More information on the currently available bundles can be found in
[`//bundles`](/bundles/README.md).

## Key bundles

* `tools` contains a broad array of the most common developer tools. This
  includes tools for spawning components from command-line shells, tools for
  reconfiguring and testing networks, making http requests, debugging programs,
  changing audio volume, and so on.
* `tests` causes all test programs to be built. Most test programs can be
  invoked using `run-test-component` on the device, or via `fx test`.
* `kitchen_sink` is a target that causes all other build targets to be
  included. It is useful when testing the impact of core changes, or when
  making large scale changes in the code base. It also may be a fun
  configuration for enthusiasts to play with, as it includes all software
  available in the source tree. Note that kitchen sink will produce more than
  20GB of build artifacts and requires at least 2GB of storage on the target
  device (size estimates from Q1/2019).

[fx-build-config]: /docs/development/build/fx.md#configure-a-build
