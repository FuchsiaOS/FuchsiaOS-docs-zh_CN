# Bringup Product Definition

The `bringup` product is the most minimal viable target for development. It is
a minimal feature set product that is focused on being very simple and very lean.

Note that the name `bringup` build should not imply “only used during bringup of
a new platform”, the name is historical.

The bringup product serves at least these purposes:

1. Bringup: When a new platform is not running Fuchsia yet (`core` product
   configuration or higher) because all the pieces necessary to run are not
   completed/work-reliable, for instance networking, storage or configurations
   needed for fx device discovery and package management.
2. Kernel and low level driver development: Developing facilities that need to
   be working to even try a `core` product requires a bringup build. This applies
   to kernel development and drivers like networking and storage that are needed
   in `core`. Note that higher level drivers like audio also can benefit from
   bringup builds, when the drivers needed for core are not ready yet.

A bringup build has these basic features:

1. Has serial output enabled: This includes debug logging from drivers (for
   instance via zxlogf). This must guarantee that developers bringing up new
   platforms are able to printf debug as needed.
1. Is RAM loadable: It must be possible to load into RAM a bringup build in
   order of preference:
   1. For those platforms that support 'fastboot boot' it must be possible to
   implement RAM booting a ZBI directly from the bootloader (for example using
   the bootshim mechanism).
   1. For those platforms that do not support 'fastboot boot' it must be
   possible to boot using an existing zedboot (for instance loaded in a bootable
   USB stick or previously flashed) with a mechanism like netsvc (used for
   netbooting) or overnet (for instance over serial).
   1. For those platforms that do not support 'fastboot boot' (for instance when
   there is no control over the bootloader) it must also be possible to
   implement RAM booting a ZBI directly from the bootloader (for example by
   creating a bootshim for the specific bootloader).
1. Does not have dependencies on drivers not available in early bringup:
   Examples of drivers available in early bringup include interrupt controllers
   and serial port. Examples of drivers not available in early bringup include
   networking and storage.
1. Has minimum dependencies on Fuchsia at large, in that it:
   1. Has workflows driven over a serial link.
   1. Allows for everything needed in the build to be loaded alongside the
   kernel (i.e. in bootfs).
   1. Does not depend on Fuchsia features like paving that require storage.
   1. Does not support fx commands such as fx serve and fx shell. As a result,
      bringup builds are not able to add new software at runtime or upgrade
      themselves.
1. Allows for easy inclusion of additional drivers or binaries: It must be
   possible to include additional binaries and drivers in the bringup build. For
   instance through inclusion into bootfs via GN to add a driver in development
   to the build.

Note that these features do not prevent the possible expansion of the bringup
build minimal configuration to other more complete configurations that allow for
improved workflows.
