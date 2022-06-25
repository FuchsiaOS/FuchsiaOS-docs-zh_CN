# GN Build Arguments

## All builds

### acpica_debug_output
Enable debug output in the ACPI library (used by the ACPI bus driver).

**Current value (from the default):** `false`

From //zircon/system/ulib/acpica/acpica.gni:7

### active_partition

**Current value (from the default):** `""`

From //build/images/args.gni:106

### add_qemu_to_build_archives
Whether to include images necessary to run Fuchsia in QEMU in build
archives.

**Current value (from the default):** `false`

From //build/images/args.gni:112

### additional_bootserver_arguments
Additional bootserver args to add to pave.sh. New uses of this should be
added with caution, and ideally discussion. The present use case is to
enable throttling of netboot when specific network adapters are combined
with specific boards, due to driver and hardware challenges.

**Current value (from the default):** `""`

From //build/images/args.gni:118

### all_font_file_paths
List of file paths to every font asset. Populated in fonts.gni.

**Current value (from the default):** `[]`

From //src/fonts/build/font_args.gni:35

### all_toolchain_variants
*These should never be set as a build argument.*
It will be set below and passed to other toolchains through toolchain_args
(see variant_toolchain.gni).

**Current value (from the default):** `[]`

From //build/config/BUILDCONFIG.gn:1756

### allow_legacy_data_partition_names
Set to true to enable legacy data partition names.

**Current value (from the default):** `false`

From //src/storage/fshost/generated_fshost_config.gni:37

### allowed_test_device_types
A list of device types this build is allowed to run tests on.

**Current value (from the default):** `[]`

From //build/testing/test_spec.gni:11

### always_zedboot
Build boot images that prefer Zedboot over local boot (only for EFI).

**Current value (from the default):** `false`

From //build/images/args.gni:133

### anv_enable_external_sync_fd
TODO(fxbug.dev/67565) - remove once external sync FD extensions fully supported

**Current value (from the default):** `false`

