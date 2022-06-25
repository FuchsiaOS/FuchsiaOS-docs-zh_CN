Fuchsia software is delivered **on demand** to the system through **packages**.
This is a critical component to Fuchsia's core design principles of
**security** and **updatability**. Packages can be updated independently and
delivered on demand, like a web page. This enables a vulnerability patch to be
pushed to all Fuchsia products at once without the need for individual product
coordination.

A package is not a single archive or image file, but rather a tree of **Binary
Large Objects** (BLOBs). The root of the tree is a BLOB called "meta.far" which
contains metadata for the package, including a "meta/contents" file which
contains references to the rest of the BLOBs. The BLOBs inside Fuchsia packages
are **content-addressed**, meaning they are referenced using a hash of their
contents. The content address of the meta.far itself is known as the **package
hash**.

<aside class="key-point">
  </b>Merkle Roots</b>
  <p>Content addresses are computed as the root hash of a Merkle tree. This is
  a common technique for verifying the integrity of content in transit and on
  disk. This allows Fuchsia to verify the contents of an entire package using
  just the package hash!</p>

  <p>To learn more about Merkle roots and how they are used in Fuchsia, see
  <a href="/docs/concepts/packages/merkleroot.md">Fuchsia package hash</a>.</p>
</aside>

The meta.far contains a `meta/` directory with at least the
following two items:

* `meta/package`: JSON file containing the package's identity information
  such as name and version.
* `meta/contents`: A map of the human-readable file names in a package to
  their content addresses.

![Diagram showing the contents of a Fuchsia package consisting of "meta.far"
metadata and a collection of content BLOBs.]
(/docs/get-started/images/intro/meta-far.png){: width="544"}

If two or more packages share the same content (such as a library dependency,
or font resource), their metadata will point to the same content address for
that resource. This enables Fuchsia to optimize package distribution and
storage by avoiding the need to fetch and save a content BLOB that already
exists.
