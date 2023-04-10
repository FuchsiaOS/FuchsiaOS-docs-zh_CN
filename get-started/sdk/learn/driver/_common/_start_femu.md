## Start the emulator

Stop all emulator instance you may have currently running:

```posix-terminal
ffx emu stop --all
```

Start a new instance of the Fuchsia emulator with Driver Framework v2 enabled:

```posix-terminal
ffx emu start workstation_eng.qemu-x64 --headless \
  --kernel-args "driver_manager.use_driver_framework_v2=true" \
  --kernel-args "driver_manager.root-driver=fuchsia-boot:///#meta/platform-bus.cm" \
  --kernel-args "devmgr.enable-ephemeral=true"
```

Note: Driver Framework v2 is not enabled by default. The `--kernel-args` options
configure the emulator instance to use the latest driver framework.
