# Random access compression in BlobFS

The BlobFS filesystem in Fuchsia transparently compresses files in order to save
disk space. BlobFS supports a number of different compression strategies, with
[zstd](https://facebook.github.io/zstd/) being the default.

One downside of file compression is that it can prevent *random access* into
files. For most compression algorithms, the entire contents must be read from
disk and decompressed to access a single byte.

For BlobFS, this is a particular challenge when files are demand paged. Demand
paging allows a file to be partially loaded into memory, which saves system
memory. Full-file compression prevents a file from being partially loaded.

The [chunked-compression](/src/lib/chunked-compression/) format and library in
Fuchsia breaks compressed files up into frames that can be independently
decompressed. This allows BlobFS to implement demand paging of compressed files,
since the file can be partially loaded and decompressed.

This document describes the design of the chunked-compression format and
explains its use in Fuchsia.

## Design goals and non-goals

The following are the goals that motivate the design of the chunked-compression
format and library:

*   **Random access decompression**. It must be possible to independently
    decompress frames of data without needing to decompress the entire file.

*   **Flexible decompression API**. The library was designed to give clients
    control over the seek table, so clients have fine grained control over which
    frames they decompress. This supports use cases like demand paging, where
    the client (BlobFS) has more information about access patterns and can
    control read-ahead and prefetch precisely by decompressing specific frames.

    This is in contrast to a more managed API that hides the details of which
    frames contain which decompressed bytes.

*   **Configurable frame sizes**. It must be possible to adjust the sizes of
    decompressed frames of data, to suit different use cases and different
    requirements.

*   **Flexible compression layout**. The format supports more exotic frame
    layouts, such as having non-uniform frame sizes or having aligned frame
    starts, in order to accommodate future use cases that require more
    flexibility. For example, non-uniform frame sizes could be used to improve
    data locality by grouping together data that is read together into smaller
    or larger frames.

*   **Comparable compression ratio to [zstd](https://facebook.github.io/zstd)**.
    Since zstd was the current default compression algorithm for BlobFS at the
    time of this document's writing, zstd's compression ratio is the baseline
    which the chunked-compression library is benchmarked against. The overhead
    due to framing and additional metadata must be minimal.

*   **Configurable compression aggressiveness**. It must be possible to trade
    off slower compression speed in favor of better compression ratios.

*   **Cross-platform library**. The library used to compress and decompress
    chunked compression archives must be usable both on Fuchsia and on the
    compilation host (e.g. Linux), to enable the use of the library in the build
    toolchain. This is necessary to compress files at build-time, for example
    when generating a base BlobFS image.

The following are non-goals:

*   **Format-level compatibility with zstd**. The chunked compression archive is
    not intended to be format-compatible with zstd, and regular zstd tooling
    will not work with chunked compression archives.

## Chunked compression

### Archive format

A chunked archive consists of a header followed by zero or more
[zstd](https://facebook.github.io/zstd/) frames.

#### Header

The header describes the format of the archive and contains a seek table that
maps the compressed frames to decompressed space.

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

The "Header CRC32" is computed based on the entire header, including the seek
table.

##### Seek table

Each seek table entry describes a contiguous range of data in the compressed
space, and where in the decompressed data it expands to.

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

Seek table entries are *contiguous* in decompressed space, but may be
*discontiguous* in compressed space. This is to support adding alignment and
padding to output files to improve storage access efficiency.

A seek table can hold up to 1023 entries (which results in a 32KiB header) and
must contain at least 1 entry (which results in a 64 byte header). Typically,
compressed frames immediately follow the seek table (but the format supports the
compressed frames starting at any offset past the end of the seek table).

##### Seek table invariants

*   I0: The first seek table entry must have decompressed offset 0.

*   I1: The first seek table entry must have compressed offset greater than or
    equal to the size of the header.

*   I2: Each entry's decompressed offset must be equal to the end of the
    previous frame (i.e. to the previous frame's decompressed offset+length).

*   I3: Each entry's compressed offset must be greater than or equal to the end
    of the previous frame (i.e. to the previous frame's compressed
    offset+length).

*   I4: Each entry must have a non-zero decompressed and compressed length.

*   I5: No compressed frame may exceed the end of the file.

#### Compressed frames

Each compressed frame in the file is a regular zstd compressed frame. A given
frame will map to some contiguous chunk of bytes in the decompressed file.

Any ranges of bytes in the file not covered by the seek table are ignored.

It is not a requirement that each frame of data is the same decompressed size,
but the current implementation of compression splits the input data into
equal-sized frames. The size of the frames is configurable during compression.

### Random-access decompression

The seek table is used to look up the frames that contain a given decompressed
range. When a given range in the decompressed file is requested, one or more
compressed frames must be loaded and decompressed.

For example, consider the following file with three frames:

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

Accessing a byte range within a single frame only requires decompressing its
corresponding compressed frame:

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

A range that spans several decompressed frames will require decompressing
multiple compressed frames:

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

## Use in BlobFS

In BlobFS, files are compressed using the chunked compression library to
facilitate random access and demand paging.

Currently, random access is only used when demand paging is enabled. With demand
paging disabled, BlobFS will load and decompress the entire file on first
access, buffering the file in memory while there are handles to the file.

With demand paging enabled, BlobFS lazily loads in portions of files as they are
accessed. BlobFS registers itself as the
[pager](/docs/reference/kernel_objects/pager.md) for the VMO using the
[zx_pager_create](/docs/reference/syscalls/pager_create.md) syscall. When a
non-present page is accessed in the VMO, a page fault occurs, which BlobFS
handles.

BlobFS looks up the decompressed frames containing the target page(s), and
decompresses each frame. After decompressing each frame, the data is verified,
and committed to the pager-backed VMO through the
[zx_pager_supply_pages](/docs/reference/syscalls/pager_supply_pages.md) syscall.
