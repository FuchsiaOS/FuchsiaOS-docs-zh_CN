# BlobFS
<!-- **BlobFS** is a content-addressable filesystem optimized for write-once,
read-often files, such as binaries and libraries. On Fuchsia, BlobFS is the
storage system used for all software packages. -->

**BlobFS**是针对一次写入、经常读取的文件优化的内容可寻址的文件系统，例如二进制和库文件。在Fuchsia的文件系统中，BlobFS 是一种应用于所有软件包的存储系统

<!-- When mounted, BlobFS presents a single logical directory containing all files
(a.k.a., blobs): -->

装载后，BlobFS显示包含所有文件的单个逻辑目录(a.k.a., blobs):

```
blob/
 ├── 00aeb9b5652a4adbf630d04a6ca22668f9c8469746f3f175687b3c0ff6699a49
 ├── 01289d3e1d2cdbc7d1b4977210877c5bbdffdbad463d992badc149152962a205
 ├── 018951bcf92091fd5d294cbd1f3a48d6ca59be7759587f28077b2eb754b437c0
 └── 01bad8536a7aee498ffd323f53e06232b8a81edd507ac2a95bd0e819c4983138
```

<!-- Files in BlobFS are:

*   **Immutable**: Once created, a blob cannot be modified (except removal).
*   **Content-Addressable**: Blob names are deterministically derived from their
    contents.
*   **Verified**: Cryptographic checksums are used to ensure integrity of blob
    data.

These properties of blobs make BlobfS a key component of Fuchsia's security
posture, ensuring that software packages' contents can be verified before they
are executed. -->
BlobFS 中的文件：
*   **Immutable**：创建后，Blob不可以被修改（除了移动）
*   **Content-Addressable**：Blob命名来源于内容
*   **Verified**：加密校验和确保数据的一致性

BlobFS中文件的这些特性是Fuchsia安全性的一个关键组成部分，它能确保程序在启动后软件包不被篡改。

<!-- ## Design and implementation of BlobFS -->

## BlobFS的设计与实现

<!-- ### On-disk format -->

### 磁盘格式化

<!-- BlobFS stores each blob in a linked list of non-adjacent extents (a contiguous
range of data blocks). Each blob has an associated Inode, which describes where
the block's data starts on disk and other metadata about the blob. -->

BlobFS将每个Blob存储在非相邻盘区（一个连续的数据块区间）的链表中。每一个blob都有一个关联的Inode,Inode描述数据块在磁盘的开始位置和blob的一些其他元数据。

<!-- BlobFS divides a disk (or a partition thereof) into five chunks:

*   The **Superblock** storing filesystem-wide metadata,
*   The **Block Map**, a bitmap used to keep track of free and allocated data
    blocks,
