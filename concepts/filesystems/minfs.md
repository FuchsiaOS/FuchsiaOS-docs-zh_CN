# MinFS

<!-- MinFS is a simple, unix-like filesystem built for Zircon.

It currently supports files up to 4 GB in size. -->

MinFs 是一个基于 Zircon 构建的简单的，类 unix 文件系统，它目前支持文件大小最高为4GB。

<!-- ## Using MinFS -->

## 使用MinFS 

<!-- ### Host Device (QEMU Only) -->

### 虚拟设备(限 QEMU )

 <!-- * Create a disk image that stores MinFS -->
 * 创建一个磁盘镜像来存储 MinFS

  ```shell
  # (Linux)
  $ truncate --size=16G blk.bin
  # (Mac)
  $ mkfile -n 16g blk.bin
  ```

 <!-- * Execute the run zircon script on your platform with the '--' to pass
   arguments directly to QEMU and then use '-hda' to point to the file. If you
   wish to attach additional devices, you can supply them with '-hdb', '-hdc,
   and so on. -->
   * 在你的平台上执行zircon脚本，通过 ‘--’ 直接向 QEMU 传递参数，然后使用 '-hda' 来指定文件。如果你想增加额外的设备，你可以通过  '-hdb', '-hdc' 等参数进行增加。

  ```shell
  fx set bringup.x64
  fx build
  fx qemu -- -hda blk.bin
  ```

<!-- ### Target Device (QEMU and Real Hardware) -->

### 真实设备（ QEMU 和实体硬件）

<!-- Warning: On real hardware, `/dev/class/block/...` refers to **REAL** storage
devices (USBs, SSDs, etc). -->
警告：在真实硬件上，`/dev/class/block/...`指的是**真实**存储设备( USB、SSD 等)。

<!-- **BE CAREFUL NOT TO FORMAT THE WRONG DEVICE.** If in doubt, only run the
following commands through QEMU.
The `lsblk` command can be used to see more information about the devices
accessible from Zircon. -->

