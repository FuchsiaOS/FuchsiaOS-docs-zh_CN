# Third party source management

This document reviews the policies and best practices for ensuring clear
separation between third party and first party code.

## Third party code in Fuchsia

Third party code is part of the Fuchsia checkout but is neither copyrighted by
the Fuchsia authors nor subject to Fuchsia's [license]. In other words, any code
that is not 100% owned by the Fuchsia authors should be managed as third party
code.

The Fuchsia project maintains copies of third party code dependencies under the
`//third_party/` directory in the checkout. This is also known as vendoring.
Vendoring ensures that third party code is served from Fuchsia-owned source
repositories and is served at revisions that are known to work with other code
in the Fuchsia checkout.

To ensure that third party code is in compliance with the the Fuchsia project
policies, follow the best practices in the section below. These best practices
also help simplify maintenances tasks, for instance, when updating third party
code to newer revisions from upstream repositories.

## Third party source layout best practices {#third-party-layout}

This section describes the recommended strategy for manually managing third
party code. However, if possible, automated
[dependency management solutions](#managed-third-party) are preferred.

Below is the recommended source layout, using `googletest` as an example:

```none {:.devsite-disable-click-to-copy}
root [fuchsia.googlesource.com/fuchsia]
  third_party/
    googletest/
      BUILD.gn
      OWNERS
      README.fuchsia
      src/ [fuchsia.googlesource.com/third_party/github.com/google/googletest]
```

Note the following:

- `//third_party/googletest/src/` is the root of a
  [Fuchsia-owned mirror repository][third-party-googletest] that contains a copy
  of the [upstream repository][googletest]{:.external} for `googletest`.
- `//third_party/googletest/` is within the [`fuchsia.git`][fuchsia-git]
  repository.
- `//third_party/googletest/README.fuchsia` contains structured metadata
  regarding `googletest`. See [`README.fuchsia` file syntax][readme-fuchsia].
- `//third_party/googletest/BUILD.gn` defines build targets for the `googletest`
  library. Since this file belongs to [`fuchsia.git`][fuchsia-git] (not the
  [`googletest` repository][third-party-googletest]), it can be updated in
  lockstep with other Fuchsia `BUILD.gn` files that depend on `googletest`. This
  makes build refactors and other large-scale changes easier.
- `//third_party/googletest/OWNERS` is the associated [`OWNERS`][owners] file.
  Because it's defined in `fuchsia.git`, it is possible to include owners from
  other files elsewhere in the Fuchsia project.

Additional files that are required to adapt the third party code to the Fuchsia
project may be present under (in this case) `//third_party/googletest`.

### Applying best practices

Third party repositories that had been introduced before the best practices were
established do not necessarily follow the recommended source layout in the
section above. For instance, these legacy repositories often mix first party
files (mainly `BUILD.gn`) with third party files.

To migrate legacy third party repositories to the new layout, follow these
steps:

Note: For Python repositories, replace `//third_party/<name>/src` with
`//third_party/<name>/<module_name>` to follow Python's convention.
This convention is expected by common Python tools like
[pyright][pyrightconfig].

1. Move Fuchsia-specific `BUILD.gn` files to
   [`//build/secondary`][build-secondary].

   1. Copy `BUILD.gn` files from `//third_party/<name>` to
      `//build/secondary/third_party/<name>`. If there is more than one
      `BUILD.gn` file, maintain the same subtree under `//build/secondary`.
   1. In the copied `BUILD.gn` files, update references to paths to third party
      files in the form of `//third_party/<name>/` to the form of
      `//third_party/<name>/src/`.
   1. Copy `OWNERS` from `//third_party/<name>` to `//build/secondary/<name>`,
      or create it if it does not exist. Review the `OWNERS` file to ensure that
      it follows the [best practices][owners-best-practices].
   1. Copy `README.fuchsia` from `//third_party/<name>` to
      `//build/secondary/<name>`. Review the contents of this file and ensure
      that the metadata is correct. In uncommon cases there are modifications
      made to third party code in third party repositories, and such changes are
      listed in `README.fuchsia`. Local modifications will often require you to
      make special accommodations that are not covered in this guide.
   1. Review `//third_party/<name>` for any other first party `.gni` files and
      move those to `//build/secondary/<name>` as well.
   1. Update `//build/secondary/third_party/<name>/BUILD.gn` (and other files
      containing source paths such as `.gni` files) to use the new source
      location `//third_party/<name>/src`. This requires updating all sources,
      including directory paths and more.

   Example: [https://fxrev.dev/622785](https://fxrev.dev/622785)

   Note: CQ won't catch errors in this change (for example, typos in paths)
   because the secondary build files are not yet active. You can validate that
   your `BUILD.gn` files are correct by staging the next commit (update the
   integration manifest), running `jiri update -local-manifest`, then building
   (such as with `fx build`).

1. Update the integration manifest.

   Note: This step can only be performed in Google's internal repositories at the
   moment.

   Replace `path` (not `name`) of the existing third party project at
   `//third_party/<name>` with `//third_party/<name>/src`, while keeping the
   revision unchanged. With this change merged, the Fuchsia build will switch to
   using the `BUILD.gn` files from the previous step.

   Example: [http://tqr/457911](http://tqr/457911)

1. Move Fuchsia-specific files added in step 1 to `//third_party/<name>`.

   Now that third party code is nested under `//third_party/<name>/src` and
   `//third_party/<name>` is part of [`fuchsia.git`][fuchsia-git], you can undo
   the transitional step 1.

   1. Wait for the integration manifest change to merge and roll, then run
      `jiri update`. Or stage the integration manifest change from the previous
      step in your local checkout, then run `jiri update -local-manifest`.

   1. Move `BUILD.gn` and other Fuchsia-specific files from
      `//build/secondary/<name>` to `//third_party/<name>`.

   1. Update [`//.gitignore`][gitignore] so that `//third_party/<name>` is
      tracked but `//third_party/<name>/src` is not tracked.

   Example: [https://fxrev.dev/622789](https://fxrev.dev/622789)

1. Turn `//third_party/<name>/src` into a mirror.

   Note: This step can only be performed in Google's internal repositories at the
   moment.

   Change `//third_party/<name>/src` to track upstream such that it only has
   upstream changes in its `git log`. You can do this by updating the
   integration manifest to reference an upstream commit hash.

   Example: [http://tqr/427570](http://tqr/427570)

## Language-specific dependency management solutions {#managed-third-party}

For automated dependency management solutions, see the following:

- **Rust**: Follow the [external Rust crates][rust-third-party] guide.

- **Dart**: Follow the [third party Dart packages][dart-third-party] guide.

- **Go**: See [`//third_party/golibs/`][golibs].

There are currently no solutions for automated management of C/C++ and Python
dependencies. Please refer to the [source layout section](#third-party-layout)
above.

## Exceptional cases

The majority of third party dependencies can follow the practices described
above. However, a small minority of dependencies that are subject to uncommon
circumstances are managed differently.

Having exotic dependencies can increase complexity and maintenance costs, which
are incurred by direct dependencies of the third party code. Additionally, they
add complexity to common global maintenance tasks such as:

- Performing git administration tasks.
- Updating and maintaining toolchains.
- Responding to disclosed security vulnerabilities by updating vulnerable third
  party code from upstream sources.
- Refactoring build rules, such as to enforce new compile-time checks.

Please exercise careful deliberation when stepping off the beaten path.

## Additional reading

- [Fuchsia open source licensing policies][oss-licensing]
- [Open Source Review Board (OSRB) process][osrb-process]
- [Source code layout][source-layout]

[build-secondary]: /build/secondary/
[dart-third-party]: /development/languages/dart/third_party.md
[fuchsia-git]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main
[gitignore]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/.gitignore
[golibs]: /third_party/golibs/
[googletest]: https://github.com/google/googletest
[license]: /LICENSE
[osrb-process]: /contribute/governance/policy/osrb-process.md
[oss-licensing]: /contribute/governance/policy/open-source-licensing-policies.md
[owners]: /development/source_code/owners.md
[owners-best-practices]: /development/source_code/owners.md#best_practices
[readme-fuchsia]: /development/source_code/third-party-metadata.md
[rust-third-party]: /development/languages/rust/external_crates.md
[source-layout]: /development/source_code/layout.md
[third-party-googletest]: https://fuchsia.googlesource.com/third_party/github.com/google/googletest/
[pyrightconfig]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/pyrightconfig.json
