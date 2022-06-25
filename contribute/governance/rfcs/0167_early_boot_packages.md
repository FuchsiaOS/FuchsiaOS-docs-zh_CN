<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0167"%}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md"%}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

This RFC proposes introduction of packages to bootfs. This will bring the
benefits of package isolation and namespacing that late usermode enjoys to early
boot, and removes blockers for third-party driver development.

## Motivation

Early-boot executable assembly and sandboxing came into existence before the
component framework had a solid packaging architecture, and to date we have not
invested in bringing the tools now available to later userspace into early boot.
As a result, userspace bootstrapping finds itself facing several problems that
packaging was introduced to solve; problems like process sandboxing, verifiable
contents, and variable library versions.

These early-boot problems manifest as unintended complexities and inefficiencies
in both run time and build time interactions with the bootfs image, and can be
largely eliminated by embracing packagification in early boot. Packagification
of early boot introduces opportunities to improve the health of the system,
beyond resolving existing issues. Standardizing on content identifiers for
executables and libraries throughout user space allows us to re-use copies of
equivalent data from previously disjoint storages, like bootfs and blobfs.

Component Framework has a 'package' concept, that acts as an abstraction over
fuchsia-pkg; while Component Framework and the packaging system are,
strictly speaking, separate, they are closely intertwined enough that the
goals of componentizing and packagifying the world are cooperative.

Recent work (e.g. component manager executing as the first post-userboot
process) are driving an increasingly "components all the way down" architecture.
Even early-boot executables like filesystems and device-drivers are launched as
Fuchsia components, or are migrating towards that model today. Now is the time
to match this componentization of the system with a
"packages-all-the-way-down" system assembly, and bring all of the value
that motivated packaging to userspace bootstrapping.

More concretely, the absence of package namespacing in the bootfs will become a
blocker for out-of-tree driver development due to its inability to
produce valid fuchsia images where executables encoded in the bootfs have
version skew in shared lib deps. Additionally, the bootfs image today is leaking
ABIs. A driver can alter its behavior at startup based on the presence or
absence of other drivers, choosing to use a newer library for example, and it
can do this without explicitly defining a dependency on the other driver.
Similarly, because drivers all end up in the same folder within bootfs, the
names of drivers create an ABI in and of itself. These types of unintentional
ABIs become harder to deprecate the longer they're available, and there is
precedent for exactly this type of behavior (Windows drivers and video game
DRM drivers expose this type of unintentional ABI).


## Stakeholders

_Facilitator:hjfreyer@google.com_

_Reviewers: geb@google.com, mcgrathr@google.com, surajmalhotra@google.com,
aaronwood@google.com, galbanum@google.com, wittrock@google.com,
jfsulliv@google.com_

_Consulted:_

_Socialization: Topic was explored via design document with stakeholders,
then opened to tq-eng for general discussion before RFC._

## Design

This change involves changes to the bootfs and /boot directory, to the
BootResolver, to the BootUrl, and to product assembly.

### <a id="bootfs_changes" /> bootfs Changes

#### Image Changes

A fuchsia bootfs package will be represented by a meta.far file in bootfs, which
the bootfs directory entry will name `blob/<merkle root of meta.far>`. Every
blob in the package's manifest will receive a new directory entry in
bootfs, `blob/<merkle root of the dependency>`.

Note: Bootfs image assembly currently deduplicates files by file source path:
if the same file in the build directory (or different links to
the same file) is used twice, it gets only one copy. However, if two separate
files in the build directory happen to have identical contents, the tool
won't notice this.  Determining content identity requires hashing the contents
of each file, while determining file identity is just two integers fetched by
a cheap(ish) system call for each file. However, it's unlikely that introducing
content identity will produce be a problem we need to solve for two
reasons:

 1. We already calculate merkles at runtime to validate contents, and it's fast
 enough there, so doing so during a process like zbi creation, which also
 compresses the image, is inconsequential.
 2. The new image construction procedure required by this proposal already
 determines content identities before the actual image construction step, so as
 to populate meta.far files.  This means that the content-based deduplication
 has mostly already happened in the process of just producing the manifests to
 drive the image construction step.  It should be pretty straightforward to
 just ensure that we've done all possible deduplication at this stage.

When we discuss adding a new entry to the bootfs under /blob, we are only
referring to adding a new bootfs directory entry referencing the same
underlying file (conceptually a hard-link).

A new file in bootfs named "pkg_map" will maintain a mapping from the
human-readable package name to the merkle root of the meta.far that encodes the
package.

The bootfs size will grow by the newly added meta.fars and the new pkg_map file.
Everything else amounts just to a new directory entry for a bootfs file that was
already present. The bootfs will, conservatively, increase by ~70KiB compressed
on x64 architecture.

#### /boot Changes

