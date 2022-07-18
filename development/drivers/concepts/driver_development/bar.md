<!--
    (C) Copyright 2018 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

<!---

# Configuration

Hardware peripherals are attached to the CPU through a bus, such as the PCI bus.

During bootup, the BIOS (or equivalent platform startup software)
discovers all of the peripherals attached to the PCI bus.
Each peripheral is assigned resources (notably interrupt vectors,
and address ranges for configuration registers).

The impact of this is that the actual resources assigned to each peripheral may
be different across reboots.
When the operating system software starts up, it enumerates
the bus and starts drivers for all supported devices.
The drivers then call PCI functions in order to obtain configuration information about
their device(s) so that they can map registers and bind to interrupts.

--->

# 配置

硬件周边件和 CPU 通过总线绑定在一起，例如 PCI 总线。

当启动时， BIOS （或者同等平台启动软件）发现绑定在 PCI 总线上的所有周边件。
每一个周边件被分配好对应资源（尤其是中断向量和配置寄存器的地址范围）。

这样做的影响在于分配给每个外设的实际资源在重启时可能是不同的。
当操作系统软件启动时，它枚举了总线和对所有支持的设备启动相关驱动。
接下来驱动调用 PCI 函数来获取关于他们的设备配置信息，这样就可以映射寄存器并绑定中断。

<!---

## Base address register

The Base Address Register (**BAR**) is a configuration register that exists on each
PCI device.
It's where the BIOS stores information about the device, such as the assigned interrupt vector
and addresses of control registers.
Other, device specific information, is stored there as well.

Call **pci_map_bar()**
to cause the BAR register to be mapped into the driver host's address space:

```c
zx_status_t pci_map_bar(const pci_protocol_t* pci, uint32_t bar_id,
                        uint32_t cache_policy, void** vaddr, size_t* size,
                        zx_handle_t* out_handle);
```

The first parameter, `pci`, is a pointer to the PCI protocol.
Typically, you obtain this in your **bind()** function through
**device_get_protocol()**.

The second parameter, `bar_id`, is the BAR register number, starting with `0`.

The third parameter, `cache_policy`, determines the caching policy for access,
and can take on the following values:

--->

## 基础地址寄存器

基础地址寄存器（**BAR**）是在每一个 PCI 设备上都存在的配置寄存器。
这是 BIOS 存储关于设备信息的地方，例如分配的中断向量和控制寄存器地址。另外设备特有信息也存放在这里。

调用**pci_map_bar()**来使 BAR 寄存器映射到驱动主机的地址空间内：

```c
zx_status_t pci_map_bar(const pci_protocol_t* pci, uint32_t bar_id,
                        uint32_t cache_policy, void** vaddr, size_t* size,
                        zx_handle_t* out_handle);
```

第一个参数`pci`是 PCI 协议的指针。通常情况下，你可以通过**device_get_protocol()**在你的**bind()**函数内获取到。

第二个参数`bar_id`，是从`0`开始的 BAR 寄存器数量。

第三个参数`cache_policy`决定了访问的缓存策略，你可以设置为以下值：

`cache_policy` value                | Meaning
------------------------------------|---------------------
`ZX_CACHE_POLICY_CACHED`            | use hardware caching
`ZX_CACHE_POLICY_UNCACHED`          | disable caching
`ZX_CACHE_POLICY_UNCACHED_DEVICE`   | disable caching, and treat as device memory
`ZX_CACHE_POLICY_WRITE_COMBINING`   | uncached with write combining

<!---

Note that `ZX_CACHE_POLICY_UNCACHED_DEVICE` is architecture dependent
and may in fact be equivalent to `ZX_CACHE_POLICY_UNCACHED` on some architectures.

The next three arguments are return values.
The `vaddr` and `size` return a pointer (and length) of the register region, while
`out_handle` stores the created handle to the
[VMO](/docs/reference/kernel_objects/vm_object.md).

--->

注意`ZX_CACHE_POLICY_UNCACHED_DEVICE`是依赖架构的，并且在某些架构中它是等同于 `ZX_CACHE_POLICY_UNCACHED`。

接下来的三个参数则为返回值。
`vaddr`和`size`返回一个寄存器区域的指针（和长度），`out_handle`则存储了创建的[VMO](/docs/reference/kernel_objects/vm_object.md)句柄。

<!---

## Reading and writing memory

Once the **pci_map_bar()**
function returns with a valid result, you can access the BAR with simple pointer
operations, for example:

--->

## 读和写内存

当**pci_map_bar()**函数返回了有效值后，你就可以通过简单的指针操作访问 BAR ，例如：

```c
volatile uint32_t* base;
...
zx_status_t rc;
rc = pci_map_bar(dev->pci, 0, ZX_CACHE_POLICY_UNCACHED_DEVICE, &base, &size, &handle);
if (rc == ZX_OK) {
    base[REGISTER_X] = 0x1234;  // configure register X for deep sleep mode
}
```

<!---

It's important to declare `base` as `volatile` &mdash; this tells the compiler not to
make any assumptions about the contents of the data that `base` points to.
For example:

--->

把`base`声明为`volatile`类型是很重要的—这告诉了编译器不要对`base`所指向的数据内容做任何的假设。例如：

```c
int timeout = 1000;
while (timeout-- > 0 && !(base[REGISTER_READY] & READY_BIT)) ;
```

<!---

is a typical (bounded) polling loop, intended for short polling sequences.
Without the `volatile` keyword in the declaration, the compiler would have no reason
to believe that the value at `base[REGISTER_READY]` would ever change, so it would
cause it to be read only once.

--->

这是一种典型（有界的）的轮询，试图达到很短的轮询序列。
在定义中没有使用`volatile`的关键字，编译器则没有理由相信`base[REGISTER_READY]`的值会产生变化，所以这将导致它只会被读取一次。