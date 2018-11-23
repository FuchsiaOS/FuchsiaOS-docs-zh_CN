# Fuchsia Rust Crates

* [fdio/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fdio/)

    Wrapper over zircon-fdio library

* [fuchsia-app/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-app/)

    Tools for working with Fuchsia Services and Applications

* [fuchsia-archive/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-archive/)

    Work with Fuchsia Archives (FARs)

* [fuchsia-async/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-async/)

    Fuchsia-specific Futures executor and asynchronous primitives (Channel, Socket, Fifo, etc.)

* [fuchsia-ddk/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-ddk/)

    Rust Driver Development Kit for Fuchsia (experimental - DO NOT USE)

* [fuchsia-device/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-device/)

    Rust bindings to common Fuchsia device libraries

* [fuchsia-framebuffer/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-framebuffer/)

    Configure, create and use FrameBuffers in Fuchsia

* [fuchsia-merkle/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-merkle/)

    Protect and verify data blobs using [Merkle Trees](https://fuchsia.googlesource.com/docs/+/master/the-book/merkleroot.md)

* [fuchsia-scenic/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-scenic/)

    Rust interface to Scenic, the Fuchsia compositor

* [fuchsia-syslog-listener/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-syslog-listener/)

    Implement fuchsia syslog listeners in Rust

* [fuchsia-syslog/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-syslog/)

    Rust interface to the fuchsia syslog

* [fuchsia-system-alloc/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-system-alloc/)

    A crate that sets the Rust allocator to the system allocator. This is automatically included for projects that use fuchsia-async, and all Fuchsia binaries should ensure that they take a transitive dependency on this crate (and “use” it, as merely setting it as a dependency in GN is not sufficient to ensure that it is linked in).

* [fuchsia-trace/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-trace/)

    A safe Rust interface to Fuchsia's tracing interface

* [fuchsia-vfs/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-vfs/)

    Bindings and protocol for serving filesystems on the Fuchsia platform

* [fuchsia-vfs/fuchsia-vfs-watcher/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-vfs/fuchsia-vfs-watcher/)

    Bindings for watching a directory for changes

* [fuchsia-zircon/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/fuchsia-zircon/)

    Rust language bindings for Zircon kernel syscalls

* [mapped-vmo/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/mapped-vmo/)

    A convenience crate for Zircon VMO objects mapped into memory

* [mundane/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/mundane/)

    A Rust crypto library backed by BoringSSL

* [shared-buffer/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/shared-buffer/)

    Utilities for safely operating on memory shared between untrusting processes

* [zerocopy/](https://fuchsia.googlesource.com/garnet/+/master/public/rust/zerocopy/)

    Work with values contained in raw Byte strings without copying
