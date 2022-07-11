# Escher build instructions

Escher can be built targeting both Fuchsia and Linux.  Building for Linux is useful because it allows use of Vulkan development tools that are not yet available on Fuchsia.

## Building for Fuchsia

Escher itself is part of any Fuchsia build that includes Scenic, i.e. any build that targets a device with a screen.  The Escher examples and tests are built by adding `//src/ui/examples/escher` and `//src/ui/lib/escher:escher_tests` to your `fx set` invocation.

## Building for Linux

Escher can also build on Linux.  In order to do so, you need to:

  * Install build dependencies:

    ```
    sudo apt install libxinerama-dev libxrandr-dev libxcursor-dev libx11-xcb-dev libx11-dev mesa-common-dev
    ```

  * Install a GPU driver that supports Vulkan:

    * NVIDIA: version >= 367.35

      ```
      sudo apt install nvidia-driver
      ```

    * Intel: Mesa >= 12.0

      ```
      sudo apt install mesa-vulkan-drivers
      ```

  * Set the `VK_LAYER_PATH` and `LD_LIBRARY_PATH` environment variables. Here we use the Vulkan layers
    and loader from Vulkan SDK:

    ```
    export VULKAN_SDK=$FUCHSIA_DIR/prebuilt/third_party/vulkansdk/linux/x86_64
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$VULKAN_SDK/lib
    export VK_LAYER_PATH=$VULKAN_SDK/etc/vulkan/explicit_layer.d
    ```

  * Specify that you want the Escher examples and unit-tests to be built:

    ```
    cd $FUCHSIA_DIR
    fx set terminal.x64 --with='//src/ui/examples/escher,//src/ui/lib/escher:escher_tests'
    ```

    * See [Getting started](/get-started/README.md) for how to set up the `fx` tool.

    * The command-line above is just an example.

  * Do the following each time you want to rebuild and run the `waterfall` example:

    ```
    fx build host_x64/waterfall && out/default/host_x64/waterfall
    ```

  * Escher unit-tests are built and run similarly:

    ```
    fx build host_x64/escher_unittests && out/default/host_x64/escher_unittests
    fx build host_x64/escher_renderer_tests && out/default/host_x64/escher_renderer_tests
    ```
