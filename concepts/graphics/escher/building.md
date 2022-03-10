<!--

# Escher build instructions

Escher can be built targeting both Fuchsia and Linux.  Building for Linux is useful because it allows use of Vulkan development tools that are not yet available on Fuchsia.

-->

# Escher 构建说明

可以针对 Fuchsia 和 Linux 构建 Escher。 
为 Linux 构建非常有用，
因为它允许使用 Fuchsia 上尚不可用的 Vulkan 开发工具。 

<!--

## Building for Fuchsia

Escher itself is part of any Fuchsia build that includes Scenic, i.e. any build that targets a device with a screen.  The Escher examples and tests are built by adding `//garnet/packages/examples:escher` and `//garnet/packages/tests:escher` to your `fx set` invocation.

-->

## 为 Fuchsia 构建

Escher 本身是包含 Scenic 的任何 Fuchsia 构建的一部分，
即任何针对具有屏幕的设备的构建。 
Escher 示例和测试是通过将 `//garnet/packages/examples:escher` 
和 `//garnet/packages/tests:escher` 添加到 `fx set` 
调用来构建的。 

<!--

## Building for Linux

Escher can also build on Linux.  In order to do so, you need to:

  * Install build dependencies:

    ```
    sudo apt install libxinerama-dev libxrandr-dev libxcursor-dev libx11-xcb-dev libx11-dev mesa-common-dev
    ```

-->

## 为 Linux 构建

Escher 也可以在 Linux 上构建。 为此，您需要：

   * 安装构建依赖：

     ```
     sudo apt install libxinerama-dev libxrandr-dev libxcursor-dev libx11-xcb-dev libx11-dev mesa-common-dev
     ```

<!--

  * Install a GPU driver that supports Vulkan:

    * NVIDIA: version >= 367.35

      ```
      sudo apt install nvidia-driver
      ```

    * Intel: Mesa >= 12.0

      ```
      sudo apt install mesa-vulkan-drivers
      ```

-->

   * 安装支持 Vulkan 的 GPU 驱动：

     * NVIDIA：version >= 367.35

        ```
       sudo apt install nvidia-driver
       ```

     * Intel：Mesa >= 12.0

        ```
       sudo apt install mesa-vulkan-drivers
       ```
<!--

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
    fx set terminal.x64 --with='//garnet/packages/examples:escher,//garnet/packages/tests:escher' --args escher_use_null_vulkan_config_on_host=false
    ```

    * See [Getting started](/docs/get-started/README.md) for how to set up the `fx` tool.

    * The command-line above is just an example.  For example, you can use a different product than `terminal` or use a more inclusive package such as `//garnet/packages/examples:all`.

  * Do the following each time you want to rebuild and run the `waterfall` example:

    ```
    fx build host_x64/waterfall && out/default/host_x64/waterfall
    ```

  * Escher unit-tests are built and run similarly:

    ```
    fx build host_x64/escher_unittests && out/default/host_x64/escher_unittests
    fx build host_x64/escher_renderer_tests && out/default/host_x64/escher_renderer_tests
    ```
-->

  * 设置 `VK_LAYER_PATH` 和 `LD_LIBRARY_PATH` 环境变量。 这里我们使用 Vulkan SDK 中的 Vulkan 层和加载器： 

    ```
    export VULKAN_SDK=$FUCHSIA_DIR/prebuilt/third_party/vulkansdk/linux/x86_64
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$VULKAN_SDK/lib
    export VK_LAYER_PATH=$VULKAN_SDK/etc/vulkan/explicit_layer.d
    ```

  * 指定您希望构建 Escher 示例和单元测试： 

    ```
    cd $FUCHSIA_DIR
    fx set terminal.x64 --with='//garnet/packages/examples:escher,//garnet/packages/tests:escher' --args escher_use_null_vulkan_config_on_host=false
    ```

    * 如何设置`fx`工具见 [Getting started](/docs/get-started/README.md)。

    * 上面的命令行只是一个例子。 例如，您可以使用与 `terminal` 不同的产品或使用更具包容性的包，例如 `//garnet/packages/examples:all`。 

  * 每次要重建并运行`waterfall`示例时，请执行以下操作：

    ```
    fx build host_x64/waterfall && out/default/host_x64/waterfall
    ```

  * Escher 单元测试的构建和运行方式类似：

    ```
    fx build host_x64/escher_unittests && out/default/host_x64/escher_unittests
    fx build host_x64/escher_renderer_tests && out/default/host_x64/escher_renderer_tests
    ```