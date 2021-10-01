<!---

# Device firmware

Device firmware are binary blobs containing code that are executed by device
hardware. The binary blob is available in the driver's namespace for loading.

Device firmware are stored in CIPD (Chrome Infrastructure Package Deployment)
and mirrored in Google Storage.

--->

# 设备固件

设备固件是包含代码的二进制blob文件，由设备硬件执行。二进制blob可在驱动命名空间中加载。

设备固件存储在 CIPD （Chrome Infrastructure Package Deployment)中，镜像在Google Storage中。

<!---

## Create a Firmware Package

To create a firmware package, create a directory containing the following
files:

* One or more firmware files
* A license file
* [README.fuchsia](/docs/concepts/source_code/third-party-metadata.md)

README.fuchsia must contain at least the following directives:

* `Name`
* `Version`
* `Upstream Git`
* `License`
* `License File`

If this is the first time you uploaded to CIPD from the host system,
authenticate with CIPD:

--->

## 创建固件包

为了创建固件包，首先要创建一个目录包含以下文件：

* 一个或多个固件文件
* 一个license文件
* [README.fuchsia](/docs/concepts/source_code/third-party-metadata.md)

README.fuchsia必须包含至少以下指示符：

* `Name`
* `Version`
* `Upstream Git`
* `License`
* `License File`

如果这是你第一次从主机系统中上传到 CIPD 中，

使用以下指令授权 CIPD ：

```
fx cipd auth-login
```

<!---

Upload and tag the package in CIPD using the following command:

--->

使用以下命令在 CIPD 中对包进行上传和打标签：

```
fx cipd create -in <package-directory> -install-mode copy \
    -name <package-name> \
    -tag git_repository:<source-git-repositry> \
    -tag git_revision:<source-git-revision>
```

<!---

`package-name` has the format `fuchsia/firmware/<name>`.

`<name>` should be a string that identifies the firmware. It may contain
any non-whitespace character. It is helpful to identify the driver that will
use the firmware in the name.

After this step, the package is uploaded to CIPD. Check the
[CIPD browser here](https://chrome-infra-packages.appspot.com/#/?path=fuchsia/firmware)
for packages under `fuchsia/firmware`.

--->

`package-name` 格式为 `fuchsia/firmware/<name>`。

`<name>`  是一个字符串来表明固件。它可以包含任意非空格字符。使用固件名称来识别驱动是很有帮助的。

经过这个步骤后，包被上传到 CIPD 中。在`fuchsia/firmware`路径下检查包 [CIPD browser here](https://chrome-infra-packages.appspot.com/#/?path=fuchsia/firmware)。

<!---

## Adding the Firmware Package to the Build

Add the following entry in `prebuilt/zircon.ensure`:

--->

## 添加固件包到构建中

添加下述条目到`prebuilt/zircon.ensure`中：

```
@Subdir firmware/<name>
<package-name> git_revision:<source-git-revision>
```

<!---

Where `<name>`, `<package-name>` and `<source-git-revision>` matches the
values passed to `cipd create` above. The package will be downloaded to
the path specified by `@Subdir` under `prebuilt`, i.e.
`prebuilt/firmware/<name>`.

Next, update `prebuilt/zircon.versions` with the following command:

--->

`<name>`, `<package-name>` 和`<source-git-revision>`匹配值传递给 `cipd create`  上。包将通过`@Subdir`被下载到特定路径 `prebuilt` 下 ，例如，`prebuilt/firmware/<name>`。

接下来，使用以下命令更新`prebuilt/zircon.versions`。

```
scripts/download-prebuilt --resolve
```

<!---

Upload this change to Gerrit and send it to the CQ. The firmware package will
be downloaded by `scripts/download-prebuilt` along with the toolchain and QEMU.

--->

上传改动到 Gerrit 并发送到 CQ 中。固件包将通过`scripts/download-prebuilt`和工具链与 QEMU 一起下载。

<!---

## Using the Firmware Package in the Driver

Add the following line to the driver's `rules.mk`:

--->

## 在驱动中使用固件包

添加下列语句到驱动 `rules.mk`中：

```
MODULE_FIRMWARE := <name>/<path-to-binary-blob>
```

<!---

This will install the firmware to bootfs under
`/boot/lib/firmware/$(basename $(MODULE_FIRMWARE))`.

The `load_firmware()` API, defined in [`driver.h`](/src/lib/ddk/include/lib/ddk/driver.h)
loads the firmware pointed to by the path in a VMO.

--->

这将安装固件到 bootfs ，在路径`/boot/lib/firmware/$(basename $(MODULE_FIRMWARE))`下 。

`load_firmware()` 的API，在 [`driver.h`](/src/lib/ddk/include/lib/ddk/driver.h)中定义，并在 VMO 中加载路径所指向的固件。
