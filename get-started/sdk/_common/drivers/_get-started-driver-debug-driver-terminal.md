Use the Fuchsia debugger ([`zxdb`][zxdb-user-guide]) to step through the
sample driver’s code.

The tasks include:

*   Identify the driver host (which is a component) of the `qemu_edu`
    driver.
*   Start the Fuchsia debugger and connect it to the emulator instance.
*   Attach the debugger to the driver host.
*   Set a breakpoint on the driver’s code.
*   Run `eductl_tool`, which triggers the driver to execute its
    instructions.
*   Step through the driver’s code.

Do the following:

1. View the list of the running driver hosts:

   ```posix-terminal
   tools/ffx driver list-hosts
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx driver list-hosts
   Driver Host: 5507
       fuchsia-boot:///#meta/bus-pci.cm
       fuchsia-boot:///#meta/display.cm
       fuchsia-boot:///#meta/goldfish-display.cm
       fuchsia-boot:///#meta/goldfish.cm
       fuchsia-boot:///#meta/goldfish_control.cm
       fuchsia-boot:///#meta/goldfish_sensor.cm
       fuchsia-boot:///#meta/goldfish_sync.cm
       fuchsia-boot:///#meta/hid.cm
       fuchsia-boot:///#meta/platform-bus-x86.cm
       fuchsia-boot:///#meta/platform-bus.cm
       unbound

   ...

   Driver Host: 25673
       fuchsia-pkg://fuchsia.com/virtual_audio#meta/virtual_audio_driver.cm
       unbound

   Driver Host: 85211
       fuchsia-pkg://bazel.pkg.component.runnable/qemu_edu#meta/qemu_edu.cm
   ```

   Make a note of the PID of the `qemu_edu` driver host (`85211` in the
   example above).

1. Start the Fuchsia debugger:

   ```posix-terminal
   tools/ffx debug connect
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx debug connect
   Connecting (use "disconnect" to cancel)...
   Connected successfully.
   👉 To get started, try "status" or "help".
   [zxdb]
   ```

1. Attach the debugger to the `qemu_edu` driver host:

   <pre class="devsite-click-to-copy">
   <span class="no-select">[zxdb] </span>attach <var>PID</var>
   </pre>

   Replace `PID` with the PID of the `qemu_edu` driver host identified
   in step 1, for example:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] attach 85211
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] attach 85211
   Attached Process 1 state=Running koid=85211 name=driver_host2.cm component=driver_host2.cm
   Downloading symbols...
   Symbol downloading complete. 4 succeeded, 0 failed.
   [zxdb]
   ```

1. Set a breakpoint at the driver’s `HandleIrq()` function:

   <pre class="devsite-click-to-copy">
   <span class="no-select">[zxdb] </span>break QemuEduDevice::HandleIrq
   </pre>

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] break QemuEduDevice::HandleIrq
   Created Breakpoint 1 @ QemuEduDevice::HandleIrq
      94 void QemuEduDevice::HandleIrq(async_dispatcher_t* dispatcher, async::IrqBase* irq,
    {{ '<strong>' }}◉ 95                               zx_status_t status, const zx_packet_interrupt_t* interrupt) { {{ '</strong>' }}
      96   irq_.ack();
   [zxdb]
   ```

