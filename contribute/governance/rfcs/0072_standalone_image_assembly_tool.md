{% set rfcid = "RFC-0072" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!--
*** This should begin with an H2 element (for example, ## Summary).
-->

## Summary

This document proposes a standalone tool, suitable for inclusion in a Fuchsia
SDK, for assembling Fuchsia "[system images](#system-image-artifacts)" from
packages and other compiled artifacts.

## Motivation

Product assembly, the process of creating a set of artifacts for delivering
Fuchsia to a device out of the components and packages that are in the Fuchsia
platform [fuchsia.git] and the product's own, and other repo(s), should be
possible without using a complete `fuchsia.git` checkout and build.

This tool performs the final step of the product assembly process, the creation
of the system artifacts, from previously built inputs.

This tool is also used as part of the `fuchsia.git` build itself, replacing
many of the scripts and GN templates that are in
`//build/resources/BUILD.gn`.

## Glossary

### Assembly
The creation of the final system output files that can be used to deliver
Fuchsia to a device.

### `base` Package
The package in BlobFS, identified to `pkgfs` by its content identity, which
contains the base system, (e.g. `/system`).  This is called the `system_image`
package in the current build.  See [this][pkgfs-cmd] for more information.

### BlobFS
[BlobFS][blobfs-docs] is Fuchsia's [content-addressed](#content-addressing)
filesystem.

### Board Support Data
These are the inputs that describe the low-level hardware details needed to
perform the assembly process (e.g partition tables, flash-block
sizes, device bootloader images for flashing onto the device or including in the
update package, etc.).

### Content-Addressing
Content-addressing is a way of identifying a thing based on a
cryptographically-secure hash of its contents.  On Fuchsia, this is used by
[BlobFS](#blobfs), [pkgfs][pkgfs], and other parts of the system to identify
files in a secure manner.

### FVM Image
The FVM image is the block-device image for the
[Fuchsia Volume Manager](/docs/glossary/README.md#Fuchsia-volume-manager).

### System Image Artifacts
The set of final output artifacts that are created by the build which contain
Fuchsia.  This is the larger set of artifacts which are used to deliver Fuchsia
to devices via different means (OTA, flashing, paving, etc.)

### `update` Package
The package that contains files and rules for updating the system.  See
[this][update-pkg] for more information.

### `vbmeta` Image
The verified-boot metadata is used by the bootloader(s) running on the device to
validate that the zbi is trusted for execution by the bootloader.

### ZBI
The [ZBI](/docs/glossary/README.md#zircon-boot-image) is the Zircon Boot Image.  This is the kernel and the ramdisk
[bootfs](/docs/glossary/README.md#bootfs).  This contains everyting needed to
[bootstrap Fuchsia][bootstrap].

## Design

This tool is primarily a replication of steps and processes that are currently
performed by `//build/resources/BUILD.gn` and its associated
scripts.

The tool is an [`ffx`][ffx] plug-in that allows it to be used both as part of the
fuchsia build, as well as outside of it.

### Inputs

Operationally, the tool takes as input:

A set of options specifying which image files it should create:

- The `base` package
- The ZBI
- The vbmeta image
- The `update` package
- The flash images:
  - BlobFS block device image
  - [FVM](/docs/concepts/filesystems/filesystems.md#fvm) block device image
- Final [image manifest](#results-manifests)


The following diagrams show which inputs (and outputs) are used to create the
various final outputs:

```
┌─────────┐┌────────┐┌────────┐┌───────────┐┌─────────────────┐
│  Board  ││ Kernel ││ Kernel ││  List of  ││     List of     │
│ Support ││ Image  ││  Args  ││  BootFS   ││    packages     │
└┬──┬─────┘└┬───────┘└┬───────┘│   Files   │└┬───────────────┬┘
 │  │       │         │        └┬──────────┘ │               │
 │  │       │         │        ┌V───────┐   ┌V─────────────┐ │
 │  │       │         │        │ BootFS │   │ base package │ │
 │  │       │         │        └┬───────┘   └┬──┬──────────┘ │
 │  │      ┌V─────────V─────────V────────────V┐ │            │
 │  │      │ZBI                               │ │            │
 │  │      └┬───────┬─────────────────────────┘ │            │
 │  │      ┌V─────┐ │                           │            │
 │  │      │VBMeta│ │                           │            │
 │  │      └┬─────┘ │                           │            │
 │ ┌V───────V───────V───────────────────────────V────────────V┐
 │ │ update package                                           │
 │ └┬──┬──────────────────────────────────────────────────────┘
 │  │ ┌V───────┐
 │  │ │ BlobFS │
 │  │ └┬───────┘
┌V──V──V┐
│  FVM  │
└───────┘
```
Note:  The update package itself is not placed in the FVM, but is a container
which contains many of the inputs used to create BlobFS and FVM, which makes it
a good intermediate from which to create the block-device image files

To create the `base` package, the tool needs:

- The list of package files to incorporate into the system (packages in the the
  "base" and "cache" package sets)


To create the ZBI, the tool needs:

- A list of files (with access to them) to incorporate into bootfs
- The `base` package's "[content identity][merkle-roots]"
- A [Zircon][zircon] kernel image to place in the ZBI
- The command line arguments to pass to the Zircon kernel


To create the vbmeta image, the tool needs:

- The ZBI


To create the update package, the tool needs:

- The list of packages incorporated into `base`
- The `base` package
- The ZBI
- The vbmeta image
- The ZBI for the recovery slot (optional)
- The vmbeta image for the recovery slot (if recovery zbi is given)
- The bootloader firmware images


To create the flashable block device images, the tool needs:

- Board Support Data:
  - partition table
  - etc.
- and either:
  - an `update` package
  - or directly provide:
    - bootloaders
    - vbmeta
    - ZBI
    - packages for BlobFS

### Outputs

The assembly tool generates the following outputs, depending on which it is
instructed to create:

#### Packages

- `base`
- `update`

#### Image Files

- blobfs block device image
- ZBI
- vbmeta
- fvm flash image

#### Results Manifests

The following manifests are produced, when the tool is instructed to create the
output files that are described within the manifests.

- Manifest of all packages (including `base` and `update`)
- Manifest of all image files produced, containing:
  - The content-identity hash of the image
  - The architecture the images are for
  - For all files incorporated into the image:
    - Their own content-identity
    - The origination (file-path) of that file

### Inputs and Schemas

For build-tool compatibility, inputs will initially be what GN produces.  For
example, the results of the metadata walks that are used to describe all
packages that are being built.

See:

- [/build/package.gni][build_package_gni] at line 604.
- [/build/resources/BUILD.gn][build_images_BUILD_gn] at line 221.

## Implementation

The final tool will be constructed from:

- An [ffx plugin] allowing it to be used via `ffx`.
- A Rust library containing the majority of the implementation and unit-tests.
- GN template for correctly using the tool in the fuchsia.git build.
- The existing tools, packaged for its use:
  - [`pm`](/src/sys/pkg/bin/pm)
  - [`zbi`](/zircon/tools/zbi)
  - [`avbtool`](/third_party/android/platform/external/avb/avbtool.py)
  - [`blobfs`](/zircon/tools/blobfs)
  - [`fvm`](/src/storage/bin/fvm)

To facilitate the transition for the fuchsia.git in-tree build, there will also
be:

- A CLI tool that exposes specific functionality as-needed for having a smooth
transition from the existing GN templates and scripts to the new tool.
- Updated GN templates to wrap that functionality.

The above transitional tools will not be part of the permanent interface for the
tool, but used to provide a transition path that minimizes risk to the
fuchsia.git build.

### Soft Launch Plan

To mitigate risks, the tool will be integrated into fuchsia.git carefully:

- As an integration test that logs if its output does not match that of the
existing scripts and tools
- Then that test becomes a failing test
- After rolling and a few days of baking against numerous CQ builds, the new
tools replace the existing

## Backwards Compatibility

The addition of this tool to the SDK doesn't change any existing backwards
compatibility concerns, as it is a different way of using existing tools that
currently exist in the SDK.  The restriction where the kernel and drivers
used with the tool(s) should match the SDK tools (or be newer than the tooling)
remains.

## Performance

Use of the CLI tool has a neglible impact to the speed of the Fuchsia build.
While it moves a number of operations from Python to Rust, it also adds itself
as a compilation step that must be performed.

The existence of the tool allows for the re-assembly of a different set of
components without performing a full build of Fuchsia itself.

## Security considerations

The output manifests allow for auditing of the contents and provenance of the
image artifacts produced.

## Privacy considerations

No privacy concerns.

## Testing

The core library will have unit tests which covers:

- input validation
- schema parsing
- schema generation
- each step of the assembly process's operation:
  - generated command lines for external tools that are run
  - parsing the output of external tools
  - correct generation and parsing of intermediate files

## Documentation

The `ffx` interface for this tool will need to be documented.

[blobfs-docs]: /docs/concepts/filesystems/blobfs.md
[bootstrap]: /docs/concepts/process/everything_between_power_on_and_your_component.md
[build_images_BUILD_gn]: https://fuchsia.googlesource.com/fuchsia/+/7461d8882167e7a9d1b494e3b1734d2c063830fc/build/resources/BUILD.gn#221
[build_package_gni]: https://fuchsia.googlesource.com/fuchsia/+/7461d8882167e7a9d1b494e3b1734d2c063830fc/build/package.gni#604
[fuchsia.git]: https://fuchsia.googlesource.com/fuchsia/
[ffx]: /docs/development/tools/ffx/overview.md
[ffx plugin]: /docs/development/tools/ffx/development/plugins.md
[merkle-roots]: /docs/concepts/packages/merkleroot.md
[pkgfs]: /docs/concepts/packages/garbage_collection.md
[pkgfs-cmd]: /docs/reference/kernel/kernel_cmdline.md#zirconsystempkgfscmdcommand
[update-pkg]: /docs/concepts/packages/update_pkg.md
[zircon]: /docs/concepts/kernel/README.md
