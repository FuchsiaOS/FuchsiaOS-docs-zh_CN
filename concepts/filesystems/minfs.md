# MinFS

MinFS is a simple, unix-like filesystem built for Zircon.

It currently supports files up to 4 GB in size.

## Using MinFS

### Host Device (QEMU Only)

 * Create a disk image that stores MinFS

  ```shell
  # (Linux)
  $ truncate --size=16G blk.bin
  # (Mac)
  $ mkfile -n 16g blk.bin
  ```

 * Execute the run zircon script on your platform with the '--' to pass
   arguments directly to QEMU and then use '-hda' to point to the file. If you
   wish to attach additional devices, you can supply them with '-hdb', '-hdc,
   and so on.

  ```shell
  fx set bringup.x64
  fx build
  fx qemu -- -hda blk.bin
  ```

### Target Device (QEMU and Real Hardware)

Warning: On real hardware, `/dev/class/block/...` refers to **REAL** storage
devices (USBs, SSDs, etc).

**BE CAREFUL NOT TO FORMAT THE WRONG DEVICE.** If in doubt, only run the
following commands through QEMU.
The `lsblk` command can be used to see more information about the devices
accessible from Zircon.

 * Within zircon, `lsblk` can be used to list the block devices currently on
   the system. On this example system below, `/dev/class/block/000` is a raw
   block device.

  ```
  > lsblk
  ID  DEV      DRV      SIZE TYPE           LABEL
  000 block    block     16G
  ```

 * Let's add a GPT to this block device.

  ```
  > gpt init /dev/class/block/000
  ...
  > lsblk
  ID  DEV      DRV      SIZE TYPE           LABEL
  002 block    block     16G
  ```

 * Now that we have a GPT on this device, let's check what we can do with it.
   (NOTE: after manipulating the gpt, the device number may change. Use `lsblk`
   to keep track of how to refer to the block device).

  ```
  > gpt dump /dev/class/block/002
  blocksize=512 blocks=33554432
  Partition table is valid
  GPT contains usable blocks from 34 to 33554398 (inclusive)
  Total: 0 partitions
  ```

 * `gpt dump` tells us some important info: it tells us (1) How big blocks are,
   and (2) which blocks we can actually use.
   Let's fill part of the disk with a MinFS filesystem.

  ```
  > gpt add 34 20000000 minfs /dev/class/block/002
  ```

 * Within Zircon, format the partition as MinFS. Using `lsblk` you should see
   a block device, which is the whole disk, and a slightly smaller device, which
   is the partition. In the above output, the partition is device 003, and would
   have the path `/dev/class/block/003`

  ```
  > mkfs <PARTITION_PATH> minfs
  ```

 * If you want the device to be mounted automatically on reboot, use the GPT
   tool to set its type. As we did above, **you must** use `lsblk` **again**
   to locate the entry for the disk. We want to edit the type of the zero-th
   partition.  Here we use the keyword 'fuchsia-data' to set the type GUID, but
   if you wanted to use an arbitrary GUID you would supply it where
   'fuchsia-data' is used.

  ```
  > gpt edit 0 type fuchsia-data <DEVICE_PATH>
  ```

 * On any future boots, the partition will be mounted automatically at `/data`.

 * If you don't want the partition to be mounted automatically, you can update
   the visibility (or GUID) of the partition, and simply mount it manually.

  ```
  > mount <PARTITION_PATH> /data
  ```

 * Any files written to `/data` (the mount point for this GUID) will persist
   across boots. To test this, try making a file on the new MinFS volume,
   rebooting, and observing it still exists.

  ```
  > touch /data/foobar
  > dm reboot
  > ls /data
  ```

 * To find out which block device/file system is mounted at each subdirectory
   under a given path, use the following command:

  ```
  > df <PATH>
  ```

## Minfs operations

The following section describes what IOs are performed to complete a simple end
user operation like read()/write().

### Assumptions

* No operation, read or write, is cached or batched. Each of these operations
  are like calling with sync and direct io set.
* For rename: The destination file does not exist. Rename can delete a file if
  the destination of the rename operation is a valid file. This assumption keeps
  the math simple.
* The "Write" operation issues a single data block write to a previously
  unaccessed portion of the vnode.
* The "Overwrite" operation issues a single data block write to a portion of the
  block that has previously been allocated from an earlier "Write" operation.

