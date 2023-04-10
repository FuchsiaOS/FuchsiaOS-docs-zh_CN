Update the source code of the sample driver and reload it to the emulator
instance.

The tasks include:

*   Restart the emulator instance to unload the `qemu_edu` driver.
*   Update the source code of the `qemu_edu` driver.
*   Load the updated driver.
*   Run `eductl_tool` to verify the change.

Do the following:

1. Stop the emulator instance:

   ```posix-terminal
   tools/ffx emu stop
   ```

   This command stops the currently running emulator instance.

1. Start a new instance of the Fuchsia emulator:

   ```posix-terminal
   tools/ffx emu start workstation_eng.qemu-x64 --headless \
     --kernel-args "driver_manager.use_driver_framework_v2=true" \
     --kernel-args "driver_manager.root-driver=fuchsia-boot:///#meta/platform-bus.cm" \
     --kernel-args "devmgr.enable-ephemeral=true"
   ```

   This command starts a headless emulator instance running the Workstation
   prebuilt image.

1. Use a text editor to open the `edu_device.cc` file of the sample driver, for example:

   ```posix-terminal
   nano src/qemu_edu/drivers/edu_device.cc
   ```

1. In the `QemuEduDevice::HandleIrq` function,
   between the line `uint32_t factorial = mmio_->Read32(kFactorialComputationOffset);`
   (Line 123) and the line `FDF_SLOG(INFO, "Replying with", KV("factorial", factorial));`
   (Line 124), add the following line:

   ```
   factorial=12345;
   ```

   The function should look like below:

   ```none {:.devsite-disable-click-to-copy}
   void QemuEduDevice::HandleIrq(async_dispatcher_t* dispatcher, async::IrqBase* irq,
                                 zx_status_t status, const zx_packet_interrupt_t* interrupt) {
     irq_.ack();
     if (!pending_callback_.has_value()) {
       FDF_LOG(ERROR, "Received unexpected interrupt!");
       return;
     }
     auto callback = std::move(*pending_callback_);
     pending_callback_ = std::nullopt;
     if (status != ZX_OK) {
       FDF_SLOG(ERROR, "Failed to wait for interrupt", KV("status", zx_status_get_string(status)));
       callback(zx::error(status));
       return;
     }

     // Acknowledge the interrupt with the edu device.
     auto int_status = mmio_->Read32(kInterruptStatusRegisterOffset);
     mmio_->Write32(int_status, kInterruptAcknowledgeRegisterOffset);

     // Deassert the legacy INTx interrupt on the PCI bus.
     auto irq_result = pci_->AckInterrupt();
     if (!irq_result.ok() || irq_result->is_error()) {
       FDF_SLOG(ERROR, "Failed to ack PCI interrupt",
                KV("status", irq_result.ok() ? irq_result->error_value() : irq_result.status()));
       callback(zx::error(ZX_ERR_IO));
       return;
     }

     // Reply with the result.
     uint32_t factorial = mmio_->Read32(kFactorialComputationOffset);
     {{ '<strong>' }}factorial=12345;{{ '</strong>' }}
     FDF_SLOG(INFO, "Replying with", KV("factorial", factorial));
     callback(zx::ok(factorial));
   }
   ```

   The function is now updated to always return the value of `12345`.

1. Save the file and close the text editor.

1. Rebuild and run the modified sample driver:

   ```posix-terminal
   tools/bazel run //src/qemu_edu/drivers:pkg.component
   ```

1. Run `eductl_tool` using `fact` and `12` as input:

   ```posix-terminal
   tools/bazel run //src/qemu_edu/tools:pkg.eductl_tool -- fact 12
   ```

   This command now prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel run //src/qemu_edu/tools:pkg.eductl_tool -- fact 12
   ...
   INFO: Build completed successfully, 1 total action
   Running workflow: pkg.eductl_tool_base
   Running task: pkg.debug_symbols_base (step 1/2)
   Running task: pkg.eductl_tool.run_base (step 2/2)
   added repository bazel.pkg.eductl.tool.runnable
   {{ '<strong>' }}Factorial(12) = 12345{{ '</strong>' }}
   ```

   The last line shows that the `qemu_edu` driver replied with the
   hardcoded value of `12345` to `eductl_tool`.
