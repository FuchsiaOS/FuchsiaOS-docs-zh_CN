<!-- ## Namespaces -->
## 命名空间
<!-- 
The namespace of a process contains its private view of the world, and controls
how much of the Fuchsia system the process can influence. This effectively
defines the rules of the sandbox in which that process runs.
 -->
进程的命名空间包含了进程对外界的私有视图，并且控制了进程能够对 Fuchsia 系统造成多少影响。
这有效地定义了进程所在的沙箱的规则。

<!-- Namespaces are populated with various resource objects, including: -->
命名空间由各种资源对象填充，包括：
<!-- 
* **Files**: Objects which contain binary data.
* **Directories**: Objects which contain other objects.
* **Sockets**: Objects which establish connections when opened, like named
  pipes.
* **Protocols and services**: Objects which provide structured services when
  opened.
* **Devices**: Objects which provide access to hardware resources.
 -->
* **文件**: 包含二进制数据的对象。
* **目录**: 包含其他对象的对象。
* **套接字**: 打开时建立连接的对象，如命名管道。
* **协议和服务**: 打开时提供结构化服务的对象。
* **设备**: 提供对硬件资源的访问的对象。
<!-- 
The ​​creator of the process populates the contents of a namespace based on the
set of required capabilities. A process cannot add objects to its own
namespace, as this would essentially amount to that process self-granting the
capabilities to access those objects.
 -->
进程的创建者基于那些所请求的能力，来填充命名空间的内容。
进程不能向自己的命名空间添加对象，因为这实际上相当于进程在授予自身访问那些对象的权限。

<aside class="key-point">
  <!-- <b>No global filesystem</b> -->
  <b>没有全局文件系统</b>
  <!-- 
  <p>In many ways, the contents of a namespace resemble the filesystem resources
  exposed by POSIX-oriented operating systems where "everything is a file".
  However, there are some very important differences to keep in mind.<p>
 -->
  <p>从许多方面来看，命名空间的内容都类似于面向 POSIX 的操作系统暴露的“一切都是文件”的文件系统资源。
  但是有一些非常重要的差异需要注意。</p>
<!-- 
  <p>Namespaces are defined per-process and unlike other operating systems,
  Fuchsia does not have a "root filesystem". Instead, the path location
  <code>/</code> refers to the root of its private namespace. This also
  means Fuchsia does not have a concept of chroot environments, since every
  process effectively has its own private "root".
 -->
<p>命名空间是为每个进程定义的。不像其他操作系统，Fuchsia 不存在“全局文件系统（root filesystem）”。
而路径地址“/”则指代进程私有命名空间的根。
这也意味着 Fuchsia 不存在 chroot 环境的概念，因为每个进程都有自己的私有“root”。</p>
<!-- 
  <p>This also affects directory traversal, and how filesystem servers resolve
  paths containing <code>../.</code> For more details, see
  <a href="/concepts/filesystems/dotdot">dot-dot considered harmful</a>.<p>
 -->
  <p>这也影响到目录遍历，以及文件系统服务器如何解析包含 <code>../.</code> 的路径。
  要获取更多详情，请参阅<a href="/concepts/filesystems/dotdot">我们认为双点（dot-dot）有害</a>。</p>
</aside>
