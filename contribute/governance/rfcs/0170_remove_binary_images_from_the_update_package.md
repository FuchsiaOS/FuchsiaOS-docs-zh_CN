<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0170" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

To reclaim space on the system, we must split up the update package. On at least
one space constrained product, we'll save ~14MiB. It is a non-trivial change
that requires a stepping stone release. According to [RFC 103][rfc-103], all
stepping stone releases require their own RFC. This RFC details the new update
package format.

[rfc-103]: /docs/contribute/governance/rfcs/0103_software_delivery_rfc_criteria.md#changes_which_require_an_rfc

## Motivation

An [Over-The-Air (OTA)](/docs/concepts/packages/ota.md) update is the mechanism
for upgrading the version of Fuchsia on a running device. If an update is
available, the system-updater will fetch the
[update package](/docs/concepts/packages/update_pkg.md). To fetch a package
means that the contents of the package are written to BlobFS and protected from
garbage collection. The update package contains the images (like the recovery
image and Zircon Boot Images) that also have reserved space on the Zircon
partitions, and a list of other packages to download to complete the update.

Currently Fuchsia devices must store two copies of each image:

1.  One copy in the destination partition (for example, ZIRCON_A) which the
    device uses at runtime.
2.  One copy in blobfs, because images are delivered to the device as blobs
    inside the update package.

By protecting the images from garbage collection, the update guarantees forward
progress if disrupted. Forward progress is an extremely important property to
guarantee for an update system. However, on a space-constrained product, writing
the images to both the disk partition and BlobFS is suboptimal budgeting.

Image writing is the [penultimate step][write-images] in the OTA process before
switching which partition is the active partition and rebooting into the new
system image. Until the next update, the kernel, firmware, and recovery images
are protected from being garbage collected and deleted. By reordering image
writing during an OTA, we can garbage collect the images from BlobFS before we
download the majority of packages in the OTA, and reclaim the space budget for
use by those other packages. Changing the SWD design to remove the duplicate
copy of images has the potential to save a significant amount of space, which is
at a premium on some Fuchsia devices.

[write-images]: /docs/concepts/packages/ota.md#write-images-block-device

In order to garbage collect our binary images during an OTA while still
guaranteeing forward progress, we need to make a change to the format of the
update package.

## Stakeholders

*Facilitator:* hjfreyer@google.com

*Reviewers:*

*   Software Delivery: wittrock@google.com, jsankey@google.com
*   MOS: gtsai@google.com
*   Security: ampearce@google.com
*   Product Assembly: awolter@google.com
*   Release: billstevenson@google.com

## Design

Currently the update package is a package that also contains images that get
fetched and written to blobfs when the update package is fetched.

We propose to pull the images out of the update package and put each image in
its own [package](/docs/concepts/packages/package.md).

This fits cleanly with our current OTA process and package format, but does
require a change to the update package format.

To reference these new packages, we'll add a file to the update package called
images.json which contains metadata describing the image packages. An example of
that file is:

```

{
  "version": "1",
  "contents": {
    "partitions": [
      {
        "type": "zbi",
        "slot": "fuchsia",
        "size": 1,
        "hash": "0a",
        "url": "fuchsia-pkg://fuchsia.com/fuchsia-zbi/0?hash={merkle_hash}#path/to/fuchsia.zbi"
      },
      {
        "type": "vbmeta",
        "slot": "fuchsia",
        "size": 2,
        "hash": "0b",
        "url": "fuchsia-pkg://fuchsia.com/fuchsia-vbmeta/0?hash={merkle_hash}#path/to/fuchsia.vbmeta"
      },
      {
        "type": "zbi",
        "slot": "recovery",
        "size": 3,
        "hash": "0c",
        "url": "fuchsia-pkg://fuchsia.com/recovery-zbi/0?hash={merkle_hash}#path/to/recovery.zbi"
      },
      {
        "type": "vbmeta",
        "slot": "recovery",
        "size": 4,
        "hash": "0d",
        "url": "fuchsia-pkg://fuchsia.com/recovery-vbmeta/0?hash={merkle_hash}#path/to/recovery.vbmeta"
      }
    ],
    "firmware": [
      {
        "type": "",
        "size": 5,
        "hash": "0e",
        "url": "fuchsia-pkg://fuchsia.com/update-images-firmware/0?hash={merkle_hash}#path/to/firmware"
      },
      {
        "type": "bl2",
        "size": 6,
        "hash": "0e",
        "url": "fuchsia-pkg://fuchsia.com/update-images-firmware/0?hash={merkle_hash}#path/to/firmware"
      }
    ]
  }
}

```

The version property defines how the contents property should be interpreted.
Version must always be "1" when using the format defined by this RFC but
introducing a version property now simplifies additional changes that may be
required in the future. This pattern has been used elsewhere in the SWD stack's
manifests and integrates well with serde.

