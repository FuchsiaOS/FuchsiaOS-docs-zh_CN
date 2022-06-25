{% set areas | yamlloads %}
{% include "docs/contribute/governance/areas/_areas.yaml" %}
{% endset %}

# Areas

{% for area in areas %}
## {{ area.name }} {:#{{ area.name|replace(" ", "-")|lower() }}}

<!--
Due to lack of support for variable include, we use a switch of sorts to render
each area's description.
-->

{% if area.name == "Foreign ABI Compatibility" %}
The set of APIs used to run and interact with programs compiled for other operating systems.

Currently this covers the Starnix (Linux binary compatibility) APIs, for example:

* [fuchsia.starnix.developer](/sdk/fidl/fuchsia.starnix.developer/) contains protocols for starting
  a component containing an unmodified Linux binary.
* [Manager](/sdk/fidl/fuchsia.starnix.developer/developer.fidl) allows developers to connect their
  development machine to a shell component running on a Fuchsia device.
{% endif %} <!-- Foreign ABI Compatibility -->

{% if area.name == "Bluetooth" %}
The set of APIs for managing and communicating via Bluetooth.  This includes
both connecting peer devices, searching for devices, advertising the local
device, and communicating or interacting via Bluetooth profiles.  Generally once
connected, Bluetooth capabilities will be exposed through APIs in other
sections, and this API only exposes surfaces for connecting, managing discovery
and pairing, and Low Energy protocols which are often custom to each device.

Often specific Bluetooth Profiles provide an API for system services to use for
status and control as well.

Examples:

* [fuchsia.bluetooth.sys](/sdk/fidl/fuchsia.bluetooth.sys/)
* [fuchsia.bluetooth.le](/sdk/fidl/fuchsia.bluetooth.le/)
* [fuchsia.bluetooth.gatt](/sdk/fidl/fuchsia.bluetooth.gatt/)
{% endif %} <!-- Bluetooth -->

{% if area.name == "Component Framework" %}
The set of APIs that are used to define components, interact with components,
and integrate with the Component Framework. These are the low level APIs for
interfacing with the Component Framework -- in some cases they may be used by
developers directly, but they may also be used to build higher level APIs such
as Session Framework.

Examples:

* [fuchsia.component](/sdk/fidl/fuchsia.component/)
* [fuchsia.component.internal](/sdk/fidl/fuchsia.component.internal/)
* [fuchsia.sys2](/sdk/fidl/fuchsia.sys2/)
* [fuchsia.sys](/sdk/fidl/fuchsia.sys/)
{% endif %} <!-- Component Framework -->

{% if area.name == "Developer" %}
Developer tool interfaces, such as the [Command-line Tools
Rubric](development/api/cli.md). APIs that affect the developer experience in
the host environment such as debugging, forensics, or the development kit.
{% endif %} <!-- Developer -->

{% if area.name == "Diagnostics" %}
The set of APIs that are used to publish and query diagnostics data from
components on the system. This includes the ability to [stream
logs](reference/diagnostics/logs/access.md), view and publish [Inspect
data](development/diagnostics/inspect/README.md), and [observe lifecycle
events](concepts/components/diagnostics/lifecycle_events/README.md).

Examples:

* [fuchsia.diagnostics](/sdk/fidl/fuchsia.diagnostics/)
* [fuchsia.inspect](/sdk/fidl/fuchsia.inspect/)
{% endif %} <!-- Diagnostics -->

{% if area.name == "Drivers" %}
The set of APIs used to communicate with various drivers that interact with
hardware or other drivers. The apis are accessible by opening the device using a
devfs path, such as `/dev/class/<protocol the device exposes>/<incremental
number>`.

Most of the APIs exposed by drivers are in the `fuchsia.hardware.*` namespaces.

Other APIs are distributed under the corresponding area (e.g. Bluetooth, WLAN,
Graphics, HCI) that the driver tackles. Although these APIs do not live under
`fuchsia.hardware.*` namespace they might interact with hardware, or other
drivers that interact with hardware.

