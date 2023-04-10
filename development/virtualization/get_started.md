# Getting Started with Virtualization

## Supported Hardware

Virtualization is supported on Intel devices with VMX enabled and ARMv8.0 and
above devices that boot into EL2. The following hardware will have the best
support due to being included in daily automated and manual testing:

* Pixelbook Go m3
* Intel NUC7 (NUC7i5DNHE)

The following hardware is also often used, but does not have the same level of
automated test coverage.

* x64 QEMU/Nested VMX

Some virtualization features will work when using the Fuchsia emulator with
nested VMX enabled. Notably, Vulkan acceleration using virtmagma will not be
available to any guests when running in the Fuchsia emulator.

## Supported Guests

While arbitrary Linux guests may run on Fuchsia, the following guest
configurations are tested in CI on Fuchsia:

* Zircon Guest - A minimal fuchsia system that boots to a zircon virtcon.
* Debian Guest - An Debian bullseye guest.
* Termina Guest - A linux guest that contains additional features for Vulkan and
  window manager integration, based on the [Termina VM][ref.termina]{:.external}
  from ChromeOS.

Note: The Debian Guest package expects the Linux kernel binaries and userspace
image to be in `//prebuilt/virtualization/packages/debian_guest`. You should
create them before running fx build by following the instructions in
[debian_guest/README.md][ref.debian_guest_readme]. Googlers: You don't need to
do this, the prebuilt images are downloaded from CIPD by Jiri.

## Build Fuchsia with Virtualization
For each guest operating system, there is a guest manager and a core shard that
must be included in the build.

Below, {{ '<var>' }}PRODUCT{{ '</var>' }} is typically one of `core`,
`workstation_eng`, `workstation_userdebug` and {{ '<var>' }}BOARD{{ '</var>' }}
is typically one of `x64`, `chromebook-x64`, `sherlock`.


```sh
$ fx set {{ '<var>' }}PRODUCT{{ '</var>' }}.{{ '<var>' }}BOARD{{ '</var>' }} \
    # For Debian Guest
    --with //src/virtualization/bundles:debian \
    --args='core_realm_shards += [ "//src/virtualization/bundles:debian_core_shards" ]' \
    # For Zircon Guest
    --with //src/virtualization/bundles:zircon \
    --args='core_realm_shards += [ "//src/virtualization/bundles:zircon_core_shards" ]' \
    # For Termina Guest
    --with //src/virtualization/bundles:termina \
    --args='core_realm_shards += [ "//src/virtualization/bundles:termina_core_shards" ]'
```

Alternatively, you can enable all the guests with the following command:

```sh
$ fx set {{ '<var>' }}PRODUCT{{ '</var>' }}.{{ '<var>' }}BOARD{{ '</var>' }} \
    --with //src/virtualization/bundles:all_guests \
    --args='core_realm_shards += [ "//src/virtualization/bundles:all_core_shards" ]'
```

## Launching a Guest from the CLI
You can launch a guest using the `guest` CLI tool. The tool will launch the
guest and then provide access to a virtio-console over stdio:

### Launch Debian Guest:

```none
(fuchsia) $ guest launch debian
Starting Debian
$ uname -a Linux machina-guest 5.10.0-13-amd64 #1 SMP Debian 5.10.106-1 (2022-03-17) x86_64 GNU/Linux
```

