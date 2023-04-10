# GN Build Arguments

## All builds

### acpica_debug_output

Enable debug output in the ACPI library (used by the ACPI bus driver).

**Current value (from the default):** `false`

From //zircon/system/ulib/acpica/acpica.gni:7

### active_partition

**Current value (from the default):** `""`

From //build/images/args.gni:118

### add_qemu_to_build_archives

Whether to include images necessary to run Fuchsia in QEMU in build
archives.

**Current value (from the default):** `false`

From //build/images/args.gni:124

### additional_bootserver_arguments

Additional bootserver args to add to pave.sh. New uses of this should be
added with caution, and ideally discussion. The present use case is to
enable throttling of netboot when specific network adapters are combined
with specific boards, due to driver and hardware challenges.

**Current value (from the default):** `""`

From //build/images/args.gni:130

### all_cpu_phys_boot_tests

Cause //zircon/kernel/phys:boot_tests to generate the phys boot tests
for all supported CPUs, not just $target_cpu.

**Current value (from the default):** `false`

From //zircon/kernel/phys/BUILD.gn:22

### all_font_file_paths

List of file paths to every font asset. Populated in fonts.gni.

**Current value (from the default):** `[]`

From //src/fonts/build/font_args.gni:35

### all_toolchain_variants

*These should never be set as a build argument.*
It will be set below and passed to other toolchains through toolchain_args
(see variant_toolchain.gni).

**Current value (from the default):** `[]`

From //build/config/BUILDCONFIG.gn:1850

### allow_legacy_data_partition_names

Set to true to enable legacy data partition names.

**Current value (from the default):** `false`

From //src/storage/fshost/generated_fshost_config.gni:35

### allowed_test_device_types

A list of device types this build is allowed to run tests on.

**Current value (from the default):** `[]`

From //build/testing/test_spec.gni:11

### always_zedboot

Build boot images that prefer Zedboot over local boot (only for EFI).

**Current value (from the default):** `false`

From //build/images/args.gni:152

### api_compatibility_testing

**Current value (from the default):** `true`

From //build/config/fuchsia/platform_version.gni:38

### appmgr_core_shards

Core shards that are required for including appmgr in a product.

**Current value (from the default):** `["//src/sys/appmgr:appmgr_core_shard"]`

From //src/sys/appmgr/core_shards.gni:7

### arm_sdk_tools

If true, then the arm64 host tools are included in the SDK.

**Current value (from the default):** `false`

From //src/developer/ffx/plugins/emulator/emu_companion.gni:9

### asan_default_options

