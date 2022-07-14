### Start the emulator

If you do not already have an instance running, start FEMU with networking
support:

```posix-terminal
ffx emu start --headless workstation_eng.qemu-x64
```

Once the emulator is running, create a local package repository and register it
with the emulator instance:

```posix-terminal
ffx repository add-from-pm $HOME/.package_repos/sdk-samples -r fuchsiasamples.com
```

```posix-terminal
ffx target repository register -r fuchsiasamples.com
```
