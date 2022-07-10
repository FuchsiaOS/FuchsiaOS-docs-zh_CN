<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0159" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

<!-- ## Summary -->
## 总述
<!-- 
This document proposes changes to kernel APIs to support binaries with
execute-only segments, by adding a new feature check in
`zx_system_get_features` and changing the `launchpad` and `process_builder`
loaders as well as the dynamic linker in Fuchsia's in-tree libc to support '--x'
segments. It lays out a plan for eventual kernel support for mapping
execute-only pages on hardware that supports it. -->

本文档提出了对内核API的更改，以支持带有仅执行段的二进制文件。本文档向 `zx_system_get_features` 
中加入了新特性判断，更改了 `launchpad` 和 `process_builder` 加载器，并更改了 Fuchsia 的 in-tree libc 
中的动态链接器来支持 '--x' 参数。其为内核最终实现在支持的硬件上对仅执行页的映射提出了计划。
<!-- 
We don't typically need to read executable memory after it has been loaded.
Enabling execute-only code by default increases security of Fuchsia’s userspace
processes and furthers the engineering best practice of least permissions. -->

我们通常并不需要读取已加载的可执行内存。默认启用仅执行代码能增强 Fuchsia 用户空间进程的安全性，
并有助于推进最小权限的工程最佳做法。

