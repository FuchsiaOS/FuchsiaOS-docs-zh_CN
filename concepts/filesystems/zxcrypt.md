# zxcrypt

<!--## Overview-->

## 概述

<!--zxcrypt is a block device filter driver that transparently encrypts data being written to and
decrypts being read from data a block device.  The underlying block device that a zxcrypt device
uses may be almost any block device, including raw disks, ramdisks, GPT partitions, FVM partitions
or even other zxcrypt devices.  The only restriction is that the block size be page-aligned.  Once
bound, the zxcrypt device will publish another block device in the device tree that consumers can
interact with normally.-->

Zxcrypt 是一个块设备过滤器驱动，它加密写入块设备的数据和解密从块设备读取的数据。Zxcrypt 设备使用的底层块设备可以是几乎任何块设备，包括原始磁盘、内存磁盘、GPT 分区、FVM 分区甚至其他 zxcrypt 设备。它唯一的限制是块大小必须与页面对齐。绑定后，zxcrypt 设备将在设备树中发布另一个可供消费者正常交互的块设备。

<!--## Usage-->

## 用法

<!--zxcrypt contains both a [driver](/src/devices/block/drivers/zxcrypt) and [library](/src/security/zxcrypt)
Provided by libzxcrypt.so are four functions for managing zxcrypt devices.  Each takes one or more
`zxcrypt_key_t` keys, which associates the key data, length, and slot in the case of multiple keys.-->

Zxcrypt 包含由 libzxcrypt 提供的[驱动程序](/src/devices/block/drivers/zxcrypt)和[库](/src/security/zxcrypt)。
管理 zxcrypt 设备的四个函数也是如此。每个密钥都有一个或多个 `zxcrypt_key_t` 密钥，如果有多个密钥，则会关联密钥数据、长度和槽。

<!--* The __zxcrypt_format__ function takes an open block device, and writes the necessary encrypted
  metadata to make it a zxcrypt device.  The zxcrypt key provided does not protect the data on the
  device directly, but is used to protect the data key material.-->

* __zxcrypt_format__ 函数获取一个开放块设备，并写入必要的加密元数据，使其成为 zxcrypt 设备。 提供的 zxcrypt 密钥不直接保护设备上的数据，但用于保护数据密钥材料。

```c++
zx_status_t zxcrypt_format(int fd, const zxcrypt_key_t* key);
```

<!--* The __zxcrypt_bind__ function instructs the driver to read the encrypted metadata and extract the
  data key material to use in transparently transforming I/O data.-->

* __zxcrypt_bind__ 函数指示驱动程序读取加密的元数据并提取数据密钥材料，以用于透明地转换 I/O 数据。

```c++
zx_status_t zxcrypt_bind(int fd, const zxcrypt_key_t *key);
```

<!--* The __zxcrypt_rekey__ function uses the old key to first read the encrypted metadata, and the new
  key to write it back.-->

* __zxcrypt_rekey__ 函数使用旧密钥首先读取加密元数据，然后使用新密钥将其写回。

```c++
zx_status_t zxcrypt_rekey(int fd, const zxcrypt_key_t* old_key, const zxcrypt_key_t* new_key);
```

<!--* The __zxcrypt_shred__ function first verifies that the caller can access the data by using the key
  provided to read the encrypted metadata.  If this succeeded, it then destroys the encrypted
  metadata containing the data key material.  This prevents any future access to the data.-->

* __zxcrypt_shred__ 函数首先验证调用方是否可以通过使用提供的密钥读取加密元数据来访问数据。如果此操作成功，则会销毁包含数据密钥材料的加密元数据。这将防止将来对数据的任何访问。

```c++
zx_status_t zxcrypt_shred(int fd, const zxcrypt_key_t* key);
```

<!--## Technical Details
### DDKTL Driver-->

## 技术细节
### DDKTL 驱动

