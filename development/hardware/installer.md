# Use the Fuchsia installer (Legacy)

## Overview

The `mkinstaller` script produces a bootable disk image from a build, which can be used to
install Fuchsia to a target machine. It supports creating installation images
for x64 EFI-based devices (the x64 product configuration), and coreboot-based devices
(the chromebook-x64 configuration), depending on what has been built.


## Implementation

Mkinstaller can write an image directly to a USB disk, or it can produce a new
image file on the host machine, which can then be written to install media using
`dd` or similar. The mkinstaller script determines the images to be written to
disk based on `${FUCHSIA_BUILD_DIR}/images.json`, and then writes each partition
to disk, labelled according to the “name” field of the entry in images.json.


## Use the installer

After you complete a build, you can create an installer image with
`fx mkinstaller /path/to/usb`.

Note: To see a full list of options, run `fx mkinstaller -h`.

Follow the steps below to install Fuchsia:

1. To access the installer, boot your computer or virtual machine from the USB.

  You should see a blue boot screen.

1. To access the Fuchsia shell, press alt+tab.

These are the ways you can install Fuchsia:

* [Automatic installation](#automatic_installation)
* [Manual installation](#manual_installation)

### Automatic installation

1. Run `installer` to launch the installer.

  <pre class="prettyprint">
  <code class="devsite-terminal">installer</code>
  <span class="no-select">Bootloader vendor = Intel Corp.
  Please select the disk you want to install Fuchsia to:
  [0] /dev/sys/platform/pci/00:17.0/ahci/sata2/block (119G)
  Enter a selection (0..0):
  </span>
  </pre>

  You will see a list of disks, along with their sizes.


1. Type in the number corresponding to the disk you wish to install Fuchsia on, and press ENTER.

  Note: What you type in won't be displayed until you press ENTER.

  <pre class="prettyprint">
  <span class="no-select">
  Enter a selection (0..0): 0
  Using /dev/sys/platform/pci/00:17.0/ahci/sata2/block as installation target.

  WARNING: Installing Fuchsia will WIPE YOUR DISK. Make sure you've backed
  everything up before proceeding!
  Do you wish to proceed? (yes/[no])
  </span>
  </pre>

1. Confirm that you want to continue. The installer will wipe your disk, set up
   the Fuchsia partition tables, and install Fuchsia to your disk. The installer
   will show its progress as it installs, and exit once the installation is
   complete.

   <pre class="prettyprint">
   <span class="no-select">
   Do you wish to proceed? (yes/[no]) yes
   Wiping old partition tables...
   Initializing Fuchsia partition tables...
   Success.
   Partition[src=/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-002/block, pave_type=Bootloader]... OK
   Partition[src=/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-003/block, pave_type=Asset { type: Kernel, config: A }, asset=Kernel, config=A]... OK
   Partition[src=/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-003/block, pave_type=Asset { type: Kernel, config: A }, asset=Kernel, config=A] [-B]... OK
   Partition[src=/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-004/block, pave_type=Asset { type: Kernel, config: Recovery }, asset=Kernel, config=Recovery]... OK
   Partition[src=/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-005/block, pave_type=Volume]... OK
   </span>
   </pre>

1. Unplug the installation USB drive.

1. Reboot your machine.

  <pre class="prettyprint">
  <code class="devsite-terminal">dm reboot</code>
  </pre>

### Manual installation


1. Run `lsblk` to determine the main disk of the target machine.

  <pre class="prettyprint">
  <code class="devsite-terminal">lsblk</code>
  <span class="no-select">ID  SIZE TYPE         	LABEL            	FLAGS  DEVICE
  000  28G                                   	RE 	/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block
  001  63M efi-system   	zedboot-efi      	RE 	/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-000/block
  002   1M cros-data    	efi              	RE 	/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-001/block
  003  25M cros-data    	zircon-a         	RE 	/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-002/block
  004  22M cros-data    	zircon-r         	RE 	/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-003/block
  005 321M cros-data    	storage-sparse   	RE 	/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-004/block
  006 119G                                          	/dev/sys/platform/pci/00:17.0/ahci/sata2/block
  </span>
  </pre>

  In this case, `/dev/sys/platform/pci/00:17.0/ahci/sata2/block` is the main disk of the
  target machine.

1. Run `install-disk-image` to wipe and initialize the partition tables on the
  target machine. Replace `/dev/sys/platform/pci/00:17.0/ahci/sata2/block` with the path
  you determined using the `lsblk` command.

  <pre class="prettyprint">
  <code class="devsite-terminal">install-disk-image init-partition-tables --block-device <var>/dev/sys/platform/pci/00:17.0/ahci/sata2/block</var></code>
  <span class="no-select">
  disk-pave: init-partition-tables operation succeeded.
  </span>
  </pre>

1. Run `lsblk` to confirm the state of the disks:

  <pre class="prettyprint">
  <code class="devsite-terminal">lsblk</code>
  <span class="no-select">
  ID  SIZE TYPE         	LABEL            	FLAGS  DEVICE
  000  28G                                   	RE 	/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block
  001  63M efi-system   	zedboot-efi      	RE 	/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-000/block
  002   1M cros-data    	efi              	RE 	/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-001/block
  003  25M cros-data    	zircon-a         	RE 	/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-002/block
  004  22M cros-data    	zircon-r         	RE 	/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-003/block
  005 321M cros-data    	storage-sparse   	RE 	/dev/sys/platform/pci/00:14.0/xhci/usb-bus/001/001/ifc-000/ums/lun-000/block/part-004/block
  006 119G                                          	/dev/sys/platform/pci/00:17.0/ahci/sata2/block
  052  16M efi-system   	efi-system              	/dev/sys/platform/pci/00:17.0/ahci/sata2/block/part-000/block
  053  64M zircon-a     	zircon-a                	/dev/sys/platform/pci/00:17.0/ahci/sata2/block/part-001/block
  054  64M zircon-b     	zircon-b                	/dev/sys/platform/pci/00:17.0/ahci/sata2/block/part-002/block
  055  96M zircon-r     	zircon-r                	/dev/sys/platform/pci/00:17.0/ahci/sata2/block/part-003/block
  056  64K vbmeta_a     	vbmeta_a                	/dev/sys/platform/pci/00:17.0/ahci/sata2/block/part-004/block
  057  64K vbmeta_b     	vbmeta_b                	/dev/sys/platform/pci/00:17.0/ahci/sata2/block/part-005/block
  058  64K vbmeta_r     	vbmeta_r                	/dev/sys/platform/pci/00:17.0/ahci/sata2/block/part-006/block
  059   4K misc         	misc                    	/dev/sys/platform/pci/00:17.0/ahci/sata2/block/part-007/block
  060  16G fuchsia-fvm  	fuchsia-fvm             	/dev/sys/platform/pci/00:17.0/ahci/sata2/block/part-008/block
  </span>
  </pre>

1. Install the ESP to the disk. Note that the "ID" column in lsblk
  corresponds to a file in /dev/class/block/<ID>. You should replace
  `002` with the ID of the partition that's labelled `efi`, and `052`
  with the ID of the partition that's labelled `efi-system`.


  <pre class="prettyprint">
  <code class="devsite-terminal">dd if=/dev/class/block/<var>002</var> of=/dev/class/block/<var>052</var></code>
  <span class="no-select">
  2194+0 records in
  2194+0 records out
  1123328 bytes copied
  </span>
  </pre>

1. Install the main kernel image. Replace `003` with the ID of the partition on
  the USB drive labelled `zircon-a`.

  Note: A limitation in the block device protocol means we need
  to `dd` to an intermediary file before using `install-disk-image`.

  <pre class="prettyprint">
  <code class="devsite-terminal">dd if=/dev/class/block/<var>003</var> of=/tmp/tmp.img</code>
  <span class="no-select">
  52520+0 records in
  52520+0 records out
  26890240 bytes copied
  </span>
  <code class="devsite-terminal">install-disk-image install-zircona --file /tmp/tmp.img</code>
  <span class="no-select">
  disk-pave: install-zircona operation succeeded.
  </span>
  </pre>

1. Install the recovery kernel image. Replace `004` with the ID of the partition
  on the USB drive labelled `zircon-r`.

  <pre class="prettyprint">
  <code class="devsite-terminal">dd if=/dev/class/block/<var>004</var> of=/tmp/tmp.img</code>
  <span class="no-select">
  52520+0 records in
  52520+0 records out
  26890240 bytes copied
  <code class="devsite-terminal">install-disk-image install-zirconr --file /tmp/tmp.img</code>
  <span class="no-select">
  disk-pave: install-zirconr operation succeeded.
  </span>
  </pre>

1. Install the Fuchsia volume image. Replace `005` with the ID of the partition
  on the USB drive labelled `storage-sparse`.

  Note: this image is much bigger than any of the others - this step
  may take a while!

  <pre class="prettyprint">
  <code class="devsite-terminal">dd if=/dev/class/block/<var>005</var> of=/tmp/tmp.img</code>
  <span class="no-select">
  658432+0 records in
  658432+0 records out
  337117184 bytes copied
  </span>
  <code class="devsite-terminal">install-disk-image install-fvm --file /tmp/tmp.img</code>
  <span class="no-select">
  disk-pave: install-fvm operation succeeded.
  </span>
  </pre>

1. Unplug the installation USB drive.

1. Reboot your machine.

  <pre class="prettyprint">
  <code class="devsite-terminal">dm reboot</code>
  </pre>

