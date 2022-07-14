# Get started with driver development

This guide provides step-by-step instructions that walk you through the basic
workflows of building, running, debugging, and updating
[drivers][driver-concepts] in a Fuchsia system using the
[Fuchsia SDK][using-the-sdk].

Important: This guide is the driver equivalent of the
[_Get started with the Fuchsia SDK_][get-started-sdk] guide. If you haven't
already, it's strongly recommended that you first complete the _Get started
with the Fuchsia SDK_ guide to become familiar with the comprehensive set of
Fuchsia SDK-based workflows.

Complete the following sections:

1. [Prerequisites](#prerequisites).
2. [Clone the SDK driver samples repository](#clone-the-sdk-driver-samples-repository).
3. [Start the emulator](#start-the-emulator).
4. [Build and load the sample driver](#build-and-load-the-sample-driver).
5. [Build and run a tools component](#build-and-run-a-tools-component).
6. [Debug the sample driver](#debug-the-sample-driver).
7. [Modify and reload the sample driver](#modify-and-reload-the-sample-driver).

Found an issue? Please [let us know][sdk-bug]{:.external}.

## 1. Prerequisites {:#prerequisites}

Before you begin, complete the prerequisite steps below:

*   [Check host machine requirements](#check-host-machine-requirements)
*   [Install dependencies](#install-dependencies)
*   [Generate Fuchsia-specific SSH keys](#generate-fuchsia-specific-ssh-keys)

### Check host machine requirements {:#check-host-machine-requirements}

This guide requires that your host machine meets the following criteria:

*  A Linux machine. **macOS** is not supported yet.
*  Has at least 15 GB of storage space.
*  Supports [KVM][kvm]{:.external} (Kernel Virtual Machine) for running a
   [QEMU][qemu]{:.external}-based emulator.
*  IPv6 is enabled.

### Install dependencies {:#install-dependencies}

`git` and `bazel` need to be installed on the host machine.
You need Bazel 5.1 or higher.

Note: You only need to complete these steps once on your host machine.

Do the following:

1. [Install Git][git-install]{:.external}.

1. [Install Bazel][bazel-install]{:.external} – the easiest install option is
   to download the [Bazelisk binary][bazelisk-download]{:.external} and rename
   it to `bazel` in a convenient place on your path.

### Generate Fuchsia-specific SSH keys {:#generate-fuchsia-specific-ssh-keys}

The `ffx` tool requires that [Fuchsia-specific SSH keys][fuchsia-ssh-keys] are
stored on the host machine for connecting to Fuchsia devices (including the
Fuchsia emulator).

To check if your host machine already has Fuchsia SSH keys, do the following:

1. Scan the `$HOME/.ssh` directory for Fuchsia SSH keys:

   ```posix-terminal
   ls $HOME/.ssh | grep fuchsia
   ```

1. Verify that the following `fuchsia_*` files are present:

   ```none {:.devsite-disable-click-to-copy}
   $ ls $HOME/.ssh | grep fuchsia
   fuchsia_authorized_keys
   fuchsia_ed25519
   fuchsia_ed25519.pub
   ```

**If you don’t see these files**, you need to generate Fuchsia SSH keys on the
host machine:

1. Generate a new private and public SSH key pair:

   Note: These Fuchsia SSH keys are only used for connecting to Fuchsia
   devices during development. Generating these SSH keys won't alter your
   current SSH settings.

   ```posix-terminal
   [[ -f "${HOME}/.ssh/fuchsia_ed25519" ]] || ssh-keygen -P "" -t ed25519 -f "${HOME}/.ssh/fuchsia_ed25519" -C "${USER}@$(hostname -f) Shared SSH Key for Fuchsia"
   ```

1. Generate a `fuchsia_authorized_keys` file:

   ```posix-terminal
   [[ -f "${HOME}/.ssh/fuchsia_authorized_keys" ]] || ssh-keygen -y -f "${HOME}/.ssh/fuchsia_ed25519" > "${HOME}/.ssh/fuchsia_authorized_keys"
   ```

1. Verify that Fuchsia SSH keys are generated:

   ```posix-terminal
   ls $HOME/.ssh | grep fuchsia
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ ls $HOME/.ssh | grep fuchsia
   fuchsia_authorized_keys
   fuchsia_ed25519
   fuchsia_ed25519.pub
   ```

## 2. Clone the SDK driver samples repository {:#clone-the-sdk-driver-samples-repository}

Clone the [SDK driver samples repository][sdk-driver-sample-repo]{:.external}
on your host machine. This repository contains sample driver components and the
Bazel-based Fuchsia SDK.

The tasks include:

*   Bootstrap the SDK driver samples repository.
*   Verify that you can build the sample driver components and run `ffx`
    commands.

Do the following:

1. In a terminal, change to your home directory:

   Note: This guide uses the home directory (`$HOME`) as a base directory. This
   is where a new work directory (`drivers`) will be created for this guide. You
   may also select a different base directory (for instance,
   `cd $HOME/my-fuchsia-project`).

   ```posix-terminal
   cd $HOME
   ```

2. Clone the SDK driver samples repository:

   ```posix-terminal
   git clone https://fuchsia.googlesource.com/sdk-samples/drivers --recurse-submodules
   ```

   This creates a new directory named `drivers`, which clones the content of the
   SDK driver samples repository.

3. Go to the new directory:

   ```posix-terminal
   cd drivers
   ```

4. To verify the Fuchsia SDK environment setup, build the sample drivers:

   ```posix-terminal
   bazel build --config=fuchsia_x64 //src/qemu_edu
   ```

   The first build may take a few minutes to download dependencies, such as
   Bazel build rules, [Clang][clang], and [Fuchsia IDK][fuchsia-idk] (which
   includes the `ffx` tool).

   When finished successfully, it prints output similar to the following in the
   end:

   ```none {:.devsite-disable-click-to-copy}
   $ bazel build --config=fuchsia_x64 //src/qemu_edu
   ...
   INFO: Elapsed time: 131.746s, Critical Path: 26.89s
   INFO: 722 processes: 454 internal, 268 linux-sandbox.
   INFO: Build completed successfully, 722 total actions
   ```

5. To verify that you can use the `ffx` tool in your environment, run the
   following command:

   ```posix-terminal
   tools/ffx version -v
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx version -v
   ffx:
     abi-revision: 0xA56735A6690E09D8
     api-level: 8
     build-version: 2022-06-09T20:02:48+00:00
     integration-commit-hash: dfddeea2221689c800ca1db7a7c7d1f2cb0bd99f
     integration-commit-time: Thu, 09 Jun 2022 20:02:48 +0000

   daemon:
     abi-revision: 0xA56735A6690E09D8
     api-level: 8
     build-version: 2022-06-09T20:02:48+00:00
     integration-commit-hash: dfddeea2221689c800ca1db7a7c7d1f2cb0bd99f
     integration-commit-time: Thu, 09 Jun 2022 20:02:48 +0000
   ```

   At this point, you only need to confirm that you can run this `ffx` command
   without any errors.

   Note: To ensure that you’re using the right version of `ffx` (which needs to
   match the version of the SDK), consider updating your `PATH` to include the
   SDK's `tools` directory where `ffx` is located (for instance,
   `export PATH="$PATH:$HOME/drivers/tools"`). However, if you don't
   wish to update your `PATH`, ensure that you specify the relative path to
   this `ffx` tool (`tools/ffx`) whenever you run `ffx` commands.

## 3. Start the emulator {:#start-the-emulator}

Start the [Fuchsia emulator][femu] on the host machine while configuring the
emulator instance to use Fuchsia’s new [driver framework][driver-framework]
(DFv2).

The tasks include:

*   Download Fuchsia's Workstation prebuilt image from Google Cloud Storage.
*   Start the Fuchsia emulator.
*   Set the emulator instance as your host machine’s default target device.
*   Start the Fuchsia package server.
*   Register the system package repository to the emulator instance.

Do the following:

1. Download the latest Workstation image for the emulator:

   ```posix-terminal
   tools/ffx product-bundle get workstation.qemu-x64
   ```

   This command may take a few minutes to download the image and product
   metadata.

    Once the download is finished, the `ffx product-bundle get` command creates
    a local Fuchsia package repository named `workstation.qemu-x64` on your host
    machine. This package repository hosts additional system packages for this
    Workstation prebuilt image. Later in Step 7 you’ll register this package
    repository to the emulator instance.

2. Stop all emulator instances:

   ```posix-terminal
   tools/ffx emu stop --all
   ```

3. Start the Fuchsia emulator:

   ```posix-terminal
   tools/ffx emu start workstation.qemu-x64 --headless --kernel-args "driver_manager.use_driver_framework_v2=true" --kernel-args "driver_manager.root-driver=fuchsia-boot:///#meta/platform-bus.cm" --kernel-args "devmgr.enable-ephemeral=true"
   ```

   This command starts a headless emulator instance running the Workstation
   prebuilt image.

   When the instance is up and running, the command prints output similar to
   the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx emu start workstation.qemu-x64 --headless --kernel-args "driver_manager.use_driver_framework_v2=true" --kernel-args "driver_manager.root-driver=fuchsia-boot:///#meta/platform-bus.cm" -- kernel-args "devmgr.enable-ephemeral=true"
   Logging to "/home/alice/.local/share/Fuchsia/ffx/emu/instances/fuchsia-emulator/emulator.log"
   Waiting for Fuchsia to start (up to 60 seconds).
   Emulator is ready.
   ```

4. Verify that the new emulator instance is running:

   ```posix-terminal
   tools/ffx emu list
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx emu list
   [Active]  fuchsia-emulator
   ```

5. Set the default target device:

   ```posix-terminal
   tools/ffx target default set fuchsia-emulator
   ```

   This command exits silently without output.

6. Start the Fuchsia package server:

   ```posix-terminal
   tools/ffx repository server start
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx repository server start
   ffx repository server is listening on [::]:8083
   ```

7. Register the system package repository (`workstation.qemu-x64`) to the
   target device:

   ```posix-terminal
   tools/ffx target repository register -r workstation.qemu-x64 --alias fuchsia.com
   ```

   This command exits silently without output.

## 4. Build and load the sample driver {:#build-and-load-the-sample-driver}

The Fuchsia emulator (launched in the [Start the emulator](#start-the-emulator)
section above) is configured to create a virtual device named
[`edu`][edu-device], which  is an educational device for writing drivers.
In the previous section, when the emulator started, Fuchsia’s driver framework
detected this `edu` device in the system, but it wasn’t able to find a driver
that could serve the `edu` device. So the `edu` device was left unmatched.

In this section, we build and publish the [`qemu_edu`][qemu-edu] sample driver
(which is a Fuchsia component). Upon detecting a new driver, the driver
framework will discover that this new `qemu_edu` driver is a match for
the `edu` device. Once matched, the `qemu_edu` driver starts providing the `edu`
device’s services (capabilities) to other components in the system – one of the
services provided by the `edu` device is that it computes a factorial given
an integer.

The tasks include:

*   View the drivers that are currently loaded in the emulator instance.
*   Build and publish the `qemu_edu` driver component.
*   Verify that the `qemu_edu` driver is loaded to the emulator instance.
*   View detailed information on the `qemu_edu` component.

Do the following:

1. View the list of the currently loaded drivers:

   ```posix-terminal
   tools/ffx driver list --loaded
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx driver list --loaded
   fuchsia-boot:///#meta/block.core.cm
   fuchsia-boot:///#meta/bus-pci.cm
   fuchsia-boot:///#meta/fvm.cm
   fuchsia-boot:///#meta/hid-input-report.cm
   fuchsia-boot:///#meta/hid.cm
   fuchsia-boot:///#meta/intel-rtc.cm
   fuchsia-boot:///#meta/netdevice-migration.cm
   fuchsia-boot:///#meta/network-device.cm
   fuchsia-boot:///#meta/pc-ps2.cm
   fuchsia-boot:///#meta/platform-bus-x86.cm
   fuchsia-boot:///#meta/platform-bus.cm
   fuchsia-boot:///#meta/ramdisk.cm
   fuchsia-boot:///#meta/sysmem.cm
   fuchsia-boot:///#meta/virtio_block.cm
   fuchsia-boot:///#meta/virtio_ethernet.cm
   fuchsia-boot:///#meta/zxcrypt.cm
   fuchsia-pkg://fuchsia.com/virtual_audio#meta/virtual_audio_driver.cm
   ```

2. Build and publish the `qemu_edu` driver component:

   ```posix-terminal
   bazel run --config=fuchsia_x64 //src/qemu_edu:pkg.component
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ bazel run --config=fuchsia_x64 //src/qemu_edu:pkg.component
   INFO: Analyzed target //src/qemu_edu:pkg.component (6 packages loaded, 162 targets configured).
   INFO: Found 1 target...
   Target //src/qemu_edu:pkg.component up-to-date:
     bazel-bin/src/qemu_edu/pkg.component_run_component.sh
   INFO: Elapsed time: 1.660s, Critical Path: 0.49s
   INFO: 21 processes: 12 internal, 8 linux-sandbox, 1 local.
   INFO: Build completed successfully, 21 total actions
   INFO: Build completed successfully, 21 total actions
   added repository bazel.pkg.component
   Registering fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm
   Successfully bound:
   Node 'root.sys.platform.platform-passthrough.PCI0.bus.00_06_0_', Driver 'fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm'.
   ```

3. Verify that the `qemu_edu` driver is now loaded to the Fuchsia emulator
   instance:

   ```posix-terminal
   tools/ffx driver list --loaded
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx driver list --loaded
   fuchsia-boot:///#meta/block.core.cm
   fuchsia-boot:///#meta/bus-pci.cm
   fuchsia-boot:///#meta/fvm.cm
   fuchsia-boot:///#meta/hid-input-report.cm
   fuchsia-boot:///#meta/hid.cm
   fuchsia-boot:///#meta/intel-rtc.cm
   fuchsia-boot:///#meta/netdevice-migration.cm
   fuchsia-boot:///#meta/network-device.cm
   fuchsia-boot:///#meta/pc-ps2.cm
   fuchsia-boot:///#meta/platform-bus-x86.cm
   fuchsia-boot:///#meta/platform-bus.cm
   fuchsia-boot:///#meta/ramdisk.cm
   fuchsia-boot:///#meta/sysmem.cm
   fuchsia-boot:///#meta/virtio_block.cm
   fuchsia-boot:///#meta/virtio_ethernet.cm
   fuchsia-boot:///#meta/zxcrypt.cm
   fuchsia-pkg://fuchsia.com/virtual_audio#meta/virtual_audio_driver.cm
   {{ '<strong>' }}fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm{{ '</strong>' }}
   ```

   Notice that the `qemu_edu` driver is shown at the bottom of the loaded
   drivers list.

4. View the `qemu_edu` component information:

   ```posix-terminal
   tools/ffx component show qemu_edu.cm
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx component show qemu_edu.cm
                  Moniker: /bootstrap/universe-pkg-drivers:root.sys.platform.platform-passthrough.PCI0.bus.00_06_0_
                      URL: fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm
                     Type: CML dynamic component
          Component State: Resolved
    Incoming Capabilities: fuchsia.device.fs.Exporter
                           fuchsia.driver.compat.Service
                           fuchsia.logger.LogSink
                           pkg
              Merkle root: a4832605ffe6bf6ddad3aad0d3d36c435ee2e66f79d43cd0b818d2aae20f7755
          Execution State: Running
             Start reason: Instance is in a single_run collection
    Outgoing Capabilities: qemu-edu
   ```

5. View device logs:

   ```posix-terminal
   tools/ffx log --filter qemu_edu
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --filter qemu_edu
   ...
   [176.540][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://bazel.pkg.component/qemu_edu to a4832605ffe6bf6ddad3aad0d3d36c435ee2e66f79d43cd0b818d2aae20f7755 with TUF
   [176.542][bootstrap/driver_index][driver_index,driver][I] Registered driver successfully: fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm.
   [176.571][core/pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://bazel.pkg.component/qemu_edu: []
   [176.573][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://bazel.pkg.component/qemu_edu to a4832605ffe6bf6ddad3aad0d3d36c435ee2e66f79d43cd0b818d2aae20f7755 with TUF
   [176.577][bootstrap/driver_manager][driver_manager.cm][I]: [driver_runner.cc:858] Binding fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm to  00_06_0_
   [176.908][bootstrap/driver-hosts:driver-host-3][driver_host2.cm][I]: [driver_host.cc:289] Started 'fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm'
   ```

   Press `CTRL+C` to exit.

## 5. Build and run a tools component {:#build-and-run-a-tools-component}

The `qemu_edu` driver sample has a “tools” component named `eductl`, which can
interact with the sample driver. Developers create these tools components for
testing and debugging drivers during development.

In this case, the `eductl` component contacts the `qemu_edu` driver and passes
an integer as input. The driver (using the resource of the `edu` virtual device)
computes the integer's factorial and returns the result to the `eductl`
component. The component then prints the result in the log.

The tasks include:

*   Build and run the `eductl` component.
*   Verify that the component can interact with the `qemu_edu` driver.

Do the following:

1. Build and run the `eductl` component:

   ```posix-terminal
   bazel run --config=fuchsia_x64 //src/qemu_edu:eductl_pkg.eductl_component
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ bazel run --config=fuchsia_x64 //src/qemu_edu:eductl_pkg.eductl_component
   INFO: Analyzed target //src/qemu_edu:eductl_pkg.eductl_component (0 packages loaded, 14 targets configured).
   INFO: Found 1 target...
   Target //src/qemu_edu:eductl_pkg.eductl_component up-to-date:
     bazel-bin/src/qemu_edu/eductl_pkg.eductl_component_run_component.sh
   INFO: Elapsed time: 1.667s, Critical Path: 1.22s
   INFO: 23 processes: 7 internal, 15 linux-sandbox, 1 local.
   INFO: Build completed successfully, 23 total actions
   INFO: Build completed successfully, 23 total actions
   added repository bazel.eductl.pkg.eductl.component
   URL: fuchsia-pkg://bazel.eductl.pkg.eductl.component/eductl#meta/eductl.cm
   Moniker: /core/ffx-laboratory:eductl
   Creating component instance...
   Starting component instance...
   Success! The component instance has been started.
   ```

2. View the device logs of the `eductl` component:

   ```posix-terminal
   tools/ffx log --filter eductl dump
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --filter eductl dump
   ...
   [367.076][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://bazel.eductl.pkg.eductl.component/eductl to 4fe2e38ed56693bf720565c3ee5e6f8314a64c601cae67288db5e8d30f1a9265 with TUF
   [367.080][core/pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://bazel.eductl.pkg.eductl.component/eductl: []
   [367.081][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://bazel.eductl.pkg.eductl.component/eductl to 4fe2e38ed56693bf720565c3ee5e6f8314a64c601cae67288db5e8d30f1a9265 with TUF
   {{ '<strong>' }}[367.166][core/ffx-laboratory:eductl][][I] Factorial(12) = 479001600{{ '</strong>' }}
   [367.173][core/pkg-resolver][pkg-resolver][I] removing repository fuchsia-pkg://bazel.eductl.pkg.eductl.component
   [367.173][core/pkg-resolver][pkg-resolver][I] closing fuchsia-pkg://bazel.eductl.pkg.eductl.component
   [367.176][core/pkg-resolver][pkg-resolver][I] AutoClient for "http://10.0.2.2:8083/bazel.eductl.pkg.eductl.component/auto" stopping
   ```

   The output `Factorial(12) = 479001600` shows that the `eductl` component
   passed 12 as input to the driver and received the result from the driver.
   (For the default input, see this [`eductl.cml`][eductl-cml] file.)

## 6. Debug the sample driver {:#debug-the-sample-driver}

Use the Fuchsia debugger ([`zxdb`][zxdb-user-guide]) to step through the
sample driver’s code as the driver is running on the emulator instance.

The tasks include:

*   Identify the driver host (which is a component) that is running the
    `qemu_edu` driver.
*   Start the Fuchsia debugger and connect it to the emulator instance.
*   Attach the debugger to the driver host.
*   Set a breakpoint on the driver’s code.
*   Run the tools component, which triggers the driver to execute its
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
   Driver Host: 4690
       fuchsia-boot:///#meta/block.core.cm
       fuchsia-boot:///#meta/bus-pci.cm
       fuchsia-boot:///#meta/fvm.cm
       fuchsia-boot:///#meta/hid.cm
       fuchsia-boot:///#meta/netdevice-migration.cm
       fuchsia-boot:///#meta/network-device.cm
       fuchsia-boot:///#meta/platform-bus-x86.cm
       fuchsia-boot:///#meta/platform-bus.cm
       fuchsia-boot:///#meta/ramdisk.cm
       fuchsia-boot:///#meta/sysmem.cm
       fuchsia-boot:///#meta/virtio_block.cm
       fuchsia-boot:///#meta/virtio_ethernet.cm
       fuchsia-boot:///#meta/zxcrypt.cm
       fuchsia-pkg://fuchsia.com/virtual_audio#meta/virtual_audio_driver.cm

   Driver Host: 7820
       fuchsia-boot:///#meta/intel-rtc.cm

   Driver Host: 7903
       fuchsia-boot:///#meta/pc-ps2.cm

   Driver Host: 50125
       fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm
   ```

   Make a note of the PID of the `qemu_edu` driver host (`50125` in the
   example above).

2. Start the Fuchsia debugger:

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

3. Attach the debugger to the `qemu_edu` driver host:

   <pre class="devsite-click-to-copy">
   <span class="no-select">[zxdb] </span>attach <var>PID</var>
   </pre>

   Replace `PID` with the PID of the `qemu_edu` driver host identified
   in Step 1, for example:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] attach 50125
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] attach 50125
   Attached Process 1 state=Running koid=50125 name=driver_host2.cm
   Downloading symbols...
   Symbol downloading complete. 7 succeeded, 0 failed.
   [zxdb]
   ```

4. Set a breakpoint at the driver’s `ComputeFactorial` function:

   <pre class="devsite-click-to-copy">
   <span class="no-select">[zxdb] </span>break QemuEduDriver::ComputeFactorial
   </pre>

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] break QemuEduDriver::ComputeFactorial
   Created Breakpoint 1 @ QemuEduDriver::ComputeFactorial
      177 void QemuEduDriver::ComputeFactorial(ComputeFactorialRequestView request,
    ◉ 178                                      ComputeFactorialCompleter::Sync& completer) {
      179   // Write a value into the factorial register.
   [zxdb]
   ```

5. In different terminal, run the tools component:

   Note:  In this new terminal, make sure that you change to the same work
   directory (for instance, `cd $HOME/drivers`).

   ```posix-terminal
   bazel run --config=fuchsia_x64 //src/qemu_edu:eductl_pkg.eductl_component
   ```

   In the `zxdb` terminal, verify that the debugger is stopped at the driver’s
   `ComputeFactorial` function, for example:

   ```none {:.devsite-disable-click-to-copy}
   🛑 thread 2 on bp 1 qemu_edu::QemuEduDriver::ComputeFactorial(qemu_edu::QemuEduDriver*, fidl::WireServer<fuchsia_hardware_qemuedu::Device>::ComputeFactorialRequestView,    fidl::Completer<fidl::internal::WireCompleterBase<fuchsia_hardware_qemuedu::Device::ComputeFactorial> >::Sync&) • qemu_edu.cc:178
      176
      177 void QemuEduDriver::ComputeFactorial(ComputeFactorialRequestView request,
    ▶ 178                                      ComputeFactorialCompleter::Sync& completer) {
      179   // Write a value into the factorial register.
      180   uint32_t input = request->input;
   [zxdb]
   ```

6. In the `zxdb` terminal, view the source code around the current breakpoint:

   <pre class="devsite-click-to-copy">
   <span class="no-select">[zxdb] </span>list
   </pre>

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] list
      173       });
      174   return outgoing_.Serve(std::move(outgoing_dir));
      175 }
      176
      177 void QemuEduDriver::ComputeFactorial(ComputeFactorialRequestView request,
    ▶ 178                                      ComputeFactorialCompleter::Sync& completer) {
      179   // Write a value into the factorial register.
      180   uint32_t input = request->input;
      181
      182   mmio_->Write32(input, regs::kFactorialCompoutationOffset);
      183
      184   // Busy wait on the factorial status bit.
      185   while (true) {
      186     const auto status = regs::Status::Get().ReadFrom(&*mmio_);
      187     if (!status.busy())
      188       break;
   [zxdb]
   ```

7. In the `zxdb` terminal, step through the code using the `next`
   command until the value of `factorial` is read from the device (that is,
   until the line 194 is reached):

   <pre class="devsite-click-to-copy">
   <span class="no-select">[zxdb] </span>next
   </pre>

   The last `next` command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   ...
   [zxdb] next
   🛑 thread 2 qemu_edu::QemuEduDriver::ComputeFactorial(qemu_edu::QemuEduDriver*, fidl::WireServer<fuchsia_hardware_qemuedu::Device>::ComputeFactorialRequestView, fidl::Completer<fidl::internal::WireCompleterBase<fuchsia_hardware_qemuedu::Device::ComputeFactorial> >::Sync&) • qemu_edu.cc:194
      192   uint32_t factorial = mmio_->Read32(regs::kFactorialCompoutationOffset);
      193
    ▶ 194   FDF_SLOG(INFO, "Replying with", KV("factorial", factorial));
      195   completer.Reply(factorial);
      196 }
   [zxdb]
   ```

8. Print the `factorial` variable:

   <pre class="devsite-click-to-copy">
   <span class="no-select">[zxdb] </span>print factorial
   </pre>

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] print factorial
   479001600
   [zxdb]
   ```

1. To exit the `zxdb` terminal, type `exit` or press `Ctrl-D`.

   Note: For more information on usages and best practices on `zxdb`, see the
   [zxdb user guide][zxdb-user-guide].

## 7. Modify and reload the sample driver {:#modify-and-reload-the-sample-driver}

Update the source code of the sample driver and reload it to the emulator
instance.

The tasks include:

*   Restart the emulator instance to unload the `qemu_edu` driver.
*   Update the source code of the `qemu_edu` driver.
*   Load the updated driver.
*   Run the tools component to verify the change.

Do the following:

1. Stop the emulator instance:

   ```posix-terminal
   tools/ffx emu stop
   ```

   This command stops the currently running emulator instance.

1. Start a new instance of the Fuchsia emulator:

   ```posix-terminal
   tools/ffx emu start workstation.qemu-x64 --headless --kernel-args "driver_manager.use_driver_framework_v2=true" --kernel-args "driver_manager.root-driver=fuchsia-boot:///#meta/platform-bus.cm" --kernel-args "devmgr.enable-ephemeral=true"
   ```

   This command starts a headless emulator instance running the Workstation
   prebuilt image.

1. Use a text editor to open the source code of the sample driver, for example:

   ```posix-terminal
   nano src/qemu_edu/qemu_edu.cc
   ```

1. In the `QemuEduDriver::ComputeFactorial` function,
   between the line
   `uint32_t factorial = mmio_->Read32(regs::kFactorialCompoutationOffset);`
   (Line 192) and the `FDF_SLOG()` call (Line 194), add the following line:

   ```
   factorial=12345;
   ```

   The function should look like below:

   ```none {:.devsite-disable-click-to-copy}
   void QemuEduDriver::ComputeFactorial(ComputeFactorialRequestView request,
                                        ComputeFactorialCompleter::Sync& completer) {
     // Write a value into the factorial register.
     uint32_t input = request->input;

     mmio_->Write32(input, regs::kFactorialCompoutationOffset);

     // Busy wait on the factorial status bit.
     while (true) {
       const auto status = regs::Status::Get().ReadFrom(&*mmio_);
       if (!status.busy())
         break;
     }

     // Return the result.
     uint32_t factorial = mmio_->Read32(regs::kFactorialCompoutationOffset);
     {{ '<strong>' }}factorial = 12345;{{ '</strong>' }}
     FDF_SLOG(INFO, "Replying with", KV("factorial", factorial));
     completer.Reply(factorial);
   }
   ```

   The function is now updated to return the value of `12345` only.

1. Save the file and close the text editor.

1. Rebuild and run the modified sample driver:

   ```posix-terminal
   bazel run --config=fuchsia_x64 //src/qemu_edu:pkg.component
   ```

1. Run the tools component:

   ```posix-terminal
   bazel run --config=fuchsia_x64 //src/qemu_edu:eductl_pkg.eductl_component
   ```

1. To verify that change, view the device logs of the tools component:

   ```posix-terminal
   tools/ffx log --filter eductl dump
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --filter eductl dump
   ...
   [43.349][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://bazel.eductl.pkg.eductl.component/eductl to 4fe2e38ed56693bf720565c3ee5e6f8314a64c601cae67288db5e8d30f1a9265 with TUF
   [43.354][core/pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://bazel.eductl.pkg.eductl.component/eductl: []
   [43.355][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://bazel.eductl.pkg.eductl.component/eductl to 4fe2e38ed56693bf720565c3ee5e6f8314a64c601cae67288db5e8d30f1a9265 with TUF
   {{ '<strong>' }}[43.439][core/ffx-laboratory:eductl][][I] Factorial(12) = 12345{{ '</strong>' }}
   [43.448][core/pkg-resolver][pkg-resolver][I] removing repository fuchsia-pkg://bazel.eductl.pkg.eductl.component
   [43.449][core/pkg-resolver][pkg-resolver][I] closing fuchsia-pkg://bazel.eductl.pkg.eductl.component
   [43.452][core/pkg-resolver][pkg-resolver][I] AutoClient for "http://10.0.2.2:8083/bazel.eductl.pkg.eductl.component/auto" stopping.
   ```

   The line in the logs shows that the `qemu_edu` driver returned the
   hardcoded value of `12345` as the factorial of 12 to the tools component.

**Congratulations! You’re now all set with the Fuchsia driver development!**

## Next steps {:#next-steps}

Learn more about how the `qemu_edu` driver works in the
[Driver sample walkthrough: qemu_edu][driver-sample-walkthrough] guide.

## Appendices

### Clean up the environment {:#clean-up-the-environment}

If you run into a problem while following this guide and decide to start over
from the beginning, consider running the commands below to clean up
your development environment (that is, to clean up directories, build artifacts,
downloaded files, symlinks, configuration settings, and more).

Remove the package repositories created in this guide:

```posix-terminal
tools/ffx repository remove workstation.qemu-x64
```

```posix-terminal
tools/ffx repository server stop
```

```posix-terminal
rm -rf $HOME/.package_repos/sdk-samples
```

Remove all existing configurations and data of `ffx`:

```posix-terminal
tools/ffx daemon stop
```

```posix-terminal
rm -rf $HOME/.local/share/Fuchsia/ffx
```

Remove the `drivers` directory and its artifacts:

Caution: If the SDK samples repository is cloned to a different location
than `$HOME/drivers`, adjust the directory path in the command below.
Be extremely careful with the directory path when you run the `rm -rf
<DIR>` command.

```posix-terminal
rm -rf $HOME/drivers
```

When Bazel fails to build, try the commands below:

Caution: Running `bazel clean` or deleting the `$HOME/.cache/bazel` directory
deletes all the artifacts downloaded by Bazel, which can be around 4 GB.
This means Bazel will need to download those dependencies again
the next time you run `bazel build`.

```posix-terminal
bazel clean --expunge
```

```posix-terminal
bazel shutdown && rm -rf $HOME/.cache/bazel
```

Other clean up commands:

```posix-terminal
killall ffx
```

```posix-terminal
killall pm
```

<!-- Reference links -->

[using-the-sdk]: /development/sdk/index.md
[get-started-sdk]: /get-started/sdk/index.md
[sdk-bug]: https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=Bazel
[kvm]: https://www.linux-kvm.org/page/Main_Page
[qemu]: https://www.qemu.org/
[bazel]: https://bazel.build/docs
[git]: https://git-scm.com/
[git-install]: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git
[bazel-install]: https://bazel.build/install
[bazelisk-download]: https://github.com/bazelbuild/bazelisk/releases
[fuchsia-ssh-keys]: /development/sdk/ffx/create-ssh-keys-for-devices.md
[ticket-01]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=97909
[sdk-driver-sample-repo]: https://fuchsia.googlesource.com/sdk-samples/drivers
[clang]: https://clang.llvm.org/
[fuchsia-idk]: /development/idk/README.md
[edu-device]: https://fuchsia.googlesource.com/third_party/qemu/+/refs/heads/main/docs/specs/edu.txt
[qemu-edu]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/
[eductl-cml]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/meta/eductl.cml
[zxdb-user-guide]: /development/debugger/README.md
[driver-concepts]: /concepts/drivers/README.md
[driver-sample-walkthrough]: /development/sdk/driver-sample-qemu-edu.md
[driver-framework]: /concepts/drivers/driver_framework.md
[femu]: /development/sdk/ffx/start-the-fuchsia-emulator.md
