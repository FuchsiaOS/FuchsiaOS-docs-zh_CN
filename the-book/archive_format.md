# Fuchsia 归档格式

An archive is a sequence of bytes, divided into chunks:
一个归档是一个字节序列，分为以下几块：
 * 第一个块是索引块，它描述了存档中其他块的位置。
 * 索引中列出的所有块必须按索引中列出的顺序出现在归档中（按类型排序）
 * 归档中可能包含索引中未引用的其他块，但这些块必须在索引中列出所有块之后出现在存档中。
   例如，内容块在索引中列出。但是，该内容块可以从目录块中获得。
 * 块不能重叠。
 * 所有块都在64bit边界上对齐。
 * 所有块必须根据他们的对齐约束尽可能地紧密打包。
 * 块之间的任何间隙必须用'0'填充。

所有偏移和长度在小字节序中被编码为无符号整数。


## 序列块-Index chunk

索引块是必需的，且必须从归档的开头开始。

 * 8字节的魔法。
    -必须是 0xc8 0xbf 0x0b 0x48 0xad 0xab 0xc5 0x11.
 * 64位长度的多行索引条目，以字节为单位。
 * 多行索引。

没有具有相同类型的两个索引项，且索引项必须按照 `八位组字典增序`排列（例如，比较memcmp）。索引中列出的块必须按照列出的顺序存储在归档中。
（文档原句：the entries must be sorted by type in increasing lexicographical octet order ）



### 索引项-Index entry

 * 64 bit 块类型。
 * 从归档的开始到引用块的开始，以64 bit字节为单位偏移。
 * 64 字节长度的引用块。

## 散列块-Hash chunk (type 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00)

散列块是可选的。

 * 32位哈希算法标识符
    - 必须是1。
    - 标识 "SHA-256"。
 * 散列数据
    - 32位散列长度。
      - 必须是32。
    - Octets of hash.
      - 从归档开始到索引末尾列出的最后一个块，用"octets of hash"替换为零的内容。
      - Note:散列包含索引中的所有块，但不包含任何不在索引中的内容块。如果需要检查内容块的完整性，请加上目录散列块。

Note:我们可能会将此哈希算法替换为我们在blobfs中使用的哈希算法。

## 目录块-Directory chunk（type "DIR -----"）

目录块是必需的。目录块中的条目必须具有唯一的名称，且必须将名称以"increasing lexicographical octet"对条目进行排序（e.g，与memcmp类比）。

 * Concatenated directory table entries.

这些条目表示归档中包含的文件。目录本身未明确表示，这意味着归档不能表示空目录。

### 目录表-Directory table entry

 * 名称
    - 从目录名块开始到路径数据的32位偏移量，以字节为单位。
    - 16位长度的名称，以字节为单位。
 * 16位零，保留供将来使用。
 * 数据
    - 从归档开始到内容块开始的64位偏移量，以字节为单位。
    - 64位长度的数据，以字节为单位。
 * 64位零，保留供将来使用。

## 目录散列块-Directory hash chunk (Type “DIRHASH-”)

目录哈希块是可选的。

 * 32位hash algorithm标识符
    - 必须是1.
    - 标识 "SHA-256".
 * 每个Hash长度为32位。
    - 必须是32。
 * Concatenated octets of hashes
    - 每个条目是目录表中与相应条目关联的内容块中的数据散列。

## 目录名称块-Directory names chunk (Type "DIRNAMES")

目录名称块是必需的，并且由目录块用来命名内容块。 路径数据必须按照"increasing lexicographical octet"排序 (e.g., 类比memcmp)。

 * Concatenated path data (未指定编码)。
 * 零填充到下一个8字节边界。

Note: 用于索引路径数据的偏移量长度为32位，这意味着没有理由创建大于4 GB的目录名称块。

虽然未指定编码，但希望使用unicode显示路径数据的客户端可能会尝试将数据解码为UTF-8。而路径数据不一定是UTF-8，这意味着解码可能会失败。

### 路径数据-Path data

 * Octets of path.
    - 不能为空。
    - 不得包含0x00八位字节。
    - leading octet不能是 0x2F（'/'）。
    - trailing octet不能是 0x2F ('/')。
    -  *segments* 是在0x2F（'/'）上拆分路径的结果。每segment必须遵循以下要求：
       - 不能为空。
       - 不能是 0x2E ('.')。
       - 不能是 0x2E 0x2E ('..')。

## 内容块-Content chunk

内容块必须在索引块之后。内容块必须按照目录中列出的顺序出现在归档文件中。

 * data

data必须在存档开始的4096字节边界上对齐，data必须用零填充，直到下一个4096字节边界。
