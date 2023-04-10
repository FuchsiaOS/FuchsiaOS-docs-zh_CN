# zxcrypt

## Overview
zxcrypt is a block device filter driver that transparently encrypts data being written to and
decrypts being read from data a block device.  The underlying block device that a zxcrypt device
uses may be almost any block device, including raw disks, ramdisks, GPT partitions, FVM partitions
or even other zxcrypt devices.  The only restriction is that the block size be page-aligned.  Once
bound, the zxcrypt device will publish another block device in the device tree that consumers can
interact with normally.

## Usage
zxcrypt contains both a [driver](/src/devices/block/drivers/zxcrypt) and [library](/src/security/lib/zxcrypt)
Provided by libzxcrypt.so are four functions for managing zxcrypt devices.  Each takes one or more
`zxcrypt_key_t` keys, which associates the key data, length, and slot in the case of multiple keys.

* The __zxcrypt_format__ function takes an open block device, and writes the necessary encrypted
  metadata to make it a zxcrypt device.  The zxcrypt key provided does not protect the data on the
  device directly, but is used to protect the data key material.

```c++
zx_status_t zxcrypt_format(int fd, const zxcrypt_key_t* key);
```

* The __zxcrypt_bind__ function instructs the driver to read the encrypted metadata and extract the
  data key material to use in transparently transforming I/O data.

```c++
zx_status_t zxcrypt_bind(int fd, const zxcrypt_key_t *key);
```

* The __zxcrypt_rekey__ function uses the old key to first read the encrypted metadata, and the new
  key to write it back.

```c++
zx_status_t zxcrypt_rekey(int fd, const zxcrypt_key_t* old_key, const zxcrypt_key_t* new_key);
```

* The __zxcrypt_shred__ function first verifies that the caller can access the data by using the key
  provided to read the encrypted metadata.  If this succeeded, it then destroys the encrypted
  metadata containing the data key material.  This prevents any future access to the data.

```c++
zx_status_t zxcrypt_shred(int fd, const zxcrypt_key_t* key);
```

## Technical Details
### DDKTL Driver
zxcrypt is written as a DDKTL device driver.  [src/lib/ddktl](/src/lib/ddktl) is a C++ framework
for writing drivers in Fuchsia.  It allows authors to automatically supply the
[src/lib/ddk](/src/lib/ddk) function pointers and callbacks by using templatized mix-ins.

There are two small pieces of functionality which cannot be written in DDKTL and C++:

* The driver binding logic, written using the C preprocessor macros of DDK's
  [binding.h](/src/lib/ddk/include/lib/ddk/binding.h).
* The completion routines of [ulib/sync](/zircon/system/ulib/sync), which are used for synchronous I/O
  and are incompatible with C++ atomics.

### Worker Threads
The device starts [worker threads](/src/devices/block/drivers/zxcrypt/worker.h) that run for the duration
of the device and create a pipeline for all I/O requests.  Each has a type of I/O it operates on, a
queue of incoming requests I/O that it will wait on, and a data cipher.  When a request is received,
if the opcode matches the one it is looking for, it will use its cipher to transform the data in the
request before passing it along.

The overall pipeline is as shown:

```
DdkIotxnQueue -+
                \       Worker 1:        Underlying      Worker 2:        Original
    BlockRead ---+--->  Encrypter   --->   Block   --->  Decrypter  ---> Completion
                /     Acts on writes       Device      Acts on reads      Callback
   BlockWrite -+
```

The "encrypter" worker encrypts the data in every I/O write request before sending it to the
underlying block device, and the "decrypter" worker decrypts the data in every I/O read response
coming from the underlying block device.  The
[cipher](/src/security/lib/fcrypto/cipher.h) must have a key length of at least 16 bytes,
be semantically secure ([IND-CCA2][ind-cca2]) and incorporate the block offset as a
"[tweak][tweak]".  Currently, [AES256-XTS][aes-xts] is in use.

### Rings and Txns
In order to keep the encryption and decryption of data transparent to original I/O requester, the
workers must copy the data when transforming it.  The I/O request sent through the pipeline is not
actually the original request, but instead a "shadow" request that encapsulates the original
request.

