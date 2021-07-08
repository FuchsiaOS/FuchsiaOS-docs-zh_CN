<!--# Random access compression in BlobFS-->

# BlobFS中的随机访问压缩

<!--The BlobFS filesystem in Fuchsia transparently compresses files in order to save
disk space. BlobFS supports a number of different compression strategies, with
[zstd](https://facebook.github.io/zstd/) being the default.-->

Fuchsia 中的 BlobFS 文件系统为了节省磁盘空间而显式的压缩文件，BlobFS 提供多种不同的压缩策略，默认使用[zstd](https://facebook.github.io/zstd/)

<!--One downside of file compression is that it can prevent *random access* into
files. For most compression algorithms, the entire contents must be read from
disk and decompressed to access a single byte.-->

文件压缩的一个缺点是阻止对文件的 *随机访问*。对于大多数压缩算法来说，必须从磁盘读取全部内容并解压缩才能访问单个字节。

<!--For BlobFS, this is a particular challenge when files are demand paged. Demand
paging allows a file to be partially loaded into memory, which saves system
memory. Full-file compression prevents a file from being partially loaded.-->

对于 BlobFS来说，当文件按需分页时，这是一个特别的挑战。按需分页允许一个文件部分装载到内存，以便节省系统内存。
完整文件压缩防止了部分加载文件方式。

<!--The [chunked-compression](/src/lib/chunked-compression/) format and library in
Fuchsia breaks compressed files up into frames that can be independently
decompressed. This allows BlobFS to implement demand paging of compressed files,
since the file can be partially loaded and decompressed.-->

Fuchsia 中的[分块压缩](/src/lib/chunked-compression/)方式和库将压缩文件分解成可以独立解压缩的帧。
这让 BlobFS 实现了压缩文件的按需分页，因为文件能够部分加载和解压。

<!--This document describes the design of the chunked-compression format and
explains its use in Fuchsia.-->

这篇文档描述了分块压缩格式的设计以及介绍了它是如何在 Fuchsia 中使用的。

<!--## Design goals and non-goals-->

## 设计目标和非目标

<!--The following are the goals that motivate the design of the chunked-compression
format and library:-->

以下是分块压缩格式和库的设计目标：

<!--*   **Random access decompression**. It must be possible to independently
    decompress frames of data without needing to decompress the entire file.-->

*   **随机存取解压** 能够独立的解压部分数据段而不需要解压整个文件。

<!--*   **Flexible decompression API**. The library was designed to give clients
    control over the seek table, so clients have fine grained control over which
    frames they decompress. This supports use cases like demand paging, where
    the client (BlobFS) has more information about access patterns and can
    control read-ahead and prefetch precisely by decompressing specific frames.

    This is in contrast to a more managed API that hides the details of which
    frames contain which decompressed bytes.-->

*   **灵活的解压 API** 该库被设计为可以让客户端控制搜索表，因此客户端可以细粒度地控制它们解压缩的帧。在支持请求分页等用例中，客户端( BlobFS )有更多有关访问模式的信息，并且可以通过解压缩特定帧来精确控制预读和预取。

    这与更受管理的API形成对比，后者隐藏哪些帧包含哪些解压缩字节的详细信息。

<!--*   **Configurable frame sizes**. It must be possible to adjust the sizes of
    decompressed frames of data, to suit different use cases and different
    requirements.-->

*   **可配置的帧大小** 为了适配不同的案例以及不同的需求，数据压缩帧的大小可适配是必须的。

<!--*   **Flexible compression layout**. The format supports more exotic frame
    layouts, such as having non-uniform frame sizes or having aligned frame
    starts, in order to accommodate future use cases that require more
    flexibility. For example, non-uniform frame sizes could be used to improve
    data locality by grouping together data that is read together into smaller
    or larger frames.-->

*   **灵活的压缩方式** 支持多种多样的帧方式，例如非统一的帧大小或者帧开始的对齐来适应需要更为灵活的场景需求。通过将一起读取的数据分组成更小或更大的帧，可以使用非均匀帧大小来改善数据局部性。

<!--*   **Comparable compression ratio to [zstd](https://facebook.github.io/zstd)**.
    Since zstd was the current default compression algorithm for BlobFS at the
    time of this document's writing, zstd's compression ratio is the baseline
    which the chunked-compression library is benchmarked against. The overhead
    due to framing and additional metadata must be minimal.-->

*   **与[ zstd ](https://facebook.github.io/zstd)的压缩率差不多**
    由于在撰写本文档时，zstd 是 BlobFS 的当前默认压缩算法，zstd 的压缩率是以分块压缩库进行基准测试为标准。因此成帧和附加元数据引起的开销一定是最小的。

<!--*   **Configurable compression aggressiveness**. It must be possible to trade
    off slower compression speed in favor of better compression ratios.-->

*   **压缩比例的选配**。在较慢的压缩速度之间进行折衷，以获得更好的压缩比。

<!--*   **Cross-platform library**. The library used to compress and decompress
    chunked compression archives must be usable both on Fuchsia and on the
    compilation host (e.g. Linux), to enable the use of the library in the build
    toolchain. This is necessary to compress files at build-time, for example
    when generating a base BlobFS image.-->

*   **库可跨平台**。用于在压缩和解压缩文档压缩块的库，必须要能够在 Fuchsia 和其他编译主机上（如： Linux ），都能在工具链上使用这个库。
    这对于在构建时压缩文件是必要的，例如在生成基本 BlobFS 映像时。

<!--The following are non-goals:

*   **Format-level compatibility with zstd**. The chunked compression archive is
    not intended to be format-compatible with zstd, and regular zstd tooling
    will not work with chunked compression archives.-->

以下是非设计目标：

*   **与 zstd 格式的兼容性**。文档压缩块不期望能和 zstd 的格式相兼容，因此，常用的 zstd 工具将无法应用在文档压缩块上。

<!--## Chunked compression-->

## 块压缩

<!--### Archive format-->

### 归档格式

<!--A chunked archive consists of a header followed by zero or more
[zstd](https://facebook.github.io/zstd/) frames.-->

一个文档块的头的组成应该是0个或者多个 [ zstd ](https://facebook.github.io/zstd/) 帧。

<!--#### Header-->

#### 头

<!--The header describes the format of the archive and contains a seek table that
maps the compressed frames to decompressed space.-->

头描述了存档的格式，并包含一个将压缩帧映射到解压缩空间的搜索表。

```
      0     1     2     3     4     5     6     7
   +-----+-----+-----+-----+-----+-----+-----+-----+
 0 |                 Magic Number                  |
   +-----+-----+-----+-----+-----+-----+-----+-----+
 8 |  Version  |  Reserved |       Num Frames      |  // Reserved must be zero.
   +-----+-----+-----+-----+-----+-----+-----+-----+
16 |    Header CRC32       |        Reserved       |  // Reserved must be zero.
   +-----+-----+-----+-----+-----+-----+-----+-----+
24 |                    Reserved                   |  // Reserved must be zero.
   +-----+-----+-----+-----+-----+-----+-----+-----+
32 |                                               |
40 |                   Seek Table                  |
48 |                     Entry                     |
56 |                                               |
   +-----+-----+-----+-----+-----+-----+-----+-----+
.. |                                               |
.. |                   Seek Table                  |
.. |                     Entry                     |
.. |                                               |
   +-----+-----+-----+-----+-----+-----+-----+-----+
```

<!--The "Header CRC32" is computed based on the entire header, including the seek
table.-->

基于包括搜索表在内的整个报头来计算“ Header CRC32 ”。

<!--##### Seek table-->

##### 查找表

<!--Each seek table entry describes a contiguous range of data in the compressed
space, and where in the decompressed data it expands to.-->

每个查找表项描述压缩空间中的连续数据范围，以及它在解压缩数据中扩展到的位置。

```
   +-----+-----+-----+-----+-----+-----+-----+-----+
 0 |               Decompressed Offset             |
   +-----+-----+-----+-----+-----+-----+-----+-----+
 8 |                Decompressed Size              |
   +-----+-----+-----+-----+-----+-----+-----+-----+
16 |                Compressed Offset              |
   +-----+-----+-----+-----+-----+-----+-----+-----+
24 |                 Compressed Size               |
   +-----+-----+-----+-----+-----+-----+-----+-----+
```

<!--Seek table entries are *contiguous* in decompressed space, but may be
*discontiguous* in compressed space. This is to support adding alignment and
padding to output files to improve storage access efficiency.-->

查找表项在解压缩空间中是 *连续的*，但是在压缩空间时可能是 *非连续的*。这是为了支持向输出文件添加对齐和填充，以提高存储访问效率。

<!-- A seek table can hold up to 1023 entries (which results in a 32KiB header) and
must contain at least 1 entry (which results in a 64 byte header). Typically,
compressed frames immediately follow the seek table (but the format supports the
compressed frames starting at any offset past the end of the seek table). -->

单个查找表最高支持1023个表项（表头占32KB）和至少1个表项（表头占64字节）。因此，压缩帧能立刻通过查找表被找到（但该格式支持从超过搜索表末尾的任何偏移量开始的压缩帧）。

<!-- ##### Seek table invariants -->

##### 查找表常量

<!-- *   I0: The first seek table entry must have decompressed offset 0.

*   I1: The first seek table entry must have compressed offset greater than or
    equal to the size of the header.

*   I2: Each entry's decompressed offset must be equal to the end of the
    previous frame (i.e. to the previous frame's decompressed offset+length).

*   I3: Each entry's compressed offset must be greater than or equal to the end
    of the previous frame (i.e. to the previous frame's compressed
    offset+length).

*   I4: Each entry must have a non-zero decompressed and compressed length.

*   I5: No compressed frame may exceed the end of the file. -->

*   I0: 第一个查找表项必须具有解压缩的偏移量0。

*   I1: 第一个搜索表条目必须具有大于或等于标题大小的压缩偏移量。

*   I2: 每个条目的解压缩偏移量必须等于前一帧的末尾(即前一帧的解压缩偏移量+长度)。

*   I3: 每个条目的压缩偏移量必须大于或等于前一帧的末尾(即前一帧的压缩偏移量+长度)。

*   I4: 每个条目必须具有非零解压缩和压缩长度。

*   I5: 任何压缩帧都不能超过文件末尾。

<!-- #### Compressed frames -->

#### 压缩帧

<!-- Each compressed frame in the file is a regular zstd compressed frame. A given
frame will map to some contiguous chunk of bytes in the decompressed file.

Any ranges of bytes in the file not covered by the seek table are ignored.

It is not a requirement that each frame of data is the same decompressed size,
but the current implementation of compression splits the input data into
equal-sized frames. The size of the frames is configurable during compression. -->

文件中的每个压缩帧都是规则的 zstd 压缩帧。给定帧将映射到解压缩文件中的某个连续字节块。
文件中未被搜索表覆盖的任何字节范围都将被忽略。
不要求每个数据帧的解压缩大小相同，但当前的压缩实现将输入数据分割成大小相等的帧。帧的大小在压缩期间是可配置的。

<!-- ### Random-access decompression -->

### 随机存取解压

<!-- The seek table is used to look up the frames that contain a given decompressed
range. When a given range in the decompressed file is requested, one or more
compressed frames must be loaded and decompressed.

For example, consider the following file with three frames: -->

搜索表用于查找包含给定解压缩范围的帧。当请求解压缩文件中的给定范围时，必须加载并解压缩一个或多个压缩帧。

例如，考虑以下具有三个帧的文件：

```
Decompressed Space            Compressed Space
+-----------------+           +--------------+
|                 | <-------- |              |
|                 |           |     +--------|
|                 |           +-----+        |
+-----------------+   +------ |              |
|                 | <-+       +--------------+
|                 |       +-- |              |
|                 |       |   +--------------=
+-----------------+       |
|                 | <-----+
|       +---------|
+-------+
```

<!-- Accessing a byte range within a single frame only requires decompressing its
corresponding compressed frame: -->

访问单个帧内的字节范围只需要解压缩其对应的压缩帧：

```
Decompressed Space            Compressed Space
+-----------------+           +--------------+
|                 | <-------- |xxxxxxxxxxxxxx|
|  xxxxxxxx       |           |xxxxx+--------|
|                 |           +-----+        |
+-----------------+   +------ |              |
|                 | <-+       +--------------+
|                 |       +-- |              |
|                 |       |   +--------------=
+-----------------+       |
|                 | <-----+
|       +---------|
+-------+
```

<!-- A range that spans several decompressed frames will require decompressing
multiple compressed frames: -->

跨越多个解压缩帧的范围需要解压缩多个压缩帧：

```
Decompressed Space            Compressed Space
+-----------------+           +--------------+
|                 | <-------- |xxxxxxxxxxxxxx|
|                 |           |xxxxx+--------|
|              xxx|           +-----+xxxxxxxx|
+-----------------+   +------ |xxxxxxxxxxxxxx|
|xxxx             | <-+       +--------------+
|                 |       +-- |              |
|                 |       |   +--------------=
+-----------------+       |
|                 | <-----+
|       +---------|
+-------+
```

<!--## Use in BlobFS-->

## BlobFS 应用

<!--In BlobFS, files are compressed using the chunked compression library to
facilitate random access and demand paging.-->

在 BlobFS 中，文件使用分块压缩库进行压缩，以便于随机访问和请求分页。

<!--Currently, random access is only used when demand paging is enabled. With demand
paging disabled, BlobFS will load and decompress the entire file on first
access, buffering the file in memory while there are handles to the file.-->

目前，随机访问仅当按需分页开启时才被使用。当按需分页被关闭时，BlobFS 将在首次加载和解压整个文件，并当操作文件时将文件缓存到内存中。

<!--With demand paging enabled, BlobFS lazily loads in portions of files as they are
accessed. BlobFS registers itself as the
[pager](/docs/reference/kernel_objects/pager.md) for the VMO using the
[zx_pager_create](/docs/reference/syscalls/pager_create.md) syscall. When a
non-present page is accessed in the VMO, a page fault occurs, which BlobFS
handles.-->

当按需分页被开启时，BlobFS 在访问文件时只懒加载文件的部分。BlobFS使用[ ZX_PAGER_CREATE ](/docs/reference/syscalls/pager_create.md)将自身注册为VMO的[ pager ](/docs/reference/kernel_objects/pager.md)
在 VMO 中访问非当前页时，会发生页错误，由 BlobFS 处理。

<!--BlobFS looks up the decompressed frames containing the target page(s), and
decompresses each frame. After decompressing each frame, the data is verified,
and committed to the pager-backed VMO through the
[zx_pager_supply_pages](/docs/reference/syscalls/pager_supply_pages.md) syscall.-->

BlobFS 查找包含目标页面的解压缩帧，然后解压缩每个帧。 解压缩每一帧后，将验证数据，并通过[ ZX_PAGER_SUPPLE_PAGES ](/docs/reference/syscalls/pager_supply_pages.md) 系统调用将数据提交给页表备份的 VMO

