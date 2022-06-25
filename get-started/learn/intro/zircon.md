# Zircon fundamentals

<<../../_common/intro/_zircon_intro.md>>

<<../../_common/intro/_zircon_syscall.md>>

<<../../_common/intro/_zircon_processes.md>>

<<../../_common/intro/_zircon_ipc.md>>

## Exercise: Jobs and processes

Let's explore some of these fundamental concepts on a running system. In
this exercise, you'll see how jobs and processes interact to form a tree.

<<../_common/_start_femu.md>>

### Dump the process list

Connect to a device shell prompt and use the `ps` command to dump the list of
running jobs and processes.

```posix-terminal
fx shell ps
```

Below is a trimmed example of what the output looks like:

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


Let's focus on two columns in the output for now:

* **TASK**: This tells you whether each entry is a job (`j`) or process (`p`)
  followed by their unique id.
* **NAME**: This provides a little more detail about what piece of the system
  is running there.

Let's break down some interesting things here based on what we've discussed so
far:

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

Next, we'll explore how the Zircon enables the fundamentals of Fuchsia's
security model.

<aside class="key-point">
  <b>Extra Credit: Fuchsia Startup</b>
  <p>Exploring the tree of running processes is also a great way to learn about
  the startup process of a Fuchsia device. Take a moment to review
  <a href="/docs/concepts/process/everything_between_power_on_and_your_component.md">
  device startup</a> and map how the initial processes align with the
  <code>ps</code> output on the emulator.</p>
</aside>