A new subdirectory will be introduced to /boot, called /blob. All files in the
bootfs image whose names are prefixed with /blob will be placed there. When
migration of all components in bootfs to packages is complete, the top
level directory will contain only kernel vmos, shell scripts, and the files
needed for /boot to be the "namespace" of the component manager.

The component manager namespace will initially be a subdirectory of /boot
that is "laid out" as all its dependencies expect. Eventually, we aim to
converge even component manager into a meta.far encoding that relies on the
SWD stack for resolution.

### BootResolver Changes

The core work for package-namespacing bootfs components *heavily* overlaps with
the work done by package-resolver. A human-readable package name must be mapped
to a content-id'ed meta.far, the meta.far must be decoded, and its contents file
must be used to construct a namespace. As such, the design aims to re-use as
much of the existing package-resolver logic as possible.

The easiest entry-point for re-use of the package-resolver's logic is
[package-directory::serve](https://fuchsia.googlesource.com/fuchsia/+/ff9e156aceb01c2bd5658236fa679d3b60db397d/src/sys/pkg/lib/package-directory/src/lib.rs#64)
. This entry-point takes a BlobFS and a content-id
identifying a meta.far, opens the meta.far, parses its meta/contents file, and
constructs and serves a namespace as encoded in the file. We can re-use this
library as is, as long as we are able to provide the bootfs-backed directory of
blobs to the API as a BlobFS client.

```
    let (proxy, server) =
        fidl::endpoints::create_proxy().map_err(ResolverError::CreateEndpoints)?;
    let () = package_directory::serve(
        package_directory::ExecutionScope::new(),
        <some blobfs::Client-like view on top of the bootfs blobs>,
        <meta.far hash>,
        fio::OPEN_RIGHT_READABLE | fio::OPEN_RIGHT_EXECUTABLE,
        server,
    )
    .await
    .map_err(ResolverError::ServePackageDirectory)?;
```

The blobfs::Client simply wraps a fio::DirectoryProxy, which is how the bootfs
is exposed from the component manager. We can simply wrap this bootfs with the
Blobfs client, and pass it into the package_directory::Serve call. Some small
changes will need to be made to ensure that certain blobfs::Client apis fail
gracefully when sitting on the bootfs (namely those that require mutability; eg
open_blob_for_write, delete_blob), however no mutability is required during the
package_directory::serve execution.

### BootUrl Changes

BootUrl currently does not use the host or path sections of its url, as there is
no repository nor packaging to encode. This means that, rather than introduce a
new url scheme and resolver, we can instead introduce the new component loading
in the existing BootResolver with the existing fuchsia-boot scheme. The presence
or absence of a package-path in the url will become the indicator that the new
resolution pathway described in "BootResolver Changes" should be used.

Examples:

`fuchsia-boot:///#my_component.cm` The bootresolver will interpret this URL as
an unpackaged component whose namespace is already correctly set up within the
/boot directory.

`fuchsia-boot:///my_package#my_component.cm` The bootresolver will interpret
this URL as a packaged component, for which a mapping from "my_package" to the
merkle root of the meta.far for my_package exists and should be used to
construct a namespace specific to "my_package"'s namespace.

### Product/Image Assembly Changes

Bootfs image construction is accomplished by a 2-phase execution of work done
by the build system and then the image assembly system.

First, we will introduce a new gn variable called `bootfs_packages`.
This list will be declared in product.gni, and will be assigned to an
invoker variable called `bootfs_package_labels` passed to any
invocations of `assemble_system` that assign `build/input:bootfs`
to a `bootfs_labels` variable.

When we migrate a bootfs component from its package-less encoding to
a bootfs package, we will remove it from the group dependency that includes
it in the `bootfs_labels` dependency set, and add it to the `bootfs_packages`
set.

Next, when generating the image assembly configuration, we will use the existing
`list_package_manifests` template to collect package manifests from the packages
defined in the `bootfs_package_labels` variable of the invoker.

Next, image assembly takes the manifests from the build traversal and uses it to
make calls to tools like `zbi` to package files in the build dirs into an image.
The package manifest format contains a list of "blob" objects which, at image
creation, will be used to add `blob/<merkle_root>` named files into the
bootfs image.

While iterating over these blob objects, we will check to see which of the blobs
is the identifier for the package meta.far and will add a mapping from the
package name to the meta.far's merkle root to a map. At the end of bootfs image
creation, the map will be written in json format to the "pkg_map" file described
in the [bootfs Changes](#bootfs_changes) section above.

We choose to implement this transformation at the Image Assembly level for three
reasons:

* ProductAssembly just merges the lists of packages from its various
bundles.

* ImageAssemblyConfig validation remains simple.

* Validation within Product Assembly of "packages" continues to
operate on "packages".

## Implementation

### <a id="feature_impl" /> Feature Implementation

Image assembly changes, bootfs changes, BootResolver changes,
and BootUrl changes can all be done at the same time.

To guarantee that we don't see an introduction of packages to
bootfs_package_labels midway through implementation,
we will start with the Image Assembly changes. We will
implement this functionality up to the point of bootfs package manifest
aggregation, and we will place a build-time check that the set is empty.

The semantics of the BootUrl will change over 3 CLs. First, prior to migrating
the first bootfs component to package namespacing, we will change the BootUrl to
be considered invalid if it includes a package path or repository (to ensure
that we retain the availability of the package path as an indicator of
resolution strategy). Second, along with the first bootfs component migrated to
a fuchsia_package we will allow package paths in fuchsia-boot urls. Third, along
with the last bootfs component migrated to a fuchsia_package, we will disallow
fuchsia-boot urls that do not include a package.

### Migration

With the features fully implemented, we will migrate bootfs components
incrementally. Migration for a given component is as follows;

 1. We will find the location where the unpackaged deps of the component
 (.cml files, binaries, etc) are added to the bootfs_labels deps.
 2. We will turn that collection of unpackaged deps into a fuchsia_package
 gn target.
 3. We will remove the package from the existing bootfs_labels group, and add
 it to the bootfs_package_labels group.
 4. We will update the url for the component in the bootstrap.cml file to
 include a package name.

## Performance

### System Size
The bootfs image will, conservatively, increase by ~70KiB compressed
on x64 architecture. This is because once all components are migrated to
packages, the bootfs size will grow by the newly added meta.fars and
the new pkg_map file. Everything else amounts just to a renaming of
a directory entry for a bootfs file that was prior to packagification.

### Runtime Implications
Today, the entire bootfs is parsed into a directory eagerly at component_manager
startup. After migration, we end up deferring the parsing work equivalent to the
set up of acomponent's namespace within /boot until that component is started.

There is some additional work beyond what was previously just the parsing of
bootfs headers; we must parse a meta.far.

The ZBI is signed, and so today we do no verification of the contents of the
bootfs files within the zbi, just verification of the zbi itself. If we
do no runtime verification of blobs in the bootfs we would be in strictly the
same security position that we're in today.

One potential errant state that the bootfs could find itself in today is that
assembly could incorrectly place a source file under a bootfs link that is
wrong. The presence of blobs in bootfs means that, if we want to, we can
protect ourselves from this case by doing runtime verification of the bootfs
blobs. We do not plan to do this initially.

### Buildtime Implications
The buildtime performance for zbi construction is unchanged; after migration
rather than walking the destination-entry manifests for every dep included
in bootfs, we instead walk the package manifests to encode the blobs. In
practice, we walk the exact same number of artifacts.

## Backwards Compatibility

Many changes involved in this proposal are self-contained within the bootfs.
There is no smaller granularity for updating units of execution within the
bootfs than the entire bootfs at once. As a result, there is no risk of a system
with incomplete bootfs packaging implementation being provided packaged bootfs
components.

One potential source of backwards compatibility issue is if image assemblers
somehow found themselves including a legacy bootfs image with a new component
manager in the same zbi. If this were to occur, once migration is complete
and non-packaged components are disallowed in bootfs, this could introduce
errors. Given how product assembly constructs the bootfs, however, there is
no way for this state to arise today.

Similarly, the changes to the semantics of the BootUrl described in [Feature
Implementation](#feature_impl) will not diverge from the feature
implementation or migrations because it too is contained within the bootfs.

## Security considerations
Package namespacing/process sandboxing strictly improves runtime security by
reducing the artifacts that a process has access to.

The bootfs image is signed and read-only, so as the main security question
is whether the image assembler is trusted which is unrelated to the formatting
of the image. If anything, providing merkle-root identities of executables in
the bootfs offers potential for further security improvements if we ever decide
that the bootfs image should be incrementally updatable.

## Privacy considerations

None.

## Testing

Testing practices in component resolvers and product assembly are well
documented, and will be expanded to cover new functionality.

BootUrl parser tests will be added at each of the 3 stages of semantic changes
to show that the expected behavior of the Urls is being enforced.

## Documentation

The API documentation of userspace bootstrapping, along with documentation about
bootfs image assembly, would need to be updated.

## Drawbacks, alternatives, and unknowns

### Reuse of bootfs_labels vs introducing a new label.
Today, accidental inclusion of packages in the list of dependencies
in  bootfs_labels list is a no-op. With this change, the
presence of a package will be semantically significant. So,
we need to "clean out" unintentionally included packages from
the bootfs_labels namespace.

The only complexity here involves fuchsia_driver_packages.
fuchsia_driver_package is a unique build template; how product assembly
interacts with a fuchsia_driver_package differs depending on whether the driver
is being placed into bootfs or blobfs. When placed in bootfs_labels, product
assembly walks through the driver package to treat the driver like a group of
the dependencies, and when placed in blobfs the driver is properly packaged via
meta.far. This was done so that a single driver target can switch between bootfs
and blobfs depending on the product. It worked in the past because bootfs
product assembly doesn't assign any significance to the presence of a package in
its deps graph. With bootfs package namespacing, however, we will be
interpreting the presence of a fuchsia_package
(and its associated package_manifest) as a declaration to place a meta.far
encoding the package namespace into bootfs.

If we were to land the bootfs namespacing feature today, ~20 drivers would end
up being both placed in bootfs as unpackaged deps, and also encoded in bootfs by
an associated meta.far and associated blobs; the meta.far encoding of the driver
would go unused until changes to the driver_manager's runner. Rather than bloat
the bootfs with unused packaging, we want to clear out all fuchsia_packages from
the bootfs, and then incrementally migrate targets to proper packaging. We
choose to clear out the package_manifests of fuchsia_driver_packages by
introducing a new gn metadata barrier that prevents list_package_manifests from
walking to fuchsia drivers. This is done, rather than splitting every driver
that uses fuchsia_driver_package into a bootfs target and a blobfs target,
because it significantly reduces the complexity of the driver product assembly
logic. Additionally, a single package definition in the build that can serve
either purpose in the eventual outcome, so this avoids churn in splitting each
thing and then removing all the "old bootfs only" pseudo-packages.

Unfortunately, this has separate complexities, like introducing shadowing of
the true fuchsia_package target, hiding its name from users. This introduces
incompatibilities with other existing and ongoing work like golden tests to
verify that all package manifests present on a device are expected by name,
or product assembly work to require that labels which generate package indices
only directly rely on the package target itself.

As a result, the less intrusive approach is to simply add a new label and,
when migrating components in early boot to packages, move them from the old
label and into the new label. Eventually, we will need to merge these lists
and if drivers have not yet been migrated to proper components, we will
need to revist the complexity of the fuchsia-driver-package template.

### Implement in the Product- or Image-Assembly Operation

*   Should the parsing of package manifests for bootfs components be done at the
    product assembly or image assembly level? In other words, should we defer
    the parsing of the manifests until like right when we're about to call the
    zbi tool to actually build the bootfs?
    *   ProductAssembly (ffx assembly product)
    *   Pros:
        *   ImageAssembly stays very focused on generating the image files
            themselves (zbi, blobfs, etc.)
        *   ImageAssemblyConfig contains a simpler list of all bootfs files.
    *   Cons:
        *   Need to have a more complicated validation that the
            ImageAssemblyConfig used to generate the legacy assembly input
            bundle matches the ImageAssemblyConfig created by ProductAssembly
            (or that validation needs to be discarded/weakened).
        *   validation done at the end of product assembly needs to know where
            to find "packages"for components in bootfs (such as the value-file
            existence validation done for structured configuration).
    *   ImageAssembly (ffx assembly create-system)
    *   Pros:
        *   ProductAssembly just merges the lists of packages from its various
            bundles
        *   ImageAssemblyConfig validation remains simple
        *   Validation within Product Assembly of "packages"continues to
            operate on "packages".
    *   Cons:
        *   ImageAssemblyConfig no longer contains the full contents of bootfs
            files.
        *   Image Assembly needs to perform the package -> to entry mapping
            instead, before creating the zbi.

### Encode meta.fars in bootfs or do the namespace setup at bootfs image construction?

One alternative approach was to simply encode "namespaces" in the bootfs by
constructing the image such that each component under /boot had its own
subdirectory that component manager used as that component's namespace root.

In encoding meta.fars in bootfs we increase the dependencies of early boot by
introducing a dependency on SWD package resolution libraries in boot-resolver,
which exist to interpret the meta.far format and is staffed and maintained to
continue doing so in the face of changes to the meta.far format.

In performing namespace setup during image construction, we increase the
complexity of product assembly by teaching it how to interpret a
package-manifest/meta.far format, and writing code to translate the meta.far
format into a package-namespace within the bootfs image. this means new code,
and that a second team becomes "in the business"of interpreting meta.fars and
becomes responsible for keeping their product in sync with it. The SWD team has
expressed concern about having new teams take on dependencies for the meta.far
format, and this can be entirely avoided by converging on a single package
encoding, and re-using the SWD-maintained tools for interacting with that
format.

The resource implications of both strategies are nearly equivalent (order of
KiBs).

Lastly, packages are more than just the namespace they encode. Metadata in
meta.fars like package version are needed for runtime features like platform
versioning. If meta.fars were not used, we would need to introduce some new way
of encoding this "extra information" in the bootfs-based namespaces and teach
services like component manager about this additional encoding of package
metadata.