As shadow requests are needed, they are allocated backed sequentially by pages in the
[VMO](/concepts/kernel/concepts.md#shared-memory-virtual-memory-objects-vmos-).  When the
worker needs to transform the data it either encrypts data from the original, encapsulated write
request into the shadow request, or decrypts data from the shadow request into the original,
encapsulated read request.  As soon as the original request can be handed back to the original
requester, the shadow request is deallocated and its page [decommitted](/reference/syscalls/vmo_op_range.md).
This ensures no more memory is used than is needed for outstanding I/O requests.

### Superblock Format
The key material for encrypting and decrypting the data is referred to as the data key, and is
stored in a reserved portion of the device called the `superblock`. The presence of this superblock
is critical; without it, it is impossible to recreate the data key and recover the data on the
device.  As a result, the superblock is copied to multiple locations on the device for redundancy.
These locations are not visible to zxcrypt block device consumers.  Whenever the zxcrypt driver
successfully reads and validates a superblock from one location, it will copy this to all other
superblock locations to help "self-heal" any corrupted superblock locations.

The superblock format is as follows, with each field described in turn:

```
+----------------+----------------+----+-----...-----+----...----+------...------+
| Type GUID      | Instance GUID  |Vers| Sealed Key  | Reserved  | HMAC          |
| 16 bytes       | 16 bytes       | 4B | Key size    |    ...    | Digest length |
+----------------+----------------+----+-----...-----+----...----+------...------+
```

* _Type [GUID][guid]_: Identifies this as a zxcrypt device. Compatible with
  [GPT](/src/storage/gpt/include/gpt/gpt.h).
* _Instance GUID_: Per-device identifier, used as the KDF salt as explained below.
* _Version_: Used to indicate which cryptographic algorithms to use.
* _Sealed Key_: The data key, encrypted by the wrap key as described below.
* _Reserved_: Unused data to align the superblock with the block boundary.
* [_HMAC_][hmac]: A keyed digest of the superblock up to this point (including the Reserved field).

The wrap key, wrap [IV][iv], and HMAC key are all derived from a
[KDF](/src/security/lib/fcrypto/hkdf.h).  This KDF is an [RFC 5869 HKDF][hkdf], which
combines the key provided, the "salt" of the instance GUID and a per-use label such as "wrap" or
"hmac".  The KDF does __NOT__ try to do any rate-limiting.  The KDF mitigates the risk of key reuse,
as a new random instance salt will lead to new derived keys.  The
[HMAC](/src/security/lib/fcrypto/hmac.h) prevents accidental or malicious modification to
go undetected, without leaking any useful information about the zxcrypt key.

_NOTE: The KDF does __NOT__ do any [key stretching][stretch].  It is assumed that an attacker can
remove a device and attempt the key derivations on their own, bypassing the HMAC check and any
possible rate limits.  To prevent this, zxcrypt consumers should include properly rate-limited
device keys, e.g. those from a [TPM][tpm], in deriving their zxcrypt key._

## Future Work
There are a number of areas where further work could, should, or must be done:

* __Surface hidden bind failures__

  Currently, `zxcrypt_bind` may indicate success even though the device fails to initialize.
  zxcrypt is __NOT__ synchronously adding the device to the device tree when the binding logic is
  run.  It must do I/O and cannot block the call to `device_bind` from returning, so it spawn an
  initializer thread and adds the device when complete.

  As of 10/2017, this is an active area of DDK development and the policy is changing to requiring
  the device to be added before return, with an additional call to publish that may come later.
  With this it may be desirable to have the call to `zxcrypt_bind` block synchronously for callers
  until the device is ready or has unambiguously failed to bind.

* __Use AEAD instead of AES-XTS__

  It is widely recognized that [AEADs][aead] provide superior cryptographic protection by validating
  the integrity of their data before decrypting it.  This is desirable, but requires additional
  per-block overhead.  This means either that consumers will need to consume non-page-aligned blocks
  (once the in-line overhead is removed), or zxcrypt will need to store the overhead out-of-line and
  handle [non-atomic write failures][atomic].

* __Support multiple keys__

  To facilitate [key escrow and/or recovery][escrow], it is straightforward to modify the superblock
  format to have a series of cryptographic envelopes.  In anticipation of this, the libzxcrypt API
  takes a variable number of keys, although the only length currently supported is 1, and the only
  valid slot is 0.

* __Adjust number of workers__

  Currently there is one encrypter and one decrypter.  These are designed to work with an arbitrary
  number of threads, so performance tuning may be need to find the optimal number of workers that
  balances I/O bandwidth with [scheduler churn][thrash].

* __Remove internal checks__

  Currently, the zxcrypt code checks for many errors conditions at internal boundaries and returns
  informative errors if those conditions aren't met.  For performance, those that arise from
  programmer error only and not data from either the requester or underlying device could be
  converted to "debug" assertions that are skipped in release mode.

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
