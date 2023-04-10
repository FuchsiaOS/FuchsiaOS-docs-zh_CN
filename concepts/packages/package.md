# Fuchsia packages

A Fuchsia package is a hierarchical collection of files that provides one or more programs,
components or services to a Fuchsia system.  A Fuchsia package is a term representing a
unit of distribution, though unlike many other package systems, that unit is composed of
parts and is not a single binary `BLOB`.

Note: For more information on components, see
[Introduction to the Fuchsia component framework](/docs/concepts/components/v2/introduction.md).

Some packages are present on a Fuchsia system at startup, and
additional packages can be downloaded from a Fuchsia package server in `BLOB`s.
The Fuchsia package server is an HTTP(S) server. These `BLOB`s are uniquely defined by a Merkle
root. A `BLOB` is named after its content, using the
[Fuchsia Merkle Root](merkleroot.md) algorithm. If two `BLOB`s have the same content,
they have the same name. As a result, each `BLOB` has a unique identifier and is
written to persistent storage using this Merkle
root as a key. This process is done to eliminate duplication of `BLOB`s that
may exist between packages. For example, a shared library which exists in
multiple packages is only stored once on the device.

The package server serves as a root of trust as it validates the authenticity of
each package.

Packages can also declare dependencies on named
[subpackage][glossary.subpackage]s, creating a hierarchy of nested packages.
Build rules link a package with the build target of each subpackage. At build
time, the package build tool records the subpackages in the parent package's
metadata, mapping each subpackage name to its package hash (the `BLOB` id that
identifies the subpackage). This ensures the list of subpackages and the
internals of each subpackage cannot change without also changing the Merkle
(package hash) of the parent.

_Subpackages enable:_

* Encapsulated dependencies (packages are inherently "package trees")
* Isolated `/pkg` directories (grouped components don't need to merge their
  files, libraries, and metadata into a single shared namespace)
* Assured dependency resolution (system and build tools ensure subpackages
  always "travel with" their packages)

For more information on packaging components with their dependencies using
Subpackages, see [Subpackaging components].

