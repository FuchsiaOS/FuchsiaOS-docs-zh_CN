<!-- 
# Prebuilt CIPD packages in Fuchsia
 -->
# Fuchsia 中的预构建 CPID 包

<!-- 
The Fuchsia project uses Chrome Infrastructure Package Deployment
([CIPD](https://github.com/luci/luci-go/tree/HEAD/cipd){: .external}) to store
and distribute prebuilt files.

Note: A CIPD store is not a package server for Fuchsia devices. In other words,
a Fuchsia device doesn't run components from prebuilt CIPD packages.
 -->
Fuchsia 项目使用了 Chrome 基础设施包部署（Chrome Infrastructure Package Deployment，[CIPD](https://github.com/luci/luci-go/tree/HEAD/cipd){: .external}）来存储和分发预构建文件。

注意：CIPD 存储不是 Fuchsia 设备的包服务器。也就是，Fuchsia 设备不从预构建的 CIPD 包中运行组件。

<!-- 
A CIPD package is an arbitrary collection of files, stored in
[a remote content-addressed store](https://chrome-infra-packages.appspot.com/p/fuchsia){: .external},
and is distributed to a Fuchsia checkout through the
<code>[jiri](https://fuchsia.googlesource.com/jiri/+/HEAD/){:.external}</code>
tool. Also, using the `cipd` command-line tool, you can download a CIPD package
directly, for example, to examine its content.
 -->
CIPD 包是文件的一个任意集合，它保存在[远程的内容寻址存储](https://chrome-infra-packages.appspot.com/p/fuchsia){: .external}上，并通过 <code>[jiri](https://fuchsia.googlesource.com/jiri/+/HEAD/){:.external}</code> 分发至 Fuchsia checkout。同样，使用 `cipd`命令行工具，您可以直接下载 CIPD 包，例如，用来检查其内容。

<!-- 
CIPD packages are typically used to distribute the following files:

*   Host prebuilt binaries required by the build (for example, clang toolchain).
*   Fuchsia prebuilt
    [ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format){: .external}
    binaries generated out-of-tree (for example, Goldfish Vulkan ICD).
*   Prebuilt Fuchsia archive
    ([FAR](/docs/concepts/source_code/archive_format.md)) files that contain
    binaries and metadata for software that is built for Fuchsia by other teams
    (for example,
    [chromium/fuchsia/webrunner-arm64](https://chrome-infra-packages.appspot.com/p/chromium/fuchsia/webrunner-arm64/+/){: .external}).
 -->
CIPD 包通常用于分发下列文件：

*   构建所需的主机预构建二进制文件（例如，clang 工具链）。
*   生成于树外的 Fuchsia 预构建 [ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format){: .external} 二进制文件（例如，Goldfish Vulkan ICD）。
*   预构建 Fuchsia 归档（[FAR](/docs/concepts/source_code/archive_format.md)）文件，其中包含了用于由其他团队构建的面向 Fuchsia 的软件的二进制文件和元数据（例如，[chromium/fuchsia/webrunner-arm64](https://chrome-infra-packages.appspot.com/p/chromium/fuchsia/webrunner-arm64/+/){: .external}）。

<!-- 
Once you set up continuous integration (CI) with Fuchsia, Fuchsia’s CI system
fetches those new packages and rolls them into the Fuchsia project through the
[global integration](https://fuchsia.googlesource.com/integration/+/refs/heads/master)
process.

<a name="figure-1"></a> <figure>
<img src="/docs/images/prebuilt_packages/publish-prebuilt-packages-to-fuchsia-00.png" alt="The latest ref and other refs shown in the CIPD UI">
<figcaption><b>Figure 1</b>. The CIPD UI shows the latest ref and other refs
used for this CIPD package instances.</figcaption> </figure>
 -->
一旦您使用 Fuchsia 设置了持续集成（continuous integration，CI），Fuchsia 的 CI 系统会取得新包并通过[全局整合](https://fuchsia.googlesource.com/integration/+/refs/heads/master)进程将它们整合进 Fuchsia 项目。

<a name="figure-1"></a> <figure>
<img src="/docs/images/prebuilt_packages/publish-prebuilt-packages-to-fuchsia-00.png" alt="CIPD 用户界面的最新参考实现和其他参考实现">
<figcaption><b>图 1</b>. 该 CIPD 用户界面展示了用于该 CIPD 包实例的最新参考实现和其他参考实现。</figcaption> </figure>

<!-- 
When you publish a new revision of your prebuilt package to CIPD, the `latest`
[ref](https://github.com/luci/luci-go/tree/HEAD/cipd#refs){: .external} in the
CIPD store automatically points to the new revision. Fuchsia’s CI system
monitors your package’s `latest` ref. When it detects that the `latest` ref is
updated, the system fetches the new package and rolls it into the Fuchsia
project.
 -->
当您向 CIPD 发布您预构建包的新版本时，CIPD 商店中的最新[参考实现](https://github.com/luci/luci-go/tree/HEAD/cipd#refs){: .external}（`latest` ref）会自动指向新版。Fuchsia 的 CI 系统监视您包的 `latest` 参考实现。当它检测到 `latest` 参考实现更新时，系统会取得新包并将它整合进 Fuchsia 项目。