1. In different terminal, run `eductl_tool` (using `fact` and `12` as input)
   to interact with the driver:

   Note:  In this new terminal, make sure that you change to the same work
   directory (for instance, `cd $HOME/fuchsia-drivers`).

   ```posix-terminal
   tools/bazel run //src/qemu_edu/tools:pkg.eductl_tool -- fact 12
   ```

   Unlike in the previous section, after printing output similar to the following,
   the command now waits:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel run //src/qemu_edu/tools:pkg.eductl_tool -- fact 12
   ...
   INFO: Build completed successfully, 1 total action
   Running workflow: pkg.eductl_tool_base
   Running task: pkg.debug_symbols_base (step 1/2)
   Running task: pkg.eductl_tool.run_base (step 2/2)
   added repository bazel.pkg.eductl.tool.runnable
   ```

   In the `zxdb` terminal, verify that the debugger is stopped at the
   `HandleIrq()` function, for example:

   ```none {:.devsite-disable-click-to-copy}
   🛑 thread 2 on bp 1 edu_device::QemuEduDevice::HandleIrq(edu_device::QemuEduDevice*, async_dispatcher_t*, async::IrqBase*, zx_status_t, zx_packet_interrupt_t const*) • edu_device.cc:95
      93 // Respond to INTx interrupts triggered by the device, and return the compute result.
      94 void QemuEduDevice::HandleIrq(async_dispatcher_t* dispatcher, async::IrqBase* irq,
    {{ '<strong>' }}▶ 95                               zx_status_t status, const zx_packet_interrupt_t* interrupt) { {{ '</strong>' }}
      96   irq_.ack();
      97   if (!pending_callback_.has_value()) {
   [zxdb]
   ```

1. In the `zxdb` terminal, view the source code around the current breakpoint:

   <pre class="devsite-click-to-copy">
   <span class="no-select">[zxdb] </span>list
   </pre>

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] list
       90   pending_callback_ = std::move(callback);
       91 }
       92
       93 // Respond to INTx interrupts triggered by the device, and return the compute result.
       94 void QemuEduDevice::HandleIrq(async_dispatcher_t* dispatcher, async::IrqBase* irq,
    {{ '<strong>' }}▶  95                               zx_status_t status, const zx_packet_interrupt_t* interrupt) { {{ '</strong>' }}
       96   irq_.ack();
       97   if (!pending_callback_.has_value()) {
       98     FDF_LOG(ERROR, "Received unexpected interrupt!");
       99     return;
      100   }
      101   auto callback = std::move(*pending_callback_);
      102   pending_callback_ = std::nullopt;
      103   if (status != ZX_OK) {
      104     FDF_SLOG(ERROR, "Failed to wait for interrupt", KV("status", zx_status_get_string(status)));
      105     callback(zx::error(status));
   [zxdb]
   ```

1. In the `zxdb` terminal, step through the `HandleIrq()` function
   using the `next` command until the value of `factorial` is computed and
   the callback is invoked (that is, until the line 126 is reached):

   <pre class="devsite-click-to-copy">
   <span class="no-select">[zxdb] </span>next
   </pre>

   The last `next` command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   ...
   [zxdb] next
   🛑 thread 2 edu_device::QemuEduDevice::HandleIrq(edu_device::QemuEduDevice*, async_dispatcher_t*, async::IrqBase*, zx_status_t, zx_packet_interrupt_t const*) • edu_device.cc:126
      124   FDF_SLOG(INFO, "Replying with", KV("factorial", factorial));
      125   callback(zx::ok(factorial));
    {{ '<strong>' }}▶ 126 } {{ '</strong>' }}
      127 // [END compute_factorial]
      128
   [zxdb]
   ```

   In the other terminal, after the `HandleIrq()` function invokes the
   callback, verify that `eductl_tool` prints the factorial result and exits:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel run //src/qemu_edu/tools:pkg.eductl_tool -- fact 12
   ...
   INFO: Build completed successfully, 1 total action
   Running workflow: pkg.eductl_tool_base
   Running task: pkg.debug_symbols_base (step 1/2)
   Running task: pkg.eductl_tool.run_base (step 2/2)
   added repository bazel.pkg.eductl.tool.runnable
   {{ '<strong>' }}Factorial(12) = 479001600{{ '</strong>' }}
   $
   ```

1. In the `zxdb` terminal, type `exit` or press `Ctrl-D` to exit the debugger.

   Note: For more information on usages and best practices on `zxdb`, see the
   [zxdb user guide][zxdb-user-guide].

<!-- Reference links -->

[zxdb-user-guide]: /docs/development/debugger/README.md
