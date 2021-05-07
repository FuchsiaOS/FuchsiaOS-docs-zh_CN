# fvm

```
usage: fvm [ output_path ] [ command ] [ <flags>* ] [ <input_paths>* ]
fvm performs host-side FVM and sparse file creation
```

## Commands:

```
 create : Creates an FVM partition
 add : Adds a Minfs or Blobfs partition to an FVM (input path is required)
 extend : Extends an FVM container to the specified size (length is required)
 ftl-raw-nand: converts the input fvm.sparse.blk into a FTL Raw Nand Image (--sparse is required).
 sparse : Creates a sparse file. One or more input paths are required.
 pave : Creates an FVM container from a sparse file.
 verify : Report basic information about sparse/fvm files and run fsck on contained partitions.
 check : verifies that the |--sparse| image provided is valid. if |--max_disk_size| is provided check that the maximum disk size is set to such value in the sparse image.
 size : Prints the minimum size required in order to pave a sparse file. If the --disk flag is provided, instead checks that the paved sparse file will fit within a disk of this size. On success, no information is outputted
 used-data-size : Prints sum of the space, in bytes, used by data on 
 different partitions. This does not include blocks used internally for 
 superblock, bitmaps, inodes, or for journal,
 used-inodes : Prints the sum of used inodes on different partitions.
 used-size : Prints sum of the space, in bytes, used by data and by
 superblock, bitmaps, inodes, and journal different partitions. All of the
 reservations for non-data blocks are considered as used.
 decompress : Decompresses a compressed sparse/raw file. --sparse/lz4/default input path is required. If option is set to --default, the tool will attempt to detect the input format
Flags (neither or both of offset/length must be specified):
 --slice [bytes] - specify slice size - only valid on container creation.
                   (default: 8388608)
 --max-disk-size [bytes] Used for preallocating metadata. Only valid for sparse image. (defaults to 0)
 --offset [bytes] - offset at which container begins (fvm only)
 --length [bytes] - length of container within file (fvm only)
 --compress - specify that file should be compressed (sparse and android sparse image only)
 --disk [bytes] - Size of target disk (valid for size command only)
 --disk-type [file, mtd OR block_device] - Type of target disk (pave only)
 --max-bad-blocks [number] - Max bad blocks for FTL (pave on mtd only)
Input options:
 --blob [path] [reserve options] - Add path as blob type (must be blobfs)
 --data [path] [reserve options] - Add path as encrypted data type (must be minfs)
 --data-unsafe [path] - Add path as unencrypted data type (must be minfs)
 --system [path] - Add path as system type (must be minfs)
 --default [path] - Add generic path
 --sparse [path] - Path to compressed sparse file
 --lz4 [path] - Path to lz4 compressed raw file
 --raw [path] - Path to raw fvm image file
 --resize-image-file-to-fit - When used with create/extend command, the output image file will be resized to just fit the metadata header and added partitions. Disk size specified in the header remains the same. It's useful for reducing the size of the image file for flashing
 --android-sparse-format - When used with create command, the image will be converted to android sparse image.
 --length-is-lowerbound - When used with extend command, if current disk size is already no smaller than the specified size, the command will be no-op. If the option is not specified, it will error out in this case.
reserve options:
 These options, on success, reserve additional fvm slices for data/inodes.
 The number of bytes reserved may exceed the actual bytes needed due to
 rounding up to slice boundary.
 --minimum-inodes inode_count - number of inodes to reserve
                                Blobfs inode size is 64
                                Minfs inode size is 256
 --minimum-data-bytes data_bytes - number of bytes to reserve for data
                                   in the fs
                                   Blobfs block size is 8192
                                   Minfs block size is 8192
 --maximum-bytes bytes - Places an upper bound of <bytes> on the total
                         number of bytes which may be used by the partition.
                         Returns an error if more space is necessary to
                         create the requested filesystem.
 --with-empty-minfs    - Adds a placeholder partition that will be formatted on boot,
                         to minfs. The partition will be the 'data' partition.
   --nand-page-size : Sets the hardware page size in bytes used by the targetted device.
   --nand-oob-size : Sets the hardware page oob size in bytes used by the targetted device.
   --nand-pages-per-block : Sets the number of pages per block in the device.
   --nand-block-count : Sets the number of blocks in the device.
```
