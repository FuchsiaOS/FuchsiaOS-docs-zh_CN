<!-- ## Storing packages -->
## 包存储

<!-- On the device, package BLOBs are stored in a content-addressable filesystem
optimized for write-once, read-often files called `blobfs`. This allows them
to be **de-duplicated** across all packages and
**cryptographically verified** using their hash. Fuchsia runs the `pkg-cache`
service on top of `blobfs` to facilitate package management. -->
在设备上，包 BLOB 存储于内容可寻址的文件系统中，该文件系统针对一次写入，频繁读取的文件进行了优化，称为 `blobfs`。这使得包在所有包范围内被**去重**并采用各自的哈希**加密验证**。Fuchsia 在 `blobfs` 上运行 `pkg-cache` 服务以便进行包管理。

<!-- ![Diagram illustrating how the package cache is built on top of "blobfs" — a
content-addressable filesystem that de-duplicates BLOBs allowing them to be
shared between packages.] -->
!["blobfs" 中包缓存建立的图示 —— 内容可寻址的文件系统使 BLOB 去重并允许它们在包间共享。]
(/get-started/images/intro/blobfs.png){: width="632"}

<!-- The `pkg-cache` layer tracks which packages in the system are currently
active. **Packages are not explicitly installed or removed in Fuchsia**.
Software is delivered on demand and likewise space can be reclaimed from
packages that are no longer active through periodic **garbage collection**.
When `pkg-cache` triggers garbage collection to reclaim space, content BLOBs
not referenced by any active package are deleted. -->
`pkg-cache` 层跟踪系统中哪些包当前处于活动状态。**包未在 Fuchsia 中明确安装或删除**。软件按需交付，同样可以通过定期的**垃圾清理**从不再活动的包中回收空间。当 `pkg-cache` 触发垃圾清理已回收空间时，任何未被活动包引用的内容 BLOB 将被删除。

<!-- Note: For more of the technical details on Fuchsia packages and software
delivery, see the [package documentation](/concepts/packages/package.md). -->
注：更多有关 Fuchsia 中包和软件交付的技术细节，请参考[包文档](/concepts/packages/package.md)。
