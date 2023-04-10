Clone the SDK samples repository on your host machine. This repository contains
the Bazel-based Fuchsia SDK and sample components.

The tasks include:

- Bootstrap the SDK samples repository.
- Download the SDK toolchain to initialize the SDK environment.
- Verify that you can run [`ffx`][ffx] commands.

Important: If you have a [remote VS Code setup][remote-vs-code-setup],
you need to be on a VS Code window that is connected to your remote machine.

In VS Code, do the following:

1. Click **Terminal > New Terminal**.

1. In the terminal, change to your home directory:

   ```posix-terminal
   cd
   ```

1. Clone the Fuchsia samples repository:

   ```posix-terminal
   git clone https://fuchsia.googlesource.com/sdk-samples/getting-started fuchsia-getting-started --recurse-submodules
   ```

   This `git clone` command creates a new directory named
   `fuchsia-getting-started` and clones the content of the
   [SDK samples repository][sdk-samples-repo]{:.external}.

1. Go to the new directory:

   ```posix-terminal
   cd fuchsia-getting-started
   ```

1. Run the bootstrap script to install Bazel and other required dependencies:

   ```posix-terminal
   scripts/bootstrap.sh
   ```

1. Download the SDK toolchain:

   ```posix-terminal
   tools/bazel build @fuchsia_sdk//:fuchsia_toolchain_sdk
   ```

   This command may take a few minutes to download all the tools and
   dependencies, such as [Clang]{:.external} and
   [Fuchsia IDK][fuchsia-idk] (which includes the `ffx` tool).

   When finished successfully, it prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel build @fuchsia_sdk//:fuchsia_toolchain_sdk
   ...
   INFO: Elapsed time: 25.063s, Critical Path: 0.03s
   INFO: 1 process: 1 internal.
   INFO: Build completed successfully, 1 total action
   ```

1. To verify that you can use the `ffx` tool in your environment, run the
   following command:

   ```posix-terminal
   tools/ffx sdk version
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx sdk version
   11.20230109.3.1
   ```

   At this point, you only need to confirm that you can run `ffx` commands
   without error.

   Note: The output above shows the version `11.20230109.3.1`, which
   indicates that this SDK was built and published on January 9, 2023.

<!-- Reference links -->

[clang]: https://clang.llvm.org
[ffx]: https://fuchsia.dev/reference/tools/sdk/ffx
[fuchsia-idk]: /development/idk/README.md
[remote-vs-code-setup]: /reference/tools/editors/vscode/remote-workspaces.md
[sdk-samples-repo]: https://fuchsia.googlesource.com/sdk-samples/getting-started

