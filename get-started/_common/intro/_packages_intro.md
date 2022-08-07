<!-- Fuchsia software is delivered **on demand** to the system through **packages**.
This is a critical component to Fuchsia's core design principles of
**security** and **updatability**. Packages can be updated independently and
delivered on demand, like a web page. This enables a vulnerability patch to be
pushed to all Fuchsia products at once without the need for individual product
coordination. -->
Fuchsia 软件通过**包****按需**交付到系统中。这是 Fuchsia 的**安全性**和**可更新性**核心设计原则的关键组成部分。包可以单独更新和按需交付，就像网页一样。这使得漏洞补丁可以一次性推送到所有的 Fuchsia 产品中，而无须针对各个产品单独协调。

<!-- A package is not a single archive or image file, but rather a tree of **Binary
Large Objects** (BLOBs). The root of the tree is a BLOB called "meta.far" which
contains metadata for the package, including a "meta/contents" file which
contains references to the rest of the BLOBs. The BLOBs inside Fuchsia packages
are **content-addressed**, meaning they are referenced using a hash of their
contents. The content address of the meta.far itself is known as the **package
hash**. -->
包不是单一文件或镜像，而是一棵**二进制大对象**(BLOBs)树。树根是一个名为“meta.far”的 BLOB，其中包含了该包的元数据，包括一个含有对其它 BLOB 引用信息的“meta/contents”文件。Fuchsia 包中的 BLOB 是**内容寻址**的，这意味着它们是使用其内容的哈希值来引用的。meta.far 本身的内容地址被称为**包哈希**。

<aside class="key-point">
  <!-- </b>Merkle Roots</b> -->
  <b>哈希根</b>
  <!-- <p>Content addresses are computed as the root hash of a Merkle tree. This is
  a common technique for verifying the integrity of content in transit and on
  disk. This allows Fuchsia to verify the contents of an entire package using
  just the package hash!</p> -->
  <p>内容地址被计算成哈希树的根哈希。这是用于验证传输过程中和磁盘上内容完整性的常用技术。这允许 Fuchsia 仅使用包哈希来验证整个包的内容！</p>

  <!-- <p>To learn more about Merkle roots and how they are used in Fuchsia, see
  <a href="/concepts/packages/merkleroot.md">Fuchsia package hash</a>.</p> -->
  <p>要学习更多关于哈希根及其在 Fuchsia 中的应用，请参看<a href="/concepts/packages/merkleroot.md">Fuchsia 包哈希</a>。</p>
</aside>

<!-- The meta.far contains a `meta/` directory with at least the
following two items: -->
meta.far 文件包含一个至少含有如下两项内容的 `meta/` 目录：

<!-- * `meta/package`: JSON file containing the package's identity information
  such as name and version.
* `meta/contents`: A map of the human-readable file names in a package to
  their content addresses. -->
* `meta/package`: 含有包名称和版本等身份信息的 JSON 文件。
* `meta/contents`: 一份人可读文件名与其内容地址的映射。

<!-- ![Diagram showing the contents of a Fuchsia package consisting of "meta.far"
metadata and a collection of content BLOBs.] -->
![图表显示了由“meta.far”元数据和内容 BLOB 集合组成的 Fuchsia 包的内容。]
(/get-started/images/intro/meta-far.png){: width="544"}

<!-- If two or more packages share the same content (such as a library dependency,
or font resource), their metadata will point to the same content address for
that resource. This enables Fuchsia to optimize package distribution and
storage by avoiding the need to fetch and save a content BLOB that already
exists. -->
如果两个或多个包共享相同内容（如库依赖或字体资源），则它们的元数据将指向那些相同的内容地址。这使 Fuchsia 可以避免获取和保存已经存在的内容的 BLOB ，优化包的分发和存储。
