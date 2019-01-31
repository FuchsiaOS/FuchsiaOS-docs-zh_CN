<!--Multi Device Setup-->
多设备设置
============

<!--This guide will walk you through the process of getting two Fuchsia devices
set up and synchronizing story state using the
[Ledger](https://fuchsia.googlesource.com/peridot/+/master/docs/ledger/).-->

<!--https://9to5google.com/2018/01/26/google-fuchsia-os-stories-and-modules/ 这里介绍了stories A story is one or more modules from different apps (or the same app) combining together to make one task or complete thought, whether they were originally designed to do so or not. story就是一个或多个模块的集合，在Fuchsia中没有最近启动的应用但是有最近启动的story-->
<!--Ledger是一个针对 Fuchsia 的分布式存储系统。-->

这篇文章将会帮助你完成两台 Fuchsia 设备通过 [Ledger](https://fuchsia.googlesource.com/peridot/+/master/docs/ledger/) 设置和同步 story 状态。

<!--## Setup-->
## 设置

<!--### Devices-->
### 设备

<!--Follow the steps at in the [top level docs](../README.md) to:-->
<!--* Check out the source and build Fuchsia.-->
<!--* Install it on two devices (or emulators).-->
<!--* Connect the devices to the network.-->
按照[目录顶级文档](../README.md)的操作步骤：

* 拉取资源，构建 Fuchsia。
* 在两台设备或者模拟器中安装 Fuchsia。
* 将两台设备都连接上网络。

<!--### [Googlers only] Private Test Network Setup-->
<!--Follow netboot instructions.-->
### [谷歌人员专用] 专用测试网络设置
遵循网络引导说明。


<!--### Identify Test Machines-->
### 识别测试机

<!--Each Fuchsia device has a unique node name based on its MAC address.  It is of
the form `power-nerd-saved-santa`.  You can list the nodes on your network with
the `netls` command.-->
每个 Fuchsia 设备都会根据 MAC 地址生成独一无二的节点名，格式为 `power-nerd-saved-santa`。你可以使用 `netls` 命令列举出网络中的节点。

```
> netls
    device glad-bats-hunk-lady (fe80::f64d:30ff:fe68:2620/6)
    device blurt-chip-sugar-wish (fe80::8eae:4cff:feee:4f40/6)
```

<!--### Running Commands On Test Machines-->
### 在测试机器上运行命令

<!--The `netruncmd <nodename> <command>` command can be used to run commands on
remote machines.  The output of the command is not shown.  If you need to see
the output, use the `loglistener [<nodename>]` command.-->
`netruncmd <nodename> <command>` 命令可以用来在远程设备上运行命令，命令输出是不会显示的。如果你需要查看输出，需要使用 `loglistener [<nodename>]` 命令。

<!--### Ledger Setup-->
### Ledger 设置

<!--Ledger is a distributed storage system for Fuchsia.  Stories use it to
synchronize their state across multiple devices.  Follow the steps in Ledger's
[User Guide](https://fuchsia.googlesource.com/peridot/+/master/docs/ledger/user_guide.md)
to:-->
Ledger 是一个针对 Fuchsia 的分布式存储系统。Stories 可以使用 Ledger 在多个设备间同步状态。按照 Ledger 的[用户指南](https://fuchsia.googlesource.com/peridot/+/master/docs/ledger/user_guide.md)：

<!--* Set up [persistent storage](https://fuchsia.googlesource.com/zircon/+/master/docs/minfs.md). (optional)-->
<!--* Verify the network is connected.-->
<!--* Configure a Firebase instance.-->
<!--* Setup sync on each device using `configure_ledger`.-->
* 设置[硬盘格式](https://fuchsia.googlesource.com/zircon/+/master/docs/minfs.md)。(可选)
* 确认网络连接无误。
* 配置 Firebase 实例。
* 使用 `configure_ledger` 在每个设备上设置同步。

<!--## Run Stories-->
## 运行 Story

<!--### Virtual consoles.-->
### 虚拟控制台

<!--The systems boots up with three virtual consoles.  Alt-F1 through
Alt-F3 can be used to switch between virtual consoles.-->
这些系统通过三个虚拟控制台启动。Alt+F1 到 Alt+F3 可以用来切换三个虚拟控制台。

<!--### Wipe Data-->
### 清除数据

<!--The format of the Ledger as well as the format of the data each story syncs is
under rapid development and no effort is currently made towards forwards and
backwards compatibility.  Because of this, after updating the Fuchsia code, it
is a good idea to wipe your remote and local data using `cloud_sync clean`.-->
Ledger 的格式和数据格式一样， story 同步也在快速开发中，目前还没有实现向前或向后的兼容。因此最好每次升级 Fuchsia 时，使用 `cloud_sync clean` 清除远程设备和本地设备的数据。

```
$ netruncmd <nodename> cloud_sync clean
```

<!--### Start A Story On One Device-->
### 在一台设备上启动一个 Story

<!--Use the `basemgr` to start a story on one device:-->
使用 `basemgr` 在设备上启动一个 story

```
$ netruncmd <first-node-name> "basemgr --session_shell=dev_session_shell \
  --session_shell_args=--root_module=example_todo_story"
```

<!--Using `loglistener <first-node-name>` take note of the story ID from a line the
following:-->
使用 `loglistener <first-node-name>` 查看 story 的 ID：

```
... DevSessionShell Starting story with id: IM7U9hBcCt
```

<!--### Open The Same Story On The Second Device.-->
### 在第二台设备上打开同一个 Story

<!--The story can be started on the second device either through the system UI or by
specifying the story ID.-->
story 可以通过 system UI 或者指定的 story ID 启动。

<!--#### System UI, aka Session Shell, aka Armadillo-->
#### System UI 又叫 Session Shell 还称作 Armadillo
<!--Launch the system UI using `basemgr`:-->
使用 `basemgr` 启动 system UI：

```
$ netruncmd <second-node-name> "basemgr"
```

<!--Once the system UI starts, you should be able to see the story started in the
step above.  Click on that to open it.-->
一旦 system UI 启动，您应该能够在上面步骤中看到已经启动的 story。点击打开它。

<!--#### By Story ID-->
#### 通过 Story ID 启动

<!--With the story ID noted above from launch the story from a shell:-->
在 shell 中 使用上边提到的 story ID 来启动一个 story：

```
$ netruncmd <second-node-name> "basemgr \
  --session_shell=dev_session_shell \
  --session_shell_args=--story_id=<story_id>
```
