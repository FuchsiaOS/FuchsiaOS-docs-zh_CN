# Fuchsia Rust Crates

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

* [fuchsia-merkle/](/src/sys/pkg/lib/fuchsia-merkle/)

    Protect and verify data blobs using [Merkle Trees](concepts/packages/merkleroot.md)

* [fuchsia-scenic/](/src/lib/ui/fuchsia-scenic/)

    Rust interface to Scenic, the Fuchsia compositor

* [fuchsia-syslog-listener/](/src/lib/syslog/rust/syslog-listener/)

    Implement fuchsia syslog listeners in Rust

* [fuchsia-syslog/](/src/lib/syslog/rust/)

    Rust interface to the fuchsia syslog

* [fuchsia-system-alloc/](/src/lib/fuchsia-system-alloc/)

    A crate that sets the Rust allocator to the system allocator. This is automatically included for projects that use fuchsia-async, and all Fuchsia binaries should ensure that they take a transitive dependency on this crate (and “use” it, as merely setting it as a dependency in GN is not sufficient to ensure that it is linked in).

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

* [mundane/](/src/lib/mundane/)

    A Rust crypto library backed by BoringSSL

* [shared-buffer/](/src/lib/shared-buffer/)

    Utilities for safely operating on memory shared between untrusting processes

* [zerocopy/](/src/lib/zerocopy/)

    Work with values contained in raw Byte strings without copying
