<!-- 
# Bringup Product Definition
 -->
# Bringup 产品定义

<!-- 
The `bringup` product is the most minimal viable target for development. It is
a minimal feature set product that is focused on being very simple and very lean.

Note that the name `bringup` build should not imply “only used during bringup of
a new platform”, the name is historical.
 -->
`bringup` 产品是开发的最小可行目标。它是一个最小特性集合产品，侧重于变得非常简单、非常高效。

<!-- 
The bringup product serves at least these purposes:

1. Bringup: When a new platform is not running Fuchsia yet (`core` product
   configuration or higher) because all the pieces necessary to run are not
   completed/work-reliable, for instance networking, storage or configurations
   needed for fx device discovery and package management.
2. Kernel and low level driver development: Developing facilities that need to
   be working to even try a `core` product requires a bringup build. This applies
   to kernel development and drivers like networking and storage that are needed
   in `core`. Note that higher level drivers like audio also can benefit from
   bringup builds, when the drivers needed for core are not ready yet.
 -->
bringup 产品至少满足以下用途：

1. 适配工作（bringup）：当新平台由于所有必需的部分尚未完成或可靠工作（例如网络、存储器或 fx 设备发现所需配置和包管理），而无法运行 Fuchsia 时（`core` 或更高级产品配置）。
2. 内核和低级（low level）驱动开发：开发需要正常运作的甚至是在 `core` 产品中运行的功能需要 bringup 产品。这应用于内核开发和在 `core` 中需要的驱动（如网络和存储器）。注意，当 core 所需的驱动尚未就绪时，例如音频的高级驱动也能从 bringup 构建中获益。

<!-- 
A bringup build has these basic features:

1. Has serial output enabled: This includes debug logging from drivers (for
   instance via zxlogf). This must guarantee that developers bringing up new
   platforms are able to printf debug as needed.
1. Is RAM loadable: It must be possible to load into RAM a bringup build in
   order of preference:
   1. For those platforms that support 'fastboot boot' it must be possible to
   implement RAM booting a ZBI directly from the bootloader (for example using
   the bootshim mechanism).
   1. For those platforms that do not support 'fastboot boot' it must be
   possible to boot using an existing zedboot (for instance loaded in a bootable
   USB stick or previously flashed) with a mechanism like netsvc (used for
   netbooting) or overnet (for instance over serial).
   1. For those platforms that do not support 'fastboot boot' (for instance when
   there is no control over the bootloader) it must also be possible to
   implement RAM booting a ZBI directly from the bootloader (for example by
   creating a bootshim for the specific bootloader).
1. Does not have dependencies on drivers not available in early bringup:
   Examples of drivers available in early bringup include interrupt controllers
   and serial port. Examples of drivers not available in early bringup include
   networking and storage.
1. Has minimum dependencies on Fuchsia at large, in that it:
   1. Has workflows driven over a serial link.
   1. Allows for everything needed in the build to be loaded alongside the
   kernel (i.e. in bootfs).
   1. Does not depend on Fuchsia features like paving that require storage.
   1. Does not support fx commands such as fx serve and fx shell. As a result,
      bringup builds are not able to add new software at runtime or upgrade
      themselves.
1. Allows for easy inclusion of additional drivers or binaries: It must be
   possible to include additional binaries and drivers in the bringup build. For
   instance through inclusion into bootfs via GN to add a driver in development
   to the build.
 -->
bringup 构建拥有这些基本特性：

1. 启用了串行输出：这包括来自驱动的调试日志记录（比如通过 zxlogf）。这必须保证适配新平台的开发人员能够根据需要进行 printf 调试。
1. 可加载到内存（RAM）：必须可以按照优先次序将 bringup 构建加载到内存中。
   1. 对于支持“fastboot 引导”的平台，必须能够直接从引导加载器（bootloader）实现内存直接引导 ZBI 文件（例如使用 bootshim 机制）。
   1. 对于不支持“fastboot 引导”的平台，必须能够通过诸如 netsvc（用于网络引导）或 overnet（用于通过串行示例）的机制使用现有 zedboot 进行引导（例如加载到 U 盘中或事先刷入）。
   1. 对于不支持“fastboot 引导”的平台（比如无法控制引导加载器时），也必须能够从引导加载器实现内存直接引导 ZBI 文件（例如通过为特定引导加载器创建 bootshim 的方式）。
1. 不依赖于早期 bringup 中不可用的驱动：早期 bringup 中可用的驱动有中断控制器、串行端口等。早期 bringup 中不可用的的驱动有网络和存储器等。
1. 总体上对 Fuchsia 的依赖性最小，其中它：
   1. 拥有通过串行链接驱动的工作流程。
   1. 考虑了在构建中除了内核另外的（即在 bootfs 中）所有需要加载的内容。
   1. 不依赖于 Fuchsia 的特性，例如 paving 需要存储器。
   1. 不支持 fx 命令，诸如 fx serve 和 fx shell。因此，bringup 构建不能在运行时添加新软件，也不能自我升级。
1. 考虑了简单地包括附加驱动或二进制文件：在 bringup 构建中必须能够包括附加二进制文件和驱动。例如通过 GN 添加包含项到 bootfs 中以向构建添加开发中的驱动。

<!-- 
Note that these features do not prevent the possible expansion of the bringup
build minimal configuration to other more complete configurations that allow for
improved workflows.
 -->
注意，这些特性不能阻止为改善的工作流考虑而将 bringup 构建的最小化配置扩展成其他更加完善配置的可能。