From [//third_party/mesa/src/intel/vulkan/BUILD.gn:27](https://fuchsia.googlesource.com/third_party/mesa/+/a479927186409cc1bcce82c0e023676cd9744f1a/src/intel/vulkan/BUILD.gn#27)

### anv_use_max_ram
Give maximum possible memory to Vulkan heap

**Current value (from the default):** `false`

From [//third_party/mesa/src/intel/vulkan/BUILD.gn:30](https://fuchsia.googlesource.com/third_party/mesa/+/a479927186409cc1bcce82c0e023676cd9744f1a/src/intel/vulkan/BUILD.gn#30)

### api_compatibility_testing
Whether to run API compatibility tests.

**Current value (from the default):** `true`

From //build/fidl/fidl_library.gni:19

### appmgr_core_shards
Core shards that are required for including appmgr in a product.
TODO(shayba): populate this after tqrev.dev/544025 lands

**Current value (from the default):** `["//src/sys/appmgr:appmgr_core_shard", "//src/sys/core:core_proxy_shard"]`

From //src/sys/appmgr/core_shards.gni:8

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

### auto_update_packages
Controls the behavior of sysmgr's PackageUpdatingLoader (v1) and the
full-resolver (v2). If true, when resolving a component an attempt to
update the component's package is first made through the Software Delivery
system (specifically, through the package resolver,
fuchsia.pkg.PackageResolver). If false, no attempt to update is made and
components are loaded only from packages already available locally (for
example, because the package is in base).

**Current value (from the default):** `true`

From //build/security.gni:151

### avb_algorithm

**Current value (from the default):** `"DEPRECATED"`

From //build/images/vbmeta.gni:41

### avb_atx_metadata
AVB metadata which will be used to validate public key

**Current value for `target_cpu = "arm64"`:** `"//third_party/android/platform/external/avb/test/data/atx_metadata.bin"`

From //boards/arm64.gni:51

**Overridden from the default:** `""`

From //build/images/vbmeta.gni:20

**Current value for `target_cpu = "x64"`:** `"//third_party/android/platform/external/avb/test/data/atx_metadata.bin"`

From //boards/x64.gni:56

**Overridden from the default:** `""`

From //build/images/vbmeta.gni:20

### avb_key
a key which will be used to sign VBMETA and images for AVB

**Current value for `target_cpu = "arm64"`:** `"//third_party/android/platform/external/avb/test/data/testkey_atx_psk.pem"`

From //boards/arm64.gni:53

**Overridden from the default:** `""`

From //build/images/vbmeta.gni:17

**Current value for `target_cpu = "x64"`:** `"//third_party/android/platform/external/avb/test/data/testkey_atx_psk.pem"`

From //boards/x64.gni:54

**Overridden from the default:** `""`

From //build/images/vbmeta.gni:17

### base_driver_package_labels
If you add fuchsia_driver_package labels to this variable, any drivers in these packages will
be visible to Driver Manager. These package labels are also considered to be in the
'base' package set (for more info see 'base_package_labels').

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/bringup.gni:56

**Overridden from the default:** `[]`

From //BUILD.gn:38

**Current value for `target_cpu = "x64"`:** `[]`

From //products/bringup.gni:56

**Overridden from the default:** `[]`

From //BUILD.gn:38

### base_package_labels
If you add package labels to this variable, the packages will be included in
the 'base' package set, which represents the set of packages that are part
of an OTA. These packages are updated as an atomic unit during an OTA
process and are immutable and are a superset of the TCB (Trusted Computing
Base) for a product. These packages are never evicted by the system.

**Current value for `target_cpu = "arm64"`:** `[]`

From //out/not-default/args.gn:10

**Overridden from the default:** `[]`

From //BUILD.gn:46

**Current value for `target_cpu = "x64"`:** `[]`

From //out/not-default/args.gn:10

**Overridden from the default:** `[]`

From //BUILD.gn:46

### base_resolver_enable_subpackages
Whether to allow base-resolver to resolve subpackages.
TODO(fxbug.dev/102652): This configuration will be removed when subpackages
is generally available.

**Current value (from the default):** `false`

From //build/security.gni:156

### basic_env_names
The list of environment names to include in "basic_envs".

**Current value (from the default):** `["emu"]`

From //build/testing/environments.gni:9

### bless_goldens
Set to true for the golden_file template to implicitly write updated goldens
instead of failing the action or test.

**Current value (from the default):** `false`

From //build/testing/config.gni:8

### blob_layout_format
The format blobfs should store blobs in.

**Current value (from the default):** `"compact"`

From //build/images/args.gni:130

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

From //build/images/fvm.gni:74

### blobfs_board_minimum_data_bytes
Number of bytes to reserve for data in the fs. This is in addition
to what is reserved, if any, for the inodes. Data bytes constitutes
"usable" space of the fs.
A value of false does not reserve any additional space than minimum
required for the filesystem.

**Current value (from the default):** `false`

From //build/images/fvm.gni:61

### blobfs_board_minimum_inodes
minimum_inodes is the number of inodes to reserve for the fs
A value of false does not reserve any additional space than minimum
required for the filesystem.

**Current value (from the default):** `false`

From //build/images/fvm.gni:53

### blobfs_capacity
Maximum allowable contents for the /blob in a release mode build for
both slot A and slot B of the system.
Zero means no limit.

**Current value for `target_cpu = "arm64"`:** `"10485760000"`

From //boards/arm64.gni:12

**Overridden from the default:** `"0"`

From //build/images/filesystem_limits.gni:15

**Current value for `target_cpu = "x64"`:** `"10485760000"`

From //boards/x64.gni:14

**Overridden from the default:** `"0"`

From //build/images/filesystem_limits.gni:15

### blobfs_enable_streaming_writes
Set this to true when configuring gn args to enable blobfs streaming writes.
This is a compile time argument which allows us to conditionally enable blobfs streaming writes
only on specific configurations.

**Current value (from the default):** `false`

From //src/storage/blobfs/BUILD.gn:18

### blobfs_maximum_runtime_bytes
blobfs_maximum_runtime_bytes is an upper bound on the partition size on the device. Partitions
can grow as needed if there are extra slices available in FVM. This limit prevents the blobfs
partition from taking too much space away from other uses.

Pass the empty string for no limit.

**Current value (from the default):** `""`

From //src/storage/fshost/generated_fshost_config.gni:13

### blobfs_page_in_metrics_recording
Set this to true when configuring gn args to enable blobfs page-in
metrics recording.
This will also increase the inspect VMO size for blobfs to 2 MiB,
to accommodate the large number of metrics entries.

**Current value (from the default):** `false`

From //src/storage/blobfs/BUILD.gn:13

### blobfs_product_maximum_bytes

**Current value (from the default):** `false`

From //build/images/fvm.gni:75

### blobfs_product_minimum_data_bytes

**Current value (from the default):** `false`

From //build/images/fvm.gni:62

### blobfs_product_minimum_inodes

**Current value (from the default):** `false`

From //build/images/fvm.gni:54

### board_bootfs_labels
A list of binary labels to include in the ZBI.

**Current value for `target_cpu = "arm64"`:** `["//src/connectivity/ethernet/drivers/virtio:virtio_ethernet", "//src/devices/block/drivers/virtio:virtio_block", "//src/devices/block/drivers/virtio:virtio_scsi", "//src/devices/misc/drivers/virtio-rng:virtio_rng", "//src/devices/misc/drivers/virtio-socket:virtio_socket", "//src/devices/serial/drivers/virtio-console:virtio_console", "//src/graphics/drivers/misc:goldfish_fuchsia_drivers", "//src/graphics/display/drivers/goldfish-display", "//src/graphics/drivers/virtio:virtio_gpu", "//src/ui/input/drivers/virtio:virtio_input", "//src/ui/input/drivers/goldfish_sensor:sensor_driver", "//src/devices/block/drivers/ahci", "//src/devices/board/drivers/acpi-arm64", "//src/devices/board/drivers/qemu-arm64", "//src/devices/rtc/drivers/pl031-rtc", "//src/graphics/display/drivers/fake:fake-display", "//src/devices/bus/drivers/pci:bus-pci", "//src/devices/usb/drivers/xhci", "//src/power/power-manager:base_config", "//src/security/policy/zxcrypt:null"]`

From //boards/arm64.gni:21

**Overridden from the default:** `[]`

From //build/board.gni:28

**Current value for `target_cpu = "x64"`:** `["//src/devices/bin/driver_host2", "//src/devices/block/drivers/ahci", "//src/devices/block/drivers/mbr", "//src/devices/block/drivers/nvme", "//src/devices/block/drivers/pci-sdhci", "//src/devices/block/drivers/sdhci", "//src/devices/board/drivers/x86:platform-bus-x86", "//src/devices/bus/drivers/pci:bus-pci", "//src/devices/i2c/drivers/intel-i2c", "//src/devices/rtc/drivers/intel-rtc", "//src/devices/spi/drivers/intel-gspi", "//src/devices/tpm/drivers/tpm", "//src/devices/usb/drivers/xhci", "//src/graphics/display/drivers/intel-i915", "//src/media/audio/drivers/codecs/alc5514", "//src/media/audio/drivers/codecs/alc5663", "//src/media/audio/drivers/codecs/max98373", "//src/media/audio/drivers/codecs/max98927", "//src/media/audio/drivers/intel-hda/codecs/hdmi:hdmi-audio-codec", "//src/media/audio/drivers/intel-hda/codecs/realtek:realtek-audio-codec", "//src/media/audio/drivers/intel-hda/controller:intel-hda", "//src/ui/input/drivers/i2c-hid", "//src/ui/input/drivers/pc-ps2", "//src/devices/bin/acpidump", "//src/devices/pci/bin:bootfs", "//src/media/audio/bin/ihda", "//src/power/power-manager:base_config", "//src/security/policy/zxcrypt:null", "//src/connectivity/ethernet/drivers/realtek-8111", "//src/devices/serial/drivers/uart16550", "//src/graphics/display/drivers/simple:simple.amd-kaveri", "//src/graphics/display/drivers/simple:simple.nv", "//zircon/third_party/dev/ethernet/e1000", "//boards/kernel_cmdline:serial-legacy", "//src/connectivity/ethernet/drivers/virtio:virtio_ethernet", "//src/devices/block/drivers/virtio:virtio_block", "//src/devices/block/drivers/virtio:virtio_scsi", "//src/devices/misc/drivers/virtio-rng:virtio_rng", "//src/devices/misc/drivers/virtio-socket:virtio_socket", "//src/devices/serial/drivers/virtio-console:virtio_console", "//src/graphics/drivers/misc:goldfish_fuchsia_drivers", "//src/graphics/display/drivers/goldfish-display", "//src/graphics/drivers/virtio:virtio_gpu", "//src/ui/input/drivers/virtio:virtio_input", "//src/ui/input/drivers/goldfish_sensor:sensor_driver", "//src/graphics/display/drivers/simple:simple.bochs", "//src/graphics/display/drivers/simple:simple.gga", "//src/graphics/display/drivers/simple:simple.intel", "//src/graphics/display/drivers/simple:simple.vmware", "//src/media/audio/drivers/intel-hda/codecs/qemu:qemu-audio-codec", "//src/devices/bin/driver_host2"]`

From //boards/x64.gni:41

**Overridden from the default:** `[]`

From //build/board.gni:28

### board_configs
Configs that are added when targeting this board.

**Current value (from the default):** `[]`

From //build/board.gni:13

### board_core_realm_shards
Core realm shards specific to this board. See //src/sys/core for more
context.

**Current value (from the default):** `[]`

From //build/board.gni:57

### board_description
Human readable board description corresponding to the board name.

**Current value for `target_cpu = "arm64"`:** `"A generic emulated arm64 device."`

From //boards/arm64.gni:10

**Overridden from the default:** `""`

From //build/board.gni:10

**Current value for `target_cpu = "x64"`:** `"A generic x64 device"`

From //boards/x64.gni:12

**Overridden from the default:** `""`

From //build/board.gni:10

### board_driver_package_labels
A list of driver package labels to include in the 'base' package set. Used
by the board definition rather than the product definition.

**Current value for `target_cpu = "arm64"`:** `["//bundles/packages/prod:drivers-system", "//bundles/packages/prod:wlan_drivers", "//src/media/audio/bundles:virtual_audio_driver"]`

From //boards/arm64.gni:27

**Overridden from the default:** `[]`

From //build/board.gni:17

**Current value for `target_cpu = "x64"`:** `["//bundles/packages/prod:drivers-system", "//bundles/packages/prod:wifi_intel", "//src/devices/acpi:drivers", "//src/graphics/drivers/msd-intel-gen", "//src/media/audio/bundles:virtual_audio_driver"]`

From //boards/common/x64-common.gni:54

**Overridden from the default:** `[]`

From //build/board.gni:17

### board_extra_vbmeta_images
DEPRECATED:  Remove when no boards set a value for these.

**Current value (from the default):** `[]`

From //build/images/vbmeta.gni:40

### board_fastboot_unlock_credentials
A list of paths to the unlock credentials file necessary to unlock this
board's fastboot protocol.

**Current value (from the default):** `[]`

From //build/board.gni:61

### board_fshost_config
A list of fshost options to add to the fshost config.

**Current value (from the default):** `{ }`

From //build/board.gni:45

### board_has_libvulkan_arm_mali
Board files can set this to true if they have a package with a mali libvulkan VCD.

**Current value (from the default):** `false`

From //src/graphics/lib/magma/gnbuild/magma.gni:45

### board_host_labels
A list of binary host tool labels to also build.

**Current value (from the default):** `[]`

From //build/board.gni:31

### board_is_emu
Whether or not the board supports emulator/physical devices.
This is used to determine if product bundle metadata should generate a
physical/virtual device spec or both.

**Current value for `target_cpu = "arm64"`:** `true`

From //boards/arm64.gni:43

**Overridden from the default:** `false`

From //build/board.gni:80

**Current value for `target_cpu = "x64"`:** `true`

From //boards/x64.gni:83

**Overridden from the default:** `false`

From //build/board.gni:80

### board_is_phys

**Current value for `target_cpu = "arm64"`:** `false`

From //boards/arm64.gni:44

**Overridden from the default:** `true`

From //build/board.gni:81

**Current value (from the default):** `true`

From //build/board.gni:81

### board_name
Board name used for paving and amber updates.

**Current value for `target_cpu = "arm64"`:** `"qemu-arm64"`

From //boards/arm64.gni:9

**Overridden from the default:** `""`

From //build/board.gni:7

**Current value for `target_cpu = "x64"`:** `"x64"`

From //boards/x64.gni:11

**Overridden from the default:** `""`

From //build/board.gni:7

### board_package_labels
A list of package labels to include in the 'base' package set. Used by the
board definition rather than the product definition.

**Current value for `target_cpu = "arm64"`:** `["//src/hwinfo:default_board_config", "//src/devices/sysmem/bin/sysmem_connector", "//src/graphics/bin/vulkan_loader"]`

From //boards/arm64.gni:33

**Overridden from the default:** `[]`

From //build/board.gni:21

**Current value for `target_cpu = "x64"`:** `["//src/devices/sysmem/bin/sysmem_connector", "//src/graphics/bin/vulkan_loader", "//src/power/thermd", "//src/hwinfo:default_board_config", "//src/graphics/drivers/intel-gen/icd:libvulkan_intel_gen", "//src/graphics/lib/goldfish-vulkan/gnbuild:goldfish-vulkan", "//src/graphics/lib/goldfish-vulkan/gnbuild:goldfish-vulkan-config", "//src/media/codec/codecs/vaapi:codec_runner_intel_gen_prebuilt"]`

From //boards/common/x64-common.gni:62

**Overridden from the default:** `[]`

From //build/board.gni:21

### board_recovery_bootfs_labels
A list of binary labels to include in the recovery ZBI.

**Current value for `target_cpu = "arm64"`:** `["//src/connectivity/ethernet/drivers/virtio:virtio_ethernet", "//src/devices/block/drivers/virtio:virtio_block", "//src/devices/block/drivers/virtio:virtio_scsi", "//src/devices/misc/drivers/virtio-rng:virtio_rng", "//src/devices/misc/drivers/virtio-socket:virtio_socket", "//src/devices/serial/drivers/virtio-console:virtio_console", "//src/graphics/drivers/misc:goldfish_fuchsia_drivers", "//src/graphics/display/drivers/goldfish-display", "//src/graphics/drivers/virtio:virtio_gpu", "//src/ui/input/drivers/virtio:virtio_input", "//src/ui/input/drivers/goldfish_sensor:sensor_driver", "//src/devices/block/drivers/ahci", "//src/devices/board/drivers/acpi-arm64", "//src/devices/board/drivers/qemu-arm64", "//src/devices/rtc/drivers/pl031-rtc", "//src/graphics/display/drivers/fake:fake-display", "//src/devices/bus/drivers/pci:bus-pci", "//src/devices/usb/drivers/xhci", "//src/power/power-manager:base_config", "//src/security/policy/zxcrypt:null"]`

From //boards/arm64.gni:25

**Overridden from the default:** `[]`

From //build/board.gni:42

**Current value for `target_cpu = "x64"`:** `["//src/devices/bin/driver_host2", "//src/devices/block/drivers/ahci", "//src/devices/block/drivers/mbr", "//src/devices/block/drivers/nvme", "//src/devices/block/drivers/pci-sdhci", "//src/devices/block/drivers/sdhci", "//src/devices/board/drivers/x86:platform-bus-x86", "//src/devices/bus/drivers/pci:bus-pci", "//src/devices/i2c/drivers/intel-i2c", "//src/devices/rtc/drivers/intel-rtc", "//src/devices/spi/drivers/intel-gspi", "//src/devices/tpm/drivers/tpm", "//src/devices/usb/drivers/xhci", "//src/graphics/display/drivers/intel-i915", "//src/media/audio/drivers/codecs/alc5514", "//src/media/audio/drivers/codecs/alc5663", "//src/media/audio/drivers/codecs/max98373", "//src/media/audio/drivers/codecs/max98927", "//src/media/audio/drivers/intel-hda/codecs/hdmi:hdmi-audio-codec", "//src/media/audio/drivers/intel-hda/codecs/realtek:realtek-audio-codec", "//src/media/audio/drivers/intel-hda/controller:intel-hda", "//src/ui/input/drivers/i2c-hid", "//src/ui/input/drivers/pc-ps2", "//src/devices/bin/acpidump", "//src/devices/pci/bin:bootfs", "//src/media/audio/bin/ihda", "//src/power/power-manager:base_config", "//src/security/policy/zxcrypt:null", "//src/connectivity/ethernet/drivers/realtek-8111", "//src/devices/serial/drivers/uart16550", "//src/graphics/display/drivers/simple:simple.amd-kaveri", "//src/graphics/display/drivers/simple:simple.nv", "//zircon/third_party/dev/ethernet/e1000", "//boards/kernel_cmdline:serial-legacy", "//src/connectivity/ethernet/drivers/virtio:virtio_ethernet", "//src/devices/block/drivers/virtio:virtio_block", "//src/devices/block/drivers/virtio:virtio_scsi", "//src/devices/misc/drivers/virtio-rng:virtio_rng", "//src/devices/misc/drivers/virtio-socket:virtio_socket", "//src/devices/serial/drivers/virtio-console:virtio_console", "//src/graphics/drivers/misc:goldfish_fuchsia_drivers", "//src/graphics/display/drivers/goldfish-display", "//src/graphics/drivers/virtio:virtio_gpu", "//src/ui/input/drivers/virtio:virtio_input", "//src/ui/input/drivers/goldfish_sensor:sensor_driver", "//src/graphics/display/drivers/simple:simple.bochs", "//src/graphics/display/drivers/simple:simple.gga", "//src/graphics/display/drivers/simple:simple.intel", "//src/graphics/display/drivers/simple:simple.vmware", "//src/media/audio/drivers/intel-hda/codecs/qemu:qemu-audio-codec", "//src/devices/bin/driver_host2"]`

From //boards/x64.gni:42

**Overridden from the default:** `[]`

From //build/board.gni:42

### board_recovery_package_labels
A list of package labels to include in the recovery package set. Used by the
board definition rather than the product definition.

**Current value (from the default):** `[]`

From //build/board.gni:25

### board_supports_update_configurator
Whether or not the board pulls in the system-update-configurator component.

**Current value (from the default):** `false`

From //build/board.gni:75

### board_system_image_deps
A list of binary labels to include in the system_image package.

**Current value (from the default):** `[]`

From //build/board.gni:53

### board_tools
List of paths to board-specific tools to include in the build output.

Most development tools can just be used in-tree and do not need to be
included here. This arg is only meant for tools which may need to be
distributed along with the build files, for example tools for flashing
from SoC recovery mode.

Assets included in this way are included best-effort only and do not form
any kind of stable contract for users of the archive.

**Current value (from the default):** `[]`

From //build/board.gni:72

### board_zedboot_bootfs_labels
A list of binary labels to include in the zedboot ZBI.

**Current value for `target_cpu = "arm64"`:** `["//src/connectivity/ethernet/drivers/virtio:virtio_ethernet", "//src/devices/block/drivers/virtio:virtio_block", "//src/devices/block/drivers/virtio:virtio_scsi", "//src/devices/misc/drivers/virtio-rng:virtio_rng", "//src/devices/misc/drivers/virtio-socket:virtio_socket", "//src/devices/serial/drivers/virtio-console:virtio_console", "//src/graphics/drivers/misc:goldfish_fuchsia_drivers", "//src/graphics/display/drivers/goldfish-display", "//src/graphics/drivers/virtio:virtio_gpu", "//src/ui/input/drivers/virtio:virtio_input", "//src/ui/input/drivers/goldfish_sensor:sensor_driver", "//src/devices/block/drivers/ahci", "//src/devices/board/drivers/acpi-arm64", "//src/devices/board/drivers/qemu-arm64", "//src/devices/rtc/drivers/pl031-rtc", "//src/graphics/display/drivers/fake:fake-display", "//src/devices/bus/drivers/pci:bus-pci", "//src/devices/usb/drivers/xhci", "//src/power/power-manager:base_config", "//src/security/policy/zxcrypt:null"]`

From //boards/arm64.gni:23

**Overridden from the default:** `[]`

From //build/board.gni:39

**Current value for `target_cpu = "x64"`:** `["//src/devices/bin/driver_host2", "//src/devices/block/drivers/ahci", "//src/devices/block/drivers/mbr", "//src/devices/block/drivers/nvme", "//src/devices/block/drivers/pci-sdhci", "//src/devices/block/drivers/sdhci", "//src/devices/board/drivers/x86:platform-bus-x86", "//src/devices/bus/drivers/pci:bus-pci", "//src/devices/i2c/drivers/intel-i2c", "//src/devices/rtc/drivers/intel-rtc", "//src/devices/spi/drivers/intel-gspi", "//src/devices/tpm/drivers/tpm", "//src/devices/usb/drivers/xhci", "//src/graphics/display/drivers/intel-i915", "//src/media/audio/drivers/codecs/alc5514", "//src/media/audio/drivers/codecs/alc5663", "//src/media/audio/drivers/codecs/max98373", "//src/media/audio/drivers/codecs/max98927", "//src/media/audio/drivers/intel-hda/codecs/hdmi:hdmi-audio-codec", "//src/media/audio/drivers/intel-hda/codecs/realtek:realtek-audio-codec", "//src/media/audio/drivers/intel-hda/controller:intel-hda", "//src/ui/input/drivers/i2c-hid", "//src/ui/input/drivers/pc-ps2", "//src/devices/bin/acpidump", "//src/devices/pci/bin:bootfs", "//src/media/audio/bin/ihda", "//src/power/power-manager:base_config", "//src/security/policy/zxcrypt:null", "//src/connectivity/ethernet/drivers/realtek-8111", "//src/devices/serial/drivers/uart16550", "//src/graphics/display/drivers/simple:simple.amd-kaveri", "//src/graphics/display/drivers/simple:simple.nv", "//zircon/third_party/dev/ethernet/e1000", "//boards/kernel_cmdline:serial-legacy", "//src/connectivity/ethernet/drivers/virtio:virtio_ethernet", "//src/devices/block/drivers/virtio:virtio_block", "//src/devices/block/drivers/virtio:virtio_scsi", "//src/devices/misc/drivers/virtio-rng:virtio_rng", "//src/devices/misc/drivers/virtio-socket:virtio_socket", "//src/devices/serial/drivers/virtio-console:virtio_console", "//src/graphics/drivers/misc:goldfish_fuchsia_drivers", "//src/graphics/display/drivers/goldfish-display", "//src/graphics/drivers/virtio:virtio_gpu", "//src/ui/input/drivers/virtio:virtio_input", "//src/ui/input/drivers/goldfish_sensor:sensor_driver", "//src/graphics/display/drivers/simple:simple.bochs", "//src/graphics/display/drivers/simple:simple.gga", "//src/graphics/display/drivers/simple:simple.intel", "//src/graphics/display/drivers/simple:simple.vmware", "//src/media/audio/drivers/intel-hda/codecs/qemu:qemu-audio-codec", "//src/devices/bin/driver_host2"]`

From //boards/x64.gni:43

**Overridden from the default:** `[]`

From //build/board.gni:39

### board_zedboot_cmdline_args
List of kernel command line arguments to bake into the zedboot image that are
required by this board. See also zedboot_cmdline_args in
//build/images/zedboot/BUILD.gn

**Current value (from the default):** `[]`

From //build/board.gni:36

### board_zedboot_fshost_config
A list of fshost options to add to the fshost config in the zedboot image.

**Current value (from the default):** `{ }`

From //build/board.gni:49

### bootfs_only
Put the "system image" package in the BOOTFS.  Hence what would
otherwise be /system/... at runtime is /boot/... instead.

**Current value for `target_cpu = "arm64"`:** `true`

From //products/bringup.gni:11

**Overridden from the default:** `false`

From //build/images/args.gni:14

**Current value for `target_cpu = "x64"`:** `true`

From //products/bringup.gni:11

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

From //build/images/args.gni:67

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

From //out/not-default/args.gn:3

**Overridden from the default:** `"qemu-arm64"`

From //build/info/info.gni:12

**Current value for `target_cpu = "x64"`:** `"x64"`

From //out/not-default/args.gn:3

**Overridden from the default:** `"x64"`

From //build/info/info.gni:12

### build_info_product
Product configuration of the current build

**Current value for `target_cpu = "arm64"`:** `"bringup"`

From //out/not-default/args.gn:4

**Overridden from the default:** `""`

From //build/info/info.gni:9

**Current value for `target_cpu = "x64"`:** `"bringup"`

From //out/not-default/args.gn:4

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

From //boards/arm64.gni:47

**Overridden from the default:** `false`

From //build/images/args.gni:29

**Current value for `target_cpu = "x64"`:** `true`

From //boards/x64.gni:51

**Overridden from the default:** `false`

From //build/images/args.gni:29

### build_usb_installer
Generate installer disk image (ISO) to be flashed to a USB drive.
Will be located at obj/build/installer_images/ relative to the build directory.
See https://fuchsia.dev/fuchsia-src/development/hardware/installer

**Current value (from the default):** `false`

From //build/images/args.gni:34

### cache_package_labels
If you add package labels to this variable, the packages will be included
in the 'cache' package set, which represents an additional set of software
that is made available on disk immediately after paving and in factory
flows. These packages are updated with an OTA, and can also be updated
ephemerally. This cache of software can be evicted by the system if storage
pressure arises or other policies indicate.

**Current value for `target_cpu = "arm64"`:** `[]`

From //out/not-default/args.gn:11

**Overridden from the default:** `[]`

From //BUILD.gn:55

**Current value for `target_cpu = "x64"`:** `[]`

From //out/not-default/args.gn:11

**Overridden from the default:** `[]`

From //BUILD.gn:55

### camera_debug

**Current value (from the default):** `false`

From //src/camera/debug.gni:6

### camera_gym_configuration_cycle_interval_ms

**Current value (from the default):** `10000`

From //src/camera/bin/camera-gym/BUILD.gn:11

### camera_gym_enable_root_presenter

**Current value (from the default):** `false`

From //src/camera/bin/camera-gym/BUILD.gn:12

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

From //build/images/args.gni:124

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

From //build/images/args.gni:83

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

### clang_ml_inliner
Controls whether to use the ML inliner in Clang to reduce size.

**Current value (from the default):** `true`

From //build/config/BUILD.gn:32

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

**Current value (from the default):** `false`

From //build/rust/config.gni:61

### clippy_force_warn
Force the lint level for all clippy lints to "warn".
Note: this overrides both source attributes and our default lint levels, and
should only be used to collect stats about clippy lints in our source tree.

**Current value (from the default):** `false`

From //build/rust/config.gni:58

### clippy_warn
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

From //build/images/args.gni:127

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

From [//third_party/openweave-core/config.gni:32](https://fuchsia.googlesource.com/third_party/openweave-core/+/46e560622906834c111bfea1186755a729ac31e4/config.gni#32)

### core_realm_package_name
The following arguments are all used to configure the contents of the core
component realm. See //src/sys/core/build/core.gni for documentation on what
each field means.
TODO: redo comments

**Current value (from the default):** `"core-generic"`

From //build/product.gni:30

### core_realm_restrict_persistent_storage

**Current value (from the default):** `true`

From //build/product.gni:32

### core_realm_shards

**Current value for `target_cpu = "arm64"`:** `["//src/sys/appmgr:appmgr_core_shard", "//src/sys/core:core_proxy_shard"]`

From //products/bringup.gni:52

**Overridden from the default:** `[]`

From //build/product.gni:31

**Current value for `target_cpu = "x64"`:** `["//src/sys/appmgr:appmgr_core_shard", "//src/sys/core:core_proxy_shard"]`

From //products/bringup.gni:52

**Overridden from the default:** `[]`

From //build/product.gni:31

### crash_diagnostics_dir
Clang crash reports directory path. Use empty path to disable altogether.

**Current value (from the default):** `"//out/not-default/clang-crashreports"`

From //build/config/clang/crash_diagnostics.gni:7

### crashpad_dependencies

**Current value (from the default):** `"fuchsia"`

From [//third_party/crashpad/build/crashpad_buildconfig.gni:22](https://fuchsia.googlesource.com/third_party/crashpad/+/ba94c46703fe45d0832778bb9f747f0d7ee9dff1/build/crashpad_buildconfig.gni#22)

### crashpad_http_transport_impl

**Current value (from the default):** `"libcurl"`

From [//third_party/crashpad/util/net/tls.gni:21](https://fuchsia.googlesource.com/third_party/crashpad/+/ba94c46703fe45d0832778bb9f747f0d7ee9dff1/util/net/tls.gni#21)

### crashpad_use_boringssl_for_http_transport_socket

**Current value (from the default):** `true`

From [//third_party/crashpad/util/net/tls.gni:30](https://fuchsia.googlesource.com/third_party/crashpad/+/ba94c46703fe45d0832778bb9f747f0d7ee9dff1/util/net/tls.gni#30)

### cts_version
Name of the CTS version.

Used to change the dependency paths of CTS deps.

Example of path change:
  //zircon/system/ulib/zxtest -> //prebuilt/cts/${cts_version}/pkg/zxtest

Usage:
  `fx set PRODUCT.BOARD --args cts_version='"version_name"'`

**Current value (from the default):** `""`

From //sdk/cts/build/cts_version.gni:16

### current_cpu

**Current value (from the default):** `""`

### current_os

**Current value (from the default):** `""`

### cursor_pointer_path
Path to file to use for pointer

**Current value (from the default):** `"//src/session/bin/cursor/data/pointer.riv"`

From //src/session/bin/cursor/cursor_args.gni:7

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

### custom_signing_script_tools
If `custom signing script` is not empty, a list of host tool labels, without
a toolchain, that the script depends on. The reason why these are not in
`custom_signing_script_deps` is because these definitions are typically in
board-specific .gni files where `host_os` or `host_toolchain` are not
defined yet. Because these are imported from `args.gn` before `BUILDCONFIG.gn`
is actually parsed.

**Current value (from the default):** `[]`

From //build/images/custom_signing.gni:21

### custom_vulkan_loader_library_name

**Current value (from the default):** `""`

From [//third_party/Vulkan-Loader/BUILD.gn:22](https://fuchsia.googlesource.com/third_party/Vulkan-Loader/+/37ddb9eec895e48acfabfff82796ccd0f558bd15/BUILD.gn#22)

### cxx_rbe_enable
Set to true to enable distributed compilation of C++ using RBE.
Enabling this takes precedence over `use_goma`.

**Current value for `target_cpu = "arm64"`:** `false`

From //out/not-default/args.gn:5

**Overridden from the default:** `false`

From //build/toolchain/rbe.gni:82

**Current value for `target_cpu = "x64"`:** `false`

From //out/not-default/args.gn:5

**Overridden from the default:** `false`

From //build/toolchain/rbe.gni:82

### cxx_rbe_exec_strategy
One of:

  * "remote": Execute action remotely on cache miss.
        The remote cache is always updated with this result.

  * "local": Lookup action in the remote cache, but execute action
        locally on cache miss.  The locally produced result is
        not uploaded to the remote cache.
  (There are other rewrapper options that are not exposed.)

**Current value (from the default):** `"remote"`

From //build/toolchain/rbe.gni:93

### dart_aot_debug_build_cfg
Builds the component in a non-product AOT build. This will
launch the vm service in the runner.
This configuration is not compatible with a --release build since the
profile aot runner is built without asserts.

**Current value (from the default):**
```
{
  enable_asserts = true
  is_aot = true
  is_debug = true
  is_product = false
  platform_name = "dart_runner"
  runner_dep = "//src/dart:dart_aot_runner"
  runtime_meta = "//build/dart/meta/aot_runtime.cmx"
  runtime_meta_v2 = "//build/dart/meta/aot_runtime.cml"
}
```

From //build/dart/dart_build_config.gni:23

### dart_debug_build_cfg
Builds the component in a non-product JIT build. This will
launch the vm service in the runner.

**Current value (from the default):**
```
{
  enable_asserts = true
  is_aot = false
  is_debug = true
  is_product = false
  platform_name = "dart_runner"
  runner_dep = "//src/dart:dart_jit_runner"
  runtime_meta = "//build/dart/meta/jit_runtime.cmx"
  runtime_meta_v2 = "//build/dart/meta/jit_runtime.cml"
}
```

From //build/dart/dart_build_config.gni:8

### dart_default_build_cfg
Non-product AOT

**Current value (from the default):**
```
{
  enable_asserts = false
  is_aot = true
  is_debug = false
  is_product = false
  platform_name = "dart_runner"
  runner_dep = "//src/dart:dart_aot_runner"
  runtime_meta = "//build/dart/meta/aot_runtime.cmx"
  runtime_meta_v2 = "//build/dart/meta/aot_runtime.cml"
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
```
{
  enable_asserts = false
  is_aot = true
  is_debug = false
  is_product = false
  platform_name = "dart_runner"
  runner_dep = "//src/dart:dart_aot_runner"
  runtime_meta = "//build/dart/meta/aot_runtime.cmx"
  runtime_meta_v2 = "//build/dart/meta/aot_runtime.cml"
}
```

From //build/dart/dart_build_config.gni:36

### dart_release_build_cfg
Builds the component in a product AOT build. This will
not launch the vm service in the runner.

**Current value (from the default):**
```
{
  enable_asserts = false
  is_aot = true
  is_debug = false
  is_product = true
  platform_name = "dart_runner"
  runner_dep = "//src/dart:dart_aot_product_runner"
  runtime_meta = "//build/dart/meta/aot_product_runtime.cmx"
  runtime_meta_v2 = "//build/dart/meta/aot_product_runtime.cml"
}
```

From //build/dart/dart_build_config.gni:49

### data_filesystem_format
Set to one of "minfs", "fxfs", "f2fs" (unstable).
If set to anything other than "minfs", any existing minfs partition will be
migrated in-place to the specified format when fshost mounts it.

**Current value (from the default):** `"minfs"`

From //src/storage/fshost/generated_fshost_config.gni:34

### data_sharing_oobe_enabled
Whether or not to provide the data sharing consent step in OOBE

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

### disable_dart_strict_deps
Enable all strict deps.

**Current value (from the default):** `false`

From //build/dart/dart_library.gni:18

### disable_kernel_pci
Disable kernel PCI driver support. A counterpart of the the build
flag platform_enable_user_pci in //src/devices/bus/drivers/pci/pci.gni.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:69

### dwarf_version
Explicitly specify DWARF version used.

**Current value (from the default):** `5`

From //build/config/compiler.gni:66

### emu_window_size_height

**Current value (from the default):** `false`

From //build/product.gni:56

### emu_window_size_width
Configuration to override the default window size for the virtual device in pixels.

**Current value (from the default):** `false`

From //build/product.gni:55

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

From //build/config/BUILD.gn:24

### enable_grpc_ares
Compiles with ares.

**Current value (from the default):** `false`

From [//third_party/grpc/BUILD.gn:13](https://fuchsia.googlesource.com/third_party/grpc/+/53d69cc581c5b7305708587f4f1939278477c28a/BUILD.gn#13)

### enable_lock_dep
Enable kernel lock dependency tracking.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:35

### enable_lock_dep_tests
Enable kernel lock dependency tracking tests.  By default this is
enabled when tracking is enabled, but can also be eanbled independently
to assess whether the tests build and *fail correctly* when lockdep is
disabled.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:121

### enable_mdns_trace
Enables the tracing feature of mdns, which can be turned on using
"mdns-util verbose".

**Current value (from the default):** `false`

From //src/connectivity/network/mdns/service/BUILD.gn:13

### enable_netboot
Whether to build the netboot zbi by default.

You can still build //build/images:netboot explicitly even if enable_netboot is false.

**Current value (from the default):** `false`

From //build/images/args.gni:77

### enable_virtual_heap
Enables the use of a virtually managed kernel heap instead of one managed
directly out of the physmap. The virtual heap may have some performance and
memory usage overheads, but will not exhaust due to fragmentation.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:93

### ermine_app_entries
Build arg that allows overriding the default set of application entries
using '--args=ermine_app_entries="config/app_launch_entries.json"'

**Current value (from the default):** `"config/app_launch_entries.json"`

From //src/experiences/session_shells/ermine/shell/BUILD.gn:16

### ermine_start_screensaver
Whether or not to launch screensaver.

**Current value (from the default):** `false`

From //src/experiences/session_shells/ermine/shell/BUILD.gn:19

### ermine_user_feedback_enabled
Whether or not to allow user feedback report from the device.

**Current value (from the default):** `false`

From //src/experiences/session_shells/ermine/shell/BUILD.gn:22

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

From //zircon/vdso/vdso.gni:7

### expat_build_root

**Current value (from the default):** `"//third_party/expat"`

From //src/graphics/lib/magma/gnbuild/magma.gni:14

### experimental_cxx_version
**NOTE:** This is for **experimentation only** and should not normally be
changed.  Set the version of the C++ standard to compile for, 17 or 20.

**Current value (from the default):** `17`

From //build/config/BUILD.gn:29

### experimental_wlan_client_mlme
Selects the SoftMAC client implementation to use. Choices:
  false (default) - C++ Client MLME implementation
  true - Rust Client MLME implementation
This argument is temporary until Rust MLME is ready to be used.

**Current value (from the default):** `false`

From //src/connectivity/wlan/lib/mlme/cpp/BUILD.gn:12

### extra_package_labels

**Current value (from the default):** `[]`

From //third_party/cobalt/BUILD.gn:10

### extra_variants
Additional variant toolchain configs to support.
This is just added to [`known_variants`](#known_variants).

**Current value (from the default):** `[]`

From //build/config/BUILDCONFIG.gn:1503

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

### extract_minfs_metadata_on_corruption
If extract_minfs_metadata_on_corruption is true, fshost extracts minfs metadata on finding it
corrupted. Setting this flag to true helps debugging corruptions.

**Current value (from the default):** `false`

From //src/storage/fshost/generated_fshost_config.gni:29

### fastboot_product

**Current value (from the default):** `""`

From //build/images/args.gni:107

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

From //build/images/args.gni:48

### firmware_prebuilts_path_suffix
Suffix to append to all `firmware_prebuilts` `path` variables.

Typically this indicates the hardware revision, and is made available so
that users can easily switch revisions using a single arg.

**Current value (from the default):** `""`

From //build/images/args.gni:54

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
```
{
  enable_asserts = true
  is_aot = true
  is_debug = true
  is_product = false
  platform_name = "flutter_runner"
  runner_dep = "//src/flutter:flutter_aot_runner"
  runtime_meta = "//build/flutter/meta/aot_runtime.cmx"
  runtime_meta_v2 = "//build/flutter/meta/aot_runtime.cml"
}
```

From //build/flutter/flutter_build_config.gni:27

### flutter_debug_build_cfg
Builds the component in a non-product JIT build. This will
launch the vm service in the runner.

**Current value (from the default):**
```
{
  enable_asserts = true
  is_aot = false
  is_debug = true
  is_product = false
  platform_name = "flutter_runner"
  runner_dep = "//src/flutter:flutter_jit_runner"
  runtime_meta = "//build/flutter/meta/jit_runtime.cmx"
  runtime_meta_v2 = "//build/flutter/meta/jit_runtime.cml"
}
```

From //build/flutter/flutter_build_config.gni:12

### flutter_default_build_cfg
Non-product AOT

**Current value (from the default):**
```
{
  enable_asserts = false
  is_aot = true
  is_debug = false
  is_product = false
  platform_name = "flutter_runner"
  runner_dep = "//src/flutter:flutter_aot_runner"
  runtime_meta = "//build/flutter/meta/aot_runtime.cmx"
  runtime_meta_v2 = "//build/flutter/meta/aot_runtime.cml"
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
```
{
  enable_asserts = false
  is_aot = true
  is_debug = false
  is_product = false
  platform_name = "flutter_runner"
  runner_dep = "//src/flutter:flutter_aot_runner"
  runtime_meta = "//build/flutter/meta/aot_runtime.cmx"
  runtime_meta_v2 = "//build/flutter/meta/aot_runtime.cml"
}
```

From //build/flutter/flutter_build_config.gni:40

### flutter_release_build_cfg
Builds the component in a product AOT build. This will
not launch the vm service in the runner.

**Current value (from the default):**
```
{
  enable_asserts = false
  is_aot = true
  is_debug = false
  is_product = true
  platform_name = "flutter_runner"
  runner_dep = "//src/flutter:flutter_aot_product_runner"
  runtime_meta = "//build/flutter/meta/aot_product_runtime.cmx"
  runtime_meta_v2 = "//build/flutter/meta/aot_product_runtime.cml"
}
```

From //build/flutter/flutter_build_config.gni:53

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

From //src/storage/fshost/generated_fshost_config.gni:25

### fshost_watch_for_nand
Make fshost watch for NAND devices.

**Current value (from the default):** `false`

From //src/storage/fshost/generated_fshost_config.gni:40

### fuchsia_async_trace_level_logging
Determines whether the fuchsia_async library used by many Rust targets will be compiled
with TRACE level log statements that increase binary size a measurable amount.
TODO(fxbug.dev/80742) move this to a toolchain to allow multiple products to build together

**Current value (from the default):** `true`

From //build/product.gni:37

### fuchsia_product_assembly_config_file
Used to provide assembly with a complete product assembly config.  This can
be a static source file, the output of a build action, or a file created by
GN using generated_file().

**Current value (from the default):** `false`

From //build/product.gni:42

### fuchsia_product_assembly_config_label
If the above file is created by a target in GN, then the label that creates
it needs to be specified as well.

**Current value (from the default):** `false`

From //build/product.gni:46

### fuchsia_route_sources_config
An optional file path to the route_sources verifier configuration to be used
on the assembled fuchsia system.

**Current value (from the default):** `""`

From //build/security.gni:85

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
An optional lit of golden files for fuchsia.zbi static pkgs list. If
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

From //build/security.gni:81

### fuchsia_verify_component_resolvers_allowlist

**Current value (from the default):** `"//src/security/policy/component_resolvers_policy.json5"`

From //build/security.gni:114

### fuchsia_verify_routes_component_tree_config
An optional component tree configuration file used to finalize dynamic
elements of the component tree constructed for route verification. When
non-empty, this value is passed as the `--component-tree-config` option
to `ffx scrutiny verify routes` to verify routes in the fuchsia component
tree.

**Current value (from the default):** `""`

From //build/security.gni:107

### fuchsia_verify_routes_exceptions_allowlist

**Current value (from the default):** `"//src/security/policy/build/verify_routes_exceptions_allowlist.json5"`

From //build/security.gni:93

### fuchsia_verify_routes_exceptions_allowlist_bootfs
Same as above, except this allowlist gets added in bootfs_only builds.

**Current value (from the default):** `"//src/security/policy/build/verify_routes_exceptions_allowlist_bootfs.json5"`

From //build/security.gni:96

### fuchsia_verify_routes_exceptions_allowlist_product
Same as above, except these allowlists get added according to
product-specific configuration.

**Current value (from the default):** `[]`

From //build/security.gni:100

### fuchsia_zbi_bootfs_filelist_goldens
An optional lit of golden files for fuchsia.zbi bootFS file list. If
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

From //build/security.gni:51

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

From //build/security.gni:21

### futex_tracing_enabled
Enables kernel tracing of futex interactions

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:51

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

From //build/images/fvm.gni:96

### fvm_ftl_nand_block_count

**Current value (from the default):** `false`

From //build/images/fvm.gni:89

### fvm_ftl_nand_oob_size

**Current value (from the default):** `false`

From //build/images/fvm.gni:87

### fvm_ftl_nand_page_size
Specifying these variables will generate a NAND FVM image suitable for
directly flashing via fastboot. The NAND characteristics are required
in order to properly initialize the FTL metadata in the OOB area.
`fvm_max_disk_size` should also be nonzero or else minfs will not have any
room to initialize on boot.

**Current value (from the default):** `false`

From //build/images/fvm.gni:86

### fvm_ftl_nand_pages_per_block

**Current value (from the default):** `false`

From //build/images/fvm.gni:88

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

From //build/images/args.gni:105

### fvm_reserved_slices
Number of slices reserved by FVM for internal usage. A reservation
partition will be added to the FVM image, containing this many slices.
If set to 0, then no reservation partition will be added.

**Current value (from the default):** `0`

From //build/images/fvm.gni:36

### fvm_slice_size
The size of the FVM partition images "slice size". The FVM slice size is a
minimum size of a particular chunk of a partition that is stored within
FVM. A very small slice size may lead to decreased throughput. A very large
slice size may lead to wasted space. The selected default size of 8mb is
selected for conservation of space, rather than performance.

**Current value (from the default):** `8388608`

From //build/images/fvm.gni:31

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

From //sdk/cts/plasa/config.gni:8

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

**Current value (from the default):** `"/b/s/w/ir/x/w/fuchsia/out/not-default/dartlang/.gocache"`

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

From //build/images/args.gni:72

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

From [//third_party/grpc/BUILD.gn:21](https://fuchsia.googlesource.com/third_party/grpc/+/53d69cc581c5b7305708587f4f1939278477c28a/BUILD.gn#21)

### hangcheck_timeout_ms
Set this to accommodate long running tests

**Current value (from the default):** `0`

From //src/graphics/drivers/msd-intel-gen/src/BUILD.gn:9

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

**Current value for `target_cpu = "arm64"`:** `["//tools/devshell:fx", "//tools/bindc:host"]`

From //out/not-default/args.gn:12

**Overridden from the default:** `[]`

From //BUILD.gn:78

**Current value for `target_cpu = "x64"`:** `["//tools/devshell:fx", "//tools/bindc:host"]`

From //out/not-default/args.gn:12

**Overridden from the default:** `[]`

From //BUILD.gn:78

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

### icu_disable_thin_archive
If true, compile icu into a standalone static library. Currently this is
only useful on Chrome OS.

**Current value (from the default):** `false`

From //third_party/icu/config.gni:17

### icu_fuchsia_override_data_dir
If set to nonempty, this is the label of the directory to be used to pull
the ICU data files content.  The setting has effect only when building
inside the Fuchsia source tree.

**Current value (from the default):** `""`

From //third_party/icu/config.gni:22

### icu_major_version_number
Contains the major version number of the ICU library, for dependencies that
need different configuration based on the library version. Currently this
is only useful in Fuchsia.

**Current value (from the default):** `"71"`

From //third_party/icu/version.gni:9

### icu_use_data_file
Tells icu to load an external data file rather than rely on the icudata
being linked directly into the binary.

**Current value (from the default):** `true`

From //third_party/icu/config.gni:8

### icu_use_stub_data
If true, then this creates a stub data file. This should be disabled if
a custom data file will be used instead, in order to avoid conflicting
symbols.

**Current value (from the default):** `true`

From //third_party/icu/config.gni:13

### include_account_in_fvm
Include an account partition in the FVM image if set to true.

**Current value (from the default):** `false`

From //build/images/args.gni:139

### include_clippy
Turns rust targets into a group with both the normal target and clippy target. This
causes clippy targets to get included in the build by default.

**Current value (from the default):** `false`

From //build/rust/config.gni:65

### include_fvm_blob_sparse
Include fvm.blob.sparse.blk image into the build if set to true

**Current value (from the default):** `false`

From //build/images/args.gni:136

### include_internal_fonts
Set to true to include internal fonts in the build.

**Current value (from the default):** `false`

From //src/fonts/build/font_args.gni:7

### include_shell_commands_package
Include the shell commands package.  Used as a parameter to
assembled_system().  See documentation there.

**Current value (from the default):** `false`

From //build/images/args.gni:157

### include_zbi_host_tests
Set this variable to true to include zbi_tests into tests.json.

**Current value (from the default):** `false`

From //build/testing/zbi_test.gni:13

### include_zxdb_large_tests
Normally these tests are not built and run because they require large amounts of optional data
be downloaded. Set this to true to enable the build for the zxdb_large_tests.
See symbols/test_data/README.md for how to download the data required for this test.

**Current value (from the default):** `false`

From //src/developer/debug/zxdb/BUILD.gn:12

### inet_config_enable_async_dns_sockets
Tells inet to support additionally support async dns sockets.

**Current value (from the default):** `true`

From [//third_party/openweave-core/config.gni:17](https://fuchsia.googlesource.com/third_party/openweave-core/+/46e560622906834c111bfea1186755a729ac31e4/config.gni#17)

### inet_want_endpoint_dns
Tells inet to include support for the corresponding protocol.

**Current value (from the default):** `true`

From [//third_party/openweave-core/config.gni:10](https://fuchsia.googlesource.com/third_party/openweave-core/+/46e560622906834c111bfea1186755a729ac31e4/config.gni#10)

### inet_want_endpoint_raw

**Current value (from the default):** `true`

From [//third_party/openweave-core/config.gni:11](https://fuchsia.googlesource.com/third_party/openweave-core/+/46e560622906834c111bfea1186755a729ac31e4/config.gni#11)

### inet_want_endpoint_tcp

**Current value (from the default):** `true`

From [//third_party/openweave-core/config.gni:12](https://fuchsia.googlesource.com/third_party/openweave-core/+/46e560622906834c111bfea1186755a729ac31e4/config.gni#12)

### inet_want_endpoint_tun

**Current value (from the default):** `true`

From [//third_party/openweave-core/config.gni:14](https://fuchsia.googlesource.com/third_party/openweave-core/+/46e560622906834c111bfea1186755a729ac31e4/config.gni#14)

### inet_want_endpoint_udp

**Current value (from the default):** `true`

From [//third_party/openweave-core/config.gni:13](https://fuchsia.googlesource.com/third_party/openweave-core/+/46e560622906834c111bfea1186755a729ac31e4/config.gni#13)

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

From //out/not-default/args.gn:6

**Overridden from the default:** `true`

From //build/config/BUILDCONFIG.gn:24

**Current value for `target_cpu = "x64"`:** `false`

From //out/not-default/args.gn:6

**Overridden from the default:** `true`

From //build/config/BUILDCONFIG.gn:24

### jtrace_enabled
Please refer to https://fuchsia.dev/fuchsia-src/development/debugging/jtrace
for a description of these configuration options.

Note that the special value "auto" is used only by the default definitions
of the entries (below).  It acts as a special value which automatically
chooses a default based on whether or not JTRACE is configured for
persistent tracing, while still allowing a user to explicitly override the
value regardless of whether persistent tracing is enabled or not.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:110

### jtrace_last_entry_storage

**Current value (from the default):** `0`

From //zircon/kernel/params.gni:111

### jtrace_target_buffer_size

**Current value (from the default):** `"auto"`

From //zircon/kernel/params.gni:112

### jtrace_use_large_entries

**Current value (from the default):** `"auto"`

From //zircon/kernel/params.gni:113

### kernel_base

**Current value (from the default):** `"0xffffffff00000000"`

From //zircon/kernel/params.gni:29

### kernel_debug_level
Enables various kernel debugging and diagnostic features.  Valid
values are between 0-3.  The higher the value, the more that are
enabled.  A value of 0 disables all of them.

TODO(fxbug.dev/41790): This value is derived from assert_level.  Decouple
the two and set kernel_debug_level independently.

**Current value (from the default):** `2`

From //zircon/kernel/params.gni:81

### kernel_debug_print_level
Controls the verbosity of kernel dprintf messages. The higher the value,
the more dprintf messages emitted. Valid values are 0-2 (inclusive):
  0 - CRITCAL / ALWAYS
  1 - INFO
  2 - SPEW

**Current value (from the default):** `2`

From //zircon/kernel/params.gni:88

### kernel_extra_defines
Extra macro definitions for kernel code, e.g. "DISABLE_KASLR",
"ENABLE_KERNEL_LL_DEBUG".

**Current value (from the default):** `[]`

From //zircon/kernel/params.gni:73

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
```
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
  tags = ["instrumented", "coverage", "llvm-profdata"]
}, {
  configs = ["//build/config/profile:coverage-rust"]
  tags = ["instrumented", "coverage", "llvm-profdata"]
}, {
  configs = ["//build/config/profile"]
  tags = ["instrumented", "profile", "llvm-profdata"]
}, {
  configs = ["//build/config/profile:coverage-cts"]
  tags = ["instrumented", "coverage", "llvm-profdata"]
}, {
  configs = ["//build/config/sanitizers:tsan"]
  tags = ["instrumentation-runtime", "instrumented", "kernel-excluded", "tsan"]
}, {
  configs = ["//build/config/sanitizers:hwasan"]
  tags = ["hwasan", "instrumentation-runtime", "instrumented", "kernel-excluded"]
}, {
  configs = ["//build/config/sanitizers:ubsan"]
  remove_common_configs = ["//build/config:no_rtti"]
  tags = ["instrumented", "instrumentation-runtime", "kernel-excluded", "ubsan"]
}, {
  configs = ["//build/config/sanitizers:ubsan", "//build/config/sanitizers:sancov"]
  remove_common_configs = ["//build/config:no_rtti"]
  tags = ["instrumented", "instrumentation-runtime", "kernel-excluded", "sancov", "ubsan"]
}, {
  configs = ["//build/config/sanitizers:asan"]
  host_only = {
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
}
  remove_common_configs = ["//build/config:default_frame_pointers"]
  tags = ["asan", "instrumentation-runtime", "instrumented", "lsan", "replaces-allocator", "kernel-excluded"]
  toolchain_args = { }
}, {
  configs = ["//build/config/sanitizers:asan", "//build/config/sanitizers:ubsan"]
  host_only = {
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
}
  remove_common_configs = ["//build/config:default_frame_pointers", "//build/config:no_rtti"]
  tags = ["asan", "instrumentation-runtime", "instrumented", "lsan", "replaces-allocator", "kernel-excluded", "ubsan"]
  toolchain_args = { }
}, {
  configs = ["//build/config/sanitizers:asan", "//build/config/sanitizers:sancov"]
  host_only = {
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
}
  remove_common_configs = ["//build/config:default_frame_pointers"]
  tags = ["asan", "instrumentation-runtime", "instrumented", "lsan", "replaces-allocator", "kernel-excluded", "sancov"]
  toolchain_args = { }
}, {
  configs = ["//build/config/sanitizers:asan", "//build/config/zircon:no_safestack"]
  host_only = {
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
}
  name = "kasan"
  remove_common_configs = []
  tags = ["asan", "instrumentation-runtime", "instrumented", "lsan", "replaces-allocator", "kernel-only"]
  toolchain_args = { }
}, {
  configs = ["//build/config/sanitizers:asan", "//build/config/zircon:no_safestack", "//build/config/sanitizers:sancov"]
  host_only = {
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
}
  name = "kasan-sancov"
  remove_common_configs = []
  tags = ["asan", "instrumentation-runtime", "instrumented", "lsan", "replaces-allocator", "kernel-only", "sancov"]
  toolchain_args = { }
}, {
  configs = ["//build/config/sanitizers:asan", "//build/config/fuzzer", "//build/config/sanitizers:rust-asan", "//build/config:icf"]
  host_only = {
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
}
  name = "asan-fuzzer"
  remove_common_configs = ["//build/config:default_frame_pointers", "//build/config:icf"]
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
  tags = ["asan", "instrumentation-runtime", "instrumented", "lsan", "replaces-allocator", "kernel-excluded", "fuzzer"]
  toolchain_args = {
  asan_default_options = "alloc_dealloc_mismatch=0:check_malloc_usable_size=0:detect_odr_violation=0:max_uar_stack_size_log=16:print_scariness=1:allocator_may_return_null=1:detect_leaks=0:detect_stack_use_after_return=1:malloc_context_size=128:print_summary=1:print_suppressions=0:strict_memcmp=0:symbolize=0"
}
}, {
  configs = ["//build/config/fuzzer", "//build/config/sanitizers:ubsan", "//build/config:icf"]
  name = "ubsan-fuzzer"
  remove_common_configs = ["//build/config:icf", "//build/config:no_rtti"]
  remove_shared_configs = ["//build/config:symbol_no_undefined"]
  tags = ["fuzzer", "instrumented", "instrumentation-runtime", "ubsan"]
}, {
  name = "gcc"
  tags = ["gcc"]
}]
```

From //build/config/BUILDCONFIG.gn:1325

### launch_basemgr_on_boot
Indicates whether to start basemgr.cmx on boot.

**Current value (from the default):** `false`

From //src/modular/build/args.gni:7

### legacy_base_driver_package_labels

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/bringup.gni:57

**Overridden from the default:** `[]`

From //BUILD.gn:39

**Current value for `target_cpu = "x64"`:** `[]`

From //products/bringup.gni:57

**Overridden from the default:** `[]`

From //BUILD.gn:39

### legacy_base_package_labels

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/bringup.gni:60

**Overridden from the default:** `[]`

From //BUILD.gn:47

**Current value for `target_cpu = "x64"`:** `[]`

From //products/bringup.gni:60

**Overridden from the default:** `[]`

From //BUILD.gn:47

### legacy_cache_package_labels

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/bringup.gni:63

**Overridden from the default:** `[]`

From //BUILD.gn:56

**Current value for `target_cpu = "x64"`:** `[]`

From //products/bringup.gni:63

**Overridden from the default:** `[]`

From //BUILD.gn:56

### legacy_host_labels

**Current value (from the default):** `[]`

From //BUILD.gn:79

### legacy_product_bootfs_labels
A list of binary labels to include in ZBIs built for this product.
product_bootfs_labels = []  (defined in product.gni)

**Current value (from the default):** `[]`

From //BUILD.gn:69

### legacy_product_system_image_deps
A list of binary labels to include in the system_image package.
product_system_image_deps = []  (defined in product.gni)

**Current value (from the default):** `[]`

From //BUILD.gn:73

### legacy_universe_package_labels

**Current value for `target_cpu = "arm64"`:** `["//tools/net/device-finder:host", "//build/images/tools:fastboot"]`

From //products/bringup.gni:66

**Overridden from the default:** `[]`

From //BUILD.gn:65

**Current value for `target_cpu = "x64"`:** `["//tools/net/device-finder:host", "//build/images/tools:fastboot"]`

From //products/bringup.gni:66

**Overridden from the default:** `[]`

From //BUILD.gn:65

### linux_runner_extras_tests
If `true`, adds additional testonly content to extras.img, which will be
built and mounted inside the container at /mnt/chromeos.

**Current value (from the default):** `true`

From //src/virtualization/bin/linux_runner/BUILD.gn:13

### linux_runner_stateful_image_path
Point this to the location of a prebuilt stateful image in QCOW2 format

**Current value (from the default):** `""`

From //src/virtualization/bin/linux_runner/BUILD.gn:24

### linux_runner_user_extras
Point this to the location of external files to be included as extras

**Current value (from the default):** `[]`

From //src/virtualization/bin/linux_runner/BUILD.gn:21

### linux_runner_volatile_block
If `true`, all block devices that would normally load as READ_WRITE will
be loaded as VOLATILE_WRITE. This is useful when working on changes to
the linux kernel as crashes and panics can sometimes corrupt the images.

**Current value (from the default):** `false`

From //src/virtualization/bin/linux_runner/BUILD.gn:18

### local_bench
Used to enable local benchmarking/fine-tuning when running benchmarks
in `fx shell`. Pass `--args=local_bench='true'` to `fx set` in order to
enable it.

**Current value (from the default):** `false`

From //src/developer/fuchsia-criterion/BUILD.gn:13

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

**Current value (from the default):** `false`

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

From //boards/arm64.gni:39

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

From //boards/arm64.gni:40

**Overridden from the default:** `0`

From //build/images/filesystem_limits.gni:39

**Current value (from the default):** `0`

From //build/images/filesystem_limits.gni:39

### mbedtls_config_file
Configuration file for MbedTLS.

**Current value (from the default):** `"mbedtls-config.h"`

From [//third_party/openthread/third_party/mbedtls/BUILD.gn:30](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/third_party/mbedtls/BUILD.gn#30)

### meta_package_labels
A list of labels for packages that are appended to the set of base packages,
but depend on all the other base, cache, and universe packages, therefore
they must be separated into their own list.

**Current value for `target_cpu = "arm64"`:** `[]`

From //products/bringup.gni:54

**Overridden from the default:** `[]`

From //build/images/args.gni:88

**Current value for `target_cpu = "x64"`:** `[]`

From //products/bringup.gni:54

**Overridden from the default:** `[]`

From //build/images/args.gni:88

### min_crashlog_size
Controls minimum amount of space of persistent RAM to reserve for the
crashlog.  When other features (such as persistent debug logging) are
enabled, this value controls the minimum number of bytes which will
_always_ be reserved for the crashlog (subject to the total amount of
available persistent RAM), regardless of how much ram is requested by other
users of persistent RAM.  Must be a multiple of 128 bytes.

**Current value (from the default):** `2048`

From //zircon/kernel/lib/crashlog/params.gni:14

### minfs_board_maximum_bytes

**Current value (from the default):** `false`

From //build/images/fvm.gni:76

### minfs_board_minimum_data_bytes

**Current value (from the default):** `false`

From //build/images/fvm.gni:63

### minfs_maximum_runtime_bytes
minfs_maximum_runtime_bytes is an upper bound on the partition size on the device. Partitions
can grow as needed if there are extra slices available in FVM. This limit prevents the minfs
partition from taking too much space away from other uses.

Pass the empty string for no limit.

**Current value (from the default):** `""`

From //src/storage/fshost/generated_fshost_config.gni:20

### minfs_product_maximum_bytes

**Current value (from the default):** `false`

From //build/images/fvm.gni:77

### minfs_product_minimum_data_bytes

**Current value (from the default):** `false`

From //build/images/fvm.gni:64

### mini_chromium_is_chromeos_ash

**Current value (from the default):** `false`

From //third_party/mini_chromium/build/platform.gni:31

### mini_chromium_is_chromeos_lacros

**Current value (from the default):** `false`

From //third_party/mini_chromium/build/platform.gni:30

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

From //src/bringup/bin/netsvc/BUILD.gn:21

### omaha_app_id
Default app id will always return no update.

**Current value (from the default):** `"fuchsia-test:no-update"`

From //src/sys/pkg/bin/omaha-client/BUILD.gn:14

### openthread_config_anycast_locator_enable
Enable anycast locator functionality

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:82](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#82)

### openthread_config_assert_enable
Enable assertions.

**Current value (from the default):** `true`

From [//third_party/openthread/etc/gn/openthread.gni:79](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#79)

### openthread_config_backbone_router_enable
Enable backbone router functionality

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:85](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#85)

### openthread_config_border_agent_enable
Enable border agent support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:88](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#88)

### openthread_config_border_router_enable
Enable border router support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:91](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#91)

### openthread_config_border_routing_enable
Enable border routing support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:94](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#94)

### openthread_config_channel_manager_enable
Enable channel manager support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:97](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#97)

### openthread_config_channel_monitor_enable
Enable channel monitor support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:100](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#100)

### openthread_config_child_supervision_enable
Enable child supervision support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:103](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#103)

### openthread_config_coap_api_enable
Enable coap api support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:106](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#106)

### openthread_config_coap_observe_api_enable
Enable coap observe (RFC7641) api support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:112](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#112)

### openthread_config_coap_secure_api_enable
Enable secure coap api support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:109](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#109)

### openthread_config_coexistence_enable
Enable radio coexistence

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:239](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#239)

### openthread_config_commissioner_enable
Enable commissioner support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:115](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#115)

### openthread_config_deps
Extra deps for OpenThread configuration.

**Current value (from the default):** `[]`

From [//third_party/openthread/etc/gn/openthread.gni:38](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#38)

### openthread_config_dhcp6_client_enable
Enable DHCP6 client support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:121](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#121)

### openthread_config_dhcp6_server_enable
Enable DHCP6 server support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:124](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#124)

### openthread_config_diag_enable
Enable diagnostic support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:127](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#127)

### openthread_config_dns_client_enable
Enable DNS client support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:130](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#130)

### openthread_config_dnssd_server_enable
Enable DNS-SD server support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:133](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#133)

### openthread_config_dua_enable
Enable Domain Unicast Address feature for Thread 1.2

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:139](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#139)

### openthread_config_ecdsa_enable
Enable ECDSA support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:136](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#136)

### openthread_config_enable_builtin_mbedtls_management

**Current value (from the default):** `true`

From [//third_party/openthread/etc/gn/openthread.gni:236](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#236)

### openthread_config_file
OpenThread config header.

**Current value (from the default):** `"<openthread-config-fuchsia.h>"`

From [//third_party/openthread/etc/gn/openthread.gni:35](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#35)

### openthread_config_full_logs
Enable full logs

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:223](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#223)

### openthread_config_heap_external_enable
Enable external heap support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:145](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#145)

### openthread_config_ip6_fragmentation_enable
Enable ipv6 fragmentation support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:148](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#148)

### openthread_config_ip6_slaac_enable
Enable support for adding of auto-configured SLAAC addresses by OpenThread

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:202](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#202)

### openthread_config_jam_detection_enable
Enable jam detection support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:151](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#151)

### openthread_config_joiner_enable
Enable joiner support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:154](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#154)

### openthread_config_legacy_enable
Enable legacy network support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:157](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#157)

### openthread_config_link_metrics_initiator_enable
Enable link metrics initiator

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:160](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#160)

### openthread_config_link_metrics_subject_enable
Enable link metrics subject

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:163](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#163)

### openthread_config_link_raw_enable
Enable link raw service

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:166](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#166)

### openthread_config_log_level_dynamic_enable
Enable dynamic log level control

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:169](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#169)

### openthread_config_log_output
Log output: none, debug_uart, app, platform

**Current value (from the default):** `""`

From [//third_party/openthread/etc/gn/openthread.gni:76](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#76)

### openthread_config_mac_csl_receiver_enable
Enable csl receiver

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:118](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#118)

### openthread_config_mac_filter_enable
Enable mac filter support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:172](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#172)

### openthread_config_message_use_heap
Enable use built-in heap for message buffers

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:175](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#175)

### openthread_config_mle_long_routes_enable
Enable MLE long routes extension (experimental, breaks Thread conformance]

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:178](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#178)

### openthread_config_mlr_enable
Enable Multicast Listener Registration feature for Thread 1.2

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:142](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#142)

### openthread_config_multiple_instance_enable
Enable multiple instances

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:184](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#184)

### openthread_config_ncp_hdlc_enable
Enable NCP HDLC support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:232](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#232)

### openthread_config_ncp_spi_enable
Enable NCP SPI support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:229](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#229)

### openthread_config_otns_enable
Enable OTNS support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:226](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#226)

### openthread_config_ping_sender
Enable ping sender support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:214](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#214)

### openthread_config_platform_netif_enable
Enable platform netif support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:187](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#187)

### openthread_config_platform_udp_enable
Enable platform UDP support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:190](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#190)

### openthread_config_reference_device_enable
Enable Thread Test Harness reference device support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:193](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#193)

### openthread_config_sntp_client_enable
Enable SNTP Client support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:205](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#205)

### openthread_config_srp_client_enable
Enable SRP Client support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:208](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#208)

### openthread_config_srp_server_enable
Enable SRP Server support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:211](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#211)

### openthread_config_thread_version
Thread version: 1.1, 1.2

**Current value (from the default):** `"1.2"`

From [//third_party/openthread/etc/gn/openthread.gni:73](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#73)

### openthread_config_time_sync_enable
Enable the time synchronization service feature

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:217](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#217)

### openthread_config_tmf_netdata_service_enable
Enable support for injecting Service entries into the Thread Network Data

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:196](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#196)

### openthread_config_tmf_network_diag_mtd_enable
Enable TMF network diagnostics on MTDs

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:181](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#181)

### openthread_config_udp_forward_enable
Enable UDP forward support

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:220](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#220)

### openthread_core_config_deps
Extra deps for OpenThread core configuration.

**Current value (from the default):** `[]`

From [//third_party/openthread/etc/gn/openthread.gni:50](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#50)

### openthread_core_config_platform_check_file
OpenThread platform-specific config check header

**Current value (from the default):** `""`

From [//third_party/openthread/etc/gn/openthread.gni:47](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#47)

### openthread_enable_core_config_args
Configure OpenThread via GN arguments.

**Current value (from the default):** `true`

From [//third_party/openthread/etc/gn/openthread.gni:67](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#67)

### openthread_external_mbedtls
Use external mbedtls. If blank, internal mbedtls will be used.

**Current value (from the default):** `""`

From [//third_party/openthread/etc/gn/openthread.gni:56](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#56)

### openthread_external_platform
Use external platform.

**Current value (from the default):** `""`

From [//third_party/openthread/etc/gn/openthread.gni:53](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#53)

### openthread_package_name
Package name for OpenThread.

**Current value (from the default):** `"OPENTHREAD"`

From [//third_party/openthread/etc/gn/openthread.gni:59](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#59)

### openthread_package_version
Package version for OpenThread.

**Current value (from the default):** `"1.0.0"`

From [//third_party/openthread/etc/gn/openthread.gni:62](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#62)

### openthread_project_core_config_file
OpenThread project-specific core config header

**Current value (from the default):** `""`

From [//third_party/openthread/etc/gn/openthread.gni:44](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#44)

### openthread_project_include_dirs
Include directories for project specific configs.

**Current value (from the default):** `[]`

From [//third_party/openthread/etc/gn/openthread.gni:41](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#41)

### openthread_settings_ram
Enable volatile-only storage of settings

**Current value (from the default):** `false`

From [//third_party/openthread/etc/gn/openthread.gni:199](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/etc/gn/openthread.gni#199)

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

From //products/bringup.gni:78

**Overridden from the default:** `[]`

From //build/packages/prebuilt_package_with_flavors.gni:29

**Current value for `target_cpu = "x64"`:** `[]`

From //products/bringup.gni:78

**Overridden from the default:** `[]`

From //build/packages/prebuilt_package_with_flavors.gni:29

### persistent_ram_allocation_granularity
Controls the granularity of allocation of the global pool of persistent RAM.
All features which wish to use persistent RAM to preserve data across reboot
must operate in allocations which are a multiple of this value.  The value
should be a power of two, and typically should be a multiple of the
cacheline size of the target architecture.

**Current value (from the default):** `128`

From //zircon/kernel/params.gni:100

### platform_enable_user_pci

**Current value (from the default):** `false`

From //src/devices/bus/drivers/pci/pci.gni:10

### pmm_checker_from_board
Used to control whether board definitions include PMM checker options.

**Current value (from the default):** `true`

From //boards/kernel_cmdline/BUILD.gn:39

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

From //build/images/args.gni:108

### prebuilt_dart_sdk
Directory containing prebuilt Dart SDK.
This must have in its `bin/` subdirectory `gen_snapshot.OS-CPU` binaries.

**Current value (from the default):** `"//prebuilt/third_party/dart/linux-x64"`

From //build/dart/dart.gni:8

### prebuilt_libvulkan_img_path
The path to a prebuilt libvulkan.so for an IMG GPU.

**Current value (from the default):** `""`

From //src/graphics/lib/magma/gnbuild/magma.gni:24

### product_bootfs_labels
A list of binary labels to include in ZBIs built for this product.

**Current value for `target_cpu = "arm64"`:** `["//build/info:bootfs", "//bundles:bootstrap", "//bundles:debugging", "//bundles:diagnostics-eng", "//bundles/bringup:manual_testing", "//bundles/drivers:bootstrap", "//bundles/drivers:bootstrap-eng", "//bundles/drivers:usb-host-stack", "//bundles/drivers:usb-peripheral-stack", "//bundles/drivers:utils", "//src/diagnostics/archivist:default-service-config", "//products/kernel_cmdline:blobfs.cache-eviction-policy--NEVER_EVICT", "//products/kernel_cmdline:console.shell--true", "//products/kernel_cmdline:kernel.enable-debugging-syscalls--true", "//products/kernel_cmdline:kernel.enable-serial-syscalls--true", "//products/kernel_cmdline:kernel.oom.behavior--jobkill", "//products/kernel_cmdline:netsvc.all-features--true", "//products/kernel_cmdline:netsvc.disable--false", "//products/kernel_cmdline:oom.reboot-timeout--low", "//src/testing/runtests", "//src/sys/component_manager:component_manager_bootfs_config"]`

From //products/bringup.gni:47

**Overridden from the default:** `[]`

From //build/product.gni:11

**Current value for `target_cpu = "x64"`:** `["//build/info:bootfs", "//bundles:bootstrap", "//bundles:debugging", "//bundles:diagnostics-eng", "//bundles/bringup:manual_testing", "//bundles/drivers:bootstrap", "//bundles/drivers:bootstrap-eng", "//bundles/drivers:usb-host-stack", "//bundles/drivers:usb-peripheral-stack", "//bundles/drivers:utils", "//src/diagnostics/archivist:default-service-config", "//products/kernel_cmdline:blobfs.cache-eviction-policy--NEVER_EVICT", "//products/kernel_cmdline:console.shell--true", "//products/kernel_cmdline:kernel.enable-debugging-syscalls--true", "//products/kernel_cmdline:kernel.enable-serial-syscalls--true", "//products/kernel_cmdline:kernel.oom.behavior--jobkill", "//products/kernel_cmdline:netsvc.all-features--true", "//products/kernel_cmdline:netsvc.disable--false", "//products/kernel_cmdline:oom.reboot-timeout--low", "//src/testing/runtests", "//src/sys/component_manager:component_manager_bootfs_config"]`

From //products/bringup.gni:47

**Overridden from the default:** `[]`

From //build/product.gni:11

### product_bootfs_packages
A list of packages to be included in the bootfs as
meta.fars and content-id'd blobs.

**Current value (from the default):** `[]`

From //build/product.gni:21

### product_build
This is a product build (vs. sdk).  If a product is set (fx set <product>.)
this is true (should be set in bringup.gni)

**Current value for `target_cpu = "arm64"`:** `true`

From //products/bringup.gni:7

**Overridden from the default:** `false`

From //BUILD.gn:93

**Current value for `target_cpu = "x64"`:** `true`

From //products/bringup.gni:7

**Overridden from the default:** `false`

From //BUILD.gn:93

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
If [`profile_source_files_list_files`](#profile_source_files_list_files) is
also set, both lists are used.  Use that option with a file constructed
separately instead to use more complex selection features such as
per-function selection, file name patterns, or exclusion.

**Current value (from the default):** `["//*"]`

From //build/config/profile/config.gni:11

### profile_source_files_list_files
List GN path to files in Clang's `-fprofile-list` format describing files
and functions to be instrumented by `profile` variants.  Note that if
[`profile_source_files`](#profile_source_files) is also set, both are used.

**Current value (from the default):** `[]`

From //build/config/profile/config.gni:20

### recovery_label
Allows a product to specify the recovery image used in the zirconr slot.
Default recovery image is zedboot. Overriding this value will keep zedboot
in the build but will not include it as the default zirconr image.
Recovery images can provide an update target by specifying the metadata item
"update_target" in the format <target>=<path>. (Such as `update_target =
[ "recovery=" + rebase_path(recovery_path, root_build_dir) ]`)
Example value: "//build/images/recovery"

**Current value (from the default):** `"//build/images/zedboot"`

From //build/images/args.gni:148

### recovery_logo_path
Path to file to use for recovery logo

**Current value (from the default):** `"//src/recovery/system/res/fuchsia-logo.riv"`

From //src/recovery/system/system_recovery_args.gni:7

### recovery_only
This is really a build for a recovery image, and so the fuchsia image that
is being built isn't properly configured, and so just disable the new image
assembly work until that's been addressed.

**Current value (from the default):** `false`

From //build/images/args.gni:19

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

From //build/security.gni:66

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

From //build/security.gni:36

### restat_cc
Set to true to make C++ compiles preserve timestamps of unchanged outputs.

**Current value (from the default):** `false`

From //build/toolchain/restat.gni:16

### restat_rust
Set to true to make Rust compiles preserve timestamps of unchanged outputs.

**Current value (from the default):** `false`

From //build/toolchain/restat.gni:13

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

From //build/config/rust/BUILD.gn:32

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

From //build/toolchain/rbe.gni:74

### rust_rbe_enable
Set to true to enable distributed compilation of Rust using RBE.

**Current value for `target_cpu = "arm64"`:** `false`

From //out/not-default/args.gn:7

**Overridden from the default:** `false`

From //build/toolchain/rbe.gni:37

**Current value for `target_cpu = "x64"`:** `false`

From //out/not-default/args.gn:7

**Overridden from the default:** `false`

From //build/toolchain/rbe.gni:37

### rust_rbe_exec_strategy
One of:

  * "remote": Execute action remotely on cache miss.
        The remote cache is always updated with this result.

  * "local": Lookup action in the remote cache, but execute action
        locally on cache miss.  The locally produced result is
        not uploaded to the remote cache.
  (There are other rewrapper options that are not exposed.)

**Current value (from the default):** `"remote"`

From //build/toolchain/rbe.gni:48

### rust_toolchain_triple_suffix
Sets the fuchsia toolchain target triple suffix (after arch)

**Current value (from the default):** `"fuchsia"`

From //build/rust/config.gni:44

### rust_v0_symbol_mangling
Controls whether the rust compiler uses v0 symbol mangling scheme
(see https://github.com/rust-lang/rfcs/blob/HEAD/text/2603-rust-symbol-name-mangling-v0.md).
The v0 symbol mangling scheme requires upstream LLVM support when demangling,
so it is not on by default.
TODO(fxbug.dev/57302): Enable v0 mangling by default.

**Current value (from the default):** `false`

From //build/config/rust/BUILD.gn:28

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

**Current value (from the default):** `"lEoZLfP0meAggQFcXgqpLlmhX43Txz-JzLwgi3KRXyMC"`

From //build/rust/config.gni:32

### scenic_display_frame_number
Draws the current frame number in the top-left corner.

**Current value (from the default):** `false`

From //src/ui/scenic/lib/gfx/BUILD.gn:12

### scenic_enable_vulkan_validation
Include the vulkan validation layers in scenic.

**Current value (from the default):** `false`

From //src/ui/scenic/lib/gfx/build_args.gni:7

### scenic_ignore_vsync

**Current value (from the default):** `false`

From //src/ui/scenic/lib/gfx/BUILD.gn:9

### scheduler_queue_tracing_enabled
Enables scheduler queue tracing for trace-based scheduler performance
analysis.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:48

### scheduler_tracing_level
The level of detail for scheduler traces when enabled. Values greater than
zero add increasing details at the cost of increased trace buffer use.

0 = Default kernel:sched tracing.
1 = Adds duration traces for key scheduler operations.
2 = Adds flow events from wakeup to running state.
3 = Adds detailed internal durations and probes.

**Current value (from the default):** `0`

From //zircon/kernel/params.gni:44

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

From //products/bringup.gni:80

**Overridden from the default:** `[]`

From //BUILD.gn:89

**Current value for `target_cpu = "x64"`:** `[]`

From //products/bringup.gni:80

**Overridden from the default:** `[]`

From //BUILD.gn:89

### sdk_id
Identifier for the Core SDK.

**Current value (from the default):** `""`

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

From //build/config/BUILDCONFIG.gn:1746

### select_variant_canonical
*This should never be set as a build argument.*
It exists only to be set in `toolchain_args`.
See //build/toolchain/clang_toolchain.gni for details.

**Current value (from the default):** `[]`

From //build/config/BUILDCONFIG.gn:1751

### select_variant_shortcuts
List of short names for commonly-used variant selectors.  Normally this
is not set as a build argument, but it serves to document the available
set of short-cut names for variant selectors.  Each element of this list
is a scope where `.name` is the short name and `.select_variant` is a
a list that can be spliced into [`select_variant`](#select_variant).

**Current value (from the default):**
```
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

From //build/config/BUILDCONFIG.gn:1549

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

### smp_max_cpus

**Current value (from the default):** `16`

From //zircon/kernel/params.gni:21

### spinel_platform_header
Platform portability header for spinel.

**Current value (from the default):** `"\"spinel_platform.h\""`

From [//third_party/openthread/src/lib/spinel/BUILD.gn:32](https://fuchsia.googlesource.com/third_party/openthread/+/3cd0ddaff845aba03bee94bc6672ebe7c6f7758d/src/lib/spinel/BUILD.gn#32)

### starnix_wayland_protocol_logging
Whether wayland protocol logging should be enabled

**Current value (from the default):** `false`

From //src/proc/bin/starnix/BUILD.gn:12

### target_cpu

**Current value for `target_cpu = "arm64"`:** `"arm64"`

From //out/not-default/args.gn:8

**Overridden from the default:** `""`

**Current value for `target_cpu = "x64"`:** `"x64"`

From //out/not-default/args.gn:8

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

### termina_kernel
The termina kernel image.

Defaults to the common linux kernel image from CIPD, but can be overridden to use a
custom kernel for development purposes.

**Current value (from the default):** `"//prebuilt/virtualization/packages/termina_guest/kernel/arm64/vm_kernel"`

From //src/virtualization/packages/termina_guest/BUILD.gn:12

### termina_tools
The termina tools disk image.

Defaults to the disk image from CIPD, but can be overridden to use a
custom disk for development purposes.

**Current value (from the default):** `"//prebuilt/virtualization/packages/termina_guest/images/arm64/vm_tools.img"`

From //src/virtualization/packages/termina_guest/BUILD.gn:24

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

From //BUILD.gn:85

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

From //build/security.gni:142

### thinlto_cache_dir
ThinLTO cache directory path.

**Current value (from the default):** `"dartlang/thinlto-cache"`

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
See //docs/concepts/build_system/internals/toolchains/build_arguments.md#toolchain_variant
for details and documentation for each field.

**Current value (from the default):**
```
{
  base = "//build/toolchain/fuchsia:arm64"
}
```

From //build/config/BUILDCONFIG.gn:100

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
```
[{
  configs = []
  name = "debug"
  toolchain_args = {
  is_debug = true
}
}]
```

From //build/config/BUILDCONFIG.gn:1533

### universe_package_labels
If you add package labels to this variable, the packages will be included
in the 'universe' package set, which represents all software that is
produced that is to be published to a package repository or to the SDK by
the build. The build system ensures that the universe package set includes
the base and cache package sets, which means you do not need to redundantly
include those labels in this variable.

**Current value for `target_cpu = "arm64"`:** `["//bundles:kitchen_sink"]`

From //out/not-default/args.gn:13

**Overridden from the default:** `[]`

From //BUILD.gn:64

**Current value for `target_cpu = "x64"`:** `["//bundles:kitchen_sink"]`

From //out/not-default/args.gn:13

**Overridden from the default:** `[]`

From //BUILD.gn:64

### universe_resolver_enable_subpackages
Whether to allow universe-resolver to resolve subpackages.
TODO(fxbug.dev/102652): This configuration will be removed when subpackages
is generally available.

**Current value (from the default):** `false`

From //build/security.gni:161

### update_kernels
(deprecated) List of kernel images to include in the update (OTA) package.
If no list is provided, all built kernels are included. The names in the
list are strings that must match the filename to be included in the update
package.

**Current value (from the default):** `[]`

From //build/images/args.gni:40

### use_bringup_assembly
Is the `assemble_system()` instantiation used by the product the standard
one or the bringup one?

**Current value for `target_cpu = "arm64"`:** `true`

From //products/bringup.gni:10

**Overridden from the default:** `false`

From //build/product.gni:8

**Current value for `target_cpu = "x64"`:** `true`

From //products/bringup.gni:10

**Overridden from the default:** `false`

From //build/product.gni:8

### use_cast_runner_canary
Whether to use the most recent (canary) version of the CastRunner prebuilt.
Otherwise, the qualified "release" version is used.
Set [`use_chromium_canary`](#use_chromium_canary) to the same value.

**Current value (from the default):** `false`

From //src/chromium/build_args.gni:23

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

From //src/chromium/build_args.gni:18

### use_direct_for_carnelian_examples
Include a config in the example packages to attempt to use view mode
direct.

**Current value (from the default):** `false`

From //src/lib/ui/carnelian/BUILD.gn:30

### use_flatland_by_default
If true, Flatland is the default graphics protocol in Scenic.

**Current value (from the default):** `false`

From //src/ui/scenic/build_args.gni:7

### use_gigaboot
Build the gigaboot bootloader.

**Current value for `target_cpu = "arm64"`:** `true`

From //boards/arm64.gni:48

**Overridden from the default:** `false`

From //build/images/args.gni:26

**Current value for `target_cpu = "x64"`:** `true`

From //boards/x64.gni:52

**Overridden from the default:** `false`

From //build/images/args.gni:26

### use_goma
Set to true to enable distributed compilation using Goma.
This has lower precedence than `use_reclient_cxx` in
//build/toolchain/rbe.gni.

**Current value for `target_cpu = "arm64"`:** `false`

From //out/not-default/args.gn:9

**Overridden from the default:** `false`

From //build/toolchain/goma.gni:13

**Current value for `target_cpu = "x64"`:** `false`

From //out/not-default/args.gn:9

**Overridden from the default:** `false`

From //build/toolchain/goma.gni:13

### use_lto
Use link time optimization (LTO).

**Current value (from the default):** `false`

From //build/config/lto/config.gni:7

### use_modern_input_injection
Set this to true when configuring gn args to use the modern input injection
protocol `fuchsia.input.injection.InputDeviceRegistry`. If not set, input-synthesis
defaults to connect to `fuchsia.ui.input.InputDeviceRegistry`.

**Current value (from the default):** `false`

From //src/lib/ui/input-synthesis/BUILD.gn:14

### use_netstack3
DO NOT SET THIS IN A PRODUCT DEFINITION!!  FOR NETSTACK DEVELOPER USE ONLY
TODO(https://fxbug.dev/85450) - Convert this to a platform configuration
option in Product Assembly

**Current value (from the default):** `false`

From //src/connectivity/network/BUILD.gn:12

### use_null_vulkan_on_host
TODO(liyl): Currently non-x64 platforms don't have Vulkan support,
so we always use the null Vulkan implementation instead.

Global arguments for whether we use a "null" Vulkan implementation on
host vulkan_executables and vulkan_tests, so that any attempt to create a
VkInstances or VkDevice will fail.

This argument will affect all vulkan_{executable/test} build targets.


**Current value (from the default):** `false`

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

Or, if already importing a different manifest from there, resulting in
errors from jiri update, it can work to just git clone (but jiri update
won't manage third_party/ffmpeg in this case):

```
mkdir third_party/ffmpeg
git clone "https://fuchsia.googlesource.com/third_party/ffmpeg" third_party/ffmpeg
```

**Current value (from the default):** `true`

From //src/media/lib/ffmpeg/BUILD.gn:28

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

### use_vbmeta
If true, then a vbmeta image will be generated for provided ZBI
and the paving script will pave vbmeta images to the target device.

**Current value for `target_cpu = "arm64"`:** `true`

From //boards/arm64.gni:49

**Overridden from the default:** `false`

From //build/images/vbmeta.gni:14

**Current value for `target_cpu = "x64"`:** `true`

From //boards/x64.gni:50

**Overridden from the default:** `false`

From //build/images/vbmeta.gni:14

### use_vboot
Use vboot images

**Current value (from the default):** `false`

From //build/images/args.gni:10

### userspace_pci_enable_msi_x

**Current value (from the default):** `false`

From //src/devices/bus/drivers/pci/pci.gni:11

### using_fuchsia_sdk
Only set in buildroots where targets configure themselves for use with the
Fuchsia SDK

**Current value (from the default):** `false`

From //build/fuchsia/sdk.gni:8

### vbmeta_a_partition

**Current value (from the default):** `""`

From //build/images/args.gni:102

### vbmeta_b_partition

**Current value (from the default):** `""`

From //build/images/args.gni:103

### vbmeta_r_partition

**Current value (from the default):** `""`

From //build/images/args.gni:104

### vboot_keys
vboot signing key directory. Must contain `kernel.keyblock` and
`kernel_data_key.vbprivk`. Defaults to the public ChromeOS test keys.

**Current value (from the default):** `"//third_party/vboot_reference/tests/devkeys"`

From //build/images/vboot/vboot.gni:15

### vboot_verbose
If true, vboot() image builds print out the exact "futility" command line.

**Current value (from the default):** `false`

From //build/images/vboot/vboot.gni:11

### vendor_linting
Whether libraries under //vendor should be linted.

**Current value (from the default):** `false`

From //build/fidl/fidl_library.gni:16

### verbose_image_assembly
Enable verbose output from `ffx assembly image`, this creates non-silent
build output and therefore should never be 'true' in checked-in configs, and
is meant solely for developer debugging.

**Current value (from the default):** `false`

From //build/images/args.gni:153

### virtcon_bold_font_path
Path to file to use for bold font

**Current value (from the default):** `""`

From //src/bringup/bin/virtcon/virtcon_args.gni:11

### virtcon_bold_italic_font_path
Path to file to use for bold italic font

**Current value (from the default):** `""`

From //src/bringup/bin/virtcon/virtcon_args.gni:17

### virtcon_boot_animation_path

**Current value (from the default):** `"//src/bringup/bin/virtcon/data/boot-animation.riv"`

From //src/bringup/bin/virtcon/virtcon_args.gni:24

### virtcon_fallback_font_paths
Paths to files to use for fallback fonts

**Current value (from the default):** `[]`

From //src/bringup/bin/virtcon/virtcon_args.gni:20

### virtcon_font_path

**Current value (from the default):** `"//prebuilt/third_party/fonts/robotomono/RobotoMono-Regular.ttf"`

From //src/bringup/bin/virtcon/virtcon_args.gni:8

### virtcon_italic_font_path
Path to file to use for italic font

**Current value (from the default):** `""`

From //src/bringup/bin/virtcon/virtcon_args.gni:14

### virtio_vsock_inprocess

**Current value (from the default):** `true`

From //src/virtualization/bin/vmm/BUILD.gn:9

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

From //build/product.gni:52

### vm_tracing_level
The level of detail for traces emitted by the VM system. Values greater than
zero add increasing details at the cost of increased trace buffer use.

0 = Default kernel:* tracing.
1 = Adds flow events for asynchronous page requests.
2 = Adds duration events related to accessed faults and page faults.
3 = Adds duration events for PMM allocations and frees.

**Current value (from the default):** `0`

From //zircon/kernel/params.gni:65

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

From //src/graphics/examples/vkproto/common/common.gni:47

### wait_queue_depth_tracing_enabled
Enables tracing of wait queue depths.  Used for post-processing analysis of
how deep wait queues tend to be under various loads, as well as how
frequently the change depth.

**Current value (from the default):** `false`

From //zircon/kernel/params.gni:56

### warn_on_sdk_changes
Whether to only warn when an SDK has been modified.
If false, any unacknowledged SDK change will cause a build failure.

**Current value (from the default):** `false`

From //build/sdk/config.gni:11

### wayland_bridge_protocol_logging
Whether protocol logging should be enabled

**Current value (from the default):** `false`

From //src/ui/wayland/bin/bridge/BUILD.gn:13

### wayland_server_fatal_object_lookup_failures
Enable this to make object lookup failures fatal for debugging.

**Current value (from the default):** `false`

From //src/lib/ui/wayland/server/BUILD.gn:12

### weave_build_legacy_wdm
Tells openweave to support legacy WDM mode.

**Current value (from the default):** `false`

From [//third_party/openweave-core/config.gni:29](https://fuchsia.googlesource.com/third_party/openweave-core/+/46e560622906834c111bfea1186755a729ac31e4/config.gni#29)

### weave_build_warm
Tells openweave to build WARM libraries.

**Current value (from the default):** `true`

From [//third_party/openweave-core/config.gni:26](https://fuchsia.googlesource.com/third_party/openweave-core/+/46e560622906834c111bfea1186755a729ac31e4/config.gni#26)

### weave_system_config_use_sockets
Tells openweave components to use bsd-like sockets.

**Current value (from the default):** `true`

From [//third_party/openweave-core/config.gni:7](https://fuchsia.googlesource.com/third_party/openweave-core/+/46e560622906834c111bfea1186755a729ac31e4/config.gni#7)

### weave_with_nlfaultinjection
Tells openweave components to support fault injection.

**Current value (from the default):** `false`

From [//third_party/openweave-core/config.gni:20](https://fuchsia.googlesource.com/third_party/openweave-core/+/46e560622906834c111bfea1186755a729ac31e4/config.gni#20)

### weave_with_verhoeff
Tells openweave to support Verhoeff checksum.

**Current value (from the default):** `true`

From [//third_party/openweave-core/config.gni:23](https://fuchsia.googlesource.com/third_party/openweave-core/+/46e560622906834c111bfea1186755a729ac31e4/config.gni#23)

### with_live_usb
Whether or not to include the live_usb component in the build.

**Current value (from the default):** `false`

From //src/sys/live_usb/BUILD.gn:15

### wlancfg_config_type
Selects the wlan configuration type to use. Choices:
  "client" - client mode
  "ap" - access point mode
  "" (empty string) - no configuration

**Current value (from the default):** `"client"`

From //src/connectivity/wlan/wlancfg/BUILD.gn:18

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

### zircon_a_partition
Arguments to `fx flash` script (along with any `firmware_prebuilts` which
specify a partition).

If `fvm_partition` is provided, the flash script will flash the full OS,
recovery + Zircon + FVM + SSH keys. In this case, the bootloader must also
support `fastboot oem add-staged-bootloader-file ssh.authorized_keys`.

Otherwise, the script will flash the recovery image to all slots, which
doesn't require the FVM or SSH keys.

**Current value (from the default):** `""`

From //build/images/args.gni:99

### zircon_asserts

**Current value (from the default):** `false`

From //build/config/fuchsia/BUILD.gn:136

### zircon_b_partition

**Current value (from the default):** `""`

From //build/images/args.gni:100

### zircon_compdb_filter
Compilation database filter. Gets passed to --export-compile-commands=<filter>.

**Current value (from the default):** `"default"`

From //build/zircon/build_args.gni:16

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

From //build/images/args.gni:101

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

From //build/config/BUILDCONFIG.gn:117

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

### build_libvulkan_goldfish

**Current value (from the default):** `"//third_party/android/device/generic/goldfish-opengl:libvulkan_goldfish"`

From //src/graphics/lib/goldfish-vulkan/gnbuild/BUILD.gn:11

### deprecated_x86_legacy_boot_protocol
**TODO(fxbug.dev/32255): This is a temporary switch that will be removed.**

Set this to make the ZBI compatible with older boot loaders such as a
gigaboot or zedboot image already installed on a machine that's hard to
update.  This is an interim workaround only for people who have machines
that are not physically accessible to update their boot images, and will
be removed after everyone has had a chance to get hold of their machines.

**Current value (from the default):** `false`

From //zircon/kernel/BUILD.gn:29

### i_can_haz_atlas_camera
If true, power on the Atlas camera at boot.
TODO(fxbug.dev/81684): remove once we have a better way to manage ACPI device power.

**Current value (from the default):** `false`

From //src/devices/board/drivers/x86/BUILD.gn:15

