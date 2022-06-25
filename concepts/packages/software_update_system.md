# Software Update System

Fuchsia is a project that constantly gets updates for new features, enhancements,
and security fixes. Fuchsia's software update system makes use of
[The Update Framework (TUF) version 1.0](https://github.com/theupdateframework/specification/blob/HEAD/tuf-spec.md).
However, Fuchsia does have some differences from TUF:

* [Specification version](#specification-version)
* [Package organization](#package-organization)
* [Merkle root](#merkle-root)

## Specification version {#specification-version}

In a Fuchsia repository, the Fuchsia repository version is listed as a
top-level attribute of the target role's signed data. This example shows
the format of the specification version:

```
{
  ...

  "signed": {
    "_type": ROLE,
    "spec_version": "1",
    "custom": {
      "fuchsia_spec_version": <FUCHSIA_SPEC_VERSION>,
  }

  ...
}
```

Definition of values:

* `FUCHSIA_SPEC_VERSION`. `INT`. The value of the Fuchsia repository specification
  version. For example, `1`.

## Package organization {#package-organization}

TUF targets in a Fuchsia repository that address Fuchsia packages contain custom
meta data that points to the Package Metadata Archive. This example shows the
format for packages:

```
{
  ...

  "targets": {
    "/PACKAGE_PATH": {
      ...
    }

  ...
  }
}
```

Definition of values:

* `PACKAGE_PATH`. The relative path to the package from the repository's
  base URL.

  Note: At the moment the only supported path is `PACKAGE/VARIANT`, where
  `PACKAGE` is the package name and `VARIANT` is the package version.

## Merkle root {#merkle-root}

In the Fuchsia repository, each package target includes the
[merkle root](/docs/concepts/packages/merkleroot.md) of the package's meta FAR as a custom attribute.
This example shows the format for the merkle root:

```
{
  ...

  "targets" : {
    PACKAGEPATH : {
      "length" : LENGTH,
      "hashes" : HASHES,
      "custom" : {
        "merkle" : <MERKLE_ROOT>,
        "size" : <BLOB_SIZE>,
      }
    }

    ...
  }
}
```

Definition of values:

* `MERKLE_ROOT`. `STRING`. The hex string of the merkle root hash of the package's
  meta FAR.
* `BLOB_SIZE`. `INT`. The size, in bytes, of the unencrypted BLOB identified by the `MERKLE_ROOT`.