*   The **Node Map**, a flat array of Inodes (reference to where a blob's data
    starts on disk) or ExtentContainers (reference to several extents containing
    some of a blob's data).
*   The **Journal**, a log of filesystem operations that ensures filesystem
    integrity, even if the device reboots or loses power during an operation,
    and
*   The **Data Blocks**, where blob contents and their verification metadata are
    stored in a series of extents. -->

BlobFS将一个磁盘(或其分区)划分为五个区块：
*   **Superblock** 存储整个文件系统的元数据区块
*   **Block Map** 存放用于寻址和分配数据块的位图的区块
*   **Node Map** 存放Inode(引用BLOB数据在磁盘上的起始位置)或ExtentContainers(引用包含某些BLOB数据的多个分区)的二维数组的区块
*   **Journal** 文件系统操作日志区块，即使设备在操作期间重新启动或断电，也可确保文件系统的完整性
*   **Data Blocks** 存储连续BLOB数据的和校验的元数据的区块。
![BlobFS disk layout](images/blobfs-disk-format.svg "disk_format")

<!-- Figure 1: BlobFS disk layout -->

1：BlobFS的磁盘布局

#### Superblock

<!-- The superblock is the first block in a BlobFS-formatted partition. It describes
the location and size of the other chunks of the filesystem, as well as other
filesystem-level metadata. -->

超级块是BlobFS格式化分区中的第一个块。它描述了文件系统的其他块的位置和大小，以及其他文件系统级元数据。

<!-- When a BlobFS-formatted filesystem is mounted, this block is mapped into memory
and parsed to determine where the rest of the filesystem lives. The block is
modified whenever a new blob is created, and (for FVM-managed BlobFS instances)
whenever the size of the BlobFS filesystem shrinks or grows. -->

当BlobFS格式化的文件系统被挂载，该块将会映射到内存中然后进行分析以确定文件系统其余部分的位置。当一个新的blob被被创建时和BlobFS文件系统的大小缩小或增大时，该块会被更新。

![BlobFS superblock](images/blobfs-superblock-layout.svg "superblock_layout")

<!-- Figure 2: BlobFS superblock -->

2：BlobFS 超级块

<!-- When BlobFS is managed by FVM, the superblock contains some additional metadata
describing the FVM slices that contain the BlobFS filesystem. These fields
(yellow in the above diagram) are ignored for non-FVM, fixed-size BlobFS images. -->

当BlobFS由FVM管理时，超级数据块包含一些描述包含BlobFS文件系统的FVM切片的附加元数据。对于非FVM、固定大小的BlobFS映像，这些字段(上图中为黄色)将被忽略。

#### Block map

<!-- The block map is a simple bit-map that marks each data block as allocated or
not. This map is used during block allocation to find contiguous ranges of
blocks, known as _extents_, to store blob contents in. -->

Block map 是一个简单的位图，它标明每一个数据块是否分配。此映射在块分配期间用于查找连续的块范围(称为_extents_)，以在其中存储BLOB内容

![Example block map](images/blobfs-example-block-map.svg "block_map_example")

<!-- Figure 3: An example block-map with several free extents of varying size. -->

3：具有多个大小不同的空闲分区的位图示例

<!-- When a BlobFS image is mounted, the block map is mapped into memory where it can
be read by the block allocator. The block map is written back to disk whenever a
block is allocated (during blob creation) or deallocated (during blob deletion). -->

挂载BlobFS映像时，位图会映射到内存中，块分配器可以在内存中读取它。每当分配块(在BLOB创建期间)或释放块(在BLOB删除期间)时，位图都会写回磁盘。

#### Node map

<!-- The node map is an array of all nodes on the filesystem, which can come in two
variations: -->
Node map 是文件系统上所有节点的数组，它可以分成两个部分：

<!-- *   **Inodes**, which describe a single blob on the filesystem, or
*   **ExtentContainers**, which point to an extent containing part of a blob's
    data. -->

*   **Inodes** 它们描述了文件系统上的单个BLOB
*   **ExtentContainers** 它指向包含BLOB数据部分的范围

<!-- Nodes of both types are stored together in a single flat array. Each node has a
common header that describes what type the node is, and whether the node is
allocated. Both node types are the same size, so there is no internal
fragmentation of the array. -->

节点这两种类型一起存储在一个二维数组。每一个节点都有一个共同的头它描述了节点的类型，以及节点是否被分配。两种类型的节点都是相同的大小，因此没有内部碎片。

##### Inodes

<!-- Each blob in the filesystem has a corresponding Inode, which describes where the
blob's data starts and some other metadata about the blob. -->

文件系统中的每个BLOB都有一个相应的Inode，它描述了BLOB的数据起始位置以及关于该BLOB的其他一些元数据。

![Layout of a BlobFS Inode](images/blobfs-inode-layout.svg "inode_layout")

<!-- Figure 4: Layout of a BlobFS Inode. -->
4:BlobFS的Inode的布局

<!-- For small blobs, the Inode may be the only node necessary to describe where the
blob is on disk. In this case `extent_count` is one, `next_node` must not be
used, and `inline_extent` describes the blob's single extent. -->

对于较小的BLOB，Inode可能是描述Blob在磁盘上的位置所必需的唯一节点。当`extout_count`为1时，不能使用`next_node`，而`inline_extent`描述blob的单个扩展区

<!-- Larger blobs will likely occupy multiple extents, especially on a fragmented
BlobFS image. In this case, the first extent of the blob is stored in
`inline_extent`, and all subsequent extents are stored in a linked list of
ExtentContainers starting at `next_node.` -->

较大的Blob可能会占用多个扩展区，尤其是在分散的BlobFS映像上。在这种情况下，BLOB的第一个区存储在`inline_extent`中，所有后续扩展区都存储在从`next_node`开始的ExtentContainers链表中。

![Format of an Extent](images/blobfs-extent-format.svg "extent_format")

<!-- Figure 5: Format of an Extent (occupying 64 bits). This format is used both in
Inodes and ExtentContainers. -->

5：扩展区的格式（占64位），此格式在inode和ExtentContainers中均可使用

<!-- Note that this representation of extents implies that an extent can have at most
2\*\*16 blocks in it (the maximum value of Extent Size). -->

请注意，这种区段表示意味着一个区段最多可以有2\*\*16个块(区段大小的最大值)。

##### ExtentContainers

<!-- An ExtentContainer holds references to several (up to 6) extents, which store
some of the contents of a blob. -->

ExtentContainer包含对多个(最多6个)分区的引用，它存储BLOB的一些内容

<!-- The extents in an ExtentContainer are logically contiguous (i.e. the logical
addressable chunk of the blob stored in extents[0] is before extents[1]) and are
filled in order. If `next_node` is set, then the ExtentContainer must be full. -->

ExtentContainer分区在逻辑上是连续的并按顺序填写(即，存储在盘区[0]中的BLOB的逻辑可寻址块在盘区[1]之前)。如果`next_node`被设置了，那么ExtentContainer一定就是空的。

![Layout of a BlobFS ExtentContainer](images/blobfs-extentcontainer-layout.svg "extentcontainer_layout")

<!-- Figure 6: Layout of a BlobFS ExtentContainer. -->

6.BlobFS的ExtentContainer布局

<!-- ##### Properties of the node linked-list -->

#### 节点链表的属性

<!-- A blob's extents are held in a linked-list of a single Inode (which holds the
first extent) and zero or more ExtentContainers (each of which holds up to 6
extents). -->