The system-updater will parse the manifest to determine if the images need to be
fetched (based on whether files with the corresponding hashes are already on the
appropriate slot). For each image that has changed, then it will be fetched,
written to its partition, and then garbage collected from BlobFS. If an image is
not present in images.json, then we do not overwrite what is present on the
zircon partition.

The size and hash of the image are included for verification checking. The hash
is a SHA256 hash of the image file represented in hex. As partitions are
variable across devices, we also need to know the size of the images for
comparison. The url has the merkle hash. Merkle hashes are more complex to
compute which is why the SHA256 hash is chosen for doing faster comparisons.

The process we propose for an OTA is:

1.  Download the update package
2.  Parse the new metadata file containing the update images package references
3.  For each image which is listed in that file, if the image is the same as the
    image in the designated Zircon partition on the non-active partition,
    continue. The metadata file contains the hash and size of the image (as the
    image size is not equal to the partition size) and we can quickly compare to
    the hash of the image on the non-active partition. Else:
    1.  Fetch the package containing the image which will write the image to
        BlobFS and will handle integrity checking. Add the package to the
        retained index.
    2.  Write to the partition.
    3.  Garbage collect (by removing the package from the retained index) from
        BlobFS in order to reclaim space.
4.  Proceed with downloading the rest of the packages specified in the update
    package, and finish the OTA.

Changing the structure of update packages allows us to solve the space
constraint problem. Writing to BlobFS and then garbage collecting allows us to
make use of the already comprehensive security guarantees provided by our
current storage architecture.

## Implementation

To make this change to the update package, we must have a three phase release:
first to handle a superset of the current update package's format and the new
update package's format, and a second release to produce just the new format,
and a final release to stop handling logic for the old version of the update
package.

In the first phase system_updater will be modified to successfully parse both
the original update package format and the modified format proposed in this RFC.
MOS will still produce update packages using the original format. This release
containing this work will be marked as a stepping stone, ensuring all Fuchsia
devices receive a system_updater that is capable of parsing the new format
before they receive an update package that uses the new format.

In the second phase MOS will begin producing update packages using the new
format proposed by this RFC.

In the third phase, once we are confident no devices will need to roll back to a
release that used the original update package format, system_updater will be
modified to remove support for the original update package format.

If we do not stage the release, devices that can only interpret the current
version of the update package will be bricked if they receive the updated update
package. We will need to mark the first phase release as a 'stepping stone'
build to ensure that all devices pass through that build.

Users of the update package will need to be aware of the staged release. Known
users are Security vis a vis Scrutiny, MOS, Product Assembly, and Software
Delivery.

## Performance

No significant change expected.

We will need to take a hash of the images and compare. In the best case, the
hashes match and we do not need to spend time fetching or writing them. In the
worst case where all images change, we still need to download and write the same
number of bytes.

## Security Considerations

Scrutiny (our build-time security analysis tool) analyzes the update package to
extract the ZBI from it. We will need to update Scrutiny tests to reflect the
new location of the ZBI in the update package.

The integrity checking of the images does not change. We will continue to use
the same method for fetching the update package, and the update package contains
the hashes of the image packages and all other security properties are enforced
by verified boot when the device reboots into the new system.

## Privacy Considerations

This RFC does not introduce any changes to the creation or content of images,
only the order in which they are delivered, and therefore does not impact
privacy.

## Testing

We already have unit and end-to-end integration tests for the update package and
the system-updater. We need to extend those tests to cover going from the
current version of the update package to the intermediate version for the first
stepping stone release. For the second release, we need to have tests that
handle both the intermediate version of the update package and the new version
of the update package. When the work is completed, we will remove the
intermediary tests and test that a downgrade OTA with the old format of the
update package will always fail.

## Documentation

We'll need to update the update package
[documentation](/docs/concepts/packages/update_pkg.md) and
[OTA docs](/docs/concepts/packages/ota.md) should this change be approved.

## Drawbacks, Alternatives, Unknowns

The alternatives are designs which do not write images to BlobFS at all.

The naive approach would be pave the images directly to their partitions,
garbage collect the update package from blobfs, and finally download the new
package blobs that were part of the retained index. This alternative is simple
to implement, avoids duplicate writes, and does not require a stepping stone
release. However, we would no longer guarantee forward progress. If an update is
interrupted, there is a chance that the device could fail to update at all.

There is an alternative in which we keep images in the update package, but treat
the update package as even more special than it already is: we could avoid
saving images to blobfs at all. This design would remove the need for format
changes to the update package, but would require extensive changes to the
system-updater logic, and diverge the handling of the update package from the
handling of 'normal' packages. We believe the proposed design simply refactors
the update package, rather than introducing special handling logic.

## Prior Art

The design of the update package was
[previously documented](/docs/concepts/packages/update_pkg.md) on fuchsia.dev.
