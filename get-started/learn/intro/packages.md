<!-- # Software delivery -->
# 软件交付

<<../../_common/intro/_packages_intro.md>>

<<../../_common/intro/_packages_serving.md>>

<<../../_common/intro/_packages_storing.md>>

<!-- 
## Exercise: Packages
 -->
## 练习：包

<!-- 
So far in this codelab, you've been experiencing on demand software delivery
to your device and you probably didn't even know it! In this exercise, you'll
peel back the covers and see the details of how packages are delivered and stored
on a Fuchsia device.
 -->
在之前的本系列代码练习中，虽然您可能没有意识到，但您已经体验过到设备的软件按需交付。在本练习中，您将进一步揭开它的神秘面纱，了解到包被交付和存储到 Fuchsia 设备中的过程的具体细节。

<<../_common/_restart_femu.md>>

<!-- 
### Start a local package server
 -->
### 启动本地包服务器

<!-- 
Run the following command to start a package server and enable the emulator to
load software packages:
 -->
运行如下命令，以启动包服务器并允许模拟器加载软件包：

```posix-terminal
fx serve-updates
```
<!-- 
The command prints output similar to the following, indicating the server is
running and has successfully registered the emulator as a target device:
 -->
该命令打印的输出类似如下所示。它表示服务器正在运行，并已成功将模拟器注册为目标设备：

```none {:.devsite-disable-click-to-copy}
[serve-updates] Discovery...
[serve-updates] Device up
[serve-updates] Registering devhost as update source
[serve-updates] Ready to push packages!
[serve-updates] Target uptime: 139
[pm auto] adding client: [fe80::5888:cea3:7557:7384%qemu]:46126
[pm auto] client count: 1
```

<!-- 
### Examine the package server
 -->
### 检查包服务器

<!-- 
The `fx serve-updates` command runs a **local package server** used to deliver
packages to the target devices. By default, this server runs at on port 8083.
 -->
`fx serve-updates` 命令会启动用于将包交付到目标设备的 **本地包服务器**。该服务器默认在 8083 端口上运行。

<!-- 
Open a browser to `http://localhost:8083`. This loads an HTML page listing all
the packages currently available in the package repository. Each one of these
are packages that can be delivered to the device.
 -->
在浏览器中访问 `http://localhost:8083`，会加载出一个当前包仓库中可用的包的列表的网页。列表中的每一项都是可被交付到设备的包。

<!-- 
### Monitor package loading
 -->
### 监视包加载

<!-- 
Packages are resolved and loaded on demand by a Fuchsia device. Take a look at
this in action with the `spinning-square` example package.
 -->
包由 Fuchsia 设备按需解析并加载。让我们在 `spinning-square` 示例包上进行操作来稍作展示。

<!-- 
From the device shell prompt, you can confirm whether a known package is
currently on the device:
 -->
在设备命令行中，您可以确定某个已知的包目前是否在设备上。

```posix-terminal
fx shell pkgctl pkg-status fuchsia-pkg://fuchsia.com/spinning-square-rs
```

```none {:.devsite-disable-click-to-copy}
Package in registered TUF repo: yes (merkle=ef65e2ed...)
Package on disk: no
```

<!-- 
Open a new terminal and begin streaming the device logs for `pkg-resolver`:
 -->
打开新终端，开始流式传输 `pkg-resolver` 的设备日志。

```posix-terminal
ffx log --filter pkg-resolver
```

<!-- 
This shows all the instances where a package was loaded from the package
server.
 -->
这里展示了从包服务器加载的包的所有实例。

<!-- 
From the device shell prompt, attempt to resolve the package:
 -->
在设备命令行中，尝试解析包：

```posix-terminal
fx shell pkgctl resolve fuchsia-pkg://fuchsia.com/spinning-square-rs
```

<!-- 
Notice the new lines added to the log output for `pkg-resolver`:
 -->
注意 `pkg-resolver` 的输出中增加的新行：

