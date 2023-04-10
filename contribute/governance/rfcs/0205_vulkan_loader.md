<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0205" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC describes how software on Fuchsia loads [Vulkan][Vulkan] [ICDs][ICD]
and [layers][layers] to perform hardware-accelerated rendering.

The system in this document has already largely been implemented, but this
document may include changes to the architecture that we intend to make in the
future.

## Motivation

The Vulkan API has a C-style interface which applications use for programming
the GPU. Vulkan applications interface with Vulkan functions using the [Vulkan
loader][loaderrepo] in one of several ways as described here: [Interfacing with
Vulkan Functions][vulkaninterfacing].

In this doc, "application" is used to denote the software component that uses
the Vulkan API.

The Vulkan loader is also responsible for loading Installable Client Drivers
(ICDs) and Vulkan layers along with delegating access to [magma][magma] or other
APIs needed to execute GPU commands on behalf of an ICD.

Vulkan ICDs are vendor-specific shared libraries that are loaded into
applications to enable them to render using the GPU.  Applications that use
Vulkan need a mechanism to identify and load the correct ICD for the hardware in
the system.

Vulkan layers are shared libraries that modify or observe the behavior of the
Vulkan API by augmenting the dispatch chain of Vulkan API calls. They can be
used to enhance the functionality of Vulkan or to interpose the API on behalf of
Vulkan debugging or profiling functionality.

## Stakeholders

_Facilitator:_

rlb@

_Reviewers:_

cstout@
costan@
jhowarth@
msandy@
rosasco@
palmer@
wittrock@

_Consulted:_

_Socialization:_

The design has been reviewed by members of the Magma team. An [early
version][early-version] of this document was shared to the Component Framework
team and at Security team office hours.

## Design

