# OTA updates

Over-The-Air updates (OTAs) are a mechanism for operating system updates on
Fuchsia. This document details how OTA updates work on Fuchsia.

The update process is divided into the following phases:

* [Checking for an update](#checking-for-update)
* [Monitoring update progress](#monitoring-update)
* [Staging an update](#staging-update)
* [Verifying an update](#verifying-update)

## Checking for an update {#checking-for-update}

The two entry points for the operating system update process are the `omaha-client`
and the `system-update-checker` components.

Both the `omaha-client` and `system-update-checker` serve the same
purpose, to find out if there is an operating system update and start the update.

Note: Omaha is an update availability management protocol. For more
information about Omaha, see [Omaha](https://github.com/google/omaha).

Generally, products should use `omaha-client` if they want to
use Omaha to determine update availability. Products should use
`system-update-checker` if they don’t want to use Omaha and instead
want to check for updates directly from package repositories.

On any given Fuchsia system, only one of these components may be running:

* [Update checks with omaha-client](#update-omaha)
* [Update checks with system-update-checker](#update-system)

### Update checks with omaha-client {#update-omaha}

During the boot process, `omaha-client` starts up and begins periodic update
checks. During these checks, `omaha-client` polls the Omaha server to check for
updates.

The benefits of using Omaha are:

Note: Omaha is an update availability management protocol. For more
information about Omaha, see [Omaha](https://github.com/google/omaha).

* It allows for a fractional rollout of system updates across a fleet of
  Fuchsia devices. For example, it can be configured that only 10% of the
  fleet of devices gets updated. This means that only 10% of these devices
  will see that there is an available update while polling Omaha. The
  remaining 90% of devices would not see an update available.
* It allows for different update channels. For example, test devices can get
  updates from the test channel and get the newest (possibly unstable)
  software. This allows production devices to get updates from the production
  channel and get the most stable software. Channel information can
  be optionally given to Omaha along with product and version.

![Figure: Checking for updates with omaha-client](images/omcl.png)

**Figure 1**. A simplified version of the update check process with `omaha-client`. There are
policies that gate whether `omaha-client` can check for an update or apply an update.

Once `omaha-client` gets the update package URL from the Omaha server, `omaha-client`
tells the `system-updater` to start an update.

### Update checks with system-update-checker {#update-system}

Devices that don’t use `omaha-client` use the `system-update-checker`. Depending
on how it is [configured], the `system-update-checker` regularly polls for
an update package. These checks default to disabled if no `auto_update` is
specified.

To check if an update is available, the `system-update-checker` checks the
following conditions:

* Is the hash of the currently running system image (located in `/pkgfs/system/meta`) different from
  the hash of system image (found in `packages.json`) in the update package?
* If the system image isn’t different, is the vbmeta that’s currently running on the system
  different from the vbmeta of the update package?
* If there is no vbmeta, is the ZBI that’s currently running on the system different from the ZBI
  of the update package?

If any of these answers are yes, then the `system-update-checker` knows the
update package has changed. Once the system-update-checker realizes the update
package has changed, the `system-update-checker` triggers the `system-updater`
to start an update using the default update package (fuchsia-pkg://fuchsia.com/update).

![Figure: Checking for updates with the system-update-checker](images/system-update-checker.png)

**Figure 2**. A simplified version of the update check process with the `system-update-checker`.

Note: There is currently no way to check bootloader-only updates because
there is no [paver API] to read the firmware. An update is not triggered
even though the update package has changed. Until this is fixed, you
should use `update force-install <update-pkg-url>` to force an update.

If no update is required, the update checker saves the last known update
package hash. On subsequent checks for an update, the hash
of the update package that is fetched is checked against the last known
hash. If the hashes are the same, no update is triggered. If the hashes
are different, the vbmeta and ZBI are  checked for changes to determine
if an update is necessary.

## Monitoring {#monitoring-update}

If a client is interested in monitoring update progress and status, they could implement
[`fuchsia.update.AttemptsMonitor`][attempts-monitor-fidl] protocol and provide the client end to
`MonitorAllUpdateChecks()` method of [`fuchsia.update.Manager`][update-manager-fidl] FIDL
protocol. [`fuchsia.update.AttemptsMonitor`][attempts-monitor-fidl] instance will only receive
messages when an update is started by another method, or if an update is currently in progress. This
will not trigger a new update.

[`fuchsia.update.AttemptsMonitor`][attempts-monitor-fidl] instance will receive `OnStart` message
which will contain server end to the [`fuchsia.update.Monitor`][monitor-fidl] protocol. This allows
client to receive and process `OnState` messages, informing about the update state changes.

Another option is to implement [`fuchsia.update.Monitor`][monitor-fidl] and provide the client end
to `CheckNow()` method of the [`fuchsia.update.Manager`][update-manager-fidl] protocol. This will
start [checking for an update](#checking-for-update). It will only monitor the update that's
currently running and will close the handle once the update completes.

## Staging an update {#staging-update}

Regardless of whether an update was triggered by `omaha-client`, `system-update-checker`,
or even a forced update check, an update needs to be written to disk.

The update process is divided into the following steps:

* [Fetch update package](#fetch-update-package)
* [Verify board matches](#verify-board)
* [Verify epoch is supported](#verify-epoch)
* [Replace retained packages set](#retained-packages)
* [Trigger garbage collection](#garbage-collection)
* [Fetch remaining packages](#fetch-reamaining-packages)
* [Write images to block device](#write-images-block-device)
* [Set alternate partition as active](#set-alternate-active)
* [Reboot](#reboot)

![Figure: Starting state diagram](images/starting-state.png)

**Figure 3**. The device is currently running hypothetical OS version 1 (on slot A) and begins to
update to hypothetical OS version 2 (to slot B). *Warning*: this may not be how the disk is
partitioned in practice.

### Fetch update package {#fetch-update-package}

The `system-updater` fetches the [update package], using the
[provided update package URL][system-updater-url-fidl]. The dynamic index is then updated to
reference the new update package. A sample update package may look like this:

```
/board
/epoch.json
/firmware
/fuchsia.vbmeta
/packages.json
/recovery.vbmeta
/version
/zbi.signed
/zedboot.signed
/meta/contents
/meta/package
```

If the fetch fails because there's not enough space, the `system-updater` will trigger garbage
collection to delete all BLOBs that aren’t referenced in either the static or dynamic indexes or
the retained packages set. After garbage collection, the `system-updater` will retry the fetch. If
the retry fails, the `system-updater` will [replace][replace-retained-packages-fidl] the retained
packages set with just the update package it is trying to fetch (if the update package URL
included the [hash](concepts/packages/package_url.md#package-hash), otherwise it will clear
the retained package set) and then again trigger garbage collection and retry the update package
fetch.

![Figure: Fetch update package](images/resolve-update-pkg.png)

**Figure 4**. The `system-updater` instructs the `pkg-resolver` to resolve the version 2
update package. We assume the `system-updater` failed to fetch the update package because of
inadequate space, triggered a garbage collection to evict the version 0 blobs referenced by slot B,
and then retried to successfully fetch the version 2 update package.

Optionally, the update package may contain an `update-mode` file. This file
determines whether the system update happens in Normal or ForceRecovery
mode. If the update-mode file is not present, the `system-updater`
defaults to the Normal mode.

When the mode is ForceRecovery, the `system-updater` writes an image to recovery,
marks slots A and B as unbootable, then boots to recovery. For more information,
see the [implementation of ForceRecovery][recovery-mode-code].

### Verify board matches {#verify-board}

The current running system has a board file located in `/config/build-info/board`.
The `system-updater` verifies that the board file on the system matches the board
file in the update package.

![Figure: Verify board matches](images/verify-board.png)

**Figure 5**. The `system-updater` verifies the board in the update package matches the board
on slot A.

### Verify epoch is supported {#verify-epoch}

The update package contains an epoch file (`epoch.json`). If the epoch of the update
package (the target epoch) is less than the epoch of the `system-updater`
(the source epoch), the OTA fails. For additional context,
see [RFC-0071](contribute/governance/rfcs/0071_ota_backstop.md).

![Figure: Verify epoch is supported](images/verify-epoch.png)

**Figure 6**. The `system-updater` verifies the epoch in the update package is supported by
comparing it to the epoch of the current OS.

### Replace retained packages set {#retained-packages}

[Replace][replace-retained-packages-fidl] the retained packages set with the current update package
and all of the packages that will be fetched later in the OTA process.

The retained packages set is a set of packages that are protected from garbage collection (in
addition to the packages in the static and dynamic indexes). It is used to prevent garbage
collection from deleting BLOBs needed by the current update process. For example, consider a device
that fetched some of the packages needed for an update and then rebooted for unrelated reasons. When
the device starts to OTA again, it still needs the packages it fetched before rebooting, but those
packages are not protected by the dynamic index (which, like the retained packages set, is cleared
on reboot). By adding those packages to the retained packages set, the `system-updater` can then
trigger garbage collection (to e.g. remove blobs used by a previous system version) without undoing
past work.

### Trigger garbage collection {#garbage-collection}

Garbage collection is triggered to delete all BLOBs exclusive to the old system.
This step frees up additional space for any new packages.

![Figure: Garbage collection](images/gc.png)

**Figure 7**. The `system-updater` instructs `pkg-cache` to garbage collect all BLOBs exclusive to
the old system. In this example, it means `pkg-cache` will evict BLOBs exclusively referenced by
the version 1 update package.

### Fetch remaining packages {#fetch-reamaining-packages}

The system-updater parses the `packages.json` file in the update package.
The `packages.json` looks like the following:

```json
{
  "version": “1”,
  "content": [
    "fuchsia-pkg://fuchsia.com/sshd-host/0?hash=123..abc",
    "fuchsia-pkg://fuchsia.com/system-image/0?hash=456..def"
    ...
  ]
}
```

The `system-updater` instructs the `pkg-resolver` to resolve all the package URLs. When resolving
packages, the package management system only fetches BLOBs that are required for an update, i.e.
only those BLOBs that aren't already present. The package management system fetches entire BLOBs,
as opposed to a diff of whatever might currently be on the system.

Once all packages have been feteched, a BlobFS sync is triggered to flush the
BLOBs to persistent storage. This process ensures that all the necessary BLOBs
for the system update are available in BlobFS.

![Figure: Fetch remaining packages](images/resolve-packages.png)

**Figure 8**. The `system-updater` instructs the pkg-resolver to resolve the version 2
packages referenced in `packages.json`.

### Write images to block device {#write-images-block-device}

The `system-updater` determines which images need to be written to the block
device. There are two kinds of images, assets and firmware.

Note: For more information on how this works, see the [`update.rs`][update-rs] file.
To see the difference between assests and firmware images, see the [`paver.rs`][image-types] file.

Then, the `system-updater` instructs the paver to write the bootloader and
firmware. The final location of these images does not depend on whether
the device supports [ABR][glossary.ABR]. To prevent flash wear,
the image is only written to a partition if the image is different from the
image that already exists on the block device.

Note: To see more information on how the Fuchsia paver works for the firmware,
see [`fuchsia.paver`][fuchsia-paver-firmware].

Then, the `system-updater` instructs the paver to write the Fuchsia ZBI and its
vbmeta. The final location of these images depends on whether the device
supports [ABR][glossary.ABR]. If the device supports
[ABR][glossary.ABR], the paver writes the Fuchsia ZBI and
its vbmeta to the slot that’s not currently booted (the alternate slot).
Otherwise, the paver writes them to both the A and B partitions (if a B
partition exists).

Note: To see more information on how the Fuchsia paver works for assets,
see [`fuchsia.paver`][fuchsia-paver-assets].

Finally, the `system-updater` instructs the paver to write the recovery
ZBI and its vbmeta. Like the bootloader and firmware, the final
location does not depend on if the device supports [ABR][glossary.ABR].

![Figure: Write images to block device](images/write-images.png)

**Figure 9**. The `system-updater` writes the version 2 images to slot B via the paver.

### Set alternate partition as active {#set-alternate-active}

If the device supports ABR, the `system-updater` uses the paver to set the
alternate partition as active. That way, the device boots into the alternate
partition on the next boot.

There are a several ways to refer to the slot state. For example, the internal
paver uses `Successful` while the [FIDL service] uses `Healthy`, while other
cases may use Active, Inactive, Bootable, Unbootable, Current, Alternate, etc...

Note: For more information on how this is implemented, see
[`data.h`][abr-slot-data].

The important metadata is 3 pieces of information that is stored for each kernel
slot. This information helps determine the state of each kernel slot. For
example, before slot B is marked as active, the metadata might look like:

|     Metadata    | Slot A | Slot B |
|:---------------:|:------:|:------:|
|     Priority    |   15   |    0   |
| Tries Remaining |    0   |    0   |
|     Healthy*    |    1   |    0   |

After slot B is marked as active, the metadata would look like:

|     Metadata    | Slot A | Slot B |
|:---------------:|:------:|:------:|
|     Priority    | **14** |**15****|
| Tries Remaining |    0   |**7**** |
|     Healthy     |    1   |    0   |

Note: These numbers are based on hardcoded values for priority and remaining
tries which are defined in [`data.h`][kAbrMaxPriority].

If the device doesn’t support ABR, this check is skipped since there is no alternate partition. Instead,
there is an active partition that is written to for every update.

![Figure: Set alternate partition as active](images/modify-boot-metadata.png)

**Figure 10**. The `system-updater` sets slot B to Active, so that the device boots into slot B
on the next boot.

### Reboot {#reboot}

Depending on the update configuration, the device may or may not reboot. After the device
reboots, the device boots into the new slot.

![Figure: Reboot](images/reboot.png)

**Figure 11**. The device reboots into slot B and begins running version 2.

## Verifying an update {#verifying-update}

The system commits an update once that update is verified by the system.

The system verifies the update in the following way:

* [Rebooting into the update version](#reboot-update)
* [Committing the update](#commiting-update)

### Rebooting into the updated version {#reboot-update}

Note: In this example, it is assumed that the update is written to partition B.

On the next boot, the bootloader needs to determine which slot to boot into.
In this example, the bootloader determines to boot into slot B because
slot B has a higher priority and more than 0 tries remaining (see
[Set alternate partition as active](#set-alternate-active)). Then, the
bootloader verifies the ZBI of B matches the vbmeta of B, and finally boots into
slot B.

Note: For more information on how the bootloader determines the slot to boot
into, see [`flow.c`][flow-c].

After early boot, `fshost` launches `pkgfs` using the new system-image package.
This is the system image package that is referenced in the `packages.json`
while staging the update. The system-image package has a `static_packages` file
in it that lists the base packages for the new system. For example:

```
pkg-resolver/0 = new-version-hash-pkg-resolver
foo/0 = new-version-hash-foo
bar/0 = new-version-hash-bar
...
// Note the system-image package is not referenced in static_packages
// because it's impossible for it to refer to its own hash.
```

`pkgfs` then loads all these packages as base packages. The packages appear in
`/pkgfs/{packages, versions}`, which indicate that the packages are installed
or activated. Then, `appmgr` starts which then starts the `pkg-resolver`,
`pkg-cache`, `netstack`, etc...

### Committing the update {#commiting-update}

The `system-update-committer` component runs various checks to verify if the
new update was successful. For example, it instructs BlobFs to arbitrarily
read 1MiB of data. If the system is already committed on boot, these checks are
skipped. If the check fails and depending on how the system is configured, the
`system-update-committer` may trigger a reboot.

After the update is verified, the current partition (slot B) is marked as
`Healthy`. Using the example described in
[Set alternate partition as active](#set-alternate-active), the boot
metadata may now look like:

|     Metadata    | Slot A | Slot B |
|:---------------:|:------:|:------:|
|     Priority    |   14   |   15   |
| Tries Remaining |  **7** |  **0** |
|     Healthy     |  **0** |  **1** |

Then, the alternate partition (slot A) is marked as unbootable. Now, the
boot metadata may look like:

|     Metadata    | Slot A | Slot B |
|:---------------:|:------:|:------:|
|     Priority    |  **0** |   15   |
| Tries Remaining |  **0** |    0   |
|     Healthy     |    0   |    1   |

After this, the update is considered committed. This means:

* The system always boots into slot B until the next system update.
* The system gives up booting into slot A until the next system update
  overwrites slot A.
* The BLOBs referenced by slot A are now able to be garbage collected.
* Subsequent system updates are now allowed. When the update checker
  discovers a new update, the whole update process starts again.


[glossary.ABR]: glossary/README.md#ABR
[configured]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/pkg/bin/system-update-checker/BUILD.gn;l=114;drc=50245a9ce68f3b877e165b004175e2a4fc12eaef
[paver API]: https://fuchsia.dev/reference/fidl/fuchsia.paver#DataSink
[update package]: concepts/packages/update_pkg.md
[recovery-mode-code]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/pkg/bin/system-updater/src/update.rs;l=429;drc=202c37fa01f75c431f61ca824b4d2f7c2ec82178
[need]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/pkg/bin/pkg-resolver/src/cache.rs;l=275;drc=c557680c2d1d59f4ec4f31981b08610bec7c8514
[update-rs]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/pkg/bin/system-updater/src/update.rs;l=507;drc=202c37fa01f75c431f61ca824b4d2f7c2ec82178
[image-types]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/pkg/bin/system-updater/src/update/paver.rs;l=200;drc=216f7ea082148714bac1e95299c1bc8b087dc1d8
[fuchsia-paver-firmware]: https://fuchsia.dev/reference/fidl/fuchsia.paver#DynamicDataSink.WriteFirmware
[fuchsia-paver-assets]: https://fuchsia.dev/reference/fidl/fuchsia.paver#DynamicDataSink.WriteAsset
[abr-slot-data]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/firmware/lib/abr/include/lib/abr/data.h;l=32;drc=bea16aa2d8a0bbc293a82ed44e03525ebe13bc94
[Successful]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/firmware/lib/abr/include/lib/abr/data.h;l=43;drc=bea16aa2d8a0bbc293a82ed44e03525ebe13bc94
[FIDL service]: https://fuchsia.dev/reference/fidl/fuchsia.paver#BootManager.SetActiveConfigurationHealthy
[kAbrMaxPriority]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/firmware/lib/abr/include/lib/abr/data.h;l=28;drc=bea16aa2d8a0bbc293a82ed44e03525ebe13bc94
[flow-c]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/firmware/lib/abr/flow.c;l=197;drc=bea16aa2d8a0bbc293a82ed44e03525ebe13bc94
[replace-retained-packages-fidl]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.pkg/cache.fidl;l=216;drc=a265f6e224c76f783a14bce7c24b085b90cc3ad8
[system-updater-url-fidl]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/pkg/fidl/fuchsia.update.installer/installer.fidl;l=53;drc=896f3220d71b442b44da13bc04a5634993488330
[update-manager-fidl]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.update/update.fidl;l=12;drc=ad0a3e8d6b96313a92807556c50e1935aa377a45
[attempts-monitor-fidl]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.update/update.fidl;l=104;drc=ad0a3e8d6b96313a92807556c50e1935aa377a45
[monitor-fidl]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.update/update.fidl;l=128;drc=ad0a3e8d6b96313a92807556c50e1935aa377a45