```none {:.devsite-disable-click-to-copy}
[pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://devhost/spinning-square-rs: [
    6b547fb59fda56866eea01cda90add0aabc1af7c7418c7850011ec6e99a996f1,
    7c1a9fd1c11e9b6b1d3c3184cf350cecfc91ec083b412d20c18b5187d0983d88,
]
[pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://fuchsia.com/spinning-square-rs as fuchsia-pkg://devhost/spinning-square-rs to 21967ecc643257800b8ca14420c7f023c1ede7a76068da5faedf328f9d9d3649 with TUF
```

<!-- 
From the device shell prompt, check the package status again on the device:
 -->
在设备命令行中，再次在设备上查看包状态：

```posix-terminal
fx shell pkgctl pkg-status fuchsia-pkg://fuchsia.com/spinning-square-rs
```

```none {:.devsite-disable-click-to-copy}
Package in registered TUF repo: yes (merkle=ef65e2ed...)
Package on disk: yes (path=/pkgfs/versions/ef65e2ed...)
```

<!-- 
Fuchsia resolved the package and loaded it from the local TUF repository on
demand!
 -->
Fuchsia 解析了包并按需从本地 TUF 仓库中将其加载了！

<!-- 
### Explore package metadata
 -->
### 探索包的元数据

<!-- 
Now that the `spinning-square` package has successfully been resolved, you can
explore the package contents. Once resolved, the package is referenced on the
target device using its content address.
 -->
现在 `spinning-square` 包已经成功解析，接下来可以探索包内容。在解析之后，包就可以在目标设备上用它的内容地址引用。

<!-- 
From the device shell prompt, use the `pkgctl get-hash` command to determine the
package hash for `spinning-square`:
 -->
在设备命令行中，可以使用 `pkgctl get-hash` 命令来确认 `spinning-square` 的包摘要。

```posix-terminal
fx shell pkgctl get-hash fuchsia-pkg://fuchsia.com/spinning-square-rs
```

<!-- 
The command returns the unique package hash:
 -->
该命令会返回如下的唯一包摘要：

```none {:.devsite-disable-click-to-copy}
ef65e2ed...
```

<!-- 
Provide the full package hash to the `pkgctl open` command to view the package
contents:
 -->
使用 `pkgctl open` 命令并提供完整的包摘要，来查看包内容。

```posix-terminal
fx shell pkgctl open {{ '<var>' }}ef65e2ed...{{ '</var>' }}
```

```none {:.devsite-disable-click-to-copy}
opening ef65e2ed...
package contents:
/bin/spinning_square
/lib/VkLayer_khronos_validation.so
/lib/ld.so.1
/lib/libasync-default.so
/lib/libbackend_fuchsia_globals.so
/lib/libc++.so.2
/lib/libc++abi.so.1
/lib/libfdio.so
/lib/librust-trace-provider.so
/lib/libstd-e3c06c8874beb723.so
/lib/libsyslog.so
/lib/libtrace-engine.so
/lib/libunwind.so.1
/lib/libvulkan.so
/meta/contents
/meta/package
/meta/spinning-square-rs.cm
/meta/spinning-square-rs.cmx
/data/fonts/RobotoSlab-Regular.ttf
/meta/fuchsia.abi/abi-revision
/data/vulkan/explicit_layer.d/VkLayer_khronos_validation.json
```

<!-- 
This lists the package metadata and each of the content BLOBs in the package.
You can see `bin/` entries for executables, `lib/` entries for shared library
dependencies, additional metadata and resources.
 -->
这个命令列出了包的元数据和包中的每个内容 BLOB（Binary Large OBject，二进制大型对象）。其中的 `bin/` 项目对应可执行文件，`lib/`
项目对应共享库依赖，还有后面的额外元数据和资源。

<!-- 
## What's Next?
 -->
## 接下来是？

<!-- 
Congratulations! You now have a better understanding of what makes Fuchsia
unique and the goals driving this new platform's design.
 -->
祝贺！您现在对 Fuchsia 的独特性和这样一个新平台的设计目标都有了更好的理解。

<!-- 
In the next module, you'll learn more about the Fuchsia open source project and
the tools used to build and customize the system:
 -->
在下一个模块中，您将了解到有关 Fuchsia 这个开源项目和用于构建与自定义系统的工具的更多知识：

<a class="button button-primary"
    href="/get-started/learn/build">构建 Fuchsia</a>
