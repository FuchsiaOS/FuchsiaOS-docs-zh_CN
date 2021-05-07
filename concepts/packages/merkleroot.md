# Fuchsia Merkle Roots

[Merkle Trees][merkletree] are used in various places in the Fuchsia ecosystem,
including the [FAR Archive Format][far], the Blob Storage Filesystem, and the
[Package Manager][pm].

In [Zircon][zircon] `zx-verity` provides an API for application components to
read data from local storage. When retrieving data the integrity of the data is
verified and causing reads to fail when the data has been modified or corrupted.
zx-verity is based on a [Merkle Tree][merkletree], and is derived from a similar
system in [Chrome OS][dmverity].

All of these implementations share the algorithm documented herein.

## Parameters of the Merkle Tree

 * Block size: 8kb, 0 padded.
 * Root digest size: 32 bytes.
 * Hash algorithm: SHA-256.
 * Block digest computation: SHA-256(u64(offset | level) + u32(length) + data)

## Definitions

The merkle tree contains levels. A level is a row of the tree, starting at 0 and
counting upward. Level 0 represents the level that contains hashes of chunks of
the input stream.

Each level contains a number of hashes of the previous level. The hashes within
a level are computed from 8kb blocks from the previous level (or data, if level
0), prepended with a block identity.

A block identity is the binary OR of the starting byte index of the block within
the current level and the current level index, followed by the length of the
block. For level 0, the length of the block is 8kb, except for the last block,
which may be less than 8kb. All other levels use a length of 8kb, even when the
last block is 0 padded.

## Computation of a level

 1. Initialize the level with an index, an offset starting at 0, and an empty
    list of hashes.
 2. For each 8kb (or remainder of) of input, compute the next block identity by
    taking the binary OR of the level index and the current offset, followed
    by the length of the input.
 3. Init a SHA-256 digest, append to it the identity, the input, and if the
    input is shorter than 8kb, a pad of 0 up to 8kb.
 4. Append the output of the digest to the level's list of hashes. Increment
    the offset by the input length.
 5. Repeat 1-4 until all input is consumed.
 6. If the length of hashes is 32, finish.
 7. If the length of hashes is not 8kb aligned, 0 fill up to an 8kb
    alignment and compute more levels until there is a root level containing a
    single 32 byte digest.

## Computation of a root digest

Compute level 0 with the input data. Construct and compute subsequent levels
using the previous level hashes as input data, until a level hashes contains
exactly 32 bytes. This last level contains the root digest of the merkle tree.

## A note about the empty digest

As a special case, when there is no input data, implementations may need to
handle the calculation independently. The digest of the empty input is simply
the SHA-256 of 12 0 bytes, the block identity of a single 0 length block.

## Example values

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
 `2feb488cffc976061998ac90ce7292241dfa86883c0edc279433b5c4370d0f30`


[merkletree]: https://en.wikipedia.org/wiki/Merkle_tree "Merkle Tree"
[dmverity]: https://www.chromium.org/chromium-os/chromiumos-design-docs/verified-boot "Chrome OS Verified Boot"
[far]: /docs/concepts/source_code/archive_format.md "Archive Format"
[pm]: /src/sys/pkg/bin/pm/README.md "Package Manager"
[zircon]: /zircon/README.md "Zircon"
