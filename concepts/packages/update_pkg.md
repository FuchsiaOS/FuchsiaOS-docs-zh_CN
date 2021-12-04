<!-- # The update package

The update package is a package containing files and rules for how to update the
system. -->

# 更新包

更新包是一个包含文件以及更新规则的包。

<!-- ## System update

The system update checker looks at the merkle root of the system image that the update
package has and compares it to the merkle root of the running system. It also checks
the merkle root of the update package and compares it to the version that the system
update checker last used. If they're different, then something other than the
system updater has updated the system. -->

## 系统更新

系统更新检查器会查看更新包中系统镜像的墨克根（Merkle Root）并将其与运行中的系统的墨克根进行比较。同时，它还会检查此次更新与系统更新检查器所使用的版本。如果不一致，则说明上次更新并未使用系统更新器。

<!-- The system updater reboots the device after a successful system update.

The system update checker periodically fetches the update package using the package
resolver and sees if it looks different. If the update package is different,
the system triggers a package update. -->

系统更新器会在系统更新成功后重启设备。系统更新检查器会通过包解析器周期性地获取更新包，并检查是否与目前更新包不一致。如果不一致，系统会启动一次更新。

<!-- The system updater is designed such that the process can be interrupted at
any time and it does not leave the system in an unbootable or corrupt state. -->

系统更新器的进程被设计为可随时打断的，因此它不会使系统陷入无法启动或损坏的状态。

<!-- First, the system updater reads the `update_mode` file to determine what operations to
perform. Then, the board file reads and verifies that there are no misconfigurations.
Then, the update package fetches the packages to serve. Finally, the update package writes
the kernel images and ensures that `vbmeta` must be written after the kernel image. -->

首先，系统更新器读取 `update_mode` 文件来决定采取何种操作。然后，读取主板文件来核实有无错误的配置。之后，更新器从服务器获取更新包。最后，更新器写入内核镜像，并确保 `vbmeta` 在内核之后写入。

<!-- ## Content of the update package

The structure of the update package, `fuchsia-pkg://fuchsia.com/update`, contains the following: -->

## 更新包的内容

更新包（`fuchsia-pkg://fuchsia.com/update`）包含如下的结构：

<!-- *   `/board`
    The board name. The updater verifies the contents and does an update only if this value matches
    the previous board name. This check prevents accidentally attempting to update a device to an
    unsupported architecture. For example, attempting to update an `x64` target to an `arm64` build will fail. -->

* `/board` 主板名。更新器会核实该文件的内容，并确保更新包中的值与当前值一致时才会启动更新。这种检查可以阻止对不受支持的架构的设备进行更新。例如，如果尝试在一台 `x64` 的设备上安装 `arm64` 的更新包将会导致失败。

<!-- *   `/bootloader`
    Image of the bootloader firmware. DEPRECATED: please use `firmware` instead. -->

*   `/bootloader`
    bootloader 的镜像。已弃用：请使用 `firmware`。

<!-- *   `/epoch.json`
    Epoch that the system cannot downgrade across via OTA. See
    [RFC-0071](/docs/contribute/governance/rfcs/0071_ota_backstop.md) for more context. For example:

    ```json
    {
        "version": "1",
        "epoch": 5
    }
    ``` -->

*   `/epoch.json`
    系统不能通过 OTA 跨越 Epoch 降级。Epoch 指系统底层发生重大改变的一个版本，详见 [RFC-0071](/docs/contribute/governance/rfcs/0071_ota_backstop.md).下例表示无法通过 OTA 将 epoch 降为 4：

    ```json
    {
        "version": "1",
        "epoch": 5
    }
    ```

<!-- *   `/firmware[_<type>]`
    Firmware image. For example: `firmware`, `firmware_bl2`, `firmware_full`. Each device
    supports a custom set of firmware types, and unsupported types are ignored. This serves
    two main purposes:
    1. Specifying multiple pieces of firmware; for example, devices which have multiple
       bootloader stages.
    2. Providing a simple and safe way to transition to new firmware types; it's just a matter of
       adding the backend paver logic and then putting the new file in the update package. -->

*   `/firmware[_<type>]`
    固件镜像。例如：`firmware`，`firmware_bl2`，`firmware_full`。每种设备支持一套定制的固件类型，不支持的类型会被忽略。这样设计有两种目的：
    1. 可以指定多个固件；例如，当设备是多阶段启动时。
    2. 提供一种简单且安全的方法来转换到新的固件类型；其实就是添加后端铺设逻辑并在更新包中放入新文件这样一档事。

<!-- *   `/packages.json`
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
    ``` -->

*   `/packages.json`
    JSON 格式的附带有哈希值的包的链接，这些包属于目标系统镜像的基础软件包集。更新包会查看 `/packages.json` 来决定哪个包（以及按哪种顺序）升级。例如：

    ```json
    {
        “version”: “1”,
        “content”: [
            "fuchsia-pkg://fuchsia.com/component_index/0?hash=40da91deffd7531391dd067ed89a19703a73d4fdf19fe72651ff30e414c4ef0a",
            "fuchsia-pkg://fuchsia.com/system_image/0?hash=c391b60a35f680b1cf99107309ded12a8219aedb4d296b7fa8a9c5e95ade5e85"
        ]
    }
    ```

<!-- *   `/version`
    Same format as the [`/config/build-info/version`](/docs/development/build/build_information.md) file. -->

*   `/version`
    与 [`/config/build-info/version`](/docs/development/build/build_information.md) 文件格式相同。

<!-- *   `/zbi[.signed]`
    Kernel image. Must not be present if the `update-mode` is `force-recovery`. `zbi` or `zbi.signed`
    is required to be present if the `update-mode` is `normal`. -->

*   `/zbi[.signed]`
    内核镜像。在 `update-mode` 为 `force-recovery` 时必须不包含此项。在 `update-mode` 为 `normal` 时，则必须包含`zbi` 或 `zbi.signed` 此项。

<!-- *   `/zedboot[.signed]`
    Recovery image -->

*   `/zedboot[.signed]`
    恢复镜像。

<!-- *   `/meta/contents` and `/meta/package`
    Metadata files present in all packages. -->

*   `/meta/contents` 与 `/meta/package`
    所有包中都有的元数据文件。

<!-- *   `/update_mode.json`
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
    ``` -->

*   `/update_mode.json`
    可选项。如果没有此文件，则 `update-mode` 是 `normal`。如果有此文件，则处于 `force-recovery` 模式。该模式下会写入一个恢复镜像并重启进入该镜像。其它的 `update-mode` 值均不合法。例如： 
    ```json
    {
        "version": "1",
        "content": {
            "mode" : "force-recovery"
        }
    }
    ```

