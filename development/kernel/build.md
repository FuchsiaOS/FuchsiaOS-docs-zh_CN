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

Create a `BUILD.gn` file somewhere under `//local` to host the options targets.
Note that this folder is not tracked by git and therefore might not exist yet in
your checkout.
Use [`dev_bootfs_labels`](/build/dev.gni) to inject the options into the build
graph via `fx set`:

```posix-terminal
fx set ... --args='dev_bootfs_labels=["//local/path/to/my:options"]'
```

To locally alter kernel options for the zedboot or recovery build, follow the
same procedure but use respectively
[`dev_zedboot_bootfs_labels`](/build/dev.gni) and
[`dev_recovery_bootfs_labels`](/build/dev.gni) to introduce the options into the
build.