<!--zxcrypt is written as a DDKTL device driver.  [src/lib/ddktl](/src/lib/ddktl) is a C++ framework
for writing drivers in Fuchsia.  It allows authors to automatically supply the
[src/lib/ddk](/src/lib/ddk) function pointers and callbacks by using templatized mix-ins.  In the
case of zxcrypt, the [device](/src/devices/block/drivers/zxcrypt/device.h) is "Messageable",
"IotxnQueueable", "GetSizable", "UnbindableDeprecated", and implements the methods listed in DDKTL's
[BlockProtocol](/sdk/banjo/fuchsia.hardware.block/block.fidl).-->

Zxcrypt 是为 DDKTL 设备编写的驱动程序。[DDKTL](/src/lib/ddktl) 是 Fuchsia 用来写驱动程序的一个 C++ 框架。
它允许开发者使用模板化混合来自动提供[DDK](/src/lib/ddk)函数指针和回调。
在 zxcrypt 中，[设备](/src/devices/block/drivers/zxcrypt/device.h)是“可传递消息的”、“可查询的”、“可获取大小的”、“不建议使用的绑定”，并实现了DDKTL 的[BlockProtocol](/sdk/banjo/fuchsia.hardware.block/block.fidl)中列出的方法

<!--There are two small pieces of functionality which cannot be written in DDKTL and C++:
* The driver binding logic, written using the C preprocessor macros of DDK's
  [binding.h](/src/lib/ddk/include/lib/ddk/binding.h).
* The completion routines of [ulib/sync](/zircon/system/ulib/sync), which are used for synchronous I/O
  and are incompatible with C++ atomics.-->

下面两个小功能不能用 DDKTL 和 C++ 编写:
* 驱动程序绑定逻辑，使用DDK的[binding.h](/src/lib/ddk/include/lib/ddk/binding.h)的 C 预处理器宏编写。
* [LIB/SYNC](/zircon/system/ulib/sync)的完成例程，用于同步 I/O，与 C++ 原子不兼容。

<!--### Worker Threads
The device starts [worker threads](/src/devices/block/drivers/zxcrypt/worker.h) that run for the duration
of the device and create a pipeline for all I/O requests.  Each has a type of I/O it operates on, a
queue of incoming requests I/O that it will wait on, and a data cipher.  When a request is received,
if the opcode matches the one it is looking for, it will use its cipher to transform the data in the
request before passing it along.

The overall pipeline is as shown:-->

### 工作线程
设备启动[工作线程](/src/device/block/rivers/zxcrypt/worker.h)时，为所有 I/O 请求创建管道。
每个 I/O 都有其操作的 I/O 类型、它将等待的传入请求 I/O 队列和数据密码。
当接收到请求时，如果操作码与它正在查找的操作码匹配，它将在传递请求之前使用其密码转换请求中的数据。
整个管道如图所示:
```
DdkIotxnQueue -+
                \       Worker 1:        Underlying      Worker 2:        Original
    BlockRead ---+--->  Encrypter   --->   Block   --->  Decrypter  ---> Completion
                /     Acts on writes       Device      Acts on reads      Callback
   BlockWrite -+
```

<!--The "encrypter" worker encrypts the data in every I/O write request before sending it to the
underlying block device, and the "decrypter" worker decrypts the data in every I/O read response
coming from the underlying block device.  The
[cipher](/src/security/fcrypto/cipher.h) must have a key length of at least 16 bytes,
be semantically secure ([IND-CCA2][ind-cca2]) and incorporate the block offset as a
"[tweak][tweak]".  Currently, [AES256-XTS][aes-xts] is in use.-->

“加密器”在将数据发送到底层块设备之前对每个I/O写请求中的数据进行加密，而“解密器”则对来自底层块设备的每个I/O读响应中的数据进行解密。
[cipher ](/src/security/fcrypto/cipher.h)的密钥长度必须至少为16个字节，确保语义安全的([ IND-CCA2 ][ind-cca2])，并将块偏移量合并为“[ tweak ][tweak]”，Fuchsia 目前正在使用[ AES256-XTS ][AES-XTS]。

