{% set rfcid = "RFC-0100" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }} - {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

Fuchsia Software Development Kits (SDKs) currently ship so called device
profile metadata (or, in this document, simply the _metadata_), whose schema is
defined in `//build/sdk/meta/device_profile.json`. The metadata is intended to
describe system images and packages that can be installed on a device. We have
found that the schema is incomplete and needs modernization. In addition, tools
outside the SDK now need the metadata, increasing its importance.

This RFC introduces a new schema for this information as a contract between the
platform and tools along with a process to update that schema in the future.

## Motivation

Currently, the metadata is shipped with SDKs. It currently contains the name
of a device, a text description of that device, and URIs from which a user can
download image and package information. We have found that this metadata is
both _difficult to keep up-to-date_ and _incomplete_, making it unusable.

Tools end up relying on unofficial conventions about images and packages, or
hard-coding assumptions that may change (such as the structure of URIs for
downloads). This makes the tools susceptible to breakages when these
conventions change.

We wish to resolve these problems. We will specify a formal schema for the
metadata, so that tools can consume it.

When we distribute metadata that is both complete and easy to use, SDK users
will be able to choose which image they want to flash to their device or boot
with an emulator without significant help from Fuchsia's infrastructure or SDK
teams. SDK developers will have a single source of truth for how they surface
information about products to users. Tool developers will have a well-defined
contract for how they consume this information.

### Ease of Update

The metadata is currently hard coded as part of SDK generation that needs to
be updated by hand whenever we publish a new image. This makes adding, renaming
or removing metadata laborious and error prone. The information regularly falls
out of date - or worse, is never added at all.

In practice, because SDK consumers cannot rely on the accuracy of metadata
shipped with the SDK, they hard code the location of packages and images, as
well as other brittle implementation-dependent information, into their tools.
For example, in addition to the hard coding of URLs, the out-of-tree emulator
boot relies on an undocumented `images.json` file's presence in the system
image.

This proposal provides a new schema for our metadata that we expect external
users to rely upon. This will provide motivation for us to keep it up to date.

### Completeness

The existing device metadata is missing crucial detail. For example, tools for
flashing a device require lists of partition names, as well as file names to be
written into those partitions. This is not currently formalized, so we have no
contract between what we distribute and tools that perform flashing. This makes
it challenging to provide stability / compatibility guarantees.

When you combine the lack of completeness of the schema with the difficulty of
updating the metadata, the problem is compounded. For example, consider the
hypothetical example of an emulator tool that wants to make sure that it starts
an emulator with the same architecture as the image the user wants to run.
Without attempting to boot the image, how is the tool supposed to know whether
any image in a particular location is compatible with a particular architecture?
With an updated schema, we can have a single set of metadata that describes
both the architecture of an image and its location. The tool can then simply
choose the correct architecture for a given image.

This proposal will outline an initial schema for our metadata, and provide a
path forward for updates to ensure that they can be kept relevant.

## Design

In order to provide useful, up-to-date [glossary.product] metadata with our
SDKs, we will:

* Define a new schema for the metadata.
* Define a process for updating that schema.
* Generate metadata that adheres to that schema as part of our build, and
  consume it in-tree.
* Distribute metadata that adheres to that schema as part of our SDK releases,
  and consume it out-of-tree.
* Provide [standard tools][ffx] and libraries that consume metadata for flashing and
  emulation workflows.

Note that, while we define an initial schema here, we will allow evolution to
take place through API-Council review, without additional RFCs.

### Definitions

For the purposes of this document, we use the following definitions:

* A _schema_ is a formal description of the organization of metadata.
* _Metadata_ are concrete instantiations of a given schema.
* An _artifact_ is any concrete file or data, frequently referenced in metadata
  by a URI.
* A _manifest_ is metadata containing a list of artifacts.
* An _image_ is a collection of artifacts necessary to flash a physical device
  or start an emulator.
* A _package_ is a unit of software distribution in Fuchsia typically packaged
  as a Fuchsia ARchive (FAR) file.
* A _product_ is a
  [build configuration][build-configuration]
  that is used to create images and packages.
* A _product bundle_ is a collection of images and packages which are an output
  of a product build.
* _Target (device)_ is a physical or virtual Fuchsia device intended to run a
  product.

We will use JSON to represent our metadata. Schemata are expressed in the
[Draft 7 format defined at json-schema.org][json-schema].

### Schemata

This section details the initial requirements and initial changes we will make
to the schemata.

#### Requirements for initial schema

1. The metadata must have a unique name or an ID.
1. The metadata must have a human readable description.
1. The metadata schema must be versioned.
1. The metadata must specify the target device hardware characteristics. At
   the minimum, this must include:
    1. The target CPU architecture, currently either arm64 or x64.
1. The metadata must specify one or more product bundles, each of which must
   include:
    1. A unique name.
    1. A human readable description.
    1. An image bundle.
    1. A package bundle.
1. The metadata may include additional product metadata as key-value pairs,
   intended to assist users in selecting a desired product. Examples of such
   metadata include a build type (user vs. eng), startup type (normal
   vs. arrested).
1. An image or package bundle must:
    1. Specify one or more base URI.
        1. URI metadata must allow users to specify specific formats. For
           example, access to the URI may be archived (tarball or zip) or plain.
        1. SDK tools that consume this metadata must support `gs`, `http(s)` and
           `file` URI schemes. The `file` URIs are required for local access.
        1. SDK tools that consume this metadata must not require authentication
             if the artifact is publicly accessible.
    1. Specify [flash metadata](#flash-schema) including sufficient details to
       provision a device via the fastboot protocol.
    1. Specify [emulator metadata](#emulator-schema) including sufficient
       details to boot the image on a virtual device.

#### Schema evolution

There are a number of possible future directions for evolution of the schema.
For example:

1. Physical hardware metadata may include device hardware capabilities such as
   the presence of a screen, keyboard or pointing device.
1. Virtual hardware metadata may specify virtual device characteristics
   sufficient for provisioning an emulator.

To account for this, the schema is versioned. The version is a string that is
updated whenever an element of the schema is added or removed. Tools may target
multiple different versions of the schema, or users may need to resort to older
versions of tools to use older schemata.

To ease the transition between versions we propose a flexible parsing and
versioning scheme. We introduce a versioned envelope that simplifies the
parsing of JSON. A parsing tool can read the version field while ignoring the
rest of the JSON document. This allows the tool to choose a correct parser for
each supported version of the schema.

We propose to version each schema file independently using a randomly generated
8 digit hexadecimal number. The number must be unique across all [existing
schema files][schemata]. To guard against collisions, it is advisable to
implement a build-time collision detection to prevent accidental duplicate
submissions.

Since a schema may include other schemas via the `$ref` facility, we propose
to append the version of the schema to the schema file name. This will not only
disambiguate between multiple versions of common files such as
`//build/sdk/meta/common.json`, it will also trigger the version recomputation
of the referrer schema.

Each incompatible schema change such as adding or removing a required field
will trigger a version recomputation.

We allow schemata to be changed via the [API review process][api-review].
Changes to the schema fall under the Developer functional area.

## Implementation

The implementation of the schema itself is largely a matter of adding the schema
to fuchsia.git and ensuring that it gets API reviewed. There are two other
questions: _where is the metadata generated?_ and _where is the metadata
consumed?_

Device metadata will initially be generated:

 * as part of the Fuchsia build in `fuchsia.git`;
 * as part of publishing images for consumption out-of-tree; and
 * as part of SDK publication.

It may also be generated in other places, especially for images and packages
published outside of an SDK release.

Device metadata will largely be consumed by the `ffx` tool, which will use them
to flash devices and launch emulators. Because we will publish this schema
with SDKs, tools outside of the Fuchsia tree may also consume metadata that
adhere to it.

With this approach, all flashing and emulation workflows, whether for in-tree or
out-of-tree use, will use device metadata for specification.

Both of these efforts require design beyond the scope of this RFC.

### Common Schema {#implementation-common-schema}

We will extend the common.json schema definitions as follows. There is no need
to account for backwards incompatibility, because this is not an incompatible
change. The existing definitions in common.json are omitted below for brevity.

For brevity, we omit the original contents of the file, using `...` to indicate
an omission.

```
/** build/sdk/meta/common.json */
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "id": "http://fuchsia.com/schemas/sdk/common.json",
  "definitions": {
    ...,
    "envelope": {
      "description": "A versioned envelope.",
      "type": "object",
      "properties": {
        "version": {"$ref": "#/definitions/version"},
        "data": {
          "description": "The payload."
         }
      },
      "required": [
        "version",
        "data"
      ],
      "additionalProperties": false
    },
    "versioned_sdk_element": {
      "type": "object",
      "allOf": [
        {"$ref": "#/definitions/envelope"},
        {
          "type": "object",
          "properties": {
            "data": {
              "oneOf": [
                {"$ref": "#/definitions/sdk_element"}
              ]
            }
          }
        }
      ]
    },
    "version": {
      "description": "An opaque version string. The string may include any characters. Tools must not attempt to draw any conclusions about inter version compatibility other than the version 'X' manifest complies with the version 'X' of the schema and is therefore parsable by the version 'X' parser. There are no guarantees, for example, a parser for version 'B' may be able to parse a JSON document versioned 'A'.",
      "type": "string",
      "minLength": 1
    },
  }
}
```

Note that, because this schema defines what version of the schema we are using,
it cannot be easily changed. Tools will have to read files conforming to this
schema to understand the enveloped JSON.

### Device

Since multiple images may be installed on a device, we factor out the device
hardware specification into its own definition. The device hardware
specification serves two purposes:

1. It describes minimum hardware requirements necessary to run an image.
1. It specifies a virtual (emulated) device that is created to run an image.

We introduce two new versioned SDK elements for physical and virtual devices.

### Physical

At the moment, we only care about the CPU architecture of a physical
device. Additional hardware attributes will be added in the future versions of
the schema. For the purposes of providing a complete example, the schema
definition below combines the envelope schema with the `sdk_element` schema[^1].

[^1]: All metadata is distributed as SDK elements.

```
/** build/sdk/meta/physical_device-c906d79c.json */
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "id": "http://fuchsia.com/schemas/sdk/physical_device-c906d79c.json",
  "description": "A physical device specification.",
  "type": "object",
  "allOf": [
    {"$ref": "common.json#/definitions/envelope"},
    {
      "type": "object",
      "properties": {
        "data": {
          "allOf": [
            {"$ref": "common.json#/definitions/sdk_element"},
            {
              "properties": {
                 "type": {
                   "allOf": [
                     {"$ref": "common.json#/definitions/type"},
                     {"enum": ["physical_device"]}
                   ]
                 }
              }
            },
            {"$ref": "hardware-f9928aa4391e2ae3644ce712074a1ef7.json#/definitions/requirements"}
          ]
        }
      }
    }
  ]
}
```


### Physical Device Requirement Schema

This schema allows us to represent device hardware requirements. This allows
tools to ensure that the device hardware matches the artifacts we will use for
provisioning.  Currently, it only describes the CPU architecture.

```
/** build/sdk/meta/hardware-c0a116ca.json */
{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "definitions": {
        "requirements": {
            "properties": {
                "hardware": {
                    "additionalProperties": false,
                    "properties": {
                        "cpu": {
                            "additionalProperties": false,
                            "properties": {
                                "arch": {
                                    "oneOf": [
                                      {"$ref": "common.json#/definitions/target_arch"}
                                    ]
                                }
                            },
                            "required": ["arch"],
                            "type": "object"
                        }
                    },
                    "required": ["cpu"],
                    "type": "object"
                }
            },
            "required": ["hardware"],
            "type": "object"
        },
    },
    "description": "Hardware requirements for running a product image.",
    "id": "http://fuchsia.com/schemas/sdk/hardware-c0a116ca.json"
}
```

### Virtual

The virtual device specification allows us to select the appropriate product
bundle for booting the emulator. In the future revisions, the schema will be
developed further to configure the emulator. For the sake of an example, the
schema definition below uses a newly defined `versioned_sdk_element` schema.

```
/** build/sdk/meta/virtual_device-8a8e2ba9.json */
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "id": "http://fuchsia.com/schemas/sdk/virtual_device-8a8e2ba9.json",
  "description": "A virtual device specification for launching emulators.",
  "type": "object",
  "allOf": [
    {"$ref": "common.json#/definitions/versioned_sdk_element"},
    {
      "type": "object",
      "properties": {
        "data": {
          "type": "object",
          "properties": {
             "type": {
               "allOf": [
                 {"$ref": "common.json#/definitions/type"},
                 {"enum": ["virtual_device"]}
               ]
             },
             "description": {"type": "string"},
             "virtual": {"$ref": "virtual_hardware-4c5d1a5d.json#/definitions/spec"}
          },
          "required": ["virtual"]
        }
      }
    }
  ]
}
```

### Virtual Device Requirement Schema

The virtual hardware specification is used for selecting the appropriate
product bundle when launching the emulator.

```
/** build/sdk/meta/virtual_hardware-4c5d1a5d.json */
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "id": "http://fuchsia.com/schemas/sdk/virtual_hardware-4c5d1a5d.json",
  "description": "A virtual device specification for launching emulators.",
  "definitions": {
    "spec": {
      "emu": {
        "type": "object",
        "properties": {
          "cpu": {
            "type": "object",
            "properties": {
              "arch": {
                "oneOf": [
                  {"$ref": "common.json#/definitions/target_arch"}
                ]
              }
            },
            "required": ["arch"],
            "additionalProperties": false
          }
        },
        "required": ["cpu"],
        "additionalProperties": false
      }
    },
    "required": ["emu"]
    }
  }
}
```

### Product Bundle

The images and packages bundle provides access to the software artifacts
installed on a device.

```
/** build/sdk/meta/product_bundle-514c2856.json */
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "id": "http://fuchsia.com/schemas/sdk/product_bundle-514c2856.json",
  "description": "Artifacts required to boot and run software on a device.",
  "type": "object",
   "allOf": [
    {"$ref": "common.json#/definitions/versioned_sdk_element"},
    {
      "type": "object",
      "properties": {
        "data": {
          "type": "object",
          "properties": {
             "type": {
               "allOf": [
                 {"$ref": "common.json#/definitions/type"},
                 {"enum": ["product_bundle"]}
               ]
             },
             "description": {
               "description": "A human readable description of the product bundle.",
               "type": "string"
             },
             "device_refs": {
               "description": "A list of physical or virtual device names this product can run on.",
               "type": "array",
               "minItems": 1,
               "items": {
                   "type": "string",
                   "minLength": 1
               }
             },
             "metadata": {
               "description": "A list of key-value pairs describing product dimensions. Tools must not rely on the presence or absence of certain keys. Tools may display them to the human user in order to assist them in selecting a desired image or log them for the sake of analytics. Typical metadata keys are: build_info_board, build_info_product, is_debug.",
               "$ref": "common.json#/definitions/key_value_list"
             },
             "manifests": {
               "description": "Manifests describing how to boot the product on a device.",
               "flash": {"$ref": "flash_manifest-c85dbd8e.json#/definitions/manifest"},
               "emu": {"$ref": "emu_manifest-b0708439.json#/definitions/manifest"}
             },
             "images": {
               "description": "A list of system image bundles. Each image bundle must be equivalent to all others. I.e., all image bundle URIs are effectively mirrors of each other. Their formats may vary. Pick an entry that best suits your needs.",
               "type": "array",
               "minItems": 1,
               "items": {"$ref": "#/definitions/image_bundle"}
             },
             "packages": {
               "description": "A list of package bundles. Each package bundle must be equivalent to all otherwise. I.e., all package bundle URIs are effectively mirrors of each other. Their formats may vary. Pick an entry that best suits your needs.",
               "type": "array",
               "minItems": 1,
               "items": {"$ref": "#/definitions/package_bundle"}
             }
          },
          "required": ["device_refs", "manifests", "images", "packages"]
        }
      }
    }
  ],
  "definitions": {
    "image_bundle": {
      "description": "A set of artifacts necessary to provision a physical or virtual device",
      "type": "object",
      "properties": {
         "base_uri": {
           "description": "A base URI for accessing artifacts in the bundle.",
           "$ref": "#/definitions/bundle_uri"
         },
         "format": {
           "description": "Bundle format: files - a directory layout; tgz - a gzipped tarball. In case of the 'files' format, the base base path points to a directory. The manifest paths is relative to the directory. To get a full path append a path inside the manifest to base_uri. In case of the 'tgz' format, the base path points to the archive. The manifest path is relative within the archive."
           "enum": [
             "files",
             "tgz"
            ]
         }
      },
      "required": [
        "base_uri",
        "format"
      ],
      "additionalProperties": false
    },
    "package_bundle": {
      "description": "A set of artifacts necessary to run a physical or virtual device",
      "type": "object",
      "properties": {
         "repo_uri": {
           "description": "A package repository URI. This may be an archive or a directory.",
           "$ref": "#/definitions/bundle_uri"
         },
         "format": {
           "description": "Repository format: files - a directory layout; tgz - a gzipped tarball.",
           "enum" : [
             "files",
             "tgz"
           ]
         },
         "blob_uri": {
           "description": "An optional blob repository URI. If omitted, it is assumed to be <repo_uri>/blobs. If repo_uri refers to a gzipped tarball, ./blobs directory is expected to be found inside the tarball.",
           "$ref": "#/definitions/bundle_uri"
         }
      },
      "required": [
        "repo_uri",
        "format"
      ],
      "additionalProperties": false
    },
    "bundle_uri": {
      "description": "Allowed system image and package bundle URIs.",
      "type": "string",
      "format": "uri",
      "pattern": "^(?:http|https|gs|file):\/\/"
    }
  }
}
```
### Flash Schema {#flash-schema}

Manifests based on the flash schema describe image artifacts necessary to flash
the device. The manifest matches the existing flash.json produced by the
build. The flash metadata will be embedded in the product bundle SDK element.

```
/** build/sdk/meta/flash_manifest-c85dbd8e.json */
{

  "$schema": "http://json-schema.org/draft-07/schema#",
  "id": "http://fuchsia.com/schemas/sdk/flash_manifest-c85dbd8e.json",
  "description": "A manifest that describes how to flash a device.",
  "type": "object",
  "properties": {
    "manifest": {"$ref": "#/definitions/manifest"}
  },
  "required": ["manifest"],
  "additionalProperties": false,
  "definitions": {
    "manifest": {
      "description": "A named list of partitions and OEM files necessary to flash a device.",
      "type": "object",
      "properties": {
        "hw_revision" : {
          "description": "A board name used to verify whether the device can be flashed using this manifest.",
          "type": "string"
        },
        "products": {
          "description": "A list of product specifications that can be flashed onto the device.",
          "type": "array",
          "items": {"$ref": "#/definitions/product"},
          "minItems": 1
        }
      },
      "required": ["hw_revision", "products"],
      "additionalProperties": false
    },
    "product": {
      "description": "A named product specification.",
      "type": "object",
      "properties": {
        "name": {
          "description": "A unique name of this manifest.",
          "type": "string"
        },
        "partitions": {
          "description": "A list of partition names and file names corresponding to the partitions.",
          "$ref": "common.json#/definitions/key_value_list"
        },
        "bootloader_partitions": {
          "description": "A list of partition names and file names corresponding to the partitions.",
          "$ref": "common.json#/definitions/key_value_list"
        },
        "oem_files": {
          "description": "A list of OEM command and file names corresponding to the command.",
          "$ref": "common.json#/definitions/key_value_list"
        }
      },
      "required": ["name", "partitions"],
      "additionalProperties": false
    }
  }
}
```
### Emulator Schema {#emulator-schema}

The emulator schema specifies image artifacts necessary to boot the product on
the emulator.

```
/** build/sdk/meta/emu_manifest-b0708439.json */
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "id": "http://fuchsia.com/schemas/sdk/emu_manifest-b0708439.json",
  "definitions": {
    "manifest": {
      "description": "A manifest that describes how to boot an emulator.",
      "type": "object",
      "properties": {
        "kernel": {
          "description": "A path to the kernel image file. The path is relative to the image bundle base.",
          "type": "string",
          "minLength": 1
        },
        "initial_ramdisk": {
          "description": "A path to the initial ramdisk, the kernel ZBI. The path is relative to the image bundle base.",
          "type": "string",
          "minLength": 1
        },
        "disk_images": {
          "description": "A list of one or more disk image paths to FVM images. Each path is relative to the image bundle base.",
          "type": "array",
          "items": {
            "type": "string",
            "minLength": 1,
          },
          "minItems": 1
        }
      },
      "required": ["kernel", "initial_ramdisk", "disk_images"],
      "additionalProperties": false
    }
  }
}
```

### Other Common Definitions

Below are extra additions to the common definitions. See the [section on
common.json](#implementation-common-schema) for version related additions. The
existing definitions in common.json are omitted below for brevity.

```
/** build/sdk/meta/common.json */
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "id": "http://fuchsia.com/schemas/sdk/common.json",
  "definitions": {
    ...,
   "key_value": {
     "description": "A key-value pair.",
      "type": "array",
      "items": [
        {"type": "string"},
        {"type": ["number", "string"]}
      ],
      "minItems": 2,
      "additionalItems": false
    },
    "key_value_list": {
      "description": "A list of key-value pairs.",
      "type": "array",
      "items": {"$ref": "#/definitions/key_value"}
    }
```
### Examples

#### Devices

##### Physical Device

Below is an example of device hardware specifications.

```
{
  "version": "c906d79c",
  "data": {
    "type": "physical_device",
    "name": "generic-x64",
    "description": "A generic x64 device.",
    "hardware": {
      "cpu": {
        "arch": "x64"
      }
    }
  }
}
```

#### Product Bundles

##### Terminal on a Physical Device

This is an example of a generic terminal product bootable on a generic x64
device containing two archived mirrors for both images and packages accessible
via `gs` and `https` based URIs respectively.

```
{
  "version": "514c2856",
  "data": {
    "type": "product_bundle",
    "name": "generic-x64",
    "description": "A terminal x64 product.",
    "device_refs": ["generic-x64"],
    "metadata": [
      ["build-type", "release"],
      ["product", "terminal"]
    ],
    "manifests": {
      "flash": {
        "hw_revision": "x64",
        "products" : [
          {
            "bootloader_partitions": [],
            "name": "fuchsia",
            "oem_files": [],
            "partitions": [
              ["", "fuchsia.zbi"],
              ["", "zedboot.zbi"],
              ["", "fuchsia.vbmeta"],
              ["", "zedboot.vbmeta"]
            ]
          }
        ]
      }
    },
    "images": [
      {
        "base_uri": "gs://fuchsia/development/0.20201216.2.1/images/generic-x64.tgz",
        "format": "tgz"
      },
      {
        "base_uri": "https://storage.googleapis.com/fuchsia/development/0.20201216.2.1/images/generic-x64.tgz",
        "format": "tgz"
      }
    ],
    "packages": [
      {
        "repo_uri": "gs://fuchsia/development/0.20201216.2.1/packages/generic-x64.tar.gz",
        "format": "tgz"
      },
      {
        "repo_uri": "https://storage.googleapis.com/fuchsia/development/0.20201216.2.1/packages/generic-x64.tar.gz",
        "format": "tgz"
      }
    ]
  }
}
```
##### Terminal on an Emulator

This is an example of a generic terminal product bootable on an x64
emulator. Both images and packages are accessible directly from GCS.

```
{
  "version": "514c2856",
  "data": {
    "type": "product_bundle",
    "name": "generic-x64",
    "description": "A terminal x64 product.",
    "device_refs": ["qemu-x64"],
    "metadata": [
      ["build-type" , "debug"],
      ["product", "terminal"]
    ],
    "manifests": {
      "emu": {
        "kernel": "qemu-kernel.kernel",
        "initial_ramdisk": "zircon-a.zbi",
        "disk_images": ["storage-sparse.blk"]
      }
    },
    "images": [
      {
        "base_uri": "https://storage.googleapis.com/fuchsia-artifacts/builds/8852232026486839104/images/",
        "format": "files"
      }
    ],
    "packages": [
      {
        "repo_uri": "https://storage.googleapis.com/fuchsia-artifacts/builds/8852232026486839104/packages/",
        "blob_repo_uri": "https://storage.googleapis.com/fuchsia-artifacts/blobs/",
        "format": "files"
      }
    ]
  }
}
```

## Performance

This is unlikely to have an effect on performance because metadata generation
and parsing is not resource intensive.

## Ergonomics

Publishing standardized metadata should make integrations simpler by removing
the need for brittle hard-coded configuration.

We choose JSON because it is a well established format, and is the de facto
standard for Fuchsia configuration files. The JSON schema format is formalized
and peer reviewed.

## Backwards Compatibility

Introduction of metadata does not affect backwards compatibility, as there are
no current users of the existing metadata.

Future revisions of these schemata will be API reviewed and change managed using
a standard LSC process involving testing and direct interaction with high
priority customers. This process will be aided by versioning the schema
definitions.

## Security considerations

Metadata based on these schemata are no less secure than the existing solution,
which is an ad hoc hard-coding of similar information.

We will increase the security of artifact delivery early in the life cycle of
these schemas. Depending on in-progress feedback from the security team, this
may be prior to deployment. Critically, tools must verify the integrity of the
contents of the downloaded artifacts: the artifacts must be what was intended
when the metadata was published. This can be accomplished either by adding the
output of a hash function to the schema, or by using a protocol that supports
integrity checks, such as TUF.

Tools vended as part of a Fuchsia SDK that consume this schema and fetch
artifacts must implement additional security measures to ensure transport
security, to prevent attacks such as person-in-the-middle. They may do this by
refusing to fetch or use artifacts from insecure / untrusted locations.

## Privacy considerations

This proposal has no known privacy considerations. There may be privacy
considerations for people or teams that publish metadata; we expect them to
conduct their own privacy review. For example, teams that publish metadata may
need to consider carefully how they track downloads.

## Testing

Although the existence of a schema does not need testing, we will ensure that
metadata produced by our build meets the schema. We will also have
compatibility testing to ensure that tools continue to consume older schema as
necessary.

## Documentation

No documentation is currently planned outside of the schema itself, although
that will likely change with demand. A developer creating or updating a schema
will likely need to provide a glossary of terms, descriptions of how each schema
is used, and how they interact with each other.

## Drawbacks, alternatives, and unknowns

We are committing to a particular API for specifying device images. This has the
advantage of increasing compatibility across versions, leveraging existing
change management processes, and providing us the ability to distribute tools
that work consistently, but the disadvantage of decreased flexibility (it takes
longer to make a change to the schema because of bureaucratic hurdles).

## Prior art and references

This document collects and systematizes common configurations throughout the
tree, which are mentioned above. Much of the existing metadata can be found in
`//build/images/BUILD.gn`; for example, [the current version of the flashing
schema can be found there][prior-art].

[api-review]: /contribute/governance/api_council.md
[build-configuration]: /products/
[ffx]: /development/tools/ffx/overview.md
[glossary.product]: /glossary/README.md#product
[json-schema]: http://json-schema.org/draft-07/schema#
[prior-art]: https://fuchsia.googlesource.com/fuchsia/+/545a693eabe7c282de7e1560a3ee64f24f6988d1/build/images/BUILD.gn#177701
[schemata]: /build/sdk/meta
