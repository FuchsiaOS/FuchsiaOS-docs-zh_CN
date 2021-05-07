# bootserver

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

usage:   bootserver [ <option> ]* [<kernel>] [ <ramdisk> ] [ -- [ <kerneloption> ]* ]
```

__options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -1         only boot once, then exit
  -a         only boot device with this IPv6 address
  -b <sz>    tftp block size (default=1428, ignored with --netboot)
  -i <NN>    number of microseconds between packets
             set between 50-500 to deal with poor bootloader network stacks (default=20)
             (ignored with --tftp)
  -n         only boot device with this nodename
  -w <sz>    tftp window size (default=256, ignored with --netboot)
  --board_name <name>      name of the board files are meant for
  --boot <file>            use the supplied file as a kernel
  --fvm <file>             use the supplied file as a sparse FVM image (up to 4 times)
  --bootloader <file>      use the supplied file as a BOOTLOADER image
  --firmware <file>        use the supplied file as a FIRMWARE image of default type
  --firmware-<type> <file> use the supplied file as a FIRMWARE image of the given type
  --zircona <file>         use the supplied file as a ZIRCON-A ZBI
  --zirconb <file>         use the supplied file as a ZIRCON-B ZBI
  --zirconr <file>         use the supplied file as a ZIRCON-R ZBI
  --vbmetaa <file>         use the supplied file as a AVB vbmeta_a image
  --vbmetab <file>         use the supplied file as a AVB vbmeta_b image
  --vbmetar <file>         use the supplied file as a AVB vbmeta_r image
  --authorized-keys <file> use the supplied file as an authorized_keys file
  --init-partition-tables <path>  initialize block device specified with partition tables
  --wipe-partition-tables <path>  wipe partition tables from block device specified
  --fail-fast  exit on first error
  --netboot    use the netboot protocol
  --tftp       use the tftp protocol (default)
  --nocolor    disable ANSI color (false)
  --allow-zedboot-version-mismatch warn on zedboot version mismatch rather than fail
  --fail-fast-if-version-mismatch  error if zedboot version does not match
  --no-bind    do not bind to bootserver port. Should be used with -a <IPV6>
```