On Fuchsia, the Vulkan loader is split into two parts: the
[libvulkan.so](#libvulkanso) shared library that is loaded into the application,
and a loader service ([vulkan_loader](#vulkan_loader)) that is responsible for
loading ICD VMOs and transferring them to `libvulkan.so`. They communicate using
the [fuchsia.vulkan.loader.Loader][loader-protocol] protocol.

### libvulkan.so

[Khronos][khronos] is the standards body for the Vulkan API. They provide a
[loader shared library implementation][loaderrepo] that's used on Linux,
Windows, macOS and most other platforms.  Google wrote a separate loader for use
on Android.

The Fuchsia loader is based on Khronos's implementation; the code lives at
[third_party/Vulkan-Loader][loader-source] in the Fuchsia repo, but eventually
it will all be upstreamed. When an application calls
[vkCreateInstance][create-instance] or other enumeration functions, the loader
reads [environment variables][environ] and JSON configuration files to determine
the set of ICDs and layers to use. Layers are loaded from the component's
namespace, so they're generally stored inside the package.  They may also be
loaded from directory capabilities provided to the component, provided the
loader configuration is set to use those directories.

#### ICD loading

![Vulkan loader startup
flow](/docs/contribute/governance/rfcs/resources/vulkan_loader/vulkan_loader_startup_flow.png)

On startup, `libvulkan.so` connects to the
[fuchsia.vulkan.loader.Loader][loader-protocol] protocol. This channel must
remain connected for the lifetime of the application. If it exits, all future
loader calls may fail.

This long-lived connection prevents the component framework from reloading or
updating the loader while a client that uses Vulkan is running. This is
desirable because it prevents unexpected changes to the versions of Vulkan ICDs
and loader interfaces while the application is using the loader API calls. Some
Vulkan entry-points for enumerating extensions or other instance properties
don't take any type of _context_ argument; as such, the implementation will have
some implicit global state.

![Vulkan loader
flow](/docs/contribute/governance/rfcs/resources/vulkan_loader/vulkan_loader_flow.png)

ICDs are loaded using the `fuchsia.vulkan.loader.Loader` protocol. The loader
uses the `fuchsia.vulkan.loader/Loader.ConnectToManifestFs` method to access a
filesystem with [manifest JSON files][manifest-json] describing all the relevant
ICDs; this filesystem looks the same as the `/usr/local/share/vulkan/icd.d`
filesystem on Linux; see [Filesystem serving](#filesystem-serving) for the
details of that filesystem.

The loader will then use the `fuchsia.vulkan.loader/Loader.Get` method to get
retrieve a VMO corresponding to the ICD, which it can `dlopen_vmo` to load into
the process and get the ICD entrypoints from. The set of Vulkan entrypoints on
Fuchsia is the same as that on Linux, except with Fuchsia-specific extensions as
described below.

Client components may also be packaged with software ICD implementations like
[SwiftShader][SwiftShader]. In the case of SwiftShader, the `VK_ICD_FILENAMES`
environment variable can be used to specify the path to the `manifest.json` of
the ICD. The ICD shared library will be loaded from `/pkg/lib` of the Vulkan
client component.

Since most ICDs are not stored in the package and are versioned separately from
application binaries, they can only make limited assumptions about the ABI of
the applications they're linking to. The exact interface they can rely on is
listed in [Fuchsia System Interface][ICD-abi], but in general they're only
allowed to use a limited list of symbols, which must all be from either
`libc.so` or `libzircon.so`.  When building ICDs, the imported symbols are
verified against an
[allowlist](/src/graphics/lib/magma/gnbuild/imported_symbols.allowlist) to
ensure that the ICD will be loadable against multiple versions of client
applications. In the future this allowlist may shrink as hermetic replacements
are created.

ICDs need to be able to connect to external protocols; in particular they must
connect to the underlying device drivers that communicate with hardware. They
may also want to read vendor-specific configuration files, as well as log
errors.  `libc.so` exports several symbols to perform I/O, but in practice the
underlying operations (like `open`) are implemented in `libfdio`.  In addition,
there's no way to connect a Zircon channel using the filesystem without
additional symbols that are directly exported from `libfdio`.

To allow ICDs to do limited I/O, these definitions are added to the Vulkan ICD
API:

```cpp
VkResult(VKAPI_PTR* PFN_vkOpenInNamespaceAddr)(const char* pName, uint32_t handle);
VKAPI_ATTR void VKAPI_CALL vk_icdInitializeOpenInNamespaceCallback(PFN_vkOpenInNamespaceAddr
open_in_namespace_addr);
```

The ICD should expose `vk_icdInitializeOpenInNamespaceCallback`. Before any
other driver functions are called, this function will be called with an
`open_in_namespace_addr` callback. The ICD can pass a file name and Zircon
channel client end to this callback to connect to filesystem nodes by name.

This function has access to the process's incoming namespace, so the ICD can
read configuration files or connect to services like `fuchsia.logger.LogSink` or
`fuchsia.tracing.provider.Registry`. Vulkan ICDs may contain global state, so if
the process is a runner that can host multiple child components (perhaps by
using a virtual machine or other non-process mechanism to isolate components),
the runner must ensure that services provided to an ICD are safe to use from any
child component. For example if multiple untrusted child components are
co-located in the process the runner should not route
`fuchsia.tracing.provider.Registry` through a child component that the runner
doesn't trust, since the component could snoop on all ICD graphics activity.

The `open_in_namespace_addr` callback special-cases access to the
`/loader-gpu-devices` path. All access to that path is routed to a filesystem
provided from `vulkan_loader` using the
`fuchsia.vulkan.loader/Loader.ConnectToDeviceFs` method; this allows the ICD to
connect to whatever hardware-specific device driver nodes it needs. ICDs can use
[zxio][zxio] or raw FIDL to traverse the filesystems; see [Filesystem
serving](#filesystem-serving) for the details of that filesystem.

Layers are generally distributed through the SDK and loaded from the same
package as the application, so they can rely on the same ABI guarantees of any
software that's in the SDK. Layers that are loaded through directory
capabilities from external packages should be treated the same as ICDs in terms
of ABI.

#### ICD unloading/reloading

It's currently not possible to unload shared libraries, so any ICD will remain
loaded for the lifetime of the process. To avoid memory bloat when creating a
new Vulkan instance, the loader keeps a no-expiration cache of all ICDs it has
seen (identified by shared library filename). This filename is unique for as
long as the `vulkan_loader` connection is alive.

#### Loader execution environment

The Vulkan API doesn't have the notion of an async runloop, so function calls
must complete synchronously from an application's perspective. The loader
doesn't receive an `async_dispatcher_t*` from the application, and isn't allowed
to use the default dispatcher from `libasync-default.so`. It may create its own
dispatcher and threads internally.

The outgoing directory of a component is hosted by the application's code so the
loader isn't able to put entries in it. This limits how it can interact with
other components. It's also not a platform requirement that the application will
only load a single copy of the loader, though currently all applications use a
copy from `libvulkan.so`, which is deduplicated at load time due to its soname.

The loader searches for config files by default in
`/vulkan-loader-configuration`, falling back to `/pkg/data`. These paths can be
overridden by environment variables or an [override layer][override-layer], the
same as on Linux.

### vulkan_loader

`vulkan_loader` is  a service that is responsible for determining what ICDs are
available, loading them, and serving them to applications. It's hosted at
`/core/vulkan_loader`, and the [fuchsia.vulkan.loader.Loader][loader-protocol]
service it exposes is routed to sessions, the test framework, and several
applications. It's written in C++, the code lives at
`//src/graphics/bin/vulkan_loader`, and documentation is at
[/src/graphics/bin/vulkan_loader/README.md][README].

In the future this service may be re-written in rust to reduce security risk and
take advantage of asynchronous programming features.

#### Identifying new devices

The `vulkan_loader` service must be able to identify what ICDs are usable. This
is driven by the set of device drivers that are running. If a device driver
isn't running for the hardware, then the ICD associated with it isn't usable.

`vulkan_loader` uses directory watchers on `/dev/class/goldfish-pipe` and
/dev/class/gpu to determine when new graphics devices appear.

When a new graphics device appears, the loader must determine the component
associated with the ICD. The exact mechanism depends on the type of device:

* `/dev/class/gpu` - [fuchsia.gpu.magma/Device.GetIcdList][GetIcdList] is called
    on the device.
* `/dev/class/goldfish-pipe`: The ICD URL is hardcoded to be
    `fuchsia-pkg://fuchsia.com/libvulkan_goldfish#meta/vulkan.cm`

More types of GPU hardware devices may be supported in the future. Software ICDs
may also be exposed through the loader protocol on some devices as a fallback
(as chosen by `vulkan_loader` configuration). Software Vulkan ICDs (such as
SwiftShader) often have JITs and require the ability to write to executable
memory; because of that, they may not be usable on production systems where that
capability is tightly controlled for security reasons.

#### Filesystem serving

`vulkan_loader` serves multiple filesystems to clients, including the _manifest
fs_ and _device fs_.  It creates these filesystems based on the contents of
multiple ICD packages and services it receives through devfs. As a result, they
must be constructed using a filesystem serving library and don't reflect
anything on-disk.

* _manifest fs_: All [manifest JSON files][manifest-json] describing all the
   relevant ICDs; this filesystem looks the same as the
   /usr/local/share/vulkan/icd.d on Linux, so that minimal changes to the loader
   are needed.
* _device fs_: Contains all GPU devices needed by supported Vulkan ICDs. For
  `/dev/<path>/<node>` device, the filesystem will contain a `<path>/<node>`
  entry.

#### ICD↔︎loader interface

ICDs are made available to the loader as [CFv2 components][component]. An ICD
component must expose a `contents` directory containing an arbitrary
directory tree containing a shared library, as well as a `metadata` directory
containing a single `metadata.json` file.

An ICD is generally contained by itself in a separate [package]. In that case,
the `contents` directory would be the root of the package, and the `metadata`
directory would be the `meta/metadata/` directory in the package. The loader
doesn't enforce this layout, however.

`metadata.json` and `manifest.json` should ideally be stored under the [`meta`
directory in the package][meta-far], since that directory is most efficient at
storing small files.

#### ICD shared libraries

ICD shared libraries should match the [Vulkan ICD ABI][ICD-abi]. ICDs are
executable shared libraries and can be placed in most subdirectories (not
`/bin`) of the package.

#### Component manifest

The Vulkan loader supplies an `icd_runner` [runner] to simplify the creation of
an ICD component from a package. The ICD package must contain a [component
manifest][component-manifest] `.cml` that exports the `contents` and `metadata`
directory capabilities.

The `icd_runner` automatically exports `/pkg/data` and `/pkg/meta/metadata`
directories from the ICD package at the `/pkg-data` and `/pkg-metadata` paths.
These can be used by the CML to export both directory capabilities (using the
`subdir` property to expose a subdirectory as a full capability).

The ICD component may also use the ELF runner, but the only service available
to it is `fuchsia.logger.LogSink`.

#### metadata.json

metadata.json is a single JSON file that describes the ICD to the loader.
Example:

```json
{
    "file_path": "lib/libvulkan_example.so",
    "version": 1,
    "manifest_path": "meta/icd.d/libvulkan_example.json"
}
```

* `version` must be 1 for this metadata version.
* `file_path` is the location of the ICD shared library relative to the exposed
  `contents` directory.
* `manifest_path` is the location of the [Khronos ICD manifest JSON
  file][loaderinterface] relative to the exposed `contents` directory.

### Other clients

The set of available Vulkan ICDs can change over time; when the system first
boots no ICDs will be available until the hardware enumerates. After that,
devices may be hotplugged and either appear or disappear.

This means that the list of devices returned by `vkEnumeratePhysicalDevices` can
change at any time. Some applications that require Vulkan may want to retry
after the set of available devices changes. They can use a filesystem watcher on
the filesystem returned from `fuchsia.vulkan.loader/Loader.ConnectToManifestFs`
to determine when to retry.

## Implementation

This design represents the current architecture of the Vulkan loader as already
implemented on Fuchsia.

## Performance

The Vulkan loader is most active at process startup. Once a Vulkan ICD is
loaded, it either trampolines Vulkan calls to go into the ICD, or returns ICD
function implementations to the application for the application to call
directly. As such, its performance is only critical during process startup.

No special consideration has been given to the performance of the loader. It has
to launch components to connect to ICDs, and traverse multiple filesystem paths
to work out the ICD and layer configuration. At the moment it's not believed
that it has any large run time performance impact.

## Backwards Compatibility

Communication between libvulkan.so and `vulkan_loader` uses filesystems, JSON,
and FIDL. The filesystems and JSON have been in use on Linux for several years
without backwards compatibility issues. There are natural ways of evolving them
(adding paths and keys, respectively) to maintain backwards compatibility.  The
FIDL interface is small and can be evolved using FIDL versioning mechanisms.

## Security considerations

Components will load shared libraries provided by the Vulkan loader. The
system's normal [verified execution][verified-execution] enforcement will ensure
that the executable shared library comes from a trustworthy location (e.g. the
filesystem). Any parent component may interpose on the
`fuchsia.vulkan.loader.Loader` protocol, so there's no guarantee that the loader
service component sees is provided by the system.

The ICDs chosen to load are referenced by path in the [Magma system driver
(MSD)][MSD] and loaded through a resolver. The full resolver is used by default,
so that can load ephemeral packages. Loading ICDs from ephemeral packages is
useful for developers of ICDs, but shouldn't be necessary for most users.
Loading ephemeral packages can be disabled by disabling the full resolver
(setting the `auto_update_packages=false` gn arg). We can also create multiple
core shards for the Vulkan loader that product owners can choose between; eng
builds could choose the shard that uses the full resolver, and user builds could
use the shard with the base resolver.

If needed for specific products, multiple instances of the `vulkan_loader`
service can be created, each with access to different resolvers. Their
`fuchsia.vulkan.loader.Loader` implementations could be routed to client
components based on the clients' security requirements. At the moment no
products have this requirement.

Configuration of the loader may cause unexpected behavior in the application, by
loading new layers, preventing the loading of other layers, or setting options
on those layers. The component must opt in to taking its configuration from
outside its package (by routing a directory capability from outside the
package), but otherwise has complete control of loader configuration.

ICD shared libraries are executed in the client process and can execute
arbitrary code within that process. The build process and conformance tests will
ensure they only import allowlisted symbols, but that isn't a security guarantee
and may easily be bypassed by e.g. looking at callstacks to find addresses and
parsing executables in memory to find useful gadgets. Applications won't
validate most values returned by Vulkan, and may be manipulated into doing
arbitrary memory accesses by careful manipulation of those values.

If a runner loads multiple components it doesn't trust into a single process
(perhaps by using a virtual machine or other non-process mechanism to isolate
components), those components must not be able to make direct Vulkan calls,
since there's no known way to validate Vulkan API calls to guarantee that
applications don't perform undefined behavior in the Vulkan ICD; even the Vulkan
validation layers provide only limited protection. Runner code may make Vulkan
calls itself, for example using Skia or ANGLE to execute validated rendering
commands on the behalf of a client. Service and device channels provided to the
ICD must be from some source the runner trusts, to prevent child components from
snooping on each other.

## Privacy considerations

The Vulkan loader has minimal privacy effects. The only information exposed over
FIDL is whether the application attempts to use Vulkan, and which devices it
attempts to use.

## Testing

`vulkan_loader` and libvulkan.so have unit and integration tests. These tests
are hermetic and don't depend on device drivers or real ICDs installed on the
system.

In addition, there are CTF tests to ensure that the implementation of the
`fuchsia.vulkan.loader.Loader` protocol is correct and that the ICDs provided by
it are compatible with old loader versions.

The Vulkan CTS and other Vulkan tests in the fuchsia tree act as end-to-end
tests, checking that the `vulkan_loader` is compatible with `libvulkan.so`.
These can only run on systems with Vulkan hardware and device drivers.

## Documentation

We have `vulkan_loader` documentation at
[/src/graphics/bin/vulkan_loader/README.md][README]. There is some [user
documentation][magmavulkan] for how to use the Vulkan loader.

The upstream Vulkan Loader has [documentation][loaderinterface].  We should try
to add and upstream Fuchsia-specific information to that document.

## Prior art and references

The [Linux/Windows/MacOS loader][loaderinterface].

[Vulkan]: https://www.vulkan.org/
[khronos]: https://www.khronos.org/
[vulkaninterfacing]: https://chromium.googlesource.com/external/github.com/KhronosGroup/Vulkan-Loader/+/HEAD/loader/LoaderAndLayerInterface.md#interfacing-with-vulkan-functions
[loaderrepo]: https://github.com/KhronosGroup/Vulkan-Loader#introduction
[ICD]: https://github.com/KhronosGroup/Vulkan-Loader/blob/master/docs/LoaderInterfaceArchitecture.md#installable-client-drivers
[layers]: https://github.com/KhronosGroup/Vulkan-Loader/blob/master/docs/LoaderInterfaceArchitecture.md#layers
[loader-protocol]: /sdk/fidl/fuchsia.vulkan.loader/loader.fidl
[create-instance]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html
[GetIcdList]: https://fuchsia.dev/reference/fidl/fuchsia.gpu.magma#Device.GetIcdList
[ICD-abi]: /docs/concepts/packages/system.md#vulkan-icd
[runner]: /docs/concepts/components/v2/capabilities/runners.md
[loader-source]: /third_party/Vulkan-Loader
[component]: /docs/glossary/README.md#component
[package]: /docs/concepts/packages/package.md
[component-manifest]: /docs/concepts/components/v2/component_manifests.md
[loaderinterface]: https://github.com/KhronosGroup/Vulkan-Loader/blob/master/loader/LoaderAndLayerInterface.md
[meta-far]: /docs/concepts/packages/package.md#meta-far
[zxio]: https://fuchsia.googlesource.com/fuchsia/+/db7a76c861cce21b30d6199442c813a1327e020a/sdk/lib/zxio/README.md
[override-layer]: https://vulkan.lunarg.com/doc/view/1.3.211.0/linux/LoaderLayerInterface.html#user-content-override-meta-layer
[magma]: /docs/development/graphics/magma
[magmavulkan]: /docs/development/graphics/magma/concepts/vulkan.md
[verified-execution]: /docs/concepts/security/verified_execution.md
[README]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/graphics/bin/vulkan_loader/README.md
[manifest-json]: https://github.com/KhronosGroup/Vulkan-Loader/blob/master/loader/LoaderAndLayerInterface.md#icd-manifest-file-format
[SwiftShader]: https://github.com/google/swiftshader
[MSD]: /docs/development/graphics/magma/concepts/design.md#architecture
[environ]: /docs/concepts/components/v2/elf_runner.md#environment-variables
[early-version]: https://docs.google.com/document/d/1qXmCrxB_YxajvSRMum6qOn5yD4Trj1lhWJ7C28gJ-w8/edit