### Keys to the columns.
1. OPERATION: The action requested by a client of the filesystem.
1. BLOCK TYPE: Each fileystem operation results in accessing one or more types
   of blocks.
    * Data: Contains user data and directory entries.
    * Indirect: Indirect block in file block map tree
    * Dindirect: Double indirect block in file block map tree.
    * Inode table: Inode table block that holds one or more inodes.
    * Inode bitmap: Contains an array of bits each representing free/used state
      of inodes.
    * Data bitmap: Contains an array of bits each representing free/used state
      of data blocks.
    * Superblock: Contains data describing layout and state of the filesystem.
1. IO TYPE: What type, read/write, of IO access it is.
1. JOURNALED: Whether the IO will be journaled. Reads are not journaled but some
   of the writes are journaled.
1. CONDITIONALLY ACCESSED: Depending on the OPERATION's input parameter and
   state of the filesystem, some blocks are conditionally accessed.
    * No:   IO is always performed.
    * Yes:  Filesystem state and input parameters decide whether this IO is
      needed or not.
1. READ COUNT: Number of filesystem blocks read.
1. WRITE COUNT (IGNORING JOURNAL): Number of filesystem blocks written. Writing
   to journal or journaling overhead are not counted towards this number.
1. WRITE COUNT (WITH JOURNAL): Number of filesystem blocks written to journal
   and then to the final location. This does not include the blocks journal
   writes to maintain the journal state.

A row \<operation\> Total, like "Create Total", gives the total number of blocks
read/written. For operations involving journaling, the journal writes two more
blocks, journal entry header and commit block, per operation. The number under
write count for <operations> Total is the sum of WRITE COUNT (WITH JOURNALING)
and journaling overhead, which is 2 blocks per operation.

Superblock, Inode table, Inode bitmap, Data bitmap, and a part of Journal are
cached in memory while starting(mount/fsck) the filesystem. So, Read IOs are
never issued for those BLOCK TYPES.