<!--### Rings and Txns
In order to keep the encryption and decryption of data transparent to original I/O requester, the
workers must copy the data when transforming it.  The I/O request sent through the pipeline is not
actually the original request, but instead a "shadow" request that encapsulates the original
request.-->

### Rings and Txns
为了保持数据在加密和解密对原始的 I/O 请求者透明，工作线程必须先复制数据然后再传输。I/O 请求通过管道发送不是真正的原始请求，而是封装原始请求的“影子”请求。

<!--As shadow requests are needed, they are allocated backed sequentially by pages in the
[VMO](/docs/concepts/kernel/concepts.md#shared-memory-virtual-memory-objects-vmos-).  When the
worker needs to transform the data it either encrypts data from the original, encapsulated write
request into the shadow request, or decrypts data from the shadow request into the original,
encapsulated read request.  As soon as the original request can be handed back to the original
requester, the shadow request is deallocated and its page [decommitted](/docs/reference/syscalls/vmo_op_range.md).
This ensures no more memory is used than is needed for outstanding I/O requests.-->

当需要“影子”请求时，它们由[ VMO ](/docs/concepts/kernel/concepts.md#shared-memory-virtual-memory-objects-vmos-)中的页面按顺序进行分配
当工作线程需要传输数据时，它会将原始数据进行加密，并封装写请求到影子请求，或者将影子请求中的数据解密为原始数据，并封装读请求。
一旦可以将原始请求交还给原始请求者，影子线程将取消分配及其页面[ decommitted ](/docs/reference/syscalls/vmo_op_range.md)。这可确保使用的内存不会超过未完成I/O请求所需的内存。

<!--### Superblock Format
The key material for encrypting and decrypting the data is referred to as the data key, and is
stored in a reserved portion of the device called the `superblock`. The presence of this superblock
is critical; without it, it is impossible to recreate the data key and recover the data on the
device.  As a result, the superblock is copied to multiple locations on the device for redundancy.
These locations are not visible to zxcrypt block device consumers.  Whenever the zxcrypt driver
successfully reads and validates a superblock from one location, it will copy this to all other
superblock locations to help "self-heal" any corrupted superblock locations.

The superblock format is as follows, with each field described in turn:-->

### 超级块的格式化
用于加密和解密数据的密钥被称为数据密钥，并且被存储在设备的被称为“超级块”的保留部分中。超级块的存在时必要的；如果没有它，就不可能重新创建数据密钥并恢复设备上的数据。
因此，超级块被复制到设备上的多个位置以实现备份。这些备份的位置是对 zxcrypt 块设备消费者不可见的。每当 zxcrypt 驱动程序成功地从一个位置读取并验证超级块时，它就会将其复制到所有其他超级块位置，以帮助“自我修复”任何损坏的超级块位置。
超块格式如下，每个字段依次描述：
```
+----------------+----------------+----+-----...-----+----...----+------...------+
| Type GUID      | Instance GUID  |Vers| Sealed Key  | Reserved  | HMAC          |
| 16 bytes       | 16 bytes       | 4B | Key size    |    ...    | Digest length |
+----------------+----------------+----+-----...-----+----...----+------...------+
```

<!--* _Type [GUID][guid]_: Identifies this as a zxcrypt device. Compatible with
  [GPT](/src/storage/gpt/include/gpt/gpt.h).
* _Instance GUID_: Per-device identifier, used as the KDF salt as explained below.
* _Version_: Used to indicate which cryptographic algorithms to use.
* _Sealed Key_: The data key, encrypted by the wrap key as described below.
* _Reserved_: Unused data to align the superblock with the block boundary.
* [_HMAC_][hmac]: A keyed digest of the superblock up to this point (including the Reserved field).-->

* _Type [GUID][guid]_: 标识这个是一个 zxcrypt 设备。 兼容[ GPT ](/src/storage/gpt/include/gpt/gpt.h)。
* _Instance GUID_: 设备的标识符, 用作 KDF，如下所述。
* _Version_: 用于指示要使用的加密算法。
* _Sealed Key_: 由如下所述的包络密钥加密的数据密钥。
* _Reserved_: 保留数据用于超级块与块边界对齐。
* [_HMAC_][hmac]: 超级块的摘要(包括保留字段)。

<!--The wrap key, wrap [IV][iv], and HMAC key are all derived from a
[KDF](/src/security/fcrypto/hkdf.h).  This KDF is an [RFC 5869 HKDF][hkdf], which
combines the key provided, the "salt" of the instance GUID and a per-use label such as "wrap" or
"hmac".  The KDF does __NOT__ try to do any rate-limiting.  The KDF mitigates the risk of key reuse,
as a new random instance salt will lead to new derived keys.  The
[HMAC](/src/security/fcrypto/hmac.h) prevents accidental or malicious modification to
go undetected, without leaking any useful information about the zxcrypt key.-->

WRAP 密钥、WRAP[ IV ][iv]和 HMAC 密钥都派生自[ KDF](/src/security/fcrypto/hkdf.h)。
KDF 是一个[RFC 5869 HKDF][hkdf]，它结合了所提供的密钥、实例 GUID 的“撒盐（salt）加密”和每次使用的标签(如“WRAP”或“HMAC”)。
KDF __不__ 会尝试进行任何速率限制。KDF 降低了密钥重用的风险，因为新的随机实例撒盐加密将导致新的派生密钥。[HMAC ](/src/security/fcrypto/hmac.h)可防止意外或恶意修改未被检测到，而不会泄露有关 zxcrypt 密钥的任何有用信息。

<!--_NOTE: The KDF does __NOT__ do any [key stretching][stretch].  It is assumed that an attacker can
remove a device and attempt the key derivations on their own, bypassing the HMAC check and any
possible rate limits.  To prevent this, zxcrypt consumers should include properly rate-limited
device keys, e.g. those from a [TPM][tpm], in deriving their zxcrypt key._-->

_注：KDF __不__ 做任何[按键延展][stretch]。假设攻击者可以移除设备并自行尝试密钥派生，从而绕过HMAC检查和任何可能的速率限制。
为了防止这个，Xcrypt 用户应包括适当的速率限制设备密钥，如，那些来自[ TPM ][tpm]的密钥，在导出它们的 zxcrypt 密钥时。_

<!--## Future Work-->

## 发展趋势
<!--There are a number of areas where further work could, should, or must be done:-->

有几个方面可以、应该或必须做进一步的工作：

<!--* __Surface hidden bind failures__

  Currently, `zxcrypt_bind` may indicate success even though the device fails to initialize.
  zxcrypt is __NOT__ synchronously adding the device to the device tree when the binding logic is
  run.  It must do I/O and cannot block the call to `device_bind` from returning, so it spawn an
  initializer thread and adds the device when complete.

  As of 10/2017, this is an active area of DDK development and the policy is changing to requiring
  the device to be added before return, with an additional call to publish that may come later.
  With this it may be desirable to have the call to `zxcrypt_bind` block synchronously for callers
  until the device is ready or has unambiguously failed to bind.-->

* __表面隐藏绑定失败__
  目前，即使设备未能初始化，`zxcrypt_bind` 也可能指示成功。当绑定逻辑运行时，zxcrypt __不__ 同步地将设备添加到设备树中。它必须执行I/O，并且不能阻止对 `device_bind` 的调用返回，因此它会产生一个初始化器线程，并在完成时添加设备。

  截至2017年10月,这是 DDK 开发的一个活跃领域，政策正在改变，要求在返回之前添加设备，之后可能还会调用发布。这样，可能希望对 `zxcrypt_bind` 的调用为调用者同步阻塞，直到设备准备就绪或绑定明确失败。

<!--* __Use AEAD instead of AES-XTS__

  It is widely recognized that [AEADs][aead] provide superior cryptographic protection by validating
  the integrity of their data before decrypting it.  This is desirable, but requires additional
  per-block overhead.  This means either that consumers will need to consume non-page-aligned blocks
  (once the in-line overhead is removed), or zxcrypt will need to store the overhead out-of-line and
  handle [non-atomic write failures][atomic].-->

* __使用 AEAD 而不是 AES-XTS__

  人们普遍认为，[AEAD ][aead]在解密数据之前验证其数据的完整性，从而提供卓越的密码保护。这虽然很好，但需要额外的每个数据块开销。这意味着要么消费者需要使用非页面对齐的块(一旦消除了内联开销)，要么 zxcrypt 将需要将开销存储在内联外并处理[非原子性写入失败][atomic]。

<!--* __Support multiple keys__

  To facilitate [key escrow and/or recovery][escrow], it is straightforward to modify the superblock
  format to have a series of cryptographic envelopes.  In anticipation of this, the libzxcrypt API
  takes a variable number of keys, although the only length currently supported is 1, and the only
  valid slot is 0.-->

* __支持多密钥__

  将超块格式直接修改为具有一系列加密信封，为[密钥托管和/或恢复][escrow]提供便利，考虑到这一点，libzxcrypt API采用数量可变的密钥，尽管当前支持的唯一长度是1，唯一有效的槽是0。

<!--* __Adjust number of workers__

  Currently there is one encrypter and one decrypter.  These are designed to work with an arbitrary
  number of threads, so performance tuning may be need to find the optimal number of workers that
  balances I/O bandwidth with [scheduler churn][thrash].-->

* __调整工作线程数量__

  目前有一个加密器和一个解密器。这些线程被设计为使用任意数量的线程，因此可能需要进行性能调优，以找到平衡 I/O 带宽和[周期程序变动][thrash]的最佳工作线程数量。

<!--* __Remove internal checks__

  Currently, the zxcrypt code checks for many errors conditions at internal boundaries and returns
  informative errors if those conditions aren't met.  For performance, those that arise from
  programmer error only and not data from either the requester or underlying device could be
  converted to "debug" assertions that are skipped in release mode.-->

* __移除内部检查__

  目前，zxcrypt 代码在内部边界检查许多错误条件，如果不满足这些条件，则返回信息性错误。为了提高性能，可以将仅由程序员错误引起的断言(而不是来自请求者或底层设备的数据)转换为“调试”断言，并在发布模式中跳过这些断言。

[ind-cca2]: https://en.wikipedia.org/wiki/Ciphertext_indistinguishability
[tweak]: https://en.wikipedia.org/wiki/Block_cipher#Tweakable_block_ciphers
[aes-xts]: https://en.wikipedia.org/wiki/Disk_encryption_theory#XEX-based_tweaked-codebook_mode_with_ciphertext_stealing_.28XTS.29
[guid]: https://en.wikipedia.org/wiki/Universally_unique_identifier
[iv]: https://en.wikipedia.org/wiki/Initialization_vector
[hkdf]: https://tools.ietf.org/html/rfc5869
[hmac]: https://www.ietf.org/rfc/rfc2104.txt
[stretch]: https://en.wikipedia.org/wiki/Key_stretching
[tpm]: https://trustedcomputinggroup.org/work-groups/trusted-platform-module/
[aead]: https://tools.ietf.org/html/rfc5116
[atomic]: https://en.wikipedia.org/wiki/Atomic_commit
[escrow]: https://en.wikipedia.org/wiki/Key_escrow
[thrash]: https://en.wikipedia.org/wiki/Thrashing_(computer_science)#Other_uses
