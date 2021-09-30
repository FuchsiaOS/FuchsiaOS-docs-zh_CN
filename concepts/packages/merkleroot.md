<!-- # Fuchsia Merkle Roots -->

# Fuchsia 墨克根

<!-- [Merkle Trees][merkletree] are used in various places in the Fuchsia ecosystem,
including the [FAR Archive Format][far], the Blob Storage Filesystem, and the
[Package Manager][pm]. -->

在 Fuchsia 生态中，许多地方都运用到了[墨克树][merkletree]，包括 [FAR Archive Format][far]，Blob 文件系统和[包管理器][pm]。

<!-- In [Zircon][zircon] `zx-verity` provides an API for application components to
read data from local storage. When retrieving data the integrity of the data is
verified and causing reads to fail when the data has been modified or corrupted.
zx-verity is based on a [Merkle Tree][merkletree], and is derived from a similar
system in [Chrome OS][dmverity]. -->

在 [Zircon][zircon] `zx-verity` 中为应用组件提供了一个从本地存储中读取数据的 API。在检索数据时，将验证数据的完整性，并在数据被修改或损坏时导致读取失败。`zx-verity` 就是基于[墨克树][merkletree]的，并且源自 [Chrome OS][dmverity] 中一个相似的系统。

<!-- All of these implementations share the algorithm documented herein. -->

这些实现全都基于本文所列算法。

<!-- ## Parameters of the Merkle Tree

 * Block size: 8kb, 0 padded.
 * Root digest size: 32 bytes.
 * Hash algorithm: SHA-256.
 * Block digest computation: SHA-256(u64(offset | level) + u32(length) + data) -->

## 墨克树的参数

* 块大小：0 kb，0 填充。
* 根哈希大小：32 字节。
* 哈希算法：SHA-256。
* 块哈希计算公式：SHA-256(u64(offset | level) + u32(length) + data)。

<!-- ## Definitions

The merkle tree contains levels. A level is a row of the tree, starting at 0 and
counting upward. Level 0 represents the level that contains hashes of chunks of
the input stream. -->

## 定义

墨克树含有多个层级，每一层就是树的一行，从 0 开始递增索引。第 0 层表示该层包含输入流分块后的哈希值（树的最底层）。

<!-- Each level contains a number of hashes of the previous level. The hashes within
a level are computed from 8kb blocks from the previous level (or data, if level
0), prepended with a block identity. -->

每一层都包含上一层的哈希值。任一层的哈希值都是根据前一层 8 kb 的块（如果是第 0 层，则是 8 kb 数据）并加上块标识后计算得来。

<!-- A block identity is the binary OR of the starting byte index of the block within
the current level and the current level index, followed by the length of the
block. For level 0, the length of the block is 8kb, except for the last block,
which may be less than 8kb. All other levels use a length of 8kb, even when the
last block is 0 padded. -->

块标识的计算方法是先求出当前层级内该块的起始字节的索引与当前层级索引的逻辑或，之后再加上该块的长度。对于第 0 层，块的长度为 8 kb，除了最后一个块可能会少于 8 kb。其它层级的所有块长度都为 8 kb，即使最后一个块是用 0 填充的。

<!-- ## Computation of a level

 1. Initialize the level with an index, an offset starting at 0, and an empty
    list of hashes.
 2. For each 8kb (or remainder of) of input, compute the next block identity by
    taking the binary OR of the level index and the current offset, followed
    by the length of the input. -->

## 各层级的计算

1. 将该层的索引、偏移量初始化为 0，并构造一个哈希值的空列表。
2. 对于每一个 8 kb 的输入，使用层级索引和当前偏移量的逻辑或并加上输入的长度来计算下一个块的标识。

 <!-- 1. Init a SHA-256 digest, append to it the identity, the input, and if the
    input is shorter than 8kb, a pad of 0 up to 8kb.
 2. Append the output of the digest to the level's list of hashes. Increment
    the offset by the input length. -->

