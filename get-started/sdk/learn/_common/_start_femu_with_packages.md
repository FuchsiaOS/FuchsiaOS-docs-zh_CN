### Start the emulator

If you do not already have an instance running, start FEMU with networking
support:

```posix-terminal
ffx emu start --headless workstation.qemu-x64
```

Once the emulator is running, register your local package server with the
emulator instance:

```posix-terminal
ffx target repository register -r fuchsiasamples.com
```
