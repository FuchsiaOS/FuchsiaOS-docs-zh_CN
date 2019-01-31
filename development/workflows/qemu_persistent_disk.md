<!--# Running QEMU with persistent disks-->

# 在硬盘上运行 QEMU

<!--It's useful to run QEMU with persistent disks, like you'd have on actual
hardware.-->

在硬盘上运行 QEMU 就像有一个真正的硬件一样有用。

<!--Specifically you'd want a `/data` minfs partition and a `/blob` blobfs
partition, but `/system` and `/boot` from your local build. Here's how to do
that.-->

具体来说，你需要一个 `/data` minfs 分区和一个 `/blob` blobfs 分区，但是 `/system` 和 `/boot` 来自本地构建。这就是 QEMU 解决的问题。

<!--*NOTE:* Lines that begin with a `$` should be typed. Other lines are example
output.-->

*注意：*以 `$` 开头的是输入行，其他为输出行。

<!--## Create the disk image-->

## 创建磁盘镜像
<!--`blk.bin` is the name of the image that `frun` / `mrun` looks for when you pass
`-d`. Make a 1g one in your `$FUCHSIA_DIR`.-->

`blk.bin` 是 `frun` / `mrun` 命令传入 `-d` 时查找的镜像名称。在 `$FUCHSIA_DIR` 创建一个 1G 大小的文件。

Linux:
```
$ cd $FUCHSIA_DIR
$ truncate -s 1g blk.bin
```
macOS:
```
$ cd $FUCHSIA_DIR
$ mkfile -n 1g blk.bin
```

<!--## Start zircon-->
## 启动 Zircon
<!--You don't need a full Fuchsia UI to set up the disk image so start just zircon
but tell it to mount-->
你不需要完整的 Fuchsia UI 来设置磁盘镜像，所以只需要 Zircon 启动之后就可以开始安装。

```
$ mrun -d
[00000.000] 00000.00000> multiboot: info @ 0xffffff8000009500
[00000.000] 00000.00000> multiboot: cmdline @ 0xffffff8000253059
[00000.000] 00000.00000> multiboot: ramdisk @ 00254000..0ec607e0
[00000.000] 00000.00000> bootdata: @ 0xffffff8000254000 (245417952 bytes)
[00000.000] 00000.00000>
[00000.000] 00000.00000> welcome to lk/MP
(etc)
```

<!--## Initialize the GPT-->

## 初始化 GPT

<!--You blank `blk.bin` image needs a partition table.-->
清空 `blk.bin` 镜像需要分区表。

```
$ gpt init /dev/class/block/000
blocksize=0x200 blocks=2097152
WARNING: You are about to permanently alter /dev/class/block/000

Type 'y' to continue, any other key to cancel
invalid header magic!
[00031.068] 02004.02044> device: 0x4e3af554b000(sata0): ref=0, busy, not
releasing
[00031.070] 01043.01046> devcoord: drv='block' bindable to dev='sata0'
[00031.072] 01043.01166> devmgr: new block device: /dev/class/block/001
GPT changes complete.
[00031.077] 01043.01166> devmgr: /dev/class/block/001: GPT?
[00031.078] 01043.01046> devcoord: dc_bind_device() '/boot/driver/gpt.so'
[00031.078] 01043.01046> devcoord: drv='gpt' bindable to dev='block'
```

<!--## Create the partitions-->
## 创建分区

<!--Now that there's a blank partition table, create a 500m data and 500m blob
partition: -->

现在有了一个空的分区表，重新分配为 500M 的数据区和 500M 的 blob 分区

```
$ gpt repartition /dev/class/block/001 data data 500m blob blobfs 500m
blocksize=0x200 blocks=2097152
data: 524288000 bytes, 1024000 blocks, 48-1024063
blob: 524288000 bytes, 1024000 blocks, 1024064-2048079
[00242.582] 02004.02044> device: 0x4e3af554b000(sata0): ref=0, busy, not
releasing
[00242.584] 01043.01046> devcoord: drv='block' bindable to dev='sata0'
[00242.587] 01043.01166> devmgr: new block device: /dev/class/block/002 GPT
changes complete.
[00242.594] 01043.01166> devmgr: /dev/class/block/002: GPT?
[00242.596] 01043.01046> devcoord: dc_bind_device() '/boot/driver/gpt.so'
[00242.596] 01043.01046> devcoord: drv='gpt' bindable to dev='block'
[00242.619] 01043.01046> devcoord: drv='block' bindable to dev='part-000'
[00242.622] 01043.01166> devmgr: new block device: /dev/class/block/003
[00242.624] 01043.01046> devcoord: drv='block' bindable to dev='part-001'
[00242.628] 01043.01166> devmgr: new block device: /dev/class/block/004

```

<!--## Create the filesystems-->
## 创建文件系统

<!--And now create the minfs and blobfs filesystems on the new partitions:-->

在新的分区上创建 minfs 文件系统和 blobfs 文件系统。

```
$ mkfs /dev/class/block/003 minfs
$ mkfs /dev/class/block/004 blobfs
```

<!--You're done. Now reboot and pass `-d` to `frun` to run with persistent disks.-->

完成后你就可以重启系统传入 `-d` 参数给 `frun` 命令运行硬盘。

<!--## Keeping a spare-->
## 备份

<!--Sometimes you will want to make a fresh clean disk image. After you have created
a new, empty `blk.bin` you can gzip it as a backup:-->

有时你会需要一个新的干净的磁盘镜像。那么在你创建完镜像后你可以压缩 `blk.bin` 作为备份。

```
$ gzip blk.bin
```
<!--Then to get a decompressed blk.bin without getting rid of your gzipped backup:-->

那么在你需要时你可以解压压缩的 blk.bin。

```
$ gunzip -k blk.bin.gz
```

<!--## Multiple instances-->
## 多种实例

<!--If you want to run multiple instances of QEMU they must not be trying to use the
same disk image. Each much have their own disk image. You can follow the
instructions above but pass `-D other-disk-name.bin` to `mrun` and `frun` to
specify the alternative location.-->
如果想要运行多种 QEMU 的实例，你需要使用不同的磁盘镜像。每种都有不同的磁盘镜像。你可以按照上面的操作说明传入 `-D other-disk-name.bin` 到 `mrun` 和 `frun` 指定其他位置。