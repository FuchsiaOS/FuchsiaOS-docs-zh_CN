The `qemu_edu` driver sample includes [tools][eductl_tools] for interacting with the
`qemu_edu` driver. Developers often include binary executables in a Fuchsia package and
run those executables as a component for testing and debugging drivers running in a
Fuchsia system.

In this driver sample, an executable named `eductl_tool` provides two options: `live` and
`fact`. The `live` command checks for the liveness of the `qemu_edu` driver in the system.
The `fact` command takes an integer as an additional argument. The value of the integer is
passed to the `qemu_edu` driver to be used as input for computing the factorial. The
driver computes the factorial and returns the result to the `fact` command, which then
prints the result on the terminal.

The tasks include:

*   Build and run `eductl_tool`.
*   Verify that this tool can interact with the `qemu_edu` driver.

Do the following:

1. Build and run `eductl_tool` (and run the `live` command):

   ```posix-terminal
   tools/bazel run //src/qemu_edu/tools:pkg.eductl_tool -- live
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel run //src/qemu_edu/tools:pkg.eductl_tool -- live
   ...
   INFO: Build completed successfully, 38 total actions
   Running workflow: pkg.eductl_tool_base
   Running task: pkg.debug_symbols_base (step 1/2)
   Running task: pkg.eductl_tool.run_base (step 2/2)
   added repository bazel.pkg.eductl.tool.runnable
   {{ '<strong>' }}Liveness check passed!{{ '</strong>' }}
   ```

   Verify that the line `Liveness check passed!` is printed in the end.

1. Run `eductl_tool` using `fact` and `12` as input:

   ```posix-terminal
   tools/bazel run //src/qemu_edu/tools:pkg.eductl_tool -- fact 12
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel run //src/qemu_edu/tools:pkg.eductl_tool -- fact 12
   ...
   INFO: Build completed successfully, 1 total action
   Running workflow: pkg.eductl_tool_base
   Running task: pkg.debug_symbols_base (step 1/2)
   Running task: pkg.eductl_tool.run_base (step 2/2)
   added repository bazel.pkg.eductl.tool.runnable
   {{ '<strong>' }}Factorial(12) = 479001600{{ '</strong>' }}
   ```

   The last line shows that the driver replied `479001600` as the result of
   the factorial to `eductl_tool`, which passed 12 as input to the driver.

1. View the device logs of the `qemu-edu` driver:

   ```posix-terminal
   tools/ffx log --tags qemu-edu dump
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --tags qemu-edu dump
   [2022-10-27 21:19:30.189][<ffx>]: logger started.
   [184.040][full-pkg-drivers:root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl][qemu-edu,driver][I]: [src/qemu_edu/drivers/qemu_edu.cc:65] edu device version major=1 minor=0
   [184.073][full-pkg-drivers:root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl][qemu-edu,driver][I]: [src/qemu_edu/drivers/qemu_edu.cc:117] Exported devfs_path=sys/platform/pt/PCI0/bus/00:06.0_/qemu-edu service_path=examples.qemuedu.Service/default/device
   [248.087][full-pkg-drivers:root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl][qemu-edu,driver][I]: [src/qemu_edu/drivers/edu_server.cc:59] Replying with result=true
   [255.504][full-pkg-drivers:root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl][qemu-edu,driver][I]: [src/qemu_edu/drivers/edu_device.cc:124] Replying with factorial=479001600
   ```

   Notice that more messages are now logged from the `qemu-edu` driver.

<!-- Reference links -->

[eductl_tools]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/tools
