# Get started with driver development

This guide provides step-by-step instructions on setting up the Fuchsia
driver development environment on your host machine using a terminal or
Visual Studio Code (VS Code). Then the guide walks through the basic
workflows of building, running, debugging, and updating
[drivers][driver-concepts] in a Fuchsia system using the
[Fuchsia SDK][using-the-sdk].

Important: This guide is the driver equivalent of the
[_Get started with the Fuchsia SDK_][get-started-sdk] guide. If you haven't
already, it's strongly recommended that you complete _Get started with the
Fuchsia SDK_ first to become familiar with the comprehensive set of
Fuchsia SDK workflows.

Which development environment are you using for this guide?

<div class="device-selector-intro">
  <devsite-nav-buttons name="env" type="text" param="always">
    <button value="terminal" default>Terminal</button>
    <button value="vscode">VS Code</button>
  </devsite-nav-buttons>
</div>

{% dynamic if request.query_string.env == "terminal" %}

Complete the following sections:

1. [Prerequisites](#prerequisites).
1. [Clone the SDK driver samples repository](#clone-the-sdk-driver-samples-repository).
1. [Start the emulator](#start-the-emulator).
1. [Build and load the sample driver](#build-and-load-the-sample-driver).
1. [Build and run a tool](#build-and-run-a-tool).
1. [Debug the sample driver](#debug-the-sample-driver).
1. [Modify and reload the sample driver](#modify-and-reload-the-sample-driver).

{% dynamic elif request.query_string.env == "vscode" %}

Complete the following sections:

1. [Prerequisites](#prerequisites).
1. [Clone the SDK driver samples repository](#clone-the-sdk-driver-samples-repository).
1. [Configure a VS Code workspace](#configure-a-vs-code-workspace)
1. [Start the emulator](#start-the-emulator).
1. [Build and load the sample driver](#build-and-load-the-sample-driver).
1. [Build and run a tool](#build-and-run-a-tool).
1. [Debug the sample driver](#debug-the-sample-driver).
1. [Modify and reload the sample driver](#modify-and-reload-the-sample-driver).

{% dynamic endif %}

Found an issue? Please [let us know][sdk-bug]{:.external}.

## Prerequisites {:#prerequisites .numbered}

This guide requires that your host machine meets the following criteria:

-  An x64-based machine running Linux or macOS.
-  Has at least 15 GB of storage space.
-  Supports [KVM][kvm]{:.external} (Kernel Virtual Machine) for running a
   [QEMU][qemu]{:.external}-based emulator.
-  IPv6 is enabled.
-  [Git][git-install]{:.external} is installed.
{% dynamic if request.query_string.env == "vscode" %}
- [Visual Studio Code][vscode-install]{:.external} is installed.
{% dynamic endif %}

## Clone the SDK driver samples repository {:#clone-the-sdk-driver-samples-repository .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/drivers/_get-started-driver-clone-repo-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/drivers/_get-started-driver-clone-repo-vs-code.md>>
{% dynamic endif %}

{% dynamic if request.query_string.env == "vscode" %}
## Configure a VS Code workspace {:#configure-a-vs-code-workspace .numbered}

<<_common/drivers/_get-started-driver-configure-vs-code.md>>
{% dynamic endif %}

## Start the emulator {:#start-the-emulator .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/drivers/_get-started-driver-start-emulator-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/drivers/_get-started-driver-start-emulator-vs-code.md>>
{% dynamic endif %}

## Build and load the sample driver {:#build-and-load-the-sample-driver .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/drivers/_get-started-driver-build-and-load-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/drivers/_get-started-driver-build-and-load-vs-code.md>>
{% dynamic endif %}

## Build and run a tool {:#build-and-run-a-tool .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/drivers/_get-started-driver-build-and-run-tool-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/drivers/_get-started-driver-build-and-run-tool-vs-code.md>>
{% dynamic endif %}

## Debug the sample driver {:#debug-the-sample-driver .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/drivers/_get-started-driver-debug-driver-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/drivers/_get-started-driver-debug-driver-vs-code.md>>
{% dynamic endif %}

## Modify and reload the sample driver {:#modify-and-reload-the-sample-driver .numbered}

{% dynamic if request.query_string.env == "terminal" %}
<<_common/drivers/_get-started-driver-modify-driver-terminal.md>>
{% dynamic elif request.query_string.env == "vscode" %}
<<_common/drivers/_get-started-driver-modify-driver-vs-code.md>>
{% dynamic endif %}

**Congratulations! You’re now all set with the Fuchsia driver development!**

## Next steps {:#next-steps}

Learn more about how the `qemu_edu` driver works
in [Codelab: QEMU edu driver][codelab-qemu-edu-driver].

## Appendices

### Update the environment to the latest SDK {:#update-the-environment-to-the-latest-sdk}

To update your development environment to use the latest version
of the Fuchsia SDK, do the following:

1. In a terminal, go to your `fuchsia-drivers` directory:

   ```posix-terminal
   cd $HOME/fuchsia-drivers
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
from the beginning, consider running the commands below to clean up
your development environment (that is, to clean up directories, build artifacts,
downloaded files, symlinks, configuration settings, and more).

Remove the package repositories created in this guide:

```posix-terminal
tools/ffx repository remove workstation-packages
```

```posix-terminal
tools/ffx repository server stop
```

Remove all existing configurations and data of `ffx`:

* {Linux}

  ```posix-terminal
  tools/ffx daemon stop
  ```

  ```posix-terminal
  rm -rf $HOME/.local/share/Fuchsia/ffx
  ```

* {macOS}

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


* {Linux}

  Note: Running `bazel clean` or deleting the `$HOME/.cache/bazel` directory
  deletes artifacts downloaded by Bazel, which can be around 4 GB. This means
  Bazel will need to download dependencies again next time you run `bazel build`.

  ```posix-terminal
  tools/bazel clean --expunge
  ```

  ```posix-terminal
  tools/bazel shutdown && rm -rf $HOME/.cache/bazel
  ```

* {macOS}

  Note: Running `bazel clean` or deleting the `/private/var/tmp/bazel$USER`
  directory deletes artifacts downloaded by Bazel, which can be around 4 GB.
  This means Bazel will need to download dependencies again next time you run
  `bazel build`.

  ```posix-terminal
  tools/bazel clean --expunge
  ```

  ```posix-terminal
  tools/bazel shutdown && rm -rf /private/var/tmp/bazel$USER
  ```

Remove the `fuchsia-drivers` directory and its artifacts:

Caution: If the driver samples repository is cloned to a different location
than `$HOME/fuchsia-drivers`, adjust the directory in the command below.
Be extremely careful with the directory path when you run the `rm -rf
<DIR>` command.

```posix-terminal
rm -rf $HOME/fuchsia-drivers
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
[git-install]: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git
[driver-concepts]: /concepts/drivers/README.md
[codelab-qemu-edu-driver]: /get-started/sdk/learn/driver/introduction.md
[vscode-install]: https://code.visualstudio.com/Download
