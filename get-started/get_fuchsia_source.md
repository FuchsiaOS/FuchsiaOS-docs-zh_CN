<!-- 
# Get Fuchsia source code

This guide provides instructions for the following tasks:

*   [Download the Fuchsia source code](#download-fuchsia-source).
*   [Set up environment variables](#set-up-environment-variables). 
-->

# 获取 Fuchsia 源代码

通过本文，你将了解到如何:

*   [下载 Fuchsia 源代码](#download-fuchsia-source).
*   [设置环境变量](#set-up-environment-variables).

<!-- 
## Before you start

We recommend you run `ffx platform preflight` before you continue.
`preflight` examines your development machine and informs you of issues that
may affect building Fuchsia from source or running the Fuchsia emulator. 
-->

## 开始之前

我们建议您在开始之前，先运行一遍 `ffx platform preflight` 命令.
其中，参数 `preflight` 用来检测你的电脑，然后罗列出影响源码构建和模拟器运行的因素。

<!-- 
*   For **Linux**, run:

    ```posix-terminal
    curl -sO https://storage.googleapis.com/fuchsia-ffx/ffx-linux-x64 && chmod +x ffx-linux-x64 && ./ffx-linux-x64 platform preflight
    ```

*   For **MacOS**, run:

    ```posix-terminal
    curl -sO https://storage.googleapis.com/fuchsia-ffx/ffx-macos-x64 && chmod +x ffx-macos-x64 && ./ffx-macos-x64 platform preflight
    ``` 
-->

*   对于 **Linux** 系统, 请执行以下命令:

    ```posix-terminal
    curl -sO https://storage.googleapis.com/fuchsia-ffx/ffx-linux-x64 && chmod +x ffx-linux-x64 && ./ffx-linux-x64 platform preflight
    ```

*   对于 **MacOS** 系统, 请执行以下命令:

    ```posix-terminal
    curl -sO https://storage.googleapis.com/fuchsia-ffx/ffx-macos-x64 && chmod +x ffx-macos-x64 && ./ffx-macos-x64 platform preflight
    ```

<!-- 
## Prerequisites

The Fuchsia project requires `curl`, `unzip`, and `git` to be up-to-date:

*   For **Linux**, install or update the following packages:

    ```posix-terminal
    sudo apt-get install curl git unzip
    ```

    Note: Fuchsia requires the version of Git to be 2.28 or higher.

*   For **macOS**, install the Xcode command line tools:

    ```posix-terminal
    xcode-select --install
    ``` 
-->

## 前提条件

构建 Fuchsia 工程需要将 `curl`、`unzip`和 `git` 保持最新版本:

*   对于 **Linux**，使用以下命令来安装或更新这些软件：

    ```posix-terminal
    sudo apt-get install curl git unzip
    ```

    提示: Fuchsia 要求 Git 使用 2.28 及以上版本.

*   对于 **macOS**, 请安装 Xcode command line tools:

    ```posix-terminal
    xcode-select --install
    ```

<!-- 
## Download Fuchsia source {#download-fuchsia-source}

Fuchsia's [bootstrap script](/scripts/bootstrap) creates a `fuchsia` directory
and downloads the content of the Fuchsia source repository to this new
directory. 
-->

## 下载 Fuchsia 源代码 {#download-fuchsia-source}

Fuchsia 的 [bootstrap 脚本](/scripts/bootstrap)会创建一个名为 `fuchsia` 的目录，
所有从源代码仓库下载的内容都会保存在该目录下。

<!-- 
Note: Downloading Fuchsia source requires ~2 GiB of space on your machine. In
addition, you will need another 80-90 GiB of space when you build Fuchsia,
depending on your build configuration. 
-->

说明：下载 Fuchsia源代码需要你的设备拥有至少 2 GiB 的存储空间。此外， 根据你选择的 Fuchsia 构建配置不同，构建时还需要额外的 80-90 GiB 存储空间。

<!-- 
To download the Fuchsia source, do the following:

1.  Select a directory for downloading the Fuchsia source code, for example:

    Note: While you can set up Fuchsia in any directory, this guide uses the
    home directory.

    ```posix-terminal
    cd ~
    ```

1.  Run the bootstrap script:

    ```posix-terminal
    curl -s "https://fuchsia.googlesource.com/fuchsia/+/HEAD/scripts/bootstrap?format=TEXT" | base64 --decode | bash
    ```
    This script creates a `fuchsia` directory to download the source code.
    Downloading Fuchsia source can take up to 60 minutes.

    If you see the `Invalid authentication credentials` error during the
    bootstrapping process, see [Authentication error](#authentication-error) for
    help. 
-->

执行下述操作，来下载 Fuchsia 源代码：

1.  选择一个目录，用来保存源代码：

    说明：你可以选择任意目录， 此处我们选择用户主目录

    ```posix-terminal
    cd ~
    ```

2.  运行 bootstrap 脚本:

    ```posix-terminal
    curl -s "https://fuchsia.googlesource.com/fuchsia/+/HEAD/scripts/bootstrap?format=TEXT" | base64 --decode | bash
    ```
    这个脚本会创建一个名为 `fuchsia` 的文件夹来保存源代码.
    下载过程最多可能需要 60 分钟.

    脚本执行过程如果出现 `Invalid authentication credentials` 错误，请参考 [身份验证错误](#authentication-error) 来解决该问题。

<!-- 
## Set up environment variables {#set-up-environment-variables}

Fuchsia recommends updating your shell script to perform the following actions
(see [Update your shell script](#update-your-shell-script) for the instructions): 
-->

## 设置环境变量 {#set-up-environment-variables}

Fuchsia 建议通过更新你所使用的 shell 的配置脚本来完成下述操作
(参见 [更新 shell 脚本](#update-your-shell-script) ):

<!-- 
*   Add the `.jiri_root/bin` directory to your `PATH`.

    The `.jiri_root/bin` directory in the Fuchsia source contains the
    <code>[jiri](https://fuchsia.googlesource.com/jiri){:.external}</code> and
    <code>[fx](/docs/development/build/fx.md)</code> tools are essential to
    Fuchsia workflows. Fuchsia uses the `jiri` tool to manage repositories in
    the Fuchsia project. The `fx` tool helps configure, build, run, and debug
    Fuchsia. The Fuchsia toolchain requires `jiri` to be available in your
    `PATH`. 
-->

*   将 `.jiri_root/bin` 目录添加到你的 `PATH` 环境变量中

    Fuchsia  源代码中 `.jiri_root/bin` 目录包含的
    <code>[jiri](https://fuchsia.googlesource.com/jiri){:.external}</code> 和
    <code>[fx](/docs/development/build/fx.md)</code> 是
    Fuchsia 工作流中至关重要的工具。`jiri` 用来管理 Fuchsia 项目中各个源代码仓库。`fx` 则用来 配置、构建、运行和调试 Fuchsia。 Fuchsia 工具链需要 `jiri` 被包含在 `PATH` 环境变量中。

<!-- 
*   Source the `scripts/fx-env.sh` file.

    Although it's not required, sourcing the
    <code>[fx-env.sh](/scripts/fx-env.sh)</code> script enables useful shell
    functions in your terminal. For instance, it creates a `FUCHSIA_DIR`
    environment variable and provides the `fd` command for navigating
    directories with auto-completion (see comments in `fx-env.sh` for more
    information). 
-->

*   执行 `scripts/fx-env.sh` 脚本文件。

    虽然并非必要，但是执行<code>[fx-env.sh](/scripts/fx-env.sh)</code> 脚本可以帮助你在终端中启用一些有用的方法。例如，它会创建一个名为 `FUCHSIA_DIR`
    的环境变量，用来指向 Fuchsia 目录， 并且提供一个名为 `fd` 的命令，用来t提供目录切换时，提供针对性的自动补全功能 (请查看 `fx-env.sh` 文件中的注释获取更多信息)。

<!-- 
### Update your shell script {#update-your-shell-script}

Update your shell script to add Fuchsia's environment variables
in your terminal.

Note: If you don't wish to update your environment variables, see
[Work on Fuchsia without updating your PATH](#work-on-fuchsia-without-updating-your-path). 
-->

### 更新你的 shell 配置脚本 {#update-your-shell-script}

更新你的 shell 配置脚本 用以将 Fuchsia 的环境变量添加到你的终端环境中。

说明：如果你不想修改环境变量，请参考[以不修改 PATH 环境变量的方式开发 Fuchsia](#work-on-fuchsia-without-updating-your-path)。

<!-- 
Do the following:

1.  Use a text editor to open your `~/.bash_profile` file, for example:

    Note: This guide uses a `bash` terminal as an example. If you are
    using `zsh`, replace `~/.bash_profile` with `~/.zprofile` in the
    following steps:

    ```posix-terminal
    nano ~/.bash_profile
-->

执行下述操作：

1.  使用文本编辑器打开 `~/.bash_profile` 文件，比如：

    说明：此处以 `bash` 终端 为例。 如果你使用的是 `zsh`，请将下面的 `~/.bash_profile` 替换为 `~/.zprofile` ：

    ```posix-terminal
    nano ~/.bash_profile
    ```

<!-- 
1.  Add the following lines to your `~/.bash_profile` file:

    Note: If your Fuchsia source code is not located in the `~/fuchsia`
    directory, replace `~/fuchsia` with your Fuchsia directory.

    ```sh
    export PATH=~/fuchsia/.jiri_root/bin:$PATH
    source ~/fuchsia/scripts/fx-env.sh
    ```
 -->

2.  向 `~/.bash_profile` 文件中添加下述内容：

    说明：如果你修改了 Fuchsia 目录，请替换 `~/fuchsia` 为你的 Fuchsia 实际保存目录。

    ```sh
    export PATH=~/fuchsia/.jiri_root/bin:$PATH
    source ~/fuchsia/scripts/fx-env.sh
    ```

<!-- 
1.  Save the file and exit the text editor.
1.  To update your environment variables, run the following command:

    ```posix-terminal
    source ~/.bash_profile
    ```
 -->

3.  保存文件，并退出文本编辑器。
4.  执行下述命令来更新环境变量：
    ```posix-terminal
    source ~/.bash_profile
    ```

<!-- 
1.  Verify that you can run the following commands inside your
    `fuchsia` directory without error:

    ```posix-terminal
    jiri help
    ```

    ```posix-terminal
    fx help
    ```
 -->

5.  在 `fuchsia` 目录执行下述命令，确保没有错误：

    ```posix-terminal
    jiri help
    ```

    ```posix-terminal
    fx help
    ```

<!-- 
## Next steps

See
[Configure and build Fuchsia](/docs/get-started/build_fuchsia.md)
in the Getting started guide for the next steps.
-->

## 下一步

前往入门指南中 [配置和构建 Fuchsia](/docs/get-started/build_fuchsia.md) ，执行下一步操作。
    
<!-- 
## Troubleshoot

### Authentication error {#authentication-error}

If you see the `Invalid authentication credentials` error during the bootstrap
process, your `~/.gitcookies` file may contain cookies from some repositories in
`googlesource.com` that the bootstrap script wants to check out anonymously.

To resolve this error, do one of the following:

*   Follow the onscreen directions to get passwords for the specified
    repositories.
*   Delete the offending cookies from the `.gitcookies` file. 
-->

## 故障排除

### 身份验证错误 {#authentication-error}

如果你在 bootstrap 脚本执行过程中遇到 `Invalid authentication credentials` 错误， 很可能是因为你的 `~/.gitcookies` 文件包含了来自 
`googlesource.com` 的仓库的cookies，bootstrap 脚本会匿名地检出这些cookies。

为避免此错误，你可以尝试执行下述任意一个操作：

*   根据屏幕上的指导，获取指定仓库的密码
*   从 `.gitcookies` 文件中删除有问题的 cookies。

<!-- 
### Work on Fuchsia without updating your PATH {#work-on-fuchsia-without-updating-your-path}

The following sections provide alternative approaches to the
[Update your shell script](#update-your-shell-script) section:

*   [Copy the tool to your binary directory](#copy-the-tool-to-your-binary-directory)
*   [Add a symlink to your binary directory](#add-a-symlink-to-your-binary-directory)
-->

### 以不修改 PATH 环境变量的方式开发 Fuchsia {#work-on-fuchsia-without-updating-your-path}

下面提供了几种[更新 shell 配置脚本](#update-your-shell-script) 的备选方案：

*   [拷贝 Fuchsia 工具到 bin 目录](#copy-the-tool-to-your-binary-directory)
*   [创建 Fuchsia 工具链接到 bin 目录](#add-a-symlink-to-your-binary-directory)

<!-- 
#### Copy the tool to your binary directory {#copy-the-tool-to-your-binary-directory}

If you don't wish to update your environment variables, but you want `jiri` to
work in any directory, copy the `jiri` tool to your `~/bin` directory, for
example:

Note: If your Fuchsia source code is not located in the `~/fuchsia` directory,
replace `~/fuchsia` with your Fuchsia directory.

```posix-terminal
cp ~/fuchsia/.jiri_root/bin/jiri ~/bin
```

However, you must have write access to the `~/bin` directory without `sudo`. If
you don't, `jiri` cannot keep itself up-to-date.
-->

#### 拷贝 Fuchsia 工具到二进制目录 {#copy-the-tool-to-your-binary-directory}

如果你不想更新环境变量，但又想要在任意目录使用 `jiri` 工具， 则可以把 `jiri` 工具拷贝到 `~/bin` 目录中， 比如：

说明：如果你的 Fuchsia 源代码不在 `~/fuchsia` 目录，请替换 `~/fuchsia` 为源代码实际目录。

```posix-terminal
cp ~/fuchsia/.jiri_root/bin/jiri ~/bin
```

然而，为了保证 `jiri` 始终为最新版本，你必须在不借助 `sudo` 的情况下，拥有对 `~/bin` 目录的写入权限。

<!-- 
#### Add a symlink to your binary directory {#add-a-symlink-to-your-binary-directory}

Similarly, if you want to use the `fx` tool without updating your environment
variables, provide the `fx` tool's symlink in your `~/bin` directory, for
example:

Note: If your Fuchsia source code is not located in the `~/fuchsia` directory,
replace `~/fuchsia` with your Fuchsia directory.

```posix-terminal
ln -s ~/fuchsia/scripts/fx ~/bin
```

Alternatively, run the `fx` tool directly using its path, for example:

```posix-terminal
./scripts/fx help
```

In either case, you need `jiri` in your `PATH`.
-->

#### 创建 Fuchsia 工具链接到二进制目录 {#add-a-symlink-to-your-binary-directory}

同样的，如果你不想更新环境变量，但又想使用 `fx` 工具， 则可以把 `jiri` 工具链接到 `~/bin` 目录中， 比如：

说明： 如果你的 Fuchsia 源代码不在 `~/fuchsia` 目录，请替换 `~/fuchsia` 为源代码实际目录。

```posix-terminal
ln -s ~/fuchsia/scripts/fx ~/bin
```

又或者，你可以直接通过绝对路径来使用 `fx` ， 比如： 

```posix-terminal
./scripts/fx help
```

但是， 无论哪种情况，你都需要在 `PATH` 环境变量中包含 `jiri` 。