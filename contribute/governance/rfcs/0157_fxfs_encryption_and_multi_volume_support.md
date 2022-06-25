<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0157" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

This RFC describes encryption and multi-volume support in Fxfs.

## Motivation

Fuchsia needs to be able to support multiple volumes that are encrypted using
different keys that are bound to different users' passwords.

## Stakeholders

_Reviewers:_ abarth@google.com, enoharemaien@google.com, palmer@google.com,
jfsulliv@google.com, jsankey@google.com, zarvox@google.com,

_Consulted:_ Fuchsia's security, storage and privacy teams.

_Socialization:_ This design was circulated and reviewed with the Storage team
and aforementioned reviewers prior to starting this RFC process.

## Design

Fxfs is described in [RFC-0136](contribute/governance/rfcs/0136_fxfs.md).

### Requirements

1. It should be possible to create, enumerate and delete volumes.

2. Each volume should be encrypted with different keys.

3. Object metadata such as file names, sizes, and timestamps should be
   encrypted.

4. It must be possible to delete volumes when no key is present.

5. Support for key-rolling should be possible without requiring herculean
   migration efforts.

6. There should be limits on the sizes of volumes (although the design for this
   is left for the future).

7. It should be possible to query the size of the volume without access to the
   key.

8. There should be protection against fingerprint attacks based on block
   count. This is an attack where knowing the encrypted size (rounded up to the
   nearest block) of a number of files in a set can be used to ascertain whether
   that set of files exists on the filesystem.

### Out-of-Scope

The following areas are not covered by this design:

1. Transferring encrypted images between devices (which includes host and target
   devices) in either direction — whilst being able to do this to aid debugging
   would be helpful, it is unlikely to ever be possible on a production device
   and there is no precedent.

2. The implementation of the crypt service. This design is covered elsewhere.

3. The design should support key rolling, but the precise APIs to be used
   between components is out-of-scope.

### Overview

#### Fxfs Encryption

Fxfs built-in encryption will support separate per-file keys. Files will
typically only have one encryption key, but to support key rolling and file
cloning, the format will support multiple keys per-file. Keys will be wrapped
and unwrapped by a crypto service that Fxfs will communicate with.

The following objects within the root parent and root stores will have some form
of encryption:

1. The journal, (which exists within the root-parent store), will have mutations
   that are for metadata in child stores encrypted using a stream cipher. The
   key for this cipher will be per-store.

2. The layer files for child stores (which exist within the root store) will be
   encrypted. The mechanism for encrypting these files will be the same as all
   other files. The key will be wrapped using a per-store key and stored with
   other unencrypted store information.

Other objects in the root parent and root stores will not be encrypted, namely
the super-block, all objects related to the allocator, the layer files backing
the root object store (which holds metadata for root-store objects) and the
objects that contain basic information about the child stores. None of this
constitutes user data.

The net result of this is that:

 * User data will be encrypted
 * All metadata including filenames, file sizes, extent information and
   directory information will be encrypted.

Some information will _not_ be encrypted:

 * The number of files within a volume.
 * The set of extents allocated to a volume.

#### Object IDs

Within Fxfs Object IDs are currently allocated in a monotonically increasing
fashion which can be used as a side-channel. To address this, the
least-significant 32 bits of the object ID will be encrypted (at allocation
time) using [ff1] encryption. The key will be rotated after cycling through 32
bits worth of object IDs. The key will be stored wrapped and stored with the
rest of the unencrypted store information. The upper 32 bits of the object ID
will increment monotonically each time the key is rolled.

The number of objects and space used within a volume also provides a
side-channel (essentially all information available via [statvfs]).  Addressing
this is out-of-scope for this design (it is not new to Fxfs).

[ff1]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-38G.pdf
[statvfs]: https://pubs.opengroup.org/onlinepubs/009695399/basedefs/sys/statvfs.h.html

#### Key Management

Key wrapping and unwrapping is the responsibility of a separate "crypt" service
which will serve something like the following protocol:

```fidl
/// Designates the purpose of a key.
type KeyPurpose = flexible enum {
    /// The key will be used to encrypt metadata.
    METADATA = 1;
    /// The key will be used to encrypt data.
    DATA = 2;
};

protocol Crypt {
    /// Creates a new wrapped key.  `owner`, effectively a nonce, identifies the
    /// owner (an object ID) of the key and must be supplied to `UnwrapKey`.
    /// `metadata` indicates that the key will be used to encrypt metadata which
    /// might influence the choice of wrapping key used by the service.  Returns
    /// the wrapping key ID used to wrap this key along with the wrapped and
    /// unwrapped key.  Errors:
    ///   ZX_ERR_INVALID_ARGS: purpose is not recognised.
    ///   ZX_ERR_BAD_STATE: the crypt service has not been correctly initialized
    ///     with a wrapping key for the specified purpose.
    CreateKey(struct {
        owner uint64;
        purpose KeyPurpose;
    }) -> (struct {
        wrapping_key_id uint64;
        wrapped_key bytes:48;
        unwrapped_key bytes:32;
    }) error zx.status;

    /// Unwraps keys that are wrapped by the key identified by `wrapping_key_id`.
    /// `owner` must be the same as that passed to `CreateKey`.  This can fail due
    /// to permission reasons or if an incorrect key is supplied.
    UnwrapKey(struct {
        wrapping_key_id uint64;
        owner uint64;
        key bytes:48;
    }) -> (struct {
        unwrapped_key bytes:32;
    }) error zx.status;
};
```

Note:

 * The meaning of `wrapping_key_id` is up to the implementer of the crypt
   service. Fxfs will not apply any significance to its value.

 * It is expected that there will be a separate connection for each encrypted
   Fxfs volume, thus allowing the servers to be homed in different processes,
   should that be desired.

 * For now, 256 bit keys will be supported (which is consistent with Zxcrypt).

 * It is anticipated that the crypt service will use AEAD to wrap the keys
   which means wrapped keys are likely to be 48 bytes in size.

 * `purpose` is used to separate keys used for metadata and keys used for data,
   and exists to facilitate key rolling (see below).

The precise implementation of the crypt service, and the key management policy,
is out of scope for this design.

#### On-disk format

Each file will have something like:

```rust
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EncryptionKeys {
    None,
    AES256XTS(WrappedKeys),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WrappedKeys {
    /// The keys (wrapped).  To support key rolling and clones, there can be more
    /// than one key.  Each of the keys is given an identifier.  The identifier is
    /// unique to the object.  AES256-XTS requires a 512 bit key, which is made
    /// of two 256 bit keys, one for the data and one for the tweak.  Both those
    /// keys are derived from the single 256 bit key we have here.
    pub keys: Vec<(/* wrapping_key_id= */ u64, /* id= */ u64, [u8; 48])>,
}
```

Each extent will be something like:

```rust
pub enum ExtentValue {
    /// Indicates a deleted extent; that is, the logical range described by the
    /// extent key is considered to be deleted.
    None,
    /// The location of the extent and other related information.  `key_id`
    /// identifies which of the object's keys should be used.  `checksums` hold
    /// post-encryption checksums.
    Some { device_offset: u64, checksums: Checksums, key_id: u64 },
}
```

Data blocks will be encrypted using AES-XTS-256 (the same as that used by
Zxcrypt). The logical offset within the file will be used for the tweak.

#### Metadata

Object stores maintain log-structured merge (LSM) trees to hold their metadata
which will be encrypted in the same way as files contained within the store. The
keys used for the layer files will be created with the purpose set to
"metadata".

As transactions are committed, mutations are written to the journal. These
mutations are applied to the in-memory layers for the LSM trees. Some time
later, the in-memory layers are flushed into persistent layers. Any mutations
for the object metadata trees need to be encrypted as they are written to the
journal. To support this, Fxfs uses a new mutation:

```rust
EncryptedObjectStore(Box<[u8]>),
```

A stream cipher (a block cipher such as AES-XTS-256 is not suitable), Chacha20,
will be used to encrypt these mutations. The key for this will be wrapped and
stored with other unencrypted store data.

At replay time, the key might not be available, in which case these mutations
will be kept in memory in encrypted form. If there is a need to flush in-memory
data (to release space in the journal), these encrypted mutations will be
written to an object in the root store.

When the key becomes available, the encrypted mutations, which might be
in-memory or exist in the aforementioned file, can be decrypted and applied. To
decrypt the mutations, the offset in the cipher stream is required; this is
stored with the unencrypted information for a store.

These are the new fields added to StoreInfo (the unencrypted information for a
store):