BLOB的范围保存在单个Inode(其中包含第一个分区)的链表中和零个或多个ExtentContainer(每个最多可容纳6个数据区)

<!-- This linked list has the following properties. Violating any of these properties results in blobfs treating the blob as corrupted. -->

该链表具有以下特性，违反这些属性中的任何一个都会导致BlobFS将该BLOB视为已损坏

<!-- *   Extents are logically contiguous:
    *   If Node A precedes Node B in the list, then all extents in Node A have
        lower logical offsets into the blob's contents.
    *   Within a given ExtentContainer, for extents 𝑥 and 𝑦, if 𝑥 < 𝑦, then
        extent 𝑥 has a lower logical offset into the blob's contents than extent
        𝑦.
*   Nodes are packed before a new node is linked. That is, if a Node has a
    non-null `next_node`, then it must be full of extents (*extent for Inodes
    and 6 extents for ExtentContainers).
*   The total number of extents in the linked-list must equal to the Inode's
    `extent_count`.
*   The sum of the size of all extents in the linked-list must equal to the
    Inode's `block_count`.
*   The end of the list is determined based on the `extent_count` in the Inode
    being satisfied. `next_node` in the final node should not be used. -->

*   扩展区在逻辑上是连续的:
    *   如果列表中节点A在节点B之前，则节点A中的所有区段在Blob内容中具有较低的逻辑偏移量
    *   在ExtentContainer中，对于扩展区𝑥和𝑦，如果𝑥 < 𝑦,x中的所有区段在Blob内容中相对于y具有较低的逻辑偏移量
*   在新节点被链接之前先打包。那是因为，如果一个节点是非空的`next_node`，那么它一定占用了整个分区（ExtentContainers的分区是Inodes的分区的6倍）
*   链表上的所有分区的总大小必须和Inode的`block_count`相等
*   链表的末尾是根据满足的inode中的`EXTEND_COUNt`确定的。`next_node`在最后一个未使用的节点中

<!-- ##### Example Node layouts -->

节点布局例子

<!-- This section contains some examples of different ways a blob's Nodes may be
formatted. -->

本节包含设置BLOB节点格式的不同方式的一些示例

