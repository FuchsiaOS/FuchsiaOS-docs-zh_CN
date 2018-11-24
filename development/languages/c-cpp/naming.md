C / C ++对象命名
====================

## 包含路径

以下准则适用于广泛使用的库，例如在Fuchsia代码库的上层或者通过SDK工具包，其中“Fuchsia代码库的上层”的意思是“garnet”和(peridot、 topaz、vendor/foo)的上级。

头文件分为三类：system、fuchsia、其它。

#### system头文件

```
<zircon/foo/bar.h>
```

###### 解释

这些头文件描述了内核接口 (系统调用, 相关结构和定义)，在内核和用户空间之间的共享定义和数据结构
(和引导程序)，通常也适用于更高层级.

###### 备注

- 这些头文件可能在`zircon/`目录下.
- This does not include things like wrappers on syscall interfaces like zx.

###### 示例

- `zircon/process.h`
- `zircon/syscalls/hypervisor.h`


#### 全局头文件

```
<fuchsia/foo/bar.h>
```

###### 解释

这些是在Fuchsia中定义低级别的ABI / API库，但不是特定于内核。

###### 备注

- FIDL-generated code for Fuchsia APIs in that very namespace,
  as well as C/C++ wrapper libraries around these APIs are installed here.
- 这些头文件可能在`fuchsia/`目录下.

###### 示例

- `fuchsia/fdio/fdio.h`
- `fuchsia/pixelformat.h`


#### 其它头文件

```
<lib/foo/bar.h>
```

###### 解释

Some libraries in that space are not necessarily Fuchsia-specific, or they
may be Fuchsia-specific but do not fall into either of the above categories.
We use a rather bland namespace that will likely not cause any collisions in
the outside world: "lib".

###### 备注

- 这些头文件可能不会放在`lib/`目录下. Subdirectories (`lib/foo/`)
  are mandatory.

###### 示例

- `lib/app/cpp/startup_context.h`
- `lib/fbl/array.h`
- `lib/zx/event.h`