{# Allow the '{{{' below: #}

{% verbatim %}

### Launch Zircon Guest

```none
(fuchsia) $ guest launch zircon
Starting zircon
physboot: {{{reset}}}
physboot: {{{module:0:physboot:elf:9f2c4d6615bd603d}}}
physboot: {{{mmap:0x100000:0x14a100:load:0:rwx:0x0}}}
physboot: | Physical memory range                    | Size    | Type
physboot: | [0x0000000000008000, 0x0000000000080000) |    480K | free RAM
physboot: | [0x0000000000100000, 0x00000000001cd000) |    820K | phys kernel image
physboot: | [0x00000000001cd000, 0x000000000024a000) |    500K | free RAM
physboot: | [0x000000000024a000, 0x000000000024a100) |    256B | phys kernel image
physboot: | [0x000000000024a100, 0x000000000024b000) |   3840B | free RAM
…
```
{# Re-enable variable substitution #}

{% endverbatim %}

### Launch Termina Guest

```none
(fuchsia) $ guest launch termina
Starting Termina
…
```

On products with a UI Stack (ex: `workstation`), the debian and zircon guests
will also create a window that displays a virtual framebuffer powered by a
virtio-gpu. Input from that window will also be sent to the guest as a virtual
keyboard.

## Running on Workstation

The Workstation product supports a more integrated Linux guest via the ‘Linux
Terminal’ button in the launcher, powered by the `termina_guest` package. The
`termina_guest` supports Vulkan acceleration inside the guest (on supported
Intel devices), and also window-manager pass-through so that Linux
applications will have their own Fuchsia Views.

```sh
# The Linux Terminal is enabled by default on _userdebug builds.
$ fx set workstation_userdebug.x64

# The Linux Terminal is disabled by on _eng builds so we need to re-add it:
$ fx set workstation_eng.x64 \
        --with-base //src/virtualization/packages/termina_guest
```

Note: The Linux Terminal requires a hardwired network connection to access the
internet. If your device is connected to WiFi the Linux Terminal may fail to
start. To use the Linux Terminal, some binaries need to be downloaded from the
internet so this feature will not be functional until that initialization has
been performed using a hard-wired network connection.

Once you have built `workstation` you can launch the in one of two ways:

1. If you have a `Linux Terminal` button in your sysui, you can click that and
the VM will start and drop you into a linux shell once it has booted.
1. If you don't have the `Linux Terminal` button, you can open the Fuchsia
`Terminal` and run `guest vsh -c` to accomplish the same result.

From here you can launch graphical Linux applications. As a simple example,
here's how to run a simple Vulkan test application:

```sh
$ sudo apt install vulkan-tools
$ vkcube
```

## Running on QEMU

Running a guest on QEMU on x64 requires KVM. You may also need to enable nested
KVM on your host machine. To check whether nested virtualization is enabled, run
the following command:

### Configure your Host

Note: The following instructions assume a Linux host machine with an Intel
processor. This will not work if your host machine has an AMD processor.

```sh
cat /sys/module/kvm_intel/parameters/nested
```

An output of `Y` indicates nested virtualization is enabled, `0` or `N`
indicates not enabled.

To enable nested virtualization until the next reboot:

```sh
modprobe -r kvm_intel
modprobe kvm_intel nested=1
```

To make the change permanent add the following line to
`/etc/modprobe.d/kvm.conf`:
```
options kvm_intel nested=1
```

### Start QEMU

One you have your host machine setup, you can start the Fuchsia Emulator:

```sh
$ ffx emu start
```

## Integration tests

Machina has a set of integration tests that launch Zircon and Debian guests to
test the VMM, hypervisor, and each of the virtio devices. To run the tests, first
add them to your build:

```sh
$ fx set {{ '<var>' }}PRODUCT{{ '</var>' }}.{{ '<var>' }}BOARD{{ '</var>' }} --with //src/virtualization:tests
$ fx build
```

Then run any of the following tests:

```sh
# Tests of the low level hypervisor syscalls:
$ fx test hypervisor_tests

# Basic tests that verify OS boot and basic functionality:
$ fx test virtualization-core-tests

# Test suites focused on specific devices:
$ fx test virtualization-block-tests
$ fx test virtualization-net-tests
$ fx test virtualization-sound-tests
$ fx test virtualization-vsock-tests
$ fx test virtualization-gpu-tests
$ fx test virtualization-input-tests
```


[ref.debian_guest_readme]:
    https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/virtualization/packages/debian_guest/README.md
[ref.termina]:
    https://chromium.googlesource.com/chromiumos/overlays/board-overlays/+/master/project-termina/
