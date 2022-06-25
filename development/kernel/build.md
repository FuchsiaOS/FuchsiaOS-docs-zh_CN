# Kernel in the build

## Commandline options {#options}

Kernel commandline options are declared using the
[`kernel_cmdline`](/build/zbi/kernel_cmdline.gni) template:

```gn
import("//build/zbi/kernel_cmdline.gni")

kernel_cmdline("foobar") {
  args = [ "foobar=true" ]
}
```

A single target may include multiple options:

```gn
import("//build/zbi/kernel_cmdline.gni")

kernel_cmdline("debug") {
  args = [
    "debug.this=true",
    "debug.that=false",
  ]
}
```

The resulting GN labels should then be inserted into the build graph via a GN
argument. Note that options will be taken into account if they are within the
dependency tree defined by such a GN argument.

### Specifying options in board or product files

In the [board](/boards) or [product](/products) file, add the label(s) for the
desired cmdline option(s) to [`board_bootfs_labels`](/build/board.gni) and
[`product_bootfs_labels`](/build/product.gni) respectively.

To alter kernel options for the zedboot or recovery build, add the labels
respectively to [`board_zedboot_bootfs_labels`](/build/board.gni) and
[`board_recovery_bootfs_labels`](/build/board.gni).

### Specifying options locally

For local development, a list of strings that should be appended to the kernel
command line can be specified in the `dev_kernel_cmdline` GN variable, as
follows:

```posix-terminal
fx set ... --args='dev_kernel_cmdline=["my_option=value"]'
```

Alternatively, an existing `args.gn` file can be modified by running `fx args`
and adding or modifying a line as follows:

```gn
dev_kernel_cmdline = [
  "my_option=value"
]
```

To locally alter kernel options for the zedboot or recovery build, the
variables `dev_zedboot_kernel_cmdline` and `dev_recovery_kernel_cmdline` can
be used, respectively.