<!-- *   [Example: Single-extent blob](#example-single-extent-blob)
*   [Example: Multiple-extent blob](#example-multiple-extent-blob) -->
*   [Example: 单分区 blob](#example-single-extent-blob)
*   [Example: 多分区 blob](#example-multiple-extent-blob)

<!-- ###### Example: Single-extent blob {: #example-single-extent-blob } -->

###### Example: 单分区 blob {: #example-single-extent-blob }

![Example: Single-extent blob](images/blobfs-example-node-1.svg "node_example_1")

<!-- Figure 7: Node layout for a blob stored in a single extent -->

7：blob存储在单分区的节点布局

<!-- ###### Example: Multiple-extent blob {: #example-multiple-extent-blob } -->

###### Example: 多分区 blob {: #example-multiple-extent-blob }

![Example: Multiple-extent blob](images/blobfs-example-node-2.svg "node_example_2")

<!-- Figure 8: Node layout for a blob stored in several extents. Note that a blob's
extents may be scattered throughout the disk. -->

8：blob存储在多分区的节点布局，请注意，BLOB的范围可能分散在整个磁盘上。

<!-- ##### Blob fragmentation -->

##### Blob 分段存储

<!-- A newly created BlobFS image has all of its data blocks free. Extents of
arbitrary size can easily be found, and blobs tend to be stored in a single
large extent (or a few large extents). -->

新创建的BlobFS映像的所有数据块都是空闲的。可以很容易地找到任意大小的段，并且BLOB往往存储在单个较大的段(或几个较大的段)中。

<!-- Over time, as blobs are allocated and deallocated, the block map will become
**fragmented** into many smaller extents. Newly created blobs will have to be
stored in multiple smaller extents. -->

随着时间的推移，随着Blob的分配和释放，块映射将变得**碎片化**，即许多较小的段。新创建的BLOB必须存储在多个较小的范围中。

![A fragmented block map](images/blobfs-fragmentation.svg "fragmentation")

<!-- Figure 9: A fragmented block map. While there are plenty of free blocks, there
are few large extents available. -->

9：分散的块，虽然存在大量的空闲块，但是可用的大块区却很少

<!-- Fragmentation is undesirable for several reasons:

*   **Slower Reads**: Reading a fragmented blob requires chasing pointers in the
    Node Map. This affects both sequential reads and random-access reads
*   **Slower Creation and Deletion:** Creating a blob requires finding free
    extents for it; this takes longer if many small extents must be found.
    Similarly, deleting a fragmented blob requires chasing down and freeing many
    extents.
*   **Metadata Overhead:** Storing fragmented blobs requires more nodes. There
    are a finite number of nodes in the Node Map, which can be exhausted,
    preventing blobs from being created. 

Currently BlobFS does not perform defragmentation.
-->
不采用分段，原因有几个：

*   **读速度慢**: 读取分段的blob需要在节点映射中跟踪指针. 这会影响顺序读取和随机访问读取
*   **创建和删除速度慢:** 创建一个blob需要找到空闲的分区; 如果大量的小分区需要被访问将花费大量时间
    相同的, 删除分散的blob需要遍历和释放许多分区
*   **元数据冗余:** 存储分散的blob需要很多节点，但是节点的数量时有限的，当他被耗尽时将会无法创建blob

目前BlobFS不支持分段 

#### Journal

TODO

#### Data blocks

<!-- Finally, the actual contents of the blobs must be stored somewhere. The
remaining storage blocks in the BlobFS image are designated for this purpose. -->

BLOB的实际内容必须存储在某个地方。BlobFS映像中的其余存储块指定用于此目的。

<!-- Each blob is allocated enough extents to contain all of its data, as well as a
number of data blocks reserved for storing verification metadata of the blob.
This metadata is always stored in the first blocks of the blob. Metadata is
padded so that the actual data always starts at a block-aligned address. -->

每个BLOB都分配了足够的区段来包含其所有数据，以及保留用于存储BLOB的验证元数据的多个数据块。此元数据始终存储在BLOB的第一个块中。元数据会被填充，因此实际数据始终从块对齐的地址开始。

<!-- This verification metadata is called a **Merkle Tree**, a data structure that
uses cryptographic hashes to guarantee the integrity of the blob's contents. -->

该验证元数据称为**Merkle Tree**，一种数据结构，它使用加密哈希来保证Blob内容的完整性。

##### Merkle tree

<!-- A blob's Merkle Tree is constructed as follows (for more details, see
[Fuchsia Merkle Roots](/docs/concepts/packages/merkleroot.md)):

*   Each leaf node is a sha256 hash of a single block's worth of data.
*   Each non-leaf node is a sha256 hash combining its children's hashes.
*   The tree terminates at the level where there is a single sha256 hash. -->

blob的Merkle tree构造如下,见[Fuchsia Merkle Roots](/docs/concepts/packages/merkleroot.md):

*   每个叶子节点都是单个数据块的sha256散列.
*   每个非叶节点都是一个sha256散列，它组合了它的子节点的散列.
*   该树终止于存在单个sha256散列的级别.

<!-- The hash value at the top-most node is known as the **Merkle Root** of the blob.
This value is used as the name of the blob. -->

最顶端节点的哈希值称为BLOB的**Merkle Root**。
该值用于对blob进行命名。

![A simplified example Merkle Tree](images/blobfs-example-merkle.svg "example_merkle")

<!-- Figure 10: A simplified example Merkle Tree. Note that in practice more
information is included in each hash value (such as the block offset and
length), and each non-leaf node is significantly wider (in particular, each
non-leaf node can contain up to 8192 / 32 == 256 children). -->

10：一个简化的Merkle tree示例。请注意，在实践中，每个散列值中包含了更多信息(如块偏移量和长度)，并且每个非叶节点都要宽得多(特别是，每个非叶节点最多可以包含8192/32==256个子节点)。

<!-- ### Implementation of BlobFS -->
### BlobFS的实现

<!-- Like other Fuchsia filesystems, BlobFS is implemented as a userspace process
that serves clients through a FIDL interface. -->

与Fuchsia其他文件系统一样，BlobFS通过实现FIDL接口的用户空间进程为客户端提供服务。
<!--

#### Startup and initialization

TODO

#### Blob lifecycle

##### Creation

TODO

##### Deletion

TODO

## BlobFS and Fuchsia

TODO: Finish this section describing Blobfs' role in the Fuchsia system and its
relationship to other components, such as pkgfs.

-->

