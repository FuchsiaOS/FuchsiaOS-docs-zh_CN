{% set rfcid = "RFC-0111" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- *** DO NOT EDIT ABOVE THIS LINE -->

## Summary {#summary}

This RFC details an initial set of technical specifications for a given hardware
platform to be compatible with Fuchsia, contributed to the Fuchsia project, and
categorized within the Fuchsia project. This RFC is intended to both codify an
initial set of hardware specifications and to outline the process for proposing
hardware-related source code contributions to the `fuchsia.git` repository.
Changes to the hardware specifications can only made through the
[Fuchsia RFC process](/contribute/governance/rfcs).

## Overview {#overview}

This document details the initial technical specifications for a given hardware platform
to be compatible with Fuchsia, contributed to the Fuchsia project, and
categorized as either **Supported** or **Experimental** hardware. Within the
Supported hardware and Experimental hardware categories, there are
Required and Recommended specifications.

Note: This RFC establishes the _current_ and _intial_ set of technical
specifications for a given hardware platform to be compatible with Fuchsia.
Hardware categories as well as their respective Recommended and Required
specifications are subject to change and could be modified as the Fuchsia
project progresses.

While compatibility with a wide range of hardware and form factors is desirable,
each new hardware target imposes architectural, maintenance, testing, and
validation costs to the Fuchsia project. This document lists Fuchsia's hardware
platform specifications and explains the rationales associated with each
hardware choice.

Specifications may be updated as the Fuchsia project evolves. Specification
changes that have broad technical impact to the project are either approved or
rejected through the [Fuchsia RFC process.](/contribute/governance/rfcs)

This policy covers different kinds of hardware in Fuchsia, including
the following:

*   Architectures (e.g. ARM, x86, RISC-V),
*   Boards (e.g. VIM3, Raspberry Pi, Intel NUC)
*   Drivers for other hardware (e.g. USB devices, network adapters,
    HID devices, etc.).

The "Initial Fuchsia hardware platform specifications" policy doesn't cover
device drivers written by external contributors using the Driver SDK and hosted outside any Fuchsia source tree repository. Developers can write drivers using
the Driver SDK and distribute the source and binaries outside of the Fuchsia
open source project repositories.

### Hardware platform categories {#hardware-platform-categories-overview}

Hardware platforms that the Fuchsia project performs Fuchsia project-hosted
testing on are categorized as **Supported** hardware. **Experimental**
hardware fulfills the minimum requirements for a hardware platform to be
compatible with running Fuchsia but no Fuchsia-hosted testing is performed.
Experimental Hardware platforms should not be considered as appropriate for
production environments. Rather, the Fuchsia project considers Experimental
hardware as avenues for exploring and learning about Fuchsia.

This document defines hardware specifications for Fuchsia-compatible hardware
platforms, specifically:

1. "Base hardware specifications" -- these are the minimum specifications for
Fuchsia to run on a given hardware platform. Base hardware specifications are
categorized as either Required or Recommended. Experimental hardware only needs
to comply with the Required Base hardware specifications.
2. "Supported hardware specifications" -- these are the specifications necessary
for the hardware platform to potentially be categorized as Supported hardware.
In addition to these specifications, hardware in this category must also be
approved through the RFC process, due to the resulting testing requirements.

The document also defines the availability expectations associated with
Supported hardware and Experimental hardware. Finally, the document defines a
process for adding hardware platforms to the Fuchsia source.

### Peripherals {#peripherals}

The "Initial Fuchsia hardware platform specifications" RFC covers the
specification of hardware for the base system that allows for peripheral
devices to work with a given platform. This includes USB storage drivers and
drivers for graphics cards (both permanently attached and
removable graphics cards).

"Initial Fuchsia hardware platform specifications" does not cover the details of
the operation of external peripherals beyond interactions directly observable by
the system running fuchsia. For instance, operating details of firmware on
the following is not covered:

*   Machine Learning (ML) accelerators
*   Audio hardware digital signal processors (DSPs)
*   Graphic cards internals (not the driver that allows communication with
the main system).

The "Initial Fuchsia hardware platform specifications" policy also does
not cover bootloaders that are not the final stage bootloader.

### Document Definitions {#document-definitions}

The following terms are used in this document:

<table>
  <tr>
   <td><strong>Term</strong>
   </td>
   <td><strong>Definition</strong>
   </td>
  </tr>
  <tr>
   <td>Architecture
   </td>
   <td>A processor architecture like x86 or ARM.
   </td>
  </tr>
  <tr>
   <td>System
   </td>
   <td>A complete computer hardware platform with a CPU, memory,
   peripherals, etc. Also commonly referred to as a "board".
   </td>
  </tr>
  <tr>
   <td>Hardware platform
   </td>
   <td>The System on a chip (SoC) (for example ARM-based SoCs) or the
   combination of the Central processing unit (CPU) and the chipset
   (for example many x86-based systems).
   </td>
  </tr>
  <tr>
   <td>SoC
   </td>
   <td>System on a chip; typically consists of a processor with integrated
   peripherals in the same silicon package, typically used as the heart of a
   computer system. Often has multiple CPU cores.
   </td>
  </tr>
  <tr>
   <td>CPU
   </td>
   <td>Central processing unit.
   </td>
  </tr>
  <tr>
   <td>Change
   </td>
   <td>The Fuchsia project uses Gerrit's web-based UI to manage code and
   documentation reviews. When a commit is uploaded to Gerrit, it is referred
   to as a change. Note that the underlying revision control system is git.
   </td>
  </tr>
  <tr>
   <td>Fuchsia-project hosted testing
   </td>
   <td>Testing that occurs through the Fuchsia project build and integration
   process.
   </td>
  </tr>
  <tr>
   <td>CI/CQ
   </td>
   <td>Continuous Integration / Commit Queue: systems that build proposed
   changes before integrating into the main source tree, and continuously test
   the entire source to discover regressions caused by the interaction of
   multiple integrated changes.
   </td>
  </tr>
</table>

## Base hardware specifications {#base-hardware-specifications}

The following are the base hardware specifications for Fuchsia.
Base specifications are further divided into two categories: Required and
Recommended.

Note: This section of the RFC establishes the _current_ and _intial_ set of
technical specifications for Base hardware. The Base hardware category as well
as its related Recommended and Required specifications are subject to change and could be modified
as the Fuchsia project progresses.

<table>
  <tr>
   <td><strong>Base specification</strong>
   </td>
   <td><strong>Definition</strong>
   </td>
  </tr>
  <tr>
   <td>Required
   </td>
   <td>Without this specification, Fuchsia is not able to build and install on
   a given hardware platform.
   </td>
  </tr>
  <tr>
   <td>Recommended
   </td>
   <td>This specification is highly desirable but not required for Fuchsia to
   build and install on a given hardware platform. Without this specification,
   Fuchsia's functionality or performance is compromised.
   </td>
  </tr>
</table>

###  Required Hardware Features {#required-base-hardware-features}

The following hardware specifications are required.

#### Required specifications

Required specifications are the basic technical requirements for Fuchsia to
build and install on a given hardware platform.

##### Required: 64-bit CPU and platform

The hardware platform CPU must support a 64-bit address space operating mode.
It is acceptable if the platform or CPU starts in a non-64 bit mode as long as
it is only used during early boot before the Zircon kernel starts.

###### Rationale

A 64-bit address space is needed with sufficient space to efficiently and
effectively implement features such as address space layout
randomization (ASLR).

###### Examples

All x86-64 hardware platforms; all ARMv8-A and later processors (except A32).

##### Required: Full featured memory management unit (MMU)

The hardware platform must provide a full-featured, modern memory management
unit that allows for creating an arbitrary number of address spaces, mapping
physical memory in reasonably sized pages to any of those spaces, and enforcing
protection between separate address spaces through hardware access
control mechanisms.

###### Rationale

Fuchsia makes heavy use of virtual memory objects, so efficient memory
management is critical to the operation of a Fuchsia hardware platform.
Fuchsia's [basic security guarantees](/concepts/principles/secure.md)
are not possible without hardware-enforced memory access controls.

###### Examples

ARM v8.0-A or later; all modern x86 CPUs (2010 and beyond).

##### Required: Little-endian mode

The hardware platform must support little-endian (LE) byte-ordering mode.

###### Rationale

Dealing with arbitrary endianness is expensive in code development and testing
time. In theory, Fuchsia's code can be made endian-agnostic, but the Fuchsia
project is not prepared to commit to the effort at this time. Most modern
architectures are either LE native or support LE mode.

###### Examples

All x86; all modern ARM processors are little-endian capable.

##### Required: For x86-64, the x86-64-v2 instruction set architecture

Hardware platforms based on the x86-64 processor architecture must support
the [x86-64-v2 instruction set architecture](https://en.wikipedia.org/wiki/X86-64#Microarchitecture_levels)
(ISA) as described in [Fuchsia RFC-0073](/contribute/governance/rfcs/0073_x86_64_platform_requirement.md).
This covers all Intel Westmere and newer CPUs and all AMD Bulldozer and
newer CPUs.

###### Rationale

Having a minimum ISA target allows useful optimizations that assume the
presence of certain instructions. Full details are in
[RFC-0073](/contribute/governance/rfcs/0073_x86_64_platform_requirement.md):
Raising x86-64 platform requirement to x86-64-v2.

##### Required: Clock and timers

Clock and timers must:

* Be invariant such that they do not arbitrarily change frequency as other parts
  of the system undergo frequency scaling.
* Be at least 56 bits wide with roll-over time of not less than 40 years.
* Have a nominal frequency which is explicitly knowable without the need for any
  runtime calibration.

### Required toolchain and language support {#required-base-toolchain-and-language-support}

The following toolchain and language support specifications are required.

#### Required: LLVM toolchain support

The architecture, platform, and target board in question must be in
the [LLVM core tier](https://llvm.org/docs/SupportPolicy.html) fully supported
by upstream [LLVM](https://llvm.org/). Clang toolchain support for the
given platform must be kept up to date. For more information on the targets
supported by LLVM see
[CMakeLists.txt](https://github.com/llvm/llvm-project/blob/f749550cfe9f0bf2364abb2139835348587062ed/llvm/CMakeLists.txt#L291)
in the LLVM project.

##### Rationale

Fuchsia uses LLVM as its default compilation toolchain.

#### Required: Tier 2 Rust language support

The architecture, platform, and target board in question must be at least
in [Rust Tier 2](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-2)
, and [preferably](#tier-1-rust-language-support)
in Rust Tier 1.

##### Rationale

Fuchsia uses Rust extensively.

#### Required: Dart language support

The architecture, platform, and target board in question must be supported by
[Dart](https://dart.dev/overview#platform).

##### Rationale

The majority of Fuchsia's user interface is built using Flutter,
which uses Dart.

#### Required: Go language support

The architecture, platform, and target board in question must be supported by
[Go](https://blog.golang.org/ports).

##### Rationale

Fuchsia's netstack uses Go.

###  Recommended Hardware Features {#recommended-base-hardware-features}

The following hardware specifications are recommended.

#### Recommended specifications

While not required to build and install Fuchsia, the following specifications
are highly desirable supplements to the base specifications because they improve
Fuchsia's functionality on a given hardware platform.

##### Recommended: Clock and timers

* Have timers which can deliver interrupts to cores when the timer value exceeds
  a given absolute threshold.
* Be implemented as part of the architecture itself, not as a peripheral present
  in the target layer.

##### Recommended: I/O memory management unit (IOMMU)

The Fuchsia project recommends that the hardware platform supports a hardware
I/O memory management unit. The IOMMU must be able to NAK read or write
transactions initiated from uniquely identifiable hardware units in the system
when there is no explicit permission to do so granted via the IOMMU.

Optionally, the IOMMU could also include the following nice-to-have features:

* The ability to do address translation for hardware DMA operations (this would
  allow the Fuchsia project to use pinned but discontinuous physical memory
  for HW DMA).
* The ability to detect and debug IOMMU page-fault situations (e.g. if a unit
  steps out of bounds, it would be nice to be able to get an exception, and some
  context like which unit attempted which operation at which location).

###### Rationale

Zircon drivers are user-mode software components with very restricted hardware
access. An IOMMU defends against bugs or attacks from devices with access to
physical memory, and is more reliable than auditing the hardware and software.
This improves the overall security of the Fuchsia operating system.

###### Examples

ARM's IOMMU specification: System Memory Management Unit (SMMU)

Intel's x86 IOMMU specifications: Intel VT-d, AMD-Vi

##### Recommended: Hardware cryptographic acceleration support

The Fuchsia project recommends that the hardware platform provides hardware
acceleration for both AES and SHA cryptographic operations.

###### Rationale

Fuchsia makes extensive use of cryptographic operations, so hardware
acceleration is highly desirable for acceptable performance, especially on
smaller hardware platforms.

###### Examples

ARM v8.0-A (Cortex A-34) or later:

*   AES: AESE, AESD, AESMC, AESIMC, PMULL and PMULL2
*   SHA2: SHA256H, SHA256H2, SHA256U0, SHA256U1

ARM 8.1 or later (or ARMv8-A if supported):

*   CRC32

x86 processors:

*   AES-NI: Intel ('Westmere' and newer; 'Silvermont' Atoms and newer)
*   AES-NI: AMD ('Bulldozer' and newer; 'Jaguar' and newer)
*   SHA(1, 256): Intel ('Ice Lake' and newer; 'Goldmont Plus' Atoms and newer)
*   SHA(1, 256): AMD ('Zen' and newer)
*   CRC32 (Processors supporting SSE 4.2)

### Recommended toolchain and language support {#recommended-base-toolchain-and-language-support}

The following toolchain and language support specifications are recommended but
not required.

#### Recommended: GCC toolchain support

The Fuchsia project recommends that the given architecture, platform and target
board be fully supported by the GCC toolchain. For pure application development
which is not platform specific, LLVM-only support is acceptable.

##### Rationale

Having a second toolchain uncovers more bugs.

#### Recommended: Tier 1 Rust language support {#tier-1-rust-language-support}

The Fuchsia project recommends
[Tier 1](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-1)
Rust  support for any given architecture, platform, and target board.

##### Rationale

Fuchsia uses Rust extensively. For additional information on the benefits of
Tier 1 support, see
[Tier 1 Target Policy](https://doc.rust-lang.org/nightly/rustc/target-tier-policy.html#tier-1-target-policy).

## Supported hardware specifications {#supported-hardware-specifications}

The following are specifications for **Supported hardware**. Tests, hosted
by the Fuchsia project, are run on Supported hardware as a part of Fuchsia's
continuous integration and testing process. Fuchsia-project hosted testing is
performed on Supported hardware and its results can be seen by external
contributors. Examples of Supported hardware include
the [Intel NUC](/development/hardware/intel_nuc.md)
and VIM3. For more information on the support expectations for
Supported hardware, see [Specification Summary](#hardware-platform-summary).

Supported hardware specifications are further divided into two
categories: Required and Recommended.

Note: This section of the RFC establishes the _current_ and _intial_ set of
technical specifications for Supported hardware. The Supported hardware
category as well as its related Recommended and Required specifications are
subject to change and could be modified as the Fuchsia project progresses.

See the following table for the further details about Required and
Recommended specifications:

<table>
  <tr>
   <td><strong>Supported hardware specification</strong>
   </td>
   <td><strong>Definition</strong>
   </td>
  </tr>
  <tr>
   <td>Required
   </td>
   <td>Without this specification, this hardware platform cannot be categorized
   as Supported hardware.
   </td>
  </tr>
  <tr>
   <td>Recommended
   </td>
   <td>This specification is highly desirable but not required for the
   Supported hardware categorization. Without this specification, Fuchsia's
   functionality or performance will be compromised.
   </td>
  </tr>
</table>

### Required specifications {#required-supported-specifications}

In addition to the [base specifications](#base-hardware-specifications),
Supported hardware must also comply with the following specifications to
be considered for the Supported hardware support tier.

#### Required: Bootloader openness

The final stage bootloader, which is the software component of the boot process
that loads the Fuchsia kernel, must adhere to the following requirements:

*   The Fuchsia project must be able to make changes to the bootloader.
*   The Fuchsia project must be able to build the bootloader.
*   The Fuchsia project must be able to distribute the source and binary that
    result from any changes made by the Fuchsia project to the bootloader.

The bootloader does not need to have a Fuchsia-compatible license. Earlier
stages in the boot process before the bootloader do not need to be open source,
but this is highly preferred if possible. See also
[requirements for documentation and support](#recommended-documentation-and-support).

It is highly desirable for the source to be developed in the open, that the
source project owners accept bug reports and external changes to be
integrated, and that they provide continuing support for the project.

##### Rationale

Open source bootloaders can be modified to directly support loading Fuchsia
rather than using boot shims. Proprietary blobs are difficult to debug or audit
for security. Parts of the early boot process are often unavailable as open
source and would be impractical to replace with open versions; for example
vendor proprietary versions of the ARM Trusted Execution Environment, or
Unified Extensible Firmware Interface (UEFI) implementations on x86 platforms.

##### Examples

[coreboot](https://coreboot.org), [U-boot](https://www.denx.de/wiki/U-Boot).

#### Required: Architecture must be supported by QEMU

The architecture must be supported by the latest Fuchsia project QEMU
version. It is desirable but not necessary that:

* The specific platform is supported.
* The architecture is virtualizable under QEMU.
* QEMU be able to run on host QEMU machines with the same architecture
  (i.e. QEMU-kvm x86 on x86 host, or QEMU-kvm arm64 on arm64 host).

If the architecture has modes that are targeted, for example ARM Trustzone, QEMU
should be able to emulate them.

##### Rationale

To provide scalable hardware support for building and testing, QEMU is used for
continuous integration. Without QEMU support for a target architecture, code
cannot be built or tested in a scalable way.

#### Required: Serial console access

The hardware platform must support some form of serial console access for
development purposes. It is not required to be present for production systems
delivered to end users. The serial console must support interrupt driven TX and
RX.

DMA support is a nice to have feature, although using DMA must not be required.

##### Rationale

A simple serial console is the most reliable way to develop and debug during
early platform development.

### Recommended specifications {#recommended-supported-specifications}

While not required for the Supported hardware tier, the following specifications
are highly desirable supplements to the Required Supported specifications.

#### Recommended: Documentation and support {#recommended-documentation-and-support}

The platform should have reasonable publicly-available documentation, including:

* Register maps
* Theory of operation
* Information about boot time hardware state

A board that is only documented using a fork of Linux or the Android Open Source
Project is not acceptable. The platform vendor must be willing to answer
questions either in person or through email.

##### Rationale

Without accurate documentation, writing drivers and debugging is reduced to
reverse engineering existing software, which is error-prone and slow. Using
existing source code for documentation that has a license incompatible with
Fuchsia's license may risk inadvertent copying of code.

#### Recommended: Bootloader support for fastboot

The Fuchsia project recommends that the bootloader support the set of fastboot
protocol commands listed in the [Appendix](#appendix).
The Fuchsia project recommends that fastboot occurs on non-proprietary
transports.

##### Rationale

Uniformly supporting fastboot makes it easier to work with different
hardware platforms, and assists with automation when dealing with fleets of
multiple machines.

#### Recommended: Virtualization support

The Fuchsia project recommends that the platform and bootloader allow full
virtualization support.

##### Intel x86

Within the Fuchsia project, the following is needed to fully support
virtualization on Intel x86 CPUs:

*   VMX
*   EPT
*   RDTSCP
*   x2APIC
*   VPID
*   Unrestricted guests
*   TPR virtualization
*   MSR bitmaps
*   Exception bitmaps

Optionally, the Fuchsia project recommends:

*   INVPCID
*   PAUSE-loop exiting

##### ARM

Within the Fuchsia project, the following is needed to fully support
virtualization on ARM:

*   ARMv8.0
*   EL2 access
*   Host physical timer / guest virtual timer split
*   GICv2 or GICv3
*   GIC virtualization

##### Rationale

Virtualization is useful for many purposes. For example on ARM64, if
Zircon executes at EL2, EL1/EL0 can be virtualized for improved isolation.

##### Examples

ARM: Armv8-A AArch64; x86: Intel VT-x, AMD VT

#### Recommended: Hardware assisted tracing

The hardware platform should support some form of hardware control flow tracing.

##### Rationale

Low-cost tracing makes it more feasible to do automated feedback-directed
optimization (AutoFDO) of release builds.

##### Examples

ARM: CoreSight ETM; x86: Intel last branch records (LBR).

## Hardware platform categories {#hardware-platform-categories}

The Fuchsia project has two categories for Fuchsia-compatible hardware
platforms: Supported and Experimental. Fuchsia's hardware specifications differ
based on the category. Both Supported and Experimental hardware must fulfill
their required [base specifications](#base-hardware-specifications).
While not required, the Fuchsia project encourages both Supported and
Experimental hardware to comply with their respective Recommended
specifications, as listed in the
[Hardware platform summary](#hardware-platform-summary).

For a summary of the differences between Supported and Experimental
hardware, see [Hardware platform summary](#hardware-platform-summary).

### Supported hardware {#supported-hardware}

Supported hardware has the following benefits:

*   Supported hardware is tested by the Fuchsia project.
*   No Experimental hardware contributions can break Supported hardware.
*   The Fuchsia project is committed to responding to Monorail issues filed
against Supported hardware. [Members](/contribute/community/contributor-roles.md#member), as
defined in Fuchsia contributor roles, can propose patches to add new Supported hardware platforms
or fix existing Supported hardware platforms.

### Experimental hardware {#experimental-hardware}

Experimental Hardware platforms should not be considered as appropriate for
production environments. Rather, the Fuchsia project considers Experimental
hardware as avenues for exploring and learning about Fuchsia. Experimental
hardware is built on a best-effort basis, meaning, Fuchsia may or may not work
at any given point in time on that given hardware platform.

Experimental hardware is not tested by the Fuchsia project and is not guaranteed
to work, but [Members](/contribute/community/contributor-roles.md#member) who develop for Fuchsia
on the Experimental tier hardware can propose patches
to add a new Experimental hardware or fix existing Experimental hardware.
Gerrit changes that add new Experimental hardware or alter existing Experimental
hardware may not break Supported hardware or introduce external code
dependencies in Zircon or other Fuchsia project code.

Members developing for hardware within the Experimental hardware category only
need to comply with the [base specifications](#base-hardware-specifications).

### Hardware platform summary {#hardware-platform-summary}

The following table summarizes Fuchsia's hardware specifications categories as
well as the availability and testing expectations associated with the Supported
hardware and Experimental hardware platforms.

<table>
  <tr>
   <td><strong>Hardware platform category</strong>
<p>

   </td>
   <td><strong>Capabilities</strong>
   </td>
   <td><strong>Accepts contributions</strong>
   </td>
   <td><strong>Availability expectations</strong>
<p>
   </td>
   <td><strong>Testing expectations</strong>
   </td>
   <td><strong>Required specifications</strong>
   </td>
   <td><strong>Recommended specifications</strong>
   </td>
  </tr>
  <tr>
   <td>Supported hardware
   </td>
   <td><ul>

<li>Developers can expect to build and install Fuchsia on Supported hardware.

<li>This hardware may not depend on any <a href="/contribute/governance/policy/open-source-licensing-policies#external_code">external code</a>. </li>
</li>

</td>
<td>
   Yes, provided that these contributions don't break Supported hardware or
   introduce Experimental code dependencies in Zircon or other
   non-Experimental code.
</td>
<td><ul>

<li>The Fuchsia project tests against Supported hardware and blocks
contributions that might cause Supported hardware to break.

<li>Monorail Issues filed against hardware in the Supported hardware tier are
responded to but there is no service level objective (SLO).</li>
</li></ul>

</td>
<td><ul>

<li>Fuchsia project-hosted testing is performed on Supported hardware. All
contributors can see results of these tests.</li></ul>

   </td>
   <td>

<a href="#base-hardware-specifications">All Required Base hardware specifications</a> which include:
<ul>
<li><a href="#required-base-hardware-features">Required Hardware Features</a></li>
<li><a href="#required-base-toolchain-and-language-support">Required toolchain and language support</a></li>
</ul> and
<a href="#required-supported-specifications">All Required Supported hardware specifications</a>

   </td>
   <td>


 <a href="#recommended-supported-specifications">All Recommended Supported specifications</a>
   </td>
  </tr>
  <tr>
   <td>Experimental hardware
   </td>
   <td><ul>

<li>Developers can expect to build and install Fuchsia on Experimental
hardware.

<li>Any external contributor may propose patches to add a new Experimental
hardware platform, but changes may not break Supported hardware, or introduce
Experimental code dependencies in Zircon or other common code. </li></ul>
</li>

   </td>
<td>
   Yes, provided that these contributions don't break Supported hardware or
   introduce Experimental code dependencies in Zircon or other non-Experimental code.
</td>
<td><ul>

<li>Experimental hardware is built on a best-effort basis and may or may not
work at any given point in time.

<li>Any Experimental contribution that could break Supported hardware
is rejected.</li>
</li></ul>

   </td>
   <td><ul>

<li>No Fuchsia project-hosted hardware-specific testing is done for Experimental
hardware beyond the standard testing done to submit and merge any Fuchsia
commit.</li></ul>

   </td>
   <td>

<a href="#base-hardware-specifications">All Required Base hardware specifications</a>
  which include:
  <ul>
  <li><a href="#required-base-hardware-features">Required Hardware Features</a></li>
  <li><a href="#required-base-toolchain-and-language-support">Required toolchain and language support</a></li>
   </ul>
   </td>
   <td>

<a href="#base-hardware-specifications">All Recommended Base hardware specifications</a> which include:
<ul>
<li><a href="#recommended-base-hardware-features">Recommended Hardware Features</a></li>
<li><a href="#recommended-base-toolchain-and-language-support">Recommended toolchain and language support</a></li>
</ul>
   </td>
  </tr>
</table>

## Adding hardware platforms to the Fuchsia source {#add-hardware-to-fuchsia-source}

This section details the process for adding new hardware to the Fuchsia source
and the associated responsibilities with adding hardware to the Fuchsia source.
The Fuchsia project accepts Gerrit contributions to both the Supported hardware
and Experimental hardware platform categories.

### Understanding contribution guidelines {#understanding-contribution-guidelines}

The Fuchsia project encourages contributions to both Supported and Experimental
hardware categories with the fundamental understanding that hardware
contributions cannot break Supported hardware. Any contribution that breaks
Supported hardware won't be merged into the Fuchsia source.

Changes that fix bugs in Fuchsia that would otherwise block Experimental
hardware are encouraged, and go through the typical code review process
described
in [Contribute changes](/development/source_code/contribute_changes.md). However, any proposed change may be rejected if it
introduces a substantial new testing, maintenance, or support burden for
Supported hardware in the Fuchsia project.

### Contribution process {#contribution-process}

The process for adding hardware platforms to the Fuchsia source varies
depending on whether you are adding architecture, boards, or drivers.

#### Experimental contributions {#experimental-contributions-process}

##### Processor Architecture

There is no project-endorsed method for adding an Experimental processor
architecture.

##### Boards and Drivers

To add an Experimental board or driver, use Fuchsia's
[code review process](/development/source_code/contribute_changes.md)
and ensure that your Gerrit change conforms to the Rubric.

###### Code management

In your Gerrit change, make sure to create a folder in the appropriate location.
For example, to add a board, create a new folder
in `/src/devices/boards/drivers`. In your Gerrit change include an OWNERS file
that states the email address of each owner of the board or driver. For more
information on OWNERS' responsibilities, see [Responsibilities](#experimental-contributions-responsibilities).

##### Responsibilities {#experimental-contributions-responsibilities}

In addition to the responsibilities [listed](#experimental-contributions-responsibilities) for OWNERS of non-hardware
related code, boards and driver OWNERS must comply with the following
Service Level Objectives (SLOs):

*   **Code review SLO:** Reply to code review requests for the repository they
own within 3 calendar days. Anyone listed in the repository's OWNERS file can
perform the code review.
    *   Failure to comply with this SLO may result in either of the following
    scenarios:
        *   Any member of the Fuchsia project performing the review.
        *   The temporary disabling of the board or driver from the build
        system.
*   **Refactor SLO:** Implement refactors required to advance the driver SDK
within 5 calendar days. If the refactor is too complex for the 5 day SLO, an
extended period can be defined in the refactor request by the Fuchsia project.
Refactor requests will be sent through e-mail sent to addresses listed in the
respective OWNERS file.
    *   Refactors required to advance the Fuchsia frameworks may be reviewed by
    members of the Fuchsia project without expecting a review from an OWNER of
    a particular board or driver repository. CCing OWNERS on Gerrit changes
    that implement these types of refactorings is encouraged but not
    required. For more information on the CC property in Gerrit, see
    [Change properties](https://gerrit-review.googlesource.com/Documentation/concept-changes.html)
    in the Gerrit documentation.
        *   Refactors include but aren't limited to the following:
            *   [FDF](/development/drivers/concepts/fdf.md),
            driver [FIDL](/sdk/fidl/)
            `[fuchsia.hardware.\*]` and [Banjo](/sdk/banjo/)
            interfaces, syscalls
        *   Failure to comply with the SLO may result in the temporary disabling
        of the board or driver from the build system.

Note: Modifications to FIDL interfaces that are used outside of `fuchsia.git`
must comply with the FIDL
[transitions](/development/source_code/working_across_petals.md)
process.

##### Disabled boards and drivers

Boards and drivers that have been temporarily disabled can be re-enabled once
either the refactor or review that prompted the disabling has been
resolved/completed.

Boards and drivers may be removed from `fuchsia.git `if there are 3 instances
of not complying with the listed SLOs. Compliance with these SLOs indicate clear commitment from the OWNERS to support the
code in their repositories. If you wish to dispute the removal of your
driver from `fuchsia.git`, you must confer with the
Fuchsia Engineering Council through the
[Fuchsia RFC process](/contribute/governance/rfcs/rfc_process.md).

###### Deleted Drivers

For drivers removed based on this policy, a `/reference/hardware/_driver_epitaphs.yaml` file
will list all deleted drivers and include the following information:

*   `Short_description: `Provides a description of the deleted driver.
*   `Tracking_bug`: A Monorail issue that describes the reason for the
    driver's deletion.
*   `Gerrit_change_id: `The ID of the Gerrit change used to delete the driver.
*   `Available_in_git: `The last known git SHA that still includes the driver.
*   `Path: `The path of the deleted driver.

For example:


```
- short_description: 'Qualcomm Peripheral Image Loading driver'
  tracking_bug: '123456'
  gerrit_change_id: '506858'
  available_in_git: 'f441460db6b70ba38150c3437f42ff3d045d2b71'
  path: '/src/devices/fw/drivers/qcom-pil'
```


Note: Generally, deleted drivers are available through the source
control history.

The process to delete a driver includes:

1. Get approval from at least one OWNER of the driver. If all of the OWNERS
have abandoned the driver and are not responding, then an OWNER higher in the
Fuchsia tree needs to approve the CL that performs the deletion.
2. Add an entry in the `/reference/hardware/_driver_epitaphs.yaml` file
for each driver deleted, including a git hash for the fuchsia.git repository
that contained the driver before deletion.

### Supported contributions {#supported-contributions-process}

#### Processor Architecture

To propose adding a new architecture to the Fuchsia source, use
the [Fuchsia RFC process](/contribute/governance/rfcs).
Adding a new architecture can add a significant amount of work to Fuchsia
development and, as such, needs to be resolved through the RFC process.

Any processor added through the RFC process is categorized as
Supported hardware.

##### Code management

If your RFC is Accepted, then you can create a Gerrit change
that includes your proposed architecture and request a code review using
Fuchsia's [code review process](/development/source_code/contribute_changes.md).
In your Gerrit change include an OWNERS file that states
the email address of each owner of the architecture. For more information on
OWNERS' responsibilities, see [Responsibilities](#experimental-contributions-responsibilities).

#### Boards and Drivers

To add a Supported hardware board or driver, use the
[Fuchsia RFC process](/contribute/governance/rfcs).
Multiple drivers can be included in one RFC. A single RFC can also propose
both a board and a driver.

Supported hardware requires infrastructure for Google-hosted testing, which
adds significant resource utilization. Allocation of resources for RFC-approved
Supported hardware may require additional review which is outside the scope of
this document.

Adding a driver for a part of a computer system that was previously approved
through the RFC process does not require a new RFC; it can simply be added
through the
code [review process](/development/source_code/contribute_changes.md).

##### Code management

If your RFC is Accepted, then you can create a Gerrit change that includes
your board or driver and request a code review using Fuchsia's
[code review process](/development/source_code/contribute_changes.md).
In your Gerrit change, make sure to create a folder in the appropriate location.
For example, to add a board, create a new folder in `/src/devices/boards`. In
your Gerrit change, include an OWNERS file that states the email address of
each owner of the board or driver. For more information on OWNERS'
responsibilities, see [Responsibilities](#experimental-contributions-responsibilities).

### Certification

At this time adding new boards or drivers will be only required to comply with
the Rubric as described above.

## Appendix {#appendix}

### Required fastboot commands

The bootloader must reliably fail when given an unsupported fastboot command
(not hang or prevent the device from functioning). Unexpected commands should
either succeed or reliably fail. For more information about fastboot, see
[fastboot](https://android.googlesource.com/platform/system/core/+/refs/heads/master/fastboot).

Note: Commands should return to the fastboot command prompt after terminating,
to allow additional commands.

The following standard fastboot commands are required:

*   erase &lt;partition>
*   flash &lt;partition> &lt;file>
*   getvar &lt;name>
*   reboot
*   reboot bootloader
*   reboot recovery
*   set\_active {a,b}
*   boot &lt;file>

Required getvar variables:

*   current-slot
*   hw-revision
*   max-download-size
*   product
*   serialno
*   slot-retry-count:a
*   slot-retry-count:b
*   slot-successful:a
*   slot-successful:b
*   slot-unbootable:a
*   slot-unbootable:b
*   version
*   all

The following commands are optional but can be helpful for development:

*   oem stage-partition &lt;partition> + get\_staged