Examples:

* [fuchsia.input.report](/sdk/fidl/fuchsia.input.report/)
{% endif %} <!-- Drivers -->

{% if area.name == "Driver SDK" %}
The set of APIs used to interact with devices via the driver manager. This may
be used by developers to retrieve information about a device or change its
current state.

Examples:

* [fuchsia.device](/sdk/fidl/fuchsia.device/)
* [fuchsia.device.manager](/sdk/fidl/fuchsia.device.manager/)
{% endif %}  <!-- Driver SDK -->

{% if area.name == "Experiences" %}
The set of APIs used to create user experiences. These include the set of APIs that facilitate user interactions that are common across multiple products.

Examples:

* [fuchsia.settings](/sdk/fidl/fuchsia.settings/)
* [fuchsia.ui.remotewidgets](/sdk/fidl/fuchsia.ui.remotewidgets/)
{% endif %} <!-- Experiences -->

{% if area.name == "FIDL" %}
Since most APIs are expressed in FIDL, the FIDL area is cross-cutting with the
goal to both support all other areas, and leverage their experience to inform
the future direction of the FIDL language and ecosystem.
{% endif %} <!-- FIDL -->

{% if area.name == "Firmware" %}
A small set of libraries necessary for firmware to boot Zircon, for example ZBI image handling, A/B/R boot metadata, verified boot. Essentially, this defines the contract for how the bootloader communicates with Zircon.

As firmware runs outside of Fuchsia, this is not generally meant for Fuchsia end-developers, but instead for bringing up Fuchsia on new platforms. These libraries together form the "Firmware SDK" which is then ported to a specific platform's firmware codebase.

Examples:

* [libabr](/src/firmware/lib/abr/), A/B/R metadata handling.
* [libavb](https://android.googlesource.com/platform/external/avb/), verified
  boot.
* [libzbi](/src/firmware/lib/zbi/), ZBI manipulation.
{% endif %} <!-- Firmware -->

{% if area.name == "Graphics" %}
The set of APIs that are used to transport and compose images on the system. It
includes interfaces for communicating with graphics hardware, as well as
scene-graph communication between Scenic and the rest of the system (not
including higher-level concepts such as views, see the [View
System](#view-system) area for that).

Examples:

* [fuchsia.hardware.display](/sdk/fidl/fuchsia.hardware.display/)
* [fuchsia.gpu.magma](/sdk/fidl/fuchsia.gpu.magma/)
* [fuchsia.ui.gfx](/sdk/fidl/fuchsia.ui.gfx/)
{% endif %} <!-- Graphics -->

{% if area.name == "HCI" %}
_Covers input, accessibility, internationalization._

The set of APIs that connect human–computer interaction (HCI) devices starting
from drivers, to filtering, semantic understanding, grouping, routing, all the
way to delivering these inputs to the [View System](#view-system). This includes
APIs associated with touch, mouse, keyboard, text editing and the accessibility
framework.

Examples:

* [fuchsia.ui.input](/sdk/fidl/fuchsia.ui.input/)
* [fuchsia.ui.pointer](/sdk/fidl/fuchsia.ui.pointer/)
* [fuchsia.ui.input.accessibility](/sdk/fidl/fuchsia.ui.input.accessibility/)
* [fuchsia.accessibility.semantics](/sdk/fidl/fuchsia.accessibility.semantics/)
* fuchsia.accessibility.*
* fuchsia.input.*
{% endif %} <!-- HCI -->

{% if area.name == "Identity" %}
The set of APIs used to manage user accounts, authentication, and identity information for service providers.

Examples:

* [fuchsia.identity.account](/sdk/fidl/fuchsia.identity.account/)
* [fuchsia.identity.tokens](/sdk/fidl/fuchsia.identity.tokens/)
* [fuchsia.auth](/sdk/fidl/fuchsia.auth/)
{% endif %} <!-- Identity -->

{% if area.name == "Kernel" %}
The Fuchsia kernel, whose API surface is:

* The set of syscalls and the set of types and constants associated with these
  syscalls. Those APIs are defined in [//zircon/vdso/](/zircon/vdso/) and
  [//zircon/system/public/zircon/](/zircon/system/public/zircon/).
* The interface with bootloaders, the most important being the
  [ZBI](/zircon/system/public/zircon/boot/image.h).
* The BOOTFS image and the ABI of the binaries within.
{% endif %} <!-- Kernel -->

<!-- Missing: Ledger -->

{% if area.name == "Media" %}
The set of APIs used to capture, process and render audio and video streams. The
media APIs also encompass adjacent concerns such as volume control and media
session management.

Examples:

* [fuchsia.camera](/sdk/fidl/fuchsia.camera/)
* [fuchsia.media](/sdk/fidl/fuchsia.media/)
* [fuchsia.media.audio](/sdk/fidl/fuchsia.media.audio/)
* [fuchsia.media.drm](/sdk/fidl/fuchsia.media.drm/)
* [fuchsia.media.sessions2](/sdk/fidl/fuchsia.media.sessions2/)
* [fuchsia.media.sounds](/sdk/fidl/fuchsia.media.sounds/)
* [fuchsia.mediacodec](/sdk/fidl/fuchsia.mediacodec/)
{% endif %} <!-- Media -->

{% if area.name == "Metrics" %}
The set of APIs that allow clients to log events that are associated with
metrics. These events are collected off-device, and can later be analyzed across
many devices.

Examples:

* [fuchsia.metrics](/sdk/fidl/fuchsia.metrics/)
* [fuchsia.cobalt](/sdk/fidl/fuchsia.cobalt/)
{% endif %} <!-- Metrics -->

{% if area.name == "Netstack" %}
The set of APIs enabling networking in Fuchsia. Encompasses APIs that drive the data, control, and management planes of networking ranging from contracts with device drivers to auxiliary application-level protocol services.

Examples:

* [fuchsia.hardware.network](/sdk/fidl/fuchsia.hardware.network/), data plane
  contract with device drivers.
* [fuchsia.posix.socket](/sdk/fidl/fuchsia.posix.socket/), POSIX sockets API.
* [fuchsia.net.interfaces](/sdk/fidl/fuchsia.net.interfaces/), Interface
  management plane.
* [fuchsia.net.name](/sdk/fidl/fuchsia.net.name/), Application level name
  resolution.
{% endif %} <!-- Netstack -->

{% if area.name == "Security" %}
The set of APIs used to directly interact with security features (for example
cryptographic key management) or tools (for example fuzzers).

Examples:

* [fuchsia.fuzzer](/src/sys/fuzzing/fidl/overview.fidl)
* [fuchsia.kms](/sdk/fidl/fuchsia.kms/)
* [fuchsia.tee](/sdk/fidl/fuchsia.tee/)
{% endif %} <!-- Security -->

{% if area.name == "Sessions" %}
A set of APIs to coordinate a product’s user experience. Specifically the API
contains protocols for communicating with the session component.

Examples:

* The session may ask a window manager to display a component view via
  [GraphicalPresenter](/sdk/fidl/fuchsia.element/graphical_presenter.fidl).
* The session may implement
  [ElementManager](https://cs.opensource.google/search?q=%22protocol%20ElementManager%22%20file:.*.fidl&sq=&ss=fuchsia%2Ffuchsia)
  to receive requests to add components to the session. A developer may use
  [Restarter](https://cs.opensource.google/search?q=%22protocol%20Restarter%22%20file:.*.fidl&sq=&ss=fuchsia%2Ffuchsia)
  to restart a running session.

The session API often makes use of protocols and data structures defined in
other areas of the platform. For example, `GraphicalPresenter` does not define
its own view type. Instead, it uses `ViewRef` from the [View
System](#view-system) to identify component views.
{% endif %} <!-- Sessions -->

<!-- Missing: Software Delivery -->

{% if area.name == "Storage" %}
Storage is a combination of the following APIs:

* [fuchsia.io](/sdk/fidl/fuchsia.io/)

  Describes the common means of service discovery, filesystem access,
  and capability sharing on Fuchsia.

  They are used primarily for client interaction with the filesystem, where a
  client can be any component/process in the system that needs to access
  files/directories in a filesystem.
* [fuchsia.fshost](/sdk/fidl/fuchsia.fshost/)

  Used for finding block devices, starting filesystem processes to service these
  block devices, and providing handles for these file systems to the rest of
  Fuchsia.
* Filesystem specific APIs, used for operations specific to a filesystem.

  Examples:
   * [fuchsia.minfs](/sdk/fidl/fuchsia.minfs/)
   * [fuchsia.blobfs](/sdk/fidl/fuchsia.blobfs/)
* [fuchsia.fs](/sdk/fidl/fuchsia.fs/), responsible for providing administration
  functionality for filesystems.
{% endif %} <!-- Storage -->

<!-- Missing: Toolchain -->

{% if area.name == "View System" %}
The set of APIs that need to reason about and interact with visual regions
("views") and their lifecycle. They generally are not tied to a particular
graphical representation, but some have close ties to graphics APIs. HCI APIs
are built on top of the View System.

Examples:

* [fuchsia.ui.views](/sdk/fidl/fuchsia.ui.views/)
* [fuchsia.ui.focus](/sdk/fidl/fuchsia.ui.focus/)
* [fuchsia.ui.app](/sdk/fidl/fuchsia.ui.app/), in particular
  [ViewProvider](https://cs.opensource.google/search?q=%22protocol%20ViewProvider%22%20file:.*.fidl&sq=&ss=fuchsia%2Ffuchsia)
* [fuchsia.ui.policy](/sdk/fidl/fuchsia.ui.policy/)
* [fuchsia.ui.annotation](/sdk/fidl/fuchsia.ui.annotation/)
* view/scene connection signals in fuchsia.ui.gfx.Event
{% endif %} <!-- View System -->

{% if area.name == "Virtualization" %}
Virtualization is the combination of:

* The hypervisor, which is implemented by the Zircon kernel, and provides the
  execution environment for a virtual machine. Specifically, it provides address
  space isolation, trapping of access to memory or IO port addresses, and
  management of virtual CPUs.
* The virtual machine manager, which uses the hypervisor in order to provide a
  complete virtual machine for an operating system to run within. This includes
  the emulation of hardware, as well as the loading and execution of the
  operating system itself. It provides a bridge between the guest operating
  system running within the virtual machine, and services within the host
  operating system, such as storage, networking, and graphics.
{% endif %} <!-- Virtualization -->

{% if area.name == "Web" %}
Web encompasses APIs for working with standard web protocols (e.g. HTTP, HTTP2),
content types (e.g. HTML) and application run-time technologies (e.g.
JavaScript, WebAssembly). Functional interfaces (e.g.
[fuchsia.web](/sdk/fidl/fuchsia.web/),
[fuchsia.net.http](/sdk/fidl/fuchsia.net.http/)) typically replace functionality
that would otherwise need to be bundled as a library into each individual client
package.

Examples:

* [fuchsia.net.http](/sdk/fidl/fuchsia.net.http/), supports basic interactions
  (e.g. GET, PUT) with HTTP-based services.
* [fuchsia.url](/sdk/fidl/fuchsia.url/), defines web-standard URL type, and
  limits.
* [fuchsia.web](/sdk/fidl/fuchsia.web/), allows component instances to be
  created to host content created using standard web technologies (HTML,
  JavaScript, etc). These are used in a similar way to in-process web-rendering
  libraries, with the benefit of stronger isolation from the calling
  application.

  An implementation provided by the Chromium project is included in the Fuchsia
  repository as a pre-built package.
{% endif %} <!-- Web -->

<!-- Missing: WLAN -->

{% endfor %}