Default [AddressSanitizer](https://clang.llvm.org/docs/AddressSanitizer.html)
options (before the `ASAN_OPTIONS` environment variable is read at
runtime).  This can be set as a build argument to affect most "asan"
variants in $variants (which see), or overridden in $toolchain_args in
one of those variants.  This can be a list of strings or a single string.

Note that even if this is empty, programs in this build **cannot** define
their own `__asan_default_options` C function.  Instead, they can use a
sanitizer_extra_options() target in their `deps` and then any options
injected that way can override that option's setting in this list.

**Current value (from the default):** `["detect_stack_use_after_return=1", "quarantine_size_mb=64"]`

From //build/config/sanitizers/sanitizer_default_options.gni:16

### assembly_fshost

Skip configuring fshost in GN and do it via Assembly instead.
Used as a parameter to assembled_system().

**Current value (from the default):** `false`

From //build/images/args.gni:185

### authorized_ssh_keys

Support SSH authorized_keys files as a build argument. This is only available
for non-production builds and requires inclusion of configuration data from
"//src/developer/sshd-host:authorized_ssh_keys_config_data".

**Current value (from the default):** `[]`

From //build/images/args.gni:172

### auto_update_packages

Controls the behavior of sysmgr's PackageUpdatingLoader (v1) and the
full-resolver (v2). If true, when resolving a component an attempt to
update the component's package is first made through the Software Delivery
system (specifically, through the package resolver,
fuchsia.pkg.PackageResolver). If false, no attempt to update is made and
components are loaded only from packages already available locally (for
example, because the package is in base).

**Current value (from the default):** `true`

From //build/security.gni:227

### av400_has_codec

**Current value (from the default):** `true`

From //src/devices/board/drivers/av400/BUILD.gn:9

### av400_has_loopback

**Current value (from the default):** `false`

From //src/devices/board/drivers/av400/BUILD.gn:10

### av400_pdm_use_dsp

**Current value (from the default):** `false`

From //src/devices/board/drivers/av400/BUILD.gn:14

### av400_tdm_use_dsp

Audio data can be processed using HW DSP FW.

**Current value (from the default):** `false`

From //src/devices/board/drivers/av400/BUILD.gn:13

### avb_algorithm

**Current value (from the default):** `"DEPRECATED"`

From //build/images/vbmeta.gni:41

### avb_atx_metadata

AVB metadata which will be used to validate public key

**Current value for `target_cpu = "arm64"`:** `"//third_party/android/platform/external/avb/test/data/atx_metadata.bin"`

From //boards/arm64.gni:37

**Overridden from the default:** `""`

From //build/images/vbmeta.gni:20

**Current value for `target_cpu = "x64"`:** `"//third_party/android/platform/external/avb/test/data/atx_metadata.bin"`

From //boards/x64.gni:64

**Overridden from the default:** `""`

From //build/images/vbmeta.gni:20

### avb_key

a key which will be used to sign VBMETA and images for AVB

**Current value for `target_cpu = "arm64"`:** `"//third_party/android/platform/external/avb/test/data/testkey_atx_psk.pem"`

From //boards/arm64.gni:39

**Overridden from the default:** `""`

From //build/images/vbmeta.gni:17

**Current value for `target_cpu = "x64"`:** `"//third_party/android/platform/external/avb/test/data/testkey_atx_psk.pem"`

From //boards/x64.gni:62

**Overridden from the default:** `""`

From //build/images/vbmeta.gni:17

### base_driver_package_labels

If you add fuchsia_driver_package labels to this variable, any drivers in these packages will
be visible to Driver Manager. These package labels are also considered to be in the
'base' package set (for more info see 'base_package_labels').

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/common/bringup.gni:21

**Overridden from the default:** `[]`

From //BUILD.gn:39

**Current value for `target_cpu = "x64"`:** `[]`

From //products/common/bringup.gni:21

**Overridden from the default:** `[]`

From //BUILD.gn:39

### base_package_labels

If you add package labels to this variable, the packages will be included in
the 'base' package set, which represents the set of packages that are part
of an OTA. These packages are updated as an atomic unit during an OTA
process and are immutable and are a superset of the TCB (Trusted Computing
Base) for a product. These packages are never evicted by the system.

**Current value for `target_cpu = "arm64"`:** `[]`

From //out/not-default/args.gn:15

**Overridden from the default:** `[]`

From //BUILD.gn:47

**Current value for `target_cpu = "x64"`:** `[]`

From //out/not-default/args.gn:15

**Overridden from the default:** `[]`

From //BUILD.gn:47

### basic_env_names

The list of environment names to include in "basic_envs".

**Current value (from the default):** `["emu"]`

From //build/testing/environments.gni:9

### bazel_assembly_targets

When set, include the corresponding Bazel assembly targets in this build.

**Current value (from the default):** `[]`

From //build/images/args.gni:188

### bazel_quiet

Suppress Bazel non-error output

**Current value (from the default):** `true`

From //build/bazel/bazel_action.gni:13

### bless_goldens

TODO(fxbug.dev/100321): delete bless_goldens, to give users time to switch to new gn arg, update_goldens

**Current value (from the default):** `false`

From //build/testing/config.gni:11

### blob_layout_format

The format blobfs should store blobs in.

**Current value (from the default):** `"compact"`

From //build/images/args.gni:142

### blobfs_board_maximum_bytes

In addition to reserving space for inodes and data, fs needs additional
space for maintaining some internal data structures. So the
space required to reserve inodes and data may exceed sum of the space
needed for inodes and data.
maximum_bytes puts an upper bound on the total bytes reserved for inodes,
data bytes and reservation for all other internal fs metadata.
A value of false does not put any upper bound. A filesystem may
reserve few blocks required for its operations.

**Current value (from the default):** `false`

From //build/images/fvm.gni:75

### blobfs_board_minimum_data_bytes

Number of bytes to reserve for data in the fs. This is in addition
to what is reserved, if any, for the inodes. Data bytes constitutes
"usable" space of the fs.
A value of false does not reserve any additional space than minimum
required for the filesystem.

**Current value (from the default):** `false`

From //build/images/fvm.gni:64

### blobfs_board_minimum_inodes

minimum_inodes is the number of inodes to reserve for the fs
A value of false does not reserve any additional space than minimum
required for the filesystem.

**Current value (from the default):** `false`

From //build/images/fvm.gni:56

### blobfs_capacity

Maximum allowable contents for the /blob in a release mode build for
both slot A and slot B of the system.
False means no limit.

**Current value for `target_cpu = "arm64"`:** `10485760000`

From //boards/common/arm64-common.gni:11

**Overridden from the default:** `false`

From //build/images/filesystem_limits.gni:15

**Current value for `target_cpu = "x64"`:** `10485760000`

From //boards/x64.gni:17

**Overridden from the default:** `false`

From //build/images/filesystem_limits.gni:15

### blobfs_enable_streaming_writes

Enable streaming writes by default when mounting Blobfs. Streaming writes can still be enabled
by setting the `streaming_writes` option when mounting Blobfs (e.g. for unit tests).

Streaming writes are only supported when writing delivery blobs or when compression is disabled.

**Current value (from the default):** `false`

From //src/storage/blobfs/BUILD.gn:14

### blobfs_maximum_runtime_bytes

blobfs_maximum_runtime_bytes is an upper bound on the partition size on the device. Partitions
can grow as needed if there are extra slices available in FVM. This limit prevents the blobfs
partition from taking too much space away from other uses.

Pass the empty string for no limit.

**Current value (from the default):** `""`

From //src/storage/fshost/generated_fshost_config.gni:15

### blobfs_num_pager_threads

The number of pager threads to spawn for blobfs.

**Current value (from the default):** `2`

From //src/storage/bin/blobfs/BUILD.gn:11

### blobfs_page_in_metrics_recording

Set this to true when configuring gn args to enable blobfs page-in metrics recording. This will
also increase the inspect VMO size for blobfs to 2 MiB, to accommodate the large number of
metrics entries.

**Current value (from the default):** `false`

From //src/storage/blobfs/BUILD.gn:19

### blobfs_product_maximum_bytes

**Current value (from the default):** `false`

From //build/images/fvm.gni:76

### blobfs_product_minimum_data_bytes

**Current value (from the default):** `false`

From //build/images/fvm.gni:65

### blobfs_product_minimum_inodes

**Current value (from the default):** `false`

From //build/images/fvm.gni:57

### blobfs_size_creep_limit

How much the size of BlobFS contents can be increased in one CL.

**Current value (from the default):** `102400`

From //build/images/size_checker/size_checker_input.gni:91

### board_bootfs_labels

A list of binary labels to include in the ZBI.

**Current value for `target_cpu = "arm64"`:** `["//src/devices/bus/drivers/pci:bus-pci", "//src/devices/usb/drivers/xhci", "//src/power/power-manager:base_config", "//src/security/policy/zxcrypt:null", "//src/devices/block/drivers/nvme", "//src/connectivity/ethernet/drivers/virtio:virtio_netdevice", "//src/devices/block/drivers/virtio:virtio_block", "//src/devices/block/drivers/virtio:virtio_scsi", "//src/devices/misc/drivers/virtio-rng:virtio_rng", "//src/devices/serial/drivers/virtio-console:virtio_console", "//src/graphics/drivers/misc:goldfish_fuchsia_drivers", "//src/graphics/display/drivers/goldfish-display", "//src/graphics/drivers/virtio:virtio_gpu", "//src/ui/input/drivers/virtio:virtio_input", "//src/ui/input/drivers/goldfish_sensor:sensor_driver", "//src/devices/block/drivers/ahci", "//src/devices/board/drivers/acpi-arm64", "//src/devices/board/drivers/qemu-arm64", "//src/devices/rtc/drivers/pl031-rtc"]`

From //boards/arm64.gni:24

**Overridden from the default:** `[]`

From //build/board.gni:50

**Current value for `target_cpu = "x64"`:** `["//src/devices/bin/driver_host2", "//src/devices/block/drivers/ahci", "//src/devices/block/drivers/mbr", "//src/devices/block/drivers/nvme", "//src/devices/block/drivers/pci-sdhci", "//src/devices/block/drivers/sdhci", "//src/devices/board/drivers/x86:platform-bus-x86", "//src/devices/bus/drivers/pci:bus-pci", "//src/devices/i2c/drivers/intel-i2c", "//src/devices/rtc/drivers/intel-rtc", "//src/devices/spi/drivers/intel-gspi", "//src/devices/tpm/drivers/tpm", "//src/devices/usb/drivers/xhci", "//src/graphics/display/drivers/intel-i915", "//src/media/audio/drivers/codecs/alc5514", "//src/media/audio/drivers/codecs/alc5663", "//src/media/audio/drivers/codecs/max98373", "//src/media/audio/drivers/codecs/max98927", "//src/media/audio/drivers/intel-hda/codecs/hdmi:hdmi-audio-codec", "//src/media/audio/drivers/intel-hda/codecs/realtek:realtek-audio-codec", "//src/media/audio/drivers/intel-hda/controller:intel-hda", "//src/ui/input/drivers/ctaphid", "//src/ui/input/drivers/i2c-hid", "//src/ui/input/drivers/pc-ps2", "//src/devices/bin/acpidump", "//src/devices/pci/bin:bootfs", "//src/media/audio/bin/ihda", "//src/power/power-manager:base_config", "//src/security/policy/zxcrypt:null", "//boards/kernel_cmdline:enable-suspend-from-board", "//src/connectivity/ethernet/drivers/gvnic", "//src/connectivity/ethernet/drivers/realtek-8111", "//src/connectivity/ethernet/drivers/third_party/igc", "//src/devices/serial/drivers/uart16550", "//src/graphics/display/drivers/simple:simple.amd-kaveri", "//src/graphics/display/drivers/simple:simple.nv", "//zircon/third_party/dev/ethernet/e1000", "//boards/kernel_cmdline:serial-legacy", "//src/connectivity/ethernet/drivers/virtio:virtio_netdevice", "//src/devices/block/drivers/virtio:virtio_block", "//src/devices/block/drivers/virtio:virtio_scsi", "//src/devices/misc/drivers/virtio-rng:virtio_rng", "//src/devices/serial/drivers/virtio-console:virtio_console", "//src/graphics/drivers/misc:goldfish_fuchsia_drivers", "//src/graphics/display/drivers/goldfish-display", "//src/graphics/drivers/virtio:virtio_gpu", "//src/ui/input/drivers/virtio:virtio_input", "//src/ui/input/drivers/goldfish_sensor:sensor_driver", "//src/graphics/display/drivers/simple:simple.bochs", "//src/graphics/display/drivers/simple:simple.gga", "//src/graphics/display/drivers/simple:simple.intel", "//src/graphics/display/drivers/simple:simple.vmware", "//src/media/audio/drivers/intel-hda/codecs/qemu:qemu-audio-codec", "//src/devices/bin/driver_host2"]`

From //boards/x64.gni:48

**Overridden from the default:** `[]`

From //build/board.gni:50

### board_configs

Configs that are added when targeting this board.

**Current value (from the default):** `[]`

From //build/board.gni:17

### board_core_realm_shards

Core realm shards specific to this board. See //src/sys/core for more
context.

**Current value (from the default):** `[]`

From //build/board.gni:79

### board_description

Human readable board description corresponding to the board name.

**Current value for `target_cpu = "arm64"`:** `"A generic emulated arm64 device."`

From //boards/arm64.gni:12

**Overridden from the default:** `""`

From //build/board.gni:10

**Current value for `target_cpu = "x64"`:** `"A generic x64 device"`

From //boards/x64.gni:15

**Overridden from the default:** `""`

From //build/board.gni:10

### board_display_rotation

Amount of rotation in degrees to apply to a board. Expected values are 0,
90, 180, or 270.

**Current value (from the default):** `0`

From //build/board.gni:14

### board_driver_package_labels

A list of driver package labels to include in the 'base' package set. Used
by the board definition rather than the product definition.

**Current value for `target_cpu = "arm64"`:** `["//bundles/packages/prod:drivers-system", "//src/media/audio/bundles:virtual_audio_driver"]`

From //boards/common/arm64-common.gni:29

**Overridden from the default:** `[]`

From //build/board.gni:21

**Current value for `target_cpu = "x64"`:** `["//bundles/packages/prod:drivers-system", "//src/connectivity/wlan/drivers/third_party/intel/iwlwifi:iwlwifi", "//src/connectivity/wlan/drivers/wlanphy:wlanphy", "//src/devices/acpi:drivers", "//src/graphics/drivers/msd-intel-gen", "//src/media/audio/bundles:virtual_audio_driver"]`

From //boards/common/x64-common.gni:58

**Overridden from the default:** `[]`

From //build/board.gni:21

### board_extra_vbmeta_images

DEPRECATED:  Remove when no boards set a value for these.

**Current value (from the default):** `[]`

From //build/images/vbmeta.gni:40

### board_fastboot_unlock_credentials

A list of paths to the unlock credentials file necessary to unlock this
board's fastboot protocol.

**Current value (from the default):** `[]`

From //build/board.gni:83

### board_fshost_config

A list of fshost options to add to the fshost config.

**Current value (from the default):** `{ }`

From //build/board.gni:67

### board_has_libvulkan_arm_mali

Board files can set this to true if they have a package with a mali libvulkan VCD.

**Current value (from the default):** `false`

From //src/graphics/lib/magma/gnbuild/magma.gni:45

### board_host_labels

A list of binary host tool labels to also build.

**Current value (from the default):** `[]`

From //build/board.gni:53

### board_information_for_assembly

BoardInformation file for use with Product Assembly

**Current value (from the default):** `false`

From //build/board.gni:100

### board_is_emu

Whether or not the board supports emulator/physical devices.
This is used to determine if product bundle metadata should generate a
physical/virtual device spec or both.

**Current value for `target_cpu = "arm64"`:** `true`

From //boards/arm64.gni:29

**Overridden from the default:** `false`

From //build/board.gni:105

**Current value for `target_cpu = "x64"`:** `true`

From //boards/x64.gni:91

**Overridden from the default:** `false`

From //build/board.gni:105

### board_is_phys

**Current value for `target_cpu = "arm64"`:** `false`

From //boards/arm64.gni:30

**Overridden from the default:** `true`

From //build/board.gni:106

**Current value (from the default):** `true`

From //build/board.gni:106

### board_name

Board name used for paving and amber updates.

**Current value for `target_cpu = "arm64"`:** `"arm64"`

From //boards/arm64.gni:11

**Overridden from the default:** `""`

From //build/board.gni:7

**Current value for `target_cpu = "x64"`:** `"x64"`

From //boards/x64.gni:14

**Overridden from the default:** `""`

From //build/board.gni:7

### board_package_labels

A list of package labels to include in the 'base' package set. Used by the
board definition rather than the product definition.

**Current value for `target_cpu = "arm64"`:** `["//src/hwinfo:default_board_config", "//src/graphics/bin/vulkan_loader"]`

From //boards/common/arm64-common.gni:34

**Overridden from the default:** `[]`

From //build/board.gni:43

**Current value for `target_cpu = "x64"`:** `["//src/graphics/bin/vulkan_loader", "//src/hwinfo:default_board_config", "//src/graphics/drivers/intel-gen/icd:libvulkan_intel_gen", "//src/graphics/lib/goldfish-vulkan/gnbuild:goldfish-vulkan", "//src/graphics/lib/goldfish-vulkan/gnbuild:goldfish-vulkan-config", "//src/media/codec/codecs/vaapi:codec_runner_intel_gen_prebuilt"]`

From //boards/common/x64-common.gni:67

**Overridden from the default:** `[]`

From //build/board.gni:43

### board_pdm_firmware_name

The name of the HW DSP FW that handles Pdm audio data.

**Current value (from the default):** `""`

From //src/media/audio/drivers/lib/aml-dsp/BUILD.gn:10

### board_provided_drivers

A list of scopes that describe drivers provided by the board, which are
added to the product assembly configuration as "product-provided" drivers.
These will later be migrated to a "board-provided" driver input to assembly,
but that mechanism is still under development.

Each scope added to this list needs to be in the following form:
  {
    # This is the label that creates the package, this can not be a group
    package_target = "//gn/label/that/to/the/driver:package"

    # These are paths to the driver components within the above package.
    driver_components = [
      "meta/driver_1.cm",
      "meta/driver_2.cm",
    ]
  }

**Current value (from the default):** `[]`

From //build/board.gni:39

### board_recovery_bootfs_labels

A list of binary labels to include in the recovery ZBI.

**Current value for `target_cpu = "arm64"`:** `["//src/devices/bus/drivers/pci:bus-pci", "//src/devices/usb/drivers/xhci", "//src/power/power-manager:base_config", "//src/security/policy/zxcrypt:null", "//src/devices/block/drivers/nvme", "//src/connectivity/ethernet/drivers/virtio:virtio_netdevice", "//src/devices/block/drivers/virtio:virtio_block", "//src/devices/block/drivers/virtio:virtio_scsi", "//src/devices/misc/drivers/virtio-rng:virtio_rng", "//src/devices/serial/drivers/virtio-console:virtio_console", "//src/graphics/drivers/misc:goldfish_fuchsia_drivers", "//src/graphics/display/drivers/goldfish-display", "//src/graphics/drivers/virtio:virtio_gpu", "//src/ui/input/drivers/virtio:virtio_input", "//src/ui/input/drivers/goldfish_sensor:sensor_driver", "//src/devices/block/drivers/ahci", "//src/devices/board/drivers/acpi-arm64", "//src/devices/board/drivers/qemu-arm64", "//src/devices/rtc/drivers/pl031-rtc"]`

From //boards/arm64.gni:25

**Overridden from the default:** `[]`

From //build/board.gni:64

**Current value for `target_cpu = "x64"`:** `["//src/devices/bin/driver_host2", "//src/devices/block/drivers/ahci", "//src/devices/block/drivers/mbr", "//src/devices/block/drivers/nvme", "//src/devices/block/drivers/pci-sdhci", "//src/devices/block/drivers/sdhci", "//src/devices/board/drivers/x86:platform-bus-x86", "//src/devices/bus/drivers/pci:bus-pci", "//src/devices/i2c/drivers/intel-i2c", "//src/devices/rtc/drivers/intel-rtc", "//src/devices/spi/drivers/intel-gspi", "//src/devices/tpm/drivers/tpm", "//src/devices/usb/drivers/xhci", "//src/graphics/display/drivers/intel-i915", "//src/media/audio/drivers/codecs/alc5514", "//src/media/audio/drivers/codecs/alc5663", "//src/media/audio/drivers/codecs/max98373", "//src/media/audio/drivers/codecs/max98927", "//src/media/audio/drivers/intel-hda/codecs/hdmi:hdmi-audio-codec", "//src/media/audio/drivers/intel-hda/codecs/realtek:realtek-audio-codec", "//src/media/audio/drivers/intel-hda/controller:intel-hda", "//src/ui/input/drivers/ctaphid", "//src/ui/input/drivers/i2c-hid", "//src/ui/input/drivers/pc-ps2", "//src/devices/bin/acpidump", "//src/devices/pci/bin:bootfs", "//src/media/audio/bin/ihda", "//src/power/power-manager:base_config", "//src/security/policy/zxcrypt:null", "//src/connectivity/ethernet/drivers/gvnic", "//src/connectivity/ethernet/drivers/realtek-8111", "//src/connectivity/ethernet/drivers/third_party/igc", "//src/devices/serial/drivers/uart16550", "//src/graphics/display/drivers/simple:simple.amd-kaveri", "//src/graphics/display/drivers/simple:simple.nv", "//zircon/third_party/dev/ethernet/e1000", "//boards/kernel_cmdline:serial-legacy", "//src/connectivity/ethernet/drivers/virtio:virtio_netdevice", "//src/devices/block/drivers/virtio:virtio_block", "//src/devices/block/drivers/virtio:virtio_scsi", "//src/devices/misc/drivers/virtio-rng:virtio_rng", "//src/devices/serial/drivers/virtio-console:virtio_console", "//src/graphics/drivers/misc:goldfish_fuchsia_drivers", "//src/graphics/display/drivers/goldfish-display", "//src/graphics/drivers/virtio:virtio_gpu", "//src/ui/input/drivers/virtio:virtio_input", "//src/ui/input/drivers/goldfish_sensor:sensor_driver", "//src/graphics/display/drivers/simple:simple.bochs", "//src/graphics/display/drivers/simple:simple.gga", "//src/graphics/display/drivers/simple:simple.intel", "//src/graphics/display/drivers/simple:simple.vmware", "//src/media/audio/drivers/intel-hda/codecs/qemu:qemu-audio-codec", "//src/devices/bin/driver_host2"]`

From //boards/x64.gni:49

**Overridden from the default:** `[]`

From //build/board.gni:64

### board_recovery_package_labels

A list of package labels to include in the recovery package set. Used by the
board definition rather than the product definition.

**Current value (from the default):** `[]`

From //build/board.gni:47

### board_supports_update_configurator

Whether or not the board pulls in the system-update-configurator component.

**Current value (from the default):** `false`

From //build/board.gni:97

### board_system_image_deps

A list of binary labels to include in the system_image package.

**Current value (from the default):** `[]`

From //build/board.gni:75

### board_tdm_firmware_name

The name of the HW DSP FW that handles Tdm audio data.

**Current value (from the default):** `""`

From //src/media/audio/drivers/lib/aml-dsp/BUILD.gn:7

### board_tools

List of paths to board-specific tools to include in the build output.

Most development tools can just be used in-tree and do not need to be
included here. This arg is only meant for tools which may need to be
distributed along with the build files, for example tools for flashing
from SoC recovery mode.

Assets included in this way are included best-effort only and do not form
any kind of stable contract for users of the archive.

**Current value (from the default):** `[]`

From //build/board.gni:94

### board_zedboot_bootfs_labels

A list of binary labels to include in the zedboot ZBI.

**Current value for `target_cpu = "arm64"`:** `["//src/devices/bus/drivers/pci:bus-pci", "//src/devices/usb/drivers/xhci", "//src/power/power-manager:base_config", "//src/security/policy/zxcrypt:null", "//src/devices/block/drivers/nvme", "//src/connectivity/ethernet/drivers/virtio:virtio_netdevice", "//src/devices/block/drivers/virtio:virtio_block", "//src/devices/block/drivers/virtio:virtio_scsi", "//src/devices/misc/drivers/virtio-rng:virtio_rng", "//src/devices/serial/drivers/virtio-console:virtio_console", "//src/graphics/drivers/misc:goldfish_fuchsia_drivers", "//src/graphics/display/drivers/goldfish-display", "//src/graphics/drivers/virtio:virtio_gpu", "//src/ui/input/drivers/virtio:virtio_input", "//src/ui/input/drivers/goldfish_sensor:sensor_driver", "//src/devices/block/drivers/ahci", "//src/devices/board/drivers/acpi-arm64", "//src/devices/board/drivers/qemu-arm64", "//src/devices/rtc/drivers/pl031-rtc"]`

From //boards/arm64.gni:26

**Overridden from the default:** `[]`

From //build/board.gni:61

**Current value for `target_cpu = "x64"`:** `["//src/devices/bin/driver_host2", "//src/devices/block/drivers/ahci", "//src/devices/block/drivers/mbr", "//src/devices/block/drivers/nvme", "//src/devices/block/drivers/pci-sdhci", "//src/devices/block/drivers/sdhci", "//src/devices/board/drivers/x86:platform-bus-x86", "//src/devices/bus/drivers/pci:bus-pci", "//src/devices/i2c/drivers/intel-i2c", "//src/devices/rtc/drivers/intel-rtc", "//src/devices/spi/drivers/intel-gspi", "//src/devices/tpm/drivers/tpm", "//src/devices/usb/drivers/xhci", "//src/graphics/display/drivers/intel-i915", "//src/media/audio/drivers/codecs/alc5514", "//src/media/audio/drivers/codecs/alc5663", "//src/media/audio/drivers/codecs/max98373", "//src/media/audio/drivers/codecs/max98927", "//src/media/audio/drivers/intel-hda/codecs/hdmi:hdmi-audio-codec", "//src/media/audio/drivers/intel-hda/codecs/realtek:realtek-audio-codec", "//src/media/audio/drivers/intel-hda/controller:intel-hda", "//src/ui/input/drivers/ctaphid", "//src/ui/input/drivers/i2c-hid", "//src/ui/input/drivers/pc-ps2", "//src/devices/bin/acpidump", "//src/devices/pci/bin:bootfs", "//src/media/audio/bin/ihda", "//src/power/power-manager:base_config", "//src/security/policy/zxcrypt:null", "//src/connectivity/ethernet/drivers/gvnic", "//src/connectivity/ethernet/drivers/realtek-8111", "//src/connectivity/ethernet/drivers/third_party/igc", "//src/devices/serial/drivers/uart16550", "//src/graphics/display/drivers/simple:simple.amd-kaveri", "//src/graphics/display/drivers/simple:simple.nv", "//zircon/third_party/dev/ethernet/e1000", "//boards/kernel_cmdline:serial-legacy", "//src/connectivity/ethernet/drivers/virtio:virtio_netdevice", "//src/devices/block/drivers/virtio:virtio_block", "//src/devices/block/drivers/virtio:virtio_scsi", "//src/devices/misc/drivers/virtio-rng:virtio_rng", "//src/devices/serial/drivers/virtio-console:virtio_console", "//src/graphics/drivers/misc:goldfish_fuchsia_drivers", "//src/graphics/display/drivers/goldfish-display", "//src/graphics/drivers/virtio:virtio_gpu", "//src/ui/input/drivers/virtio:virtio_input", "//src/ui/input/drivers/goldfish_sensor:sensor_driver", "//src/graphics/display/drivers/simple:simple.bochs", "//src/graphics/display/drivers/simple:simple.gga", "//src/graphics/display/drivers/simple:simple.intel", "//src/graphics/display/drivers/simple:simple.vmware", "//src/media/audio/drivers/intel-hda/codecs/qemu:qemu-audio-codec", "//src/devices/bin/driver_host2"]`

From //boards/x64.gni:50

**Overridden from the default:** `[]`

From //build/board.gni:61

### board_zedboot_cmdline_args

List of kernel command line arguments to bake into the zedboot image that are
required by this board. See also zedboot_cmdline_args in
//build/images/zedboot/BUILD.gn

**Current value (from the default):** `[]`

From //build/board.gni:58

### board_zedboot_fshost_config

A list of fshost options to add to the fshost config in the zedboot image.

**Current value (from the default):** `{ }`

From //build/board.gni:71

### bootfs_only

Put the "system image" package in the BOOTFS.  Hence what would
otherwise be /system/... at runtime is /boot/... instead.

**Current value for `target_cpu = "arm64"`:** `true`

From //products/common/bringup.gni:7

**Overridden from the default:** `false`

From //build/images/args.gni:14

**Current value for `target_cpu = "x64"`:** `true`

From //products/common/bringup.gni:7

**Overridden from the default:** `false`

From //build/images/args.gni:14

### bootstrap_files

List of files needed to bootstrap the device.

Flashing a device assumes a certain state; bootstrapping instead allows
initially provisioning a device from unknown state, so may require
additional resources that would not be included in an OTA.

Each entry in the list is a scope containing:
 * `path`: path to file.
 * `partition` (optional): `fastboot flash` partition.
 * `condition` (optional): a scope with `variable` and `value` keys; file is
   only flashed if `fastboot getvar <variable>` == <value>.

**Current value (from the default):** `[]`

From //build/images/args.gni:78

### buckeye_has_codec

**Current value (from the default):** `false`

From //src/devices/board/drivers/buckeye/BUILD.gn:9

### build_all_vp9_file_decoder_conformance_tests

**Current value (from the default):** `false`

From //src/media/codec/examples/BUILD.gn:11

### build_id_format

Build ID algorithm to use for Fuchsia-target code.  This does not apply
to host or guest code.  The value is the argument to the linker's
`--build-id=...` switch.  If left empty (the default), the linker's
default format is used.

**Current value (from the default):** `""`

From //build/config/build_id.gni:10

### build_info_board

Board configuration of the current build

**Current value for `target_cpu = "arm64"`:** `"arm64"`

From //out/not-default/args.gn:5

**Overridden from the default:** `"arm64"`

From //build/info/info.gni:12

**Current value for `target_cpu = "x64"`:** `"x64"`

From //out/not-default/args.gn:5

**Overridden from the default:** `"x64"`

From //build/info/info.gni:12

### build_info_product

Product configuration of the current build

**Current value for `target_cpu = "arm64"`:** `"bringup"`

From //out/not-default/args.gn:6

**Overridden from the default:** `""`

From //build/info/info.gni:9

**Current value for `target_cpu = "x64"`:** `"bringup"`

From //out/not-default/args.gn:6

**Overridden from the default:** `""`

From //build/info/info.gni:9

### build_info_version

Logical version of the current build. If not set, defaults to the timestamp
of the most recent update.

**Current value (from the default):** `""`

From //build/info/info.gni:16

### build_libvulkan_img_rgx

Targets that will be built as IMG vulkan ICDS.

**Current value (from the default):** `[]`

From //src/graphics/lib/magma/gnbuild/magma.gni:42

### build_libvulkan_vsi_vip

Targets that will be built as verisilicon vulkan ICDS.

**Current value (from the default):** `[]`

From //src/graphics/lib/magma/gnbuild/magma.gni:39

### build_sdk_archives

Whether to build SDK tarballs.

**Current value (from the default):** `false`

From //build/sdk/config.gni:7

### build_should_trace_actions

If enabled, all filesystem activity by actions will be traced and checked
against their declared inputs and outputs and depfiles (if present).
An action that accesses undeclared inputs or outputs will fail the build.

**Current value (from the default):** `false`

From //build/tracer/tracer.gni:12

### build_uefi_disk

Generate a UEFI disk image

**Current value for `target_cpu = "arm64"`:** `true`

From //boards/arm64.gni:33

**Overridden from the default:** `false`

From //build/images/args.gni:29

**Current value for `target_cpu = "x64"`:** `true`

From //boards/x64.gni:59

**Overridden from the default:** `false`

From //build/images/args.gni:29

### build_usb_installer

Generate installer disk image (ISO) to be flashed to a USB drive.
Will be located at obj/build/images/installer relative to the build directory.
See https://fuchsia.dev/fuchsia-src/development/hardware/installer

**Current value (from the default):** `false`

From //build/images/args.gni:34

### bump_api_level

If true, build for N+1 api level, where N is platform_version.in_development_api_level

**Current value (from the default):** `false`

From //build/config/fuchsia/platform_version.gni:8

### cache_package_labels

If you add package labels to this variable, the packages will be included
in the 'cache' package set, which represents an additional set of software
that is made available on disk immediately after paving and in factory
flows. These packages are updated with an OTA, and can also be updated
ephemerally. This cache of software can be evicted by the system if storage
pressure arises or other policies indicate.

**Current value for `target_cpu = "arm64"`:** `[]`

From //out/not-default/args.gn:16

**Overridden from the default:** `[]`

From //BUILD.gn:56

**Current value for `target_cpu = "x64"`:** `[]`

From //out/not-default/args.gn:16

**Overridden from the default:** `[]`

From //BUILD.gn:56

### camera_debug

**Current value (from the default):** `false`

From //src/camera/debug.gni:6

### camera_gym_configuration_cycle_interval_ms

**Current value (from the default):** `10000`

From //src/camera/bin/camera-gym/BUILD.gn:12

### camera_gym_enable_root_presenter

**Current value (from the default):** `false`

From //src/camera/bin/camera-gym/BUILD.gn:13

### carnelian_enable_vulkan_validation

Include the vulkan validation layers in carnelian examples.

**Current value (from the default):** `false`

From //src/lib/ui/carnelian/BUILD.gn:14

### carnelian_static_images_extras

Point this to the location of external image files to be included as extras

**Current value (from the default):** `[]`

From //src/lib/ui/carnelian/BUILD.gn:17

### carnelian_static_rives_extras

Point this to the location of external rive files to be included as extras

**Current value (from the default):** `[]`

From //src/lib/ui/carnelian/BUILD.gn:20

### carnelian_static_txts_extras

Point this to the location of external txt files to be included as extras

**Current value (from the default):** `[]`

From //src/lib/ui/carnelian/BUILD.gn:23

### check_output_dir_leaks

If enabled, check that the output dir path does not leak into
the command or any of its output files.  This is important for
remote build consistency and caching.

**Current value (from the default):** `true`

From //build/tracer/tracer.gni:21

### check_production_eligibility

Whether to perform check on the build's eligibility for production.
If true, base_packages and cache_packages are checked against dependencies
on //build/validate:non_production_tag, which is used to tag any
non-production GN labels. Build will fail if such dependency is found.

**Current value (from the default):** `false`

From //build/images/args.gni:136

### check_repeatability

If enabled, run each affected action twice (once with renamed outputs)
and compare the outputs' contents for reproducibility.

**Current value (from the default):** `false`

From //build/tracer/tracer.gni:16

### check_vtables_in_rodata

Check that all vtables in fuchsia binaries listed in binaries.json are in
readonly data sections. This check will be run at the end of a full build.

This is primarily meant to be used by the clang canary builders.

**Current value (from the default):** `false`

From //build/images/args.gni:94

### chromium_build_dir

This variable specifies a fully qualified Chromium build output directory,
such as `/home/$USER/chrome/src/out/fuchsia`, from which `chrome`,
`cast_runner`, `web_runner`, and `web_engine` will be obtained.
All of those targets must exist in the output directory.
If unset, the prebuilt packages from CIPD will be used.

**Current value (from the default):** `""`

From //src/chromium/build_args.gni:11

### clang_embed_bitcode

Embed LLVM bitcode as .llvmbc section in ELF files. This is intended
primarily for external tools that use bitcode for analysis.

**Current value (from the default):** `false`

From //build/config/clang/clang.gni:17

### clang_enable_error_reproducers

Enable reproducers on error. This provides crash-like reproducers on
compiler errors in addition to crashes.
Note, this flag should be used by very few people at the moment
because it depends on features that are not yet in prebuilt clang.
It is only useful for clang canary builders, and folks with a custom
clang.

**Current value (from the default):** `false`

From //build/config/clang/clang.gni:25

### clang_ml_inliner

Controls whether to use the ML inliner in Clang to reduce size.

**Current value (from the default):** `true`

From //build/config/clang/clang.gni:28

### clang_prefix

The default clang toolchain provided by the prebuilt. This variable is
additionally consumed by the Go toolchain.

**Current value (from the default):** `"//prebuilt/third_party/clang/linux-x64/bin"`

From //build/config/clang/clang.gni:13

### clang_tool_dir

Directory where the Clang toolchain binaries ("clang", "llvm-nm", etc.) are
found.  If this is "", then the behavior depends on $clang_prefix.
This toolchain is expected to support both Fuchsia targets and the host.

**Current value (from the default):** `""`

From //build/toolchain/zircon/clang.gni:11

### clippy_cause_failure

Makes clippy targets fail to build when any "deny" lints are found

**Current value (from the default):** `true`

From //build/rust/config.gni:61

### clippy_force_warn_all

Force the lint level for all clippy lints to "warn".
Note: this overrides both source attributes and our default lint levels, and
should only be used to collect stats about clippy lints in our source tree.

**Current value (from the default):** `false`

From //build/rust/config.gni:58

### clippy_warn_all

Set the lint level for all clippy lints to "warn".
Note: setting lint levels in source takes precedence over this.

**Current value (from the default):** `false`

From //build/rust/config.gni:53

### cobalt_environment

Selects the Cobalt environment to send data to. Choices:
  "LOCAL" - record log data locally to a file
  "DEVEL" - the non-prod environment for use in testing
  "PROD" - the production environment

**Current value (from the default):** `"PROD"`

From //src/cobalt/bin/app/BUILD.gn:15

### compress_blobs

Whether to compress the blobfs image.

**Current value (from the default):** `true`

From //build/images/args.gni:139

### config_example_cpp_greeting

Set this in args.gn to override the greeting emitted by this example.

**Current value (from the default):** `"World"`

From //examples/components/config/cpp/BUILD.gn:10

### config_example_rust_greeting

Set this in args.gn to override the greeting emitted by this example.

**Current value (from the default):** `"World"`

From //examples/components/config/rust/BUILD.gn:11

### config_have_heap

Tells openweave to include files that require heap access.

**Current value (from the default):** `true`

From //third_party/openweave-core/config.gni:32

### core_realm_shards

The following arguments are all used to configure the contents of the core
component realm. See //src/sys/core/build/core.gni for documentation on what
each field means.
TODO: redo comments

**Current value (from the default):** `[]`

From //build/product.gni:30

### crash_diagnostics_dir

Clang crash reports directory path. Use empty path to disable altogether.

**Current value (from the default):** `"//out/not-default/clang-crashreports"`

From //build/config/clang/crash_diagnostics.gni:7

### crashpad_dependencies

**Current value (from the default):** `"fuchsia"`

From //third_party/crashpad/src/build/crashpad_buildconfig.gni:22

### crashpad_http_transport_impl

**Current value (from the default):** `"socket"`

From //third_party/crashpad/src/util/net/tls.gni:19

### crashpad_use_boringssl_for_http_transport_socket

**Current value (from the default):** `true`

From //third_party/crashpad/src/util/net/tls.gni:30

### cts_version

The replacement of "99991231.0.1" changed the string, so this is in-tree.

**Current value (from the default):** `""`

From //sdk/ctf/build/internal/ctf_version.gni:18

### current_cpu

**Current value (from the default):** `""`

### current_os

**Current value (from the default):** `""`

### custom_signing_script

If non-empty, the given script will be invoked to produce a signed ZBI
image. The given script must accept -z for the input zbi path, and -o for
the output signed zbi path. The path must be in GN-label syntax (i.e.
starts with //).

**Current value (from the default):** `""`

From //build/images/custom_signing.gni:10

### custom_signing_script_deps

If `custom_signing_script` is not empty, a list of dependencies for the script.

**Current value (from the default):** `[]`

From //build/images/custom_signing.gni:13

### custom_signing_script_inputs

If `custom_signing_script` is not empty, these inputs will be listed in the
assembly target so that the hermeticity checker knows to expect these files
to be read.

**Current value (from the default):** `[]`

From //build/images/custom_signing.gni:18

### custom_signing_script_tools

If `custom signing script` is not empty, a list of host tool labels, without
a toolchain, that the script depends on. The reason why these are not in
`custom_signing_script_deps` is because these definitions are typically in
board-specific .gni files where `host_os` or `host_toolchain` are not
defined yet. Because these are imported from `args.gn` before `BUILDCONFIG.gn`
is actually parsed.

**Current value (from the default):** `[]`

From //build/images/custom_signing.gni:26

### custom_vulkan_loader_library_name

**Current value (from the default):** `""`

From //third_party/Vulkan-Loader/BUILD.gn:22

### cxx_rbe_enable

Set to true to enable distributed compilation of C++ using RBE.
Enabling this takes precedence over `use_goma`.

**Current value for `target_cpu = "arm64"`:** `false`

From //out/not-default/args.gn:7

**Overridden from the default:** `false`

From //build/toolchain/rbe.gni:137

**Current value for `target_cpu = "x64"`:** `false`

From //out/not-default/args.gn:7

**Overridden from the default:** `false`

From //build/toolchain/rbe.gni:137

### cxx_rbe_exec_strategy

One of:

  * "remote": Execute action remotely on cache miss.
        The remote cache is always updated with this result.

  * "local": Lookup action in the remote cache, but execute action
        locally on cache miss.  The locally produced result is
        not uploaded to the remote cache.

  * "remote_local_fallback": Execute action remotely first.
        If that fails, run locally instead.  The locally produced
        result are not uploaded to the remote cache.

  * "racing": Race local vs. remote execution, take the first to finish.

  (There are other rewrapper options that are not exposed.)

**Current value (from the default):** `"remote_local_fallback"`

From //build/toolchain/rbe.gni:155

### cxx_rbe_use_python_impl

EXPERIMENTAL.
Set to true to use the Python implementation for remote wrappers,
as opposed to the older (slower) bash scripts.  This option will
eventually disappear upon completing the transition to the Python scripts.

**Current value (from the default):** `false`

From //build/toolchain/rbe.gni:22

### dart_aot_debug_build_cfg

Builds the component in a non-product AOT build. This will
launch the vm service in the runner.
This configuration is not compatible with a --release build since the
profile aot runner is built without asserts.

**Current value (from the default):**

```none
{
  enable_asserts = true
  is_aot = true
  is_debug = true
  is_product = false
  platform_name = "dart_runner"
  runner_dep = "//src/dart:dart_aot"
  runtime_meta = "//build/dart/meta/aot_runtime.cml"
}
```

From //build/dart/dart_build_config.gni:22

### dart_debug_build_cfg

Builds the component in a non-product JIT build. This will
launch the vm service in the runner.

**Current value (from the default):**

```none
{
  enable_asserts = true
  is_aot = false
  is_debug = true
  is_product = false
  platform_name = "dart_runner"
  runner_dep = "//src/dart:dart_jit"
  runtime_meta = "//build/dart/meta/jit_runtime.cml"
}
```

From //build/dart/dart_build_config.gni:8

### dart_default_build_cfg

Non-product AOT

**Current value (from the default):**

```none
{
  enable_asserts = false
  is_aot = true
  is_debug = false
  is_product = false
  platform_name = "dart_runner"
  runner_dep = "//src/dart:dart_aot"
  runtime_meta = "//build/dart/meta/aot_runtime.cml"
}
```

From //build/dart/config.gni:20

### dart_force_aot

Forces all Dart apps to use an AOT runner regardless of whether the build is debug or release.

**Current value (from the default):** `false`

From //build/dart/args.gni:11

### dart_force_product

Forces all Dart apps to build in product mode which is a
stripped down version of the VM running in AOT mode.

**Current value (from the default):** `false`

From //build/dart/args.gni:8

### dart_profile_build_cfg

Builds the component in a non-product AOT build. This will
launch the vm service in the runner.

**Current value (from the default):**

```none
{
  enable_asserts = false
  is_aot = true
  is_debug = false
  is_product = false
  platform_name = "dart_runner"
  runner_dep = "//src/dart:dart_aot"
  runtime_meta = "//build/dart/meta/aot_runtime.cml"
}
```

From //build/dart/dart_build_config.gni:34

### dart_release_build_cfg

Builds the component in a product AOT build. This will
not launch the vm service in the runner.

**Current value (from the default):**

```none
{
  enable_asserts = false
  is_aot = true
  is_debug = false
  is_product = true
  platform_name = "dart_runner"
  runner_dep = "//src/dart:dart_aot_product"
  runtime_meta = "//build/dart/meta/aot_product_runtime.cml"
}
```

From //build/dart/dart_build_config.gni:46

### data_filesystem_format

Set to one of "minfs", "fxfs", "f2fs" (unstable).
If set to anything other than "minfs", any existing minfs partition will be
migrated in-place to the specified format when fshost mounts it.

**Current value (from the default):** `"fxfs"`

From //src/storage/fshost/generated_fshost_config.gni:32

### data_sharing_oobe_enabled

Whether or not to provide the data sharing consent step in OOBE.

**Current value (from the default):** `false`

From //src/experiences/session_shells/ermine/login/BUILD.gn:13

### debian_guest_earlycon

**Current value (from the default):** `false`

From //src/virtualization/packages/debian_guest/BUILD.gn:12

### debian_guest_qcow

Package the rootfs as a QCOW image (as opposed to a flat file).

**Current value (from the default):** `true`

From //src/virtualization/packages/debian_guest/BUILD.gn:11

### debuginfo

* `none` means no debugging information
* `backtrace` means sufficient debugging information to symbolize backtraces
* `debug` means debugging information suited for debugging

**Current value (from the default):** `"debug"`

From //build/config/compiler.gni:52

### default_configs

Default configs and dependencies targets provided by the toolchain. These
are applied to all of the pw_* target types. They are set from a toolchain's
toolchain_args for cross-toolchain deps, e.g. for

  `deps = [ //pw_some_module(//pw_toolchain:not_default) ]`

The default toolchain is never used.

**Current value (from the default):** `[]`

From //third_party/pigweed/src/pw_build/defaults.gni:25

### default_public_deps

**Current value (from the default):** `[]`

From //third_party/pigweed/src/pw_build/defaults.gni:26

### delivery_blob_type

Controls what type of delivery blob blobfs accepts.
Supported types can be found in //src/storage/blobfs/delivery_blob.h
Valid values are integers, for example: 1
By default no delivery blobs are accepted.
This arg is for local developer only, products should not set this arg.

**Current value (from the default):** `false`

From //build/images/args.gni:149

### deny_warnings

Controls whether to promote warnings to errors.

**Current value (from the default):** `true`

From //build/config/BUILD.gn:35

### deprecated_x86_legacy_boot_protocol

**TODO(fxbug.dev/32255): This is a temporary switch that will be removed.**

Set this to make the ZBI compatible with older boot loaders such as a
gigaboot or zedboot image already installed on a machine that's hard to
update.  This is an interim workaround only for people who have machines
that are not physically accessible to update their boot images, and will
be removed after everyone has had a chance to get hold of their machines.

**Current value (from the default):** `false`

From //zircon/kernel/BUILD.gn:32

### dev_bootfs_labels

List of labels for objects to include in the ZBI.

**Current value (from the default):** `[]`

From //build/dev.gni:17

### dev_build_only_deps

List of labels for targets that should be built but not included in any
build outputs that are part of the build API (e.g. zbi's, package servers).

**Current value (from the default):** `[]`

From //build/dev.gni:14

### dev_kernel_cmdline

List of strings to append to the kernel command line.

**Current value (from the default):** `[]`

From //build/dev.gni:26

### dev_recovery_bootfs_labels

List of binary labels to include in the recovery ZBI.

**Current value (from the default):** `[]`

From //build/dev.gni:23

### dev_recovery_kernel_cmdline

List of strings to append to the recovery kernel command line.

**Current value (from the default):** `[]`

From //build/dev.gni:32

### dev_system_image_deps

List of labels for binaries to include in the system image.

**Current value (from the default):** `[]`

From //build/dev.gni:10

### dev_zedboot_bootfs_labels

List of binary labels to include in the zedboot ZBI.

**Current value (from the default):** `[]`

From //build/dev.gni:20

### dev_zedboot_kernel_cmdline

List of strings to append to the zedboot kernel command line.

**Current value (from the default):** `[]`

From //build/dev.gni:29

### devmgr_config

List of arguments to add to /boot/config/devmgr.
These come after synthesized arguments to configure blobfs and pkgfs.

**Current value (from the default):** `[]`

From //build/images/args.gni:23

### dir_docker

**Current value (from the default):** `"//third_party/pigweed/src/docker"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:30

### dir_pigweed

Location of the Pigweed repository.

**Current value (from the default):** `"//third_party/pigweed/src"`

From //build_overrides/pigweed.gni:11

### dir_pw_alignment

**Current value (from the default):** `"//third_party/pigweed/src/pw_alignment"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:31

### dir_pw_allocator

**Current value (from the default):** `"//third_party/pigweed/src/pw_allocator"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:32

### dir_pw_analog

**Current value (from the default):** `"//third_party/pigweed/src/pw_analog"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:33

### dir_pw_android_toolchain

**Current value (from the default):** `"//third_party/pigweed/src/pw_android_toolchain"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:34

### dir_pw_arduino_build

**Current value (from the default):** `"//third_party/pigweed/src/pw_arduino_build"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:35

### dir_pw_assert

**Current value (from the default):** `"//third_party/pigweed/src/pw_assert"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:36

### dir_pw_assert_basic

**Current value (from the default):** `"//third_party/pigweed/src/pw_assert_basic"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:37

### dir_pw_assert_log

**Current value (from the default):** `"//third_party/pigweed/src/pw_assert_log"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:38

### dir_pw_assert_tokenized

**Current value (from the default):** `"//third_party/pigweed/src/pw_assert_tokenized"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:39

### dir_pw_assert_zephyr

**Current value (from the default):** `"//third_party/pigweed/src/pw_assert_zephyr"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:40

### dir_pw_async

**Current value (from the default):** `"//third_party/pigweed/src/pw_async"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:41

### dir_pw_async_basic

**Current value (from the default):** `"//third_party/pigweed/src/pw_async_basic"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:42

### dir_pw_base64

**Current value (from the default):** `"//third_party/pigweed/src/pw_base64"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:43

### dir_pw_bloat

**Current value (from the default):** `"//third_party/pigweed/src/pw_bloat"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:44

### dir_pw_blob_store

**Current value (from the default):** `"//third_party/pigweed/src/pw_blob_store"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:45

### dir_pw_bluetooth

**Current value (from the default):** `"//third_party/pigweed/src/pw_bluetooth"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:46

### dir_pw_bluetooth_hci

**Current value (from the default):** `"//third_party/pigweed/src/pw_bluetooth_hci"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:47

### dir_pw_bluetooth_profiles

**Current value (from the default):** `"//third_party/pigweed/src/pw_bluetooth_profiles"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:49

### dir_pw_boot

**Current value (from the default):** `"//third_party/pigweed/src/pw_boot"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:50

### dir_pw_boot_cortex_m

**Current value (from the default):** `"//third_party/pigweed/src/pw_boot_cortex_m"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:51

### dir_pw_build

**Current value (from the default):** `"//third_party/pigweed/src/pw_build"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:52

### dir_pw_build_info

**Current value (from the default):** `"//third_party/pigweed/src/pw_build_info"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:53

### dir_pw_build_mcuxpresso

**Current value (from the default):** `"//third_party/pigweed/src/pw_build_mcuxpresso"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:54

### dir_pw_bytes

**Current value (from the default):** `"//third_party/pigweed/src/pw_bytes"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:55

### dir_pw_checksum

**Current value (from the default):** `"//third_party/pigweed/src/pw_checksum"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:56

### dir_pw_chrono

**Current value (from the default):** `"//third_party/pigweed/src/pw_chrono"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:57

### dir_pw_chrono_embos

**Current value (from the default):** `"//third_party/pigweed/src/pw_chrono_embos"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:58

### dir_pw_chrono_freertos

**Current value (from the default):** `"//third_party/pigweed/src/pw_chrono_freertos"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:59

### dir_pw_chrono_stl

**Current value (from the default):** `"//third_party/pigweed/src/pw_chrono_stl"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:60

### dir_pw_chrono_threadx

**Current value (from the default):** `"//third_party/pigweed/src/pw_chrono_threadx"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:61

### dir_pw_chrono_zephyr

**Current value (from the default):** `"//third_party/pigweed/src/pw_chrono_zephyr"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:62

### dir_pw_cli

**Current value (from the default):** `"//third_party/pigweed/src/pw_cli"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:63

### dir_pw_compilation_testing

**Current value (from the default):** `"//third_party/pigweed/src/pw_compilation_testing"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:65

### dir_pw_console

**Current value (from the default):** `"//third_party/pigweed/src/pw_console"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:66

### dir_pw_containers

**Current value (from the default):** `"//third_party/pigweed/src/pw_containers"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:67

### dir_pw_cpu_exception

**Current value (from the default):** `"//third_party/pigweed/src/pw_cpu_exception"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:68

### dir_pw_cpu_exception_cortex_m

**Current value (from the default):** `"//third_party/pigweed/src/pw_cpu_exception_cortex_m"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:70

### dir_pw_crypto

**Current value (from the default):** `"//third_party/pigweed/src/pw_crypto"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:71

### dir_pw_digital_io

**Current value (from the default):** `"//third_party/pigweed/src/pw_digital_io"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:72

### dir_pw_docgen

**Current value (from the default):** `"//third_party/pigweed/src/pw_docgen"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:73

### dir_pw_doctor

**Current value (from the default):** `"//third_party/pigweed/src/pw_doctor"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:74

### dir_pw_env_setup

**Current value (from the default):** `"//third_party/pigweed/src/pw_env_setup"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:75

### dir_pw_file

**Current value (from the default):** `"//third_party/pigweed/src/pw_file"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:76

### dir_pw_function

**Current value (from the default):** `"//third_party/pigweed/src/pw_function"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:77

### dir_pw_fuzzer

**Current value (from the default):** `"//third_party/pigweed/src/pw_fuzzer"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:78

### dir_pw_hdlc

**Current value (from the default):** `"//third_party/pigweed/src/pw_hdlc"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:79

### dir_pw_hex_dump

**Current value (from the default):** `"//third_party/pigweed/src/pw_hex_dump"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:80

### dir_pw_i2c

**Current value (from the default):** `"//third_party/pigweed/src/pw_i2c"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:81

### dir_pw_i2c_mcuxpresso

**Current value (from the default):** `"//third_party/pigweed/src/pw_i2c_mcuxpresso"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:82

### dir_pw_ide

**Current value (from the default):** `"//third_party/pigweed/src/pw_ide"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:83

### dir_pw_interrupt

**Current value (from the default):** `"//third_party/pigweed/src/pw_interrupt"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:84

### dir_pw_interrupt_cortex_m

**Current value (from the default):** `"//third_party/pigweed/src/pw_interrupt_cortex_m"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:86

### dir_pw_interrupt_zephyr

**Current value (from the default):** `"//third_party/pigweed/src/pw_interrupt_zephyr"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:87

### dir_pw_intrusive_ptr

**Current value (from the default):** `"//third_party/pigweed/src/pw_intrusive_ptr"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:88

### dir_pw_kvs

**Current value (from the default):** `"//third_party/pigweed/src/pw_kvs"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:89

### dir_pw_libc

**Current value (from the default):** `"//third_party/pigweed/src/pw_libc"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:90

### dir_pw_log

**Current value (from the default):** `"//third_party/pigweed/src/pw_log"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:91

### dir_pw_log_android

**Current value (from the default):** `"//third_party/pigweed/src/pw_log_android"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:92

### dir_pw_log_basic

**Current value (from the default):** `"//third_party/pigweed/src/pw_log_basic"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:93

### dir_pw_log_null

**Current value (from the default):** `"//third_party/pigweed/src/pw_log_null"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:94

### dir_pw_log_rpc

**Current value (from the default):** `"//third_party/pigweed/src/pw_log_rpc"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:95

### dir_pw_log_string

**Current value (from the default):** `"//third_party/pigweed/src/pw_log_string"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:96

### dir_pw_log_tokenized

**Current value (from the default):** `"//third_party/pigweed/src/pw_log_tokenized"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:97

### dir_pw_log_zephyr

**Current value (from the default):** `"//third_party/pigweed/src/pw_log_zephyr"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:98

### dir_pw_malloc

**Current value (from the default):** `"//third_party/pigweed/src/pw_malloc"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:99

### dir_pw_malloc_freelist

**Current value (from the default):** `"//third_party/pigweed/src/pw_malloc_freelist"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:100

### dir_pw_metric

**Current value (from the default):** `"//third_party/pigweed/src/pw_metric"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:101

### dir_pw_minimal_cpp_stdlib

**Current value (from the default):** `"//third_party/pigweed/src/pw_minimal_cpp_stdlib"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:103

### dir_pw_module

**Current value (from the default):** `"//third_party/pigweed/src/pw_module"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:104

### dir_pw_multisink

**Current value (from the default):** `"//third_party/pigweed/src/pw_multisink"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:105

### dir_pw_package

**Current value (from the default):** `"//third_party/pigweed/src/pw_package"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:106

### dir_pw_perf_test

**Current value (from the default):** `"//third_party/pigweed/src/pw_perf_test"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:107

### dir_pw_persistent_ram

**Current value (from the default):** `"//third_party/pigweed/src/pw_persistent_ram"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:108

### dir_pw_polyfill

**Current value (from the default):** `"//third_party/pigweed/src/pw_polyfill"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:109

### dir_pw_preprocessor

**Current value (from the default):** `"//third_party/pigweed/src/pw_preprocessor"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:110

### dir_pw_presubmit

**Current value (from the default):** `"//third_party/pigweed/src/pw_presubmit"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:111

### dir_pw_protobuf

**Current value (from the default):** `"//third_party/pigweed/src/pw_protobuf"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:112

### dir_pw_protobuf_compiler

**Current value (from the default):** `"//third_party/pigweed/src/pw_protobuf_compiler"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:113

### dir_pw_random

**Current value (from the default):** `"//third_party/pigweed/src/pw_random"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:114

### dir_pw_result

**Current value (from the default):** `"//third_party/pigweed/src/pw_result"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:115

### dir_pw_ring_buffer

**Current value (from the default):** `"//third_party/pigweed/src/pw_ring_buffer"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:116

### dir_pw_router

**Current value (from the default):** `"//third_party/pigweed/src/pw_router"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:117

### dir_pw_rpc

**Current value (from the default):** `"//third_party/pigweed/src/pw_rpc"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:118

### dir_pw_rust

**Current value (from the default):** `"//third_party/pigweed/src/pw_rust"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:119

### dir_pw_snapshot

**Current value (from the default):** `"//third_party/pigweed/src/pw_snapshot"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:120

### dir_pw_software_update

**Current value (from the default):** `"//third_party/pigweed/src/pw_software_update"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:121

### dir_pw_span

**Current value (from the default):** `"//third_party/pigweed/src/pw_span"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:122

### dir_pw_spi

**Current value (from the default):** `"//third_party/pigweed/src/pw_spi"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:123

### dir_pw_status

**Current value (from the default):** `"//third_party/pigweed/src/pw_status"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:124

### dir_pw_stm32cube_build

**Current value (from the default):** `"//third_party/pigweed/src/pw_stm32cube_build"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:125

### dir_pw_stream

**Current value (from the default):** `"//third_party/pigweed/src/pw_stream"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:126

### dir_pw_string

**Current value (from the default):** `"//third_party/pigweed/src/pw_string"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:127

### dir_pw_symbolizer

**Current value (from the default):** `"//third_party/pigweed/src/pw_symbolizer"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:128

### dir_pw_sync

**Current value (from the default):** `"//third_party/pigweed/src/pw_sync"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:129

### dir_pw_sync_baremetal

**Current value (from the default):** `"//third_party/pigweed/src/pw_sync_baremetal"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:130

### dir_pw_sync_embos

**Current value (from the default):** `"//third_party/pigweed/src/pw_sync_embos"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:131

### dir_pw_sync_freertos

**Current value (from the default):** `"//third_party/pigweed/src/pw_sync_freertos"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:132

### dir_pw_sync_stl

**Current value (from the default):** `"//third_party/pigweed/src/pw_sync_stl"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:133

### dir_pw_sync_threadx

**Current value (from the default):** `"//third_party/pigweed/src/pw_sync_threadx"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:134

### dir_pw_sync_zephyr

**Current value (from the default):** `"//third_party/pigweed/src/pw_sync_zephyr"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:135

### dir_pw_sys_io

**Current value (from the default):** `"//third_party/pigweed/src/pw_sys_io"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:136

### dir_pw_sys_io_arduino

**Current value (from the default):** `"//third_party/pigweed/src/pw_sys_io_arduino"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:137

### dir_pw_sys_io_baremetal_lm3s6965evb

**Current value (from the default):** `"//third_party/pigweed/src/pw_sys_io_baremetal_lm3s6965evb"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:139

### dir_pw_sys_io_baremetal_stm32f429

**Current value (from the default):** `"//third_party/pigweed/src/pw_sys_io_baremetal_stm32f429"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:141

### dir_pw_sys_io_emcraft_sf2

**Current value (from the default):** `"//third_party/pigweed/src/pw_sys_io_emcraft_sf2"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:143

### dir_pw_sys_io_mcuxpresso

**Current value (from the default):** `"//third_party/pigweed/src/pw_sys_io_mcuxpresso"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:144

### dir_pw_sys_io_pico

**Current value (from the default):** `"//third_party/pigweed/src/pw_sys_io_pico"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:145

### dir_pw_sys_io_stdio

**Current value (from the default):** `"//third_party/pigweed/src/pw_sys_io_stdio"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:146

### dir_pw_sys_io_stm32cube

**Current value (from the default):** `"//third_party/pigweed/src/pw_sys_io_stm32cube"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:147

### dir_pw_sys_io_zephyr

**Current value (from the default):** `"//third_party/pigweed/src/pw_sys_io_zephyr"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:148

### dir_pw_system

**Current value (from the default):** `"//third_party/pigweed/src/pw_system"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:149

### dir_pw_target_runner

**Current value (from the default):** `"//third_party/pigweed/src/pw_target_runner"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:150

### dir_pw_third_party

**Current value (from the default):** `"//third_party/pigweed/src/third_party"`

From //third_party/pigweed/src/modules.gni:25

### dir_pw_third_party_boringssl

If compiling backends with boringssl, this variable is set to the path to the
boringssl source code. When set, a pw_source_set for the boringssl library is
created at "$dir_pw_third_party/boringssl".

**Current value (from the default):** `""`

From //third_party/pigweed/src/third_party/boringssl/boringssl.gni:19

### dir_pw_third_party_emboss

If compiling with Emboss, this variable is set to the path to the Emboss
source code.

**Current value for `target_cpu = "arm64"`:** `"//third_party/github.com/google/emboss/src"`

From //.gn:83

**Overridden from the default:** `""`

From //third_party/pigweed/src/third_party/emboss/emboss.gni:20

**Current value for `target_cpu = "x64"`:** `"//third_party/github.com/google/emboss/src"`

From //.gn:83

**Overridden from the default:** `""`

From //third_party/pigweed/src/third_party/emboss/emboss.gni:20

### dir_pw_third_party_googletest

If compiling tests with googletest, this variable is set to the path to the
googletest installation. When set, a pw_source_set for the googletest
library is created at "$dir_pw_third_party/googletest".

**Current value (from the default):** `""`

From //third_party/pigweed/src/third_party/googletest/googletest.gni:19

### dir_pw_third_party_nanopb

If compiling protos for nanopb, this variable is set to the path to the
nanopb installation. When set, a pw_source_set for the nanopb library is
created at "$dir_pw_third_party/nanopb".

**Current value (from the default):** `""`

From //third_party/pigweed/src/third_party/nanopb/nanopb.gni:22

### dir_pw_third_party_protobuf

If compiling host tools that use libprotobuf, this variable is set to the
path to the protobuf installation. When set, a pw_source_set for the
protobuf library is created at "$dir_pw_third_party/protobuf".

**Current value (from the default):** `""`

From //third_party/pigweed/src/third_party/protobuf/protobuf.gni:19

### dir_pw_thread

**Current value (from the default):** `"//third_party/pigweed/src/pw_thread"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:151

### dir_pw_thread_embos

**Current value (from the default):** `"//third_party/pigweed/src/pw_thread_embos"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:152

### dir_pw_thread_freertos

**Current value (from the default):** `"//third_party/pigweed/src/pw_thread_freertos"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:153

### dir_pw_thread_stl

**Current value (from the default):** `"//third_party/pigweed/src/pw_thread_stl"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:154

### dir_pw_thread_threadx

**Current value (from the default):** `"//third_party/pigweed/src/pw_thread_threadx"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:155

### dir_pw_thread_zephyr

**Current value (from the default):** `"//third_party/pigweed/src/pw_thread_zephyr"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:156

### dir_pw_tls_client

**Current value (from the default):** `"//third_party/pigweed/src/pw_tls_client"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:157

### dir_pw_tls_client_boringssl

**Current value (from the default):** `"//third_party/pigweed/src/pw_tls_client_boringssl"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:159

### dir_pw_tls_client_mbedtls

**Current value (from the default):** `"//third_party/pigweed/src/pw_tls_client_mbedtls"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:161

### dir_pw_tokenizer

**Current value (from the default):** `"//third_party/pigweed/src/pw_tokenizer"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:162

### dir_pw_tool

**Current value (from the default):** `"//third_party/pigweed/src/pw_tool"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:163

### dir_pw_toolchain

**Current value (from the default):** `"//third_party/pigweed/src/pw_toolchain"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:164

### dir_pw_trace

**Current value (from the default):** `"//third_party/pigweed/src/pw_trace"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:165

### dir_pw_trace_tokenized

**Current value (from the default):** `"//third_party/pigweed/src/pw_trace_tokenized"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:166

### dir_pw_transfer

**Current value (from the default):** `"//third_party/pigweed/src/pw_transfer"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:167

### dir_pw_unit_test

**Current value (from the default):** `"//third_party/pigweed/src/pw_unit_test"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:168

### dir_pw_varint

**Current value (from the default):** `"//third_party/pigweed/src/pw_varint"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:169

### dir_pw_watch

**Current value (from the default):** `"//third_party/pigweed/src/pw_watch"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:170

### dir_pw_web

**Current value (from the default):** `"//third_party/pigweed/src/pw_web"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:171

### dir_pw_work_queue

**Current value (from the default):** `"//third_party/pigweed/src/pw_work_queue"`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:172

### disable_dart_strict_deps

Enable all strict deps.

**Current value (from the default):** `false`

From //build/dart/dart_library.gni:18

### disable_kernel_pci

Disable kernel PCI driver support. A counterpart of the the build
flag platform_enable_user_pci in //src/devices/bus/drivers/pci/pci.gni.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:81

### dont_profile_source_files

List of GN paths to source files to NOT instrument by `profile` variants.
These take precedence over `profile_source_files`.

**Current value (from the default):** `[]`

From //build/config/profile/config.gni:15

### dwarf_version

Explicitly specify DWARF version used.

**Current value (from the default):** `5`

From //build/config/compiler.gni:66

### emu_window_size_height

**Current value (from the default):** `false`

From //build/product.gni:48

### emu_window_size_width

Configuration to override the default window size for the virtual device in pixels.

**Current value (from the default):** `false`

From //build/product.gni:47

### enable_api_diff

Detect dart API changes

**Current value (from the default):** `true`

From //build/dart/dart_library.gni:21

### enable_dart_analysis

Enable all dart analysis
TODO(fxbug.dev/98703) reenable analysis when hangs are fixed.

**Current value (from the default):** `false`

From //build/dart/dart_library.gni:15

### enable_frame_pointers

Controls whether the compiler emits full stack frames for function calls.
This reduces performance but increases the ability to generate good
stack traces, especially when we have bugs around unwind table generation.
It applies only for Fuchsia targets (see below where it is unset).

TODO(fxbug.dev/32216): Theoretically unwind tables should be good enough so we can
remove this option when the issues are addressed.

**Current value (from the default):** `false`

From //build/config/BUILD.gn:31

### enable_grpc_ares

Compiles with ares.

**Current value (from the default):** `false`

From //third_party/grpc/BUILD.gn:13

### enable_lock_dep

Enable kernel lock dependency tracking.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:40

### enable_lock_dep_tests

Enable kernel lock dependency tracking tests.  By default this is
enabled when tracking is enabled, but can also be eanbled independently
to assess whether the tests build and *fail correctly* when lockdep is
disabled.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:153

### enable_mdns_trace

Enables the tracing feature of mdns, which can be turned on using
"mdns-util verbose".

**Current value (from the default):** `false`

From //src/connectivity/network/mdns/service/BUILD.gn:13

### enable_netboot

Whether to build the netboot zbi by default.

You can still build //build/images:netboot explicitly even if enable_netboot is false.

**Current value (from the default):** `false`

From //build/images/args.gni:88

### enable_netstack2_tracing

TODO(https://fxbug.dev/122670): If we make tracing in netstack3
conditionally compiled, make this flag generic over netstack and choose the
appropriate netstack package based on the combination of netstack3
(true/false) and tracing (true/false).

**Current value (from the default):** `false`

From //src/connectivity/network/BUILD.gn:14

### enable_perfetto_benchmarks

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:195

### enable_perfetto_fuzzers

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:198

### enable_perfetto_grpc

Enables gRPC in the Perfetto codebase. gRPC significantly increases build
times and the general footprint of Perfetto. As it only required for
cloud trace processor and even then only to build the final ready-to-ship
binary, don't enable this by default.

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:315

### enable_perfetto_heapprofd

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:157

### enable_perfetto_integration_tests

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:193

### enable_perfetto_ipc

**Current value (from the default):** `true`

From //third_party/perfetto/gn/perfetto.gni:150

### enable_perfetto_llvm_demangle

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:309

### enable_perfetto_platform_services

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:141

### enable_perfetto_site

Allows to build the perfetto.dev website.
WARNING: if this flag is enabled, the build performs globbing at generation
time. Incremental builds that add/remove files will not be supported without
rerunning gn.

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:332

### enable_perfetto_stderr_crash_dump

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:235

### enable_perfetto_system_consumer

**Current value (from the default):** `true`

From //third_party/perfetto/gn/perfetto.gni:251

### enable_perfetto_tools

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:187

### enable_perfetto_trace_processor

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:173

### enable_perfetto_trace_processor_httpd

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:296

### enable_perfetto_trace_processor_json

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:291

### enable_perfetto_trace_processor_linenoise

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:285

### enable_perfetto_trace_processor_percentile

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:280

### enable_perfetto_trace_processor_sqlite

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:275

### enable_perfetto_traceconv

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:321

### enable_perfetto_traced_perf

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:165

### enable_perfetto_traced_probes

The traced_probes daemon is very Linux-specific, as it depends on ftrace and
various /proc interfaces. There is no point making its code platform-neutral
as it won't do anything useful on Windows.
The only reason why we still build it on Mac OS is to be able to run the
unittests there and making dev on mac less cumbersome. The traced_probes
code happens to build cleanly and for now the mainteinance cost on Mac is
extremely low.

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:267

### enable_perfetto_ui

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:325

### enable_perfetto_unittests

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:189

### enable_perfetto_version_gen

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:204

### enable_perfetto_watchdog

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:182

### enable_perfetto_x64_cpu_opt

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:243

### enable_perfetto_zlib

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:302

### enable_power_manager_debug

**Current value (from the default):** `false`

From //src/power/power-manager/BUILD.gn:145

### enable_recovery_ui_v2

Whether to use the new UX UI

**Current value (from the default):** `true`

From //src/recovery/system/system_recovery_args.gni:14

### enable_recovery_ui_v2_debug_logging

Whether to log lots of debug info including PII

**Current value (from the default):** `false`

From //src/recovery/system/system_recovery_args.gni:17

### enable_suspend

While suspend is being developed we define this flag which enables all
suspend features. Developers working on suspend can then simply add this
one flag to their build arguments.

**Current value (from the default):** `false`

From //build/suspend/config.gni:9

### enable_virtual_heap

Enables the use of a virtually managed kernel heap instead of one managed
directly out of the physmap. The virtual heap may have some performance and
memory usage overheads, but will not exhaust due to fragmentation.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:105

### ermine_app_entries

Build arg that allows overriding the default set of application entries
using '--args=ermine_app_entries="config/app_launch_entries.json"'

**Current value (from the default):** `"config/app_launch_entries.json"`

From //src/experiences/session_shells/ermine/shell/BUILD.gn:16

### ermine_manifest

Build arg that allows overriding the default manifest for Ermine.
TODO(fxbug.dev/120106): Remove this argument when we have a better way of
overriding the manifest.

**Current value (from the default):** `"meta/ermine.cml"`

From //src/experiences/session_shells/ermine/shell/BUILD.gn:21

### ermine_start_screensaver

Whether or not to launch screensaver.

**Current value (from the default):** `false`

From //src/experiences/session_shells/ermine/shell/BUILD.gn:24

### ermine_user_feedback_enabled

Whether or not to allow user feedback report from the device.

**Current value (from the default):** `false`

From //src/experiences/session_shells/ermine/shell/BUILD.gn:27

### escher_test_for_glsl_spirv_mismatch

If true, this enables the |SpirvNotChangedTest| to check if the precompiled
shaders on disk are up to date and reflect the current shader source code
compiled with the latest shaderc tools/optimizations. People on the Scenic
team should build with this flag turned on to make sure that any shader
changes that were not run through the precompiler have their updated spirv
written to disk. Other teams and CQ do not need to worry about this flag.

**Current value (from the default):** `false`

From //src/ui/lib/escher/build_args.gni:18

### escher_use_runtime_glsl

Determines whether or not escher will build with the glslang and shaderc
libraries. When false, these libraries will not be included in the scenic/
escher binary and as a result shaders will not be able to be compiled at
runtime. Precompiled spirv code will be loaded into memory from disk instead.

**Current value (from the default):** `false`

From //src/ui/lib/escher/build_args.gni:10

### exclude_testonly_syscalls

If true, excludes syscalls with the [testonly] attribute.

**Current value (from the default):** `false`

From //zircon/vdso/vdso.gni:9

### expat_build_root

**Current value (from the default):** `"//third_party/expat"`

From //src/graphics/lib/magma/gnbuild/magma.gni:14

### experimental_cxx_version

**NOTE:** This is for **experimentation only** and should not normally be
changed.  Set the version of the C++ standard to compile for, 17 or 20.
Note also that GN code should never use this variable directly, but always
instead use the `fuchsia_cxx_version` variable.

**Current value (from the default):** `17`

From //build/config/fuchsia_cxx_version.gni:10

### experimental_wlan_client_mlme

Selects the SoftMAC client implementation to use. Choices:
  false (default) - C++ Client MLME implementation
  true - Rust Client MLME implementation
This argument is temporary until Rust MLME is ready to be used.

**Current value (from the default):** `false`

From //src/connectivity/wlan/lib/mlme/cpp/BUILD.gn:12

### extra_gn_labels_for_bazel_inputs

A list of extra labels to bazel_input_xxx() targets that complement
`gn_labels_for_bazel_inputs`. These labels can be defined anywhere,
including the //vendor/... directory.

This can be set in args.gn by vendor-specific build configurations.
Consider the following example from a fictitious
//vendor/acme/proprietary/BUILD.gn file:

     # Generate the firmware for our device.
     action("generate_firmware") {
       ...
     }

     # Ensure the generated firmware is visible to Bazel as a filegroup()
     # @legacy_ninja_build_outputs repository//:acme_firmware
     bazel_input_resource("acme_firmware") {
       deps = [ ":generate_firmware" ]
       sources = get_target_outputs(deps[0])
       outputs = [ "{{source_file_part}}" ]
       visibility = [ ":*" ]
     }

     # Build the installer with Bazel.
     bazel_action("build_installer") {
       command = "build"
       bazel_targets = "//vendor/acme/proprietary/installer"
       bazel_inputs = [ ":acme_firmware" ]
       copy_outputs = [
         {
           bazel = "vendor/acme/proprietary/installer/installer"
           ninja = "installer"
         }
       ]
     }

Which requires the following, which could be in args.gn, or in a file
imported from it (e.g. //vendor/acme/products/device.gni):

   extra_gn_labels_for_bazel_inputs = [
     "//vendor/acme/proprietary:acme_firmware"
   ]

In order to ensure that the @legacy_ninja_build_outputs//:acme_firmware
filegroup() will be defined and properly generated before building
vendor/acme/proprietary:build_installer with Ninja:


**Current value (from the default):** `[]`

From //build/bazel/legacy_ninja_build_outputs.gni:106

### extra_package_labels

**Current value (from the default):** `[]`

From //third_party/cobalt/BUILD.gn:10

### extra_variants

Additional variant toolchain configs to support.
This is just added to [`known_variants`](#known_variants).

**Current value (from the default):** `[]`

From //build/config/BUILDCONFIG.gn:1597

### extra_vbmeta_descriptors

Additional VBMeta Descriptors to add to the vbmeta image during assembly.

To add a descriptor, include GN code such as:

```
extra_vbmeta_descriptors = [{
  name = "zircon"         # name of the partition
  size = 12345            # size of the partition in bytes
  flags = 1               # custom vbmeta flags to add
  min_avb_version = "1.1" # minimum avb version
}]
```

**Current value (from the default):** `false`

From //build/images/vbmeta.gni:37

### fastboot_product

**Current value (from the default):** `""`

From //build/images/args.gni:119

### ffmpeg_profile

**Current value (from the default):** `"default"`

From //src/media/lib/ffmpeg/BUILD.gn:55

### ffx_build_dual_mode_plugins_as_subtools

Build any ffx plugins that can be built either as built-in or as separate
subtools as subtools.

Note that if you change this and don't `fx clean` you may wind up with stale
copies of either the main `ffx` binary (with all the plugins built in) or
the separately compiled ones, and that might produce confusing `ffx help`
or `ffx commands` output.

When all subtools that will be migrated to the SDK have been migrated,
this config flag will be set to true by default, deprecated, and eventually
removed: fxb/117339

**Current value (from the default):** `false`

From //src/developer/ffx/config.gni:17

### fidl_trace_level

0 = Disable FIDL userspace tracing (default).
1 = Enable FIDL userspace tracing.

**Current value (from the default):** `0`

From //build/fidl/args.gni:8

### firmware_prebuilts

List of prebuilt firmware blobs to include in update packages.

Each entry in the list is a scope containing:
 * `path`: path to the image (see also `firmware_prebuilts_path_suffix`)
 * `type`: firmware type, a device-specific unique identifier
 * `partition` (optional): if specified, the `fastboot flash` partition

**Current value (from the default):** `[]`

From //build/images/args.gni:59

### firmware_prebuilts_path_suffix

Suffix to append to all `firmware_prebuilts` `path` variables.

Typically this indicates the hardware revision, and is made available so
that users can easily switch revisions using a single arg.

**Current value (from the default):** `""`

From //build/images/args.gni:65

### flatland_enable_display_composition

If true, this enables the compositor to attempt to use the display controller rendering
path instead of always using the GPU/Vulkan rendering path.

**Current value (from the default):** `false`

From //src/ui/scenic/lib/flatland/engine/build_args.gni:14

### flatland_verbose_logging

If true, Flatland will log an excruciating amount of data.  For debugging.

**Current value (from the default):** `false`

From //src/ui/scenic/lib/utils/build_args.gni:7

### flatland_visual_debugging

If true, this enables the display compositor to tint all Flatland views that
are rendered via the GPU renderer instead of by direct DisplayController scanout.

**Current value (from the default):** `false`

From //src/ui/scenic/lib/flatland/engine/build_args.gni:8

### flutter_aot_debug_build_cfg

Builds the component in a non-product AOT build. This will
launch the vm service in the runner.
This configuration is not compatible with a --release build since the
profile aot runner is built without asserts.

**Current value (from the default):**

```none
{
  enable_asserts = true
  is_aot = true
  is_debug = true
  is_product = false
  platform_name = "flutter_runner"
  runner_dep = "//src/flutter:flutter_aot_runner"
  runtime_meta = "//build/flutter/meta/aot_runtime.cml"
}
```

From //build/flutter/flutter_build_config.gni:26

### flutter_debug_build_cfg

Builds the component in a non-product JIT build. This will
launch the vm service in the runner.

**Current value (from the default):**

```none
{
  enable_asserts = true
  is_aot = false
  is_debug = true
  is_product = false
  platform_name = "flutter_runner"
  runner_dep = "//src/flutter:flutter_jit_runner"
  runtime_meta = "//build/flutter/meta/jit_runtime.cml"
}
```

From //build/flutter/flutter_build_config.gni:12

### flutter_default_build_cfg

Non-product AOT

**Current value (from the default):**

```none
{
  enable_asserts = false
  is_aot = true
  is_debug = false
  is_product = false
  platform_name = "flutter_runner"
  runner_dep = "//src/flutter:flutter_aot_runner"
  runtime_meta = "//build/flutter/meta/aot_runtime.cml"
}
```

From //build/flutter/config.gni:20

### flutter_driver_enabled

Enables/Disables flutter driver using '--args=flutter_driver_enabled=[true/false]'
in fx set. (Disabled by default)
This is effective only on debug builds.

**Current value (from the default):** `false`

From //build/testing/flutter_driver.gni:9

### flutter_force_aot

Forces all Flutter apps to use an AOT runner regardless of whether the build is debug or
release.

**Current value (from the default):** `false`

From //build/flutter/args.gni:12

### flutter_force_product

If set to true, will force the runners to be built in
product mode which means they will not have an exposed vm service

**Current value (from the default):** `false`

From //build/flutter/args.gni:8

### flutter_profile_build_cfg

Builds the component in a non-product AOT build. This will
launch the vm service in the runner.

**Current value (from the default):**

```none
{
  enable_asserts = false
  is_aot = true
  is_debug = false
  is_product = false
  platform_name = "flutter_runner"
  runner_dep = "//src/flutter:flutter_aot_runner"
  runtime_meta = "//build/flutter/meta/aot_runtime.cml"
}
```

From //build/flutter/flutter_build_config.gni:38

### flutter_release_build_cfg

Builds the component in a product AOT build. This will
not launch the vm service in the runner.

**Current value (from the default):**

```none
{
  enable_asserts = false
  is_aot = true
  is_debug = false
  is_product = true
  platform_name = "flutter_runner"
  runner_dep = "//src/flutter:flutter_aot_product_runner"
  runtime_meta = "//build/flutter/meta/aot_product_runtime.cml"
}
```

From //build/flutter/flutter_build_config.gni:50

### font_catalog_paths

**Current value (from the default):** `["//prebuilt/third_party/fonts/fuchsia.font_catalog.json"]`

From //src/fonts/build/font_args.gni:17

### font_pkg_entries

Merged contents of .font_pkgs.json files. Populated in fonts.gni.

**Current value (from the default):** `[]`

From //src/fonts/build/font_args.gni:32

### font_pkgs_paths

Locations of .font_pkgs.json files, which list the locations of font files
within the workspace, as well as safe names that are derived from the fonts'
file names and can be used to name Fuchsia packages.

**Current value (from the default):** `["//prebuilt/third_party/fonts/fuchsia.font_pkgs.json"]`

From //src/fonts/build/font_args.gni:22

### fonts_dir

Directory into which all fonts are checked out from CIPD

**Current value (from the default):** `"//prebuilt/third_party/fonts"`

From //src/fonts/build/font_args.gni:12

### format_minfs_on_corruption

If format_minfs_on_corruption is true (the default), fshost formats minfs partition on finding
it corrupted.  Set to false to keep the devices in a corrupted state which might be of help to
debug issues.

**Current value (from the default):** `true`

From //src/storage/fshost/generated_fshost_config.gni:27

### fshost_watch_for_nand

Make fshost watch for NAND devices.

**Current value (from the default):** `false`

From //src/storage/fshost/generated_fshost_config.gni:38

### fuchsia_async_trace_level_logging

Determines whether the fuchsia_async library used by many Rust targets will be compiled
with TRACE level log statements that increase binary size a measurable amount.
TODO(fxbug.dev/80742) move this to a toolchain to allow multiple products to build together

**Current value (from the default):** `true`

From //build/product.gni:35

### fuchsia_product_assembly_config_label

The product assembly config used to configure the main Fuchsia image.

**Current value (from the default):** `false`

From //build/product.gni:38

### fuchsia_route_sources_config

An optional file path to the route_sources verifier configuration to be used
on the assembled fuchsia system.

**Current value (from the default):** `""`

From //build/security.gni:143

### fuchsia_sdk_root

Consumers of the Fuchsia SDK instantiate templates for various SDK parts at
a specific spot within their buildroots. The target name for the specific
part is then derived from the part name as specified in the meta.json
manifest. Different buildroot instantiate the SDK parts at different
locations and then set this variable. GN rules can then prefix this variable
name in SDK builds to the name of the SDK part. This flag is meaningless in
non-SDK buildroots.

**Current value (from the default):** `""`

From //build/fuchsia/sdk.gni:17

### fuchsia_static_pkgs_goldens

An optional list of golden files for fuchsia.zbi static pkgs list. If
specified, they would be compared against fuchsia.zbi static pkgs list
during build time. At least one of the golden files must match.
In normal case, there should only be one golden file in this list.
During a soft transition where changes are made in a different repo than
the golden file repo, user need to
1. copy the old golden file before the change to '*.orig'
2. create a new golden file reflecting the changes
3. add both the old golden file and new golden file to this list. e.g. there
would be 'product.txt' and 'product.txt.orig' in this list and check in the
above changes.
4. check in the changes that is made in a different repo.
5. delete 'product.txt.orig' and remove it from this list.

**Current value (from the default):** `[]`

From //build/security.gni:114

### fuchsia_structured_config_policy

An optional file path to the structured configuration policy to be used on the
assembled fuchsia system. Defaults to no enforcement. Policy must be provided
for any product which is not an `eng` build type.

**Current value (from the default):** `""`

From //build/security.gni:134

### fuchsia_verify_component_resolvers_allowlist

**Current value (from the default):** `"//src/security/policy/component_resolvers_policy.json5"`

From //build/security.gni:197

### fuchsia_verify_routes_component_tree_config

An optional component tree configuration file used to finalize dynamic
elements of the component tree constructed for route verification on the
fuchsia assembled system. When non-empty, this value is passed as the
`--component-tree-config` option to `ffx scrutiny verify routes` to verify
routes in the fuchsia component tree.

**Current value (from the default):** `""`

From //build/security.gni:182

### fuchsia_verify_routes_exceptions_allowlist

**Current value (from the default):** `"//src/security/policy/build/verify_routes_exceptions_allowlist.json5"`

From //build/security.gni:155

### fuchsia_verify_routes_exceptions_allowlist_product

Same as fuchsia_verify_routes_exceptions_allowlist, except these allowlists
get added according to product-specific configuration.

**Current value (from the default):** `[]`

From //build/security.gni:171

### fuchsia_zbi_bootfs_filelist_goldens

An optional list of golden files for fuchsia.zbi bootFS file list. If
specified, they would be compared against fuchsia.zbi bootFS file list
during build time. At least one of the golden files must match.
In normal case, there should only be one golden file in this list.
During a soft transition where changes are made in a different repo than
the golden file repo, user need to
1. copy the old golden file before the change to '*.orig'
2. create a new golden file reflecting the changes
3. add both the old golden file and new golden file to this list. e.g. there
would be 'product.txt' and 'product.txt.orig' in this list and check in the
above changes.
4. check in the changes that is made in a different repo.
5. delete 'product.txt.orig' and remove it from this list.

**Current value (from the default):** `[]`

From //build/security.gni:54

### fuchsia_zbi_bootfs_packages_goldens

An optional list of golden files for fuchsia.zbi bootFS package index. If
specified, they would be compared against fuchsia.zbi bootFS package index
during build time. At least one of the golden files must match.
In normal case, there should only be one golden file in this list.
During a soft transition where changes are made in a different repo than
the golden file repo, user need to
1. copy the old golden file before the change to '*.orig'
2. create a new golden file reflecting the changes
3. add both the old golden file and new golden file to this list. e.g. there
would be 'product.txt' and 'product.txt.orig' in this list and check in the
above changes.
4. check in the changes that is made in a different repo.
5. delete 'product.txt.orig' and remove it from this list.

**Current value (from the default):** `[]`

From //build/security.gni:84

### fuchsia_zbi_kernel_cmdline_goldens

An optional list of golden files for fuchsia.zbi kernel cmdline args. If
specified, they would be compared against fuchsia.zbi kernel cmdline during
build time.
In normal case, there should only be golden file in this list.
During a soft transition where changes are made in a different repo than
the golden file repo, user need to
1. copy the old golden file before the change to '*.orig'
2. create a new golden file reflecting the changes
3. add both the old golden file and new golden file to this list. e.g. there
would be 'product.txt' and 'product.txt.orig' in this list and check in the
above changes.
4. check in the changes that is made in a different repo.
5. delete 'product.txt.orig' and remove it from this list.

**Current value (from the default):** `[]`

From //build/security.gni:24

### futex_tracing_enabled

Enables kernel tracing of futex interactions

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:63

### fvm_emmc_partition_size

The size in bytes of the FVM partition on the target eMMC devices.
Specifying this parameter will lead build to generate a fvm.fastboot.blk
suitable for flashing through fastboot for eMMC devices.

**Current value (from the default):** `false`

From //build/images/fvm.gni:11

### fvm_fastboot_compression

How to compress the FVM image used for fastboot flashing.
Possible values:
  * "none": no compression
  * "default": no compression on NAND, lz4 compression on eMMC.
  * any other value is passed as the FVM "--compress" arg

**Current value (from the default):** `"default"`

From //build/images/fvm.gni:95

### fvm_ftl_nand_block_count

**Current value (from the default):** `false`

From //build/images/fvm.gni:88

### fvm_ftl_nand_oob_size

**Current value (from the default):** `false`

From //build/images/fvm.gni:86

### fvm_ftl_nand_page_size

Specifying these variables will generate a NAND FVM image suitable for
directly flashing via fastboot. The NAND characteristics are required
in order to properly initialize the FTL metadata in the OOB area.
`fvm_max_disk_size` should also be nonzero or else minfs will not have any
room to initialize on boot.

**Current value (from the default):** `false`

From //build/images/fvm.gni:85

### fvm_ftl_nand_pages_per_block

**Current value (from the default):** `false`

From //build/images/fvm.gni:87

### fvm_max_disk_size

The max size of the disk where the FVM is written. This is used for
preallocating metadata to determine how much the FVM can expand on disk.
Only applies to sparse FVM images. At sparse image construction time, the
build fails if the inputs are larger than `fvm_max_disk_size`. At paving
time, the FVM will be sized to the target's disk size up to
`fvm_max_disk_size`. If the size of the disk increases after initial paving,
the FVM will resize up to `fvm_max_disk_size`. During paving, if the target
FVM has declared a smaller size than `fvm_max_disk_size`, the FVM is
reinitialized to the larger size.
The default value is false which sets the max disk size to the size of the disk
at pave/format time.

**Current value (from the default):** `false`

From //build/images/fvm.gni:24

### fvm_partition

**Current value (from the default):** `""`

From //build/images/args.gni:116

### fvm_reserved_slices

Number of slices reserved by FVM for internal usage. A reservation
partition will be added to the FVM image, containing this many slices.
If set to 0, then no reservation partition will be added.

**Current value (from the default):** `0`

From //build/images/fvm.gni:39

### fvm_slice_size

The size of the FVM partition images "slice size". The FVM slice size is a
minimum size of a particular chunk of a partition that is stored within
FVM. A very small slice size may lead to decreased throughput. A very large
slice size may lead to wasted space. The selected default size of 8mb is
selected for conservation of space, rather than performance.
LINT.IfChange

**Current value (from the default):** `8388608`

From //build/images/fvm.gni:32

### fxfs_blob

Use Fxfs's blob implementation

**Current value (from the default):** `false`

From //src/storage/fshost/generated_fshost_config.gni:45

### fxfs_partition

**Current value (from the default):** `""`

From //build/images/args.gni:117

### gcc_tool_dir

Directory where the GCC toolchain binaries ("gcc", "nm", etc.) are found.
This directory is expected to contain `aarch64-elf-*` and `x86_64-elf-*`
tools used to build for the Fuchsia targets.  This directory will not be
used for host tools.  If this is "", then a standard prebuilt is used.

**Current value (from the default):** `""`

From //build/toolchain/zircon/gcc.gni:15

### generate_plasa_artifacts

If set, causes the plasa artifacts to be generated.  Not all builds need to
use the plasa artifacts, so we set the default to skip the generation.

**Current value (from the default):** `false`

From //build/sdk/plasa/config.gni:8

### gigaboot_backends

Specifies the gn target that implements the required backends defined in
`gigaboot/cpp/backends.h`

**Current value (from the default):** `"//src/firmware/gigaboot/cpp:backends_nuc"`

From //src/firmware/gigaboot/cpp/backends.gni:8

### gigaboot_eng_permanent_attributes

Permanent attributes file for eng gigaboot

**Current value (from the default):** `"//third_party/android/platform/external/avb/test/data/atx_permanent_attributes.bin"`

From //src/firmware/gigaboot/cpp/backends.gni:11

### gigaboot_user_permanent_attributes

Permanent attributes file for prod-signed gigaboot. Setting this enables
target //src/firmware/gigaboot/cpp:user-esp

**Current value (from the default):** `""`

From //src/firmware/gigaboot/cpp/backends.gni:19

### gigaboot_userdebug_permanent_attributes

Permanent attributes file for userdebug gigaboot. Setting this enables target
//src/firmware/gigaboot/cpp:userdebug-esp

**Current value (from the default):** `""`

From //src/firmware/gigaboot/cpp/backends.gni:15

### go_vet_enabled

  go_vet_enabled
    [bool] if false, go vet invocations are disabled for all builds.

**Current value (from the default):** `false`

From //build/go/go_build.gni:22

### gocache_dir

  gocache_dir
    Directory GOCACHE environment variable will be set to. This directory
    will have build and test results cached, and is safe to be written to
    concurrently. If overridden, this directory must be a full path.

**Current value (from the default):** `"/b/s/w/ir/x/w/fuchsia/out/not-default/.gocache"`

From //build/go/go_build.gni:18

### goma_dir

Directory containing the Goma source code.  This can be a GN
source-absolute path ("//...") or a system absolute path.

**Current value (from the default):** `"//prebuilt/third_party/goma/linux-x64"`

From //build/toolchain/goma.gni:17

### gpt_image

GUID Partition Table (GPT) image.

Typically useful for initially flashing a device from zero-state.

**Current value (from the default):** `""`

From //build/images/args.gni:83

### graphics_compute_generate_debug_shaders


Set to true in your args.gn file to generate pre-processed and
auto-formatted shaders under the "debug" sub-directory of HotSort
and Spinel target generation output directories.

These are never used, but can be reviewed manually to verify the
impact of configuration parameters, or when modifying a compute
shader.

Example results:

  out/default/
    gen/src/graphics/lib/compute/
       hotsort/targets/hs_amd_gcn3_u64/
          comp/
            hs_transpose.comp -> unpreprocessed shader
          debug/
            hs_transpose.glsl -> preprocessed shader


**Current value (from the default):** `true`

From //src/graphics/lib/compute/gn/glsl_shader_rules.gni:29

### graphics_compute_generate_spirv_debug_info


If you're using GPU-assisted validation then it's useful to
include debug info in combination with skipping the spirv-opt and
spirv-reduce pass.


**Current value (from the default):** `false`

From //src/graphics/lib/compute/gn/glsl_shader_rules.gni:47

### graphics_compute_skip_spirv_opt


At times we may want to compare the performance of unoptimized
vs. optimized shaders.  On desktop platforms, use of spirv-opt
doesn't appear to provide major performance improvements but it
significantly reduces the size of the SPIR-V modules.

Disabling the spirv-opt pass may also be useful in identifying and
attributing code generation bugs.


**Current value (from the default):** `false`

From //src/graphics/lib/compute/gn/glsl_shader_rules.gni:40

### grpc_use_static_linking

TODO(169395837): Somehow gRPC symbols cannot be found on Android.
Keep using static linking for now.
In windows and mac use static linking.
Use static linking on Chrome OS as a workaround for the symbol lookup
error(crbug/1241330) due to a gRPC version mismatch between what Chrome
uses and what CrOS provides.

**Current value (from the default):** `false`

From //third_party/grpc/BUILD.gn:21

### hangcheck_timeout_ms

Set this to accommodate long running tests

**Current value (from the default):** `0`

From //src/graphics/drivers/msd-intel-gen/src/BUILD.gn:9

### has_board

This is a build that imports a board (vs. sdk).  If a board is set
(fx set <product>.<board>) this is true.

**Current value for `target_cpu = "arm64"`:** `true`

From //boards/common/arm64-common.gni:7

**Overridden from the default:** `false`

From //BUILD.gn:94

**Current value for `target_cpu = "x64"`:** `true`

From //boards/common/x64-common.gni:10

**Overridden from the default:** `false`

From //BUILD.gn:94

### have_libvulkan_arm_mali

Driver developers can set this to true if they are manually including a Mali package. This will
change test environments so Vulkan tests run on boards with Mali GPUs.

**Current value (from the default):** `false`

From //src/graphics/lib/magma/gnbuild/magma.gni:34

### host_byteorder

**Current value (from the default):** `"undefined"`

From //build/config/host_byteorder.gni:7

### host_cpu

**Current value (from the default):** `"x64"`

### host_labels

If you add labels to this variable, these will be included in the 'host'
artifact set, which represents an additional set of host-only software that
is produced by the build.

**Current value for `target_cpu = "arm64"`:** `[]`

From //out/not-default/args.gn:8

**Overridden from the default:** `[]`

From //BUILD.gn:79

**Current value for `target_cpu = "x64"`:** `[]`

From //out/not-default/args.gn:8

**Overridden from the default:** `[]`

From //BUILD.gn:79

### host_os

**Current value (from the default):** `"linux"`

### host_tools_dir

This is the directory where host tools intended for manual use by
developers get installed.  It's something a developer might put
into their shell's $PATH.  Host tools that are just needed as part
of the build do not get copied here.  This directory is only for
things that are generally useful for testing or debugging or
whatnot outside of the GN build itself.  These are only installed
by an explicit install_host_tools() rule (see //build/host.gni).

**Current value (from the default):** `"//out/not-default/host-tools"`

From //build/host.gni:13

### hwasan_default_options

Default [HawrdwareAddressSanitizer](https://clang.llvm.org/docs/HardwareAssistedAddressSanitizerDesign.html)
options (before the `HWASAN_OPTIONS` environment variable is read at
runtime).  This can be set as a build argument to affect most "hwasan"
variants in $variants (which see), or overridden in $toolchain_args in
one of those variants.  This can be a list of strings or a single string.

Note that even if this is empty, programs in this build **cannot** define
their own `__hwasan_default_options` C function.  Instead, they can use a
sanitizer_extra_options() target in their `deps` and then any options
injected that way can override that option's setting in this list.

**Current value (from the default):** `[]`

From //build/config/sanitizers/sanitizer_default_options.gni:94

### i_can_haz_atlas_camera

If true, power on the Atlas camera at boot.
TODO(fxbug.dev/81684): remove once we have a better way to manage ACPI device power.

**Current value (from the default):** `false`

From //src/devices/board/lib/acpi/BUILD.gn:12

### icons_path

Path to file to use for recovery logo

**Current value (from the default):** `"//src/recovery/system/res/ota_icons.riv"`

From //src/recovery/system/system_recovery_args.gni:7

### icu_copy_icudata_to_root_build_dir

If set, the ":icudata" target will copy the ICU data to $root_build_dir.

**Current value (from the default):** `false`

From //build/icu.gni:27

### icu_disable_thin_archive

If true, compile icu into a standalone static library. Currently this is
only useful on Chrome OS.

**Current value (from the default):** `false`

From //build/icu.gni:19

### icu_fuchsia_extra_compile_flags

Fuchsia sometimes requires extra compilation flags for ICU to adapt it to
its current toolchain. Since it takes a while for ICU to roll through
Fuchsia, it can take a long time from an ICU commit to a fix rolling into
Fuchsia. This flag allows us to define the flag ahead of time in
//build/icu.gni, and remove the rollout issues.

**Current value (from the default):** `[]`

From //build/icu.gni:38

### icu_fuchsia_override_data_dir

If set to nonempty, this is the label of the directory to be used to pull
the ICU data files content.  The setting has effect only when building
inside the Fuchsia source tree.

**Current value (from the default):** `""`

From //build/icu.gni:24

### icu_major_version_number

Contains the major version number of the ICU library, for dependencies that
need different configuration based on the library version. Currently this
is only useful in Fuchsia.

**Current value (from the default):** `"72"`

From //third_party/icu/default/version.gni:13

### icu_root

The GN files for the ICU library are located in this directory.
Some Fuchsia builds use a different value here.

**Current value (from the default):** `"//third_party/icu/default"`

From //build/icu/config.gni:8

### icu_tzres_path

**Current value (from the default):** `"//prebuilt/third_party/tzres"`

From //src/lib/icu/tzdata/icu_tzres_source.gni:26

### icu_tzres_source

Which source location to use for ICU's time zone .res files:
"git" or "prebuilt".

If set to "git", then the tzres files will contain the same time zone
definitions as the ICU data monolith.

If set to "prebuilt", then the tzres files will be retrieved from CIPD
and may be newer than what's available in the ICU data monolith.

**Current value (from the default):** `"prebuilt"`

From //src/lib/icu/tzdata/icu_tzres_source.gni:16

### icu_use_data_file

Tells icu to load an external data file rather than rely on the icudata
being linked directly into the binary.

**Current value (from the default):** `true`

From //build/icu.gni:10

### icu_use_stub_data

If true, then this creates a stub data file. This should be disabled if
a custom data file will be used instead, in order to avoid conflicting
symbols.

**Current value (from the default):** `true`

From //build/icu.gni:15

### icu_use_target_out_dir

If set, the built libraries will live in their respective default output
directories, not the root_build_dir.

**Current value (from the default):** `true`

From //build/icu.gni:31

### images_config_label

The images config information used during assembly.

**Current value for `target_cpu = "arm64"`:** `"//boards/images:arm64"`

From //boards/arm64.gni:41

**Overridden from the default:** `false`

From //build/board.gni:109

**Current value for `target_cpu = "x64"`:** `"//boards/images:x64"`

From //boards/x64.gni:93

**Overridden from the default:** `false`

From //build/board.gni:109

### include_account_in_fvm

Include an account partition in the FVM image if set to true.

**Current value (from the default):** `false`

From //build/images/args.gni:158

### include_clippy

Turns rust targets into a group with both the normal target and clippy target. This
causes clippy targets to get included in the build by default.

**Current value (from the default):** `true`

From //build/rust/config.gni:65

### include_fvm_blob_sparse

Include fvm.blob.sparse.blk image into the build if set to true

**Current value (from the default):** `false`

From //build/images/args.gni:155

### include_internal_fonts

Set to true to include internal fonts in the build.

**Current value (from the default):** `false`

From //src/fonts/build/font_args.gni:7

### include_next_api_level

If true, generate golden files for next api level. This is used by
platform-version-roller when the API level is incremented.

**Current value (from the default):** `false`

From //build/config/fuchsia/platform_version.gni:37

### include_shell_commands_package

Include the shell commands package.  Used as a parameter to
assembled_system().  See documentation there.

**Current value (from the default):** `false`

From //build/images/args.gni:181

### include_zxdb_large_tests

Normally these tests are not built and run because they require large amounts of optional data
be downloaded. Set this to true to enable the build for the zxdb_large_tests.
See symbols/test_data/README.md for how to download the data required for this test.

**Current value (from the default):** `false`

From //src/developer/debug/zxdb/BUILD.gn:12

### inet_config_enable_async_dns_sockets

Tells inet to support additionally support async dns sockets.

**Current value (from the default):** `true`

From //third_party/openweave-core/config.gni:17

### inet_want_endpoint_dns

Tells inet to include support for the corresponding protocol.

**Current value (from the default):** `true`

From //third_party/openweave-core/config.gni:10

### inet_want_endpoint_raw

**Current value (from the default):** `true`

From //third_party/openweave-core/config.gni:11

### inet_want_endpoint_tcp

**Current value (from the default):** `true`

From //third_party/openweave-core/config.gni:12

### inet_want_endpoint_tun

**Current value (from the default):** `true`

From //third_party/openweave-core/config.gni:14

### inet_want_endpoint_udp

**Current value (from the default):** `true`

From //third_party/openweave-core/config.gni:13

### instructions_path

**Current value (from the default):** `"//src/recovery/system/res/instructions.txt"`

From //src/recovery/system/system_recovery_args.gni:8

### is_analysis

If set, the build will produce compilation analysis dumps, used for code
cross-referencing in code search.  The extra work done during analysis
is only needed for cross-referencing builds, so we're keeping the flag
and the analysis overhead turned off by default.

**Current value (from the default):** `false`

From //build/config/BUILDCONFIG.gn:21

### is_debug

Debug build.

**Current value for `target_cpu = "arm64"`:** `false`

From //out/not-default/args.gn:9

**Overridden from the default:** `true`

From //build/config/BUILDCONFIG.gn:24

**Current value for `target_cpu = "x64"`:** `false`

From //out/not-default/args.gn:9

**Overridden from the default:** `true`

From //build/config/BUILDCONFIG.gn:24

### is_perfetto_build_generator

All the tools/gen_* scripts set this to true. This is mainly used to locate
.gni files from //gn rather than //build.

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:82

### is_perfetto_embedder

This is for override via `gn args` (e.g. for tools/gen_xxx). Embedders
based on GN (e.g. v8) should NOT set this and instead directly sets
perfetto_build_with_embedder=true in their GN files.

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:87

### jtrace_enabled

Please refer to https://fuchsia.dev/fuchsia-src/development/debugging/jtrace
for a description of these configuration options.

Note that the special value "auto" is used only by the default definitions
of the entries (below).  It acts as a special value which automatically
chooses a default based on whether or not JTRACE is configured for
persistent tracing, while still allowing a user to explicitly override the
value regardless of whether persistent tracing is enabled or not.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:122

### jtrace_last_entry_storage

**Current value (from the default):** `0`

From //zircon/kernel/params.gni:123

### jtrace_target_buffer_size

**Current value (from the default):** `"auto"`

From //zircon/kernel/params.gni:124

### jtrace_use_large_entries

**Current value (from the default):** `"auto"`

From //zircon/kernel/params.gni:125

### kernel_base

**Current value (from the default):** `"0xffffffff00000000"`

From //zircon/kernel/params.gni:29

### kernel_based_memory_attribution_enabled

Controls the instantiation of AttributionObjects
on ProcessDispatcher creation.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:129

### kernel_debug_level

Enables various kernel debugging and diagnostic features.  Valid
values are between 0-3.  The higher the value, the more that are
enabled.  A value of 0 disables all of them.

TODO(fxbug.dev/41790): This value is derived from assert_level.  Decouple
the two and set kernel_debug_level independently.

**Current value (from the default):** `2`

From //zircon/kernel/params.gni:93

### kernel_debug_print_level

Controls the verbosity of kernel dprintf messages. The higher the value,
the more dprintf messages emitted. Valid values are 0-2 (inclusive):
  0 - CRITCAL / ALWAYS
  1 - INFO
  2 - SPEW

**Current value (from the default):** `2`

From //zircon/kernel/params.gni:100

### kernel_extra_defines

Extra macro definitions for kernel code, e.g. "DISABLE_KASLR",
"ENABLE_KERNEL_LL_DEBUG".

**Current value (from the default):** `[]`

From //zircon/kernel/params.gni:85

### kernel_no_userabi

Build a kernel with no user-space support, for development only.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:132

### kernel_version_string

Version string embedded in the kernel for `zx_system_get_version_string`.
If set to the default "", a string is generated based on the
status of the fuchsia git repository.

**Current value (from the default):** `""`

From //zircon/kernel/lib/version/BUILD.gn:15

### known_variants

List of variants that will form the basis for variant toolchains.
To make use of a variant, set [`select_variant`](#select_variant).

Normally this is not set as a build argument, but it serves to
document the available set of variants.
See also [`universal_variants`](#universal_variants).
Only set this to remove all the default variants here.
To add more, set [`extra_variants`](#extra_variants) instead.

Each element of the list is one variant, which is a scope defining:

  `configs` (optional)
      [list of labels] Each label names a config that will be
      automatically used by every target built in this variant.
      For each config `${label}`, there must also be a target
      `${label}_deps`, which each target built in this variant will
      automatically depend on.  The `variant()` template is the
      recommended way to define a config and its `_deps` target at
      the same time.

  `remove_common_configs` (optional)
  `remove_shared_configs` (optional)
      [list of labels] This list will be removed (with `-=`) from
      the `default_common_binary_configs` list (or the
      `default_shared_library_configs` list, respectively) after
      all other defaults (and this variant's configs) have been
      added.

  `deps` (optional)
      [list of labels] Added to the deps of every target linked in
      this variant (as well as the automatic `${label}_deps` for
      each label in configs).

  `name` (required if configs is omitted)
      [string] Name of the variant as used in
      [`select_variant`](#select_variant) elements' `variant` fields.
      It's a good idea to make it something concise and meaningful when
      seen as e.g. part of a directory name under `$root_build_dir`.
      If name is omitted, configs must be nonempty and the simple names
      (not the full label, just the part after all `/`s and `:`s) of these
      configs will be used in toolchain names (each prefixed by a "-"),
      so the list of config names forming each variant must be unique
      among the lists in `known_variants + extra_variants`.

  `tags` (optional)
      [list of strings] A list of liberal strings describing properties
      of the toolchain instances created from this variant scope. See
      //build/toolchain/variant_tags.gni for the list of available
      values and their meaning.

  `toolchain_args` (optional)
      [scope] Each variable defined in this scope overrides a
      build argument in the toolchain context of this variant.

  `host_only` (optional)
  `target_only` (optional)
      [scope] This scope can contain any of the fields above.
      These values are used only for host or target, respectively.
      Any fields included here should not also be in the outer scope.


**Current value (from the default):**

```none
[{
  configs = ["//build/config/lto"]
  tags = ["lto"]
}, {
  configs = ["//build/config/lto:thinlto"]
  tags = ["lto"]
}, {
  name = "novariant"
}, {
  configs = ["//build/config/profile:coverage"]
  tags = ["instrumented", "coverage", "llvm-profdata", "needs-writable-globals"]
}, {
  configs = ["//build/config/profile:coverage-rust"]
  tags = ["instrumented", "coverage", "llvm-profdata", "needs-writable-globals"]
}, {
  configs = ["//build/config/profile"]
  tags = ["instrumented", "profile", "llvm-profdata", "needs-writable-globals"]
}, {
  configs = ["//build/config/profile:coverage-cts"]
  tags = ["instrumented", "coverage", "llvm-profdata"]
}, {
  configs = ["//build/config/sanitizers:tsan"]
  tags = ["tsan", "instrumentation-runtime", "instrumented", "needs-compiler-abi", "needs-writable-globals", "uses-shadow", "kernel-excluded"]
}, {
  configs = ["//build/config/sanitizers:hwasan"]
  tags = ["hwasan", "instrumentation-runtime", "instrumented", "needs-compiler-abi", "needs-writable-globals", "kernel-excluded", "replaces-allocator", "uses-shadow", "fuchsia-only"]
}, {
  configs = ["//build/config/sanitizers:hwasan", "//build/config/sanitizers:ubsan"]
  tags = ["hwasan", "instrumentation-runtime", "instrumented", "needs-compiler-abi", "needs-writable-globals", "kernel-excluded", "replaces-allocator", "uses-shadow", "fuchsia-only", "ubsan"]
}, {
  configs = ["//build/config/sanitizers:ubsan"]
  remove_common_configs = ["//build/config:no_rtti"]
  tags = ["instrumented", "instrumentation-runtime", "needs-compiler-abi", "needs-writable-globals", "kernel-excluded", "ubsan"]
}, {
  configs = ["//build/config/sanitizers:ubsan", "//build/config/sanitizers:sancov"]
  remove_common_configs = ["//build/config:no_rtti"]
  tags = ["instrumented", "instrumentation-runtime", "needs-compiler-abi", "needs-writable-globals", "kernel-excluded", "sancov", "ubsan"]
}, {
  configs = ["//build/config/sanitizers:asan"]
  host_only = {
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
}
  remove_common_configs = ["//build/config:default_frame_pointers"]
  tags = ["asan", "instrumentation-runtime", "instrumented", "needs-compiler-abi", "needs-writable-globals", "lsan", "replaces-allocator", "uses-shadow", "kernel-excluded"]
  toolchain_args = { }
}, {
  configs = ["//build/config/sanitizers:asan", "//build/config/sanitizers:ubsan"]
  host_only = {
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
}
  remove_common_configs = ["//build/config:default_frame_pointers", "//build/config:no_rtti"]
  tags = ["asan", "instrumentation-runtime", "instrumented", "needs-compiler-abi", "needs-writable-globals", "lsan", "replaces-allocator", "uses-shadow", "kernel-excluded", "ubsan"]
  toolchain_args = { }
}, {
  configs = ["//build/config/sanitizers:asan", "//build/config/sanitizers:sancov"]
  host_only = {
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
}
  remove_common_configs = ["//build/config:default_frame_pointers"]
  tags = ["asan", "instrumentation-runtime", "instrumented", "needs-compiler-abi", "needs-writable-globals", "lsan", "replaces-allocator", "uses-shadow", "kernel-excluded", "sancov"]
  toolchain_args = { }
}, {
  configs = ["//build/config/sanitizers:asan", "//build/config:no-safe-stack"]
  host_only = {
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
}
  name = "kasan"
  remove_common_configs = []
  tags = ["asan", "instrumentation-runtime", "instrumented", "needs-compiler-abi", "needs-writable-globals", "lsan", "replaces-allocator", "uses-shadow", "kernel-only"]
  toolchain_args = { }
}, {
  configs = ["//build/config/sanitizers:asan", "//build/config:no-safe-stack", "//build/config/sanitizers:sancov"]
  host_only = {
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
}
  name = "kasan-sancov"
  remove_common_configs = []
  tags = ["asan", "instrumentation-runtime", "instrumented", "needs-compiler-abi", "needs-writable-globals", "lsan", "replaces-allocator", "uses-shadow", "kernel-only", "sancov"]
  toolchain_args = { }
}, {
  configs = ["//build/config/sanitizers:asan", "//build/config/fuzzer", "//build/config/sanitizers:rust-asan", "//build/config:icf"]
  host_only = {
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
}
  name = "asan-fuzzer"
  remove_common_configs = ["//build/config:default_frame_pointers", "//build/config:icf"]
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
  tags = ["asan", "instrumentation-runtime", "instrumented", "needs-compiler-abi", "needs-writable-globals", "lsan", "replaces-allocator", "uses-shadow", "kernel-excluded", "fuzzer"]
  toolchain_args = {
  asan_default_options = "alloc_dealloc_mismatch=0:check_malloc_usable_size=0:detect_odr_violation=0:max_uar_stack_size_log=16:print_scariness=1:allocator_may_return_null=1:detect_leaks=0:detect_stack_use_after_return=1:malloc_context_size=128:print_summary=1:print_suppressions=0:strict_memcmp=0:symbolize=0"
}
}, {
  configs = ["//build/config/fuzzer", "//build/config/sanitizers:ubsan", "//build/config:icf"]
  name = "ubsan-fuzzer"
  remove_common_configs = ["//build/config:icf", "//build/config:no_rtti"]
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
  tags = ["fuzzer", "instrumented", "instrumentation-runtime", "needs-compiler-abi", "ubsan"]
}, {
  name = "gcc"
  tags = ["gcc"]
}, {
  name = "cxx20"
  toolchain_args = {
  experimental_cxx_version = 20
}
}]
```

From //build/config/BUILDCONFIG.gn:1401

### legacy_base_driver_package_labels

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/common/bringup.gni:22

**Overridden from the default:** `[]`

From //BUILD.gn:40

**Current value for `target_cpu = "x64"`:** `[]`

From //products/common/bringup.gni:22

**Overridden from the default:** `[]`

From //BUILD.gn:40

### legacy_base_package_labels

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/common/bringup.gni:25

**Overridden from the default:** `[]`

From //BUILD.gn:48

**Current value for `target_cpu = "x64"`:** `[]`

From //products/common/bringup.gni:25

**Overridden from the default:** `[]`

From //BUILD.gn:48

### legacy_cache_package_labels

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/common/bringup.gni:28

**Overridden from the default:** `[]`

From //BUILD.gn:57

**Current value for `target_cpu = "x64"`:** `[]`

From //products/common/bringup.gni:28

**Overridden from the default:** `[]`

From //BUILD.gn:57

### legacy_host_labels

**Current value (from the default):** `[]`

From //BUILD.gn:80

### legacy_product_bootfs_labels

A list of binary labels to include in ZBIs built for this product.
product_bootfs_labels = []  (defined in product.gni)

**Current value (from the default):** `[]`

From //BUILD.gn:70

### legacy_product_system_image_deps

A list of binary labels to include in the system_image package.
product_system_image_deps = []  (defined in product.gni)

**Current value (from the default):** `[]`

From //BUILD.gn:74

### legacy_universe_package_labels

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/common/bringup.gni:31

**Overridden from the default:** `[]`

From //BUILD.gn:66

**Current value for `target_cpu = "x64"`:** `[]`

From //products/common/bringup.gni:31

**Overridden from the default:** `[]`

From //BUILD.gn:66

### local_bench

Used to enable local benchmarking/fine-tuning when running benchmarks
in `fx shell`. Pass `--args=local_bench='true'` to `fx set` in order to
enable it.

**Current value (from the default):** `false`

From //src/developer/fuchsia-criterion/BUILD.gn:13

### lock_tracing_enabled

Enable lock contention tracing.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:43

### log_startup_sleep

**Current value (from the default):** `"30000"`

From //src/diagnostics/log_listener/BUILD.gn:12

### lsan_default_options

Default [LeakSanitizer](https://clang.llvm.org/docs/LeakSanitizer.html)
options (before the `LSAN_OPTIONS` environment variable is read at
runtime).  This can be set as a build argument to affect most "lsan"
variants in $variants (which see), or overridden in $toolchain_args in
one of those variants.  This can be a list of strings or a single string.

Note that even if this is empty, programs in this build **cannot** define
their own `__lsan_default_options` C function.  Instead, they can use a
sanitizer_extra_options() target in their `deps` and then any options
injected that way can override that option's setting in this list.

**Current value (from the default):** `[]`

From //build/config/sanitizers/sanitizer_default_options.gni:35

### magma_build_root

**Current value (from the default):** `"//src/graphics/lib/magma"`

From //src/graphics/lib/magma/gnbuild/magma.gni:13

### magma_debug

**Current value (from the default):** `false`

From //src/graphics/lib/magma/src/magma_util/BUILD.gn:14

### magma_enable_tracing

Enable this to include fuchsia tracing capability

**Current value (from the default):** `true`

From //src/graphics/lib/magma/gnbuild/magma.gni:21

### magma_openvx_include

The path to OpenVX headers

**Current value (from the default):** `""`

From //src/graphics/lib/magma/gnbuild/magma.gni:27

### magma_openvx_package

The path to an OpenVX implementation

**Current value (from the default):** `""`

From //src/graphics/lib/magma/gnbuild/magma.gni:30

### magma_python_path

**Current value (from the default):** `"/b/s/w/ir/x/w/fuchsia/third_party/mako"`

From //src/graphics/lib/magma/gnbuild/magma.gni:18

### max_blob_contents_size

Maximum allowable contents for the /blob in a release mode build.
False means no limit.
contents_size refers to contents stored within the filesystem (regardless
of how they are stored).

**Current value for `target_cpu = "arm64"`:** `5216665600`

From //boards/common/arm64-common.gni:13

**Overridden from the default:** `false`

From //build/images/filesystem_limits.gni:10

**Current value for `target_cpu = "x64"`:** `5216665600`

From //boards/x64.gni:19

**Overridden from the default:** `false`

From //build/images/filesystem_limits.gni:10

### max_blob_image_size

Maximum allowable image_size for /blob in a release mode build.
Zero means no limit.
image_size refers to the total image size, including both contents and
metadata.

**Current value (from the default):** `"0"`

From //build/images/filesystem_limits.gni:21

### max_data_contents_size

Maximum allowable contents_size for /data in a release mode build.
Zero means no limit.
contents_size refers to contents stored within the filesystem (regardless
of how they are stored).

**Current value (from the default):** `"0"`

From //build/images/filesystem_limits.gni:27

### max_data_image_size

Maximum allowable image_size for /data in a release mode build.
Zero means no limit.
image_size refers to the total image size, including both contents and
metadata.

**Current value (from the default):** `"0"`

From //build/images/filesystem_limits.gni:33

### max_fuchsia_zbi_size

Maximum allowable size for fuchsia.zbi

**Current value for `target_cpu = "arm64"`:** `16777216`

From //boards/common/arm64-common.gni:39

**Overridden from the default:** `0`

From //build/images/filesystem_limits.gni:36

**Current value (from the default):** `0`

From //build/images/filesystem_limits.gni:36

### max_log_disk_usage

Controls how many bytes of space on disk are used to persist device logs.
Should be a string value that only contains digits.

**Current value (from the default):** `"0"`

From //src/diagnostics/log_listener/BUILD.gn:11

### max_zedboot_zbi_size

Maximum allowable size for zedboot.zbi

**Current value for `target_cpu = "arm64"`:** `16777216`

From //boards/common/arm64-common.gni:40

**Overridden from the default:** `0`

From //build/images/filesystem_limits.gni:39

**Current value (from the default):** `0`

From //build/images/filesystem_limits.gni:39

### mbedtls_config_file

Configuration file for MbedTLS.

**Current value (from the default):** `"mbedtls-config.h"`

From //third_party/openthread/third_party/mbedtls/BUILD.gn:30

### meta_package_labels

A list of labels for packages that are appended to the set of base packages,
but depend on all the other base, cache, and universe packages, therefore
they must be separated into their own list.

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/common/bringup.gni:19

**Overridden from the default:** `[]`

From //build/images/args.gni:99

**Current value for `target_cpu = "x64"`:** `[]`

From //products/common/bringup.gni:19

**Overridden from the default:** `[]`

From //build/images/args.gni:99

### min_crashlog_size

Controls minimum amount of space of persistent RAM to reserve for the
crashlog.  When other features (such as persistent debug logging) are
enabled, this value controls the minimum number of bytes which will
_always_ be reserved for the crashlog (subject to the total amount of
available persistent RAM), regardless of how much ram is requested by other
users of persistent RAM.  Must be a multiple of 128 bytes.

**Current value (from the default):** `2048`

From //zircon/kernel/lib/crashlog/params.gni:14

### minfs_maximum_runtime_bytes

minfs_maximum_runtime_bytes is an upper bound on the partition size on the device. Partitions
can grow as needed if there are extra slices available in FVM. This limit prevents the minfs
partition from taking too much space away from other uses.

Pass the empty string for no limit.

**Current value (from the default):** `""`

From //src/storage/fshost/generated_fshost_config.gni:22

### mini_chromium_is_chromeos_ash

**Current value (from the default):** `false`

From //third_party/mini_chromium/src/build/platform.gni:31

### mini_chromium_is_chromeos_lacros

**Current value (from the default):** `false`

From //third_party/mini_chromium/src/build/platform.gni:30

### monolithic_binaries

Only for local development. When true the binaries (perfetto, traced, ...)
are monolithic and don't use a common shared library. This is mainly to
avoid LD_LIBRARY_PATH dances when testing locally.
On Windows we default to monolithic executables, because pairing
dllexport/import adds extra complexity for little benefit. Te only reason
for monolithic_binaries=false is saving binary size, which matters mainly on
Android. See also comments on PERFETTO_EXPORT_ENTRYPOINT in compiler.h.

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:214

### msd_arm_enable_all_cores

Enable all 8 cores, which is faster but emits more heat.

**Current value (from the default):** `true`

From //src/graphics/drivers/msd-arm-mali/src/BUILD.gn:9

### msd_arm_enable_cache_coherency

With this flag set the system tries to use cache coherent memory if the
GPU supports it.

**Current value (from the default):** `true`

From //src/graphics/drivers/msd-arm-mali/src/BUILD.gn:13

### msd_arm_enable_protected_debug_swap_mode

In protected mode, faults don't return as much information so they're much harder to debug. To
work around that, add a mode where protected atoms are executed in non-protected mode and
vice-versa.

NOTE: The memory security ranges should also be set (in TrustZone) to the opposite of normal, so
that non-protected mode accesses can only access protected memory and vice versa.  Also,
growable memory faults won't work in this mode, so larger portions of growable memory should
precommitted (which is not done by default).

**Current value (from the default):** `false`

From //src/graphics/drivers/msd-arm-mali/src/BUILD.gn:23

### msd_build_root

**Current value (from the default):** `"//src/graphics/drivers"`

From //src/graphics/lib/magma/gnbuild/magma.gni:15

### msd_intel_gen_build_root

**Current value (from the default):** `"//src/graphics/drivers/msd-intel-gen"`

From //src/graphics/lib/magma/gnbuild/magma.gni:16

### msd_intel_gen_enable_hardware_unit_tests

**Current value (from the default):** `false`

From //src/graphics/drivers/msd-intel-gen/tests/integration/BUILD.gn:8

### msd_vsi_vip_enable_suspend

Enable suspend.
This will stop the ring buffer and suspend the clks when there are no
submitted commands.

**Current value (from the default):** `true`

From //src/graphics/drivers/msd-vsi-vip/BUILD.gn:14

### netsvc_extra_defines

**Current value (from the default):** `[]`

From //src/bringup/bin/netsvc/BUILD.gn:9

### omaha_app_id

Default app id will always return no update.

**Current value (from the default):** `"fuchsia-test:no-update"`

From //src/sys/pkg/bin/omaha-client/BUILD.gn:16

### openthread_config_anycast_locator_enable

Enable anycast locator functionality

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:82

### openthread_config_assert_enable

Enable assertions.

**Current value (from the default):** `true`

From //third_party/openthread/etc/gn/openthread.gni:79

### openthread_config_backbone_router_enable

Enable backbone router functionality

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:85

### openthread_config_border_agent_enable

Enable border agent support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:88

### openthread_config_border_router_enable

Enable border router support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:91

### openthread_config_border_routing_enable

Enable border routing support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:94

### openthread_config_channel_manager_enable

Enable channel manager support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:97

### openthread_config_channel_monitor_enable

Enable channel monitor support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:100

### openthread_config_child_supervision_enable

Enable child supervision support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:103

### openthread_config_coap_api_enable

Enable coap api support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:106

### openthread_config_coap_observe_api_enable

Enable coap observe (RFC7641) api support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:112

### openthread_config_coap_secure_api_enable

Enable secure coap api support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:109

### openthread_config_coexistence_enable

Enable radio coexistence

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:239

### openthread_config_commissioner_enable

Enable commissioner support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:115

### openthread_config_deps

Extra deps for OpenThread configuration.

**Current value (from the default):** `[]`

From //third_party/openthread/etc/gn/openthread.gni:38

### openthread_config_dhcp6_client_enable

Enable DHCP6 client support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:121

### openthread_config_dhcp6_server_enable

Enable DHCP6 server support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:124

### openthread_config_diag_enable

Enable diagnostic support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:127

### openthread_config_dns_client_enable

Enable DNS client support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:130

### openthread_config_dnssd_server_enable

Enable DNS-SD server support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:133

### openthread_config_dua_enable

Enable Domain Unicast Address feature for Thread 1.2

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:139

### openthread_config_ecdsa_enable

Enable ECDSA support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:136

### openthread_config_enable_builtin_mbedtls_management

**Current value (from the default):** `true`

From //third_party/openthread/etc/gn/openthread.gni:236

### openthread_config_file

OpenThread config header.

**Current value (from the default):** `"<openthread-config-fuchsia.h>"`

From //third_party/openthread/etc/gn/openthread.gni:35

### openthread_config_full_logs

Enable full logs

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:223

### openthread_config_heap_external_enable

Enable external heap support

**Current value (from the default):** `true`

From //third_party/openthread/etc/gn/openthread.gni:145

### openthread_config_ip6_fragmentation_enable

Enable ipv6 fragmentation support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:148

### openthread_config_ip6_slaac_enable

Enable support for adding of auto-configured SLAAC addresses by OpenThread

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:202

### openthread_config_jam_detection_enable

Enable jam detection support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:151

### openthread_config_joiner_enable

Enable joiner support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:154

### openthread_config_legacy_enable

Enable legacy network support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:157

### openthread_config_link_metrics_initiator_enable

Enable link metrics initiator

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:160

### openthread_config_link_metrics_subject_enable

Enable link metrics subject

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:163

### openthread_config_link_raw_enable

Enable link raw service

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:166

### openthread_config_log_level_dynamic_enable

Enable dynamic log level control

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:169

### openthread_config_log_output

Log output: none, debug_uart, app, platform

**Current value (from the default):** `""`

From //third_party/openthread/etc/gn/openthread.gni:76

### openthread_config_mac_csl_receiver_enable

Enable csl receiver

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:118

### openthread_config_mac_filter_enable

Enable mac filter support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:172

### openthread_config_message_use_heap

Enable use built-in heap for message buffers

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:175

### openthread_config_mle_long_routes_enable

Enable MLE long routes extension (experimental, breaks Thread conformance]

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:178

### openthread_config_mlr_enable

Enable Multicast Listener Registration feature for Thread 1.2

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:142

### openthread_config_multiple_instance_enable

Enable multiple instances

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:184

### openthread_config_ncp_hdlc_enable

Enable NCP HDLC support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:232

### openthread_config_ncp_spi_enable

Enable NCP SPI support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:229

### openthread_config_otns_enable

Enable OTNS support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:226

### openthread_config_ping_sender

Enable ping sender support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:214

### openthread_config_platform_netif_enable

Enable platform netif support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:187

### openthread_config_platform_udp_enable

Enable platform UDP support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:190

### openthread_config_reference_device_enable

Enable Thread Test Harness reference device support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:193

### openthread_config_sntp_client_enable

Enable SNTP Client support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:205

### openthread_config_srp_client_enable

Enable SRP Client support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:208

### openthread_config_srp_server_enable

Enable SRP Server support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:211

### openthread_config_thread_version

Thread version: 1.1, 1.2

**Current value (from the default):** `"1.3"`

From //third_party/openthread/etc/gn/openthread.gni:73

### openthread_config_time_sync_enable

Enable the time synchronization service feature

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:217

### openthread_config_tmf_netdata_service_enable

Enable support for injecting Service entries into the Thread Network Data

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:196

### openthread_config_tmf_network_diag_mtd_enable

Enable TMF network diagnostics on MTDs

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:181

### openthread_config_udp_forward_enable

Enable UDP forward support

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:220

### openthread_core_config_deps

Extra deps for OpenThread core configuration.

**Current value (from the default):** `[]`

From //third_party/openthread/etc/gn/openthread.gni:50

### openthread_core_config_platform_check_file

OpenThread platform-specific config check header

**Current value (from the default):** `""`

From //third_party/openthread/etc/gn/openthread.gni:47

### openthread_enable_core_config_args

Configure OpenThread via GN arguments.

**Current value (from the default):** `true`

From //third_party/openthread/etc/gn/openthread.gni:67

### openthread_external_mbedtls

Use external mbedtls. If blank, internal mbedtls will be used.

**Current value (from the default):** `""`

From //third_party/openthread/etc/gn/openthread.gni:56

### openthread_external_platform

Use external platform.

**Current value (from the default):** `""`

From //third_party/openthread/etc/gn/openthread.gni:53

### openthread_package_name

Package name for OpenThread.

**Current value (from the default):** `"OPENTHREAD"`

From //third_party/openthread/etc/gn/openthread.gni:59

### openthread_package_version

Package version for OpenThread.

**Current value (from the default):** `"1.0.0"`

From //third_party/openthread/etc/gn/openthread.gni:62

### openthread_project_core_config_file

OpenThread project-specific core config header

**Current value (from the default):** `""`

From //third_party/openthread/etc/gn/openthread.gni:44

### openthread_project_include_dirs

Include directories for project specific configs.

**Current value (from the default):** `[]`

From //third_party/openthread/etc/gn/openthread.gni:41

### openthread_settings_ram

Enable volatile-only storage of settings

**Current value (from the default):** `false`

From //third_party/openthread/etc/gn/openthread.gni:199

### optimize

* `none`: really unoptimized, usually only build-tested and not run
* `debug`: "optimized for debugging", light enough to avoid confusion
* `default`: default optimization level
* `size`:  optimized for space rather than purely for speed
* `speed`: optimized purely for speed
* `sanitizer`: optimized for sanitizers (ASan, etc.)
* `profile`: optimized for coverage/profile data collection
* `coverage`: optimized for coverage data collection

**Current value (from the default):** `"size"`

From //build/config/compiler.gni:22

### output_breakpad_syms

Sets if we should output breakpad symbols for Fuchsia binaries.

**Current value (from the default):** `false`

From //build/config/BUILDCONFIG.gn:27

### output_gsym

Controls whether we should output GSYM files for Fuchsia binaries.

**Current value (from the default):** `false`

From //build/config/BUILDCONFIG.gn:30

### override_target_api_level

Allows building the platform source code for a specific API level.

**Current value (from the default):** `-1`

From //build/config/fuchsia/platform_version.gni:11

### package_flavor_selections

Used to configure the set of package flavors desired.

Usage:

 package_flavor_selections = [
   {
     name = "snazzy"
     flavor = "with_hooks"
   },
   {
     name = "some_other_package"
     flavor = "some_other_flavor"
   },
 ]

The above specifies that the package "snazzy" should use the
"with_hooks" flavor, and that "some_other_package" should use
the "some_other_flavor" flavor instead of their default flavor
all other packages using this template would use their default
package flavors.

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/common/bringup.gni:33

**Overridden from the default:** `[]`

From //build/packages/prebuilt_package_with_flavors.gni:29

**Current value for `target_cpu = "x64"`:** `[]`

From //products/common/bringup.gni:33

**Overridden from the default:** `[]`

From //build/packages/prebuilt_package_with_flavors.gni:29

### page_compression_enabled

TODO(fxbug.dev/60238): For use while the related RFC is in iterate and can
be removed once the RFC is finalized.
Controls kernel page compression and the linking and instantiation of its
dependencies.
Defaults to true only when the kernel_debug_level is high enough to enable
in testing but disable for production.

**Current value (from the default):** `true`

From //zircon/kernel/params.gni:145

### partitions_config_contents

**Current value for `target_cpu = "arm64"`:** `["//out/not-default/fuchsia.esp.blk"]`

From //boards/arm64.gni:44

**Overridden from the default:** `[]`

From //build/board.gni:128

**Current value for `target_cpu = "x64"`:** `["//out/not-default/fuchsia.esp.blk"]`

From //boards/x64.gni:96

**Overridden from the default:** `[]`

From //build/board.gni:128

### partitions_config_label

The partitions config information used to create an update package and
product bundle.

**Current value for `target_cpu = "arm64"`:** `"//boards/partitions:arm64"`

From //boards/arm64.gni:43

**Overridden from the default:** `"//boards/partitions:default"`

From //build/board.gni:127

**Current value for `target_cpu = "x64"`:** `"//boards/partitions:x64"`

From //boards/x64.gni:95

**Overridden from the default:** `"//boards/partitions:default"`

From //build/board.gni:127

### perfetto_build_with_android

The Android blueprint file generator set this to true (as well as
is_perfetto_build_generator). This is just about being built in the
Android tree (AOSP and internal) and is NOT related with the target OS.
In standalone Android builds and Chromium Android builds, this is false.

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:78

### perfetto_enable_git_rev_version_header

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:257

### perfetto_force_dcheck

Whether DCHECKs should be enabled or not. Values: "on" | "off" | "".
By default ("") DCHECKs are enabled only:
- If DCHECK_ALWAYS_ON is defined (which is mainly a Chromium-ism).
- On debug builds (i.e. if NDEBUG is NOT defined) but only in Chromium,
  Android and standalone builds.
- On all other builds (e.g., SDK) it's off regardless of NDEBUG (unless
  DCHECK_ALWAYS_ON is defined).
See base/logging.h for the implementation of all this.

**Current value (from the default):** `""`

From //third_party/perfetto/gn/perfetto.gni:229

### perfetto_force_dlog

Whether DLOG should be enabled on debug builds (""), all builds ("on"), or
none ("off"). We disable it by default for embedders to avoid spamming their
console.

**Current value (from the default):** `""`

From //third_party/perfetto/gn/perfetto.gni:219

### perfetto_use_system_protobuf

Used by CrOS system builds. Uses the system version of protobuf
from /usr/include instead of the hermetic one.

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:339

### perfetto_use_system_sqlite

Used by CrOS system builds. Uses the system version of sqlite
from /usr/include instead of the hermetic one.

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:343

### perfetto_use_system_zlib

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:345

### perfetto_verbose_logs_enabled

**Current value (from the default):** `true`

From //third_party/perfetto/gn/perfetto.gni:271

### persistent_ram_allocation_granularity

Controls the granularity of allocation of the global pool of persistent RAM.
All features which wish to use persistent RAM to preserve data across reboot
must operate in allocations which are a multiple of this value.  The value
should be a power of two, and typically should be a multiple of the
cacheline size of the target architecture.

**Current value (from the default):** `128`

From //zircon/kernel/params.gni:112

### platform_enable_user_pci

**Current value (from the default):** `false`

From //src/devices/bus/drivers/pci/pci.gni:10

### pmm_checker_from_board

Used to control whether board definitions include PMM checker options.

**Current value (from the default):** `true`

From //boards/kernel_cmdline/BUILD.gn:60

### policy_labels

Default policy_labels definition to ease with the transition away from
manually defining SWD policies across different product gni files.
The transition process will be as follows:
1. Define all SWD policies.
2. Add dependency on SWD policies in base_package_deps and
   system_image_deps, but do not apply a SWD policy to retain existing
   product behavior.
3. Apply SWD policies in product definitions alongside the existing
   SWD targets configuration. Assert that product configuration has not
   changed.
4. Remove manually defined SWD targets in product definitions, rely solely
   on defined SWD policies.
This is initially empty as part of the first step of the transition.

**Current value (from the default):** `{ }`

From //build/security/policies.gni:21

### pre_erase_flash

**Current value (from the default):** `false`

From //build/images/args.gni:120

### prebuilt_dart_sdk

Directory containing prebuilt Dart SDK.
This must have in its `bin/` subdirectory `gen_snapshot.OS-CPU` binaries.

**Current value (from the default):** `"//prebuilt/third_party/dart/linux-x64"`

From //build/dart/dart.gni:8

### prebuilt_fastboot

**Current value (from the default):** `"//prebuilt/third_party/fastboot/fastboot"`

From //build/images/tools/fastboot.gni:6

### prebuilt_go_dir

  prebuilt_go_dir
    [string] points to the directory containing the prebuilt host go
    binary. By default, this points to the //prebuilts directory.

**Current value (from the default):** `"//prebuilt/third_party/go/linux-x64"`

From //build/go/go_build.gni:27

### prebuilt_libvulkan_img_path

The path to a prebuilt libvulkan.so for an IMG GPU.

**Current value (from the default):** `""`

From //src/graphics/lib/magma/gnbuild/magma.gni:24

### product_bootfs_labels

A list of binary labels to include in ZBIs built for this product.

**Current value for `target_cpu = "arm64"`:** `["//bundles:bootstrap", "//bundles:debugging", "//bundles/bringup:manual_testing", "//bundles/drivers:bootstrap", "//bundles/drivers:bootstrap-eng", "//bundles/drivers:usb-host-stack", "//bundles/drivers:utils", "//products/kernel_cmdline:kernel.oom.behavior--jobkill", "//products/kernel_cmdline:oom.reboot-timeout--low", "//bundles/drivers:usb-peripheral-stack", "//src/sys/component_manager:component_manager_bootfs_config", "//bundles:diagnostics-eng"]`

From //products/bringup.gni:14

**Overridden from the default:** `[]`

From //build/product.gni:11

**Current value for `target_cpu = "x64"`:** `["//bundles:bootstrap", "//bundles:debugging", "//bundles/bringup:manual_testing", "//bundles/drivers:bootstrap", "//bundles/drivers:bootstrap-eng", "//bundles/drivers:usb-host-stack", "//bundles/drivers:utils", "//products/kernel_cmdline:kernel.oom.behavior--jobkill", "//products/kernel_cmdline:oom.reboot-timeout--low", "//bundles/drivers:usb-peripheral-stack", "//src/sys/component_manager:component_manager_bootfs_config", "//bundles:diagnostics-eng"]`

From //products/bringup.gni:14

**Overridden from the default:** `[]`

From //build/product.gni:11

### product_bootfs_packages

A list of packages to be included in the bootfs as
meta.fars and content-id'd blobs.

**Current value (from the default):** `[]`

From //build/product.gni:21

### product_description

A human readable product description.

**Current value (from the default):** `""`

From //build/product.gni:24

### product_host_labels

A list of binary host tool labels to also build.

**Current value (from the default):** `[]`

From //build/product.gni:17

### product_system_image_deps

A list of binary labels to include in the system_image package.

**Current value (from the default):** `[]`

From //build/product.gni:14

### profile_source_files

List of GN paths to source files to be instrumented by `profile` variants.

**Current value (from the default):** `["//*"]`

From //build/config/profile/config.gni:7

### profile_source_files_list_files

List GN path to files in Clang's `-fprofile-list` format describing files
and functions to be instrumented by `profile` variants.

**Current value (from the default):** `[]`

From //build/config/profile/config.gni:24

### proprietary_codecs

**Current value (from the default):** `false`

From //build/config/features.gni:9

### pw_JAVA_NATIVE_INTERFACE_INCLUDE_DIRS

pw_JAVA_NATIVE_INTERFACE_INCLUDE_DIRS specifies the paths to use for
building Java Native Interface libraries. If no paths are provided, targets
that require JNI may not build correctly.

Example JNI include paths for a Linux system:

  pw_JAVA_NATIVE_INTERFACE_INCLUDE_DIRS = [
    "/usr/local/buildtools/java/jdk/include/",
    "/usr/local/buildtools/java/jdk/include/linux",
  ]


**Current value (from the default):** `[]`

From //third_party/pigweed/src/pw_tokenizer/BUILD.gn:304

### pw_arduino_build_BOARD

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_arduino_build/arduino.gni:30

### pw_arduino_build_CORE_NAME

Expected args for an Arduino build:

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_arduino_build/arduino.gni:24

### pw_arduino_build_CORE_PATH

Enable/disable Arduino builds via group("arduino").
Set to the full path of where cores are installed.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_arduino_build/arduino.gni:21

### pw_arduino_build_MENU_OPTIONS

Menu options should be a list of strings.

**Current value (from the default):** `[]`

From //third_party/pigweed/src/pw_arduino_build/arduino.gni:33

### pw_arduino_build_PACKAGE_NAME

TODO(tonymd): "teensy/avr" here should match the folders in this dir:
"../third_party/arduino/cores/$pw_arduino_build_CORE_NAME/hardware/*")
For teensy: "teensy/avr", for adafruit-samd: "samd/1.6.2"

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_arduino_build/arduino.gni:29

### pw_assert_BACKEND

Backend for the pw_assert module's CHECK facade.

**Current value for `target_cpu = "arm64"`:** `"//third_party/pigweed/backends/pw_assert"`

From //.gn:64

**Overridden from the default:** `""`

From //third_party/pigweed/src/pw_assert/backend.gni:19

**Current value for `target_cpu = "x64"`:** `"//third_party/pigweed/backends/pw_assert"`

From //.gn:64

**Overridden from the default:** `""`

From //third_party/pigweed/src/pw_assert/backend.gni:19

### pw_assert_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_assert/BUILD.gn:27

### pw_assert_LITE_BACKEND

Backend for the pw_assert module's ASSERT facade.

Warning: This naming is transitional. Modifying this build argument WILL
    result in future breakages. (b/235149326)

**Current value (from the default):** `"//third_party/pigweed/src/pw_assert:assert_compatibility_backend"`

From //third_party/pigweed/src/pw_assert/backend.gni:25

### pw_bloat_BLOATY_CONFIG

Path to the Bloaty configuration file that defines the memory layout and
capacities for the target binaries.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_bloat/bloat.gni:23

### pw_bloat_SHOW_SIZE_REPORTS

Controls whether to display size reports in the build output.

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_bloat/bloat.gni:40

### pw_bloat_TOOLCHAINS

List of toolchains to use in pw_toolchain_size_diff templates.

Each entry is a scope containing the following variables:

  name: Human-readable toolchain name.
  target: GN target that defines the toolchain.
  linker_script: Optional path to a linker script file to build for the
    toolchain's target.
  bloaty_config: Optional Bloaty confirugation file defining the memory
    layout of the binaries as specified in the linker script.

If this list is empty, pw_toolchain_size_diff targets become no-ops.

**Current value (from the default):** `[]`

From //third_party/pigweed/src/pw_bloat/bloat.gni:37

### pw_build_DEFAULT_MODULE_CONFIG

The default implementation for all Pigweed module configurations.

This variable makes it possible to configure multiple Pigweed modules from
a single GN target. Module configurations can still be overridden
individually by setting a module's config backend directly (e.g.
pw_some_module_CONFIG = "//my_config").

Modules are configured through compilation options. The configuration
implementation might set individual compiler macros or forcibly include a
config header with multiple options using the -include flag.

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_build/module_config.gni:28

### pw_build_DEFAULT_VISIBILITY

Controls the default visibility of C/C++ libraries and executables
(pw_source_set, pw_static_library, pw_shared_library pw_executable). This
can be "*" or a list of paths.

This is useful for limiting usage of Pigweed modules via an explicit
allowlist. For the GN build to work, pw_build_DEFAULT_VISIBILITY must always
at least include the Pigweed repository ("$dir_pigweed/*").

Explicitly setting a target's visibility overrides this default.

**Current value for `target_cpu = "arm64"`:** `["//third_party/pigweed/*"]`

From //.gn:61

**Overridden from the default:** `"*"`

From //third_party/pigweed/src/pw_build/defaults.gni:38

**Current value for `target_cpu = "x64"`:** `["//third_party/pigweed/*"]`

From //.gn:61

**Overridden from the default:** `"*"`

From //third_party/pigweed/src/pw_build/defaults.gni:38

### pw_build_EXECUTABLE_TARGET_TYPE

The name of the GN target type used to build Pigweed executables.

If this is a custom template, the .gni file containing the template must
be imported at the top of the target configuration file to make it globally
available.

**Current value (from the default):** `"executable"`

From //third_party/pigweed/src/pw_build/gn_internal/build_target.gni:31

### pw_build_EXECUTABLE_TARGET_TYPE_FILE

The path to the .gni file that defines pw_build_EXECUTABLE_TARGET_TYPE.

If pw_build_EXECUTABLE_TARGET_TYPE is not the default of `executable`, this
.gni file is imported to provide the template definition.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_build/gn_internal/build_target.gni:37

### pw_build_LINK_DEPS

Additional build targets to add as dependencies for pw_executable,
pw_static_library, and pw_shared_library targets. The
$dir_pw_build:link_deps target pulls in these libraries.

pw_build_LINK_DEPS can be used to break circular dependencies for low-level
libraries such as pw_assert.

**Current value for `target_cpu = "arm64"`:** `["//third_party/pigweed/src/pw_assert:impl", "//third_party/pigweed/src/pw_log:impl"]`

From //.gn:74

**Overridden from the default:** `[]`

From //third_party/pigweed/src/pw_build/gn_internal/build_target.gni:24

**Current value for `target_cpu = "x64"`:** `["//third_party/pigweed/src/pw_assert:impl", "//third_party/pigweed/src/pw_log:impl"]`

From //.gn:74

**Overridden from the default:** `[]`

From //third_party/pigweed/src/pw_build/gn_internal/build_target.gni:24

### pw_build_PIP_CONSTRAINTS

**Current value (from the default):** `["//third_party/pigweed/src/pw_env_setup/py/pw_env_setup/virtualenv_setup/constraint.list"]`

From //third_party/pigweed/src/pw_build/python.gni:27

### pw_build_PIP_REQUIREMENTS

Default pip requirements file for all Pigweed based projects.

**Current value (from the default):** `[]`

From //third_party/pigweed/src/pw_build/python.gni:30

### pw_build_PYTHON_BUILD_VENV

Default gn build virtualenv target.

**Current value (from the default):** `"//third_party/pigweed/src/pw_env_setup:pigweed_build_venv"`

From //third_party/pigweed/src/pw_build/python_gn_args.gni:23

### pw_build_PYTHON_TEST_COVERAGE

If true, GN will run each Python test using the coverage command. A separate
coverage data file for each test will be saved. To generate reports from
this information run: pw presubmit --step gn_python_test_coverage

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_build/python.gni:35

### pw_build_PYTHON_TOOLCHAIN

Python tasks, such as running tests and Pylint, are done in a single GN
toolchain to avoid unnecessary duplication in the build.

**Current value (from the default):** `"//third_party/pigweed/src/pw_build/python_toolchain:python"`

From //third_party/pigweed/src/pw_build/python_gn_args.gni:20

### pw_checksum_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_checksum/BUILD.gn:28

### pw_chrono_SYSTEM_CLOCK_BACKEND

Backend for the pw_chrono module's system_clock.

**Current value for `target_cpu = "arm64"`:** `"//third_party/pigweed/src/pw_chrono_stl:system_clock"`

From //.gn:67

**Overridden from the default:** `""`

From //third_party/pigweed/src/pw_chrono/backend.gni:17

**Current value for `target_cpu = "x64"`:** `"//third_party/pigweed/src/pw_chrono_stl:system_clock"`

From //.gn:67

**Overridden from the default:** `""`

From //third_party/pigweed/src/pw_chrono/backend.gni:17

### pw_chrono_SYSTEM_TIMER_BACKEND

Backend for the pw_chrono module's system_timer.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_chrono/backend.gni:20

### pw_command_launcher

Prefix for compilation commands (e.g. the path to a Goma or CCache compiler
launcher). Example for ccache:
  gn gen out --args='pw_command_launcher="ccache"'

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_toolchain/generate_toolchain.gni:29

### pw_compilation_testing_NEGATIVE_COMPILATION_ENABLED

Enables or disables negative compilation tests for the current toolchain.
Disabled by default since negative compilation tests increase gn gen time
significantly.

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_compilation_testing/negative_compilation_test.gni:24

### pw_cpu_exception_ENTRY_BACKEND

Backends for the pw_cpu_exception module.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_cpu_exception/backend.gni:17

### pw_cpu_exception_HANDLER_BACKEND

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_cpu_exception/backend.gni:18

### pw_cpu_exception_SUPPORT_BACKEND

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_cpu_exception/backend.gni:19

### pw_cpu_exception_cortex_m_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_cpu_exception_cortex_m/BUILD.gn:28

### pw_crypto_ECDSA_BACKEND

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_crypto/backend.gni:17

### pw_crypto_SHA256_BACKEND

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_crypto/backend.gni:16

### pw_docgen_BUILD_DOCS

Whether or not the current target should build docs.

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_docgen/docs.gni:22

### pw_docs_google_analytics_id

Set to enable Google Analytics tracking of generated docs.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_docgen/docs.gni:25

### pw_env_setup_CIPD_BAZEL

**Current value (from the default):** `"../../prebuilt/third_party/bazel/linux-x64/bin"`

From //build_overrides/pigweed_environment.gni:24

### pw_env_setup_CIPD_DEFAULT

**Current value (from the default):** `"//prebuilt/third_party"`

From //build_overrides/pigweed_environment.gni:18

### pw_env_setup_CIPD_PIGWEED

**Current value (from the default):** `"//prebuilt/third_party"`

From //build_overrides/pigweed_environment.gni:19

### pw_env_setup_CIPD_PYTHON

**Current value (from the default):** `"../../prebuilt/third_party/python/linux-x64/bin"`

From //build_overrides/pigweed_environment.gni:21

### pw_function_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value for `target_cpu = "arm64"`:** `"//third_party/pigweed/backends/pw_function:define_overrides"`

From //.gn:71

**Overridden from the default:** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_function/BUILD.gn:27

**Current value for `target_cpu = "x64"`:** `"//third_party/pigweed/backends/pw_function:define_overrides"`

From //.gn:71

**Overridden from the default:** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_function/BUILD.gn:27

### pw_log_BACKEND

Backend for the pw_log module.

**Current value for `target_cpu = "arm64"`:** `"//third_party/pigweed/backends/pw_log/dfv1"`

From //.gn:65

**Overridden from the default:** `""`

From //third_party/pigweed/src/pw_log/backend.gni:17

**Current value for `target_cpu = "x64"`:** `"//third_party/pigweed/backends/pw_log/dfv1"`

From //.gn:65

**Overridden from the default:** `""`

From //third_party/pigweed/src/pw_log/backend.gni:17

### pw_log_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_log/BUILD.gn:29

### pw_log_GLOG_ADAPTER_CONFIG

The build target that overrides the default configuration options for the
glog adapter portion of this module.

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_log/BUILD.gn:33

### pw_log_tokenized_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_log_tokenized/BUILD.gn:29

### pw_log_tokenized_HANDLER_BACKEND

Backend for the pw_log_tokenized log handler.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_log_tokenized/backend.gni:17

### pw_module_docs

A list with all Pigweed modules docs groups. DO NOT SET THIS BUILD ARGUMENT!

**Current value (from the default):** `["//third_party/pigweed/src/docker:docs", "//third_party/pigweed/src/pw_alignment:docs", "//third_party/pigweed/src/pw_allocator:docs", "//third_party/pigweed/src/pw_analog:docs", "//third_party/pigweed/src/pw_android_toolchain:docs", "//third_party/pigweed/src/pw_arduino_build:docs", "//third_party/pigweed/src/pw_assert:docs", "//third_party/pigweed/src/pw_assert_basic:docs", "//third_party/pigweed/src/pw_assert_log:docs", "//third_party/pigweed/src/pw_assert_tokenized:docs", "//third_party/pigweed/src/pw_assert_zephyr:docs", "//third_party/pigweed/src/pw_async:docs", "//third_party/pigweed/src/pw_async_basic:docs", "//third_party/pigweed/src/pw_base64:docs", "//third_party/pigweed/src/pw_bloat:docs", "//third_party/pigweed/src/pw_blob_store:docs", "//third_party/pigweed/src/pw_bluetooth:docs", "//third_party/pigweed/src/pw_bluetooth_hci:docs", "//third_party/pigweed/src/pw_bluetooth_profiles:docs", "//third_party/pigweed/src/pw_boot:docs", "//third_party/pigweed/src/pw_boot_cortex_m:docs", "//third_party/pigweed/src/pw_build:docs", "//third_party/pigweed/src/pw_build_info:docs", "//third_party/pigweed/src/pw_build_mcuxpresso:docs", "//third_party/pigweed/src/pw_bytes:docs", "//third_party/pigweed/src/pw_checksum:docs", "//third_party/pigweed/src/pw_chrono:docs", "//third_party/pigweed/src/pw_chrono_embos:docs", "//third_party/pigweed/src/pw_chrono_freertos:docs", "//third_party/pigweed/src/pw_chrono_stl:docs", "//third_party/pigweed/src/pw_chrono_threadx:docs", "//third_party/pigweed/src/pw_chrono_zephyr:docs", "//third_party/pigweed/src/pw_cli:docs", "//third_party/pigweed/src/pw_compilation_testing:docs", "//third_party/pigweed/src/pw_console:docs", "//third_party/pigweed/src/pw_containers:docs", "//third_party/pigweed/src/pw_cpu_exception:docs", "//third_party/pigweed/src/pw_cpu_exception_cortex_m:docs", "//third_party/pigweed/src/pw_crypto:docs", "//third_party/pigweed/src/pw_digital_io:docs", "//third_party/pigweed/src/pw_docgen:docs", "//third_party/pigweed/src/pw_doctor:docs", "//third_party/pigweed/src/pw_env_setup:docs", "//third_party/pigweed/src/pw_file:docs", "//third_party/pigweed/src/pw_function:docs", "//third_party/pigweed/src/pw_fuzzer:docs", "//third_party/pigweed/src/pw_hdlc:docs", "//third_party/pigweed/src/pw_hex_dump:docs", "//third_party/pigweed/src/pw_i2c:docs", "//third_party/pigweed/src/pw_i2c_mcuxpresso:docs", "//third_party/pigweed/src/pw_ide:docs", "//third_party/pigweed/src/pw_interrupt:docs", "//third_party/pigweed/src/pw_interrupt_cortex_m:docs", "//third_party/pigweed/src/pw_interrupt_zephyr:docs", "//third_party/pigweed/src/pw_intrusive_ptr:docs", "//third_party/pigweed/src/pw_kvs:docs", "//third_party/pigweed/src/pw_libc:docs", "//third_party/pigweed/src/pw_log:docs", "//third_party/pigweed/src/pw_log_android:docs", "//third_party/pigweed/src/pw_log_basic:docs", "//third_party/pigweed/src/pw_log_null:docs", "//third_party/pigweed/src/pw_log_rpc:docs", "//third_party/pigweed/src/pw_log_string:docs", "//third_party/pigweed/src/pw_log_tokenized:docs", "//third_party/pigweed/src/pw_log_zephyr:docs", "//third_party/pigweed/src/pw_malloc:docs", "//third_party/pigweed/src/pw_malloc_freelist:docs", "//third_party/pigweed/src/pw_metric:docs", "//third_party/pigweed/src/pw_minimal_cpp_stdlib:docs", "//third_party/pigweed/src/pw_module:docs", "//third_party/pigweed/src/pw_multisink:docs", "//third_party/pigweed/src/pw_package:docs", "//third_party/pigweed/src/pw_perf_test:docs", "//third_party/pigweed/src/pw_persistent_ram:docs", "//third_party/pigweed/src/pw_polyfill:docs", "//third_party/pigweed/src/pw_preprocessor:docs", "//third_party/pigweed/src/pw_presubmit:docs", "//third_party/pigweed/src/pw_protobuf:docs", "//third_party/pigweed/src/pw_protobuf_compiler:docs", "//third_party/pigweed/src/pw_random:docs", "//third_party/pigweed/src/pw_result:docs", "//third_party/pigweed/src/pw_ring_buffer:docs", "//third_party/pigweed/src/pw_router:docs", "//third_party/pigweed/src/pw_rpc:docs", "//third_party/pigweed/src/pw_rust:docs", "//third_party/pigweed/src/pw_snapshot:docs", "//third_party/pigweed/src/pw_software_update:docs", "//third_party/pigweed/src/pw_span:docs", "//third_party/pigweed/src/pw_spi:docs", "//third_party/pigweed/src/pw_status:docs", "//third_party/pigweed/src/pw_stm32cube_build:docs", "//third_party/pigweed/src/pw_stream:docs", "//third_party/pigweed/src/pw_string:docs", "//third_party/pigweed/src/pw_symbolizer:docs", "//third_party/pigweed/src/pw_sync:docs", "//third_party/pigweed/src/pw_sync_baremetal:docs", "//third_party/pigweed/src/pw_sync_embos:docs", "//third_party/pigweed/src/pw_sync_freertos:docs", "//third_party/pigweed/src/pw_sync_stl:docs", "//third_party/pigweed/src/pw_sync_threadx:docs", "//third_party/pigweed/src/pw_sync_zephyr:docs", "//third_party/pigweed/src/pw_sys_io:docs", "//third_party/pigweed/src/pw_sys_io_arduino:docs", "//third_party/pigweed/src/pw_sys_io_baremetal_lm3s6965evb:docs", "//third_party/pigweed/src/pw_sys_io_baremetal_stm32f429:docs", "//third_party/pigweed/src/pw_sys_io_emcraft_sf2:docs", "//third_party/pigweed/src/pw_sys_io_mcuxpresso:docs", "//third_party/pigweed/src/pw_sys_io_pico:docs", "//third_party/pigweed/src/pw_sys_io_stdio:docs", "//third_party/pigweed/src/pw_sys_io_stm32cube:docs", "//third_party/pigweed/src/pw_sys_io_zephyr:docs", "//third_party/pigweed/src/pw_system:docs", "//third_party/pigweed/src/pw_target_runner:docs", "//third_party/pigweed/src/pw_thread:docs", "//third_party/pigweed/src/pw_thread_embos:docs", "//third_party/pigweed/src/pw_thread_freertos:docs", "//third_party/pigweed/src/pw_thread_stl:docs", "//third_party/pigweed/src/pw_thread_threadx:docs", "//third_party/pigweed/src/pw_thread_zephyr:docs", "//third_party/pigweed/src/pw_tls_client:docs", "//third_party/pigweed/src/pw_tls_client_boringssl:docs", "//third_party/pigweed/src/pw_tls_client_mbedtls:docs", "//third_party/pigweed/src/pw_tokenizer:docs", "//third_party/pigweed/src/pw_tool:docs", "//third_party/pigweed/src/pw_toolchain:docs", "//third_party/pigweed/src/pw_trace:docs", "//third_party/pigweed/src/pw_trace_tokenized:docs", "//third_party/pigweed/src/pw_transfer:docs", "//third_party/pigweed/src/pw_unit_test:docs", "//third_party/pigweed/src/pw_varint:docs", "//third_party/pigweed/src/pw_watch:docs", "//third_party/pigweed/src/pw_web:docs", "//third_party/pigweed/src/pw_work_queue:docs"]`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:453

### pw_module_tests

A list with all Pigweed module test groups. DO NOT SET THIS BUILD ARGUMENT!

**Current value (from the default):** `["//third_party/pigweed/src/docker:tests", "//third_party/pigweed/src/pw_alignment:tests", "//third_party/pigweed/src/pw_allocator:tests", "//third_party/pigweed/src/pw_analog:tests", "//third_party/pigweed/src/pw_android_toolchain:tests", "//third_party/pigweed/src/pw_arduino_build:tests", "//third_party/pigweed/src/pw_assert:tests", "//third_party/pigweed/src/pw_assert_basic:tests", "//third_party/pigweed/src/pw_assert_log:tests", "//third_party/pigweed/src/pw_assert_tokenized:tests", "//third_party/pigweed/src/pw_assert_zephyr:tests", "//third_party/pigweed/src/pw_async:tests", "//third_party/pigweed/src/pw_async_basic:tests", "//third_party/pigweed/src/pw_base64:tests", "//third_party/pigweed/src/pw_bloat:tests", "//third_party/pigweed/src/pw_blob_store:tests", "//third_party/pigweed/src/pw_bluetooth:tests", "//third_party/pigweed/src/pw_bluetooth_hci:tests", "//third_party/pigweed/src/pw_bluetooth_profiles:tests", "//third_party/pigweed/src/pw_boot:tests", "//third_party/pigweed/src/pw_boot_cortex_m:tests", "//third_party/pigweed/src/pw_build:tests", "//third_party/pigweed/src/pw_build_info:tests", "//third_party/pigweed/src/pw_build_mcuxpresso:tests", "//third_party/pigweed/src/pw_bytes:tests", "//third_party/pigweed/src/pw_checksum:tests", "//third_party/pigweed/src/pw_chrono:tests", "//third_party/pigweed/src/pw_chrono_embos:tests", "//third_party/pigweed/src/pw_chrono_freertos:tests", "//third_party/pigweed/src/pw_chrono_stl:tests", "//third_party/pigweed/src/pw_chrono_threadx:tests", "//third_party/pigweed/src/pw_chrono_zephyr:tests", "//third_party/pigweed/src/pw_cli:tests", "//third_party/pigweed/src/pw_compilation_testing:tests", "//third_party/pigweed/src/pw_console:tests", "//third_party/pigweed/src/pw_containers:tests", "//third_party/pigweed/src/pw_cpu_exception:tests", "//third_party/pigweed/src/pw_cpu_exception_cortex_m:tests", "//third_party/pigweed/src/pw_crypto:tests", "//third_party/pigweed/src/pw_digital_io:tests", "//third_party/pigweed/src/pw_docgen:tests", "//third_party/pigweed/src/pw_doctor:tests", "//third_party/pigweed/src/pw_env_setup:tests", "//third_party/pigweed/src/pw_file:tests", "//third_party/pigweed/src/pw_function:tests", "//third_party/pigweed/src/pw_fuzzer:tests", "//third_party/pigweed/src/pw_hdlc:tests", "//third_party/pigweed/src/pw_hex_dump:tests", "//third_party/pigweed/src/pw_i2c:tests", "//third_party/pigweed/src/pw_i2c_mcuxpresso:tests", "//third_party/pigweed/src/pw_ide:tests", "//third_party/pigweed/src/pw_interrupt:tests", "//third_party/pigweed/src/pw_interrupt_cortex_m:tests", "//third_party/pigweed/src/pw_interrupt_zephyr:tests", "//third_party/pigweed/src/pw_intrusive_ptr:tests", "//third_party/pigweed/src/pw_kvs:tests", "//third_party/pigweed/src/pw_libc:tests", "//third_party/pigweed/src/pw_log:tests", "//third_party/pigweed/src/pw_log_android:tests", "//third_party/pigweed/src/pw_log_basic:tests", "//third_party/pigweed/src/pw_log_null:tests", "//third_party/pigweed/src/pw_log_rpc:tests", "//third_party/pigweed/src/pw_log_string:tests", "//third_party/pigweed/src/pw_log_tokenized:tests", "//third_party/pigweed/src/pw_log_zephyr:tests", "//third_party/pigweed/src/pw_malloc:tests", "//third_party/pigweed/src/pw_malloc_freelist:tests", "//third_party/pigweed/src/pw_metric:tests", "//third_party/pigweed/src/pw_minimal_cpp_stdlib:tests", "//third_party/pigweed/src/pw_module:tests", "//third_party/pigweed/src/pw_multisink:tests", "//third_party/pigweed/src/pw_package:tests", "//third_party/pigweed/src/pw_perf_test:tests", "//third_party/pigweed/src/pw_persistent_ram:tests", "//third_party/pigweed/src/pw_polyfill:tests", "//third_party/pigweed/src/pw_preprocessor:tests", "//third_party/pigweed/src/pw_presubmit:tests", "//third_party/pigweed/src/pw_protobuf:tests", "//third_party/pigweed/src/pw_protobuf_compiler:tests", "//third_party/pigweed/src/pw_random:tests", "//third_party/pigweed/src/pw_result:tests", "//third_party/pigweed/src/pw_ring_buffer:tests", "//third_party/pigweed/src/pw_router:tests", "//third_party/pigweed/src/pw_rpc:tests", "//third_party/pigweed/src/pw_rust:tests", "//third_party/pigweed/src/pw_snapshot:tests", "//third_party/pigweed/src/pw_software_update:tests", "//third_party/pigweed/src/pw_span:tests", "//third_party/pigweed/src/pw_spi:tests", "//third_party/pigweed/src/pw_status:tests", "//third_party/pigweed/src/pw_stm32cube_build:tests", "//third_party/pigweed/src/pw_stream:tests", "//third_party/pigweed/src/pw_string:tests", "//third_party/pigweed/src/pw_symbolizer:tests", "//third_party/pigweed/src/pw_sync:tests", "//third_party/pigweed/src/pw_sync_baremetal:tests", "//third_party/pigweed/src/pw_sync_embos:tests", "//third_party/pigweed/src/pw_sync_freertos:tests", "//third_party/pigweed/src/pw_sync_stl:tests", "//third_party/pigweed/src/pw_sync_threadx:tests", "//third_party/pigweed/src/pw_sync_zephyr:tests", "//third_party/pigweed/src/pw_sys_io:tests", "//third_party/pigweed/src/pw_sys_io_arduino:tests", "//third_party/pigweed/src/pw_sys_io_baremetal_lm3s6965evb:tests", "//third_party/pigweed/src/pw_sys_io_baremetal_stm32f429:tests", "//third_party/pigweed/src/pw_sys_io_emcraft_sf2:tests", "//third_party/pigweed/src/pw_sys_io_mcuxpresso:tests", "//third_party/pigweed/src/pw_sys_io_pico:tests", "//third_party/pigweed/src/pw_sys_io_stdio:tests", "//third_party/pigweed/src/pw_sys_io_stm32cube:tests", "//third_party/pigweed/src/pw_sys_io_zephyr:tests", "//third_party/pigweed/src/pw_system:tests", "//third_party/pigweed/src/pw_target_runner:tests", "//third_party/pigweed/src/pw_thread:tests", "//third_party/pigweed/src/pw_thread_embos:tests", "//third_party/pigweed/src/pw_thread_freertos:tests", "//third_party/pigweed/src/pw_thread_stl:tests", "//third_party/pigweed/src/pw_thread_threadx:tests", "//third_party/pigweed/src/pw_thread_zephyr:tests", "//third_party/pigweed/src/pw_tls_client:tests", "//third_party/pigweed/src/pw_tls_client_boringssl:tests", "//third_party/pigweed/src/pw_tls_client_mbedtls:tests", "//third_party/pigweed/src/pw_tokenizer:tests", "//third_party/pigweed/src/pw_tool:tests", "//third_party/pigweed/src/pw_toolchain:tests", "//third_party/pigweed/src/pw_trace:tests", "//third_party/pigweed/src/pw_trace_tokenized:tests", "//third_party/pigweed/src/pw_transfer:tests", "//third_party/pigweed/src/pw_unit_test:tests", "//third_party/pigweed/src/pw_varint:tests", "//third_party/pigweed/src/pw_watch:tests", "//third_party/pigweed/src/pw_web:tests", "//third_party/pigweed/src/pw_work_queue:tests"]`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:316

### pw_modules

A list with paths to all Pigweed module. DO NOT SET THIS BUILD ARGUMENT!

**Current value (from the default):** `["//third_party/pigweed/src/docker", "//third_party/pigweed/src/pw_alignment", "//third_party/pigweed/src/pw_allocator", "//third_party/pigweed/src/pw_analog", "//third_party/pigweed/src/pw_android_toolchain", "//third_party/pigweed/src/pw_arduino_build", "//third_party/pigweed/src/pw_assert", "//third_party/pigweed/src/pw_assert_basic", "//third_party/pigweed/src/pw_assert_log", "//third_party/pigweed/src/pw_assert_tokenized", "//third_party/pigweed/src/pw_assert_zephyr", "//third_party/pigweed/src/pw_async", "//third_party/pigweed/src/pw_async_basic", "//third_party/pigweed/src/pw_base64", "//third_party/pigweed/src/pw_bloat", "//third_party/pigweed/src/pw_blob_store", "//third_party/pigweed/src/pw_bluetooth", "//third_party/pigweed/src/pw_bluetooth_hci", "//third_party/pigweed/src/pw_bluetooth_profiles", "//third_party/pigweed/src/pw_boot", "//third_party/pigweed/src/pw_boot_cortex_m", "//third_party/pigweed/src/pw_build", "//third_party/pigweed/src/pw_build_info", "//third_party/pigweed/src/pw_build_mcuxpresso", "//third_party/pigweed/src/pw_bytes", "//third_party/pigweed/src/pw_checksum", "//third_party/pigweed/src/pw_chrono", "//third_party/pigweed/src/pw_chrono_embos", "//third_party/pigweed/src/pw_chrono_freertos", "//third_party/pigweed/src/pw_chrono_stl", "//third_party/pigweed/src/pw_chrono_threadx", "//third_party/pigweed/src/pw_chrono_zephyr", "//third_party/pigweed/src/pw_cli", "//third_party/pigweed/src/pw_compilation_testing", "//third_party/pigweed/src/pw_console", "//third_party/pigweed/src/pw_containers", "//third_party/pigweed/src/pw_cpu_exception", "//third_party/pigweed/src/pw_cpu_exception_cortex_m", "//third_party/pigweed/src/pw_crypto", "//third_party/pigweed/src/pw_digital_io", "//third_party/pigweed/src/pw_docgen", "//third_party/pigweed/src/pw_doctor", "//third_party/pigweed/src/pw_env_setup", "//third_party/pigweed/src/pw_file", "//third_party/pigweed/src/pw_function", "//third_party/pigweed/src/pw_fuzzer", "//third_party/pigweed/src/pw_hdlc", "//third_party/pigweed/src/pw_hex_dump", "//third_party/pigweed/src/pw_i2c", "//third_party/pigweed/src/pw_i2c_mcuxpresso", "//third_party/pigweed/src/pw_ide", "//third_party/pigweed/src/pw_interrupt", "//third_party/pigweed/src/pw_interrupt_cortex_m", "//third_party/pigweed/src/pw_interrupt_zephyr", "//third_party/pigweed/src/pw_intrusive_ptr", "//third_party/pigweed/src/pw_kvs", "//third_party/pigweed/src/pw_libc", "//third_party/pigweed/src/pw_log", "//third_party/pigweed/src/pw_log_android", "//third_party/pigweed/src/pw_log_basic", "//third_party/pigweed/src/pw_log_null", "//third_party/pigweed/src/pw_log_rpc", "//third_party/pigweed/src/pw_log_string", "//third_party/pigweed/src/pw_log_tokenized", "//third_party/pigweed/src/pw_log_zephyr", "//third_party/pigweed/src/pw_malloc", "//third_party/pigweed/src/pw_malloc_freelist", "//third_party/pigweed/src/pw_metric", "//third_party/pigweed/src/pw_minimal_cpp_stdlib", "//third_party/pigweed/src/pw_module", "//third_party/pigweed/src/pw_multisink", "//third_party/pigweed/src/pw_package", "//third_party/pigweed/src/pw_perf_test", "//third_party/pigweed/src/pw_persistent_ram", "//third_party/pigweed/src/pw_polyfill", "//third_party/pigweed/src/pw_preprocessor", "//third_party/pigweed/src/pw_presubmit", "//third_party/pigweed/src/pw_protobuf", "//third_party/pigweed/src/pw_protobuf_compiler", "//third_party/pigweed/src/pw_random", "//third_party/pigweed/src/pw_result", "//third_party/pigweed/src/pw_ring_buffer", "//third_party/pigweed/src/pw_router", "//third_party/pigweed/src/pw_rpc", "//third_party/pigweed/src/pw_rust", "//third_party/pigweed/src/pw_snapshot", "//third_party/pigweed/src/pw_software_update", "//third_party/pigweed/src/pw_span", "//third_party/pigweed/src/pw_spi", "//third_party/pigweed/src/pw_status", "//third_party/pigweed/src/pw_stm32cube_build", "//third_party/pigweed/src/pw_stream", "//third_party/pigweed/src/pw_string", "//third_party/pigweed/src/pw_symbolizer", "//third_party/pigweed/src/pw_sync", "//third_party/pigweed/src/pw_sync_baremetal", "//third_party/pigweed/src/pw_sync_embos", "//third_party/pigweed/src/pw_sync_freertos", "//third_party/pigweed/src/pw_sync_stl", "//third_party/pigweed/src/pw_sync_threadx", "//third_party/pigweed/src/pw_sync_zephyr", "//third_party/pigweed/src/pw_sys_io", "//third_party/pigweed/src/pw_sys_io_arduino", "//third_party/pigweed/src/pw_sys_io_baremetal_lm3s6965evb", "//third_party/pigweed/src/pw_sys_io_baremetal_stm32f429", "//third_party/pigweed/src/pw_sys_io_emcraft_sf2", "//third_party/pigweed/src/pw_sys_io_mcuxpresso", "//third_party/pigweed/src/pw_sys_io_pico", "//third_party/pigweed/src/pw_sys_io_stdio", "//third_party/pigweed/src/pw_sys_io_stm32cube", "//third_party/pigweed/src/pw_sys_io_zephyr", "//third_party/pigweed/src/pw_system", "//third_party/pigweed/src/pw_target_runner", "//third_party/pigweed/src/pw_thread", "//third_party/pigweed/src/pw_thread_embos", "//third_party/pigweed/src/pw_thread_freertos", "//third_party/pigweed/src/pw_thread_stl", "//third_party/pigweed/src/pw_thread_threadx", "//third_party/pigweed/src/pw_thread_zephyr", "//third_party/pigweed/src/pw_tls_client", "//third_party/pigweed/src/pw_tls_client_boringssl", "//third_party/pigweed/src/pw_tls_client_mbedtls", "//third_party/pigweed/src/pw_tokenizer", "//third_party/pigweed/src/pw_tool", "//third_party/pigweed/src/pw_toolchain", "//third_party/pigweed/src/pw_trace", "//third_party/pigweed/src/pw_trace_tokenized", "//third_party/pigweed/src/pw_transfer", "//third_party/pigweed/src/pw_unit_test", "//third_party/pigweed/src/pw_varint", "//third_party/pigweed/src/pw_watch", "//third_party/pigweed/src/pw_web", "//third_party/pigweed/src/pw_work_queue"]`

From //third_party/pigweed/src/pw_build/generated_pigweed_modules_lists.gni:179

### pw_perf_test_EXECUTABLE_TARGET_TYPE

Chooses the executable template for performance tests

**Current value (from the default):** `"pw_executable"`

From //third_party/pigweed/src/pw_perf_test/perf_test.gni:30

### pw_perf_test_MAIN_FUNCTION

Chooses the EventHandler for running the perf tests

**Current value (from the default):** `"//third_party/pigweed/src/pw_perf_test:log_perf_handler_main"`

From //third_party/pigweed/src/pw_perf_test/perf_test.gni:27

### pw_perf_test_TIMER_INTERFACE_BACKEND

Chooses the backend for how the framework calculates time

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_perf_test/perf_test.gni:24

### pw_preprocessor_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_preprocessor/BUILD.gn:26

### pw_protobuf_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_protobuf/BUILD.gn:31

### pw_protobuf_compiler_GENERATE_LEGACY_ENUM_SNAKE_CASE_NAMES

pwpb previously generated field enum names in SNAKE_CASE rather than
kConstantCase. Set this variable to temporarily enable legacy SNAKE_CASE
support while you migrate your codebase to kConstantCase.
b/266298474

**Current value (from the default):** `true`

From //third_party/pigweed/src/pw_protobuf_compiler/toolchain.gni:28

### pw_protobuf_compiler_TOOLCHAIN

**Current value (from the default):** `"//third_party/pigweed/src/pw_protobuf_compiler/toolchain:protocol_buffer"`

From //third_party/pigweed/src/pw_protobuf_compiler/toolchain.gni:22

### pw_rbe_arm_gcc_config

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_toolchain/rbe.gni:30

### pw_rbe_clang_config

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_toolchain/rbe.gni:29

### pw_rpc_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_rpc/config.gni:23

### pw_rpc_system_server_BACKEND

Backend for the pw_rpc_system_server module.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_rpc/system_server/backend.gni:17

### pw_software_update_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_software_update/BUILD.gn:30

### pw_span_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

Most modules depend on pw_build_DEFAULT_MODULE_CONFIG as the default config,
but since this module's config options require interaction with the build
system, this defaults to an internal config to properly support
pw_span_ENABLE_ASSERTS.

**Current value (from the default):** `"//third_party/pigweed/src/pw_span:span_asserts"`

From //third_party/pigweed/src/pw_span/BUILD.gn:38

### pw_span_ENABLE_ASSERTS

Whether or not to enable bounds-checking asserts in pw::span. Enabling this
may significantly increase binary size, and can introduce dependency cycles
if your pw_assert backend's headers depends directly or indirectly on
pw_span. It's recommended to enable this for debug builds if possible.

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_span/BUILD.gn:28

### pw_status_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_status/BUILD.gn:26

### pw_string_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_string/BUILD.gn:27

### pw_sync_BINARY_SEMAPHORE_BACKEND

Backend for the pw_sync module's binary semaphore.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_sync/backend.gni:17

### pw_sync_CONDITION_VARIABLE_BACKEND

Backend for the pw_sync module's condition variable.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_sync/backend.gni:20

### pw_sync_COUNTING_SEMAPHORE_BACKEND

Backend for the pw_sync module's counting semaphore.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_sync/backend.gni:23

### pw_sync_INTERRUPT_SPIN_LOCK_BACKEND

Backend for the pw_sync module's interrupt spin lock.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_sync/backend.gni:35

### pw_sync_MUTEX_BACKEND

Backend for the pw_sync module's mutex.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_sync/backend.gni:26

### pw_sync_OVERRIDE_SYSTEM_CLOCK_BACKEND_CHECK

Whether the GN asserts should be silenced in ensuring that a compatible
backend for pw_chrono_SYSTEM_CLOCK_BACKEND is chosen.
Set to true to disable the asserts.

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_sync/backend.gni:46

### pw_sync_RECURSIVE_MUTEX_BACKEND

Backend for the pw_sync module's recursive mutex.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_sync/backend.gni:32

### pw_sync_THREAD_NOTIFICATION_BACKEND

Backend for the pw_sync module's thread notification.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_sync/backend.gni:38

### pw_sync_TIMED_MUTEX_BACKEND

Backend for the pw_sync module's timed mutex.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_sync/backend.gni:29

### pw_sync_TIMED_THREAD_NOTIFICATION_BACKEND

Backend for the pw_sync module's timed thread notification.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_sync/backend.gni:41

### pw_sys_io_BACKEND

Backend for the pw_sys_io facade.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_sys_io/backend.gni:17

### pw_third_party_boringssl_ALIAS

Create a "$dir_pw_third_party/boringssl" target that aliases an existing
target. This can be used to fix a diamond dependency conflict if a
downstream project uses its own boringssl target and cannot be changed to
use Pigweed's boringssl exclusively.

**Current value for `target_cpu = "arm64"`:** `"//third_party/boringssl"`

From //.gn:80

**Overridden from the default:** `""`

From //third_party/pigweed/src/third_party/boringssl/boringssl.gni:25

**Current value for `target_cpu = "x64"`:** `"//third_party/boringssl"`

From //.gn:80

**Overridden from the default:** `""`

From //third_party/pigweed/src/third_party/boringssl/boringssl.gni:25

### pw_third_party_emboss_CONFIG

**Current value (from the default):** `"//third_party/pigweed/src/third_party/emboss:default_config"`

From //third_party/pigweed/src/third_party/emboss/emboss.gni:24

### pw_third_party_nanopb_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/third_party/nanopb/nanopb.gni:27

### pw_thread_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_thread/BUILD.gn:29

### pw_thread_ID_BACKEND

Backend for the pw_thread module's pw::thread::Id.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_thread/backend.gni:17

### pw_thread_OVERRIDE_SYSTEM_CLOCK_BACKEND_CHECK

Whether the GN asserts should be silenced in ensuring that a compatible
backend for pw_chrono_SYSTEM_CLOCK_BACKEND is chosen.
Set to true to disable the asserts.

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_thread/backend.gni:34

### pw_thread_SLEEP_BACKEND

Backend for the pw_thread module's pw::thread::sleep_{for,until}.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_thread/backend.gni:20

### pw_thread_TEST_THREAD_CONTEXT_BACKEND

Backend for the pw_thread module's pw::thread::test_thread_context.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_thread/backend.gni:29

### pw_thread_THREAD_BACKEND

Backend for the pw_thread module's pw::thread::Thread to create threads.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_thread/backend.gni:23

### pw_thread_THREAD_ITERATION_BACKEND

Backend for the pw_thread module's pw::thread::thread_iteration.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_thread/backend.gni:37

### pw_thread_YIELD_BACKEND

Backend for the pw_thread module's pw::thread::yield.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_thread/backend.gni:26

### pw_tokenizer_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_tokenizer/BUILD.gn:30

### pw_toolchain_CLANG_PREFIX

**Current value (from the default):** `"../../prebuilt/third_party/bin/"`

From //third_party/pigweed/src/pw_toolchain/clang_tools.gni:26

### pw_toolchain_COVERAGE_ENABLED

Indicates if this toolchain supports generating coverage reports from
pw_test targets.

For example, the static analysis toolchains that run `clang-tidy` instead
of the test binary itself cannot generate coverage reports.

This is typically set by individual toolchains and not by GN args.

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_toolchain/host_clang/toolchains.gni:31

### pw_toolchain_FUZZING_ENABLED

Indicates if this toolchain supports building fuzzers. This is typically
set by individual toolchains and not by GN args.

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_toolchain/host_clang/toolchains.gni:35

### pw_toolchain_OSS_FUZZ_ENABLED

Indicates if this build is a part of OSS-Fuzz, which needs to be able to
provide its own compiler and flags. This violates the build hermeticisim and
should only be used for OSS-Fuzz.

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_toolchain/host_clang/toolchains.gni:40

### pw_toolchain_RBE_DEBUG

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_toolchain/rbe.gni:26

### pw_toolchain_RUST_PREFIX

**Current value (from the default):** `"../../prebuilt/third_party/rust/bin/"`

From //third_party/pigweed/src/pw_toolchain/clang_tools.gni:33

### pw_toolchain_SANITIZERS

Sets the sanitizer to pass to clang. Valid values are "address", "memory",
"thread", "undefined", "undefined_heuristic".

**Current value (from the default):** `[]`

From //third_party/pigweed/src/pw_toolchain/host_clang/toolchains.gni:22

### pw_toolchain_SCOPE

Scope defining the current toolchain. Contains all of the arguments required
by the generate_toolchain template. This should NOT be manually modified.

**Current value (from the default):** `{ }`

From //third_party/pigweed/src/pw_toolchain/generate_toolchain.gni:23

### pw_toolchain_STATIC_ANALYSIS_SKIP_INCLUDE_PATHS

Disable clang-tidy for specific include paths. In the clang-tidy command,
include paths that end with one of these, or match as a regular expression,
are switched from -I to -isystem, which causes clang-tidy to ignore them.
Unfortunately, clang-tidy provides no other way to filter header files.

For example, the following ignores header files in "mbedtls/include":

  pw_toolchain_STATIC_ANALYSIS_SKIP_INCLUDE_PATHS = ["mbedtls/include"]

While the following ignores all third-party header files:

  pw_toolchain_STATIC_ANALYSIS_SKIP_INCLUDE_PATHS = [".*/third_party/.*"]


**Current value (from the default):** `[]`

From //third_party/pigweed/src/pw_toolchain/static_analysis_toolchain.gni:44

### pw_toolchain_STATIC_ANALYSIS_SKIP_SOURCES_RES

Regular expressions matching the paths of the source files to be excluded
from the analysis. clang-tidy provides no alternative option.

For example, the following disables clang-tidy on all source files in the
third_party directory:

  pw_toolchain_STATIC_ANALYSIS_SKIP_SOURCES_RES = ["third_party/.*"]


**Current value (from the default):** `[]`

From //third_party/pigweed/src/pw_toolchain/static_analysis_toolchain.gni:29

### pw_toolchain_USE_RBE

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_toolchain/rbe.gni:20

### pw_transfer_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_transfer/BUILD.gn:29

### pw_unit_test_AUTOMATIC_RUNNER

Path to a test runner to automatically run unit tests after they are built.

If set, a ``pw_test`` target's ``<target_name>.run`` action will invoke the
test runner specified by this argument, passing the path to the unit test to
run. If this is unset, the ``pw_test`` target's ``<target_name>.run`` step
will do nothing.

Targets that don't support parallelized execution of tests (e.g. a on-device
test runner that must flash a device and run the test in serial) should
set pw_unit_test_POOL_DEPTH to 1.

Type: string (name of an executable on the PATH, or path to an executable)
Usage: toolchain-controlled only

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_unit_test/test.gni:51

### pw_unit_test_AUTOMATIC_RUNNER_ARGS

Optional list of arguments to forward to the automatic runner.

Type: list of strings (args to pass to pw_unit_test_AUTOMATIC_RUNNER)
Usage: toolchain-controlled only

**Current value (from the default):** `[]`

From //third_party/pigweed/src/pw_unit_test/test.gni:57

### pw_unit_test_AUTOMATIC_RUNNER_TIMEOUT

Optional timeout to apply when running tests via the automatic runner.
Timeout is in seconds. Defaults to empty which means no timeout.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_unit_test/test.gni:61

### pw_unit_test_CONFIG

The build target that overrides the default configuration options for this
module. This should point to a source set that provides defines through a
public config (which may -include a file or add defines directly).

**Current value (from the default):** `"//third_party/pigweed/src/pw_build:empty"`

From //third_party/pigweed/src/pw_unit_test/BUILD.gn:28

### pw_unit_test_EXECUTABLE_TARGET_TYPE

The name of the GN target type used to build pw_unit_test executables.

Type: string (name of a GN template)
Usage: toolchain-controlled only

**Current value (from the default):** `"pw_executable"`

From //third_party/pigweed/src/pw_unit_test/test.gni:90

### pw_unit_test_EXECUTABLE_TARGET_TYPE_FILE

The path to the .gni file that defines pw_unit_test_EXECUTABLE_TARGET_TYPE.

If pw_unit_test_EXECUTABLE_TARGET_TYPE is not the default of
`pw_executable`, this .gni file is imported to provide the template
definition.

Type: string (path to a .gni file)
Usage: toolchain-controlled only

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_unit_test/test.gni:100

### pw_unit_test_FACADE_TESTS_ENABLED

Controls whether to build and run facade tests. Facade tests add
considerably to build time, so they are disabled by default.

**Current value (from the default):** `false`

From //third_party/pigweed/src/pw_unit_test/facade_test.gni:24

### pw_unit_test_FACADE_TEST_NAME

Pigweed uses this internally to manage toolchain generation for facade
tests. This should NEVER be set manually, or depended on as stable API.

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_unit_test/facade_test.gni:28

### pw_unit_test_GOOGLETEST_BACKEND

The GoogleTest implementation to use for Pigweed unit tests. This library
provides "gtest/gtest.h" and related headers. Defaults to
pw_unit_test:light, which implements a subset of GoogleTest.

Type: string (GN path to a source set)
Usage: toolchain-controlled only

**Current value (from the default):** `"//third_party/pigweed/src/pw_unit_test:light"`

From //third_party/pigweed/src/pw_unit_test/test.gni:29

### pw_unit_test_MAIN

Implementation of a main function for ``pw_test`` unit test binaries. Must
be set to an appropriate target for the pw_unit_test backend.

Type: string (GN path to a source set)
Usage: toolchain-controlled only

**Current value (from the default):** `"//third_party/pigweed/src/pw_unit_test:simple_printing_main"`

From //third_party/pigweed/src/pw_unit_test/test.gni:36

### pw_unit_test_POOL_DEPTH

The maximum number of unit tests that may be run concurrently for the
current toolchain. Setting this to 0 disables usage of a pool, allowing
unlimited parallelization.

Note: A single target with two toolchain configurations (e.g. release/debug)
      will use two separate test runner pools by default. Set
      pw_unit_test_POOL_TOOLCHAIN to the same toolchain for both targets to
      merge the pools and force serialization.

Type: integer
Usage: toolchain-controlled only

**Current value (from the default):** `0`

From //third_party/pigweed/src/pw_unit_test/test.gni:74

### pw_unit_test_POOL_TOOLCHAIN

The toolchain to use when referring to the pw_unit_test runner pool. When
this is disabled, the current toolchain is used. This means that every
toolchain will use its own pool definition. If two toolchains should share
the same pool, this argument should be by one of the toolchains to the GN
path of the other toolchain.

Type: string (GN path to a toolchain)
Usage: toolchain-controlled only

**Current value (from the default):** `""`

From //third_party/pigweed/src/pw_unit_test/test.gni:84

### qr_codes_path

**Current value (from the default):** `"//src/recovery/system/res/qr_codes.riv"`

From //src/recovery/system/system_recovery_args.gni:11

### recovery_fdr_images_config_label

The images config information used for recovery images, including
recovery-fdr and recovery-ota.

NOTE: Only one recovery image can be selected for a build configuration.
However, images config is selected based on board, while recovery images are
selected based on product, so the build system doesn't always have full
information to match them. Also this is expected to be temporary until we
fully migrate assembly to Bazel.

**Current value (from the default):** `false`

From //build/board.gni:122

### recovery_label

Allows a product to specify the recovery image used in the zirconr slot.
Default recovery image is zedboot. Overriding this value will keep zedboot
in the build but will not include it as the default zirconr image.
Recovery images can provide an update target by specifying the metadata item
"update_target" in the format <target>=<path>. (Such as `update_target =
[ "recovery=" + rebase_path(recovery_path, root_build_dir) ]`)
Example value: "//build/images/recovery"

**Current value (from the default):** `"//build/images/zedboot"`

From //build/images/args.gni:167

### recovery_logo_path

**Current value (from the default):** `"//src/recovery/system/res/fuchsia-logo.riv"`

From //src/recovery/system/system_recovery_args.gni:9

### recovery_only

This is really a build for a recovery image, and so the fuchsia image that
is being built isn't properly configured, and so just disable the new image
assembly work until that's been addressed.

**Current value (from the default):** `false`

From //build/images/args.gni:19

### recovery_ota_images_config_label

**Current value (from the default):** `false`

From //build/board.gni:123

### recovery_route_sources_config

An optional file path to the route_sources verifier configuration to be used
on the assembled recovery system.

**Current value (from the default):** `""`

From //build/security.gni:147

### recovery_static_pkgs_goldens

An optional list of golden files for recovery.zbi static pkgs list. If
specified, they would be compared against recovery.zbi static pkgs list
during build time.  At least one of the golden files must match.
In normal case, there should only be golden file in this list.
During a soft transition where changes are made in a different repo than
the golden file repo, user need to
1. copy the old golden file before the change to '*.orig'
2. create a new golden file reflecting the changes
3. add both the old golden file and new golden file to this list. e.g. there
would be 'product.txt' and 'product.txt.orig' in this list and check in the
above changes.
4. check in the changes that is made in a different repo.
5. delete 'product.txt.orig' and remove it from this list.

**Current value (from the default):** `[]`

From //build/security.gni:129

### recovery_structured_config_policy

An optional file path to the structured configuration policy to be used on the
assembled recovery system. Defaults to no enforcement. Policy must be provided
for any product which is not an `eng` build type.

**Current value (from the default):** `""`

From //build/security.gni:139

### recovery_verify_component_resolvers_allowlist

**Current value (from the default):** `"//src/security/policy/component_resolvers_policy.json5"`

From //build/security.gni:205

### recovery_verify_routes_component_tree_config

An optional component tree configuration file used to finalize dynamic
elements of the component tree constructed for route verification on the
recovery assembled system. When non-empty, this value is passed as the
`--component-tree-config` option to `ffx scrutiny verify routes` to verify
routes in the fuchsia component tree.

**Current value (from the default):** `""`

From //build/security.gni:189

### recovery_verify_routes_exceptions_allowlist

An optional list of (capability, moniker) pairs that determine exceptions
to the verify_route.gni build rule that prevents v2 components from
attempting to use capabilities they were not offered in the recovery
system. Generally new entries should not be added to this allowlist and acts
as a marker for future technical debt to clean up.

The path to this list defaults to "" because most build configurations do
perform recovery build verification. The canonical allowlist for build
configurations that do perform recovery build verification is
"//src/security/policy/build/verify_routes_exceptions_allowlist.json5".

**Current value (from the default):** `""`

From //build/security.gni:167

### recovery_verify_routes_exceptions_allowlist_product

Same as recovery_verify_routes_exceptions_allowlist, except these allowlists
get added according to product-specific configuration.

**Current value (from the default):** `[]`

From //build/security.gni:175

### recovery_zbi_bootfs_filelist_goldens

An optional list of golden files for recovery.zbi bootFS file list. If
specified, they would be compared against recovery.zbi bootFS file list
during build time.  At least one of the golden files must match.
In normal case, there should only be golden file in this list.
During a soft transition where changes are made in a different repo than
the golden file repo, user need to
1. copy the old golden file before the change to '*.orig'
2. create a new golden file reflecting the changes
3. add both the old golden file and new golden file to this list. e.g. there
would be 'product.txt' and 'product.txt.orig' in this list and check in the
above changes.
4. check in the changes that is made in a different repo.
5. delete 'product.txt.orig' and remove it from this list.

**Current value (from the default):** `[]`

From //build/security.gni:69

### recovery_zbi_bootfs_packages_goldens

An optional list of golden files for recovery.zbi bootfs package index. If
specified, they would be compared against recovery.zbi bootfs package index
during build time.  At least one of the golden files must match.
In normal case, there should only be golden file in this list.
During a soft transition where changes are made in a different repo than
the golden file repo, user need to
1. copy the old golden file before the change to '*.orig'
2. create a new golden file reflecting the changes
3. add both the old golden file and new golden file to this list. e.g. there
would be 'product.txt' and 'product.txt.orig' in this list and check in the
above changes.
4. check in the changes that is made in a different repo.
5. delete 'product.txt.orig' and remove it from this list.

**Current value (from the default):** `[]`

From //build/security.gni:99

### recovery_zbi_kernel_cmdline_goldens

An optional list of golden files for recovery.zbi kernel cmdline args. If
specified, they would be compared against recovery.zbi kernel cmdline
during build time. At least one of the golden files must match.
In normal case, there should only be one golden file in this list.
During a soft transition where changes are made in a different repo than
the golden file repo, user need to
1. copy the old golden file before the change to '*.orig'
2. create a new golden file reflecting the changes
3. add both the old golden file and new golden file to this list. e.g. there
would be 'product.txt' and 'product.txt.orig' in this list and check in the
above changes.
4. check in the changes that is made in a different repo.
5. delete 'product.txt.orig' and remove it from this list.

**Current value (from the default):** `[]`

From //build/security.gni:39

### remove_default_configs

**Current value (from the default):** `[]`

From //third_party/pigweed/src/pw_build/defaults.gni:27

### repository_publish_blob_copy_mode

Controls which mode to use when copying blobs into the repository.
Supported modes are:

* `copy`: copy the blob if the blob does not already exist in the
  repository. This will use copy-on-write to efficiently copy the blob on
  file systems that support it.

* `copy-overwrite`: always copy the blob, overwriting any blob that
  exists in the blob repository. This will use copy-on-write to efficiently
  copy the blob on file systems that support it.

* `hard-link`: hard link the blob into the repository, or copy if we cannot
  create a hard link between the blob and the blob repository. Note that it
  is possible to modify the blob through the hard link, which would result
  in the blob not matching the blob's merkle.

**Current value (from the default):** `"copy"`

From //src/sys/pkg/bin/package-tool/package-tool.gni:296

### restat_cc

Set to true to make C++ compiles preserve timestamps of unchanged outputs.

**Current value (from the default):** `false`

From //build/toolchain/restat.gni:22

### restat_rust

Set to true to make Rust compiles preserve timestamps of unchanged outputs.

**Current value (from the default):** `false`

From //build/toolchain/restat.gni:19

### roboto_font_path

**Current value (from the default):** `"//prebuilt/third_party/fonts/roboto/Roboto-Regular.ttf"`

From //src/recovery/system/system_recovery_args.gni:10

### run_scrutiny_verifiers

Whether the scrutiny verifiers should be ran.

**Current value (from the default):** `false`

From //build/security.gni:9

### rust_cap_lints

Sets the maximum lint level.
"deny" will make all warnings into errors, "warn" preserves them as warnings, and "allow" will
ignore warnings.

**Current value (from the default):** `"deny"`

From //build/rust/config.gni:49

### rust_incremental

Enable incremental rust compilation. Takes a path to the directory to use
as the cache.

**Current value (from the default):** `""`

From //build/config/rust/BUILD.gn:29

### rust_lto

Sets the default LTO type for rustc bulids.

**Current value (from the default):** `""`

From //build/rust/config.gni:41

### rust_rbe_check

Run one of the more expensive checks, intended for CI.
All of these require rust_rbe_enable=true.

One of:

  * "none": No additional check.

  * "determinism":
      Check of determinism of rustc targets by running locally twice
      and comparing outputs, failing if any differences are found.
      Even though this check doesn't involve RBE, it uses the same
      wrapper script, which knows what output files to expect and
      compare.

      Build outputs that depend on time are discouraged because they
      impact caching.
      If your result depends on the current time, this check will
      definitely fail.  If it depends on only the date, there is still
      a nonzero chance of failure, if the rerun falls on the next day.

  * "consistency":
      Check consistency between local and remote rust compiles,
      by running both and comparing results.


**Current value (from the default):** `"none"`

From //build/toolchain/rbe.gni:129

### rust_rbe_enable

Set to true to enable distributed compilation of Rust using RBE.

**Current value for `target_cpu = "arm64"`:** `false`

From //out/not-default/args.gn:10

**Overridden from the default:** `false`

From //build/toolchain/rbe.gni:85

**Current value for `target_cpu = "x64"`:** `false`

From //out/not-default/args.gn:10

**Overridden from the default:** `false`

From //build/toolchain/rbe.gni:85

### rust_rbe_exec_strategy

One of:

  * "remote": Execute action remotely on cache miss.
        The remote cache is always updated with this result.

  * "local": Lookup action in the remote cache, but execute action
        locally on cache miss.  The locally produced result is
        not uploaded to the remote cache.

  * "remote_local_fallback": Execute action remotely first.
        If that fails, run locally instead.  The locally produced
        results are not uploaded to the remote cache.

  * "racing": Race local vs. remote execution, take the first to finish.

  (There are other rewrapper options that are not exposed.)

**Current value (from the default):** `"remote"`

From //build/toolchain/rbe.gni:103

### rust_rbe_use_python_impl

**Current value (from the default):** `false`

From //build/toolchain/rbe.gni:23

### rust_toolchain_triple_suffix

Sets the fuchsia toolchain target triple suffix (after arch)

**Current value (from the default):** `"fuchsia"`

From //build/rust/config.gni:44

### rust_v0_symbol_mangling

Controls whether the rust compiler uses v0 symbol mangling scheme
(see https://github.com/rust-lang/rfcs/blob/HEAD/text/2603-rust-symbol-name-mangling-v0.md).

**Current value (from the default):** `true`

From //build/config/rust/BUILD.gn:25

### rust_virtio_net

If true, uses the new Rust virtio-net device instead of the legacy C++ device.

**Current value (from the default):** `true`

From //src/virtualization/bin/args.gni:7

### rustc_prefix

Sets a custom base directory for `rustc` and `cargo`.
This can be used to test custom Rust toolchains.

**Current value (from the default):** `"//prebuilt/third_party/rust/linux-x64"`

From //build/rust/config.gni:24

### rustc_version_description

Human-readable identifier for the toolchain version.

TODO(tmandry): Make this the same repo/revision info from `rustc --version`.
e.g., clang_version_description = read_file("$_rustc_lib_dir/VERSION")

**Current value (from the default):** `""`

From //build/rust/config.gni:38

### rustc_version_string

This is a string identifying the particular toolchain version in use.  Its
only purpose is to be unique enough that it changes when switching to a new
toolchain, so that recompilations with the new compiler can be triggered.

When using the prebuilt, this is ignored and the CIPD instance ID of the
prebuilt is used.

**Current value (from the default):** `"5-xRQw1e9K9D6ql9rQ9dn9ahyVQ__V_w-ypg6ovEBwcC"`

From //build/rust/config.gni:32

### scenic_display_frame_number

Draws the current frame number in the top-left corner.

**Current value (from the default):** `false`

From //src/ui/scenic/lib/gfx/BUILD.gn:9

### scenic_enable_vulkan_validation

Include the vulkan validation layers in scenic.

**Current value (from the default):** `false`

From //src/ui/scenic/lib/gfx/build_args.gni:7

### scheduler_extra_invariant_validation

Enables extra (expensive) validation of scheduler invariants to assist in
debugging changes to the scheduler's behavior.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:60

### scheduler_queue_tracing_enabled

Enables scheduler queue tracing for trace-based scheduler performance
analysis.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:56

### scheduler_tracing_level

The level of detail for scheduler traces when enabled. Values greater than
zero add increasing details at the cost of increased trace buffer use.

0 = Default kernel:sched tracing.
1 = Adds duration traces for key scheduler operations.
2 = Adds flow events from wakeup to running state.
3 = Adds detailed internal durations and probes.

**Current value (from the default):** `0`

From //zircon/kernel/params.gni:52

### scudo_default_options

Default [Scudo](https://llvm.org/docs/ScudoHardenedAllocator.html) options
(before the `SCUDO_OPTIONS` environment variable is read at runtime).
Scudo is the memory allocator in Fuchsia's C library, so this affects all
Fuchsia programs.  This can be a list of strings or a single string.

This operates similarly to [`asan_default_options`](#asan_default_options)
and its cousins for other sanitizers, but is slightly different.  If this
variable is empty, then no `__scudo_default_options` function is injected
into programs at all.  Individual targets can use dependencies on
sanitizer_extra_options() targets to cause options to be injected, and that
will be compatible with any build-wide settings of `scudo_default_options`.
Programs **can** define their own `__scudo_default_options` functions, but
doing so will break all builds with this variable is set to nonempty, so
any program in the build that needs such a setting (which should be only in
tests) can use the sanitizer_extra_options() mechanism instead.

**Current value (from the default):** `[]`

From //build/config/sanitizers/sanitizer_default_options.gni:82

### sdk_archive_labels

Extra sdk() archive labels to be uploaded to the artifacts store. This is
an extension mechanism for SDK bits outside of the main repository.

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/common/bringup.gni:35

**Overridden from the default:** `[]`

From //BUILD.gn:90

**Current value for `target_cpu = "x64"`:** `[]`

From //products/common/bringup.gni:35

**Overridden from the default:** `[]`

From //BUILD.gn:90

### sdk_cross_compile_host_tools

Whether to cross-compile SDK tools for all supported host toolchains,
rather than just the current host toolchains.
For example, if this is true then for instance if building on linux x64 then
you'll also build SDK host tools for linux arm64.

**Current value (from the default):** `false`

From //sdk/config.gni:13

### sdk_id

Identifier for the Core SDK.

**Current value (from the default):** `"12.99991231.0.1"`

From //sdk/config.gni:7

### select_variant

List of "selectors" to request variant builds of certain targets.
Each selector specifies matching criteria and a chosen variant.
The first selector in the list to match a given target determines
which variant is used for that target.

Each selector is either a string or a scope.  A shortcut selector is
a string; it gets expanded to a full selector.  A full selector is a
scope, described below.

A string selector can match a name in
[`select_variant_shortcuts`](#select_variant_shortcuts).  If it's not a
specific shortcut listed there, then it can be the name of any variant
described in [`known_variants`](#known_variants) and
[`universal_variants`](#universal_variants) (and combinations thereof).
A `selector` that's a simple variant name selects for every binary
built in the target toolchain: `{ host=false variant=selector }`.

If a string selector contains a slash, then it's `"shortcut/filename"`
and selects only the binary in the target toolchain whose `output_name`
matches `"filename"`, i.e. it adds `output_name=["filename"]` to each
selector scope that the shortcut's name alone would yield.

The scope that forms a full selector defines some of these:

    variant (required)
        [string or `false`] The variant that applies if this selector
        matches.  This can be `false` to choose no variant, or a string
        that names the variant.  See
        [`known_variants`](#known_variants) and
        [`universal_variants`](#universal_variants).

The rest below are matching criteria.  All are optional.
The selector matches if and only if all of its criteria match.
If none of these is defined, then the selector always matches.

The first selector in the list to match wins and then the rest of
the list is ignored.  To construct more complex rules, use a blocklist
selector with `variant=false` before a catch-all default variant, or
a list of specific variants before a catch-all false variant.

Each "[strings]" criterion is a list of strings, and the criterion
is satisfied if any of the strings matches against the candidate string.

    host
        [boolean] If true, the selector matches in the host toolchain.
        If false, the selector matches in the target toolchain.

    testonly
        [boolean] If true, the selector matches targets with testonly=true.
        If false, the selector matches in targets without testonly=true.

    target_type
        [strings]: `"executable"`, `"loadable_module"`, or `"fuchsia_driver"`

    output_name
        [strings]: target's `output_name` (default: its `target name`)

    label
        [strings]: target's full label with `:` (without toolchain suffix)

    name
        [strings]: target's simple name (label after last `/` or `:`)

    dir
        [strings]: target's label directory (`//dir` for `//dir:name`).

**Current value (from the default):** `[]`

From //build/config/BUILDCONFIG.gn:1840

### select_variant_canonical

*This should never be set as a build argument.*
It exists only to be set in `toolchain_args`.
See //build/toolchain/clang_toolchain.gni for details.

**Current value (from the default):** `[]`

From //build/config/BUILDCONFIG.gn:1845

### select_variant_shortcuts

List of short names for commonly-used variant selectors.  Normally this
is not set as a build argument, but it serves to document the available
set of short-cut names for variant selectors.  Each element of this list
is a scope where `.name` is the short name and `.select_variant` is a
a list that can be spliced into [`select_variant`](#select_variant).

**Current value (from the default):**

```none
[{
  name = "host_asan"
  select_variant = [{
  host = true
  variant = "asan"
}]
}, {
  name = "host_asan-ubsan"
  select_variant = [{
  host = true
  variant = "asan-ubsan"
}]
}, {
  name = "host_coverage"
  select_variant = [{
  host = true
  variant = "coverage"
}]
}, {
  name = "host_coverage-rust"
  select_variant = [{
  host = true
  variant = "coverage-rust"
}]
}, {
  name = "host_profile"
  select_variant = [{
  host = true
  variant = "profile"
}]
}, {
  name = "host_tsan"
  select_variant = [{
  host = true
  variant = "tsan"
}]
}]
```

From //build/config/BUILDCONFIG.gn:1643

### size_checker_input

The input to the size checker.
The build system will produce a JSON file to be consumed by the size checker, which
will check and prevent integration of subsystems that are over their space allocation.
The input consists of the following keys:

asset_ext(string array): a list of extensions that should be considered as assets.

asset_limit(number): maximum size (in bytes) allocated for the assets.

core_limit(number): maximum size (in bytes) allocated for the core system and/or services.
This is sort of a "catch all" component that consists of all the area / packages that weren't
specified in the components list below.

core_creep_limit(number): maximum size creep (in bytes) per-CL allocated for the core system and/or services.
This may be enforced by Gerrit.

components(object array): a list of component objects. Each object should contain the following keys:

  component(string): name of the component.

  src(string array): path of the area / package to be included as part of the component.
  The path should be relative to the obj/ in the output directory.
  For example, consider two packages foo and far, built to out/.../obj/some_big_component/foo and out/.../obj/some_big_component/bar.
  If you want to impose a limit on foo, your src will be ["some_big_component/foo"].
  If you want to impose a limit on both foo and far, your src will be ["some_big_component"].
  If a package has config-data, those prebuilt blobs actually live under the config-data package.
  If you wish to impose a limit of those data as well, you should add "build/images/config-data/$for_pkg" to your src.
  The $for_pkg corresponds to the $for_pkg field in config.gni.

  limit(number): maximum size (in bytes) allocated for the component.
  creep_limit(number): maxmium size creep (in bytes) per-CL allocated for the component.
  This may be enforced by Gerrit.

distributed_shlibs(string array): a list of shared libraries which are distributed in the Fuchsia SDK for
partners to use in their prebuilt packages.

distributed_shlibs_limit(number): maximum size (in bytes) allocated for distributed shared libraries.

distributed_shlibs_creep_limit(number): maximum size creep (in bytes) allocated for distributed shared
libraries. This may be enforced by Gerrit.

icu_data(string array): a list of files which contribute to the ICU data limit.

icu_data_limit(number): maximum size (in bytes) allocated to ICU data files.

icu_data_creep_limit(number): maximum size creep (in bytes) allocated to ICU data files. This may be
enforced by Gerrit.

Example:
size_checker_input = {
  asset_ext = [ ".ttf" ]
  asset_limit = 10240
  core_limit = 10240
  core_creep_limit = 320
  distributed_shlibs = [
    "lib/ld.so.1",
    "lib/libc++.so.2",
  ]
  distributed_shlibs_limit = 10240
  distributed_shlibs_creep_limit = 320
  icu_data = [ "icudtl.dat" ]
  icu_data_limit = 20480
  icu_data_creep_limit = 320
  components = [
    {
      component = "Foo"
      src = [ "topaz/runtime/foo_runner" ]
      limit = 10240
      creep_limit = 320
    },
    {
      component = "Bar"
      src = [ "build/images" ]
      limit = 20480
      creep_limit = 640
    },
  ]
}

**Current value (from the default):** `{ }`

From //build/images/size_checker/size_checker_input.gni:84

### skip_buildtools_check

Skip buildtools dependency checks (needed for ChromeOS).

**Current value (from the default):** `false`

From //third_party/perfetto/gn/perfetto.gni:335

### smp_max_cpus

**Current value (from the default):** `16`

From //zircon/kernel/params.gni:21

### spinel_platform_header

Platform portability header for spinel.

**Current value (from the default):** `"\"spinel_platform.h\""`

From //third_party/openthread/src/lib/spinel/BUILD.gn:32

### stack_size_section

Whether to emit a stack-size section in the output file
https://clang.llvm.org/docs/ClangCommandLineReference.html#cmdoption-clang-fstack-size-section

**Current value (from the default):** `false`

From //build/config/clang/stack_size_section.gni:8

### starnix_disable_logging

Whether or not logging is disabled globally.

**Current value (from the default):** `false`

From //src/starnix/kernel/BUILD.gn:20

### starnix_disable_tracing

Whether or not tracing is disabled globally.

**Current value (from the default):** `true`

From //src/starnix/kernel/BUILD.gn:23

### starnix_restricted_mode

**Current value (from the default):** `false`

From //src/starnix/kernel/BUILD.gn:16

### target_cpu

**Current value for `target_cpu = "arm64"`:** `"arm64"`

From //out/not-default/args.gn:11

**Overridden from the default:** `""`

**Current value for `target_cpu = "x64"`:** `"x64"`

From //out/not-default/args.gn:11

**Overridden from the default:** `""`

### target_os

**Current value (from the default):** `""`

### target_persistent_debuglog_size

Controls (in bytes) the target size of the persistent debug log, in bytes.
Setting this to zero disables all persistent debug log functionality.  Note
that while the system will make an attempt to secure this many bytes for the
persistent debug log, it may not be able to due to limited persistent RAM
resources.  Must be a multiple of 128 bytes.

**Current value (from the default):** `0`

From //zircon/kernel/lib/persistent-debuglog/params.gni:13

### target_sysroot

The absolute path of the sysroot that is used with the target toolchain.

**Current value (from the default):** `""`

From //build/config/sysroot.gni:7

### termina_disk

The termina disk image.

Defaults to the disk image from CIPD, but can be overridden to use a
custom disk for development purposes.

**Current value (from the default):** `"//prebuilt/virtualization/packages/termina_guest/images/arm64/vm_rootfs.img"`

From //src/virtualization/packages/termina_guest/BUILD.gn:18

### termina_extras

The termina extras disk image.

Defaults to the disk image from CIPD, but can be overridden to use a
custom disk for development purposes.

**Current value (from the default):** `"//prebuilt/virtualization/packages/termina_guest/images/arm64/vm_extras.img"`

From //src/virtualization/packages/termina_guest/BUILD.gn:30

### termina_extras_tests

If `true`, adds additional testonly content to extras.img, which will be
built and mounted inside the container at /mnt/chromeos.

**Current value (from the default):** `true`

From //src/virtualization/bin/termina_guest_manager/BUILD.gn:14

### termina_fxfs_stateful_image

Whether to use Fxfs for the stateful image

**Current value (from the default):** `false`

From //src/virtualization/bin/termina_guest_manager/BUILD.gn:25

### termina_hermetic_bootstrap

If 'true', bundle the container image with the termina_guest_manager package
and use that to initialize the linux container.

If this is 'false', no container image will be bundled and instead the
container will be downloaded by the target at runtime. This makes the build
smaller but requires the target device to have a functional internet
connection at runtime.

**Current value (from the default):** `false`

From //src/virtualization/bin/termina_guest_manager/BUILD.gn:40

### termina_kernel

The termina kernel image.

Defaults to the common linux kernel image from CIPD, but can be overridden to use a
custom kernel for development purposes.

**Current value (from the default):** `"//prebuilt/virtualization/packages/termina_guest/kernel/arm64/vm_kernel-5.15"`

From //src/virtualization/packages/termina_guest/BUILD.gn:12

### termina_stateful_partition_size_bytes

Default stateful image size (40GiB).

If you change this value you will need to rebuild the guest partition using
'guest wipe termina' for the new size to take effect.

**Current value (from the default):** `42949672960`

From //src/virtualization/bin/termina_guest_manager/BUILD.gn:31

### termina_tools

The termina tools disk image.

Defaults to the disk image from CIPD, but can be overridden to use a
custom disk for development purposes.

**Current value (from the default):** `"//prebuilt/virtualization/packages/termina_guest/images/arm64/vm_tools.img"`

From //src/virtualization/packages/termina_guest/BUILD.gn:24

### termina_user_extras

Point this to the location of external files to be included as extras

**Current value (from the default):** `[]`

From //src/virtualization/bin/termina_guest_manager/BUILD.gn:22

### termina_volatile_block

If `true`, all block devices that would normally load as READ_WRITE will
be loaded as VOLATILE_WRITE. This is useful when working on changes to
the linux kernel as crashes and panics can sometimes corrupt the images.

**Current value (from the default):** `false`

From //src/virtualization/bin/termina_guest_manager/BUILD.gn:19

### terminal_bold_font_path

**Current value (from the default):** `"//prebuilt/third_party/fonts/robotomono/RobotoMono-Bold.ttf"`

From //src/ui/bin/terminal/terminal_args.gni:12

### terminal_bold_italic_font_path

**Current value (from the default):** `"//prebuilt/third_party/fonts/robotomono/RobotoMono-BoldItalic.ttf"`

From //src/ui/bin/terminal/terminal_args.gni:20

### terminal_fallback_font_paths

Paths to files to use for fallback fonts

**Current value (from the default):** `[]`

From //src/ui/bin/terminal/terminal_args.gni:23

### terminal_font_path

**Current value (from the default):** `"//prebuilt/third_party/fonts/robotomono/RobotoMono-Regular.ttf"`

From //src/ui/bin/terminal/terminal_args.gni:8

### terminal_italic_font_path

**Current value (from the default):** `"//prebuilt/third_party/fonts/robotomono/RobotoMono-Italic.ttf"`

From //src/ui/bin/terminal/terminal_args.gni:16

### test_durations_file

A file containing historical test duration data for this build
configuration, used used by testsharder to evenly split tests across
shards. It should be set for any builds where testsharder will be run
afterwards.

**Current value (from the default):** `""`

From //BUILD.gn:86

### testonly_in_containers

Whether to allow testonly=true targets in fuchsia ZBI or base/cache packages.

Possible values are
  "all": Allow testonly=true target in fuchsia ZBI and base/cache packages.
  "all_but_base_cache_packages": Do not allow testonly=true target in
     base/cache packages, but allow in other fuchsia ZBI dependencies.
  "none": Do not allow testonly=true target in all ZBI dependencies
     including base/cache packages.

Default value is 'all', it is preferable to set to 'none' for production
  image to avoid accidental inclusion of testing targets.

**Current value (from the default):** `"all"`

From //build/security.gni:218

### thinlto_cache_dir

ThinLTO cache directory path.

**Current value (from the default):** `"thinlto-cache"`

From //build/config/lto/config.gni:16

### thinlto_jobs

Number of parallel ThinLTO jobs.

**Current value (from the default):** `8`

From //build/config/lto/config.gni:13

### time_trace

Whether to export time traces when building with clang.
https://releases.llvm.org/9.0.0/tools/clang/docs/ReleaseNotes.html#new-compiler-flags

**Current value (from the default):** `false`

From //build/config/clang/time_trace.gni:8

### toolchain_variant

*This should never be set as a build argument.*
It exists only to be set in `toolchain_args`.
See //concepts/build_system/internals/toolchains/build_arguments.md#toolchain_variant
for details and documentation for each field.

**Current value (from the default):**

```none
{
  base = "//build/toolchain/fuchsia:arm64"
}
```

From //build/config/BUILDCONFIG.gn:115

### tsan_default_options

Default [ThreadSanitizer](https://clang.llvm.org/docs/ThreadSanitizer.html)
options (before the `TSAN_OPTIONS` environment variable is read at runtime).
This can be set as a build argument to affect most "tsan" variants in
$variants (which see), or overrideen in $toolchain_args in one of those
variants. This can be a list of strings or a single string.

Note that even if this is empty, programs in this build **cannot** define
their own `__tsan_default_options` C function.  Instead, they can use a
sanitizer_extra_options() target in their `deps` and then any options
injected that way can override that option's setting in this list.

TODO(fxbug.dev/89981): `ignore_noninstrumented_modules=1` can be reevaluated
when/if we have an instrumented libstd for Rust.

**Current value (from the default):** `["ignore_noninstrumented_modules=1"]`

From //build/config/sanitizers/sanitizer_default_options.gni:65

### ubsan_default_options

Default [UndefinedBehaviorSanitizer](https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html)
options (before the `UBSAN_OPTIONS` environment variable is read at
runtime).  This can be set as a build argument to affect most "ubsan"
variants in $variants (which see), or overridden in $toolchain_args in
one of those variants.  This can be a list of strings or a single string.

Note that even if this is empty, programs in this build **cannot** define
their own `__ubsan_default_options` C function.  Instead, they can use a
sanitizer_extra_options() target in their `deps` and then any options
injected that way can override that option's setting in this list.

**Current value (from the default):** `["print_stacktrace=1", "halt_on_error=1"]`

From //build/config/sanitizers/sanitizer_default_options.gni:47

### universal_variants

**Current value (from the default):**

```none
[{
  configs = []
  name = "debug"
  toolchain_args = {
  is_debug = true
}
}]
```

From //build/config/BUILDCONFIG.gn:1627

### universe_package_labels

If you add package labels to this variable, the packages will be included
in the 'universe' package set, which represents all software that is
produced that is to be published to a package repository or to the SDK by
the build. The build system ensures that the universe package set includes
the base and cache package sets, which means you do not need to redundantly
include those labels in this variable.

**Current value for `target_cpu = "arm64"`:** `["//bundles/kitchen_sink"]`

From //out/not-default/args.gn:17

**Overridden from the default:** `[]`

From //BUILD.gn:65

**Current value for `target_cpu = "x64"`:** `["//bundles/kitchen_sink"]`

From //out/not-default/args.gn:17

**Overridden from the default:** `[]`

From //BUILD.gn:65

### update_goldens

Set to true for the golden_file template to implicitly write updated goldens
instead of failing the action or test.

**Current value (from the default):** `false`

From //build/testing/config.gni:8

### update_kernels

(deprecated) List of kernel images to include in the update (OTA) package.
If no list is provided, all built kernels are included. The names in the
list are strings that must match the filename to be included in the update
package.

**Current value (from the default):** `[]`

From //build/images/args.gni:40

### update_package_size_creep_limit

How much the size of Update Package can be increased in one CL.

**Current value (from the default):** `90112`

From //build/images/size_checker/size_checker_input.gni:88

### update_product_epoch

The epoch to use in the update (OTA) package.
Before applying an update, Fuchsia confirms that the epoch in the update
package is not smaller than the epoch installed on the system. This prevents
Fuchsia from downloading an update that may not boot.

The product epoch is added to the platform epoch before placed in the update
package. Having a separate platform epoch ensures that every time the
platform introduces a backwards-incompatible change, each product gets their
epoch increased.

**Current value (from the default):** `0`

From //build/images/args.gni:51

### use_bazel_images_only

If true, the images.json build API modules will only include images from
`bazel_assembly_targets` and its dependencies.

NOTE: This field is highly experimental, do not set it unless you know
exactly what you are doing.

**Current value (from the default):** `false`

From //build/images/args.gni:195

### use_blink

**Current value (from the default):** `false`

From //build/config/features.gni:13

### use_bringup_assembly

Is the `assemble_system()` instantiation used by the product the standard
one or the bringup one?

**Current value for `target_cpu = "arm64"`:** `true`

From //products/common/bringup.gni:6

**Overridden from the default:** `false`

From //build/product.gni:8

**Current value for `target_cpu = "x64"`:** `true`

From //products/common/bringup.gni:6

**Overridden from the default:** `false`

From //build/product.gni:8

### use_cast_runner_canary

Whether to use the most recent (canary) version of the CastRunner prebuilt.
Otherwise, the qualified "release" version is used.
Set [`use_chromium_canary`](#use_chromium_canary) to the same value.

**Current value (from the default):** `false`

From //src/cast/generate_cast_targets.gni:19

### use_ccache

Set to true to enable compiling with ccache

**Current value (from the default):** `false`

From //build/toolchain/ccache.gni:9

### use_chromium_canary

Whether to use the most recent (canary) version of prebuilt Chromium
components. Otherwise, the qualified "release" version is used.
For scenarios where CastRunner is used,
[`use_cast_runner_canary`](#use_cast_runner_canary) must be set to the same
value.

**Current value (from the default):** `false`

From //src/chromium/BUILD.gn:34

### use_dbus

**Current value (from the default):** `false`

From //build/config/features.gni:11

### use_direct_for_carnelian_examples

Include a config in the example packages to attempt to use view mode
direct.

**Current value (from the default):** `false`

From //src/lib/ui/carnelian/BUILD.gn:30

### use_disk_based_minfs_migration

If true, will attempt to copy the data partition to a new partition and then delete the old one
to migrate from minfs to fxfs.

**Current value (from the default):** `false`

From //src/storage/fshost/generated_fshost_config.gni:42

### use_driver_framework_v2_default

If this is true, then DriverManager will load DFv2 drivers when unspecified by
fuchsia.driver.test/RealmArgs.use_driver_framework_v2.

**Current value (from the default):** `false`

From //sdk/lib/driver_test_realm/driver_test_realm.gni:8

### use_elf_kernel

Build an ELF kernel rather than a ZBI image kernel.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:135

### use_flatland_by_default

If true, Flatland is the default graphics protocol in Scenic.

**Current value (from the default):** `true`

From //src/ui/scenic/build_args.gni:7

### use_gigaboot

Build the gigaboot bootloader.

**Current value for `target_cpu = "arm64"`:** `true`

From //boards/arm64.gni:34

**Overridden from the default:** `false`

From //build/images/args.gni:26

**Current value for `target_cpu = "x64"`:** `true`

From //boards/x64.gni:60

**Overridden from the default:** `false`

From //build/images/args.gni:26

### use_gio

**Current value (from the default):** `false`

From //build/config/features.gni:12

### use_goma

Set to true to enable distributed compilation using Goma.
This has lower precedence than `use_reclient_cxx` in
//build/toolchain/rbe.gni.

**Current value for `target_cpu = "arm64"`:** `false`

From //out/not-default/args.gn:12

**Overridden from the default:** `false`

From //build/toolchain/goma.gni:13

**Current value for `target_cpu = "x64"`:** `false`

From //out/not-default/args.gn:12

**Overridden from the default:** `false`

From //build/toolchain/goma.gni:13

### use_llvm_libc_string_functions

**NOTE: Experimental** Use the llvm-libc implementations of string functions.

**Current value (from the default):** `false`

From //zircon/system/ulib/c/libc.gni:13

### use_lto

Use link time optimization (LTO).

**Current value (from the default):** `false`

From //build/config/lto/config.gni:7

### use_netstack3

DO NOT SET THIS IN A PRODUCT DEFINITION!!  FOR NETSTACK DEVELOPER USE ONLY
TODO(https://fxbug.dev/85450) - Convert this to a platform configuration
option in Product Assembly

**Current value (from the default):** `false`

From //src/connectivity/network/use_netstack3.gni:11

### use_new_account_manager

Whether or not to use the new AccountManager API for enrollment and authentication.

**Current value (from the default):** `false`

From //src/experiences/session_shells/ermine/login/BUILD.gn:16

### use_null_vulkan_on_host

TODO(liyl): Currently non-x64 platforms don't have Vulkan support,
so we always use the null Vulkan implementation instead.

Global arguments for whether we use a "null" Vulkan implementation on
host vulkan_executables and vulkan_tests, so that any attempt to create a
VkInstances or VkDevice will fail.

This argument will affect all vulkan_{executable/test} build targets.


**Current value (from the default):** `true`

From //src/lib/vulkan/build/config.gni:40

### use_oz

Controls whether to use -Oz when `optimize` is set to `"size"`.

**Current value (from the default):** `false`

From //build/config/compiler.gni:41

### use_prebuilt_codec_runner_intel_gen

True if a prebuilt codec_runner_intel_gen package is used. If false, the codec_runner will be
built from scratch (requires a checkout of the intel media-driver repo).

**Current value (from the default):** `true`

From //src/media/codec/codecs/vaapi/BUILD.gn:11

### use_prebuilt_ffmpeg

Use a prebuilt ffmpeg binary rather than building it locally.  See
//src/media/lib/ffmpeg/README.md for details.  This is ignored when
building in variant builds for which there is no prebuilt.  In that case,
ffmpeg is always built from source so as to be built with the selected
variant's config.  When this is false (either explicitly or in a variant
build) then //third_party/ffmpeg must be in the source tree, which
requires:

```
jiri import -name third_party/ffmpeg -revision HEAD third_party/ffmpeg http://fuchsia.googlesource.com/integration
```
TODO(fxbug.dev/116951): This isn't currently working. Use the method below.

Or, if already importing a different manifest from there, resulting in
errors from jiri update, it can work to just git clone (but jiri update
won't manage third_party/ffmpeg in this case):

```
mkdir build/secondary/third_party/ffmpeg
git clone https://fuchsia.googlesource.com/third_party/ffmpeg build/secondary/third_party/ffmpeg
mkdir third_party/yasm
git clone https://fuchsia.googlesource.com/third_party/yasm third_party/yasm
mkdir third_party/ffmpeg/src
git clone https://chromium.googlesource.com/chromium/third_party/ffmpeg third_party/ffmpeg/src
```

**Current value (from the default):** `true`

From //src/media/lib/ffmpeg/BUILD.gn:33

### use_scene_manager_as_scene_owner

If true, use scene_manager as the scene owner component (via ui.cml).
Otherwise use root_presenter (via ui_with_root_presenter.cml).

**Current value (from the default):** `true`

From //src/ui/build_args.gni:8

### use_spinel_for_carnelian_examples

Include a config in the example packages to attempt to use Spinel

**Current value (from the default):** `false`

From //src/lib/ui/carnelian/BUILD.gn:26

### use_swiftshader_vulkan_icd_on_host


Global arguments for whether we use the SwiftShader Vulkan ICD on host
vulkan_executables and vulkan_tests.

This argument will affect all vulkan_{executable/test} build targets and
it only works when use_null_vulkan_on_host is set to false.


**Current value (from the default):** `true`

From //src/lib/vulkan/build/config.gni:49

### use_thinlto

Use ThinLTO variant of LTO if use_lto = true.

**Current value (from the default):** `true`

From //build/config/lto/config.gni:10

### use_udev

**Current value (from the default):** `false`

From //build/config/features.gni:10

### use_vbmeta

If true, then a vbmeta image will be generated for provided ZBI
and the paving script will pave vbmeta images to the target device.

**Current value for `target_cpu = "arm64"`:** `true`

From //boards/arm64.gni:35

**Overridden from the default:** `false`

From //build/images/vbmeta.gni:14

**Current value for `target_cpu = "x64"`:** `true`

From //boards/x64.gni:58

**Overridden from the default:** `false`

From //build/images/vbmeta.gni:14

### use_vboot

Use vboot images

**Current value (from the default):** `false`

From //build/images/args.gni:10

### using_fuchsia_sdk

Only set in buildroots where targets configure themselves for use with the
Fuchsia SDK

**Current value (from the default):** `false`

From //build/fuchsia/sdk.gni:8

### vbmeta_a_partition

**Current value (from the default):** `""`

From //build/images/args.gni:113

### vbmeta_b_partition

**Current value (from the default):** `""`

From //build/images/args.gni:114

### vbmeta_r_partition

**Current value (from the default):** `""`

From //build/images/args.gni:115

### vboot_keys

vboot signing key directory. Must contain `kernel.keyblock` and
`kernel_data_key.vbprivk`. Defaults to the public ChromeOS test keys.

**Current value (from the default):** `"//third_party/vboot_reference/tests/devkeys"`

From //build/images/vboot/vboot.gni:16

### vboot_verbose

If true, vboot() image builds print out the exact "futility" command line.

**Current value (from the default):** `false`

From //build/images/vboot/vboot.gni:12

### verbose_image_assembly

Enable verbose output from `ffx assembly image`, this creates non-silent
build output and therefore should never be 'true' in checked-in configs, and
is meant solely for developer debugging.

**Current value (from the default):** `false`

From //build/images/args.gni:177

### virtcon_boot_animation_path

**Current value (from the default):** `"//src/bringup/bin/virtcon/data/boot-animation.riv"`

From //src/bringup/bin/virtcon/virtcon_args.gni:8

### virtmagma_debug

Enable verbose logging in virtmagma-related code

**Current value (from the default):** `false`

From //src/graphics/lib/magma/include/virtio/virtmagma_debug.gni:7

### virtual_alloc_host_size_shift

Set the page size shift of the host. This is used when running the allocator
in a host environment where page size constants may not exist. If this does
not much the actual host page size then a run time error will occur.

**Current value (from the default):** `12`

From //zircon/kernel/lib/virtual_alloc/BUILD.gn:13

### virtual_device_name_prefix

TODO(fxbug.dev/94051): move to board definitions.
Adds a prefix to the start of the virtual device name. Used to distinguish
between similar virtual device's using different configuration's such as
`emu_window_size`.

**Current value (from the default):** `""`

From //build/product.gni:44

### vm_tracing_level

The level of detail for traces emitted by the VM system. Values greater than
zero add increasing details at the cost of increased trace buffer use.

0 = Default kernel:* tracing.
1 = Adds flow events for asynchronous page requests.
2 = Adds duration events related to accessed faults and page faults.
3 = Adds duration events for PMM allocations and frees.

**Current value (from the default):** `0`

From //zircon/kernel/params.gni:77

### vulkan_host_runtime_dir


|vulkan_host_runtime_dir| is the path to Vulkan runtime libraries, which
contains prebuilt Vulkan loader, Vulkan layers, SwiftShader Vulkan ICD,
and descriptor files required to load the libraries.


**Current value (from the default):** `"//prebuilt/third_party/vulkan_runtime/linux-x64"`

From //src/lib/vulkan/build/config.gni:23

### vulkan_host_sdk_dir


|vulkan_host_sdk_dir| is the path to Vulkan SDK, which contains Vulkan
headers and sources to Vulkan loader, layers and tools.


**Current value (from the default):** `"//prebuilt/third_party/vulkansdk/linux/x86_64"`

From //src/lib/vulkan/build/config.gni:16

### vulkan_sdk

**Current value (from the default):** `""`

From //src/graphics/examples/vkproto/common/common.gni:48

### wait_queue_depth_tracing_enabled

Enables tracing of wait queue depths.  Used for post-processing analysis of
how deep wait queues tend to be under various loads, as well as how
frequently the change depth.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:68

### warn_on_sdk_changes

Whether to only warn when an SDK has been modified.
If false, any unacknowledged SDK change will cause a build failure.

**Current value (from the default):** `false`

From //build/sdk/config.gni:11

### wayland_bridge_protocol_logging

Whether protocol logging should be enabled

**Current value (from the default):** `false`

From //src/ui/wayland/bin/bridge/BUILD.gn:12

### wayland_server_fatal_object_lookup_failures

Enable this to make object lookup failures fatal for debugging.

**Current value (from the default):** `false`

From //src/lib/ui/wayland/server/BUILD.gn:12

### weave_build_legacy_wdm

Tells openweave to support legacy WDM mode.

**Current value (from the default):** `false`

From //third_party/openweave-core/config.gni:29

### weave_build_warm

Tells openweave to build WARM libraries.

**Current value (from the default):** `true`

From //third_party/openweave-core/config.gni:26

### weave_system_config_use_sockets

Tells openweave components to use bsd-like sockets.

**Current value (from the default):** `true`

From //third_party/openweave-core/config.gni:7

### weave_with_nlfaultinjection

Tells openweave components to support fault injection.

**Current value (from the default):** `false`

From //third_party/openweave-core/config.gni:20

### weave_with_verhoeff

Tells openweave to support Verhoeff checksum.

**Current value (from the default):** `true`

From //third_party/openweave-core/config.gni:23

### wlancfg_config_type

Selects the wlan configuration type to use. Choices:
  "client" - client mode
  "ap" - access point mode
  "" (empty string) - no configuration

**Current value (from the default):** `"client"`

From //src/connectivity/wlan/wlancfg/BUILD.gn:19

### zbi_compression

Compression setting for ZBI "storage" items.
This can be "zstd", optionally followed by ".LEVEL"
where `LEVEL` can be an integer or "max".

**Current value (from the default):** `"zstd"`

From //build/zbi/zbi.gni:12

### zedboot_devmgr_config

List of arguments to populate /boot/config/devmgr in the Zedboot image.

**Current value (from the default):** `[]`

From //build/images/zedboot/zedboot_args.gni:7

### zedboot_images_config_label

The images config information used for zedboot images.

**Current value for `target_cpu = "arm64"`:** `"//boards/images:zedboot_default"`

From //boards/arm64.gni:42

**Overridden from the default:** `false`

From //build/board.gni:112

**Current value for `target_cpu = "x64"`:** `"//boards/images:zedboot_default"`

From //boards/x64.gni:94

**Overridden from the default:** `false`

From //build/board.gni:112

### zircon_a_partition

Arguments to `fx flash` script (along with any `firmware_prebuilts` which
specify a partition).

If (exactly one of) `fvm_partition` or `fxfs_partition` is provided, the flash script will flash
the full OS, recovery + Zircon + FVM (or Fxfs) + SSH keys. In this case, the bootloader must
also support `fastboot oem add-staged-bootloader-file ssh.authorized_keys`.

Otherwise, the script will flash the recovery image to all slots, which
doesn't require the FVM or SSH keys.

**Current value (from the default):** `""`

From //build/images/args.gni:110

### zircon_asserts

**Current value (from the default):** `false`

From //build/config/fuchsia/BUILD.gn:138

### zircon_b_partition

**Current value (from the default):** `""`

From //build/images/args.gni:111

### zircon_kernel_disable_asserts

Forcibly disable all assertions for the Zircon kernel. If this is set, the
default is to use the value of zx_assert_level to control assertions when
building the kernel.

**Current value (from the default):** `false`

From //build/zircon/build_args.gni:9

### zircon_optimize

Zircon optimization level. Same acceptable values as `optimize`.
Note that this will be ignored, in favor of the global `optimize` variable
if the latter is one of: "none", "sanitizer", or "profile".

"Default" optimization offers a good balance of size and speed,
as measured by size comparisons of release builds and extensive microbenchmarks.
See: https://fuchsia-review.googlesource.com/c/fuchsia/+/600221/comments/3a4855ec_cf46619c

**Current value (from the default):** `"default"`

From //build/config/zircon/levels.gni:22

### zircon_r_partition

**Current value (from the default):** `""`

From //build/images/args.gni:112

### zircon_toolchain

*This should never be set as a build argument.*
It exists only to be set in `toolchain_args`.
For Zircon toolchains, this will be a scope whose schema
is documented in //build/toolchain/zircon/zircon_toolchain.gni.
For all other toolchains, this will be false.

This allows testing for a Zircon-specific toolchain with:

  if (zircon_toolchain != false) {
    // code path for Zircon-specific toolchains
  } else {
    // code path for non-Zircon ones.
  }

**Current value (from the default):** `false`

From //build/config/BUILDCONFIG.gn:132

### zircon_tracelog

Where to emit a tracelog from Zircon's GN run. No trace will be produced if
given the empty string. Path can be source-absolute or system-absolute.

**Current value (from the default):** `""`

From //build/zircon/build_args.gni:13

### zvb_partition_name

Partition name from where image will be verified

**Current value (from the default):** `"zircon"`

From //build/images/vbmeta.gni:23

### zx_assert_level

Controls which asserts are enabled.

`ZX_ASSERT` is always enabled.

* 0 disables standard C `assert()` and `ZX_DEBUG_ASSERT`.
* 1 disables `ZX_DEBUG_ASSERT`. Standard C `assert()` remains enabled.
* 2 enables all asserts.

**Current value (from the default):** `2`

From //build/config/zircon/levels.gni:13

## `target_cpu = "arm64"`

### amlogic_decoder_firmware_path

Path to the amlogic decoder firmware file. Overrides the default in the build.

**Current value (from the default):** `""`

From //src/media/drivers/amlogic_decoder/BUILD.gn:12

### arm_use_neon

Whether to use the neon FPU instruction set or not.
TODO(fxbug.dev/87237): move this to boards.

**Current value (from the default):** `true`

From //build/config/arm.gni:9

## `target_cpu = "x64"`

### anv_enable_external_sync_fd

TODO(fxbug.dev/67565) - remove once external sync FD extensions fully supported

**Current value (from the default):** `false`

From //third_party/mesa/src/intel/vulkan/BUILD.gn:27

### anv_use_max_ram

Give maximum possible memory to Vulkan heap

**Current value (from the default):** `false`

From //third_party/mesa/src/intel/vulkan/BUILD.gn:30

### build_libvulkan_goldfish

**Current value (from the default):** `"//third_party/android/device/generic/goldfish-opengl:libvulkan_goldfish"`

From //src/graphics/lib/goldfish-vulkan/gnbuild/BUILD.gn:11