Note: To understand how components and packages work together in Fuchsia,
see [Fuchsia's software model](/docs/concepts/software_model.md).

## Types of packages

The packages that comprise the Fuchsia operating system are categorized into
three groups, which affect how they are managed:

Note: To understand how these packages work in a Fuchsia build, see
[Dependency sets](/docs/development/build/build_system/boards_and_products.md#dependency_sets)

* [Base packages](#base-packages)
* [Cached packages](#cached-packages)
* [Universe packages](#universe-packages)

### Base packages {#base-packages}

Note: Base packages are part of the system assembly process.
There is no way to determine if a package is a base package.

These are the packages that are part of the foundation of the Fuchsia
operating system and are considered critical for security and the system.
Resolving a package which is in base on a running Fuchsia system always
returns the version that is on the device, and not a new version which
may exist on a package server. However, base packages can be updated as part of
the [OTA process](/docs/concepts/packages/ota.md).

Since these packages are immutable for the runtime of a
system, these packages must be updated with
[`fx ota`](https://fuchsia.dev/reference/tools/fx/cmd/ota) which triggers an
over-the-air (OTA) update.

### Cached packages {#cached-packages}

These are packages on the device which are not part of base. These
packages exist when the device is flashed or paved, so these packages
are usable if the device boots without a network connection. Cached packages
are updated during the resolution process if a different package is available
on the package server. These packages are not updated during a system update,
but are updated ephermerally.

Fuchsia can also evict cached packages on a running system to free up
resources based on runtime resource demands.

### Universe packages {#universe-packages}

These are packages that exist on the package server, but not on the device.

## Structure of a package {#structure-of-a-package}

In most cases, a package in Fuchsia is a collection of `BLOB`s, which at a
minimum contains one content-addressed `BLOB` named [`meta.far`](#meta-far).

Note: For more information on the Fuchsia archive format (FAR), see
[Fuchsia archive format (FAR)](/docs/development/source_code/archive_format.md).

In Fuchsia, you build a package with the `ffx package build` command or the
legacy `pm` tool, which both exist in the `//tools/` directory of the
Fuchsia IDK.

Essentially, a package is a tree of zero or more content-addressed items.
A package contains the following:

* [`meta.far`](#meta-far)
* [`BLOB`s outside of `meta/`](#outside-blobs)

### `meta.far` {#meta-far}

Note: For more information on the Fuchsia archive format (FAR), see
[Fuchsia archive format (FAR)](/docs/development/source_code/archive_format.md).

The package metadata archive, `meta.far`, contains metadata about
a package, presented as the `meta/` directory. `meta.far` has a
[merkleroot](merkleroot.md) which in practical terms is also known as the
merkleroot of a package.

The `meta/` directory of a package contains at minimum two files:

* `meta/package`

   The package identity file. This is a JSON file that contains the name and
   version of the package.

* `meta/contents`

   The contents file. This file is created by the `ffx package build` command,
   (or the legacy `pm update` and `pm build` commands). This file maps the
   user-facing file names of a package to the Merkle root of those files.

If the package declares subpackages, the `meta/` directory also contains:

* `meta/fuchsia.pkg/subpackages`

   The subpackages file. This is a JSON file that contains the name and version
   of each declared subpackage. From the perspective of the parent package, the
   subpackage name is used as a relative package URL when resolving the
   subpackage.

   Package build tools traverse subpackage references (declared through build
   dependency declarations and package manifest files that reference other
   package manifest files for each subpackage) to compute the version (package
   hash) of each subpackage and generate the `subpackages` file.

Additionally, the `meta/` directory can contain files such as a component manifest.
For more information on component manifests, see
[Component manifests](/docs/concepts/components/v2/component_manifests.md).

### `BLOB`s outside of `meta/` {#outside-blobs}

Most files of a package exist outside of the `meta/`directory and each are a `BLOB`.

For example, these files can be like the following:

* `bin/foo`
* `lib/libfdio.so`
* `data/mydata.db`

## Identification of a package

Every package in Fuchsia is identified by a `package-url`.

Note: For more information about [Fuchsia package URLs](/docs/concepts/packages/package_url.md).

### Absolute package URLs

An absolute Fuchsia package URL identifies a system-addressable package, without
requiring any additional context, and looks like the following:

```
fuchsia-pkg://{{ '<var>' }}repository{{ '</var>' }}/{{ '<var>' }}package-name{{ '</var>' }}?hash={{ '<var>' }}package-hash{{ '</var>' }}#{{ '<var>' }}resource-path{{ '</var>' }}
```

Fuchsia has different intereprations of `fuchsia-pkg` URL depending on which parts of the URL are
present.

 * If the repository, package, and resource parts are present, then the URL
   identifies the indicated resource within the package.
 * If only the repository and package parts are present, then the URL identifies
   the indicated package itself.
 * If only the repository parts are present, then the URL identifies the
   indicated repository itself.

The package parts can express varying degrees of specificity. At minimum the
package name must be present, optionally followed by the package hash.

If the package hash is missing, the package resolver fetches the resources
from the newest revision of the package variant available to the client.

### Relative package URLs

A relative Fuchsia package URL identifies a subpackage given previously loaded
package (or subpackage) as "context". The repository and parent package are
implicit, and the subpackage name is used to look up the package hash in the
parent package's `"meta/fuchsia.pkg/subpackages"` file. (The package hash
cannot be overridden). A relative package URL looks like the following:

```
{{ '<var>' }}package-name{{ '</var>' }}#{{ '<var>' }}resource-path{{ '</var>' }}
```

As with absolute package URLs, the resource path may or may not be included.

[Subpackaging components]: /docs/concepts/components/v2/subpackaging.md
[glossary.subpackage]: /docs/glossary/README.md#subpackage
