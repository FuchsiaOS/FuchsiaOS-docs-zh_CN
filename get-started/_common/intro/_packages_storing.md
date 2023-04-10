## Storing packages

On the device, package BLOBs are stored in a content-addressable filesystem
optimized for write-once, read-often files called `blobfs`. This allows them
to be **de-duplicated** across all packages and
**cryptographically verified** using their hash. Fuchsia runs the `pkg-cache`
service on top of `blobfs` to facilitate package management.

![Diagram illustrating how the package cache is built on top of "blobfs" — a
 content-addressable filesystem that de-duplicates BLOBs allowing them to be
 shared between packages.](/docs/get-started/images/intro/blobfs.png){: width="632"}

The `pkg-cache` layer tracks which packages in the system are currently
active. **Packages are not explicitly installed or removed in Fuchsia**.
Software is delivered on demand and likewise space can be reclaimed from
packages that are no longer active through periodic **garbage collection**.
When `pkg-cache` triggers garbage collection to reclaim space, content BLOBs
not referenced by any active package are deleted.

Note: For more of the technical details on Fuchsia packages and software
delivery, see the [package documentation](/docs/concepts/packages/package.md).
