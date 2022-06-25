<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0135" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

Design for encoding the system ABI revision into packages.

## Motivation

[RFC-0002] introduces the concepts of [API level] and [ABI revision]. The _API
level_ is a monotonically increasing human-understandable number that
corresponds to a set of APIs that are available when building applications. The
_ABI revision_ corresponds to the specific semantics exposed by the [Fuchsia
System Interface] the application expects the platform to provide. An API level
corresponds to a specific ABI revision, but multiple API levels can refer to the
same ABI revision. This functionality allows for the Fuchsia platform to evolve
over time while continuing to support older applications.

Implementing platform versioning requires that packages encode the ABI level
they wish to target, so that the platform can know whether a package is
compatible with the running system.

## Stakeholders

_Facilitator:_ abarth@google.com

_Reviewers:_

The following stakeholders were chosen as this RFC touches Fuchsia Tools,
Software Delivery, and will eventually impact how Component Framework will
select the component system interface to present to applications.

* abarth@google.com (FEC member)
* computerdruid@google.com (SWD)
* geb@google.com (CF)
* mkember@google.com (FIDL)
* raggi@google.com (Tools)
* wittrock@google.com (SWD)

_Consulted:_

* lijiaming (PDK)

_Socialization:_

This RFC went through a design review with the Software Delivery and Fuchsia
Tools teams.

## Terminology

A Fuchsia _[package]_ is the unit of distribution for distributing and
organizing files for Fuchsia programs, components, and services.

The _[meta.far]_ is the package metadata archive. It contains the mapping of
user-facing file names to the underlying content addressed blobs. This is used
by the packaging system to download package contents, and construct a namespace
hierarchy. It also contains custom user-provided files.

## Design

This design introduces the notion of a _package ABI revision_, which is the
target ABI revision of the Fuchsia platform interface used by a package. This
value will be written as an unsigned little endian 64-bit integer written into
the `meta.far` in the file `meta/fuchsia.abi/abi-revision`.

All package generation tooling will be updated to require either the API level
or the ABI revision to be specified during package building. When the API level
is specified, the tooling should encode the corresponding ABI revision found in
the [SDK version history], or error out. Likewise, the tooling should enforce
that the specified ABI revision is supported by the SDK, or error out. This
should avoid the risk of creating a component with an older SDK, but specifying
an ABI only present in a newer SDK. At best this would result in a component
that failed to run. At worst this could lead the component to strange or
dangerous bugs.

### Use cases for package ABI revision

This proposal only covers how the ABI revision is encoded into a package. Users
of the package ABI revision will be defined in future designs. However, here
are some potential use cases to help illustrate how this may be used:

* [RFC-0002 Platform] describes a mechanism where Component Manager could use
  the package ABI revision to select which platform interface to use when
  running a component.
* _System assembly_ is the process of composing a Fuchsia [system image] from a
  set of packages and other compiled artifacts. The system image can then be
  used to deliver Fuchsia through a different means (OTA, flashing, paving,
  etc). The package ABI revision can be used by the the assembler to reject
  integrating a package if the system does not support the required package ABI.
* Similarly, petals or application developers can use the package ABI revision
  to understand if all of their components in a complex tree can run on their
  target Fuchsia release.
