## Restart the emulator

Shut down any existing emulator instances:

```posix-terminal
ffx emu stop --all
```

Start a new instance of the Fuchsia emulator with driver framework enabled:

```posix-terminal
ffx emu start workstation_eng.qemu-x64 --headless \
 --kernel-args "driver_manager.use_driver_framework_v2=true" \
 --kernel-args "driver_manager.root-driver=fuchsia-boot:///#meta/platform-bus.cm" \
 --kernel-args "devmgr.enable-ephemeral=true"
```
