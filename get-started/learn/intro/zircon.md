<!-- # Zircon fundamentals -->
# Zircon 基础知识

<<../../_common/intro/_zircon_intro.md>>

<<../../_common/intro/_zircon_syscall.md>>

<<../../_common/intro/_zircon_processes.md>>

<<../../_common/intro/_zircon_ipc.md>>

<!-- ## Exercise: Jobs and processes -->
## 练习：作业（job）与进程（process）

<!-- 
Let's explore some of these fundamental concepts on a running system. In
this exercise, you'll see how jobs and processes interact to form a tree.
 -->
让我们在运行中的系统上探索这些基础概念。
在本练习中，您将看到作业与进程间是如何交互以形成一棵树的。

<<../_common/_start_femu.md>>

<!-- ### Dump the process list -->
### 打印进程列表

<!-- 
Connect to a device shell prompt and use the `ps` command to dump the list of
running jobs and processes.
 -->
连接到设备命令行，并使用 `ps` 命令打印运行中的作业和进程的列表。

```posix-terminal
fx shell ps
```

<!-- Below is a trimmed example of what the output looks like: -->
下面是一个截断的输出样例：

```none {:.devsite-disable-click-to-copy}
TASK                     PSS PRIVATE  SHARED   STATE NAME
j: 1027               507.8M  507.4M                 root
  p: 1061             564.4k    564k     36k         bin/bootsvc
  p: 1150            4264.4k   4264k     36k         bin/component_manager
  j: 1479             228.4k    228k
    p: 1583           228.4k    228k     36k         pwrbtn-monitor.cm
  j: 1484             532.4k    532k
    p: 1599           532.4k    532k     36k         svchost.cm
  j: 1544             402.4k    304k
    p: 1633           402.4k    304k    232k         netsvc.cm
  j: 1681             296.4k    296k
    p: 1733           296.4k    296k     36k         console-launcher.cm
  j: 1799            7232.4k   7232k
    p: 1825          7232.4k   7232k     36k         archivist.cm
  j: 1927             660.4k    660k
    p: 1955           660.4k    660k     36k         base-resolver.cm
  j: 2072            1016.4k   1016k
    p: 2088          1016.4k   1016k     36k         driver_manager.cm
  j: 2239             348.4k    348k
    p: 2252           348.4k    348k     36k         device-name-provider.cm
  j: 2364             275.3M  275.3M
    p: 2380          1012.4k   1012k     36k         fshost.cm
    p: 6544           252.1M  252.1M     36k         /pkg/bin/blobfs
    p: 10205         9744.4k   9744k     36k         /pkg/bin/minfs
    p: 10475           12.8M   12.8M     36k         pkgfs
```


<!-- Let's focus on two columns in the output for now: -->
我们暂时关注其中两列：

<!-- 
* **TASK**: This tells you whether each entry is a job (`j`) or process (`p`)
  followed by their unique id.
* **NAME**: This provides a little more detail about what piece of the system
  is running there.
 -->
* **TASK**（任务）：这一列告诉您每个条目是作业（`j`）还是进程（`p`），其后紧接着它的唯一 id。
* **NAME**（名称）：这一列提供了稍多的详细信息，告诉您正在运行的是系统的哪一部分。

<!-- 
Let's break down some interesting things here based on what we've discussed so
far:
 -->
基于之前讨论的内容，我们可以发现一些有趣的事情：

<!-- 
1. Every process is connected to a parent job. Some jobs have multiple
   processes.
1. All jobs trace back to the `root` job as the ultimate parent, forming a tree.
1. During startup, the system launches a few processes directly into the `root` job.
   Most other processes are launched under their own parent jobs.
1. After the initial startup work, many of the entries have a `.cm` extension. These
   refer to **components**, and you will learn more about them later on.
1. Some of these components are core services like filesystems (`fshost.cm`) and
   drivers (`driver_manager.cm`) that live in user space separate from the
   kernel.
 -->
1. 每个进程都连接到一个父作业。有些作业有多个进程。
1. 所有作业都能追溯到 `root` 这个终极作业，形成了一棵树。
1. 在系统启动时，有一些进程直接在 `root` 作业中启动。大多数其他进程会在它们自己的父作业中启动。
1. 在系统启动的初始阶段之后，有许多条目带有 `.cm` 的扩展名。它们指的是 **组件**（component）。
   您将在稍后了解有关组件的更多知识。
1. 有些组件是核心服务，它们独立于内核而存在于用户空间，如文件系统（`fshost.cm`）和驱动（`driver_manager.cm`）。

<!-- 
Next, we'll explore how the Zircon enables the fundamentals of Fuchsia's
security model.
 -->
接下来，我们将探索 Zircon 如何奠定了 Fuchsia 安全模型的基础。

<aside class="key-point">
  <!-- <b>Extra Credit: Fuchsia Startup</b> -->
  <b>附加题：Fuchsia 启动</b>
<!-- 
  <p>Exploring the tree of running processes is also a great way to learn about
  the startup process of a Fuchsia device. Take a moment to review
  <a href="concepts/process/everything_between_power_on_and_your_component.md">
  device startup</a> and map how the initial processes align with the
  <code>ps</code> output on the emulator.</p>
   -->
  <p>探索运行中的进程树也是了解 Fuchsia 设备启动过程的好方法。
  请稍微花些时间回顾一下<a href="/concepts/process/everything_between_power_on_and_your_component.md">
  设备启动</a>，并比较初始启动过程与模拟器上 <code>ps</code> 的输出是否相符。</p>
</aside>
