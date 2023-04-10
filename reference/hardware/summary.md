# Hardware requirements and recommendations

This page is a summary of Fuchsia's current hardware requirements and
recommendations. The goal is to provide an easy to understand explanation of
[RFC-0111: Initial hardware platform specifications][RFC]
for hardware vendors who only need to know what hardware features
Fuchsia requires or recommends.

Note: The source of truth for Fuchsia hardware requirements information is
[RFC-0111: Initial Fuchsia hardware platform specifications][RFC]. If there
is a disagreement between the information on this page and the information in
the RFC, the RFC's information should be considered the truth.

## Required features {#requirements}

All required features must be supported. If a required feature is not supported,
Fuchsia will not build or run correctly.

<table>
  <tr>
    <th>Feature</th>
    <th>Area</th>
    <th>Details</th>
    <th>Examples</th>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_for_x86-64_the_x86-64-v2_instruction_set_architecture">
        Instruction set architecture (ISA)
      </a>
    </td>
    <td>
      <a href="#architecture">Architecture</a>
    </td>
    <td>
      <ul>
        <li>
          x86-64 architectures must support x86-64-v2 ISA.
        </li>
        <li>
          Arm architectures must support Armv8.0-A at minimum.
        </li>
    </td>
    <td>
      <ul>
        <li>Intel Westmere and newer CPUs.</li>
        <li>AMD Bulldozer and newer CPUs.</li>
        <li>
          Arm A35, A53, A55, A57, A65, A72, A73,
          A75, A76, A77, A78, and X1.
        </li>
      </ul>
    </td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_architecture_must_be_supported_by_qemu">
        QEMU support
      </a>
    </td>
    <td>
      <a href="#architecture">Architecture</a>
    </td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_little-endian_mode">
        Little endian byte-ordering mode
      </a>
    </td>
    <td>
      <a href="#architecture">Architecture</a>
    </td>
    <td></td>
    <td>
      <ul>
        <li>All x86 CPUs.</li>
        <li>Arm processors support little endian.</li>
      </ul>
    </td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_llvm_toolchain_support">
        LLVM toolchain support
      </a>
    </td>
    <td>
      <a href="#architecture">Architecture</a>
    </td>
    <td>
      <ul>
        <li>
          The architecture must be in the <a href="https://llvm.org/docs/SupportPolicy.html#core-tier">LLVM core tier</a>.
        </li>
        <li>
          Clang toolchain support must be kept up-to-date.
        </li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_tier_2_rust_language_support">
        Tier 2 Rust language support
      </a>
    </td>
    <td>
      <a href="#architecture">Architecture</a>
    </td>
    <td>
      <ul>
        <li>
          The architecture must have <a href="https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-2">Rust Tier 2</a>
          support.
        </li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_dart_language_support">
        Dart language support
      </a>
    </td>
    <td>
      <a href="#architecture">Architecture</a>
    </td>
    <td>
      <ul>
        <li>
          The architecture must support the <a href="https://dart.dev/overview#platform">Dart platform</a>.
        </li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_go_language_support">
        Go language support
      </a>
    </td>
    <td>
      <a href="#architecture">Architecture</a>
    </td>
    <td>
      <ul>
        <li>
          The architecture must have Go language support.
          See <a href="https://go.dev/blog/ports">Go on ARM and beyond</a>.
        </li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_full_featured_memory_management_unit_mmu">
        Full-featured modern memory management unit (MMU)
      </a>
    </td>
    <td>
      <a href="#platform">Platform</a>
    </td>
    <td>
      <ul>
        <li>
          The MMU must support creating an arbitrary number of address spaces.
        </li>
        <li>
          The MMU must map physical memory in reasonably sized pages to any of those spaces.
        </li>
        <li>
          The MMU must enforce protection between separate address spaces through hardware
          access control mechanisms.
        </li>
    </td>
    <td>
      <ul>
        <li>Armv8.0-A and later.</li>
        <li>All x86 CPUs after 2010.</li>
      </ul>
    </td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_64-bit_cpu_and_platform">
        64-bit platform
      </a>
    </td>
    <td>
      <a href="#platform">Platform</a>
    </td>
    <td>
      <ul>
        <li>
          The platform must support 64-bit address space operating mode.
        </li>
        <li>
          The <a href="#esb">early stage bootloader</a> doesn't need to run in
          64-bit mode.
        </li>
      </ul>
    </td>
    <td>
      <ul>
        <li>All x86-64 platforms.
        <li>All Armv8-A and later processors (except A32).</li>
      </ul>
    </td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_clock_and_timers">
        Clocks and timers
      </a>
    </td>
    <td>
      <a href="#system">System</a>
    </td>
    <td>
      <ul>
        <li>Clocks and timers must not arbitrarily change frequency.</li>
        <li>Clocks and timers must be at least 56-bits wide.</li>
        <li>Clocks and timers must have a rollover time greater than 40 years.</li>
        <li>
          Clocks and timers must have an explicitly knowable nominal
          frequency that doesn't require runtime calibration.
        </li>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_bootloader_openness">
        Bootloader openness
      </a>
    </td>
    <td>
      <a href="#system">System</a>
    </td>
    <td>
      <ul>
        <li>
          Fuchsia must be able to change and build the <a href="#fsb">final-stage bootloader</a>.
        </li>
        <li>
          Fuchsia must be able to distribute the source and binary of any changes it makes to the
          <a href="#fsb">final-stage bootloader</a>.
        </li>
      </ul>
    </td>
    <td>
      <ul>
        <li><a href="https://coreboot.org/">coreboot</a>.</li>
        <li><a href="https://www.denx.de/wiki/U-Boot">U-Boot</a>.</li>
      </ul>
    </td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_serial_console_access">
        Serial console access
      </a>
    </td>
    <td>
      <a href="#system">System</a>
    </td>
    <td>
      <ul>
        <li>
          The system must support interrupt-driven TX and RX during development.
        </li>
        <li>
          Serial console access is not required for production systems intended
          for end users.
        </li>
        <li>
          (Recommended) Direct memory access (DMA).
        </li>
      </ul>
    </td>
    <td></td>
  </tr>
