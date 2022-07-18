# Fuchsia Rust Crates
<!--
* [carnelian/](/src/lib/ui/carnelian)

    A user-interface framework in Rust.

* [fdio/](/src/lib/fdio/rust/)

    Wrapper over zircon-fdio library

* [fuchsia-archive/](/src/sys/pkg/lib/far/rust/)

    Work with Fuchsia Archives (FARs)

* [fuchsia-async/](/src/lib/fuchsia-async/)

    Fuchsia-specific Futures executor and asynchronous primitives (Channel, Socket, Fifo, etc.)

* [fuchsia-framebuffer/](/src/lib/ui/fuchsia-framebuffer/)

    Configure, create and use FrameBuffers in Fuchsia
-->

* [carnelian/](/src/lib/ui/carnelian)

    一种 Rust UI 框架

* [fdio/](/src/lib/fdio/rust/)

    zircon-fdio 封装库

* [fuchsia-archive/](/src/sys/pkg/lib/far/rust/)

    Fuchsia 归档 (FARs)

* [fuchsia-async/](/src/lib/fuchsia-async/)

    Fuchsia 特定的功能执行器和异步原语 (Channel, Socket, Fifo, 等)

* [fuchsia-framebuffer/](/src/lib/ui/fuchsia-framebuffer/)

    在 Fuchsia 上配置， 生成和使用帧缓冲

<!--
* [fuchsia-merkle/](/src/sys/pkg/lib/fuchsia-merkle/)

    Protect and verify data blobs using [Merkle Trees](/docs/concepts/packages/merkleroot.md)

* [fuchsia-scenic/](/src/lib/ui/fuchsia-scenic/)

    Rust interface to Scenic, the Fuchsia compositor

* [fuchsia-syslog-listener/](/src/lib/syslog/rust/syslog-listener/)

    Implement fuchsia syslog listeners in Rust

* [fuchsia-syslog/](/src/lib/syslog/rust/)

    Rust interface to the fuchsia syslog

* [fuchsia-system-alloc/](/src/lib/fuchsia-system-alloc/)

    A crate that sets the Rust allocator to the system allocator. This is automatically included for projects that use fuchsia-async, and all Fuchsia binaries should ensure that they take a transitive dependency on this crate (and “use” it, as merely setting it as a dependency in GN is not sufficient to ensure that it is linked in).
-->

* [fuchsia-merkle/](/src/sys/pkg/lib/fuchsia-merkle/)

    使用 [Merkle Trees](/docs/concepts/packages/merkleroot.md) 保护和认证数据块

* [fuchsia-scenic/](/src/lib/ui/fuchsia-scenic/)

    Fuchsia 合成器 Scenic 的 Rust 接口

* [fuchsia-syslog-listener/](/src/lib/syslog/rust/syslog-listener/)

    用 Rust 实现的 fuchsia 系统日志监听器

* [fuchsia-syslog/](/src/lib/syslog/rust/)

    fuchsia 系统日志的 Rust 接口

* [fuchsia-system-alloc/](/src/lib/fuchsia-system-alloc/)

    一个可以把 Rust 分配器设置为系统分配器的 crate 。使用 fuchsia 异步的工程会自动包含这个 crate, 所有 Fuchsia 二进制文件都应确保它们对这个 crate 具有可传递的依赖关系（并“使用”它，因为仅将其设置为 GN 中的依赖项不足以确保它被链接）

<!--
* [fuchsia-trace/](/src/lib/trace/rust/)

    A safe Rust interface to Fuchsia's tracing interface

* [storage](/src/lib/storage/)

    Bindings and protocol for serving filesystems on the Fuchsia platform

* [storage/fuchsia-vfs-watcher/](/src/lib/storage/fuchsia-vfs-watcher/)

    Bindings for watching a directory for changes

* [fuchsia-zircon/](/src/lib/zircon/rust/)

    Rust language bindings for Zircon kernel syscalls

* [mapped-vmo/](/src/lib/mapped-vmo/)

    A convenience crate for Zircon VMO objects mapped into memory
-->

* [fuchsia-trace/](/src/lib/trace/rust/)

    一个安全的 Rust 接口，对接 Fuchsia 的追踪接口

* [storage](/src/lib/storage/)

    Fuchsia 平台上服务于文件系统的绑定和协议

* [storage/fuchsia-vfs-watcher/](/src/lib/storage/fuchsia-vfs-watcher/)

    绑定用于观察目录的修改

* [fuchsia-zircon/](/src/lib/zircon/rust/)

    基于 Rust 语言的绑定用于 Zircon 内核的系统调用

* [mapped-vmo/](/src/lib/mapped-vmo/)

    一个很方便的 crate 用于 Zircon VMO 对象的内存映射

<!--
* [mundane/](/src/lib/mundane/)

    A Rust crypto library backed by BoringSSL

* [shared-buffer/](/src/lib/shared-buffer/)

    Utilities for safely operating on memory shared between untrusting processes

* [zerocopy/](/src/lib/zerocopy/)

    Work with values contained in raw Byte strings without copying
-->

* [mundane/](/src/lib/mundane/)

    一个由 BoringSSL 支持的 Rust 加密库

* [shared-buffer/](/src/lib/shared-buffer/)

    在非信任进程之间进行内存共享的安全操作工具

* [zerocopy/](/src/lib/zerocopy/)

    不用拷贝就可以对原始字节字符串进行值操作