**注意不要格式化错误的设备**。如果有疑问，请仅通过QEMU运行以下命令。可以使用 `lsblk` 命令查看有关可从 Zircon 访问的设备的更多信息。

 <!-- * Within zircon, `lsblk` can be used to list the block devices currently on
   the system. On this example system below, `/dev/class/block/000` is a raw
   block device. -->

 * 在 Zircon 中，可以使用 `lsblk` 列出系统中当前的块设备。在下面的示例系统中，`/dev/class/block/000` 是一个真实的块设备。
  ```
  > lsblk
  ID  DEV      DRV      SIZE TYPE           LABEL
  000 block    block     16G
  ```

 <!-- * Let's add a GPT to this block device. -->

 * 将此块设备 GPT 格式化。

  ```
  > gpt init /dev/class/block/000
  ...
  > lsblk
  ID  DEV      DRV      SIZE TYPE           LABEL
  002 block    block     16G
  ```

 <!-- * Now that we have a GPT on this device, let's check what we can do with it.
   (NOTE: after manipulating the gpt, the device number may change. Use `lsblk`
   to keep track of how to refer to the block device). -->

 * 对这个块设备 GPT 格式化后，接下来检查是否生效(注：GPT 格式化后，设备号可能会更改。 使用 `lsblk` 跟踪引用块设备)。

  ```
  > gpt dump /dev/class/block/002
  blocksize=512 blocks=33554432
  Partition table is valid
  GPT contains usable blocks from 34 to 33554398 (inclusive)
  Total: 0 partitions
  ```

 <!-- * `gpt dump` tells us some important info: it tells us (1) How big blocks are,
   and (2) which blocks we can actually use.
   Let's fill part of the disk with a MinFS filesystem. -->

 * `gpt dump` 能够获得一些重要的信息：(1)块容量大小，(2)哪些块最终能被使用。使用 MinFS 格式的文件对磁盘进行填充。

  ```
  > gpt add 34 20000000 minfs /dev/class/block/002
  ```

 <!-- * Within Zircon, format the partition as MinFS. Using `lsblk` you should see
   a block device, which is the whole disk, and a slightly smaller device, which
   is the partition. In the above output, the partition is device 003, and would
   have the path `/dev/class/block/003` -->

 * 在 Zircon 中，使用 MinFS 来对分区进行格式化。可以使用 `lsblk` 来查看整个磁盘或者小一点的分区。如果格式化输出后分区设备时叫003，那么在路径中就会有`/dev/class/block/003`。

  ```
  > mkfs <PARTITION_PATH> minfs
  ```

 <!-- * If you want the device to be mounted automatically on reboot, use the GPT
   tool to set its type. As we did above, **you must** use `lsblk` **again**
   to locate the entry for the disk. We want to edit the type of the zero-th
   partition.  Here we use the keyword 'fuchsia-data' to set the type GUID, but
   if you wanted to use an arbitrary GUID you would supply it where
   'fuchsia-data' is used. -->

 * 如果你希望在启动的时候自动挂载设备，可以使用GPT工具设置设备的类型。设置后，**你还是需要**使用`lsblk`来重新分配你的磁盘。我们想对第0号分区进行类型编辑。那么我们可以使用关键字 'fuchsia-data' 来设置 GUID ，但是 GUID 类型需要是 'fuchsia-data' 已经支持的。

  ```
  > gpt edit 0 type fuchsia-data <DEVICE_PATH>
  ```

 <!-- * On any future boots, the partition will be mounted automatically at `/data`. -->

 * 启动后，分区将会被自动挂载到 `/data` 下面。

 <!-- * If you don't want the partition to be mounted automatically, you can update
   the visibility (or GUID) of the partition, and simply mount it manually. -->

 * 如果你不想分区被自动挂载，你可以更新分区（或 GUID ）的可见性,然后手动挂载。

  ```
  > mount <PARTITION_PATH> /data
  ```

 <!-- * Any files written to `/data` (the mount point for this GUID) will persist
   across boots. To test this, try making a file on the new MinFS volume,
   rebooting, and observing it still exists. -->

 * 任何写入`/data`(此 GUID 的挂载点)的文件都将在引导过程中保持不变。 要测试这一点，请尝试在新的 MinFS 卷上创建一个文件，重新启动，然后观察它仍然存在。

  ```
  > touch /data/foobar
  > dm reboot
  > ls /data
  ```

 <!-- * To find out which block device/file system is mounted at each subdirectory
   under a given path, use the following command: -->

 * 要找出在给定路径下的每个子目录中装载了哪个数据块设备/文件系统，请使用以下命令：

  ```
  > df <PATH>
  ```

<!-- ## Minfs operations -->

## Minfs 操作

<!-- The following section describes what IOs are performed to complete a simple end
user operation like read()/write(). -->

以下部分介绍执行哪些 IO 来完成简单的最终用户操作，如 read()/write()。

<!-- ### Assumptions -->

### 假设

<!-- * No operation, read or write, is cached or batched. Each of these operations
  are like calling with sync and direct io set.
* For rename: The destination file does not exist. Rename can delete a file if
  the destination of the rename operation is a valid file. This assumption keeps
  the math simple.
* The "Write" operation issues a single data block write to a previously
  unaccessed portion of the vnode.
* The "Overwrite" operation issues a single data block write to a portion of the
  block that has previously been allocated from an earlier "Write" operation. -->

* 无论是读操作还是写操作，都不会缓存或批处理。这些操作中的每一个都类似于使用异步和直接 IO 进行调用。
* 对于重命名：目标文件需要是不存在的。 如果重命名操作的目标是有效文件，则重命名可以删除文件。 这一假设使操作变得简单。
* “Write”操作向虚拟节点之前未被访问的部分发出单个数据块写入请求。
* “Overwrite”操作向虚拟节点之前已经分配的部分发出数据块写入请求。


<!-- ### Keys to the columns. -->

