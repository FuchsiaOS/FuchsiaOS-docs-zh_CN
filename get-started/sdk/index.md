# Get started with the Fuchsia SDK

This guide provides step-by-step instructions on setting up the
Fuchsia SDK development environment on your host machine. The guide then walks
you through the basic workflows of building, running, debugging, and testing
Fuchsia components using the [Fuchsia SDK][using-the-sdk].

Important: The Fuchsia SDK is in active development. At the moment, Fuchsia
does not support general public usage of the Fuchsia SDK. The APIs in the
SDK are subject to change without notice.

Complete the following sections:

1. [Prerequisites](#prerequisites)
1. [Clone the SDK samples repository](#clone-the-sdk-samples-repository)
1. [Start the emulator](#start-the-emulator)
1. [Build and run the sample component](#build-and-run-the-sample-component)
1. [View symbolized logs](#view-symbolized-logs)
1. [Debug the sample component](#debug-the-sample-component)
1. [Inspect components](#inspect-components)
1. [Run tests](#run-tests)

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
*  Supports [KVM][kvm]{:.external} (Kernel Virtual Machine) for running a [QEMU][qemu]{:.external}-based emulator.
*  IPv6 is enabled.

### Install dependencies {:#install-dependencies}

`git` and `bazel` need to be installed on the host machine.
You need Bazel 5.1 or higher.

Note: You only need to complete these steps once on your host machine.

Do the following:

1. [Install Git][git-install]{:.external}.

1. [Install Bazel][bazel-install]{:.external} – the easiest install option is
   to download the [Bazelisk binary][bazelisk-download]{:.external} and rename it to `bazel` in a
   convenient place on your path.

### Generate Fuchsia-specific SSH keys {:#generate-fuchsia-specific-ssh-keys}

The `ffx` tool requires that [Fuchsia-specific SSH keys][fuchsia-ssh-keys] are stored
on the host machine for connecting to Fuchsia devices (including the Fuchsia emulator).

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

**If you don’t see these files**, you need to generate Fuchsia SSH keys on the host machine:

1. Generate a new private and public SSH key pair:

   Note: These Fuchsia SSH keys are only used for connecting to Fuchsia
   devices during development. Generating these SSH keys won't alter your current
   SSH settings.

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

## 2. Clone the SDK samples repository {:#clone-the-sdk-samples-repository}

Clone the [SDK samples repository][sdk-samples-repo]{:.external} on your host machine.
This repository contains the Bazel-based Fuchsia SDK and sample components.

Note: Support for installing the SDK by itself, without a samples repository, is planned.
See [this ticket][ticket-01].

The tasks include:

*   Bootstrap the SDK samples repository.
*   Build the SDK tools to initialize the SDK environment.
*   Verify that you can run [`ffx`][ffx] commands.

Do the following:

1. In a terminal, change to your home directory:

   Note: This guide uses the home directory (`$HOME`) as a base directory. This is where a new
   work directory (`fuchsia-getting-started`) will be created for this guide. You may also select
   a different base directory (for instance, `cd $HOME/my-fuchsia-project`).

   ```posix-terminal
   cd $HOME
   ```

1. Clone the Fuchsia samples repository:

   ```posix-terminal
   git clone https://fuchsia.googlesource.com/sdk-samples/getting-started fuchsia-getting-started --recurse-submodules
   ```

   This creates a new directory named `fuchsia-getting-started`, which clones the content of
   the SDK samples repository.

1. Go to the new directory:

   ```posix-terminal
   cd fuchsia-getting-started
   ```

1. Build the SDK tools:

   ```posix-terminal
   bazel build @fuchsia_sdk//:fuchsia_toolchain_sdk
   ```

   The first build may take a few minutes to download dependencies, such as Bazel build rules,
   [Clang][clang]{:.external} and [Fuchsia IDK][fuchsia-idk] (which includes the `ffx` tool).

   When finished successfully, it prints output similar to the following in the end:

   ```none {:.devsite-disable-click-to-copy}
   $ bazel build @fuchsia_sdk//:fuchsia_toolchain_sdk
   Starting local Bazel server and connecting to it...
   INFO: Analyzed target @fuchsia_sdk//:fuchsia_toolchain_sdk (2 packages loaded, 2 targets configured).
   INFO: Found 1 target...
   Target @fuchsia_sdk//:fuchsia_toolchain_sdk up-to-date (nothing to build)
   INFO: Elapsed time: 26.344s, Critical Path: 0.02s
   INFO: 1 process: 1 internal.
   INFO: Build completed successfully, 1 total action
   ```

1. To verify that you can use the `ffx` tool in your environment,
   run the following command:

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
   `export PATH="$PATH:$HOME/fuchsia-getting-started/tools"`). However, if you don't
   wish to update your `PATH`, ensure that you specify the relative path to
   this `ffx` tool (`tools/ffx`) whenever you run `ffx` commands.

## 3. Start the emulator {:#start-the-emulator}

Start the [Fuchsia emulator][femu] on the host machine. This guide uses an instance
of the Fuchsia emulator as the target device for running and testing Fuchsia
software. And to be able to supply new Fuchsia software to the target device,
you also need to start the [Fuchsia package server][fuchsia-package-server]
on the host machine.

The tasks include:

*   Download one of Fuchsia's prebuilt images from Google Cloud Storage.
*   Start the Fuchsia emulator to run the downloaded Fuchsia prebuilt image.
*   Set the emulator instance as the default target device.
*   Verify that various `ffx` commands can connect to the emulator instance.
*   Start the Fuchsia package server.
*   Register the system package repository to the emulator instance.

Do the following:

1. Download the latest Fuchsia Workstation prebuilt image for the emulator
   (`workstation.qemu-x64`):

   ```posix-terminal
   tools/ffx product-bundle get workstation.qemu-x64
   ```

   This command may take a few minutes to download the image and product metadata.

   Once the download is finished, the `ffx product-bundle get` command creates
   a local Fuchsia package repository named `workstation.qemu-x64` on your host
   machine. This package repository hosts additional system packages for this
   Workstation prebuilt image. Later in Step 11 you’ll register this package
   repository to the emulator instance.

1. Stop all running emulator instances:

   ```posix-terminal
   tools/ffx emu stop --all
   ```

1. Start a new Fuchsia emulator instance:

   ```posix-terminal
   tools/ffx emu start workstation.qemu-x64 --headless
   ```

   This command starts a headless emulator instance running a Fuchsia prebuilt
   image.

   When the instance is up and running, the command prints output similar to the
   following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx emu start workstation.qemu-x64 --headless
   Logging to "/home/alice/.local/share/Fuchsia/ffx/emu/instances/fuchsia-emulator/emulator.log"
   Waiting for Fuchsia to start (up to 60 seconds)...........
   Emulator is ready.
   ```

1. Verify that the new emulator instance is running:

   ```posix-terminal
   tools/ffx emu list
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx emu list
   [Active]    fuchsia-emulator
   ```

1. Verify that the emulator instance is detected as a device:

   ```posix-terminal
   tools/ffx target list
   ```

   This command prints output similar to the following:


   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx target list
   NAME               SERIAL       TYPE                    STATE      ADDRS/IP                           RCS
   fuchsia-emulator   <unknown>    Unknown                 Product    [fe80::d4e3:9a5b:c2e:2534%qemu]    Y
   ```

1. Set this emulator instance to be the default device:

   ```posix-terminal
   tools/ffx target default set fuchsia-emulator
   ```

   This command exits silently without output.


1. Verify that the default device is set:

   ```
   tools/ffx target default get
   ```

   This command prints output similar to the following:


   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx target default get
   fuchsia-emulator
   ```

1. To verify that you can establish an SSH connection to the emulator instance,
   run the following command:

   Note: To retrieve detailed device information, the `ffx target show` command uses
   [Fuchsia-specific SSH keys](#generate-fuchsia-specific-ssh-keys) to make an SSH
   connection to the device.

   ```posix-terminal
   tools/ffx target show
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx target show
   Target:
       Name: "fuchsia-emulator"
       SSH Address: "127.0.0.1:39189"
   Board:
       Name: "default-board"
       Revision: "1"
       Instruction set: "x64"
   Device:
       ...
   Build:
       Version: "8.20220609.3.1"
       Product: "workstation"
       Board: "qemu-x64"
       Commit: "2022-06-09T20:02:48+00:00"
   Last Reboot:
       Graceful: "false"
       Reason: "Cold"
   ...
   ```

   The example output above shows that the target device is running a `workstation.qemu-x64`
   prebuilt image whose version is `8.20220609.3.1` (which indicates that this image was
   built and published on June 9, 2022).

1. Verify that you can stream the device logs:

   ```posix-terminal
   tools/ffx log
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log
   ...
   [33.698][core/cobalt][cobalt,fidl_service,core][I] LocalAggregation: Enabling local aggregation.
   [33.698][core/cobalt][cobalt,fidl_service,core][I] ClearcutV1ShippingManager: Disabling observation uploading.
   [34.818][core/network/netstack][netstack,DHCP][W] client.go(692): ethp0004: recv timeout waiting for dhcpOFFER; retransmitting dhcpDISCOVER
   [34.818][core/network/netstack][netstack,DHCP][I] client.go(891): ethp0004: send dhcpDISCOVER from :68 to 255.255.255.255:67 on NIC:2 (broadcast_flag=false ciaddr=false)
   [35.654][core/remote-control][remote_control,remote-control][I] attempting to connect hub_path="/discovery_root/children/bootstrap/resolved/expose/fuchsia.diagnostics.LogSettings"
   ...
   ```

   Press `CTRL+C` to exit.

1. Start the Fuchsia package server:

   ```posix-terminal
   tools/ffx repository server start
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx repository server start
   ffx repository server is listening on [::]:8083
   ```

1. Register the system package repository (`workstation.qemu-x64`) to the target device:

   ```posix-terminal
   tools/ffx target repository register -r workstation.qemu-x64 --alias fuchsia.com
   ```

   This command exits silently without output.

## 4. Build and run the sample component {:#build-and-run-the-sample-component}

Build and run the [C++ Hello World component][hello-world-component]{:.external}
included in the SDK samples repository. [Components][fuchsia-component] are the basic
unit of executable software on Fuchsia.

The tasks include:

*   Build and run the sample Hello World component.
*   Make a change to the component.
*   Repeat the build and run steps.
*   Verify the change.

Do the following:

1. Build and run the sample component:

   ```posix-terminal
   bazel run --config=fuchsia_x64 //src/hello_world:pkg.component
   ```
   When the build is successful, this command generates build artifacts in a temporary
   Fuchsia package repository, which is then removed after the component runs.

   The command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ bazel run --config=fuchsia_x64 //src/hello_world:pkg.component
   INFO: Build options --copt, --cpu, --crosstool_top, and 1 more have changed, discarding analysis cache.
   INFO: Analyzed target //src/hello_world:pkg.component (46 packages loaded, 1160 targets configured).
   INFO: Found 1 target...
   Target //src/hello_world:pkg.component up-to-date:
     bazel-bin/src/hello_world/pkg.component_run_component.sh
   INFO: Elapsed time: 60.410s, Critical Path: 1.62s
   INFO: 44 processes: 29 internal, 14 linux-sandbox, 1 local.
   INFO: Build completed successfully, 44 total actions
   INFO: Build completed successfully, 44 total actions
   added repository bazel.pkg.component
   URL: fuchsia-pkg://bazel.pkg.component/hello_world#meta/hello_world.cm
   Moniker: /core/ffx-laboratory:hello_world
   Creating component instance...
   Starting component instance...
   Success! The component instance has been started.
   ```

1. Check the status of the `hello_world` component:

   ```posix-terminal
   tools/ffx component show hello_world
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx component show hello_world
                  Moniker: /core/ffx-laboratory:hello_world
                      URL: fuchsia-pkg://fuchsiasamples.com/hello_world#meta/hello_world.cm
                     Type: CML dynamic component
          Component State: Resolved
    Incoming Capabilities: fuchsia.logger.LogSink
                           pkg
              Merkle root: b44de670cf30c77c55823af0fea67d19e0fabc86ddd0946646512be12eeb8dc0
          Execution State: Stopped
   ```

   The output shows that the `hello_world` component has run and is now terminated (`Stopped`).

1. Verify the `Hello, World!` message in the device logs:

   ```posix-terminal
   tools/ffx log --filter hello_world dump
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --filter hello_world dump
   ...
   [1702.331][core/pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://fuchsiasamples.com/hello_world: []
   [1702.331][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://fuchsiasamples.com/hello_world as fuchsia-pkg://fuchsiasamples.com/hello_world to dbdc177180730f521849484c7a0e11dbe763b75804a7d1b97158a668b463526c with TUF
   {{ '<strong>' }}[1702.405][core/ffx-laboratory:hello_world][][I] Hello, World!{{ '</strong>' }}
   ```

1. Use a text editor to edit the  `src/hello_world/hello_world.cc` file, for example:

   ```posix-terminal
   nano src/hello_world/hello_world.cc
   ```

1. Change the message to `"Hello again, World!"`.

   The `main()` method should look like below:

   ```none {:.devsite-disable-click-to-copy}
   int main() {
     {{ '<strong>' }}std::cout << "Hello again, World!\n";{{ '</strong>' }}
     return 0;
   }
   ```

1. Save the file and exit the text editor.

1. Build and run the sample component again:

   ```posix-terminal
   bazel run --config=fuchsia_x64 //src/hello_world:pkg.component
   ```

1. Verify the `Hello again, World!` message in the device logs:

   ```posix-terminal
   tools/ffx log --filter hello_world dump
   ```

   This command prints output similar to the following;

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --filter hello_world dump
   ...
   [2013.380][core/pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://fuchsiasamples.com/hello_world: []
   [2013.380][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://fuchsiasamples.com/hello_world as fuchsia-pkg://fuchsiasamples.com/hello_world to da1c95e829ec32f78e7b4e8eb845b697679d9cb82432da3cc85763dbc3269395 with TUF
   {{ '<strong>' }}[2013.418][core/ffx-laboratory:hello_world][][I] Hello again, World!{{ '</strong>' }}
   ```

## 5. View symbolized logs {:#view-symbolized-logs}

Examine the [symbolized logs][symbolize-logs] (that is, human readable
stack traces) of a crashed component.

The tasks include:

*   Update the sample component to crash when it's started.
*   Build and run the sample component, which generates and registers the debug symbols
    of the component.
*   Verify that the crashed component's logs are in symbolized format.

Do the following:

1. Use a text editor to edit the  `src/hello_world/hello_world.cc` file, for example:

   ```posix-terminal
   nano src/hello_world/hello_world.cc
   ```

1. Just above the line `return 0;`, add the following line:

   ```
   abort();
   ```

   The `main()` method should look like below:

   ```none {:.devsite-disable-click-to-copy}
   int main() {
     std::cout << "Hello again, World!\n";
     {{ '<strong>' }}abort();{{ '</strong>' }}
     return 0;
   }
   ```

   This update will cause the component to crash immediately after printing a message.

1. Save the file and exit the text editor.

1. Build and run the sample component:

   ```posix-terminal
   bazel run --config=fuchsia_x64 //src/hello_world:pkg.component
   ```

   Building a component automatically generates and registers the component’s debug
   symbols in your development environment.

1. For newly registered symbols to be used in your environment, restart the `ffx` daemon:

   Note: This is a temporary workaround. This issue is being tracked in
   [Issue 94614][ticket-94614]{:.external}.

   ```posix-terminal
   tools/ffx daemon stop
   ```

   A new instance of the `ffx `daemon starts the next time you run a `ffx` command.

1. Verify that the sample component's crash stack is symbolized in the kernel logs:

   ```posix-terminal
   tools/ffx log --kernel
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --kernel
   ...
   [174978.449][klog][klog][I] [[[ELF module #0x6 "libzircon.so" BuildID=5679a47f32c6fa7b 0x422808b26000]]]
   [174978.449][klog][klog][I] [[[ELF module #0x7 "libc.so" BuildID=1c3e8dded0fc94eb 0x428049099000]]]
   [174978.450][klog][klog][I]    #0    0x00004280490fd74b in abort() ../../zircon/third_party/ulib/musl/src/exit/abort.c:7 <libc.so>+0x6474b sp 0x11d191bcf70
   {{ '<strong>' }}[174978.450][klog][klog][I]    #1    0x000001d56b552047 in main() src/hello_world/hello_world.cc:9 <<VMO#32996646=blob-a4c56246>>+0x2047 sp 0x11d191bcf80{{ '</strong>' }}
   [174978.450][klog][klog][I]    #2    0x00004280490fcef2 in start_main(const start_params*) ../../zircon/third_party/ulib/musl/src/env/__libc_start_main.c:140 <libc.so>+0x63ef2 sp 0x11d191bcfa0
   [174978.450][klog][klog][I]    #3    0x00004280490fd145 in __libc_start_main(zx_handle_t, int (*)(int, char**, char**)) ../../zircon/third_party/ulib/musl/src/env/__libc_start_main.c:215 <libc.so>+0x64145 sp 0x11d191bcff0
   [174978.450][klog][klog][I]    #4    0x000001d56b552011 in _start(zx_handle_t) ../../zircon/system/ulib/c/Scrt1.cc:7 <<VMO#32996646=blob-a4c56246>>+0x2011 sp 0x42d5c7089fe0
   [174978.450][klog][klog][I]    #5    0x0000000000000000 is not covered by any module sp 0x42d5c7089ff0
   [174978.457][klog][klog][I] KERN: terminating process 'hello_world.cm' (32996655)
   ```

   Verify that the lines in the kernel logs show the exact filenames and line numbers (for example,
   `main() src/hello_world/hello_world.cc:9`) that might've caused the component to crash.

   Press `CTRL+C` to exit.

## 6. Debug the sample component {:#debug-the-sample-component}

Launch the Fuchsia debugger ([`zxdb`][fuchsia-debugger]) for debugging the sample
component, which is now updated to crash when it's started.

The tasks include:

*   Start the Fuchsia debugger and connect it to the emulator instance.
*   Attach the debugger to the sample component.
*   Set a breakpoint.
*   Run the sample component and step through the code.

Do the following:

1. Start the Fuchsia debugger:

   ```posix-terminal
   tools/ffx debug connect
   ```

   This command automatically connects the debugger to the default target
   device, which is the emulator instance.

   When connected to the device, this command starts the `zxdb` terminal, for example:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx debug connect
   Connecting (use "disconnect" to cancel)...
   Connected successfully.
   👉 To get started, try "status" or "help".
   [zxdb]
   ```

1. In the `zxdb` terminal, attach the debugger to the `hello_world.cm` component:

  <pre class="devsite-click-to-copy">
  <span class="no-select">[zxdb] </span>attach hello_world.cm
  </pre>

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] attach hello_world.cm
   Waiting for process matching "hello_world.cm".
   Type "filter" to see the current filters.
   ```

1. In the `zxdb` terminal, set a breakpoint at the `main()` method:

  <pre class="devsite-click-to-copy">
  <span class="no-select">[zxdb] </span>break main
  </pre>

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   [zxdb] break main
   Created Breakpoint 1 @ main
   Pending: No current matches for location. It will be matched against new
            processes and shared libraries.
   ```

1. In a different terminal, run the sample component:

   Note: In this new terminal, make sure that you change to the same work
   directory (for instance,  `cd $HOME/fuchsia-getting-started`).

   ```posix-terminal
   bazel run --config=fuchsia_x64 //src/hello_world:pkg.component
   ```

   In the `zxdb` terminal, the sample component is paused
   at the breakpoint:

   ```none {:.devsite-disable-click-to-copy}
   Attached Process 1 state=Running koid=17658651 name=hello_world.cm
   Downloading symbols...
   Breakpoint 1 now matching 1 addrs for main
   Could not load symbols for "<vDSO>" because there was no mapping for build ID "1dbd2861a642d61b".
   Symbol downloading complete. 0 succeeded, 1 failed.
   🛑 on bp 1, 2 main() • hello_world.cc:8
       6
       7 int main() {
    ▶  8   std::cout << "Hello again, World!\n";
       9   abort();
      10   return 0;
   [zxdb]
   ```

   Note: You can re-build and re-run your component as many times as you want, but do not need to
   restart the debugger or run `attach` again. The debugger will preserve your breakpoints and
   continue watching for future processes called `hello_world.cm`.

1. In the new terminal, monitor device logs for the `hello_world` component:

   ```posix-terminal
   tools/ffx log --filter hello_world
   ```

   This comment prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --filter hello_world
   ...
   [5538.385][core/pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://fuchsiasamples.com/hello_world: []
   [5538.385][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://fuchsiasamples.com/hello_world as fuchsia-pkg://fuchsiasamples.com/hello_world to 940cbd84428125a90e1fbeba7033af7cb0f857f8f0bb2879d6b07cd1001f2225 with TUF
   [5538.408][core/pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://fuchsiasamples.com/hello_world: []
   [5538.409][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://fuchsiasamples.com/hello_world as fuchsia-pkg://fuchsiasamples.com/hello_world to 940cbd84428125a90e1fbeba7033af7cb0f857f8f0bb2879d6b07cd1001f2225 with TUF

   ```

   Notice the `Hello again, World!` line is not printed yet.

1. In the `zxdb` terminal, use `next` to step through the code:

  <pre class="devsite-click-to-copy">
  <span class="no-select">[zxdb] </span>next
  </pre>

   In the `zxdb` terminal, the code get executed line by line, for
   example:

   ```none {:.devsite-disable-click-to-copy}
   ...
   🛑 on bp 1 main() • hello_world.cc:8
       6 
       7 int main() {
    ▶  8   std::cout << "Hello again, World!\n";
       9   abort();
      10   return 0;
   [zxdb] {{ '<strong>'}}next{{ '</strong>'}}
   🛑 main() • hello_world.cc:9
       7 int main() {
       8   std::cout << "Hello again, World!\n";
    ▶  9   abort();
      10   return 0;
      11 }
   ```

   In the device logs terminal, verify that the `Hello again, World!` line is
   now printed:

   ```none {:.devsite-disable-click-to-copy}
   [5694.479][core/ffx-laboratory:hello_world][][I] Hello again, World!
   ```

1. To exit the `zxdb` terminal, type `exit` or press `Ctrl-D`.

   This causes the component to finish the execution of the rest of the code.

   Note: For more information on usages and best practices on `zxdb`, see the
   [zxdb user guide][zxdb-user-guide].

## 7. Inspect components {:#inspect-components}

Retrieve a component's data exposed by Fuchsia's Inspect API. This data can be any
set of specialized information that a Fuchsia component is programmed to collect
while it is running on the device.

Note: For a component to collect and expose inspect data, the implementation
of inspect operations and data types must be placed in the component’s code.
Developers use this inspect feature to collect and expose information that will be
helpful for debugging the component or the system. For details,
see [Fuchsia component inspection overview][inspect-overview].

The tasks include:

*   Scan the list of components on the device that expose inspect data (for
    example, the `bootstrap/archivist` component).
*   Scan the list of selectors provided by the `bootstrap/archivist` component.
*   Inspect a specific set of data from the `bootstrap/archivist` component.

Do the following:

1. View the list of components on the device that expose inspect data:

   ```posix-terminal
   tools/ffx inspect list
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx inspect list
   <component_manager>
   audio_core.cmx
   bootstrap/archivist
   bootstrap/driver_manager
   bootstrap/fshost
   ...
   core/wlandevicemonitor
   core/wlanstack
   crash_reports.cmx
   feedback_data.cmx
   httpsdate_time_source.cmx
   scenic.cmx
   timekeeper.cmx
   ```

   Notice that the `bootstrap/archivist` component is on the list.

1. View all available selectors for the `bootstrap/archivist` component:

   ```posix-terminal
   tools/ffx inspect selectors bootstrap/archivist
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx inspect selectors bootstrap/archivist
   bootstrap/archivist:root/archive_accessor_stats/all/inspect/batch_iterator/get_next:errors
   bootstrap/archivist:root/archive_accessor_stats/all/inspect/batch_iterator/get_next:requests
   bootstrap/archivist:root/archive_accessor_stats/all/inspect/batch_iterator/get_next:responses
   ...
   ```

   Each of these selectors represents a different type of data you can inspect.

1. Inspect the `bootstrap/archivist` component for the recent events data:

   ```posix-terminal
   tools/ffx inspect show bootstrap/archivist:root/events/recent_events
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx inspect show bootstrap/archivist:root/events/recent_events
   bootstrap/archivist:
     metadata:
       filename = fuchsia.inspect.Tree
       component_url = fuchsia-boot:///#meta/archivist.cm
       timestamp = 531685168169295
     payload:
       root:
         events:
           recent_events:
             319:
               @time = 7730479794
               event = log_sink_requested
               moniker = core/memory_monitor
             320:
               @time = 7782621023
               event = log_sink_requested
               moniker = core/bt-a2dp
             321:
               ...
             516:
               @time = 5538432236492
               event = log_sink_requested
               moniker = core/ffx-laboratory:hello_world
             517:
               @time = 5825449627765
               event = component_stopped
               moniker = debug_agent_channel.cmx:1064825
             518:
               @time = 5825475597828
               event = component_stopped
               moniker = core/ffx-laboratory:hello_world
   ```

   This data records all the events triggered by components on the device so far.

## 8. Run tests {:#run-tests}

Run tests on the device by launching test components, which are included
in the [SDK samples repository][sdk-samples-repo]{:.external}.

The tasks include:

*   Build and run the sample test components.
*   Update one of the tests to fail.
*   Verify the failure in the test results.

Do the following:

1. Build and run the sample test components:

   ```posix-terminal
   bazel test --config=fuchsia_x64 --test_output=all //src/hello_world:test_pkg
   ```

   This command runs all the tests in the Hello World component’s test package
   ([`hello_world:test_pkg`][hello-world-test-package]{:.external}).

   The command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ bazel test --config=fuchsia_x64 --test_output=all //src/hello_world:test_pkg
   INFO: Analyzed target //src/hello_world:test_pkg (11 packages loaded, 441 targets configured).
   INFO: Found 1 test target...
   INFO: From Testing //src/hello_world:test_pkg:
   ==================== Test output for //src/hello_world:test_pkg:
   Running 2 test components...
   added repository bazel.test.pkg.hello.gtest
   Running test 'fuchsia-pkg://bazel.test.pkg.hello.gtest/hello_test#meta/hello_gtest_autogen_cml.cm'
   [RUNNING]    HelloTest.BasicAssertions
   [stdout - HelloTest.BasicAssertions]
   Running main() from gmock_main.cc
   Example stdout.
   [PASSED]    HelloTest.BasicAssertions

   1 out of 1 tests passed...
   fuchsia-pkg://bazel.test.pkg.hello.gtest/hello_test#meta/hello_gtest_autogen_cml.cm completed with result: PASSED
   added repository bazel.test.pkg.hello.test
   Running test 'fuchsia-pkg://bazel.test.pkg.hello.test/hello_test#meta/hello_test_autogen_cml.cm'
   [RUNNING]    main
   [stdout - main]
   Example stdout.
   [PASSED]    main

   1 out of 1 tests passed...
   fuchsia-pkg://bazel.test.pkg.hello.test/hello_test#meta/hello_test_autogen_cml.cm completed with result: PASSED
   2 test components passed.
   ================================================================================
   Target //src/hello_world:test_pkg up-to-date:
     bazel-bin/src/hello_world/test_pkg_test_package.sh
   INFO: Elapsed time: 16.866s, Critical Path: 9.80s
   INFO: 105 processes: 46 internal, 56 linux-sandbox, 3 local.
   INFO: Build completed successfully, 105 total actions
   //src/hello_world:test_pkg                                               PASSED in 5.0s

   Executed 1 out of 1 test: 1 test passes.
   INFO: Build completed successfully, 105 total actions
   ```

1. Use a text editor to edit the `src/hello_world/hello_gtest.cc` file, for example:

   ```posix-terminal
   nano src/hello_world/hello_gtest.cc
   ```

1. Replace `EXPECT_STRNE()` with `EXPECT_STREQ()`:

   The test should look like below:

   ```none {:.devsite-disable-click-to-copy}
   TEST(HelloTest, BasicAssertions)
   {
     // Expect two strings not to be equal.
     {{ '<strong>' }}EXPECT_STREQ("hello", "world");{{ '</strong>' }}
     // Expect equality.
     EXPECT_EQ(7 * 6, 42);
   }
   ```

   This change will cause the [GoogleTest][google-test]{:.external} (`hello_gtest`)
   to fail.

1. Save the file and exit the text editor.

1. To verify that the updated test now fails, build and run the `hello_gtest` component:

   ```posix-terminal
   bazel test --config=fuchsia_x64 --test_output=all //src/hello_world:test_pkg.hello_gtest
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ bazel test --config=fuchsia_x64 --test_output=all //src/hello_world:test_pkg.hello_gtest
   INFO: Analyzed target //src/hello_world:test_pkg.hello_gtest (0 packages loaded, 0 targets configured).
   INFO: Found 1 test target...
   FAIL: //src/hello_world:test_pkg.hello_gtest (see /home/alice/.cache/bazel/_bazel_alice/ea119f1048230a864836be3d62fead2c/execroot/__main__/bazel-out/x86_64-fastbuild/testlogs/src/hello_world/test_pkg.hello_gtest/test.log)
   INFO: From Testing //src/hello_world:test_pkg.hello_gtest:
   ==================== Test output for //src/hello_world:test_pkg.hello_gtest:
   added repository bazel.test.pkg.hello.gtest
   Running test 'fuchsia-pkg://bazel.test.pkg.hello.gtest/hello_test#meta/hello_gtest_autogen_cml.cm'
   [RUNNING]   HelloTest.BasicAssertions
   [stdout - HelloTest.BasicAssertions]
   Running main() from gmock_main.cc
   Example stdout.
   src/hello_world/hello_gtest.cc:14: Failure
   Expected equality of these values:
     "hello"
     "world"
   [FAILED]   HelloTest.BasicAssertions

   Failed tests: HelloTest.BasicAssertions
   0 out of 1 tests passed...
   fuchsia-pkg://bazel.test.pkg.hello.gtest/hello_test#meta/hello_gtest_autogen_cml.cm completed with result: FAILED
   One or more test runs failed.
   Tests failed.
   ================================================================================
   Target //src/hello_world:test_pkg.hello_gtest up-to-date:
     bazel-bin/src/hello_world/test_pkg.hello_gtest_run_component.sh
   INFO: Elapsed time: 3.810s, Critical Path: 3.42s
   INFO: 11 processes: 2 internal, 6 linux-sandbox, 3 local.
   INFO: Build completed, 1 test FAILED, 11 total actions
   //src/hello_world:test_pkg.hello_gtest                                   FAILED in 1.8s
     /home/alice/.cache/bazel/_bazel_alice/ea119f1048230a864836be3d62fead2c/execroot/__main__/bazel-out/x86_64-fastbuild/testlogs/src/hello_world/test_pkg.hello_gtest/test.log

   INFO: Build completed, 1 test FAILED, 11 total actions
   ```

**Congratulations! You're now all set with the Fuchsia SDK!**

## Next steps {:#next-steps}

Learn more about the Fuchsia platform and tools in [Fuchsia SDK Fundamentals][fundamentals].

## Appendices

### Clean up the environment {:#clean-up-the-environment}

If you run into a problem while following this guide and decide to start over
from the beginning, consider running the commands below to clean up
your development environment (that is, to clean up directories, build artifacts,
downloaded files, symlinks, configuration settings, and more).

Remove the package repositories created in this guide:

```posix-terminal
tools/ffx repository remove fuchsiasamples.com
```

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

Remove the `fuchsia-getting-started` directory and its artifacts:

Caution: If the SDK samples repository is cloned to a different location
than `$HOME/fuchsia-getting-started`, adjust the directory path in the command
below. Be extremely careful with the directory path when you run the
`rm -rf <DIR>` command.

```posix-terminal
rm -rf $HOME/fuchsia-getting-started
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

### Update the firewall rules {:#update-the-firewall-rules}

When you launch the sample component (for instance, using the command
`tools/ffx component run "fuchsia-pkg://fuchsiasamples.com/hello_world#meta/hello_world.cm"`),
you might run into an issue where the command hangs for a long time and
eventually fails with the following error:

```none {:.devsite-disable-click-to-copy}
Lifecycle protocol could not start the component instance: InstanceCannotResolve
```

In that case, you may need to update the firewall rules on your host machine.

If you’re using the `ufw` firewall, run the following commands:

```posix
sudo ufw allow proto tcp from fe80::/10 to any port 8083 comment 'Fuchsia Package Server'
```

```posix
sudo ufw allow proto tcp from fc00::/7 to any port 8083 comment 'Fuchsia Package Server'
```

However, for other non-`ufw`-based firewalls, you will need to ensure that port 8083
is available for the Fuchsia package server.

<!-- Reference links -->

[using-the-sdk]: /development/sdk/index.md
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
[clang]: https://clang.llvm.org/
[fuchsia-idk]: /development/idk/README.md
[fuchsia-package-repo]: /development/sdk/ffx/create-a-package-repository.md
[symbolize-logs]: /development/sdk/ffx/symbolize-logs.md
[fuchsia-component]: /concepts/components/v2/README.md
[fuchsia-debugger]: /development/sdk/ffx/start-the-fuchsia-debugger.md
[zxdb-user-guide]: /development/debugger/README.md
[inspect-overview]: /development/diagnostics/inspect/README.md
[google-test]: https://google.github.io/googletest/
[ffx]: https://fuchsia.dev/reference/tools/sdk/ffx
[fundamentals]: /get-started/sdk/learn/README.md
[femu]: /development/sdk/ffx/start-the-fuchsia-emulator.md
[sdk-samples-repo]: https://fuchsia.googlesource.com/sdk-samples/getting-started
[hello-world-component]: https://fuchsia.googlesource.com/sdk-samples/getting-started/+/refs/heads/main/src/hello_world/
[fuchsia-package-server]: /development/sdk/ffx/create-a-package-repository.md
[hello-world-test-package]: https://fuchsia.googlesource.com/sdk-samples/getting-started/+/refs/heads/main/src/hello_world/BUILD.bazel#68
[ticket-94614]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=94614
