<!--

// LINT.IfChange

-->

# i2cutil

List, read from, and write to I2C devices.

## Usage

```none
i2cutil read <device> <address> [<address>...]
i2cutil write <device> <address> [<address>...] <data> [<data>...]
i2cutil transact <device> (r <bytes>|w <address> [<address>...] [<data>...])...
i2cutil dump <device> <address> <count>
i2cutil list
i2cutil ping
i2cutil help
```

## Commands

### read {#read}

```none
i2cutil read <device> <address> [<address>...]
```

Read one byte from an I2C device. Use [`transact`](#transact)
to read multiple bytes. `<device>` can be the full path of
a devfs node (example: `/dev/class/i2c/031`) or only the
devfs node's index (example: `31`) or it can simply be the
device's friendly named obtained via `i2cutil list`. Use
[`ping`](#ping) to get devfs node paths and indexes.
`<address>` is the internal register of `<device>` to read
from. Use multiple `<address>` values to access a multi-byte
(little-endian) register address. For example
`i2cutil read 4 0x20 0x3D` to read the register at `0x203D`.

### write {#write}

```none
i2cutil write <device> <address> [<address>...] <data> [<data>...]
```

Write one or more bytes (`<data>`) to an I2C device. See the
[`read`](#read) description for explanations of `<device>`
and `<address>`.

### transact {#transact}

```none
i2cutil transact <device> (r <bytes>|w <address> [<address>...] [<data>...])...
```

Perform a transaction with multiple segments. Each segment can be a write
(`w`) or a read (`r`).

### dump {#dump}

```none
i2cutil dump <device> <start> <count>
```

Reads and prints `<count>` registers from `<device>` starting at the address
indicated by `<address>`

### list {#list}

```none
i2cutil list
```

Lists all I2C devices available on the system. The friendly name of the device will
also be listed if one is provided. Otherwise the device will appear as `(ANONYMOUS)`.

### ping {#ping}

```none
i2cutil ping
```

Ping all I2C devices under devfs path `/dev/class/i2c` by
reading from each device's `0x00` address.

### help {#help}

```none
i2cutil help
```

Print help text.

## Examples

### Read one byte

Read one byte from the register at `0x20` of the I2C device
represented by devfs node index `4`:

```none
$ i2cutil read 4 0x20
```

### Read three bytes

Read three bytes from the register at `0x20` of the I2C device
represented by devfs node index `4`:

```none
$ i2cutil transact 4 w 0x20 r 3
```

### Dump nine registers starting at address 0x10

Dump nine registers starting at address 0x10.

```none
$ i2cutil dump pmic 0x10 9
0x10: 0x00
0x11: 0x00
0x12: 0x00
0x13: 0x00
0x14: 0x00
0x15: 0x00
0x16: 0x00
0x17: 0x00
0x18: 0x00
```

### Read one byte from a multi-byte address

Read one byte from the register at the multi-byte address `0x203D`
of the I2C device represented by devfs node index `4`:

```none
$ i2cutil read 4 0x20 0x3D
```

### Read one byte using a devfs node path

Read one byte from the register at the multi-byte address `0x203D`
of the I2C device represented by the devfs node path
`/dev/class/i2c/004`:

```none
$ i2cutil read /dev/class/i2c/004 0x20 0x3D
```

### Read one byte using the device's friendly name

Read one byte from the register at the multi-byte address `0x203D`
of the I2C device named `temp_sensor`:

```none
$ i2cutil list
378: temp_sensor
379: (ANONYMOUS)
380: humidity_sensor
381: pmic
$ i2cutil read temp_sensor 0x20 0x3D
```

### Write one byte

Write one byte `0x12` to the register at `0x2C` of the I2C device represented by
devfs node index `3`:

```none
$ i2cutil write 3 0x2C 0x12
```

### Write two bytes using a devfs node path

Write two bytes `0x121B` to the register at `0x2C` of the
I2C device represented by devfs node index `/dev/class/i2c/003`:

```none
$ i2cutil write /dev/class/i2c/003 0x2C 0x12 0x1B
```

Caution: This exact same command could mean "write byte `0x1B`
to register `0x2C12` for a different I2C device. The meaning
of the arguments depends on the I2C device.

### Ping all I2C devices

Read the `0x00` address of all devices found under
`/dev/class/i2c`.

```none
$ i2cutil ping
/dev/class/i2c/821: OK
/dev/class/i2c/822: OK
/dev/class/i2c/823: OK
/dev/class/i2c/824: OK
Error ZX_ERR_TIMED_OUT
/dev/class/i2c/825: ERROR
```

### List all I2C devices

List all I2C devices on the system along with their friendly names if one is available.

```none
$ i2cutil list
378: temp_sensor
379: (ANONYMOUS)
380: humidity_sensor
381: pmic
```

## Notes

### Source code

Source code for `i2cutil`: [`//src/devices/i2c/bin/i2cutil.cc`](/src/devices/i2c/bin/i2cutil.cc)

<!--

// LINT.ThenChange(//src/devices/i2c/bin/i2cutil.cc)

-->