### 关键词

<!-- 1. OPERATION: The action requested by a client of the filesystem. -->

1.OPERATION：客户端向文件系统发出请求
<!-- 1. BLOCK TYPE: Each fileystem operation results in accessing one or more types
   of blocks. -->
1.BLOCK TYPE: 每个文件系统操作都会导致访问一种或多种类型的数据块
    <!-- * Data: Contains user data and directory entries.
    * Indirect: Indirect block in file block map tree
    * Dindirect: Double indirect block in file block map tree.
    * Inode table: Inode table block that holds one or more inodes.
    * Inode bitmap: Contains an array of bits each representing free/used state
      of inodes.
    * Data bitmap: Contains an array of bits each representing free/used state
      of data blocks.
    * Superblock: Contains data describing layout and state of the filesystem. -->
    * Data: 数据块，包含用户数据和目录条目。
    * Indirect: 一级文件映射块。
    * Dindirect: 二级文件映射块。
    * Inode table: 索引块，包含一个或多个索引.
    * Inode bitmap: 索引位图，用字节数组表示节点是空闲还是使用。
    * Data bitmap: 数据块位图，用字节数组表示数据块是空闲还是使用。
    * Superblock: 超级块，对文件系统的结构以及状态数据。
<!-- 1. IO TYPE: What type, read/write, of IO access it is. -->
1. IO TYPE: 它是哪种类型的读/写IO访问
<!-- 1. JOURNALED: Whether the IO will be journaled. Reads are not journaled but some
   of the writes are journaled. -->
1. JOURNALED: 是否记录 IO。Reads 表示不会记录读取，但会记录一些写入。
<!-- 1. CONDITIONALLY ACCESSED: Depending on the OPERATION's input parameter and
   state of the filesystem, some blocks are conditionally accessed.
    * No:   IO is always performed.
    * Yes:  Filesystem state and input parameters decide whether this IO is
      needed or not. -->
1. CONDITIONALLY ACCESSED: 根据 OPERATION 的输入参数和文件系统的状态，有条件地访问某些块。
    * No:   始终执行 IO。
    * Yes:  文件系统状态和输入参数决定是否需要此 IO。
<!-- 1. READ COUNT: Number of filesystem blocks read. -->
1. READ COUNT: 读取的文件系统数据块数。
<!-- 1. WRITE COUNT (忽略 JOURNAL): Number of filesystem blocks written. Writing
   to journal or journaling overhead are not counted towards this number. -->
1. WRITE COUNT (忽略 JOURNAL): 写入的文件系统数据块数，写入日志或日志开销不计入此数字。
<!-- 1. WRITE COUNT (WITH JOURNAL): Number of filesystem blocks written to journal
   and then to the final location. This does not include the blocks journal
   writes to maintain the journal state. -->
1. WRITE COUNT (JOURNAL): 写入日志然后写入最终位置的文件系统数据块数。这不包括用于维护日志状态的日志写入块。

<!-- A row \<operation\> Total, like "Create Total", gives the total number of blocks
read/written. For operations involving journaling, the journal writes two more
blocks, journal entry header and commit block, per operation. The number under
write count for <operations> Total is the sum of WRITE COUNT (WITH JOURNALING)
and journaling overhead, which is 2 blocks per operation. -->

\<请求\>统计，例如 "Create Total",给出了读/写的块总数。如关于日志的操作作，日志会在每个操作中再写入两个数据块，即日志条目标题和提交数据块。写<请求>总计数下的数字是写计数(写日志)和日志开销的总和，即每个操作2个数据块。

<!-- Superblock, Inode table, Inode bitmap, Data bitmap, and a part of Journal are
cached in memory while starting(mount/fsck) the filesystem. So, Read IOs are
never issued for those BLOCK TYPES. -->

超级块，索引表，索引位图，数据位图以及日志将在文件系统( mount/fsck )开始时被缓存到内存。因此，这些数据块类型永远不会发出读 IO。

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
