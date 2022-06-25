# Nand testing

Warning: Most of these tests are destructive in nature.

## Accessing the desired device

In order to test a particular device, that device must not be in use by the rest
of the system. There are two ways to make that happen:

* Prevent other drivers from binding to the device. This may involve building
  the system with modified binding rules for the driver that normally binds
  to the desired device, or passing kernel command line arguments to disable
  that driver.

* Unbind devices that are bound to the desired device.

For example, in order to use a test tool against the core nand driver, nandpart
devices may be removed like so:

```shell
$ unbind /dev/sys/platform/05:00:f/aml-raw_nand/nand/fvm
```

Warning: Before removing a particular device, remove its descendants. By
extension, file systems must be unmounted before a block device is removed.
Note that this requirement is likely to render a running system unusable, as
the backing for the OS may be going away. Netboot may be the only viable option.

Note that all other devices created by nandpart must also be removed. Use
`driver dump` to inspect the device tree.

## Protocol testing

`nand-test` is an integration test that performs basic tests of nand protocol
drivers.

For example, this command will test an existing ram-nand device making sure the
test does not modify anything outside blocks [100, 109]:

```shell
$ /boot/test/sys/nand-test --device /dev/sys/platform/00:00:2e/nand-ctl/ram-nand-0 --first-block 100 --num-blocks 10
```

## Correctness testing

`nand-util` is a troubleshooting tool that can perform a simple read-reliability
test.

```shell
$ nand-util --device /dev/sys/platform/00:00:2e/nand-ctl/ram-nand-0 --check
```

## Inspection / manipulation

```shell
$ nand-util --device /dev/sys/platform/05:00:f/aml-raw_nand/nand --info
$ nand-util --device /dev/sys/platform/05:00:f/aml-raw_nand/nand/fvm --read --block 1 --page 2
```

## Grab an image

`nand-util` can also be used to grab an image of the nand contents:

Note: If a file system is already mounted, unbind will fail, and forcing it to work is
likely to render the system unusable. Remember to netboot or use Zedboot as
needed.

```shell
$ unbind /dev/sys/platform/05:00:f/aml-raw_nand/nand/fvm/ftl/block
$ unbind /dev/sys/platform/05:00:f/aml-raw_nand/nand/fvm/ftl
$ nand-util --device /dev/sys/platform/05:00:f/aml-raw_nand/nand/fvm --save --file /tmp/image
```

Transfer the image file to the host:

```shell
$ out/default/host-tools/netcp :/tmp/image /tmp/saved_image_file
```

## Replay

A saved nand image can be loaded on top of a ram-nand device using nand-loader.

First, transfer the image to a device running Zircon. For example, on the host:

```shell
echo /nand.dmp=/tmp/saved_image_file > /tmp/manifest.txt
out/default/host-tools/minfs /tmp/image.dsk create --manifest /tmp/manifest.txt
fx set bringup.x64
fx build
fx qemu -k -- -hda /tmp/image.dsk
```

Then, inside zircon:

```shell
$ mkdir data/a
$ mount /dev/class/block/000 data/a
$ nand-loader data/a/nand.dmp
```
