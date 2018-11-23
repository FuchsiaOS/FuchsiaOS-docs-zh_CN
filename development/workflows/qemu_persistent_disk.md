# Running QEMU with persistent disks

It's useful to run QEMU with persistent disks, like you'd have on actual
hardware.

Specifically you'd want a `/data` minfs partition and a `/blob` blobfs
partition, but `/system` and `/boot` from your local build. Here's how to do
that.

*NOTE:* Lines that begin with a `$` should be typed. Other lines are example
output.

## Create the disk image
`blk.bin` is the name of the image that `frun` / `mrun` looks for when you pass
`-d`. Make a 1g one in your `$FUCHSIA_DIR`.

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

## Start zircon
You don't need a full Fuchsia UI to set up the disk image so start just zircon
but tell it to mount ```
$ mrun -d
[00000.000] 00000.00000> multiboot: info @ 0xffffff8000009500
[00000.000] 00000.00000> multiboot: cmdline @ 0xffffff8000253059
[00000.000] 00000.00000> multiboot: ramdisk @ 00254000..0ec607e0
[00000.000] 00000.00000> bootdata: @ 0xffffff8000254000 (245417952 bytes)
[00000.000] 00000.00000>
[00000.000] 00000.00000> welcome to lk/MP
(etc)
```

## Initialize the GPT

You blank `blk.bin` image needs a partition table.
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

## Create the partitions

Now that there's a blank partition table, create a 500m data and 500m blob
partition: ```
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

## Create the filesystems

And now create the minfs and blobfs filesystems on the new partitions:

```
$ mkfs /dev/class/block/003 minfs
$ mkfs /dev/class/block/004 blobfs
```

You're done. Now reboot and pass `-d` to `frun` to run with persistent disks.

## Keeping a spare

Sometimes you will want to make a fresh clean disk image. After you have created
a new, empty `blk.bin` you can gzip it as a backup:
```
$ gzip blk.bin
```
Then to get a decompressed blk.bin without getting rid of your gzipped backup:
```
$ gunzip -k blk.bin.gz
```

## Multiple instances

If you want to run multiple instances of QEMU they must not be trying to use the
same disk image. Each much have their own disk image. You can follow the
instructions above but pass `-D other-disk-name.bin` to `mrun` and `frun` to
specify the alternative location.
