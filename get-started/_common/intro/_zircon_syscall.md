## System calls

User space code interacts with the objects in kernel space using
**system calls**. Zircon has system calls to perform low-level operations such
as:

*   Memory management
*   Task and process management
*   Inter-process communication (IPC) and synchronization
*   Exception handling
*   Hardware support services (clocks, entropy, device I/O)

<aside class="key-point">
  <p>Zircon has fewer system calls than POSIX-oriented operating systems due to
  services like filesystems and drivers being hosted outside the kernel. See the
  full list of available Zircon system calls in the
  <a href="reference/syscalls.md"> reference documentation.</a></p>
</aside>

User space processes access system calls through `libzircon.so` — a
[virtual Dynamic Shared Object (vDSO)][glossary.virtual-dynamic-shared-object].
The Zircon vDSO is a shared library in ELF format that the kernel maps into the
address space of each new process. This library is considered "virtual" because
it is exposed directly by the kernel image rather than being loaded from a file.

Most system calls operate directly with one or more [handles][glossary.handle]
— process-local references to objects living in kernel space represented as a
32-bit integer (`zx_handle_t`). Each handle declares the privileges, or
**rights**, the holder has to perform actions on the handle itself or the
referenced object.

<aside class="key-point">
 <b>Handles vs. file descriptors</b>
 <p>Similar to POSIX file descriptors, handles are references to a specific
 kernel object and they play a role in granting capabilities. However, Zircon
 handles are slightly more flexible with rights applied to the handle rather
 than the calling process. It is possible for a single process to have two
 different handles to the same kernel object with different rights.</p>

 <p>In addition, handles cannot be referenced by name and Zircon does not
 reserve any identifiers for common streams like stdin and stdout.</p>

 <p>For more details, see <a href="concepts/kernel/handles.md">
 Zircon handles</a>.</p>
</aside>


[glossary.virtual-dynamic-shared-object]: glossary/README.md#virtual-dynamic-shared-object
[glossary.handle]: glossary/README.md#handle
