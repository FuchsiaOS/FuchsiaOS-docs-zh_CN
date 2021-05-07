# Garbage collection

## Static vs dynamic indexing

Static indexing is used for base packages. At `pkgfs` startup, base packages are
pre-populated in `/pkgfs/packages` based on the `static_packages` index located in
`/system/data/static_packages`. These static packages are then used to bootstrap
the system, so that core components like `pkg_resolver`, `pkg_cache`, `netstack`,
`sysmgr` can be started.

`pkgfs` doesn't maintain state across reboots but the base package set is
guaranteed to always be present. Base packages cannot be deleted.

The dynamic index stores a mapping of all ephemerally fetched packages. `pkgfs`
will pre-populate the dynamic index with any present packages (i.e. `meta.far`
and all `BLOB`s resolved) listed in `/system/data/cache_packages`. In memory, the dynamic
index has the most recently resolved version of a package with the same name
by keying on the `$name/$variant` of the package. `pkgfs` then "forgets" about
the old version of the package. The old version of the package is still present
in the system but no longer referenced. The dynamic index is then used to implement
garbage collection.

## How to garbage collect

There is no notion of installing a package in fuchsia and likewise no notion of
deleting a package. Rather, garbage collection can be thought of as a means to
reclaim space. Garbage collection can be triggered manually by running `pkgctl gc`
or it can be triggered by the `system-updater`. The implementation of garbage
collection uses the [`fuchsia.space/Manager` protocol](https://fuchsia.dev/reference/fidl/fuchsia.space?hl=en#fuchsia.space/Manager.Gc). The `system-updater` trigger
happens twice; once before a system update and once after fetching the [update package](update_pkg.md).

The `pkgfs` garbage collector currently uses set differences to determine which
packages are live packages. A package is considered live if any of the following
is true:

* A package is a base package in the static index.
* A package is in the process of being updated (by tracking the `meta.far` merkle
  root and any missing `BLOB`s until they’ve been fully resolved).
* A package is the most recently resolved version of an ephemeral package according to its `meta` or `package` in the dynamic index.

When garbage collection runs, it deletes every `BLOB` in `blobfs` that is not referenced
by a live package.

## Known issues

Existing garbage collection implementation is suboptimal.

* An old version of an ephemeral package that is open can be garbage
collected. This may lead the garbage collector to erase a package out
from under a component.

* If `system-updater` fails to download a new package, the garbage collector
 protects both the base package and the most recent package version, which leads
 to duplicate copies of every package. If this happens, you should reboot the
 Fuchsia device to clear the list of activated packages.
