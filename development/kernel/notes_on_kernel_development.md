# <!-- Notes on kernel development -->

# 内核开发说明



## <!-- Low level kernel development -->

## 内核底层开发

<!-- For kernel development it's not uncommon to need to monitor or break things
before the gfxconsole comes up. -->

对于内核开发，在图形控制台出现之前监控或者破坏事情的情况是常见的。

<!-- To force-enable log output to the legacy serial console on an x64 machine, pass
"kernel.serial=legacy".  For other serial configurations, see the kernel.serial
docs in [kernel_cmdline.md](/docs/reference/kernel/kernel_cmdline.md). -->

在 x64 机器上通过参数 "kernel.serial=legacy" 强制日志输出到串行控制台。对于其他的串口配置哦，请查看 [内核命令](/reference/kernel/kernel_cmdline.md)

<!-- To enable the early console before the graphical console comes up use the
``gfxconsole.early`` cmdline option. More information can be found in
[kernel_cmdline.md](/docs/reference/kernel/kernel_cmdline.md). -->

在图形控制台出现前，使用 `gfxconsole.early` 命令选项，开启早期控制。更多的信息查看[内核命令](/reference/kernel/kernel_cmdline.md)

<!-- Enabling ``startup.keep-log-visible``will ensure that the kernel log stays
visible if the gfxconsole comes up after boot. To disable the gfxconsole
entirely you can disable the video driver it is binding to via ``driver.<driver
name>.disable``. -->

如果启动后图形控制台出现，当开启 `startup.keep-log-visible` 会使内核日志一直保持显示。要取消图形控制台，你可以取消视频驱动，它是通过 `driver.<drivername>.disable` 绑定的。

<!-- On a skylake system, all these options together would look something like: -->

在天湖（skylake，英特尔的芯片架构）系统上，全部的选项类似下面：

```
$ tools/build-x86/bootserver build-x86/zircon.bin -- gfxconsole.early driver.intel-i915-display.disable
```

<!-- To directly output to the console rather than buffering it (useful in the event
of kernel freezes) you can enable ``ENABLE_KERNEL_LL_DEBUG`` in your build like so: -->

构建的参数类似下面的，开启 `ENABLE_KERNEL_LL_DEBUG` 直接输出到控制台而不是缓存它（用于内核的冻结事件）

```
fx set ... --args='kernel_extra_defines=["ENABLE_KERNEL_LL_DEBUG"]'

```

<!-- There is also a kernel cmdline parameter kernel.bypass-debuglog, which can be set
to true to force output to the console instead of buffering it. The reason we have
both a compile switch and a cmdline parameter is to facilitate prints in the kernel
before cmdline is parsed to be forced to go to the console. The compile switch setting
overrides the cmdline parameter (if both are present). Note that both the compile switch
and the cmdline parameter have the side effect of disabling irq driven uart Tx. -->

这里有内核命令参数 `kernel.bypass-debuglog`，它可以设置 true 强制输出到控制台而不是缓冲它。我们同时使用编译开关和命令参数在命令行被强制解析到控制台之前，便于打印到内核。编译开关设置会覆盖命令行参数（如果两者都存在）。注意，编译开关和命令行参数都有禁用中断请求驱动uart传输的副作用。

## <!-- Changing the compiler optimization level of a module -->

## 更改编译器模块优化

<!-- You can override the default `-On` level for a module by defining in its
build arguments: -->

你可以通过定义构建参数模块来覆盖默认的 `-On`

```
opt_level := <n>
```

## <!-- Requesting a backtrace -->

## 请求回溯

### <!-- From within a user process -->

### 从用户进程内

<!-- For debugging purposes, the system crashlogger can print backtraces by
request. It requires modifying your source, but in the absence of a
debugger, or as a general builtin debug mechanism, this can be useful. -->

为了调试的目的，系统故障日志可以通过请求回溯打印。它需要修改你的代码，但在缺少调试器或者平常的内部调试机制下，是有用的。

```
#include <lib/backtrace-request/backtrace-request.h>

void my_function() {
  backtrace_request();
}
```

<!-- When `backtrace\_request` is called, it causes an
exception used by debuggers for breakpoint handling.
If a debugger is not attached, the system crashlogger will
process the exception, print a backtrace, and then resume the thread. -->

当 `backtrace\_request` 被调用，为了断点处理，它导致了一个调试器的异常使用。

### <!-- From a kernel thread -->

### 来自内核线程

```
#include <kernel/thread.h>

void my_function() {
  thread_print_backtrace(get_current_thread(), __GET_FRAME(0));
}
```

## <!-- Exporting debug data during boot -->

## 在启动期输出调试数据

<!-- To support testing the system during early boot, there is a mechanism to export
data files from the kernel to the /boot filesystem. To export a data file,
create a VMO, give it a name, and pass it to userboot with handle\_info of type
PA\_VMO\_DEBUG\_FILE (and argument 0). Then userboot will automatically pass it
through to devmgr, and devmgr will export the VMO as a file at the path -->

为了在早期启动支持测试系统，这里有个机制是从内核输出数据文件到 /boot 文件夹的。为了输出数据文件，创建一个 VMO（虚拟内存对象），获取它的名字，使用 `PA\_VMO\_DEBUG\_FILE` 类型的句柄 `handle\_info` 传递它到 `userboot` 。

```
/boot/kernel/<name-of-vmo>
```

<!-- This mechanism is used by the entropy collector quality tests to export
relatively large (~1 Mbit) files full of random data. -->

熵收集器质量测试使用该机制输出相对较大的随机数据文件（约1Mbit）。
