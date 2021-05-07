# Everything between power on and your component

This document aims to detail, at a high-level, everything that happens between
machine power on and a component (v1 or v2) running on the system.

Outline:

- [Kernel](#kernel)
- [Initial processes](#initial-processes)
- [Component manager](#component-manager)
- [Initial v2 components](#v2-components)
- [Initial v1 components](#v1-components)

## Kernel {#kernel}

The process for loading the Fuchsia kernel (zircon) onto the system varies by
platform. At a high level the kernel is stored in the [ZBI][ZBI], which holds
everything needed to bootstrap Fuchsia.

[Once the kernel (zircon) is running on the system][bootloader-and-kernel] its
main objective is to start [userspace][userspace], where processes can be run.
Since zircon is a [micro kernel][micro-kernel], it doesn't have to do a whole
lot in this stage (especially compared to Linux). The executable for the first
user process is baked into the kernel, which the kernel copies into a new
process and starts. This program is called [userboot][userboot].

## Initial processes {#initial-processes}

Userboot is carefully constructed to be [easy for the kernel to
start][userboot-loading], because otherwise the kernel would have to implement a
lot of [process bootstrap functionality][process-bootstrap] (like a library
loader service) that would never be used after the first process has been
started.

Userboot’s job is really straightforward, to find and start the next process.
The kernel gives userboot a handle to the ZBI, inside of which is the
[bootfs][bootfs] image. Userboot reads through the ZBI to find the bootfs image,
decompresses it if necessary, and copies it to a fresh [VMO][vmo]. The bootfs
image contains a read-only filesystem, which userboot then accesses to find an
executable and its libraries. With these it starts the next process, which is
[bootsvc][bootsvc].

Userboot may exit at this point, unless the userboot.shutdown option was given
on the [kernel command line][kernel-command-line].

Bootsvc, the next process, is [dynamically linked][dynamic-linking] by userboot.
This makes it a better home than userboot for complex logic, as it can use
libraries. Because of this bootsvc runs various FIDL services for its children,
the most notable of which is bootfs, a [FIDL-based filesystem][fuchsia-io2]
backed by the bootfs image that userboot decompressed.

Aside from hosting various services and the bootfs filesystem, bootsvc’s main
job is to start the next process, which is [component
manager][component-manager]. Just like bootsvc, component manager is stored in
bootfs, which is still the only filesystem available at this point.

Note that all of bootsvc’s responsibilities are currently being moved to
component manager, and it will eventually be deleted from the system. After this
happens, userboot will launch component manager directly instead of bootsvc.

Both bootsvc and component manager mark their processes as
[critical][critical-processes], which means that if something goes wrong with
either and they crash, the [job][job] that they are in is killed. Both run in
the root job, which has the special property that if it is killed, the kernel
force restarts the system.

![A diagram showing that userboot comes first, then bootsvc, then component
manager](images/userboot-bootsvc-cm.png)

## Component manager {#component-manager}

[Component manager][component-manager] is the program that drives the v2
component framework. This framework controls how and when programs are run and
which capabilities these programs can access from other programs. A program run
by this framework is referred to as a [component][component].

The components that component manager runs are organized into a tree. There is a
root component, and it has two children named bootstrap and core. Bootstrap's
children are the parts of the system needed to get the system functional enough
to run more complex software like appmgr.

The root, bootstrap, and core components are non-executable components, which
means that they have no program running on the system that corresponds to them.
They exists solely for organizational purposes.

![A diagram showing that fshost and driver manager, are children of the
bootstrap component, appmgr is a child of the core component, and core and
bootstrap are children of the root component](images/v2-topology.png)

## Initial v2 components {#v2-components}

### Background

There are two important components under bootstrap, fshost and driver manager.
These two components work together to bring up a functional enough system for
appmgr, which then starts up all the user-facing software.

#### driver manager

[Driver manager][driver-manager] is the component responsible for finding
hardware, running drivers to service the hardware, and exposing a handle for
[devfs][devfs] to Fuchsia.

Drivers are run by [driver hosts][driver-host], which are child processes that
driver manager starts. Each driver is a dynamic library stored in either bootfs
or a package, and when a driver is to be run it is dynamically linked into a
driver host and then executed.

The drivers stored in packages aren't available when driver manager starts, as
those are stored on disk and drivers must be running before block devices for
filesystems can appear. driver manager starts a thread that
[waits on a synchronous open to the /system-delayed handle][wait-for-system],
and once this open call succeeds it loads the drivers in the system package.

#### fshost

Fshost is a v2 component responsible for finding block devices, starting
filesystem processes to service these block devices, and
[providing handles][fshost-exposes] for these filesystems to the rest of
Fuchsia. To accomplish this, fshost attempts to access the /dev handle in its
namespace. This capability is
[provided by driver manager][driver-manager-exposes].

As fshost finds block devices, it
[reads headers from each device][fshost-magic-headers] to detect the filesystem
type. It will initially find the [fvm][fvm] block, which contains partitions for
other block devices. Fshost will use devfs to cause driver manager to run the
fvm driver for this block device, which causes other block devices to appear for
fshost to inspect. It does a similar thing when it discovers a
[zxcrypt][zxcrypt] partition, as the disk will need to be decrypted to be
usable. Once fvm and zxcrypt are loaded, fshost will find the appropriate block
devices and start the [minfs][minfs] and [blobfs][blobfs] filesystems, which are
needed for a fully functioning system.

Currently fshost runs a [memfs][memfs] for its outgoing directory, and
[mounts][fs-mount] handles into this memfs as filesystems come online. This
means that attempting to access a fshost-provided directory too early will
result in components seeing an empty directory. The requests are not pipelined
in such a way that they are ignored until the given filesystem is available.

To work around this fshost provides two directories,
[`/pkgfs-delayed`][pkgfs-delayed] and [`/system-delayed`][system-delayed], which
do ignore requests until they are able to properly service them.

The `/pkgfs-delayed` handle is provided to component manager, which uses it to
load components that are stored in packages.

#### appmgr

[Appmgr][appmgr] runs the v1 component framework, which coexists with the v2
component framework. Appmgr is [stored in a package][appmgr-pkg], unlike fshost
and driver manager, which are stored in bootfs, so component manager uses the
/pkgfs-delayed handle from fshost to load appmgr.

Capabilities from the v2 framework can be
[forwarded to the `sys` realm][appmgr-uses] in appmgr, and services managed by
[sysmgr][sysmgr] can be [exposed to the v2 framework][appmgr-exposes]. By this
mechanism, the two frameworks can access capabilities from each other and
cooperate to run the system.

### Drivers, filesystems, and v1 components come online

Component manager generally starts components lazily on-demand in response to
something accessing a capability provided by the component. Components may also
be marked as "eager", which causes the component to start at the same point its
parent starts.

In order to get the system running, appmgr is [marked as an eager
component][appmgr-is-eager]. Since appmgr is stored in a package this causes
component manager to attempt to load appmgr, and thus access the /pkgfs-delayed
handle from fshost, causing fshost to be started.

Once running, fshost attempts to access the /dev handle from driver manager,
which causes driver manager to start. Together they bring up drivers and
filesystems, eventually culminating in pkgfs running. At this point fshost
starts responding to requests on the /pkgfs-delayed handle, and component
manager finishes loading appmgr and starts it.

![A sequence diagram showing that appmgr loading begins due to it being an eager
component, fshost starting due to the /pkgfs-delayed handle, driver manager
starting due to the /dev handle, block devices appearing, filesystems appearing,
and then appmgr successfully starting.](images/boot-sequence-diagram.png)

## Initial v1 components {#v1-components}

When appmgr is started it creates a top-level realm called the "app"
[realm][v1-realm]. Into this realm it launches the first v1 component,
[sysmgr][sysmgr]. Sysmgr’s job is to manage the "sys" realm, which is created
under the "app" realm.

The sys realm holds a large number of FIDL services, the exact set of which is
determined by [sysmgr configuration files][sysmgr-config]. Components running in
the sys realm are allowed to connect to these sysmgr-managed services. Service
connections for the sys realm are handled by sysmgr, which will lazily start
components as services they provide are needed.

There is also a set of components that sysmgr will start eagerly, each of which
may or may not also provide FIDL services for the sys realm.

![A diagram showing the app realm holding the sysmgr component and the sys
realm, and the sys realm holding other
components.](images/appmgr-realm-layout.png)

## The rest of the v1 components

With the initial set of v1 components launched, they will cause other components
to be launched through accessing FIDL services and by directly launching them
with services provided by appmgr. It is at this point that the remaining set of
components on the system can be run.

[ZBI]: /docs/glossary.md#zircon-boot-image
[appmgr-exposes]: https://fuchsia.googlesource.com/fuchsia/+/7cf46e0c7a8e5e4c78dba846f867ab96bcce5c5b/src/sys/appmgr/meta/appmgr.cml#168
[appmgr-is-eager]: https://fuchsia.googlesource.com/fuchsia/+/5a6fe7db58d2869ccfbb22caf53343d40e57c6ba/src/sys/root/meta/root.cml#14
[appmgr-pkg]: https://fuchsia.googlesource.com/fuchsia/+/5a6fe7db58d2869ccfbb22caf53343d40e57c6ba/src/sys/appmgr/BUILD.gn#159
[appmgr-uses]: https://fuchsia.googlesource.com/fuchsia/+/7cf46e0c7a8e5e4c78dba846f867ab96bcce5c5b/src/sys/appmgr/meta/appmgr.cml#40
[appmgr]: /docs/glossary.md#appmgr
[blobfs]: /docs/concepts/filesystems/blobfs.md
[bootfs]: /docs/glossary.md#bootfs
[bootloader-and-kernel]: /docs/concepts/booting/userboot.md#boot_loader_and_kernel_startup
[bootsvc]: /docs/glossary.md#bootsvc
[component-manager]: /docs/concepts/components/v2/introduction.md#component-manager
[component]: /docs/glossary.md#component
[critical-processes]: /docs/reference/syscalls/job_set_critical.md
[devfs]: /docs/concepts/drivers/device_driver_model/device-model.md
[driver-host]: /docs/glossary.md#devhost
[driver-manager-exposes]: https://fuchsia.googlesource.com/fuchsia/+/5a6fe7db58d2869ccfbb22caf53343d40e57c6ba/src/sys/root/meta/driver_manager.cml#91
[driver-manager]: /docs/glossary.md#devmgr
[dynamic-linking]: https://en.wikipedia.org/wiki/Dynamic_linker
[fs-mount]: /docs/concepts/filesystems/filesystems.md#mounting
[fshost-exposes]: https://fuchsia.googlesource.com/fuchsia/+/5a6fe7db58d2869ccfbb22caf53343d40e57c6ba/src/sys/root/meta/fshost.cml#17
[fshost-magic-headers]: https://fuchsia.googlesource.com/fuchsia/+/514f9474502cf6cafcd1d5edadfc7164566d4453/zircon/system/ulib/fs-management/mount.cc#155
[fuchsia-io2]: https://fuchsia.dev/reference/fidl/fuchsia.io2
[fvm]: /docs/glossary.md#fuchsia-volume-manager
[job]: /docs/reference/kernel_objects/job.md
[kernel-command-line]: /docs/reference/kernel/kernel_cmdline.md
[memfs]: /docs/concepts/filesystems/filesystems.md#memfs_an_in-memory_filesystem
[micro-kernel]: https://en.wikipedia.org/wiki/Microkernel
[minfs]: /docs/concepts/filesystems/minfs.md
[pkgfs-delayed]: https://fuchsia.googlesource.com/fuchsia/+/5a6fe7db58d2869ccfbb22caf53343d40e57c6ba/src/sys/root/meta/fshost.cml#18
[process-bootstrap]: /docs/concepts/booting/program_loading.md
[sysmgr-config]: https://fuchsia.googlesource.com/fuchsia/+/5a6fe7db58d2869ccfbb22caf53343d40e57c6ba/src/sys/sysmgr/sysmgr-configuration.md
[sysmgr]: https://fuchsia.googlesource.com/fuchsia/+/7cf46e0c7a8e5e4c78dba846f867ab96bcce5c5b/src/sys/sysmgr/README.md
[system-delayed]: https://fuchsia.googlesource.com/fuchsia/+/5a6fe7db58d2869ccfbb22caf53343d40e57c6ba/src/sys/root/meta/fshost.cml#19
[userboot-loading]: /docs/concepts/booting/userboot.md#kernel_loads_userboot
[userboot]: /docs/concepts/booting/userboot.md
[userspace]: https://en.wikipedia.org/wiki/User_space
[v1-realm]: /docs/glossary.md#realm
[vmo]: /docs/glossary.md#virtual-memory-object
[wait-for-system]: https://fuchsia.googlesource.com/fuchsia/+/5a6fe7db58d2869ccfbb22caf53343d40e57c6ba/src/devices/driver_manager/system-instance.cc#726
[zxcrypt]: /docs/concepts/filesystems/zxcrypt.md