```rust
    // The (wrapped) key that encrypted mutations should use.
    mutations_key: Option<Box<[u8]>>,

    // Mutations for the store are encrypted using a stream cipher.  To decrypt the
    // mutations, we need to know the offset in the cipher stream to start it.
    mutations_cipher_offset: u64,

    // If we have to flush the store whilst we do not have the key, we need to
    // write the encrypted mutations to an object. This is the object ID of that
    // file if it exists.
    encrypted_mutations_object_id: u64,
```

#### Volume Deletion

To support deleting volumes, allocation metadata will include the object ID of
the store ID that owns metadata:

```rust
pub struct AllocatorKey {
    pub device_range: Range<u64>,
}

pub enum AllocationRefCount {
    // Tombstone variant indicating an extent is no longer allocated.
    None,
    // Used when we know there are no possible allocations below us in the stack.
    // (e.g. on the first allocation of an extent.)
    // This variant also tracks the owning ObjectStore for the extent.
    Abs { count: u64, owner_object_id: u64 },
}

pub struct AllocatorValue {
     /// Reference count for the allocated range.
     /// (A zero reference count is treated as a 'tombstone', indicating that older
     /// values in the LSM Tree for this range should be ignored).
     pub refs: AllocationRefCount,
}
```

Deleting a volume will involve changing the allocator so that it regards all
records belonging to a deleted volume as free. After a major compaction, we can
be sure that those records no longer exist and we can then forget that the
volume exists. The allocator keeps information about the list of deleted volumes
with the rest of its metadata.

#### Inline data

Fxfs does not currently support inline data, but when it does, it will be
encrypted with the same keys used to encrypt metadata. These keys effectively
get rolled as new layer files are written.

#### Secure Erase

Securely erasing a file requires that a file remains unrecoverable even if keys
(excepting those present in hardware) are subsequently compromised. Given that
the filesystem is typically running on flash devices (which employ garbage
collection), wiping all occurrences of data is non-trivial. The only practical
solution is to roll the wrapping keys (which should not be stored on flash).

Secure erase is not a feature that is planned to be implemented, but it should
be possible with the following procedure:

1. Start wrapping new metadata keys with a new wrapping key.

2. Fxfs rewrites all objects using the old metadata key.

3. The old metadata wrapping key is shredded (typically using a TPM feature
   either directly or indirectly — this is out of scope for this design).

Step 2 can be accomplished relatively easily within Fxfs by performing a major
compaction. As this is something that happens naturally, this is a procedure
that can take place routinely as well as on demand. It would be possible to
arrange for Fxfs to guarantee a major compaction once every, say, week, although
it would depend on the keys being available.

This procedure requires that metadata keys are wrapped using a different
wrapping key to data (since otherwise it would force all data to be rewritten,
which is prohibitive), hence the metadata argument supplied to the create_key
method.

Note that the procedure requires rewriting *all* metadata for a volume, which
should therefore not be something that should be performed frequently.

#### Key Rolling

Key rolling of metadata wrapping keys is outlined above for secure
erase. Rolling the keys used to wrap data keys should be possible using the
following procedure:

1. Start wrapping new keys with a new wrapping key. Return a new wrapping_key_id
   for keys wrapped in this way.

2. Ask Fxfs to rewrap all keys matching a given wrapping_key_id. The API for
   this is left for a subsequent design: it should require no on-disk format
   change.

3. Follow the Secure Erase procedure to wrap the metadata keys. Since keys are
   only ever written to metadata files, it should ensure that the data wrapping
   key is successfully rolled once the old wrapping key has been shredded.

Note that steps 2 and 3 could be combined — it should be possible to perform a
major compaction and rewrap the keys at the same time.

Like Secure Erase, key rolling support is not planned to be implemented
initially.

#### Fsck

Fsck without the key will perform a limited set of checks. Consistency of the
encrypted store is obviously not possible, but it will be possible to check
extents and consistency of all other unencrypted stores.

With the key available, it will be possible to perform a complete consistency
check or just a check of the metadata for an individual volume.

#### Multi-Volume Support

`fshost` will export a new directory: volumes. The nodes within the directory
will represent volumes exported by fshost and will support a new volume protocol
(no other protocol will be supported on the nodes, i.e. they will not support
the fuchsia.io Node protocol, so they cannot be cloned). The protocol will be
something like:

```fidl
type MountOptions = table {
    // A handle to the crypt service. Unencrypted volumes will ignore this option.
    crypt client_end:Crypt;
}

protocol Volume {
    // Mounts the volume and starts serving the filesystem. An error will be
    // returned if the volume is currently being served from a previous call
    // to Mount.
    Mount(resource struct {
        export_root server_end:Directory;
        options MountOptions;
    }) -> () error zx.status;
}
```

