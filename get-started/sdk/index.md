# Get started with the Fuchsia SDK

This guide provides step-by-step instructions on setting up the Fuchsia SDK
development environment on your host machine using a terminal or
Visual Studio Code (VS Code). Then the guide walks through the basic workflows
of building, running, debugging, and testing Fuchsia components using the
[Fuchsia SDK][using-the-sdk].

Important: The Fuchsia SDK is in active development. At the moment, Fuchsia does
not support general public usage of the Fuchsia SDK. The APIs in the SDK are
subject to change without notice.

Which development environment are you using for this guide?

<div class="device-selector-intro">
  <devsite-nav-buttons name="env" type="text" param="always">
    <button value="terminal" default>Terminal</button>
    <button value="vscode">VS Code</button>
  </devsite-nav-buttons>
</div>

{% dynamic if request.query_string.env == "terminal" %}

Complete the following sections:

1. [Prerequisites](#prerequisites)
1. [Clone the SDK samples repository](#clone-the-sdk-samples-repository)
1. [Start the emulator](#start-the-emulator)
1. [Build and run the sample component](#build-and-run-the-sample-component)
1. [View symbolized logs](#view-symbolized-logs)
1. [Debug the sample component](#debug-the-sample-component)
1. [Inspect components](#inspect-components)
1. [Run tests](#run-tests)

{% dynamic elif request.query_string.env == "vscode" %}

Complete the following sections:

1. [Prerequisites](#prerequisites)
1. [Clone the SDK samples repository](#clone-the-sdk-samples-repository)
1. [Configure a VS Code workspace](#configure-a-vs-code-workspace)
1. [Start the emulator](#start-the-emulator)
1. [Build and run the sample component](#build-and-run-the-sample-component)
1. [View symbolized logs](#view-symbolized-logs)
1. [Debug the sample component](#debug-the-sample-component)
1. [Inspect components](#inspect-components)
1. [Run tests](#run-tests)

{% dynamic endif %}

Found an issue? Please [let us know][sdk-bug]{:.external}.

## Prerequisites {:#prerequisites .numbered}

This guide requires that your host machine meets the following criteria:

- An x64-based machine running Linux or macOS.
- Has at least 15 GB of storage space.
- Supports virtualization for running a [QEMU]{:.external}-based emulator.
- IPv6 is enabled.
- [Git][git-install]{:.external} is installed.
{% dynamic if request.query_string.env == "vscode" %}
- [Visual Studio Code][vscode-install]{:.external} is installed.
{% dynamic endif %}

## Clone the SDK samples repository {:#clone-the-sdk-samples-repository .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/_get-started-sdk-clone-sdk-repo-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/_get-started-sdk-clone-sdk-repo-vs-code.md>>
{% dynamic endif %}

{% dynamic if request.query_string.env == "vscode" %}
## Configure a VS Code workspace {:#configure-a-vs-code-workspace .numbered}

<<_common/_get-started-sdk-configure-vs-code.md>>
{% dynamic endif %}

## Start the emulator {:#start-the-emulator .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/_get-started-sdk-start-emulator-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/_get-started-sdk-start-emulator-vs-code.md>>
{% dynamic endif %}

## Build and run the sample component {:#build-and-run-the-sample-component .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/_get-started-sdk-build-and-run-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/_get-started-sdk-build-and-run-vs-code.md>>
{% dynamic endif %}

## View symbolized logs {:#view-symbolized-logs .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/_get-started-sdk-view-symbolized-logs-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/_get-started-sdk-view-symbolized-logs-vs-code.md>>
{% dynamic endif %}

## Debug the sample component {:#debug-the-sample-component .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/_get-started-sdk-debug-component-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/_get-started-sdk-debug-component-vs-code.md>>
{% dynamic endif %}

## Inspect components {:#inspect-components .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/_get-started-sdk-inspect-components-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/_get-started-sdk-inspect-components-vs-code.md>>
{% dynamic endif %}

## Run tests {:#run-tests .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/_get-started-sdk-run-tests-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/_get-started-sdk-run-tests-vs-code.md>>
{% dynamic endif %}

**Congratulations! You're now all set with the Fuchsia SDK!**

## Next steps {:#next-steps}

Learn more about the Fuchsia platform and tools in
[Fuchsia SDK Fundamentals][fundamentals].

## Appendices

### Update the environment to the latest SDK {:#update-the-environment-to-the-latest-sdk}

To update your development environment to use the latest version
of the Fuchsia SDK, do the following:

1. In a terminal, go to your `fuchsia-getting-started` directory:

   ```posix-terminal
   cd $HOME/fuchsia-getting-started
   ```

1. Update the project repository and its submodules
   to the latest version:

   ```posix-terminal
   git pull --rebase --recurse-submodules
   ```

1. Update the Fuchsia SDK toolchain and dependencies:

   ```posix-terminal
   tools/bazel build @fuchsia_sdk//:fuchsia_toolchain_sdk
   ```

1. Check the new version of the Fuchsia SDK:

   ```posix-terminal
   tools/ffx sdk version
   ```

   Verify that the SDK version is now the latest release version.

### Clean up the environment {:#clean-up-the-environment}

If you run into a problem while following this guide and decide to start over
from the beginning, consider running the commands below to clean up your
development environment (that is, to clean up directories, build artifacts,
downloaded files, symlinks, configuration settings, and more).

Remove the package repositories created in this guide:

```posix-terminal
tools/ffx repository remove workstation-packages
```

```posix-terminal
tools/ffx repository server stop
```

Remove all existing configurations and data of `ffx`:

- {Linux}

  ```posix-terminal
  tools/ffx daemon stop
  ```

  ```posix-terminal
  rm -rf $HOME/.local/share/Fuchsia/ffx
  ```

- {macOS}

  ```posix-terminal
  tools/ffx daemon stop
  ```

  ```posix-terminal
  rm -rf $HOME/Library/Caches/Fuchsia/ffx
  ```

  ```posix-terminal
  rm -rf $HOME/Library/Fuchsia/ffx
  ```

  ```posix-terminal
  rm -rf $HOME/Library/Preferences/Fuchsia/ffx
  ```

  ```posix-terminal
  rm -rf $HOME/Library/Application\ Support/Fuchsia/ffx
  ```

When Bazel fails to build, try the commands below:

- {Linux}

  Note: Running `bazel clean` or deleting the `$HOME/.cache/bazel` directory
  deletes artifacts downloaded by Bazel, which can be around 4 GB. This means
  Bazel will need to download dependencies again next time you run
  `bazel build`.

  ```posix-terminal
  tools/bazel clean --expunge
  ```

  ```posix-terminal
  tools/bazel shutdown && rm -rf $HOME/.cache/bazel
  ```

- {macOS}

  Note: Running `bazel clean` or deleting the `/private/var/temp/bazel$USER`
  directory deletes artifacts downloaded by Bazel, which can be around 4 GB.
  This means Bazel will need to download dependencies again next time you run
  `bazel build`.

  ```posix-terminal
  tools/bazel clean --expunge
  ```

  ```posix-terminal
  tools/bazel shutdown && rm -rf /private/var/tmp/bazel$USER
  ```

Remove the `fuchsia-getting-started` directory and its artifacts:

Caution: If the SDK samples repository is cloned to a different location than
`$HOME/fuchsia-getting-started`, adjust the directory in the command below. Be
extremely careful with the directory path when you run the `rm -rf <DIR>`
command.

```posix-terminal
rm -rf $HOME/fuchsia-getting-started
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
`tools/bazel run`), you might run into an issue where the command hangs for a
long time and eventually fails with the following error:

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

However, for other non-`ufw`-based firewalls, you will need to ensure that port
8083 is available for the Fuchsia package server.

### Check if your Linux machine supports KVM virtualization {:#check-if-your-linux-machine-supports-kvm-virtualization}

To check if your Linux machine supports KVM hardware virtualization,
run the following command:

```posix-terminal
lscpu
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ lscpu
Architecture:            x86_64
  CPU op-mode(s):        32-bit, 64-bit
  Address sizes:         46 bits physical, 48 bits virtual
  Byte Order:            Little Endian
...
Virtualization features:
  {{ '<strong>' }}Virtualization:        VT-x{{ '</strong>' }}
  Hypervisor vendor:     KVM
  Virtualization type:   full
...
```

If you see the following field in the output, your machine
**supports** KVM hardware virtualization:

```none {:.devsite-disable-click-to-copy}
  Virtualization:        VT-x
```

Note: If your machine supports KVM hardware virtualization, see
[Set up KVM virtualization on a Linux machine](#set-up-kvm-virtualization-on-a-linux-machine)
to verify that KVM is configured correctly.

On the other hand, for machines that **support** AMD
virtualization, you may see the following field in the output:

```none {:.devsite-disable-click-to-copy}
  Virtualization:        AMD-V
```

However, if your output does not have the `Virtualization` field at all,
while the `Hypervisor vendor` and `Virtualization type` fields may still
be shown (see the example output below), your machine
**does not support** hardware virtualization.

```none {:.devsite-disable-click-to-copy}
$ lscpu
...
Virtualization features:
  Hypervisor vendor:     KVM
  Virtualization type:   full
...
```

### Set up KVM virtualization on a Linux machine {:#set-up-kvm-virtualization-on-a-linux-machine}

Note: The instructions in this section require that
[your Linux machine supports KVM hardware virtualization](#check-if-your-linux-machine-supports-kvm-virtualization).

To verify that KVM is configured correctly on your Linux machine,
run the following `bash` shell script:

```posix-terminal
if [[ -w /dev/kvm ]] && grep '^flags' /proc/cpuinfo | grep -qE 'vmx|svm'; then echo 'KVM is working'; else echo 'KVM not working'; fi
```

Verify that this shell script prints the following output:

```none {:.devsite-disable-click-to-copy}
KVM is working
```

If the output is `KVM is working`, KVM hardware virtualization is
enabled on your Linux machine.

However, if the output is `KVM not working`, do the following to
enable KVM hardware virtualization:

1. Add yourself to the `kvm` group on your Linux machine:

   ```posix-terminal
   sudo usermod -a -G kvm ${USER}
   ```

1. Reboot the machine.
1. Run the `bash` shell script above again.

   Verify that the output now prints `KVM is working`.

<!-- Reference links -->

[fundamentals]: /get-started/sdk/learn/README.md
[git-install]: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git
[qemu]: https://www.qemu.org/
[sdk-bug]: https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=Bazel
[using-the-sdk]: /development/sdk/index.md
[vscode-install]: https://code.visualstudio.com/Download