* _Ephemeral packages_ are packages that are downloaded upon demand, rather than
  being built into the system image. The Package Resolver could leverage the
  package ABI revision to reject downloading a package that it knows is not
  executable on the system. Note though this may not work for system updates.
  See [System update considerations](#system-update-considerations) for more
  details.

### Target API level or ABI revision selection

This proposal indends to make the API level or ABI revision a required argument
when building a package, but this will be optional during the initial rollout
of this feature. With that in mind, end-developers should parameterize their
build rules for the target API level or ABI revision. This should make it easy
to target new releases. It may even be possible to set up automatic testing of
new API levels by setting up a test roller that experimentally advances the
target API level and sees if anything fails. This could feed back into platform
bug reports if these failures were unexpected.

### System update considerations

In order for a Fuchsia device to update from one release to another, the
_system updater_ resolves a special package known as the _update package_. This
package describes all the base packages and other artifacts necessary for the
new Fuchsia version to run. These new packages are not intended to be used on
the current system, but instead would be used once the device is rebooted into
the new system.

In light of this, if we decided to use the package ABI revision to reject
downloading a package, we need to be careful to avoid breaking the OTA process.
This can be seen through the following example:

Consider two Fuchsia builds `A`, which only supports `ABI-1` and `C`, which
supports `ABI-2`. If we filter packages based off ABI, the update package for
`C` would need to be `ABI-1` for it to download. However, the `C` version base
packages would have to be `ABI-2` to use the new ABI, but these would be
rejected by the version `A` package resolver.

One way to avoid this is to introduce new ABI revisions through a _stepping
stone release_. Fuchsia is designed to allow a device to skip multiple releases
in order to update to the latest version. A stepping stone release is a special
release that cannot be skipped. It is designed to enable graceful transitions
through platform interfaces.

Redoing the prior example, instead lets have 3 sequential Fuchsia releases:

* `A`: supports `ABI-1`, base packages at `ABI-1`
* `B`: supports `ABI-1` and `ABI-2`, base packages at `ABI-1`.
* `C`: supports `ABI-2`, base packages at `ABI-2`.

If we mark `B` as a stepping stone release, then a device running `A` would
first update to `B`, then update to `C`.

This idea can be taken even further by decoupling how packages are defined from
the ABI revision. Since we do not intend to frequently make backwards
incompatible changes to package layout, we can version our package layout, and
then state which package layout versions are supported by a given ABI revision.
This would allow us to install a greater range of update packages without
needing to create stepping stone releases.

### Avoiding namespace collisions in the meta.far

The `meta.far` currently contains two package metadata files, `meta/package`
and `meta/contents`, as well as arbitrary user specified files. This means that
there is a possibility of collision between user files and any new metadata
files we introduce into a package. To avoid this possibility, the package ABI
revision will be written in a directory `meta/fuchsia.abi`. This directory name
follows the convention used elsewhere in Fuchsia, such as the platform FIDL
namespaces. The package building tools will be further updated to prevent users
from defining custom files in `meta/fuchsia.abi`.

Since this is a directory controlled by the platform, it's possible to use this
location to store ABI-related files without colliding with user files. This
would allow us to evolve our notion of a package ABI revision. For example, if
we wanted to support multiple ABI revisions in a package, we could add
`meta/fuchsia.abi/abi-revision-set` to contain all the supported revisions.
We could drop `meta/fuchsia.abi/abi-revision` once all users have migrated to
the new file.

### Storage overhead

The overhead of adding the package ABI revision to the meta adds 8KiB
uncompressed to the meta.far in the worst case, where the first 4KiB comes from
the FAR data chunk, and the second if the DIRNAMES section causes the first
content chunk to be pushed out to the next 4KiB aligned offset. However, this
shouldn't be a significant impact in practice, because [blobfs] has an 8KiB
block alignment. Furthermore, it compresses very well with the default
compressor [zstd]. According to the following example, this only adds 47 bytes
to the meta.far:

```
# Create an empty package.
% mkdir empty && cd empty
% pm init && pm build
% ls -l meta.far
-rw-r--r--  1 etryzelaar  primarygroup  12288 Oct  5 10:21 meta.far
% fx chunked-compress c meta.far meta.far.compressed
Wrote 447 bytes (97% compression)

# Then create a meta.far that contain's the abi-revision.
% cd .. && mkdir with-abi && cd with-abi
% pm init
% mkdir meta/fuchsia.abi
% python3 -c "
import struct;
abi_revision = int('0xC7003BF9', 16)
f = open('meta/fuchsia.abi/abi-revision', 'wb')
f.write(struct.pack('<Q', abi_revision))
"
% pm build
% ls -l meta.far
-rw-r--r--  1 etryzelaar  primarygroup  16384 Oct  5 10:22 meta.far
% fx chunked-compress c meta.far meta.far.compressed
Wrote 494 bytes (97% compression)
```

## Implementation

All package building tools will be extended to support specifying the API level
or the ABI revision on the command line.

### pm CLI

The `pm` CLI will be extended to allow the ABI to be specified during package
initialization and package building.

Example invocations:

```
# Create a package with an API level. Under the covers this will look up the
# ABI revision from the SDK.
% pm init --api-level 5

# Create a package with an ABI revision. These commands are equivalent.
% pm init --abi-revision 0xC7003BF9
% pm init --abi-revision 3338681337

# Build a package with an ABI revision.
% pm build --abi-revision 0xC7003BF9
```

Options:

* `--api-level LEVEL`: The API level to encode into the package. This value
  must be supported by the SDK. This option conflicts with the `--abi-revision`
  flag.
* `--abi-revision REVISION`: The ABI revision which will be baked into the
  package. This may be a decimal or hexadecimal integer, which must be
  supported by the SDK. This option conflicts with the `--api-level` flag.

Initially the `--api-level` and `--abi-revision` flags will be optional to
allow for petals and end-developers to implement support over a period of time.
Eventually this will be a required argument once the ecosystem has transitioned
over to specifying the flags.

## Performance

This RFC introduces a trivial amount of additional work when building packages,
and a small overhead to the storage, so the performance implications should be
negligible.

## Ergonomics

By itself this proposal does not significantly change the ergonomics of Fuchsia.
However, this will eventually enable product developers to target a specific
system interface for their components. This should provide the stability they
need while not blocking Fuchsia's ability to evolve.

## Backwards compatibility

This change is backwards compatible, since this design introduces a new file
with no consumers. However, this functionality will eventually allow the system
to deprecate and eventually remove support for old ABI revisions.

## Security considerations

The `meta/fuchsia.abi/abi-revision` file will need to be parsed. However, this
format is simple to parse, and should be easy to validate that parsers are
correct.

See [RFC-0002 Security Considerations] for more details.

## Privacy considerations

This proposal has no meaningful impact on privacy.

## Testing

The packaging tooling will be extended to verify that packages are generated
with the expected ABI to be set. Since there will be a transition period, the
package resolver tests will be extended to verify that it can work with packages
that contain or not contain the `meta/fuchsia.abi/abi-revision` file.

## Documentation

The [packaging documentation] will be updated to discuss how the ABI is
expressed in packages, and how users can select the API level or the ABI
revision when building packages.

## Drawbacks, alternatives, and unknowns

### Alternative ABI revision file formats

Rather than encoding the ABI revision as a little endian integer, we could
instead use:

* a human readable integer string
* JSON
* Persistent FIDL

A little endian integer was selected because:

* We don't expect humans to be conuming this value. Instead they should be
  referring to the human-understandable API level.
* This value will be passed around as a little endian integer, so this avoids
  extra transformations.
* It is simple to parse.

### Packages containing a set of ABI revisions

[RFC-0002 Applications] states that packages can include multiple components,
each of which targets a different ABI revision. To support this, rather than
encoding a single ABI revision in the package, the package ABI could be a set of
all the component ABI revisions.

However, as of writing there are no use cases that require a package that
contains components that target different system ABIs. A single number
simplifies the initial implemation for ABI support. It would not be difficult
to extend this design if this scenario becomes important.

### Encoding the ABI revision directly into the meta.far

The meta.far could directly embed the ABI revision, by either:

* Add the ABI into the [FAR index chunk].
  * Pros:
    * Only adds 8 bytes of overhead.
  * Cons:
    * The index chunk does not have any reserved bytes, so adding this to the
      index would require a breaking change to the FAR format. In order to make
      this change, we would have to first implement support for reading the new
      FAR format without using it, then create a stepping stone release, and
      finally then migrate to the the format.
    * It would be more difficult for consumers to read the ABI revision. The
      package resolver does not directly expose the meta.far to users as a file,
      so it would need to add an API to expose this information.
* Create a new ABI revision chunk type.
  * Pros:
    * It should be backwards compatible to add new chunk types.
    * The overhead in bytes should be much smaller since we wouldn't need to
      create a 4096 byte aligned content chunk. Chunks are 64-bit aligned, and
      have an index overhead entry size of 24 bytes, so this approach should
      only add 32 bytes to the uncompressed FAR format.
  * Cons:
    * We haven't added new chunk types to the FAR format, so it's possible FAR
      libraries may error out or behave unexpectedly if they encounter a new
      chunk type.
    * We would have to update all the FAR libraries to understand the new chunk
      type. Historically any changes to FAR libraries has been quite expensive
      to perform.
    * It would be more difficult for consumers to read the ABI revision. The
      package resolver does not directly expose the meta.far to users as a file,
      so it would need to add an API to expose this information.

Since the storage overhead is minimal, especially with compression, we do not
think the space savings are worth defining a new chunk type.

### ABI revision in the repository metadata

Rather than directly encoding the ABI into packages, we could instead add ABI
revision to the package metadata in the package repository. This has the
following cons:

* The ABI revision is an intrinsic property of the package. It could be
  difficult to make sure the ABI revision is properly expressed in the TUF
  repositories, or in the base package lists.
* The packaging metadata would still contain the ABI revision, so the packaging
  system would still have to carry the overhead.

### ABI revision in the upcoming persistent FIDL package metadata

Software Delivery is writing an RFC that proposes a new [persistent-FIDL
packaging metadata]. We could instead store the ABI revision in this metadata by
updating the proposed schema to this:

```
flexible union Contents {
    1: ContentsV1 v1;
};

table ContentsV1 {
    1: vector<fuchsia.io.Path>:MAX paths;
    2: vector<fuchsia.pkg.BlobId>:MAX hashes;
    3: vector<uint64>:MAX blob_sizes;
    4: uint64 abi_revision;
};
```

Advantages:

* The advantages of consolidating all packaging metadata into a single FIDL file
  is that we leverage FIDL tooling to keep the documentation and validation up
  to date. Using multiple files would make it easier for our documentation and
  validation to drift out of sync with each other.
*   We would only add a handful of bytes to the uncompressed meta.far, rather
    than ~4KiB.

The disadvantages of using FIDL:

* There is no guarantee that the persistent-FIDL based RFC will be accepted,
  and how long it will take to implement.
* While the persistent-FIDL based approach is versioned, the internal version
  would not help us migrate the packaging metadata to another file format. For
  example, if we decided to switch from persistent FIDL to a JSON file. It
  would be easier to use a new ABI revision to inform the packaging system to
  look for a different file format than before.

## Prior art and references

### Android

Android applications target an SDK version by specifying the
[uses-sdk] element in the
app manifest.

### Windows

Windows applications target the OS version by specifying the [SupportedOS]
GUID listed in the application manifest.

### macOS

macOS applications target the OS version by specifying the
[LSMinimumSystemVersion] in the application bundle's `Info.plist` file.

### iOS

iOS applications target the OS version by specifying the [MinimumOSVersion] in
the application bundle's `Info.plist` file.

[ABI revision]: 0002_platform_versioning.md#abi_revision
[API level]: 0002_platform_versioning.md#api_level
[FAR index chunk]: development/source_code/archive_format.md#index_chunk
[Fuchsia System Interface]: concepts/packages/system.md
[LSMinimumSystemVersion]: https://developer.apple.com/documentation/bundleresources/information_property_list/lsminimumsystemversion?language=objc
[MinimumOSVersion]: https://developer.apple.com/documentation/bundleresources/information_property_list/minimumosversion?language=objc
[RFC-0002 Applications]: 0002_platform_versioning.md#applications
[RFC-0002 Platform]: 0002_platform_versioning.md#platform
[RFC-0002 Security Considerations]: 0002_platform_versioning.md#security-considerations
[RFC-0002]: 0002_platform_versioning.md
[SDK version history]: https://fuchsia.googlesource.com/fuchsia/+/main/sdk/version_history.json
[SupportedOS]: https://docs.microsoft.com/en-us/windows/win32/win7appqual/compatibility---application-manifest#leveraging-feature-capabilities
[blobfs]: concepts/filesystems/random-access-compression.md
[meta.far]: concepts/packages/package.md#meta-far
[package]: concepts/packages/package.md
[packaging documentation]: concepts/packages/package.md
[persistent-FIDL packaging metadata]: https://fuchsia-review.googlesource.com/c/fuchsia/+/586937
[system image]: 0072_standalone_image_assembly_tool.md#system_image_artifacts
[uses-sdk]: https://developer.android.com/guide/topics/manifest/uses-sdk-element
[zstd]: https://github.com/facebook/zstd
