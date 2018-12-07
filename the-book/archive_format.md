<!--
# Fuchsia Archive Format

An archive is a sequence of bytes, divided into chunks:

 * The first chunk is the index chunk, which describes where other chunks are
   located in the archive.
 * All the chunks listed in the index must appear in the archive in the order
   listed in the index (which is sorted by their type).
 * The archive may contain additional chunks that are not referenced in the
   index, but these chunks must appear in the archive after all the chunks
   listed in the index. For example, content chunks are not listed in the
   index. Instead, the content chunks are reachable from the directory chunk.
 * The chunks must not overlap.
 * All chunks are aligned on 64 bit boundaries.
 * All chunks must be packed as tightly as possible subject to their alignment
   constraints.
 * Any gaps between chunks must be filled with zeros.

All offsets and lengths are encoded as unsigned integers in little endian. -->

# Fuchsia 归档格式

一个归档是一个字节序列，分为以下几块：
 * 第一个块是索引块，它描述了存档中其他块的位置。
 * 索引中列出的所有块必须按索引中列出的顺序出现在归档中（按类型排序）
 * 归档中可能包含索引中未引用的其他块，但这些块必须在索引中列出所有块之后出现在存档中。
   例如，内容块在索引中列出。但是，该内容块可以从目录块中获得。
 * 块不能重叠。
 * 所有块都在 64 bit 边界上对齐。
 * 所有块必须根据他们的对齐约束尽可能地紧密打包。
 * 块之间的任何间隙必须用'0'填充。
所有偏移和长度在小字节序中被编码为无符号整数。

<!--
## Index chunk

The index chunk is required and must start at the beginning of the archive.

 * 8 bytes of magic.
    - Must be 0xc8 0xbf 0x0b 0x48 0xad 0xab 0xc5 0x11.
 * 64 bit length of concatenated index entries, in bytes.
 * Concatenated index entries.

No two index entries can have the same type and the entries must be sorted by
type in increasing lexicographical octet order (e.g., as compared by memcmp).
The chunks listed in the index must be stored in the archive in the order listed
in the index. -->

## 序列块

索引块是必需的，且必须从归档的开头开始。
 * 8字节的魔法。
    -必须是 0xc8 0xbf 0x0b 0x48 0xad 0xab 0xc5 0x11。
 * 64位长度的多行索引条目，以字节为单位。
 * 多行索引。

没有具有相同类型的两个索引项，且索引项必须按照 `八位组字典增序`排列（例如，比较 memcmp ）。索引中列出的块必须按照列出的顺序存储在归档中。

<!--
### Index entry

 * 64 bit chunk type.
 * 64 bit offset from start of the archive to the start of the referenced
   chunk, in bytes.
 * 64 bit length of referenced chunk, in bytes. -->

### 索引项

 * 64 bit 块类型。
 * 从归档的开始到引用块的开始，以 64 bit 字节为单位偏移。
 * 64 字节长度的引用块。

<!--
## Hash chunk (Type 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00)

The hash chunk is optional.

 * 32 bit hash algorithm identifier.
    - Must be 1.
    - Identifies "SHA-256".
 * Hash data.
    - 32 bit length of hash.
      - Must be 32.
    - Octets of hash.
      - The result of hashing the contents of archive from the start of the
        archive until the end of the last chunk listed in the index with the
        “octets of hash” replaced with zeros.
      - Note: The hash includes all the chunks in the index but does not include
        any of the content chunks, which are not in the index. If you wish to
        check the integrity of the content chunks, include a directory hash
        chunk.

Note: We might want to replace this hash algorithm with the one we use in
blobfs. -->

## 哈希块 (type 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00)

哈希块是可选的。

 * 32位哈希算法标识符
    - 必须是1。
    - 标识 "SHA-256"。
 * 哈希数据
    - 32位哈希长度。
      - 必须是32。
    - Octets的哈希。
      - 从归档开始到索引末尾列出的最后一个块，用"octets of hash"替换为零的内容。
      - Note:哈希包含索引中的所有块，但不包含任何不在索引中的内容块。如果需要检查内容块的完整性，请加上目录哈希块。

Note:我们可能会将此哈希算法替换为我们在blobfs中使用的哈希算法。

<!--
## Directory chunk (Type "DIR-----")

The directory chunk is required.  Entries in the directory chunk must have
unique names and the entries must be sorted by name in increasing
lexicographical octet order (e.g., as compared by memcmp).

 * Concatenated directory table entries.

These entries represent the files contained in the archive. Directories
themselves are not represented explicitly, which means archives cannot represent
empty directories. -->

