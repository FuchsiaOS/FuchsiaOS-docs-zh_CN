<!--

# Dot Dot Considered Harmful

-->

# 点点（".."）被认为是有害的

<!--

Child processes on Fuchsia are only capable of accessing the resources provided
to them -- this is an essential idea encompassing microkernels, and other
“capability-based” systems. If a handle is provided to a service, access to
that handle implies the client can use it.

-->

Fuchsia 上的子进程只能访问提供给它们的资源——这是包含微内核和其他“基于能力”
系统的基本思想。 如果向服务提供句柄，则访问该句柄意味着客户端可以使用它。 

<!--
Intuitively, this concept can be applied to filesystems: If a handle is
provided to a directory, it should imply access to resources within that
directory (and additionally, their subdirectories). Unfortunately, however, a
holdout from POSIX prevents directory handles from cleanly integrating with
these concepts in a capability system: “..”. If a handle is provided to a
directory, the client can simply request “..”, and the handle will be
“upgraded” to access the parent directory, with broader scope. As a
consequence, this implies that a handle to a directory can be upgraded
arbitrarily to access the entire filesystem.

-->

直观地说，这个概念可以应用于文件系统：如果为目录提供了句柄，
它应该意味着可以访问该目录中的资源（以及它们的子目录）。 
然而，不幸的是，POSIX 阻止了目录句柄与能力系统中的这些概念完全集成：
".."。 如果为目录提供句柄，客户端可以简单地请求".."，
句柄将被“升级”以访问父目录，范围更广。 
因此，这意味着可以任意升级目录句柄以访问整个文件系统。 

<!--

Traditionally, filesystems have tried to combat this using "chroot", which
changes the notion of a filesystem root, preventing access beyond ".." in
trivial cases of path traversal. However, this approach has some problems:

-->

传统上，文件系统试图使用"chroot"来解决这个问题，
它改变了文件系统根的概念，防止在路径遍历的琐碎情况下访问超出".."。 
但是，这种方法存在一些问题： 

<!--

  * Chroot changes the notion of root on a coarse, "per-program" basis, not on
    a per-descriptor basis
  * Chroots are often misused (i.e., fchdir to a different open handle that
    sits outside the chroot)
  * Chroots are not "on by default", so it may be tempting for programs to
    simply not use them.

-->

* chroot 在粗略的“每个程序”的基础上改变了 root 的概念，而不是在每个描述符的基础上。
* chroot 经常被误用（例如，将 fchdir 指向位于 chroot 之外的不同打开句柄）
* chroot 不是“默认开启”的，因此程序可能忽略它们。

<!--

To overcome these deficiencies, Fuchsia does not implement traditional dot dot
semantics on filesystem servers, which would allow open directories to traverse
upward. More specifically, it disallows access to “..”, preventing clients
from trivially accessing parent directories. This provides some strong
properties for process creation: If an application manager only wants to give a
process access to "/data/my_private_data", then it can simply provide a handle
to that open directory to the child process, and it will "automatically" be
sandboxed.

-->

为了克服这些缺陷，Fuchsia 没有在文件系统服务器上实现传统的点点（".."）语义，
这将允许打开的目录向上遍历。 更具体地说，它不允许访问".."，
从而防止客户端随意访问父目录。 这为进程创建提供了一些强大的属性：
如果应用程序管理器只想让进程访问"/data/my_private_data"，
那么它可以简单地为子进程提供该打开目录的句柄，
并且它将“自动” 被沙盒化。 

<!--

## What about paths that can be resolved without the filesystem server?

-->

## 关于那些可以在没有文件系统服务器的情况下解析的路径？

<!--

Certain paths, such as “foo/../bar”, which can be transformed to “bar”, can be
determined without accessing a filesystem server in the absence of symbolic
links (and at the time of writing, symbolic links do not exist on Fuchsia).
These paths may be canonicalized, or cleaned, on the client-side, prior to
sending path-based requests to filesystem servers: the libfdio library already
does this for any fdio operations that are eventually transmitted to
filesystem servers in a function called `__fdio_cleanpath`.

-->

在没有符号链接的情况下，可以在不访问文件系统服务器的情况下确定某些路径，
例如可以转换为"bar"的"foo/../bar"（在撰写本文时，符号链接不存在于 Fuchsia 上）。 
在向文件系统服务器发送基于路径的请求之前，这些路径可以在客户端进行规范化或清理：
libfdio 库已经为最终在名为 __fdio_cleanpath 的函数中传输到文件系统服务器的任何 fdio 操作执行此操作。

<!--

## What about shell traversal?

-->

## 关于shell遍历？

<!--

I.e, if someone “cd”s into a directory, how can they leave? Internally, the
notion of “CWD” isn’t merely a file descriptor to an open directory; rather,
it’s a combination of “file descriptor” and “absolute path interpreted to mean
CWD”. If all operations to cd act on this absolute path, then “..” can always
be resolved locally on a client, rather than being transmitted to a filesystem
server. For example, if the CWD is “/foo/bar”, and a user calls “cd ..”, then
the underlying call may be transformed into “chdir /foo/bar/..”, which can be
canonicalized to “/foo”.

-->

也就是说，如果有人"cd"到一个目录，他们怎么能离开？ 
在内部，"CWD"的概念不仅仅是打开目录的文件描述符； 
相反，它是“文件描述符”和“解释为 CWD 的绝对路径”的组合。 
如果对 cd 的所有操作都作用于这个绝对路径，
那么".."总是可以在客户端本地解析，而不是传输到文件系统服务器。 
例如，如果 CWD 是"/foo/bar"，并且用户调用"cd .."，
那么底层调用可能会转换为"chdir /foo/bar/.."，
可以将其规范化为"/foo"。 

<!--

Once these hurdles have been overcome, the benefits of removing “..” are
enormous: access to filesystem resources fits naturally within the capability
system, [sandboxing](/docs/concepts/process/sandboxing.md) new processes becomes massively easier, and
resource access can more naturally be composed through filesystem
[namespaces](/docs/concepts/process/namespaces.md).

-->

一旦克服了这些障碍，删除".."的好处是巨大的：对文件系统资源的访问自然适合能力系统，[sandboxing](/docs/concepts/process/sandboxing.md)
新进程变得非常容易， 并且资源访问可以更自然地通过文件系统 [namespaces](/docs/concepts/process/namespaces.md) 组成。 