3. 使用块标识、输入计算出一个 SHA-256 值。如果输入小于 8 kb，则用 0 填充至 8 kb。
4. 将输出的哈希值保存到第 1 步中建立的列表中。将偏移量更新为原偏移量加上输入长度。
   
 <!-- 3. Repeat 1-4 until all input is consumed.
 4. If the length of hashes is 32, finish.
 5. If the length of hashes is not 8kb aligned, 0 fill up to an 8kb
    alignment and compute more levels until there is a root level containing a
    single 32 byte digest. -->

5. 重复 1-4 直到处理完所有输入。
6. 如果哈希值的长度为 32 字节，结束计算。
7. 如果哈希的长度不是 8 kb 对齐的，则使用 0 将其填充到 8 kb 对齐。并重复 1-7，直到计算出一个包含 32 字节的摘要的根摘要。

<!-- ## Computation of a root digest

Compute level 0 with the input data. Construct and compute subsequent levels
using the previous level hashes as input data, until a level hashes contains
exactly 32 bytes. This last level contains the root digest of the merkle tree. -->

## 根摘要的计算

使用输入的数据计算出层级 0。使用前一层数据的哈希值作为输入，计算并构建其后的层级，直到某一层的摘要长度正好为 32 字节。这个最终的层级就包含了这棵墨克树的根摘要。

<!-- ## A note about the empty digest

As a special case, when there is no input data, implementations may need to
handle the calculation independently. The digest of the empty input is simply
the SHA-256 of 12 0 bytes, the block identity of a single 0 length block. -->

## 关于空摘要的提示

当没有输入数据时，需要单独处理这样一种特殊情况。空输入的摘要就是简单的去求 12 个 0 字节的 SHA-256 值。（按照前面的公式，块的标识为 64 位，块长度为 32 位，因此一共 12 个字节，并且每一位都是 0）

<!-- ## Example values

 * The empty digest:
 `15ec7bf0b50732b49f8228e07d24365338f9e3ab994b00af08e5a3bffe55fd8b`
 * 8192 bytes of `0xff` - "oneblock"
 `68d131bc271f9c192d4f6dcd8fe61bef90004856da19d0f2f514a7f4098b0737`
 * 65536 bytes of `0xff` - "small"
 `f75f59a944d2433bc6830ec243bfefa457704d2aed12f30539cd4f18bf1d62cf`
 * 2105344 bytes of `0xff` - "large"
 `7d75dfb18bfd48e03b5be4e8e9aeea2f89880cb81c1551df855e0d0a0cc59a67`
 * 2109440 bytes of `0xff` - "unaligned"
 `7577266aa98ce587922fdc668c186e27f3c742fb1b732737153b70ae46973e43`
 * `0xff0080` bytes filled with repetitions of `0xff0080` - "fuchsia"
 `2feb488cffc976061998ac90ce7292241dfa86883c0edc279433b5c4370d0f30` -->

## 根摘要的例子

 * 空输入的摘要：
 `15ec7bf0b50732b49f8228e07d24365338f9e3ab994b00af08e5a3bffe55fd8b`
 * 8192 个字节的 `0xff` - “正好一块”
 `68d131bc271f9c192d4f6dcd8fe61bef90004856da19d0f2f514a7f4098b0737`
 * 65536 个字节的 `0xff` - “一小段输入”
 `f75f59a944d2433bc6830ec243bfefa457704d2aed12f30539cd4f18bf1d62cf`
 * 2105344 个字节的 `0xff` - “超长的输入”
 `7d75dfb18bfd48e03b5be4e8e9aeea2f89880cb81c1551df855e0d0a0cc59a67`
 * 2109440 个字节的 `0xff` - “不是 8 kb 对齐的”
 `7577266aa98ce587922fdc668c186e27f3c742fb1b732737153b70ae46973e43`
 * 重复了 `0xff0080` 次的 `0xff0080` - “去调色板试试？”
 `2feb488cffc976061998ac90ce7292241dfa86883c0edc279433b5c4370d0f30`

[merkletree]: https://en.wikipedia.org/wiki/Merkle_tree "Merkle Tree"
[dmverity]: https://www.chromium.org/chromium-os/chromiumos-design-docs/verified-boot "Chrome OS Verified Boot"
[far]: /docs/concepts/source_code/archive_format.md "Archive Format"
[pm]: /src/sys/pkg/bin/pm/README.md "Package Manager"
[zircon]: /zircon/README.md "Zircon"