Mount will fail if the wrong crypt service is supplied but this should be
exceptional; users should call Mount with the expectation that Mount will
succeed; it should not be used as a means to test a credential.

The export root will look just like that exposed by filesystems now. The
fs.Admin service will be exposed and volumes can be locked/shut-down using the
Shutdown method. A volume will also become locked if all connections to the
volume are closed. When a volume is locked, care will be taken to make sure that
all unwrapped keys are discarded.

Enumeration and removal will be done via the fuchsia.io Directory protocol on
the volumes directory. Removal might or might not be asynchronous, but success
will not be returned until it is guaranteed the volume will eventually be
removed. The name can be reused immediately, but the space might take some time
to become available for use.

New volumes cannot use the Directory protocol's open method because we need to
supply a crypt service and other options. Instead a new protocol will be added:

```fidl
type CreateVolumeOptions = table {
    // Reserved for future use.
}

protocol Manager {
    // Creates a new data volume.
    CreateVolume(resource struct {
        name string:MAX_FILENAME;
        crypt client_end:Crypt;
        export_root server_end: Directory;
        options CreateVolumeOptions;
    }) -> () error zx.status;;
}
```

Initially, implementation will proceed assuming that Fxfs will be managing all
these volumes, but it should be possible in future to provide a component that
works with file systems backed by Zxcrypt.

The names and precise set of volumes that might exist on a volume will be
based on product decisions and are outside the scope of this RFC.

## Implementation

Support for multiple volumes and encryption will be implemented in typical
fashion.

There are no current plans to support for key-rolling, secure-erase and
inline-data.

AES-XTS-256, ChaCha20 and ff1 encryption will be provided by third-party crates.

Security audits will be required.

## Performance

Encryption will have an impact on performance. Existing filesystem benchmarks
will be used to evaluate performance. Once implemented, Zxcrypt can be
eliminated which should counter any loss due to this implementation. Any
regressions from current performance will be investigated.

## Backwards Compatibility

This will be a breaking change to Fxfs and will require a reformat.

## Security considerations

This RFC has been reviewed by the security team.

The following requirements are driven by security:

1. Each volume should be encrypted by different keys.

2. File data and metadata should be encrypted.

3. It should be difficult to use object IDs as a side-channel.

The implementation will require a security review.

## Privacy considerations

This RFC has been reviewed by the privacy team.

The main privacy consideration is that the following data will _not_ be
encrypted:

 * The number of files within a volume.
 * The set of extents allocated to a volume (which would include the amount of
   space allocated to a volume).
 * The names of volumes.

All other data and metadata should be encrypted. This should be sufficient to
address the requirements including the fingerprint attack.

The security review of the implementation should verify the design meets these
requirements.

## Testing

This will be tested using the usual combination of unit, integration and
end-to-end tests. Fxfs will be used for the data partition for many of the
tests that run under CQ and will therefore gain exposure that way.

## Documentation

At some stage, documentation will be required to help system designers choose
between different storage options for a given product.  However, the way in
which that configuration takes place is yet to be decided and is outside the
scope of this RFC.

The APIs introduced in this RFC will initially (and quite possibly indefinitely)
be in-tree, and for the time being will be documented as part of the FIDL.

## Drawbacks, alternatives, and unknowns

The encryption design outlined in this RFC is considerably more complicated than
partition-based encryption as used by Zxcrypt.

It will still be possible for system designers to use partition-based
encryption, but this comes with limitations on space sharing between volumes: a
partition-based scheme that is to be flexible with space requires a
volume-manager (thus incurring a performance-hit due to the additional
indirection) and can suffer fragmentation issues (moving space between
partitions might require defragmentation). It is also considerably harder to
support secure erase and key rolling on partition based schemes.

## Prior art and references

Zxcrypt uses AES-256 XTS to encrypt blocks in a similar way as this RFC although
there are differences in the tweak used.

The design here is mostly compatible with [Android 12's encryption
requirements].  One significant difference is that metadata that passes through
Fxfs's journal needs to be encrypted with a stream cipher which is not
explicitly mentioned as acceptable.

[Android 12's encryption requirements]: https://source.android.com/compatibility/12/android-12-cdd#99_data_storage_encryption
