## Namespaces

The namespace of a process contains its private view of the world, and controls
how much of the Fuchsia system the process can influence. This effectively
defines the rules of the sandbox in which that process runs.

Namespaces are populated with various resource objects, including:

* **Files**: Objects which contain binary data.
* **Directories**: Objects which contain other objects.
* **Sockets**: Objects which establish connections when opened, like named
  pipes.
* **Protocols and services**: Objects which provide structured services when
  opened.
* **Devices**: Objects which provide access to hardware resources.

The ​​creator of the process populates the contents of a namespace based on the
set of required capabilities. A process cannot add objects to its own
namespace, as this would essentially amount to that process self-granting the
capabilities to access those objects.

<aside class="key-point">
  <b>No global filesystem</b>
  <p>In many ways, the contents of a namespace resemble the filesystem resources
  exposed by POSIX-oriented operating systems where "everything is a file".
  However, there are some very important differences to keep in mind.<p>

  <p>Namespaces are defined per-process and unlike other operating systems,
  Fuchsia does not have a "root filesystem". Instead, the path location
  <code>/</code> refers to the root of its private namespace. This also
  means Fuchsia does not have a concept of chroot environments, since every
  process effectively has its own private "root".

  <p>This also affects directory traversal, and how filesystem servers resolve
  paths containing <code>../.</code> For more details, see
  <a href="/docs/concepts/filesystems/dotdot">dot-dot considered harmful</a>.<p>
</aside>
