# GN C++ Frontend SDK

## Prerequisites

### Supported host platforms and build systems

The GN C++ Frontend SDK samples only support Linux hosts and the [GN build system](https://gn.googlesource.com/gn/).

### Supported Fuchsia device CPU architectures

Fuchsia devices or emulators must have one of the following CPU architectures to run Fuchsia:

* `x64`
* `arm64`

## Setup

1. Install required dependencies:

   ```shell
   sudo apt-get install curl unzip python2
   ```

1. Clone [the samples repo](https://fuchsia.googlesource.com/samples) and submodules:

   ```shell
   git clone https://fuchsia.googlesource.com/samples --recursive
   ```

   Note: If you have already cloned this repo without the `--recursive` flag you can download the submodules: `git submodule update --init --recursive`

1. Change directory to the root of the repo and run the tests:

   ```shell
   cd samples
   ./scripts/setup-and-test.sh
   ```

   This script:

   * Downloads additional build tool dependencies (may take 5-30 minutes)
   * Builds the samples
   * Runs tests on your host

   If the script fails with an error, address the issue and run the script again.

## Testing on a Fuchsia compatible device

Note: This guide assumes a `x64` Fuchsia compatible device. For Fuchsia compatible ARM devices
replace `x64` with `arm64` and `generic-x64`  and `generic-arm64` or other compatible image name in
the instructions below. For a full list of available images [install gsutil](https://cloud.google.com/storage/docs/gsutil_install)
and run: `gsutil ls gs://fuchsia/development/$(gsutil cat gs://fuchsia/development/LATEST_LINUX)/images`

1. Setup your device and install Zedboot:
   Find the instructions for your device (e.g. a [NUC](/docs/development/hardware/intel_nuc.md) or [Pixelbook](/docs/development/hardware/pixelbook.md))
   to setup and install Zedboot on your device.

1. From the `samples` directory, create Ninja build files for the samples by running the following command:

   ```shell
   buildtools/linux64/gn gen out/generic-x64 --args='target_os="fuchsia" target_cpu="x64"'
   ```

1. Build the samples by executing the Ninja build files you created in the previous step:

   ```shell
   buildtools/linux64/ninja -C out/generic-x64
   ```

1. Make sure your Fuchsia device is booted to Zedboot and connected to your host machine the pave your device:

   ```shell
   third_party/fuchsia-sdk/bin/fpave.sh --image generic-x64
   ```

1. Start a package server from your host machine:

   ```shell
   third_party/fuchsia-sdk/bin/fserve.sh --image generic-x64
   ```

1. To publish your packages open another another terminal window and run:

   ```shell
   third_party/fuchsia-sdk/bin/fpublish.sh out/generic-x64/hello_far.far
   third_party/fuchsia-sdk/bin/fpublish.sh out/generic-x64/rot13_server.far
   third_party/fuchsia-sdk/bin/fpublish.sh out/generic-x64/rot13_client.far
   ```

1. Run the sample components on your Fuchsia device:

    1. SSH to the device:

       ```shell
       third_party/fuchsia-sdk/bin/fssh.sh
       ```

    1. Once SSHed into the device, run the hello world component on the device:

       ```shell
       run fuchsia-pkg://fuchsia.com/hello_world#meta/hello_world.cmx
       ```

    1. Run the rot13 server:

       ```shell
       run fuchsia-pkg://fuchsia.com/rot13_server#meta/rot13_server.cmx
       ```

    1. In a new terminal window on your host machine, open a new ssh connection to your device, and run the rot13 client:

       ```shell
       third_party/fuchsia-sdk/bin/fssh.sh
       run fuchsia-pkg://fuchsia.com/rot13_client#meta/rot13_client.cmx
       ```

## Links

* [GN C++ Frontend SDK Samples Repository](https://fuchsia.googlesource.com/samples)