|  OPERATION   |  BLOCK TYPE  | IO TYPE | JOURNALED | CONDITIONALLY ACCESSED | READ COUNT  | WRITE COUNT(IGNORING JOURNAL) |  WRITE COUNT(WITH JOURNAL)  | COMMENTS    |
|--------------|--------------|---------|-----------|------------------------|------------:|------------:|---------------------------:|--------------------------------|
| Lookup/Open  | Data         | Read    | No        | No                     | >=1         | 0           | 0                          | If the directory is large, multiple blocks are read. |
|              | Indirect     | Read    | No        | Yes                    | >=0         | 0           | 0                          | Lookup can be served by direct blocks. So indirect is optional.  |
|              | DIndirect    | Read    | No        | Yes                    | >=0         | 0           | 0                          | Lookup can be served by direct blocks. So dindirect is optional. |
| **Lookup/Open Total** |     |         |           |                        | >=1         | 0           | 0                          |                                |
| Create       | Data         | Read    | No        | No                     | >=1         | 0           | 0                          | Create involves lookup first for name collisions. |
|              | Indirect     | Read    | No        | Yes                    | >=0         | 0           | 0                          |                                |
|              | DIndirect    | Read    | No        | Yes                    | >=0         | 0           | 0                          |                                |
|              | Data         | Write   | Yes       | No                     | 0           | >=1         | >=2                        |                                |
|              | Indirect     | Write   | Yes       | Yes                    | 0           | >=0         | >=0                        |                                |
|              | DIndirect    | Write   | Yes       | Yes                    | 0           | >=0         | >=0                        |                                |
|              | Inode table  | Write   | Yes       | No                     | 0           | 1           | 2                          | Inode for the new file.        |
|              | Inode bitmap | Write   | Yes       | No                     | 0           | 1           | 2                          | Mark inode as allocated.       |
|              | Data bitmap  | Write   | Yes       | No                     | 0           | >=0         | >=0                        | Directory may grow to contain new directory entry.  |
|              | Superblock   | Write   | Yes       | No                     | 0           | 1           | 2                          | Among other things, allocated inode number changes. |
| **Create Total** |          |         |           |                        | >=1         | >=4         | >=10                       | Includes 2 blocks for journal entry. |
| Rename       | Data         | Read    | No        | No                     | >=1         | 0           | 0                          | Rename involves a lookup in source directory. |
|              | Indirect     | Read    | No        | Yes                    | >=0         | 0           | 0                          |                                |
|              | DIndirect    | Read    | No        | Yes                    | >=0         | 0           | 0                          |                                |
|              | Data         | Write   | Yes       | No                     | 0           | >=1         | >=2                        | Source directory entry.        |
|              | Indirect     | Write   | Yes       | Yes                    | 0           | >=0         | >=0                        |                                |
|              | DIndirect    | Write   | Yes       | Yes                    | 0           | >=0         | >=0                        |                                |
|              | Inode table  | Write   | Yes       | No                     | 0           | 1           | 2                          | To update source directory inode. |
|              | Data         | Read    | No        | No                     | >=0         | 0           | 0                          | Rename involves a lookup in source directory. |
|              | Indirect     | Read    | No        | Yes                    | >=0         | 0           | 0                          |                                |
|              | DIndirect    | Read    | No        | Yes                    | >=0         | 0           | 0                          |                                |
|              | Data         | Write   | Yes       | Yes                    | 0           | >=0         | >=0                        | Writing destination directory entry. |
|              | Indirect     | Write   | Yes       | Yes                    | 0           | >=0         | >=0                        |                                |
|              | DIndirect    | Write   | Yes       | Yes                    | 0           | >=0         | >=0                        |                                |
|              | Inode table  | Write   | Yes       | Yes                    | 0           | 1           | 2                          | To update destination directory inode. |
|              | Inode table  | Write   | Yes       | No                     | 0           | 1           | 2                          | Renamed file’s mtime.          |
|              | Data bitmap  | Write   | Yes       | No                     | 0           | >=0         | >=0                        | In case we allocated data, indirect or Dindirect block(s). |
|              | Superblock   | Write   | Yes       | No                     | 0           | 1           | 2                          |                                |
| **Rename Total** |          |         |           |                        | >=1         | >=5         | >=12                       | Includes 2 blocks for journal entry. |
| Read         | Data         | Read    | No        | No                     | >=1         | 0           | 0                          |                                |
|              | Indirect     | Read    | No        | Yes                    | >=0         | 0           | 0                          |                                |
|              | DIndirect    | Read    | No        | Yes                    | >=0         | 0           | 0                          |                                |
| **Read Total** |            |         |           |                        | >=1         | 0           | 0                          |                                |
| Write        | Indirect     | Read    | No        | Yes                    | >=0         | 0           | 0                          | Even if the write is not overwriting, we may share (D)indirect block with existing data. Leading to read modify write. |
|              | DIndirect    | Read    | No        | Yes                    | >=0         | 0           | 0                          |                                |
|              | Data         | Write   | No        | No                     | 0           | 1           | 1                          |                                |
|              | Indirect     | Write   | Yes       | Yes                    | 0           | >=0         | >=0                        |                                |
|              | DIndirect    | Write   | Yes       | Yes                    | 0           | >=0         | >=0                        |                                |
|              | Inode table  | Write   | Yes       | No                     | 0           | 1           | 2                          | Inode's mtime update.          |
|              | Data bitmap  | Write   | Yes       | No                     | 0           | 1           | 2                          | For the allocated block.       |
|              | Superblock   | Write   | Yes       | No                     | 0           | 1           | 2                          | Change in number of allocated blocks. |
| **Write Total** |           |         |           |                        | >=0        | >=4        | >=9                          | Includes 2 blocks for journal entry. |
| Overwrite    | Data         | Read    | No        | Yes                    | >=0         | 0           | 0                          | Read modify write.             |
|              | Indirect     | Read    | No        | Yes                    | >=0         | 0           | 0                          |                                |
|              | DIndirect    | Read    | No        | Yes                    | >=0         | 0           | 0                          |                                |
|              | Data         | Write   | No        | No                     | 0           | 1           | 1                          |                                |
|              | Indirect     | Write   | Yes       | Yes                    | 0           | >=0         | >=0                        |                                |
|              | DIndirect    | Write   | Yes       | Yes                    | 0           | >=0         | >=0                        |                                |
|              | Inode table  | Write   | Yes       | No                     | 0           | 1           | 2                          |                                |
|              | Data bitmap  | Write   | Yes       | No                     | 0           | 1           | 2                          | Write new allocation.          |
|              | Data bitmap  | Write   | Yes       | No                     | 0           | >=0         | >=0                        | Free old block. This block bit may belong to allocated block bitmap.|
|              | Superblock   | Write   | Yes       | No                     | 0           | 1           | 2                          |                                |
| **Overwrite Total** |       |         |           |                        | >=0         | >=4         | >=9                        | Includes 2 blocks for journal entry. |