</table>

## Recommended features {#recommendations}

Recommended features are not required to build or run Fuchsia correctly but are
highly desirable because they improve Fuchsia's base functionality.

<table>
  <tr>
    <th>Feature</th>
    <th>Area</th>
    <th>Details</th>
    <th>Examples</th>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#recommended_virtualization_support">
        Virtualization support
      </a>
    </td>
    <td><a href="#architecture">Architecture</a></td>
    <td>
      <ul>
        <li>
          Intel x86 CPUs: VMX, EPT, RDTSCP, x2APIC, VPID, unrestricted guests,
          TPR virtualization, MSR bitmaps, exception bitmaps, INVPCID
          (recommended), PAUSE-loop exiting (recommended).
        </li>
        <li>
          Arm CPUs: Armv8.0, EL2 access, host physical timer / guest virtual
          timer split, GICv2 or GICv3, GIC virtualization.
        </li>
      </ul>
    </td>
    <td>
      <ul>
        <li>Armv8-A AArch64.</li>
        <li>Intel VT-x.</li>
        <li>AMD VT.</li>
      </ul>
    </td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#recommended_clock_and_timers">
        Clocks and timers
      </a>
    </td>
    <td><a href="#architecture">Architecture</a></td>
    <td>
      <ul>
        <li>
          Timers should deliver interrupts to cores when timer values exceed a given
          absolute threshold.
        </li>
        <li>
          Clocks and timers should be implemented as part of the architecture itself, not as a peripheral.
        </li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#recommended_io_memory_management_unit_iommu">
        I/O memory management unit (IOMMU)
      </a>
    </td>
    <td><a href="#platform">Platform</a></td>
    <td>
      <ul>
        <li>
          The IOMMU should be able to <a href="https://www.techtarget.com/searchnetworking/definition/NAK">NAK</a>
          read and write transactions initiated from
          uniquely identifiable hardware units.
        </li>
        <li>
          The IOMMU should do address translation for hardware DMA operations.
        </li>
        <li>
         IOMMU page-fault situations should be detectable and debuggable.
        </li>
    </td>
    <td>
      <ul>
        <li>System Memory Management Unit (SMMU) in Arm IOMMU spec.</li>
        <li>Intel x86 IOMMU spec.</li>
        <li>Intel VT-d.</li>
        <li>AMD-Vi.</li>
      </ul>
    </td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#recommended_hardware_cryptographic_acceleration_support">
        Hardware cryptographic acceleration support
      </a>
    </td>
    <td><a href="#platform">Platform</a></td>
    <td>
      <ul>
        <li>
          The platform should provide hardware acceleration for AES and SHA.
        </li>
      </l>
    </td>
    <td>
      <ul>
        <li>
          AES for Armv8.0-A (Cortex A34) or later: AESE, AESD, AESMC, AESIMC,
          PMULL, PMULL2.
        </li>
        <li>
          SHA2 for Armv8.0-A (Cortex A34) or later: SHA256H, SHA256H2, SHA256UO,
          SHA256U1.
        </li>
      </ul>
    </td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#recommended_hardware_assisted_tracing">
        Hardware-assisted tracing
      </a>
    </td>
    <td><a href="#platform">Platform</a></td>
    <td>
      <ul>
        <li>
          The platform should support <a href="https://menloparktech.com/hardware-tracing.html">hardware-assisted tracing</a>
          of <a href="https://en.wikipedia.org/wiki/Control_flow">control flow</a>.
        </li>
      </ul>
    </td>
    <td>
      <ul>
        <li>Arm CoreSight ETM.</li>
        <li>Intel Last Branch Records (LBR).</li>
      </ul>
    </td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#recommended_gcc_toolchain_support">
        GCC toolchain support
      </a>
    </td>
    <td><a href="#system">System</a></td>
    <td>
      <ul>
        <li>
          The system should be fully supported by the GCC toolchain.
        </li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#tier-1-rust-language-support">
        Tier 1 Rust language support
      </a>
    </td>
    <td><a href="#system">System</a></td>
    <td>
      <ul>
        <li>
          The system should have <a href="https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-1">Rust Tier 1</a> support.
        </li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#recommended-documentation-and-support">
        Documentation and support
      </a>
    </td>
    <td><a href="#system">System</a></td>
    <td>
      <ul>
        <li>
          The system should have publicly available documentation about register maps,
          theory of operation, and boot-time hardware state.
        </li>
        <li>
          Board documentation can't be a fork of the documentation source code for Linux
          or Android Open Source Project or any other project that has a license incompatible
          with Fuchsia.
        </li>
        <li>
          The system vendor must provide support channels where Fuchsia contributors can get their
          questions answered.
        </li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>
      <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#recommended_bootloader_support_for_fastboot">
        Fastboot support
      </a>
    </td>
    <td><a href="#system">System</a></td>
    <td>
      <ul>
        <li>
          The bootloader should support fastboot over non-proprietary transports. See
          <a href="/docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#required_fastboot_commands">Required fastboot commands</a>
          for the full list of fastboot protocol commands that should be supported.
        </li>
      </ul>
    </td>
    <td></td>
  </tr>
</table>

## Appendix: Terminology {#terminology}

These terminology definitions are based on [Document Definitions][terms].

### Architecture {#architecture}

A processor architecture like x86 or Arm.

### Early stage bootloader {#esb}

All stages of the bootloader before the [final-stage bootloader](#fsb).

### Final-stage bootloader {#fsb}

The software component that loads Fuchsia's kernel.

### Platform {#platform}

The system-on-a-chip (SoC) or the combination of the CPU and
chipset.

### System {#system}

A complete computer hardware system with a CPU, memory, peripherals
and so on. Also called a *board*.

[RFC]: /docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md
[terms]: /docs/contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md#document-definitions
