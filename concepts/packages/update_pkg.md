# The update package

The update package is a package containing files and rules for how to update the
system.

## System update

The system update checker looks at the merkle root of the system image that the update
package has and compares it to the merkle root of the running system. It also checks
the merkle root of the update package and compares it to the version that the system
update checker last used. If they're different, then something other than the
system updater has updated the system.

The system updater reboots the device after a successful system update.

The system update checker periodically fetches the update package using the package
resolver and sees if it looks different. If the update package is different,
the system triggers a package update.

The system updater is designed such that the process can be interrupted at
any time and it does not leave the system in an unbootable or corrupt state.

First, the system updater reads the `update_mode` file to determine what operations to
perform. Then, the board file reads and verifies that there are no misconfigurations.
Then, the update package fetches the packages to serve. Finally, the update package writes
the kernel images and ensures that `vbmeta` must be written after the kernel image.

## Content of the update package

The structure of the update package, `fuchsia-pkg://fuchsia.com/update`, contains the following:

*   `/board`
    The board name. The updater verifies the contents and does an update only if this value matches
    the previous board name. This check prevents accidentally attempting to update a device to an
    unsupported architecture. For example, attempting to update an `x64` target to an `arm64` build will fail.

*   `/bootloader`
    Image of the bootloader firmware. DEPRECATED: please use `firmware` instead.

*   `/epoch.json`
    Epoch that the system cannot downgrade across via OTA. See
    [RFC-0071](/contribute/governance/rfcs/0071_ota_backstop.md) for more context. For example:

    ```json
    {
        "version": "1",
        "epoch": 5
    }
    ```

*   `/firmware[_<type>]`
    Firmware image. For example: `firmware`, `firmware_bl2`, `firmware_full`. Each device
    supports a custom set of firmware types, and unsupported types are ignored. This serves
    two main purposes:
    1. Specifying multiple pieces of firmware; for example, devices which have multiple
       bootloader stages.
    2. Providing a simple and safe way to transition to new firmware types; it's just a matter of
       adding the backend paver logic and then putting the new file in the update package.

*   `/packages.json`
    JSON-formatted list of merkle-pinned package URLs that belong to the base package set
    of the target OS image. The update package looks at `/packages.json` to determine what
    (and in what order) needs to be updated.
    For example:

    ```json
    {
        “version”: “1”,
        “content”: [
            "fuchsia-pkg://fuchsia.com/component_index/0?hash=40da91deffd7531391dd067ed89a19703a73d4fdf19fe72651ff30e414c4ef0a",
            "fuchsia-pkg://fuchsia.com/system_image/0?hash=c391b60a35f680b1cf99107309ded12a8219aedb4d296b7fa8a9c5e95ade5e85"
        ]
    }
    ```

*   `/version`
    Same format as the [`/config/build-info/version`](/development/build/build_information.md) file.
*   `/zbi[.signed]`
    Kernel image. Must not be present if the `update-mode` is `force-recovery`. `zbi` or `zbi.signed`
    is required to be present if the `update-mode` is `normal`.
*   `/recovery`
    Recovery image
*   `/meta/contents` and `/meta/package`
    Metadata files present in all packages.
*   `/update_mode.json`
    Optional. If the file is not present, the `update-mode` is `normal`. The other option is
    `force-recovery`, which writes a recovery image and reboots into it. Any other `update-mode`
    value is invalid.
    For example:

    ```json
    {
        "version": "1",
        "content": {
            "mode" : "force-recovery"
        }
    }
    ```
