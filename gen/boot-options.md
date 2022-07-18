<!-- 
# Zircon Kernel Commandline Options

TODO([fxbug.dev/53594](https://fxbug.dev/53594)): move kernel_cmdline.md verbiage here 
-->

# Zircon 内核命令行选项

TODO([fxbug.dev/53594](https://fxbug.dev/53594))：移动 kernel_cmdline.md 到当前目录。

<!-- 
## Options common to all machines

### aslr.disable=\<bool>
**Default:** `false`

If this option is set, the system will not use Address Space Layout
Randomization. 
-->

## 使用所有机器的通用选项

### aslr.disable=\<bool>
**默认值:** `false`

该选项用来禁用ASLR（地址空间配置随机加载）。

<!-- 

### aslr.entropy_bits=\<uint8_t>
**Default:** `0x1e`

For address spaces that use ASLR this controls the number of bits of entropy in
the randomization. Higher entropy results in a sparser address space and uses
more memory for page tables. Valid values range from 0-36.
-->

### aslr.entropy_bits=\<uint8_t>
**默认值：** `0x1e`

对于使用ASLR的地址空间，这控制了随机化过程中熵的位数。更高的熵意味着更松散的地址空间，同时分页表也会占用更多的内存。有效的参数值范围是：0至36。

<!-- 
### kernel.cprng-reseed-require.hw-rng=\<bool>
**Default:** `false`

When enabled and if HW RNG fails at reseeding, CPRNG panics.
 -->

### kernel.cprng-reseed-require.hw-rng=\<bool>
**默认值：** `false`

此选项启用后，当HW RNG（硬件随机数生成器）在补种失败时，CPRNG（密码伪随机数生成器）返回错误。

<!-- 
### kernel.cprng-reseed-require.jitterentropy=\<bool>
**Default:** `false`

When enabled and if jitterentropy fails at reseeding, CPRNG panics.
 -->

### kernel.cprng-reseed-require.jitterentropy=\<bool>
**默认值：** `false`

此选项启用后，当 jitterentropy 补种失败时，CPRNG 返回错误。

<!-- 
### kernel.cprng-seed-require.hw-rng=\<bool>
**Default:** `false`

When enabled and if HW RNG fails at initial seeding, CPRNG panics.
 -->

### kernel.cprng-seed-require.hw-rng=\<bool>
**默认值：** `false`

此选项启用后，当 HW RNG 初始化种子失败时，CPRNG 返回错误。

<!-- 
### kernel.cprng-seed-require.jitterentropy=\<bool>
**Default:** `false`

When enabled and if jitterentrop fails initial seeding, CPRNG panics.
 -->

### kernel.cprng-seed-require.jitterentropy=\<bool>
**默认值：** `false`

此选项启用后，当 jitterentrop 在初始化种子失败时，CPRNG 返回错误。

<!-- 
### kernel.cprng-seed-require.cmdline=\<bool>
**Default:** `false`

When enabled and if you do not provide entropy input from the kernel command
line, CPRNG panics.
-->

### kernel.cprng-seed-require.cmdline=\<bool>
**默认值：** `false`

此选项启用后，当命令行参数未提供熵的输入时，CPRNG 返回错误

<!-- 
### kernel.entropy-mixin=\<hexadecimal>

Provides entropy to be mixed into the kernel's CPRNG.  The value must be a
string of lowercase hexadecimal digits.

The original value will be scrubbed from memory as soon as possible and will be
redacted from all diagnostic output.
-->

### kernel.entropy-mixin=\<hexadecimal>

该选项提供熵用以混入内核 CPRNG，选项值必须为小写十六进制数字的字符串。

原始值将尽快从内存中清除，并将从所有诊断输出中删除。

<!-- 
### kernel.jitterentropy.bs=\<uint32_t>
**Default:** `0x40`

Sets the "memory block size" parameter for jitterentropy. When jitterentropy is
performing memory operations (to increase variation in CPU timing), the memory
will be accessed in blocks of this size.
-->

### kernel.jitterentropy.bs=\<uint32_t>
**默认值：** `0x40`

为 jitterentropy 设置"memory block size"参数。当 jitterentropy 执行内存操作时（以增加CPU时序的差异），在此范围内的内存区块是可以被 jitterentropy 存取的。

<!-- 
### kernel.jitterentropy.bc=\<uint32_t>
**Default:** `0x200`

Sets the "memory block count" parameter for jitterentropy. When jitterentropy
is performing memory operations (to increase variation in CPU timing), this
controls how many blocks (of size `kernel.jitterentropy.bs`) are accessed.
-->

### kernel.jitterentropy.bc=\<uint32_t>
**默认值：** `0x200`

为 jitterentropy 设置"memory block count"参数。当 jitterentropy 执行内存操作时（以增加CPU时序的差异），此选项控制可以被 jitterentropy 存取的内存区块数量（`kernel.jitterentropy.bs` 定义的内存范围内）。

<!-- 
### kernel.jitterentropy.ml=\<uint32_t>
**Default:** `0x20`

Sets the "memory loops" parameter for jitterentropy. When jitterentropy is
performing memory operations (to increase variation in CPU timing), this
controls how many times the memory access routine is repeated. This parameter
is only used when `kernel.jitterentropy.raw` is true. If the value of this
parameter is `0` or if `kernel.jitterentropy.raw` is `false`, then
jitterentropy chooses the number of loops is a random-ish way.
-->

### kernel.jitterentropy.ml=\<uint32_t>
**默认值：** `0x20`

为 jitterentropy 设置"memory loops"参数。当 jitterentropy 执行内存操作时（以增加CPU时序的差异），此选项控制内存访问例程的重复次数。此选项仅当参数 `kernel.jitterentropy.raw` 为 `true` 时有效。如果此参数值为 `0` 或者 `kernel.jitterentropy.raw` 为 `false` ，jitterentropy 以随机的方式选择循环数。

<!-- 
### kernel.jitterentropy.ll=\<uint32_t>
**Default:** `0x1`

Sets the "LFSR loops" parameter for jitterentropy (the default is 1). When
jitterentropy is performing CPU-intensive LFSR operations (to increase variation
in CPU timing), this controls how many times the LFSR routine is repeated.  This
parameter is only used when `kernel.jitterentropy.raw` is true. If the value of
this parameter is `0` or if `kernel.jitterentropy.raw` is `false`, then
jitterentropy chooses the number of loops is a random-ish way.
-->

### kernel.jitterentropy.ll=\<uint32_t>
**默认值：** `0x1`

为 jitterentropy 设置 "LFSR loops" 参数（默认值为 1）。
Sets the "LFSR loops" parameter for jitterentropy (the default is 1). 当 jitterentropy 正在执行CPU密集型LFSR操作（以增加CPU时序的变化）时，此选项将控制LFSR例程重复次数。  此选项仅当 `kernel.jitterentropy.raw`  为 `true` 时有效。如果此参数值为 `0` 或者 `kernel.jitterentropy.raw` 为 `false` ，jitterentropy 以随机的方式选择循环数。

<!-- 
### kernel.jitterentropy.raw=\<bool>
**Default:** `true`

When true (the default), the jitterentropy entropy collector will return raw,
unprocessed samples. When false, the raw samples will be processed by
jitterentropy, producing output data that looks closer to uniformly random. Note
that even when set to false, the CPRNG will re-process the samples, so the
processing inside of jitterentropy is somewhat redundant.
-->

### kernel.jitterentropy.raw=\<bool>
**默认值：** `true`

当此选项值为 `true`（默认值） 时，熵控制器 jitterentropy 将会返回原始、未处理的样本。为 `false` 时，jitterentropy 会对原始样本进行处理，让输出数据看起来像是随机均匀的。值得注意的是，即使参数设为 `false` ，CPRNG 也会对样本进行再处理，所以，有时候 jitterentropy 内部的处理稍显多余。

<!-- 
### kernel.lockup-detector.critical-section-threshold-ms=\<uint64_t>
**Default:** `0xbb8`

When a CPU remains in a designated critical section for longer than
this threshold, a KERNEL OOPS will be emitted.

See also `k lockup status` and
[lockup detector](/zircon/kernel/lib/lockup_detector/README.md).

When 0, critical section lockup detection is disabled.

When kernel.lockup-detector.heartbeat-period-ms is 0, critical section lockup
detection is disabled.
-->

### kernel.lockup-detector.critical-section-threshold-ms=\<uint64_t>
**默认值：** `0xbb8`

当 CPU 在指定临界区停留时间超过此阈值时，会触发 `KERNEL OOPS` 事件。

更多信息，可以参见 `k lockup status`和[lockup detector](/zircon/kernel/lib/lockup_detector/README.md).

设置选项值设为0时，临界区锁定检测被禁用。

当选项 `kernel.lockup-detector.heartbeat-period-ms` 设置为 `0` 时，临界区锁定检测被禁用。

<!-- 
### kernel.lockup-detector.critical-section-fatal-threshold-ms=\<uint64_t>
**Default:** `0x2710`

When a CPU remains in a designated critical section for longer than this
threshold, a crashlog will be generated and the system will reboot, indicating a
reboot reason of `SOFTWARE_WATCHDOG` as it does.

See also `k lockup status` and
[lockup detector](/zircon/kernel/lib/lockup_detector/README.md).

When 0, critical section crashlog generation and reboot is disabled.

When kernel.lockup-detector.heartbeat-period-ms is 0, critical section lockup
detection is disabled.
-->

### kernel.lockup-detector.critical-section-fatal-threshold-ms=\<uint64_t>
**默认值：** `0x2710`

当 CPU 在指定临界区停留超过此阈值时，系统会生成崩溃日志并重启，可以在重启原因中，查找 `SOFTWARE_WATCHDOG` 来定位该问题。


更多信息，可以参见 `k lockup status` 和 [lockup detector](/zircon/kernel/lib/lockup_detector/README.md).

设置选项值设为0时，临界区锁定检测被禁用。

当选项 `kernel.lockup-detector.heartbeat-period-ms` 设置为 `0` 时，临界区锁定检测被禁用。

<!-- 
### kernel.lockup-detector.heartbeat-period-ms=\<uint64_t>
**Default:** `0x3e8`

How frequently a secondary CPU should emit a heartbeat via kernel timer.  This
value should be large enough to not impact system performance, but should be
smaller than the heartbeat age threshold.  1000 is a reasonable value.

See also [lockup detector](/zircon/kernel/lib/lockup_detector/README.md).

When 0, heartbeat detection is disabled.
-->

### kernel.lockup-detector.heartbeat-period-ms=\<uint64_t>
**Default:** `0x3e8`

该选项用来设置辅助CPU通过内核计时器发出心跳的频率。选项值要足够大，以免影响系统性能，同时要小于心跳周期的阈值。推荐使用 1000。

更多信息，可以参见 [lockup detector](/zircon/kernel/lib/lockup_detector/README.md)。

选项值为 0 时，心跳检测会被禁用。

<!-- 
### kernel.lockup-detector.heartbeat-age-threshold-ms=\<uint64_t>
**Default:** `0xbb8`

The maximum age of a secondary CPU's last heartbeat before it is considered to
be locked up.  This value should be larger than the heartbeat peroid, but small
enough so as to not miss short-lived lockup events.  3000 is a reasonable value.

See also [lockup detector](/zircon/kernel/lib/lockup_detector/README.md).

When 0, heartbeat detection is disabled.
-->

### kernel.lockup-detector.heartbeat-age-threshold-ms=\<uint64_t>
**默认值：** `0xbb8`

该选项指定了辅助 CPU 最后一次心跳的寿命阈值，超过该阈值，辅助 CPU 将被认定为已锁定。选项值应该大于心跳周期，同时足够小，以免错误短期锁定事件。 推荐使用 3000。

更多信息，可以参见 [lockup detector](/zircon/kernel/lib/lockup_detector/README.md)。

选项值为 0 时，心跳检测会被禁用。

<!-- 
### kernel.lockup-detector.heartbeat-age-fatal-threshold-ms=\<uint64_t>
**Default:** `0x2710`

The maximum age of a CPU's last heartbeat before it is considered to be locked
up, triggering generation of a crashlog indicating a reboot reason of
`SOFTWARE_WATCHDOG` followed by a reboot.

See also [lockup detector](/zircon/kernel/lib/lockup_detector/README.md).

When 0, heartbeat crashlog generation and reboot is disabled.
-->

### kernel.lockup-detector.heartbeat-age-fatal-threshold-ms=\<uint64_t>
**Default:** `0x2710`

该选项指定了主 CPU 最后一次心跳的寿命阈值，超过该阈值，CPU 将会被认定为已锁定，超时会触发一个在重启日志中标记为 `SOFTWARE_WATCHDOG` 的崩溃日志。

更多信息，可以参见 [lockup detector](/zircon/kernel/lib/lockup_detector/README.md)。

选项值为 0 时，心跳崩溃日志生成和重启会被禁用。

<!-- 
### kernel.oom.behavior=[reboot | jobkill]
**Default:** `reboot`

This option can be used to configure the behavior of the kernel when
encountering an out-of-memory (OOM) situation. Valid values are `jobkill`, and
`reboot`.

If set to `jobkill`, when encountering OOM, the kernel attempts to kill jobs that
have the `ZX_PROP_JOB_KILL_ON_OOM` bit set to recover memory.

If set to `reboot`, when encountering OOM, the kernel signals an out-of-memory
event (see `zx_system_get_event()`), delays briefly, and then reboots the system.
-->

### kernel.oom.behavior=[reboot | jobkill]
**默认值：** `reboot`

此选项用以配置 内存溢出（OOM）时，内核需要执行的操作。有效的选项值为：`jobkill`、
`reboot`。

当选项值设为 `jobkill` 时，内核会在 OOM 时尝试杀死拥有 `ZX_PROP_JOB_KILL_ON_OOM` bit set 的任务，用以恢复内存。

当选项值设为 `jobkill` 时，内核会在 OOM 时，内核会触发 out-of-memory 事件（参见 `zx_system_get_event()`），短暂的延迟后，将重启系统。

<!-- 
### kernel.mexec-force-high-ramdisk=\<bool>
**Default:** `false`

This option is intended for test use only. When set to `true` it forces the
mexec syscall to place the ramdisk for the following kernel in high memory
(64-bit address space, >= 4GiB offset).
-->

### kernel.mexec-force-high-ramdisk=\<bool>
**默认值：** `false`

此选项原意是用作测试目的。当设为 `true` 时，会强制使用 mexec syscall 将后面内核的 ramdisk 放置到更高的地址空间去（对于 64 位地址空间，将偏移 >= 4GiB）。

<!-- 
### kernel.mexec-pci-shutdown=\<bool>
**Default:** `true`

If false, this option leaves PCI devices running when calling mexec.
-->

### kernel.mexec-pci-shutdown=\<bool>
**默认值：** `true`

若设置为 false，在调用 mexec 时，此选项使 PCI 设备保持运行状态。

<!-- 
### kernel.oom.enable=\<bool>
**Default:** `true`

This option turns on the out-of-memory (OOM) kernel thread, which kills
processes or reboots the system (per `kernel.oom.behavior`), when the PMM has
less than `kernel.oom.outofmemory-mb` free memory.

An OOM can be manually triggered by the command `k pmm oom`, which will cause
free memory to fall below the `kernel.oom.outofmemory-mb` threshold. An
allocation rate can be provided with `k pmm oom <rate>`, where `<rate>` is in MB.
This will cause the specified amount of memory to be allocated every second,
which can be useful for observing memory pressure state transitions.

Refer to `kernel.oom.outofmemory-mb`, `kernel.oom.critical-mb`,
`kernel.oom.warning-mb`, and `zx_system_get_event()` for further details on
memory pressure state transitions.

The current memory availability state can be queried with the command
`k pmm mem_avail_state info`.
-->

### kernel.oom.enable=\<bool>
**默认值：** `true`

此选项会启用 内存溢出（OOM）内核线程，在 PMM 少于 `kernel.oom.outofmemory-mb` 指定的空闲内存时，该线程将终止进程或重启系统。

使用命令 `k pmm oom` 可以手动触发 OOM，造成空闲内存降至 `kernel.oom.outofmemory-mb` 指定阈值之下。同时，命令 `k pmm oom <rate>` 可以用来指定内存分配率，其中参数 `<rate>` 单位是 MB。分配率将控制每秒内存分配的总量，从而方便观察变化过程中的内存压力状态。

想要了解更多关于内存压力变化的细节，可以参考 `kernel.oom.outofmemory-mb`、 `kernel.oom.critical-mb`、`kernel.oom.warning-mb`、`zx_system_get_event()`。

使用命令 `k pmm mem_avail_state info` 可查看当前内存可用状态。

<!-- 
### kernel.oom.outofmemory-mb=\<uint64_t>
**Default:** `0x32`

This option specifies the free-memory threshold at which the out-of-memory (OOM)
thread will trigger an out-of-memory event and begin killing processes, or
rebooting the system.
-->

### kernel.oom.outofmemory-mb=\<uint64_t>
**默认值：** `0x32`

此选项指定了空闲内存的阈值，超出该阈值后，内存溢出（OOM）线程将触发内存溢出事件，用以终止进程或者重启系统。

<!-- 
### kernel.oom.critical-mb=\<uint64_t>
**Default:** `0x96`

This option specifies the free-memory threshold at which the out-of-memory
(OOM) thread will trigger a critical memory pressure event, signaling that
processes should free up memory.
-->

### kernel.oom.critical-mb=\<uint64_t>
**默认值：** `0x96`

此选项指定了空闲内存的阈值，超出该阈值后，内存溢出（OOM）线程将触发一个严重内存压力事件，从而表明进程此时应该释放内存。

<!-- 
### kernel.oom.warning-mb=\<uint64_t>
**Default:** `0x12c`

This option specifies the free-memory threshold at which the out-of-memory
(OOM) thread will trigger a warning memory pressure event, signaling that
processes should slow down memory allocations.
-->

### kernel.oom.warning-mb=\<uint64_t>
**默认值：** `0x12c`

此选项指定了空闲内存的阈值，超出该阈值后，内存溢出（OOM）线程将触发一个警告内存压力事件，从而表明进程此时应该放缓申请内存分配。

<!-- 
### kernel.oom.debounce-mb=\<uint64_t>
**Default:** `0x1`

This option specifies the memory debounce value used when computing the memory
pressure state based on the free-memory thresholds
(`kernel.oom.outofmemory-mb`, `kernel.oom.critical-mb` and
`kernel.oom.warning-mb`). Transitions between memory availability states are
debounced by not leaving a state until the amount of free memory is at least
`kernel.oom.debounce-mb` outside of that state.

For example, consider the case where `kernel.oom.critical-mb` is set to 100 MB
and `kernel.oom.debounce-mb` set to 5 MB. If we currently have 90 MB of free
memory on the system, i.e. we're in the Critical state, free memory will have to
increase to at least 105 MB (100 MB + 5 MB) for the state to change from
Critical to Warning.
-->

### kernel.oom.debounce-mb=\<uint64_t>
**默认值：** `0x1`

此选项指定了内存防抖值，该值用于计算基于空闲内存阈值的内存压力状态（阈值包括 `kernel.oom.outofmemory-mb`、`kernel.oom.critical-mb`、`kernel.oom.warning-mb`）。内存可用状态切换的防抖方式是：切换状态时，要求**可用内存总量超过`该状态内存阈值`+`kernel.oom.debounce-mb`**。

比如：`kernel.oom.critical-mb` 设为 100 MB，同时 `kernel.oom.debounce-mb` 设为 5 MB。如果此时我们拥有 90 MB 空闲内存，此时是 Critical 状态，想要从 Critical 切换到 Warning 状态，至少需要空闲内存增加到 105 MB（100 MB + 5 MB）。

<!-- 
### kernel.oom.evict-at-warning=\<bool>
**Default:** `false`

This option triggers eviction of file pages at the Warning pressure state,
in addition to the default behavior, which is to evict at the Critical and OOM
states.
-->

### kernel.oom.evict-at-warning=\<bool>
**默认值：** `false`

设置此选项时，在Warning 压力状态下，分页文件会被移出内存；而默认情况下，移出操作是在 Critical 和 OOM 状态下才会触发的。

<!-- 
### kernel.oom.hysteresis-seconds=\<uint64_t>
**Default:** `0xa`

This option specifies the hysteresis interval (in seconds) between memory
pressure state transitions. Note that hysteresis is only applicable for
transitions from a state with less free memory to a state with more free memory;
transitions in the opposite direction are not delayed.
-->

### kernel.oom.hysteresis-seconds=\<uint64_t>
**默认值：** `0xa`

此选项指定了内存状态切换时，需要延后的时间（以秒为单位）。要注意的是，延后操作只有在 *低内存状态* 向 *高内存状态* 切换时才有效，反之，则无效。

<!-- 
### kernel.oom.imminent-oom-delta-mb=\<uint64_t>
**Default:** `0xa`

This option specifies the delta (in MB) above the out-of-memory threshold at which an
imminent-out-of-memory event will be signaled. This signal is intended to be used for
capturing diagnostic memory information close to the OOM, since capturing state exactly
at the OOM might not be possible.

For example, if `kernel.oom.outofmemory-mb` is set to 50 and `kernel.oom.imminent-oom-delta-mb`
is set to 20, an imminent-out-of-memory event will be signaled at 70MB (i.e. 50MB + 20MB)
free memory, while out-of-memory will be signaled at 50MB free memory.
-->

### kernel.oom.imminent-oom-delta-mb=\<uint64_t>
**默认值：** `0xa`

此选项将触发 *即将内存溢出*（imminent-out-of-memory）事件信号的空闲内存值设为 *内存溢出* 阈值 + 该增量（以 MB 为单位）。该信号意在捕获接近 OOM 的内存信息，因为我们基本不可能精确地捕获 OOM 时的状态。

比如：当 `kernel.oom.outofmemory-mb` 设为 50，同时 `kernel.oom.imminent-oom-delta-mb` 设为 20时， 则系统空闲内存为 70 MB（也就是 50 MB + 20 MB）时，触发 *即将内存溢出*（imminent-out-of-memory）事件，当空闲内存降到 50 MB 时，触发 *内存溢出* 事件。

<!-- 
### kernel.serial=[none | legacy | qemu | \<type>,\<base>,\<irq>]
**Default:** `none`

TODO(53594)
-->

### kernel.serial=[none | legacy | qemu | \<type>,\<base>,\<irq>]
**默认值：** `none`

TODO(53594)

<!-- 
### vdso.ticks_get_force_syscall=\<bool>
**Default:** `false`

If this option is set, the `zx_ticks_get` vDSO call will be forced to be a true
syscall, even if the hardware cycle counter registers are accessible from
user-mode.
-->

### kernel.ticks_get_force_syscall=\<bool>
**默认值：** `false`

设置此选项，则 `zx_ticks_get` vDSO 的调用会被强制设为一个真实的系统调用，即使用户模式下是可以访问硬件循环计数器寄存器的。

<!-- 
### vdso.clock_get_monotonic_force_syscall=\<bool>
**Default:** `false`

If this option is set, the `zx_clock_get_monotonic` vDSO call will be forced to
be a true syscall, instead of simply performing a transformation of the tick
counter in user-mode.
-->

### kernel.clock_get_monotonic_force_syscall=\<bool>
**默认值：** `false`

设置此选项，则 `zx_clock_get_monotonic` vDSO 的调用会被强制设为一个真实的系统调用，而不是在用户模式下，简单地执行 tick 计时器的转换。
<!-- 译者注： 关于tick http://www.embeddedlinux.org.cn/emb-linux/system-development/201708/22-7146.html -->

<!-- 
### kernel.userpager.overtime_wait_seconds=\<uint64_t>
**Default:** `0x14`

This option configures how long a user pager fault may block before being
considered overtime and printing an information message to the debuglog and
continuing to wait. A value of 0 indicates a wait is never considered to be
overtime.
-->

### kernel.userpager.overtime_wait_seconds=\<uint64_t>
**默认值：** `0x14`

此选项定义了在被认定为超时、在调试日志中打印相关信息并继续等待之前，用户分页错误将被阻塞多久。设为 0，则永远不会被认定为超时。

<!-- 
### kernel.userpager.overtime_timeout_seconds=\<uint64_t>
**Default:** `0x12c`

This option configures how long a user pager fault may block before being
aborted. For a hardware page fault, the faulting thread will terminate with a
fatal page fault exception. For a software page fault triggered by a syscall,
the syscall will fail with `ZX_ERR_TIMED_OUT`. A value of 0 indicates a page
fault is never aborted due to a time out.
-->

### kernel.userpager.overtime_timeout_seconds=\<uint64_t>
**默认值：** `0x12c`

此选项定义了在被取消之前，用户分页错误将被阻塞多久。对于硬件分页错误，问题线程将伴随一个严重分页错误异常而终止。对于由系统调用触发的软件分页错误，该系统调用将因`ZX_ERR_TIMED_OUT` 而失败。 设为 0，则永远不会因超时而取消该分页错误。。

<!-- 
### kernel.bufferchain.reserve-pages=\<uint64_t>
**Default:** `0x20`

Specifies the number of pages per CPU to reserve for buffer chain allocations
(channel messages). Higher values reduce contention on the PMM when the
system is under load at the cost of using more memory when the system is
idle.

TODO(fxbug.dev/68456): Determine an upper bound for this value to prevent
consuming too much memory.
-->

### kernel.bufferchain.reserve-pages=\<uint64_t>
**默认值：** `0x20`

此选项定义了每个 CPU 为缓冲链配额（channel messages）预留的分页数量。当系统处于负载状态时，设置更高的数值有助于减少持久化内存模块（PMM）的竞争，当然，代价是在系统空闲时会占用更多的内存。

TODO(fxbug.dev/68456): 需要确定此值的上限，防止占用过多内存。

<!-- 
### kernel.bypass-debuglog=\<bool>
**Default:** `false`

When enabled, forces output to the console instead of buffering it. The reason
we have both a compile switch and a cmdline parameter is to facilitate prints
in the kernel before cmdline is parsed to be forced to go to the console.
The compile switch setting overrides the cmdline parameter (if both are present).
Note that both the compile switch and the cmdline parameter have the side effect
of disabling irq driven uart Tx.
-->

### kernel.bypass-debuglog=\<bool>
**默认值：** `false`

此选项启用时，强制输出调试日志到控制台，而不是缓存该日志。之所以在编译开关和命令行参数两处都可以设置该参数，是为了在命令行被解析为强制输出到控制台之前，加速其在内核的输出。编译开关的设置将覆盖命令行参数的设置（如果二者同时设置了该参数）。请注意，这两处参数设置后，都有禁用 irq 驱动的 uart Tx 的副作用。

<!-- 
### kernel.debug_uart_poll=\<bool>
**Default:** `false`

If true, will periodically poll UART and forwards its contents into the console.
-->

### kernel.debug_uart_poll=\<bool>
**默认值：** `false`

设为 true 时，将定期轮询 UART 并将其内容转发到控制台。

<!-- 
### kernel.enable-debugging-syscalls=\<bool>
**Default:** `false`

When disabled, certain debugging-related syscalls will fail with
`ZX_ERR_NOT_SUPPORTED`. These are:
- `zx_debug_send_command()`
- `zx_ktrace_control()`
- `zx_ktrace_init()`
- `zx_ktrace_read()`
- `zx_mtrace_control()`
- `zx_process_write_memory()`
- `zx_system_mexec()`
- `zx_system_mexec_payload_get()
-->

### kernel.enable-debugging-syscalls=\<bool>
**默认值：** `false`

此选项禁用时，指定的调试相关的系统调用将失败并返回 `ZX_ERR_NOT_SUPPORTED`。 指定的系统调用如下：
- `zx_debug_send_command()`
- `zx_ktrace_control()`
- `zx_ktrace_init()`
- `zx_ktrace_read()`
- `zx_mtrace_control()`
- `zx_process_write_memory()`
- `zx_system_mexec()`
- `zx_system_mexec_payload_get()`

<!-- 
### kernel.enable-serial-syscalls=\<string>
**Default:** `false`

Can be one of three values:
- `false`
- `true`
- `output-only`

When `false`, both `zx_debug_read()` and `zx_debug_write()` will fail with
`ZX_ERR_NOT_SUPPORTED`.

When `output-only`, `zx_debug_read()` will fail with `ZX_ERR_NOT_SUPPORTED`, but `zx_debug_write()`
will work normally.

When `true`, both will work normally.
-->

### kernel.enable-serial-syscalls=\<string>
**默认值：** `false`

可选值如下：
- `false`
- `true`
- `output-only`

设为`false`时，`zx_debug_read()` 和 `zx_debug_write()`调用时会失败并返回`ZX_ERR_NOT_SUPPORTED`。

设为`output-only`时，`zx_debug_read()`调用时会失败并返回`ZX_ERR_NOT_SUPPORTED`，`zx_debug_write()`可正常执行。

设为`true`时，`zx_debug_read()` 和 `zx_debug_write()`均可以正常执行。

<!-- 
### kernel.entropy-test.src=[hw_rng | jitterentropy]
**Default:** `hw_rng`

When running an entropy collector quality test, use the provided entropy source.
This option is ignored unless the kernel was built with `ENABLE_ENTROPY_COLLECTOR_TEST=1`.
-->

### kernel.entropy-test.src=[hw_rng | jitterentropy]
**默认值：** `hw_rng`

当运行熵收集器质量测试时，请使用参数提供的熵源。 除非内核是使用“ENABLE_ENTROPY_COLLECTOR_TEST=1”构建的，否则将忽略此选项。

<!-- 
### kernel.entropy-test.len=\<uint64_t>
**Default:** `0x100000`

When running an entropy collector quality test, collect the provided number of
bytes.

The maximum value can be increased by defining `ENTROPY_COLLECTOR_TEST_MAXLEN` as such value.
-->

### kernel.entropy-test.len=\<uint64_t>
**默认值：** `0x100000`

当运行熵收集器质量测试时，此参数指定了要收集的字节数。

可以通过定义 `ENTROPY_COLLECTOR_TEST_MAXLEN` 来定义此参数的最大值。

<!-- 
### kernel.force-watchdog-disabled=\<bool>
**Default:** `false`

When set, the system will attempt to disable any hardware watchdog timer armed
and passed by the bootloader as soon as it possibly can in the boot sequence,
presuming that the bootloader provides enough information to know how to disable
the WDT at all.
-->

### kernel.force-watchdog-disabled=\<bool>
**默认值：** `false`

此选项设为 true 时，系统将尝试在引导序列中尽快禁用引导加载程序设置和传递的任何硬件看门狗定时器，假设引导加载程序提供了足够的信息来了解如何禁用 WDT。

<!-- 
### gfxconsole.early=\<bool>
**Default:** `false`

This option requests that the kernel start a graphics console
during early boot (if possible), to display kernel debug print
messages while the system is starting.  When userspace starts up, a usermode
graphics console driver takes over.

The early kernel console can be slow on some platforms, so if it is not
needed for debugging it may speed up boot to disable it.
-->

### gfxconsole.early=\<bool>
**默认值：** `false`

此选项请求内核在启动初期打开一个图形控制台（如果可行的话），用来系统启动过程中，显示内核调试打印出的信息。当用户空间启动后，用户模式图形控制台驱动程序将接管。

早期内核控制台在一些平台上可能比较缓慢，所以，如非调试所需，禁用该选项有助于加速系统启动速度。

<!-- 
### gfxconsole.font=[9x16 | 18x32]
**Default:** `9x16`

This option asks the graphics console to use a specific font.
-->

### gfxconsole.font=[9x16 | 18x32]
**默认值：** `9x16`

此选项要求图形控制台使用指定的字体。

<!-- 
### kernel.halt-on-panic=\<bool>
**Default:** `false`

If this option is set, the system will halt on a kernel panic instead
of rebooting. To enable halt-on-panic, pass the kernel command line
argument `kernel.halt-on-panic=true`.

Since the kernel can't reliably draw to a framebuffer when the GPU is enabled,
the system will reboot by default if the kernel crashes or panics.

If the kernel crashes and the system reboots, the log from the kernel panic will
appear at `/boot/log/last-panic.txt`, suitable for viewing, downloading, etc.

> Please attach your `last-panic.txt` and `zircon.elf` files to any kernel
> panic bugs you file.

If there's a `last-panic.txt`, that indicates that this is the first successful
boot since a kernel panic occurred.

It is not "sticky" -- if you reboot cleanly, it will be gone, and if you crash
again it will be replaced.
-->

### kernel.halt-on-panic=\<bool>
**默认值：** `false`

此选项设为 true 时，当发生内核错误，系统将死机而非重启。

当 GPU 启用时，因为内核无法可靠地绘制图形到帧缓冲器，故而在内核崩溃或错误时，系统会默认重启。

如果内核崩溃，系统重启了，内核错误的日志信息会出现在  `/boot/log/last-panic.txt`中，方便查看、下载等操作。

> 请在你提交的任何内核 bug 后，附上`last-panic.txt`和`zircon.elf`文件。

如果存在一个`last-panic.txt`文件，则表明这是发生内核错误后第一次成功启动系统。

当然，错误日志并不是一个 “粘人的小妖精” -- 如果你顺利地重启了系统，它就会自动消失的， 又或者你的内核再次崩溃了，该文件将被替换为新的日志。

<!-- 
### ktrace.bufsize=\<uint32_t>
**Default:** `0x20`

This option specifies the number of megabytes allocated for ktrace records.
-->

### ktrace.bufsize=\<uint32_t>
**默认值：** `0x20`

此选项指定了为 ktrace 记录分配的千字节（MegaBytes，MB）数。

<!-- 
### ktrace.grpmask=\<uint32_t>
**Default:** `0xfff`

This option specifies what ktrace records are emitted.
The value is a bitmask of KTRACE\_GRP\_\* values from zircon/ktrace.h.
Hex values may be specified as 0xNNN.
-->

### ktrace.grpmask=\<uint32_t>
**默认值：** `0xfff`

此选项指定有哪些 ktrace 记录需要被输出。选项值是 zircon/ktrace.h 头文件中 KTRACE\_GRP\_\* 值的掩码。 十六进制值类似于 0xNNN。

<!-- 
### kernel.memory-limit-dbg=\<bool>
**Default:** `true`

This option enables verbose logging from the memory limit library.
-->

### kernel.memory-limit-dbg=\<bool>
**默认值：** `true`

此选项会为内存限制库启用详细日志输出。

<!-- 
### kernel.memory-limit-mb=\<uint64_t>
**Default:** `0x0`

This option sets an upper-bound in megabytes for the system memory.
If set to zero, then no upper-bound is set.

For example, choosing a low enough value would allow a user simulating a system with
less physical memory than it actually has.
-->

### kernel.memory-limit-mb=\<uint64_t>
**默认值：** `0x0`

此选项指定了系统内存的上限，以 MB（MegaByte）为单位。设为 0 则无上限。

比如，设置一个足够小的值，则可以让用户模拟出一个小于真实物理内存的系统。

<!-- 
### kernel.page-scanner.enable-eviction=\<bool>
**Default:** `true`

When set, allows the page scanner to evict user pager backed pages. Eviction can
reduce memory usage and prevent out of memory scenarios, but removes some
timing predictability from system behavior.
-->

### kernel.page-scanner.enable-eviction=\<bool>
**默认值：** `true`

此选项设为 true 时，允许页扫描程序删除用户分页器备份的分页。该操作可以减少内存使用并防止内存不足的情况，但会删除系统行为中一些时序可预测性。

<!-- 
### kernel.page-scanner.discardable-evictions-percent=\<uint32_t>
**Default:** `0x0`

Percentage of page evictions, that should be satisfied from
discardable VMOs, as opposed to pager-backed VMOs. For example, if this value
is set to `X` and the kernel needs to reclaim `N` pages to relieve memory
pressure, it will evict `(N * X / 100)` pages from discardable VMOs, and the
remaining `(N * (100 - X) / 100)` pages from pager-backed VMOs.

Note that the kernel will try its best to honor this ratio between discardable
and pager-backed pages evicted, but the actual numbers might not be exact.
-->

### kernel.page-scanner.discardable-evictions-percent=\<uint32_t>
**默认值：** `0x0`

**TODO-SYLSD: NEED DISCUSSION**
将删除分页时，可丢弃 VMO 的占删除分页总数的百分比，剩余分页从 pager-backed VMO 中删除。例如，如果参数值设为`X`，而内核为了缓解内存压力，需要回收的分页数为`N`，则删除可丢弃 VMO 数量为`(N * X / 100)`，删除 pager-backed VMO 数量为`(N * (100 - X) / 100)`。

注意，内核会尽量分配好两种分页被删除的比例，但实际数值可能并不精确。

<!-- 
### kernel.page-scanner.page-table-eviction-policy=[always | never | on_request]
**Default:** `always`

Sets the policy for what to do with user page tables that are not accessed
between scanner runs.

When `on_request`, only performs eviction on request, such as in response to a
low memory scenario.

When `never`, page tables are never evicted.

When `always`, Unused page tables are always evicted every time the
scanner runs.


## Options available only on arm64 machines
-->

### kernel.page-scanner.page-table-eviction-policy=[always | never | on_request]
**默认值：** `always`

对于扫描程序运行之间无法访问的用户分页表，设置处理策略。

设为`on_request`时，仅在请求时执行删除操作，例如对于内存值偏低时的响应。

设为`never`时，分页表用不删除。

设为`on_request`时，每次扫描程序运行时，都删除无用的（Unused）分页。

<!-- 
### kernel.arm64.disable_spec_mitigations=\<bool>
**Default:** `false`

If set, disables all speculative execution information leak mitigations.

If unset, the per-mitigation defaults will be used.
-->

### kernel.arm64.disable_spec_mitigations=\<bool>
**默认值：** `false`

设为 true 时，禁用所有推测执行信息泄漏缓解措施。

反之，将默认使用 per-mitigation。

<!-- 
### kernel.arm64.event-stream.enable=\<bool>
**Default:** `true`

When enabled, each ARM cpu will enable an event stream generator, which per-cpu
sets the hidden event flag at a particular rate. This has the effect of kicking
cpus out of any WFE states they may be sitting in.
-->

### kernel.arm64.event-stream.enable=\<bool>
**默认值：** `true`

设为 true 时，每个 ARM cpu 都将启用一个事件流生成器，每个 cpu 以一个特定的比例设置隐藏事件标记。具体效果是 CPU 会退出他们可能处于的任何 WFE 状态。

<!-- 
### kernel.arm64.event-stream.freq-hz=\<uint32_t>
**Default:** `0x2710`

If the event stream is enabled, specifies the frequency at which it will attempt
to run. The resolution is limited, so the driver will only be able to pick the
nearest power of 2 from the cpu timer counter.
-->

### kernel.arm64.event-stream.freq-hz=\<uint32_t>
**默认值：** `0x2710`

如果参数 `kernel.arm64.event-stream.enable` 被启用，指定此参数则可以设置器运行的频率。分辨率是有限的，因此驱动程序只能从 CPU 计时器计数器中选择最接近的 2 次方。

<!-- 
### kernel.arm64.debug.dap-rom-soc=\<string>

If set, tries to initialize the dap debug aperture at a hard coded address for the particular
system on chip. Currently accepted values are amlogic-t931g, amlogic-s905d2, and amlogic-s905d3g.


## Options available only on x86 machines
-->

### kernel.arm64.debug.dap-rom-soc=\<string>

此选项启用时，则尝试在特定芯片系统（SoC，System on Chip）的硬编码地址处初始化 dap 调试孔。 此时可接受参数为 amlogic-t931g、amlogic-s905d2 和 amlogic-s905d3g。

## x86 设备专用参数

<!-- 
### kernel.x86.disable_spec_mitigations=\<bool>
**Default:** `false`

If set, disables all speculative execution information leak mitigations.

If unset, the per-mitigation defaults will be used.
-->

### kernel.x86.disable_spec_mitigations=\<bool>
**默认值：** `false`

设为 true 时，禁用所有推测执行信息泄漏缓解措施。

反之，将默认使用 per-mitigation。

<!-- 
### kernel.x86.hwp=\<bool>
**Default:** `true`

This settings enables HWP (hardware P-states) on supported chips. This feature
lets Intel CPUs automatically scale their own clock speed.
-->

### kernel.x86.hwp=\<bool>
**默认值：** `true`

此设置在支持的芯片上启用 HWP（hardware P-states）。此功能可让 Intel CPU 自动调整自己的时钟速度。

<!-- 
### kernel.x86.hwp_policy=[bios-specified | performance | balanced | power-save | stable-performance]
**Default:** `bios-specified`

Set a power/performance tradeoff policy of the CPU. x86 CPUs with HWP
(hardware P-state) support can be configured to autonomusly scale their
frequency to favour different policies.

Currently supported policies are:

*   `bios-specified`: Use the power/performance tradeoff policy
    specified in firmware/BIOS settings. If no policy is available, falls back
    to `balanced`.
*   `performance`: Maximise performance.
*   `balanced`: Balance performance / power savings.
*   `power-save`: Reduce power usage, at the cost of lower performance.
*   `stable-performance`: Use settings that keep system performance consistent.
    This may be useful for benchmarking, for example, where keeping performance
    predictable is more important than maximising performance.
-->

### kernel.x86.hwp_policy=[bios-specified | performance | balanced | power-save | stable-performance]
**默认值：** `bios-specified`

设置 CPU 的功率/性能权衡策略。 具有 HWP（hardware P-states）支持的 x86 CPU 可以配置为自主调整其频率以支持不同的策略。

目前支持的策略有：
*   `bios-specified`: 使用 firmware/BIOS 设置中定义的功率/性能权衡策略。 如果没有可用策略，则回滚为 `balanced`。
*   `performance`: 最佳性能。
*   `balanced`: 平衡 性能/省电
*   `power-save`: 以低性能换取低能耗
*   `stable-performance`: 使用保持系统性能一致的设置。这对于基准测试可能很有用，例如，保持性能可预测比最大化性能更重要。

<!-- 
### kernel.x86.md_clear_on_user_return=\<bool>
**Default:** `true`

MDS (Microarchitectural Data Sampling) is a family of speculative execution
information leak bugs that allow the contents of recent loads or stores to be
inferred by hostile code, regardless of privilege level (CVE-2019-11091,
CVE-2018-12126, CVE-2018-12130, CVE-2018-12127). For example, this could allow
user code to read recent kernel loads/stores.

To avoid this bug, it is required that all microarchitectural structures
that could leak data be flushed on trust level transitions. Also, it is
important that trust levels do not concurrently execute on a single physical
processor core.

This option controls whether microarchitectual structures are flushed on
the kernel to user exit path, if possible. It may have a negative performance
impact.

*   If set to true (the default), structures are flushed if the processor is
    vulnerable.
*   If set to false, no flush is executed on structures.
-->

### kernel.x86.md_clear_on_user_return=\<bool>
**默认值：** `true`

MDS（微架构数据采样）是一系列推测执行信息泄漏漏洞，允许恶意代码推断最近加载或存储的内容，而不考虑权限级别（CVE-2019-11091、CVE-2018-12126、CVE- 2018-12130，CVE-2018-12127）。例如，这可以允许用户代码读取最近的内核加载/存储。

为避免此错误，需要在信任级别转换时刷新所有可能泄漏数据的微体系结构结构。 此外，重要的是不要在单个物理处理器内核上同时执行信任级别。

如果可能，此选项将控制是否将内核上的微架构结构 flush 到用户程序出口路径。 同时，它可能会对性能产生负面影响。

*   此选项设为 true 时（默认），如果处理器易受攻击，则 flush 结构。
*   反之，则不执行操作。

<!-- 
### kernel.x86.pti.enable=\<uint32_t>
**Default:** `0x2`

Page table isolation configures user page tables to not have kernel text or
data mapped. This may impact performance negatively. This is a mitigation
for Meltdown (AKA CVE-2017-5754).

* If set to 1, this force-enables page table isolation.
* If set to 0, this force-disables page table isolation. This may be insecure.
* If set to 2 or unset (the default), this enables page table isolation on
CPUs vulnerable to Meltdown.

TODO(joshuaseaton): make this an enum instead of using magic integers.
-->

### kernel.x86.pti.enable=\<uint32_t>
**默认值：** `0x2`

分页表隔离将用户分页表配置为不映射内核文本或数据。 这可能会对性能产生负面影响。 这是漏洞 Meltdown (AKA CVE-2017-5754) 的缓解措施。

* 设为 1 时，强制分页表隔离。
* 设为 0 时，强制禁用分页表隔离。此操作会造成安全风险。
* 设为 2 或不设置 (默认), 这可以在易受 Meltdown 影响的 CPU 上启用分页表隔离。

<!-- 
### kernel.x86.spec_store_bypass_disable=\<bool>
**Default:** `false`

Spec-store-bypass (Spectre V4) is a speculative execution information leak
vulnerability that affects many Intel and AMD x86 CPUs. It targets memory
disambiguation hardware to infer the contents of recent stores. The attack
only affects same-privilege-level, intra-process data.

This command line option controls whether a mitigation is enabled. The
mitigation has negative performance impacts.

* If true, the mitigation is enabled on CPUs that need it.
* If false (the default), the mitigation is not enabled.
-->

### kernel.x86.spec_store_bypass_disable=\<bool>
**默认值：** `false`

Spec-store-bypass (Spectre V4) 是一个推测执行信息泄漏漏洞，影响了许多 Intel 和 AMD x86 CPU。 它针对内存消歧硬件来推断最近存储的内容。 攻击只影响相同权限级别的进程内数据。

此命令行选项控制是否启用缓解。 缓解措施会对性能产生负面影响。

* 设为 true 时， 在需要的 CPU 上，启用缓解措施。
* 设为 false (默认)时，禁用缓解措施。

<!-- 
### kernel.x86.turbo=\<bool>
**Default:** `true`

Turbo Boost or Core Performance Boost are mechanisms that allow processors to
dynamically vary their performance at runtime based on available thermal and
electrical budget. This may provide improved interactive performance at the cost
of performance variability. Some workloads may benefit from disabling Turbo; if
this command line flag is set to false, turbo is disabled for all CPUs in the
system.

TODO: put something here
-->

### kernel.x86.turbo=\<bool>
**默认值：** `true`

Turbo Boost 或 Core Performance Boost 是允许处理器在运行时根据可用的热能和电能预算动态改变其性能的机制。 这可以以性能可变性为代价提供改进的交互性能。 某些工作负载可能会从禁用 Turbo 中受益； 如果此命令行标志设置为 false，则系统中的所有 CPU 都将禁用 turbo。

TODO： 未完待续
