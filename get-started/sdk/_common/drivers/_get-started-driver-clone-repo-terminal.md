Clone the SDK driver samples repository on your host machine. This repository
contains sample driver components and the Bazel-based Fuchsia SDK.

The tasks include:

*   Bootstrap the SDK driver samples repository.
*   Download the SDK toolchain to initialize the SDK environment.
*   Verify that you can run `ffx` commands.

Do the following:

1. Open a terminal.

1. In the terminal, change to your home directory:

   ```posix-terminal
   cd
   ```

1. Clone the SDK driver samples repository:

   ```posix-terminal
   git clone https://fuchsia.googlesource.com/sdk-samples/drivers fuchsia-drivers --recurse-submodules
   ```

   This `git clone` command creates a new directory named `fuchsia-drivers` and
   clones the content of the
   [SDK driver samples repository][sdk-driver-sample-repo]{:.external}.

1. Go to the new directory:

   ```posix-terminal
   cd fuchsia-drivers
   ```

1. Run the bootstrap script to install Bazel and other required dependencies:

   ```posix-terminal
   scripts/bootstrap.sh
   ```

1. Download the SDK toolchain:

   ```posix-terminal
   tools/bazel build @fuchsia_sdk//:fuchsia_toolchain_sdk
   ```

   The first build may take a few minutes to download dependencies, such as
   [Clang][clang] and [Fuchsia IDK][fuchsia-idk] (which includes the `ffx` tool).

   When finished successfully, it prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel build @fuchsia_sdk//:fuchsia_toolchain_sdk
   ...
   INFO: Elapsed time: 23.608s, Critical Path: 0.03s
   INFO: 1 process: 1 internal.
   INFO: Build completed successfully, 1 total action
   ```

5. To verify that you can use the `ffx` tool in your environment, run the
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

   Note: The output above shows the version `11.20230109.3.1`, which indicates that
   this SDK was built and published on January 9, 2023.

<!-- Reference links -->

[clang]: https://clang.llvm.org/
[fuchsia-idk]: /development/idk/README.md
[sdk-driver-sample-repo]: https://fuchsia.googlesource.com/sdk-samples/drivers