## 目录块（类型 "DIR -----"）

目录块是必需的。目录块中的条目必须具有唯一的名称，且必须将名称以"increasing lexicographical octet"对条目进行排序（例如，与memcmp类比）。

 * Concatenated directory table entries.

这些条目表示归档中包含的文件。目录本身未明确表示，这意味着归档不能表示空目录。

<!--
### Directory table entry

 * Name.
    - 32 bit offset from the start of the directory names chunk to the path
      data, in bytes.
    - 16 bit length of name, in bytes.
 * 16 bits of zeros, reserved for future use.
 * Data.
    - 64 bit offset from start of archive to the start of the content chunk, in
      bytes.
    - 64 bit length of the data, in bytes.
 * 64 bits of zeros, reserved for future use. -->

### 目录表

 * 名称
    - 从目录名块开始到路径数据的32位偏移量，以字节为单位。
    - 16位长度的名称，以字节为单位。
    * 16位零，保留供将来使用。
 * 数据
    - 从归档开始到内容块开始的64位偏移量，以字节为单位。
    - 64位长度的数据，以字节为单位。
 * 64位零，保留供将来使用。

<!--
## Directory hash chunk (Type “DIRHASH-”)

The directory hash chunk is optional.

 * 32 bit hash algorithm identifier.
    - Must be 1.
    - Identifies "SHA-256".
 * 32 bit length of each hash.
    - Must be 32.
 * Concatenated octets of hashes
    - Each entry is the hash of the data in the content chunk associated with
      the corresponding entry in the directory table. -->

## 目录散列块(类型 "DIRHASH-")
目录哈希块是可选的。
 * 32位 hash 算法标识符
    - 必须是1。
    - 标识 "SHA-256"。
 * 每个Hash长度为32位。
    - 必须是32。
 * Concatenated octets of hashes
    - 每个条目是目录表中与相应条目关联的内容块中的数据散列。

<!--
## Directory names chunk (Type "DIRNAMES")

The directory names chunk is required and is used by the directory chunk to name
the content chunks. Path data must be sorted in increasing lexicographical
octet order (e.g., as compared by memcmp).

 * Concatenated path data (no encoding specified).
 * Zero padding to next 8 byte boundary.

Note: The offsets used to index into the path data are 32 bits long, which means
there is no reason to create a directory name chunk that is larger than 4 GB.

Although no encoding is specified, clients that wish to display path data using
unicode may attempt to decode the data as UTF-8. The path data might or might
not be UTF-8, which means that decoding might fail. -->

## 目录名称块

目录名称块是必需的，并且由目录块用来命名内容块。 路径数据必须按照"increasing lexicographical octet"排序 (比如, 类比memcmp)。

 * Concatenated path data (未指定编码)。
 * 零填充到下一个8字节边界。

Note: 用于索引路径数据的偏移量长度为32位，这意味着没有理由创建大于4 GB的目录名称块。

虽然未指定编码，但希望使用unicode显示路径数据的客户端可能会尝试将数据解码为UTF-8。而路径数据不一定是UTF-8，这意味着解码可能会失败。

<!--
### Path data

 * Octets of path.
    - Must not be empty.
    - Must not contain a 0x00 octet.
    - The leading octet must not be 0x2F ('/').
    - The trailing octet must not be 0x2F ('/').
    - Let *segments* be the result of splitting the path on 0x2F ('/'). Each
      segment must meet the following requirements:
       - Must not be empty.
       - Must not be exactly 0x2E ('.')
       - Must not be exactly 0x2E 0x2E ('..') -->

### 路径数据-Path data

 * Octets of path。
   - 不能为空。
   - 不得包含0x00八位字节。
   - leading octet不能是 0x2F（'/'）。
   - trailing octet不能是 0x2F ('/')。
   -  *segments* 是在0x2F（'/'）上拆分路径的结果。每segment必须遵循以下要求：
     - 不能为空。
     - 不能是 0x2E ('.')。
     - 不能是 0x2E 0x2E ('..')。

<!--
## Content chunk

Content chunks must be after all the chunks listed in the index chunk. The
content chunks must appear in the archive in the order they are listed in the
directory.

 * data

The data must be aligned on a 4096 byte boundary from the start of the archive
and the data must be padded with zeros until the next 4096 byte boundary. -->

## 内容块-Content chunk

内容块必须在索引块之后。内容块必须按照目录中列出的顺序出现在归档文件中。

 * data

data必须在存档开始的4096字节边界上对齐，data必须用零填充，直到下一个4096字节边界。
