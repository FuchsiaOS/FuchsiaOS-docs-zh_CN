<!--
# Download the Fuchsia source code
-->
# 下载 Fuchsia 源码
<!--
This guide provides instructions on how to download the
Fuchsia source code and set up the Fuchsia development
environment on your machine.
-->
这篇文章将指导您下载 Fuchsia 源码以及在您的机器上部署开发环境。
<!--
The steps are:
-->
步骤如下：
<!--
1. [Perform a preflight check](#perform-a-preflight-check).
2. [Install prerequisite packages](#install-prerequisite-packages).
3. [Download the Fuchsia source code](#download-the-fuchsia-source-code).
4. [Set up environment variables](#set-up-environment-variables).
5. [Configure firewall rules (Optional)](#configure-firewall-rules).
-->
1. [前期检查](#perform-a-preflight-check).
2. [安装用到的软件包](#install-prerequisite-packages).
3. [下载 Fuchsia 源码](#download-the-fuchsia-source-code).
4. [设置环境变量](#set-up-environment-variables).
5. [配置防火墙（可选项）](#configure-firewall-rules).

<!--
## 1. Perform a preflight check {#perform-a-preflight-check}
-->
## 1. 前期检查 {#perform-a-preflight-check}
<!--
Fuchsia provides a preflight check tool
([`ffx platform preflight`][ffx-platform-preflight])
that examines your machine and informs you of any issues that may
affect building Fuchsia from source on the machine.
-->
Fuchsia 提供了一个自动检测工具（[`ffx platform preflight`][ffx-platform-preflight]），这个工具可以测试您的机器，并会在发现任何影响您编译 Fuchsia 源码的问题时通知您。
<!--
Note: The preflight tool only works for the x64 architecture. Fuchsia
is currently not guaranteed to build successfully on other host
architectures, such as Windows and ARM64.
-->
注意：这个检测工具只支持 x64 架构。Fuchsia 目前对其他架构的机器上的编译不做成功担保，比如 Windows 和 ARM64。
<!--
Run the following command:
-->
运行下面的命令：

* {Linux}

  ```posix-terminal
  curl -sO https://storage.googleapis.com/fuchsia-ffx/ffx-linux-x64 && chmod +x ffx-linux-x64 && ./ffx-linux-x64 platform preflight
  ```

* {macOS}

  ```posix-terminal
  curl -sO https://storage.googleapis.com/fuchsia-ffx/ffx-macos-x64 && chmod +x ffx-macos-x64 && ./ffx-macos-x64 platform preflight
  ```
<!--
## 2. Install prerequisite packages {#install-prerequisite-packages}
-->
## 2. 安装用到的软件包 {#install-prerequisite-packages}
<!--
Fuchsia requires `curl`, `file`, `unzip`, and `git` to be up to date. The version
of `git` needs to be 2.28 or higher.
-->
Fuchsia 要求 `curl`、 `file`、 `unzip` 以及 `git` 等工具是最新版。 `git` 版本要求 2.28 或以上。

* {Linux}
<!--
  Install (or update) the following packages:
-->
  安装（或更新）下面的软件：

  ```posix-terminal
  sudo apt install curl file git unzip
  ```

* {macOS}
<!--
  Install the Xcode command line tools:

  Note: Skip this step if `ffx platform preflight` shows that Xcode tools
  are already installed on your machine.
-->
  安装 Xcode 命令行工具：

  注意：如果 `ffx platform preflight` 工具检测到 Xcode 已经安装，就省去这一步操作。

  ```posix-terminal
  xcode-select --install
  ```
<!--
## 3. Download the Fuchsia source code {#download-the-fuchsia-source-code}
-->
## 3. 下载 Fuchsia 源码 {#download-the-fuchsia-source-code}
<!--
Fuchsia provides a [bootstrap script](/scripts/bootstrap) that creates a
directory named `fuchsia` and downloads the Fuchsia source code in that
directory.
-->
Fuchsia 提供了一个[引导脚本](/scripts/bootstrap)，这个脚本会创建一个 `fuchsia` 的文件夹并把 Fuchsia 源码下载到这里。
<!--
Downloading the Fuchsia source code requires about 2 GB of space
on your machine. Depending on your build configuration, you need
another 80 to 90 GB of space later when you build Fuchsia. Additionally,
the download process uses a substantial amount of memory. It is advisible
to close non-crucial processes during this time.
-->
下载 Fuchsia 源码要求你机器上有 2 GB 的存储空间。之后在编译 Fuchsia 过程中需要另外 80 到 90 GB 的存储空间，这取决于你的编译选项配置。另外，下载过程中也会使用一部分的内存空间。建议您在此过程中关闭其他非必要的程序进程。
<!--
To download the Fuchsia source, do the following:
-->
下载 Fuchsia 源码，请按如下步骤：
<!--
1.  Select a directory for downloading the Fuchsia source code, for example:

    Note: You can set up Fuchsia in any directory. This guide selects
    the `$HOME` directory as an example.
-->
1.  选择一个 Fuchsia 源码存储目录，例如：
    
    注意：你可以把 Fuchsia 下载到任何路径。本篇教程选择 `$HOME` 作为示例。

    ```posix-terminal
    cd $HOME
    ```
<!--
1.  Run the bootstrap script:

    Note: Downloading the Fuchsia source code can take up to 60 minutes.
-->
1.  运行引导脚本：
    
    注意：下载 Fuchsia 源码可能需要 60 分钟。

    ```posix-terminal
    curl -s "https://fuchsia.googlesource.com/fuchsia/+/HEAD/scripts/bootstrap?format=TEXT" | base64 --decode | bash
    ```
<!--
    This script creates the `fuchsia` directory and downloads the source code.

    If you see the `Invalid authentication credentials` error during the
    bootstrapping process, see [Authentication error](#authentication-error) for
    help.
-->
    这个脚本会先创建 `fuchsia` 路径，然后下载源码到该路径下。

    如果你在脚本运行期间看到了 `Invalid authentication credentials` 错误信息，参考[认证失败](#authentication-error)章节寻求帮助。


<!--
## 4. Set up environment variables {#set-up-environment-variables}
-->
## 4. 设置环境变量 {#set-up-environment-variables}
<!--
Fuchsia recommends that you update your shell profile to include the following
actions:
-->
Fuchsia 建议您在 shell 的配置文件中添加如下几个配置：
<!--
*   Add the `.jiri_root/bin` directory to your `PATH`.

    The `.jiri_root/bin` directory in the Fuchsia source contains the
    [`jiri`](https://fuchsia.googlesource.com/jiri) and
    [`fx`](/development/build/fx.md) tools that are essential to
    Fuchsia workflows. Fuchsia uses the `jiri` tool to manage repositories in
    the Fuchsia project, and the `fx` tool helps configure, build, run, and
    debug Fuchsia. The Fuchsia toolchain requires that `jiri` is available in
    your `PATH`.
-->
*  添加 `.jiri_root/bin` 路径到你的 `PATH` 环境变量
    
   `.jiri_root/bin` 路径在 Fuchsia 源码中包含了 [`jiri`](https://fuchsia.googlesource.com/jiri) 和 [`fx`](/development/build/fx.md) 工具，这些工具对 Fuchsia 的后续操作起着至关重要的作用。Fuchsia 使用 `jiri` 工具在 Fuchsia 的项目中管理仓库，使用 `fx` 工具帮助配置、编译、运行以及调试 Fuchsia。Fuchsia 的工具链需要可以在你 `PATH` 环境变量中找得到 `jiri` 工具。
<!--
*   Source the `scripts/fx-env.sh` file.

    Though it's not required, sourcing the
    [`fx-env.sh`](/scripts/fx-env.sh) script enables a number of
    useful shell functions in your terminal. For instance, it creates the
    `FUCHSIA_DIR` environment variable and provides the `fd` command for
    navigating directories with auto-completion. (For more information, see
    comments in `fx-env.sh`.)
-->
*   导入 `scripts/fx-env.sh` 文件
    
    虽然这并不是必须的，导入 [`fx-env.sh`](/scripts/fx-env.sh) 文件可以在你的终端中建立一系列的非常有用的 shell 函数。比如，它会创建给 `fd` 工具用来切换路径时提供自动补全的 `FUCHSIA_DIR` 环境变量（如果想获取更多信息，请参考 `fx-env.sh` 文件）。

<!--
Note: If you don't wish to update your shell profile, see
[Work on Fuchsia without updating your PATH](#work-on-fuchsia-without-updating-your-path)
in Appendices instead.
-->
注意：如果你不想更新你的 shell 配置，在附录中参考[不更新 PATH 变量编译 Fuchsia](#work-on-fuchsia-without-updating-your-path)。
<!--
To update your shell profile to configure Fuchsia's environment variables,
do the following:
-->
为了更新你的 shell 配置文件来设置 Fuchsia 的环境变量，请按如下步骤：
<!--
1.  Use a text editor to open your `~/.bash_profile` file (in the example below,
    we use the [Nano][nano]{:.external} text editor):

    Note: This guide uses a `bash` terminal as an example. If you're
    using `zsh`, replace `~/.bash_profile` with `~/.zprofile` in the
    following steps:
-->
1.  请用文本编辑器打开你的 `~/.bash_profile` 文件（参考如下示例，我们使用 [Nano][nano]{:.external} ）

    注意：这篇文章使用 `bash` 终端作为示例，如果你使用 `zsh` 的话，在如下步骤中请替换文件 `~/.bash_profile` 为 `~/.zprofile`：

    ```posix-terminal
    nano ~/.bash_profile
    ```
<!--
1.  Add the following lines to your `~/.bash_profile` file:

    Note: If your Fuchsia source code is not located in the `~/fuchsia`
    directory, replace `~/fuchsia` with your Fuchsia directory.
-->
1.  在你的 `~/.bash_profile` 文件中添加如下配置：

    注意：如果你的 Fuchsia 源码不在 `~/fuchsia` 路径下，替换 `~/fuchsia` 路径为你的 Fuchsia 路径。

    ```sh
    export PATH=~/fuchsia/.jiri_root/bin:$PATH
    source ~/fuchsia/scripts/fx-env.sh
    ```
<!--
1.  Save the file and exit the text editor.
-->
1.  保存文件并退出。
<!--
1.  To update your environment variables, run the following command:
-->
1.  更新环境变量，请运行如下命令：

    ```posix-terminal
    source ~/.bash_profile
    ```
<!--
1.  Verify that you can run the following commands inside your
    `fuchsia` directory without error:
-->
1.  验证你是否可以运行如下命令且没有报错：

    ```posix-terminal
    jiri help
    ```

    ```posix-terminal
    fx help
    ```
<!--
## 5. Configure firewall rules (Optional) {#configure-firewall-rules}
-->
## 5. 配置防火墙（可选项） {#configure-firewall-rules}
<!--
Note: This step is not required for building or running Fuchsia. But it is
recommended to ensure that Fuchsia's emulator instances run smoothly on Linux.

(**Linux only**) If you're planning on running Fuchsia on Linux, it is advised to
run the following command to allow Fuchsia-specific traffic on the host machine:
-->
注意：这一步在你的编译或者运行 Fuchsia 步骤中并不是必须的。但是是被建议的，因为可以用来确保你能在 Linux 中流畅的运行 Fuchsia 虚拟机。

(**Linux**) 如果你计划在 Linux 中运行 Fuchsia，建议您运行如下命令来允许 Fuchsia 使用一些特殊的特性：

```posix-terminal
fx setup-ufw
```
<!--
This script requires `sudo` (which asks for your password) to set the appropriate
firewall rules. (For more information on this script, see [`setup-ufw`][setup-ufw].)
-->
这个脚本需要 `sudo` （会提示你输入密码）权限来设置防火墙规则。（更多关于脚本的的信息，参考 [`setup-ufw`][setup-ufw] 文件）。
<!--
## Next steps
-->
## 下一步

<!--
To build your first Fuchsia system image, see
[Configure and build Fuchsia](/get-started/build_fuchsia.md).
-->
编译你第一个 Fuchsia 系统镜像，参考[配置和编译 Fuchsia](/get-started/build_fuchsia.md) 文件。
<!--
## Appendices
-->
## 附录
<!--
### Authentication error {#authentication-error}
-->
### 验证失败 {#authentication-error}
<!--
If you see the `Invalid authentication credentials` error during the bootstrap
process, your `~/.gitcookies` file may contain cookies from some repositories in
`googlesource.com` that the bootstrap script wants to check out anonymously.
-->
如果你在脚本运行期间看到了 `Invalid authentication credentials` 报错信息，你的 `~/.gitcookies` 文件中可能包含了一些引导脚本想要自动检测的来自 `googlesource.com` 的 cookie 信息。
<!--
To resolve this error, do one of the following:
-->
为了解决这个问题，使用以下任何一个方式：

<!--
*   Follow the onscreen directions to get passwords for the specified
    repositories.
*   Delete the offending cookies from the `.gitcookies` file.
-->

*  按照屏幕路径的提示为仓库输入密码
*  删除 `.gitcookies` 文件中不需要的 cookie。

<!--
### Work on Fuchsia without updating your PATH {#work-on-fuchsia-without-updating-your-path}
-->
###  不更新 PATH 变量编译 Fuchsia {#work-on-fuchsia-without-updating-your-path}
<!--
The following sections provide alternative approaches to the
[Set up environment variables](#set-up-environment-variables) section:
-->
下面的章节为[设置环境变量](#set-up-environment-variables)章节提供了替代选项：
<!--
*   [Copy the tool to your binary directory](#copy-the-tool-to-your-binary-directory)
*   [Add a symlink to your binary directory](#add-a-symlink-to-your-binary-directory)
-->
*   [把工具复制到可执行文件目录](#copy-the-tool-to-your-binary-directory)
*   [为工具添加链接文件到可执行文件目录](#add-a-symlink-to-your-binary-directory)

<!--
#### Copy the tool to your binary directory {#copy-the-tool-to-your-binary-directory}
-->
####  把工具复制到可执行文件目录 {#copy-the-tool-to-your-binary-directory}
<!--
If you don't wish to update your environment variables, but you want `jiri` to
work in any directory, copy the `jiri` tool to your `~/bin` directory, for
example:

Note: If your Fuchsia source code is not located in the `~/fuchsia` directory,
replace `~/fuchsia` with your Fuchsia directory.
-->
如果你不想更新你的环境变量，但是你想在任何目录中使用 `jiri` 工具，把 `jiri` 工具复制到 `~/bin` 目录中，比如：

注意：如果你的 Fuchsia 源码不在 `~/fuchsia` 目录中，把 `~/fuchsia` 替换为你的 Fuchsia 目录。

```posix-terminal
cp ~/fuchsia/.jiri_root/bin/jiri ~/bin
```

<!--
However, you must have write access to the `~/bin` directory without `sudo`. If
you don't, `jiri` cannot keep itself up-to-date.
-->
无论如何，你首先需要有不用 `sudo` 命令就可以访问 `~/bin` 路径的权限。如果没有，`jiri` 就不能使它自己保持在最新版本。
<!--
#### Add a symlink to your binary directory {#add-a-symlink-to-your-binary-directory}
-->
#### 为工具添加链接文件到可执行文件目录 {#add-a-symlink-to-your-binary-directory}
<!--
Similarly, if you want to use the `fx` tool without updating your environment
variables, provide the `fx` tool's symlink in your `~/bin` directory, for
example:

Note: If your Fuchsia source code is not located in the `~/fuchsia` directory,
replace `~/fuchsia` with your Fuchsia directory.
-->
同样的，如果你想不更新环境变量使用 `fx` 工具，在 `~/bin` 路径中提供你的 `fx` 工具的链接文件，比如：

注意：如果你的 Fuchsia 源码不在 `~/fuchsia` 目录中，把 `~/fuchsia` 替换为你的 Fuchsia 目录。

```posix-terminal
ln -s ~/fuchsia/scripts/fx ~/bin
```
<!--
Alternatively, run the `fx` tool directly using its path, for example:
-->
相应地，你也可以使用 `fx` 工具的完整路径，比如：

```posix-terminal
./scripts/fx help
```
<!--
In either case, you need `jiri` in your `PATH`.
-->
另一方面，您需要在 `PATH` 环境变零中设置你的 `jiri` 工具。
<!-- Reference links -->

[ffx-platform-preflight]: https://fuchsia.dev/reference/tools/sdk/ffx#preflight
[nano]: https://www.nano-editor.org/docs.php
[setup-ufw]: https://fuchsia.dev/reference/tools/fx/cmd/setup-ufw