<!-- ## Motivation -->
## 动机
<!-- 
Support for execute-only pages was added to ARM MMUs in ARMv7m and allows pages
of memory to be mapped such that they are only executable and not readable or
writable. Though writable code pages have been considered a security threat for
a long time, allowing code to remain readable has been shown to expose
applications to needless risk. Specifically, reading code pages is often a first
step in an attack chain, and preventing code from being read hinders
adversaries. See [Readable Code Security](#readable-code-security). Moreover,
supporting execute-only pages fits well with Fuchsia’s permissions model and
more strongly aligns with the principle of least privilege: often code doesn’t
need to be read, but just be executed. -->

ARM 的 MMU 在 ARMv7m 中加入了对仅执行页的支持，允许内存页被映射为既不可读也不可写的仅执行状态。
可写的代码页很早就被认为有安全威胁，但允许代码保持可读也将应用暴露于不必要的风险中。实际上，对代码页
的读取常常成为攻击链的第一步，防止对代码的读取能对攻击形成阻碍。详见[可读代码安全](#readable-code-security)。
而且，支持仅执行页不仅很好地符合了 Fuchsia 的权限模型，也更符合最小特权原则：代码通常并不需要读，而只用执行。

<!-- ## Stakeholders -->
## 相关方

<!-- _Facilitator:_ -->
协调人：

- cpu@google.com

<!-- _Reviewers:_ -->
审阅人：

- phosek@google.com
- mvanotti@google.com
- maniscalco@google.com
- travisg@google.com

<!-- ## Background -->
## 背景

<!-- ### Execute-only Memory -->
### 仅执行内存
<!-- 
Execute-only memory (XOM) describes memory pages that have neither read nor
write permissions and can only be executed. ARMv7m and above have native support
for XOM, however there are some considerations on older ISA’s. Discussed further
in [XOM and PAN](#xom-and-pan). -->

仅执行内存（XOM）指没有读和写权限，仅能执行的内存页。ARMv7m 及之后的 ISA 原生支持 XOM，但对
旧 ISA 的支持也在考虑中。进一步讨论请到 [XOM 与 PAN](#xom-and-pan)。
<!-- 
This doc focuses almost exclusively on AArch64, however the implementation is
architecture agnostic. When hardware and toolchain support matures for other
architectures, they would all easily be able to take advantage of execute-only
support in Fuchsia. -->

本文几乎仅关注 AArch64，但具体实现与架构无关。当其他架构的硬件和工具链支持成熟后，应该能很容易受益于 
Fuchsia 中的仅执行支持。

<!-- ### Permissions of Code Pages -->

### 代码页权限
<!-- 
Initially, computers supported direct memory access to physical memory without
any checks or protections. The introduction of MMUs provided a key abstraction,
in the form of virtual memory, by decoupling a program's view of memory from the
underlying physical resources. This facilitated a more flexible, safe, and
secure programming model by allowing OS implementers to provide strong isolation
between their programs via the process abstraction. Today's MMUs provide a
number of critical facilities, such as paged memory, fast address translation,
and permission checking. They also allow users significant control over how
memory regions can be accessed and used, via the page permissions that typically
control if memory pages can be read, written to, or executed. This is a key
property for program safety, fault isolation, and security, since it restricts a
program's ability to misuse system resources through hardware enforced
permission checks. -->

最开始，计算机支持对物理内存的直接内存访问，没有任何检查或保护。MMU 的引入提供了关键的抽象，它以
虚拟内存的形式将程序视角的内存和底层的物理资源解耦。这样就为一种灵活又安全的编程模式提供了便利，
即允许操作系统通过进程这种抽象，在其上运行的程序间提供强大的隔离能力。如今的 MMU 提供了大量关键
功能，如内存分页、快速地址翻译和权限检查，进而允许用户控制内存页的读、写与执行等权限，使用户对内存区域
的访问和使用方式有了显著控制。这对于程序安全和错误隔离很关键，因为这样就能通过硬件强制的权限检查
限制程序恶意使用系统资源的能力。
<!-- 
Memory that is both writable and executable is particularly dangerous because it
provides an easy way for an adversary to achieve arbitrary code execution
through common vulnerabilities, like buffer overflows. For this reason, many OS
configurations explicitly disallow pages to be both writable and executable
(W^X). This has been the standard for over a decade, OpenBSD added support for
W^X in 2003 with OpenBSD 3.3 [openbsd-wxorx]. See also SELinux W^X policies
[selinux-wxorx].  Writable code can be useful for things like just-in-time (JIT)
compilation, which writes executable instructions to memory at runtime. Having
W|X pages can be disallowed and JIT’s need to work around this. An easy way is
to write code to non-executable pages and later change the page protections,
i.e., through `mprotect` or `zx_vmar_protect`, to be executable but not writable
[example-fuchsia-test]. In nearly all cases pages that are W|X are too
permissive. Similarly, executable pages rarely ever need to be read [See
exceptions](#readable-code). Allowing read operations on executable pages is
generally unnecessary and should not be the default. -->

既能写又能执行的内存相当危险，因为它为攻击者利用缓冲区溢出等常见漏洞达到任意代码执行提供了一种容易的路径。
由于这个原因，许多操作系统的配置显式地禁止内存页同时可写和可执行（W^X）。这已成为十数年间的标准。
OpenBSD 于 2003 年在 OpenBSD 3.3 中加入了对 W^X 的支持 [openbsd-wxorx]。也请参考 
SELinux 的 W^X 政策 [selinux-wxorx]。可写的内存对于 just-in-time (JIT) 编译这种在
运行时往内存里写入可执行指令的技术确实有用。由于不能允许 W|X 的页，JIT 需要绕过它。一种简便的
方法是先将代码写入不可执行页，再通过 `mprotect` 或者 `zx_vmar_protect` 来将页保护状态变为可执行
但不可写 [example-fuchsia-test]。在几乎所有情况下 W|X 的页权限都过大了。与此类似，可执行
页也几乎不怎么需要读 [参见例外](#readable-code)。允许对可执行页的读操作通常并无必要，也不应
成为默认配置。

<!-- ### Readable Code -->
### 可读代码
<!-- 
Because of ARM’s fixed instruction width, immediate values have size
constraints. For this reason loads are done using PC-Relative addressing. To get
around this, the pseudo instruction `ldr Rd, =imm` will emit `imm` in literal
pools close to the code loading it. This is incompatible with XOM because it
puts data in the text section which must be readable. When searching for use of
literal pools in the codebase to ensure we don’t read executable segments, we
have found some usages of `ldr Rd, =imm` in Zircon, but all has since been
removed. Clang will not use literal pools for aarch64, instead it will emit
multiple instructions to create a large immediate. Clang has a `-mexecute-only`
flag and alias `-mpure-code` but these are only meaningful on arm32 because
these flags are inherent when targeting aarch64. -->

由于 ARM 的指令是定长的，对立即数大小有约束，所以 load 操作会以相对 PC 寻址的方式实现。具体来说，
`ldr Rd, =立即数` 伪指令会在临近的文字池中放置 `立即数`。这与 XOM 不兼容，因为它将数据放在了 text 
 section 中，必须要可读。我们在代码库里搜索文字池的使用以确保没有对可执行段的读操作时，
 发现 Zircon 中有一些 `ldr Rd, =立即数` 的使用，但现在都移除了。Clang 不会在 aarch64 中使用
 文字池，而会生成多条指令来创建一个大立即数。Clang有个 `-mexecute-only` 标志和其别名 `-mpure-code`，
 但这只在 arm32 上有意义，因为 aarch64 本就蕴涵这个标志。

<!-- #### Example: Large Intermediates -->
#### 示例：大立即数

<!-- 
This example shows how Clang compiles this C code to assembly given different
targets [clang-example]. The top row shows aarch64, and the bottom shows arm32: -->
本示例展示 Clang 在给定不同的 target 时如何将这段 C 代码编译为汇编 [clang-example]。
其中上面是 aarch64，下面是 arm32：

```
uint32_t a() {
    return 0x12345678u;
}
```
```
# -target aarch64
a:
    mov w0, #22136
    movk w0, #4660, lsl #16
    ret
```
```
# -target arm
a:
    ldr r0, .LCPI0_0
    bx lr
.LCPI0_0:
    .long 305419896
```

<!-- ### XOM and PAN -->
### XOM 与 PAN
<!-- 
Privileged access never (PAN) is a security feature on ARM chips that prevents
normal memory access to user pages from kernel mode. It helps protect against
potential kernel vulnerabilities because the kernel cannot touch user memory
with a normal load or store instructions. Instead the OS would need to turn PAN
off or use the `ldtr` and `sttr` instructions for accessing those pages. PAN is
not currently enabled for Fuchsia, but there are already plans to support it in
zircon [pan-fxb]. -->

特权访问禁止（PAN）是 ARM 芯片中阻止内核态以正常方式访问用户页内存的安全特性。这种特性有助于防范
潜在的内核漏洞，因为内核无法用正常的 load 和 store 指令接触用户内存。操作系统若要访问这些内存页，
需要先关闭 PAN，或使用 `ldtr` 和 `sttr` 指令。PAN 现在在 Fuchsia 中并未启用，但已有在 Zircon 
中提供相应支持的计划 [pan-fxb]。
<!-- 
Aarch64 page table entries have 4 relevant bits to control page permissions. 2
bits are used for user and privileged execute-never. The remaining two are used
to describe read and write page permissions for both access levels. An
execute-only mapping has both read and write access removed but allows user
execution. -->

aarch64 的页表项有四个控制页权限的位。其中两个用于用户和特权的执行禁止，另两个用来描述这两个访问级别
的读和写权限。仅执行的映射既无写权限也无读权限，但允许用户执行。
<!-- 
This table from the ARMv8 Reference Manual shows the possible memory protections
using the only 4 available bits. EL0 is the exception level for userspace. Rows
0 and 2 show how to create userspace execute-only pages. See Table D5-34 Stage 1
from the ARMv8 Reference Manual. -->

这张 ARMv8 参考手册中的表格展示了使用四个二进制位能表示的内存保护状态。EL0 表示用户空间的特权级别。
第0和第2行展示了创建用户空间仅执行页的方法。见 ARMv8 参考手册的表 D5-34 Stage 1。

<!-- | UXN | PXN | AP[2:1] | Access from a higher Exception level | Access from EL0  | -->
| UXN | PXN | AP[2:1] | 从更高特权级别访问                      | 从 EL0 访问       |
|-----|-----|---------|--------------------------------------|------------------|
| 0   | 1   | 00      | R, W                                 | X                |
| 0   | 1   | 01      | R, W                                 | R, W, X          |
| 0   | 1   | 10      | R                                    | X                |
| 0   | 1   | 11      | R                                    | R, X             |
<!-- 
Unfortunately, PAN’s algorithm for deciding if a page should not be privileged
accessible checks if the page is user-readable. From the perspective of PAN, a
user-execute-only page looks like a privileged mapping. This allows the kernel
to access user memory where it otherwise should not, thereby bypassing PAN’s
intended purpose and making PAN and XOM incompatible [pan-issue]. This would
make any future usage of PAN not useful against attacks trying to exploit the
kernel touching user memory, however it would still be useful for detecting
kernel bugs. -->

很不幸，PAN 决定一个内存页能否被特权访问的算法只检查了这个页是否用户不可读。在 PAN 眼中，只能
被用户执行的页与能被特权访问的页看起来一样。这使内核能在本不应该的地方访问用户内存，从而绕过了 
PAN 的设计意图，使得 PAN 与 XOM 不兼容 [pan-issue]。这样一来，尽管 PAN 还能用来探测内核 bug，
但它再也无法用来防止那些意图通过攻破内核来接触用户内存的攻击。
<!-- 
This problem caused both Linux and Android to drop support for XOM. This was
particularly noticeable for Android who dropped support indefinitely in Android
11 after being added and made the default for all aarch64 binaries in Android 10
[linux-revert][android-xom]. They plan to re-enable the feature as hardware
which fixes the problem becomes more ubiquitous but there is no concrete time
frame when it will be readded. -->

这个问题导致 Linux 和 Android 都放弃了对 XOM 的支持。特别是 Android，先在 Android 10 中
加入了支持，并将其设为所有 aarch64 二进制的默认选项，又在 Android 11 中将其无限期放弃 [linux-revert]
[android-xom]。他们计划在能解决这个问题的硬件更加普及后再重启这一特性，但尚无将其重新加入的
明确时间表。
<!-- 
ARM has since proposed a solution with “enhanced” PAN or ePAN, which changes PAN
to check not just if a page is user readable but also not user executable.
Unfortunately, hardware with the feature may not be on any Fuchsia-targeted
devices for years. Linux has since re-added their implementation of XOM after
ePAN was made [linux-re-land]. Support for ePAN on devices is out of our control
and the incompatibility with PAN and XOM should not block the kernel’s
implementation of PAN [See more](#risks). -->

ARM 随后提出了一种称为“加强”PAN，或 ePAN 的解决方法，将 PAN 改为不仅检查内存页是否用户可读，也
检查其是否用户可执行。然而，带有此特性的硬件也许好多年都不会出现在 Fuchsia 的目标设备上。Linux 
在 ePAN 出现后重新添加了他们的 XOM 实现 [linux-re-land]。设备对 ePAN 的支持情况不在我们的
掌控中，PAN 与 XOM 的不兼容也不应阻碍内核对 PAN 的实现 [详见](#risks)。
<!-- 
From figure 2, there is no possible configuration where read permission can be
stripped from the kernel. The only exception is PAN, which can cause an
exception when the kernel tries to touch a user-readable page. For this reason,
it is not possible to create an execute-only mapping for the kernel, since the
kernel cannot mark a page executable at EL1 but not readable. Thus, it is only
possible to create an execute-only mapping for userspace processes. -->

根据表二，并不存在一种配置能将读权限从内核中剥离。唯一的例外是 PAN，其能在内核试图访问用户可读
的页时引发异常。因此，没有办法为内核创建一种仅执行映射，因为内核没法将某页标记为 EL1 可执行的同时
让它不可读。所以，仅执行映射只能为用户空间进程创建。

<!-- ### Targeting XOM Hardware -->
### 为 XOM 硬件构建
<!-- 
Segment permissions in ELF indicate what permissions the code requires to run
correctly. In other words, software doesn’t need to know at build time if the
hardware it will run on can support XOM or not. Instead, it should
unconditionally use XOM if it will not need to read code pages. It is up to the
OS and loaders to enforce those permissions to the greatest extent the system
allows [elf-segment-perm]. -->

ELF 中的段权限表明代码需要哪些权限才能正确运行。也就是说，软件在构建时并不需要知道硬件是否支持 
XOM，而应该只要不需要读代码页就无条件地使用 XOM。至于怎样将那些权限启用到系统允许的最大程度，应该由
操作系统和程序加载器决定 [elf-segment-perm]。

<!-- ### Virtual Memory Permissions -->
### 虚拟内存权限
<!-- 
POSIX specifies that `mmap` may permit read access to pages where `PROT_READ`
has not been explicitly set [posix-mmap]. Both Linux and macOS on x86, and macOS
on M1 chips, will not fail when requesting pages from mmap with just `PROT_EXEC`
and instead make the pages `PROT_READ | PROT_EXEC`. These implementations have
syscalls which are “best-effort” in their ability to honor a user's requests.
Fuchsia syscalls, on the other hand, are always explicit in what they can and
cannot honor. The `zx_vmar_*` syscalls do not silently escalate permissions of
pages like their POSIX counterparts are permitted to by the standard. Requesting
pages without `ZX_VM_PERM_READ` will currently always fail as the hardware and
the OS do not support mapping pages without read permissions. A graceful
transition to supporting binaries with execute-only segments and userspace
programs which allocate execute-only memory will require a way to check if the
OS can map execute-only pages prior to requesting them. -->

POSIX 规定 `mmap` 可以允许对没有显式设置 `PROT_READ` 的页的读取操作 [posix-mmap]。M1 芯片
上的 macOS 和 x86 上的 Linux 和 macOS 在遇到用 mmap 请求内存页但只设置了 `PROT_EXEC` 的情况时都不会
报错，而是将被请求内存页设为 `PROT_READ | PROT_EXEC`。这些系统调用的实现只是在能力范围内“尽力”满足
用户的请求。与此相对，Fuchsia 的系统调用在能否满足用户请求的问题上从来很明确。`zx_vmar_*` 系统调用
并不会像 POSIX 中的对应调用按照标准允许的一样静默提升内存页权限。请求内存页时不设置 `ZX_VM_PERM_READ` 
目前必定报错，因为硬件和操作系统不支持映射没有读权限的页。要平滑地向对带有仅执行段的二进制和
分配仅执行内存的用户空间程序的支持迁移，需要一种在请求前判断操作系统能否映射仅执行页的方法。

<!-- ### Readable Code Security -->
### 可读代码安全
<!-- 
Many attacks rely on finding out information about the process through reading
code pages to find “gadgets”, or executable code of interest. Address space
layout randomization (ASLR) is a technique used by operating systems to load
binary segments at semi-random places in the process's address space. It is used
by Fuchsia and many other OS to hinder attacks which rely on knowing where code
or other data is in memory.  Making code unreadable further reduces the attack
surface. -->

许多攻击方式依赖于读取代码页来找出 gadget 或者说感兴趣的可执行代码，进而收集关于进程的信息。地址空间
布局随机化（ASLR）是一种将二进制段加载到进程地址空间中半随机的位置的操作系统技术。Fuchsia 和
许多其他操作系统利用这种技术来防范依赖于知晓代码或数据在内存中的位置的攻击。让代码不再可读更加
减小了攻击面。
<!-- 
Code reuse attacks, like “return-to-libc” [rtl-attack], are used to return
control of a function to a known address. libc is a logical choice to return or
jump into since it contains rich functionality useful to an attacker, and
because it is extremely likely the process will link against libc. It has been
demonstrated that the available gadgets in a typical program are
Turing-complete, giving an adversary the ability to execute arbitrary code. -->

代码复用攻击，如 return-to-libc [rtl-attack]，被用来将函数控制流返回到已知地址。libc 常常
成为返回或跳转的目的地，因为其包含对攻击者有用的丰富功能，并且进程极有可能会链接 libc。人们已经证明，
典型程序中的可用 gadget 是图灵完全的。这赋予了攻击者执行任意代码的能力。
<!-- 
In many cases an adversary's objective is to obtain a shell. ASLR makes these
kinds of attacks harder because the addresses of functions are different between
invocations of a program. However, ASLR isn’t a comprehensive mitigation,
because attackers can read code pages to find the address of functions that they
would otherwise not know by looking at their address in the binary. XOM makes it
impossible for ASLR to be broken in this way and attackers will need to use
another way to find out information about the location of specific code pages. -->

许多时候攻击者的目标是拿到 shell。ASLR 给这些攻击增加了难度，因为每次运行程序时函数地址都不同。
然而，ASLR 并不是完美的解决方案。攻击者虽然不能通过分析二进制找到函数地址，但仍可以
分析内存中的代码页来达到目的。XOM 使得攻击者无法以这种方式绕过 ASLR。
欲找出特定代码页中位置信息的攻击者将被迫另寻他法。

<!-- ### Common Notation -->
### 通用记号

<!-- #### ‘rwx/r-x/–x’ -->
#### ‘rwx/r-x/–x’
<!-- 
These represent permissions of ELF segments, which get mapped into the processes
address space with the corresponding permissions. This notation is used commonly
both when describing permissions of files, as well as ELF segments by tools like
`readelf`. r, w and x mean read, write and execute respectively and ‘-’ means
the permission is not granted. An execute-only segment will have ‘--x’
permissions. -->

这些记号表示 ELF 的段的权限。段按照对应权限被映射到进程地址空间。这种记号在描述文件权限和 `readelf` 之类的
工具描述 ELF 的段权限时是通用的。r, w 和 x 分别表示读、写和执行，‘-’ 表示对应权限未授予。
仅执行段的权限表示为 ‘--x’。

<!-- #### R^X, W|X, etc… -->
#### R^X, W|X 等等…
<!-- 
As above, R, W and X refer to read, write and execute. ‘^’ and ‘|’ are C-like
operators for xor and or. R^X is read as “read xor execute”. -->

如前所述，R，W 和 X 指的是读、写和执行。‘^’ 和 ‘|’ 是 C 风格的操作符，意为“异或”和“或”。
R^X 读作 “读异或执行”。

<!-- #### "ax" -->
#### "ax"

<!-- 
This is assembler syntax which marks a section as allocated and executable.
Currently linkers will put “ax” sections into segments that are ‘r-x’. The
`--execute-only` flag in lld will mark these segments as ‘--x’ instead.
 -->
这是汇编中的一种标记，其将一个 section 标记为需分配内存且可执行。链接器目前会
将 “ax” 的 section 放进 ‘r-x’ 的段里，而 lld 中的 `--execute-only` 标志会将这些段
标记为 ‘--x’。

<!-- ## Design -->
## 设计

<!-- 
To increase security of our userspace programs by supporting XOM, both our
toolchain and loaders will need to be updated. The clang driver will need to
pass the ‘--execute-only’ flag to the linker to ensure “ax” sections which would
otherwise be mapped to ‘r-x’ segments are instead mapped to ‘--x’ segments. The
loaders will also need to change the sanity checks that all requested
permissions contain at least read, because this will no longer be true.
 -->
为了支持 XOM，提高我们用户空间程序的安全性，我们的工具链和加载器都需要升级。clang driver 需要
给链接器传递 ‘--execute-only’ 标志来让 “ax” 的 section 映射到 ‘--x’ 段而不是 ‘r-x’ 段。
加载器也需要修改那些要求至少有读权限的 sanity check，因为现在不一定有读权限了。

<!-- 
As it will only be possible to use XOM on hardware that has ePAN, we will need
to gracefully support the transition. We have two options:
 -->
由于只有在支持 ePAN 的硬件上才能启用 XOM，我们需要支持平滑过渡。我们有如下选项：

<!-- 
1. Change `vmar_*` functions to be best effort like many `mmap` implementations
1. Create a way to query the kernel if it supports execute-only mappings and
have the loader escalate permissions of a ‘--x’ segment to ‘r-x’ if XOM is not
available.
1. Add a new `ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` flag for loaders to use with
‘--x’ segments.
 -->
1. 将 `vmar_*` 系列函数改成跟很多 `mmap` 实现一样的尽力而为。
1. 创造一种查询内核是否支持仅执行映射的方法，并在 XOM 不可用时让加载器将 ‘--x’ 段的权限提升
到 ‘r-x’。
1. 加入新的 `ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` 标志来让加载器使用 ‘--x’ 段。

<!-- 
In all cases, there will be a potential silent escalation of permissions. The
first option would be the easiest, the loaders would need no changes other than
removing their sanity checks. The second option is not significantly more
complex, it just would add a simple check in the loaders before deciding what
memory permissions to request from the OS. The third option is helpful because
it is less error prone in user code.
 -->
无论怎么选，都有潜在的静默提权问题。第一种选项最容易实现，加载器除了移除 sanity check 外
什么都不用改。第二种选项也没有复杂多少，只用在加载器决定向操作系统请求哪些内存权限前
加一个简单的判断。第三种选项很有帮助，因为它在用户代码中更不容易出错。

<!-- 
The first option would end up breaking Fuchsia’s current strict contract with
userspace of always being explicit about what a syscall can and cannot honor.
The 2nd and 3rd option also end up with ambiguous handling of memory permissions
when loading ELF files. However this fits within the ELF specification. Segment
permissions don’t specify 1:1 what permissions the memory allocated for a
segment will have, but rather which permissions the memory must at least have
for the program to operate correctly. ELF loaders are within their rights to map
a ‘--x’ segment into ‘r-x’ memory [elf-segment-perm].
 -->
第一种选项会造成对 Fuchsia 目前与用户空间的严格约定的破坏，约定要求对系统调用能满足哪些用户请求
必须明确表达。第二和第三种选项也会导致加载 ELF 文件时对内存权限的处理产生歧义。然而这是符合 
ELF 规范的。段权限并不是说分配给这个段的内存只能有这些权限，而是说分配的内存必须至少有这些权限
程序才能正常运行。ELF 加载器要把 ‘--x’ 的段映射进 ‘r-x’ 的内存也在权力范围内 [elf-segment-perm]。

<!-- 
The first option of breaking Fuchsia’s current contract of explicit syscall
handling isn’t ideal. Both option 2 and 3 have value and the implementation
proposed in this RFC will be based on both options.
 -->
第一种选项会对 Fuchsia 当前明确表达系统调用的处理方式的约定造成破坏，并不理想。选项 2 和 3 
都有价值，本 RFC 提出的实现将基于这两种选项。

<!-- ## Implementation -->
## 实现

<!-- ### System Call Additions -->
### 系统调用新增

<!-- 
A new flag `ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` will be added which will make
the various `zx_vmar_*` syscalls which take a permissions flag in `options`
which will implicitly add read permission if XOM is not supported.
`ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` is logically only useful with
`ZX_VM_PERM_EXEC` and not `ZX_VM_PERM_READ`, however the various syscall which
accept this flag will not be treating this as an invariant. It is safe to have
`ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` with any other combination of flags, it
will just be treated as `ZX_VM_PERM_READ` in contexts where the system
cannot map execute-only pages.
 -->
将添加新标志 `ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED`，其会使在 `options` 中接收
权限标志的各种 `zx_vmar_*` 系统调用在 XOM 不受支持的情况下隐式添加读权限。按照逻辑，
`ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` 只能与 `ZX_VM_PERM_EXEC` 一起使用，不能
与 `ZX_VM_PERM_READ` 一起使用。然而接收该标志的各种系统调用并不会处理得这么死板。
`ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` 可以安全地与任意其他标志组合，在系统不能映射
仅执行页的情况下它只会被当作 `ZX_VM_PERM_READ`。

<!-- 
A new `kind` value `ZX_FEATURE_KIND_VM` will be added for
`zx_system_get_features`, which will yield a bitset similar to
`ZX_FEATURE_KIND_CPU`. There will also be a new feature
`ZX_VM_FEATURE_CAN_MAP_XOM`. The current implementation will always keep this
bit false because XOM will not be enabled until later. This will not be used by
the loaders because ‘r-x’ memory permissions are valid for a ‘--x’ segments, but
is still important for userspace to be able to query for this functionality.
 -->
将为 `zx_system_get_features` 添加新 `kind` 值 `ZX_FEATURE_KIND_VM`，其会返回
与 `ZX_FEATURE_KIND_CPU` 类似的 bitset。也会有一个新特性 `ZX_VM_FEATURE_CAN_MAP_XOM`。
目前的实现总会保持这个位为假，因为 XOM 目前暂不会启用。加载器不会使用这个，因为 ‘r-x’ 内存
权限对于 ‘--x’ 段也是有效的，但让用户空间能够查询这一功能仍然很重要。

<!-- ### System Loader ABI Changes -->
### 系统加载器 ABI 更改

<!-- 
Current and future loaders will ensure '--x' segments can be loaded into memory
even if the target can't support XOM. The loaders will add
`ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` when mapping execute-only segments.
 -->
目前和以后的加载器会保证即使在硬件不支持 XOM 的情况下 '--x' 段也能加载进内存。
加载器在映射仅执行段时会添加 `ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED`。

<!-- ### Shipped Dynamic Linker ABI Changes -->
### 自带动态链接器 ABI 更改

<!-- 
Similarly, the dynamic linker in Fuchsia’s libc shipped with the SDK will also
escalate permissions where necessary when allocating memory for  ‘--x’ segments
with `ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED`.
 -->
相似地，Fuchsia 的 SDK 自带的 libc 中的动态链接器在为带有 
`ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` 的 ‘--x’ 段分配内存时，也会在必要的时候提权。

<!-- ### Compiler Toolchain Changes -->
### 编译器工具链更改
<!-- 
The clang driver will also be changed to always pass `--execute-only` to the
linker when targeting `aarch64-*-fuchsia`. We will also need a way to opt out of
this behavior, most likely by adding a new ‘--no-execute-only’ flag to the
linker, so programs can easily opt out of the new default behavior.
 -->
clang driver 也会改为在目标为 `aarch64-*-fuchsia` 时总是向链接器传递 
`--execute-only`。我们也将需要一种退出这种行为的方法，最可能的是向链接器添加一个新的 
‘--no-execute-only’ 标志，这样程序可以轻易退出新的默认行为。

<!-- ### Kernel XOM Implementation -->
### 内核 XOM 实现

<!-- 
Once hardware arrives that supports ePAN, the kernel can service a request for
memory pages to have just `ZX_VM_PERM_EXECUTE`. The arm64 user-copy
implementation may need updates to ensure it's consistent with how user memory
access is constrained. `user_copy` should be updated to use the `ldtr` and
`sttr` instructions. This will ensure that users cannot trick the kernel to read
unreadable pages for them. Moreover, the kernel makes assumptions about mappings
being readable in a couple of places and these will need to be changed where
appropriate. This work will be done later.
 -->
一旦支持 ePAN 的硬件到来，内核就可以服务于那些对只有 `ZX_VM_PERM_EXECUTE` 的内核页的请求。
arm64 的 user-copy 实现也需要更新，以保证其与用户内存的约束方式保持一致。`user_copy` 应该被更新
以使用 `ldtr` 和 `sttr` 指令。这将确保用户不能欺骗内核为他们读取不可读的页面。此外，内核在一些地方
假设了映射总是可读，这些也需要进行适当的修改。这项工作将在以后完成。

<!-- ### Unnecessary Changes -->
### 不必要的更改

<!-- 
`zx_process_read_memory` does not need to be changed, and debuggers should work
normally when debugging execute-only binaries. `zx_process_read_memory` ignores
the permissions of the pages it is reading from, and only checks that the
process handle has `ZX_RIGHT_READ` and `ZX_RIGHT_WRITE`.
 -->
`zx_process_read_memory` 不需要更改，调试器在调试仅执行二进制时也应该正常工作。
`zx_process_read_memory` 忽略了它所读取的页面的权限，只检查进程句柄是否有
`ZX_RIGHT_READ` 和 `ZX_RIGHT_WRITE`。

<!-- 
`zx_vmar_protect` will continue to work as it does currently. Most notably this
means that processes can protect their code pages with read permission in cases
where that is necessary.
 -->
`zx_vmar_protect` 将继续像目前那样工作。最值得注意的是，这意味着必要时，
进程可以用读取权限保护他们的代码页。

<!-- ## Performance -->
## 性能

<!-- There is no expected impact in performance. -->
预计对性能没有影响。

<!-- ## Security -->
## 安全

<!-- 
Until XOM is implemented in the kernel a binary with ‘--x’ segments will be just
as secure as an equivalent binary using ‘r-x’ segments. Once XOM is supported
both by hardware and the OS, programs which elect to use execute-only memory
will become more secure. See sections [Permissions of Code
Pages](#permissions-of-code-pages), [XOM and PAN](#xom-and-pan) and [Readable
Code Security](#readable-code-security).
 -->
在 XOM 在内核中实现之前，使用 ‘--x’ 段的二进制文件只会与使用 ‘r-x’ 段的等效二进制文件一样安全。
一旦硬件和操作系统都支持了 XOM，选择使用仅执行内存的程序将变得更加安全。参见
[代码页权限](#permissions-of-code-pages)，[XOM 与 PAN](#xom-and-pan)和
[可读代码安全](#readable-code-security) 这几节。

<!-- ## Privacy -->
## 隐私

<!-- No extra considerations other than those mentioned in [Security](#security). -->
除了在 [安全](#security) 中提到的以外无需额外考虑。

<!-- ## Testing -->
## 测试

<!-- 
`zx_system_get_features` will have trivial testing when we are forcing XOM
support in the kernel where we can know at build time what we expect the
syscall to return.
 -->
当我们在内核中强制对XOM的支持时，`zx_system_get_features` 会有一些 trivial 的测试，
针对那些在构建时就知道系统调用应该返回什么的情况。

<!-- 
The `ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` will be tested that it makes a page
readable when it is reported by `zx_system_get_features` that the OS cannot
create execute-only pages.
 -->
会有针对 `ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` 的测试，测试其能否在
`zx_system_get_features` 报告操作系统无法创建仅执行页时使内存页可读。

<!-- 
Likewise, the elfload library doesn't have any real testing, save for fuzz tests
which don't test expected functionality. Instead its functionality is inherently
tested by other components that rely on it. Testing should be added here to
ensure '--x' segments are correctly mapped. The process_builder library does
have tests, and these will ensure it properly requests readable and executable
memory when XOM is not available.
 -->
类似地，elfload 库也没有任何真正的测试，除了模糊测试，但它并不测试预期功能。
相反，它的功能是由依赖它的其他组件来测试的。这里应该增加测试，以确保 '--x' 段被正确映射。
process_builder 库确实有测试，这些测试将确保它在 XOM 不可用时正确请求可读和可执行内存。

<!-- 
The changes to the current dynamic linker will not be tested directly. A new
dynamic linker is planned and it will have extensive testing, including testing
of ‘--x’ segments.
 -->
对当前动态链接器的改变将不会被直接测试。一个新的动态链接器正在计划中，它将有广泛的测试，
包括对 '--x' 段的测试。

<!-- The changes to the clang driver will have testing in upstream LLVM. -->
对 clang driver 的更改会在上游 LLVM 中得到测试。

<!-- 
We will also set up testing configuration for enabling XOM on test bots, even if
that hardware does not have ePAN and we would otherwise not enable XOM. This
will help us catch in tree programs that read their code pages and need to opt
out of execute-only.
 -->
我们也会设置测试配置，来在 test bot 上启用 XOM，虽然我们通常不会在那种没有 ePAN 的硬件上启用 XOM。
这将有助于我们找到那些需要阅读代码页，从而需要退出只执行的 in-tree 程序。

<!-- ## Documentation -->
## 文档

The changes to `zx_system_get_features` will be documented, as well as the
motivation for why user space would want to query with the kind
`ZX_VM_FEATURE_CAN_MAP_XOM`. Likewise the new
`ZX_VM_PERM_READ_IF_XOM_UNSUPPORTED` flag will also be documentated. Changes to
the various loaders and the clang driver defaults will not be documented outside
of this RFC.

## Drawbacks, Alternatives, Unknowns

It is unknown how much current and future out of tree code relies on executable
code being readable. This could be from use of data constants in text from
handwritten assembly, code compiled from other toolchains or program
introspection. Regardless, programs which need to have readable code pages, will
still benefit because their shared library dependencies, including libc, will be
marked execute only. Changing our clang toolchain to default to execute-only
segments will break programs which depend on readable code. There is no easy way
to check at build time if a program relies on this behavior. However once it is
identified that a program needs ‘r-x’ segments, opting out of the default ‘--x’
will be simple.

For programs which need to be able to read some of their code but not all,
current tooling cannot easily support this. The `--execute-only linker` flag
will strip read permissions from any executable segment, and there is no way to
mark a single section as needed to be read. Programs which want this behavior
will need to opt out of execute-only completely.

## Risks

It is possible that the clang driver defaults to using `--execute-only` and code
that reads from a ‘--x’ segment won’t be broken until hardware and kernel
support for XOM lands. This creates potential forward compatibility problems for
software that didn’t change. Testing will exist for in tree software, but most
likely not for out of tree code.

## Prior Art and References

Because of the ambiguous handling of `mmap` permission flags in many POSIX
implementations, they have no need for an analogue to
`zx_system_get_features(ZX_FEATURE_KIND_CAN_MAP_XOM, &feature)`.

Darwin supports XOM on newer Apple chips, but their implementation is more
robust using proprietary hardware features. Their chips have hardware support
for stripping individual permission bits from both kernel and user memory. It is
not enabled for userspace in macOS. [apple-xom]

[example-fuchsia-test]: https://source.corp.google.com/fuchsia/zircon/system/utest/core/memory-mapping/memory-mapping.cc;l=126
[openbsd-wxorx]: https://www.openbsd.org/33.html
[selinux-wxorx]: https://akkadia.org/drepper/selinux-mem.html
[clang-example]: https://godbolt.org/z/hGzr49qYs
[android-xom]: https://source.android.com/devices/tech/debug/execute-only-memory
[elf-segment-perm]: https://www.sco.com/developers/gabi/latest/ch5.pheader.html
[posix-mmap]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/mmap.html
[rtl-attack]: https://dl.acm.org/doi/10.1145/1315245.1315313
[pan-fxb]: https://fxbug.dev/59284
[pan-issue]: https://blog.siguza.net/PAN/
[linux-revert]: https://github.com/torvalds/linux/commit/cab15ce604e550020bb7115b779013b91bcdbc21
[linux-re-land]: https://github.com/torvalds/linux/commit/18107f8a2df6bf1c6cac8d0713f757f866d5af51
[apple-xom]: https://i.blackhat.com/USA-19/Thursday/us-19-Krstic-Behind-The-Scenes-Of-IOS-And-Mas-Security.pdf
