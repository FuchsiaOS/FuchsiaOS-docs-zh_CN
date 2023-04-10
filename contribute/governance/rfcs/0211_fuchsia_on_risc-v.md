<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0211" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC proposes adding support for the RISC-V CPU architecture to Fuchsia.
The proposed board name is `riscv64`.

## Motivation

RISC-V is a free and open instruction set architecture originally developed at
[UC Berkeley](https://riscv.org/about/history/#berkeley) which has become
increasingly popular in recent years. RISC-V International, a nonprofit based
in Switzerland whose founding members include Google, defines the RISC-V
specifications. All work is done through an open collaboration process that
takes place on GitHub and the resulting specifications are freely available to
the public. The RISC-V architecture is designed to be flexible enough to be
used for everything between small microcontrollers and server-class CPUs. It
has 32- and 64-bit address space variants and defines sets of instruction
extensions which are grouped into categories that are convenient for modern
operating system support. In addition, implementations can define custom
non-standard instructions for special applications.

The recent availability of high-performance 64-bit cores from companies like
SiFive has moved RISC-V from being primarily a small microcontrollers solution
into the space where Fuchsia is currently doing the most development. We
believe this is the perfect time for Fuchsia to support this architecture. By
building robust RISC-V support now, we will ensure that Zircon and Fuchsia will
be ready to support the next generation of computing devices from the very
beginning of their appearance in the market.

The open philosophy of RISC-V is a good fit with our goals for the open source
Fuchsia project and will help us in collaborating with other stakeholders in
the RISC-V world.

## Stakeholders

_Facilitator:_

- jamesr@google.com

_Contributors:_

- curtisgalloway@google.com - RFC wrangler
- revest@google.com, rogerta@google.com - pioneer Fuchsia on RISC-V 20%-ers
- cpu@google.com - RISC-V champion
- travisg@google.com - Kernel wrangler emeritus

_Reviewers:_

- phosek@google.com *(Toolchain)*
- shayba@google.com *(Testing, build system)*
- mcgrathr@google.com *(Zircon)*
- dpursell@google.com *(Firmware)*
- mkearney@google.com *(Documentation)*
- mvanotti@google.com *(Security)*

_Consulted:_

- keir@google.com *(Pigweed)*

## Design

### Target hardware

Fuchsia only targets 64-bit processors. So RISC-V support will target 64-bit
CPUs only. Here we propose that only the QEMU
[virt](https://www.qemu.org/docs/master/system/riscv/virt.html) platform is
targeted. AEMU support is not considered in this RFC and as such, workflows or
testing that require AEMU will not be supported. Support for a specific
development board will be proposed separately in a future RFC.

### ISA Extensions

RISC-V defines common sets of ISA extensions as
[profiles](https://github.com/riscv/riscv-profiles). We'll align our
requirements with official profile definitions and work with the RISC-V
community on future profile definitions.

As a first step, we'll only require RVA20 profiles since they are supported by
QEMU. User code can assume that RVA20U64 extensions are available on all
Fuchsia systems without runtime feature checks. The platform itself requires
hardware/firmware to support the RVA20S64 profile.

We expect a future RFC to update these policies before RISC-V Fuchsia systems
are deployed on hardware.

### SBI Firmware

The kernel will only run in Supervisor mode (S-Mode) and will require the help
of a lower level firmware running in Machine mode (M-mode). The
[SBI specification](https://github.com/riscv-non-isa/riscv-sbi-doc/releases/download/v1.0.0/riscv-sbi.pdf)
provides services to, for example, start secondary cores, timers and use remote
fences. This spec is typically implemented by open-source firmware
implementations such as OpenSBI; QEMU ships with a prebuilt
[OpenSBI 1.0](https://github.com/riscv-software-src/opensbi) which Zircon will
require as a baseline.

### Boot

On the QEMU "virt" platform, we will start with Zircon getting booted by a
`boot-shim` similar to the one used in arm64. The boot-shim gets
[DeviceTree](https://www.devicetree.org/) data describing the QEMU
configuration and presents ZBI items representing those details according to
the ZBI boot protocol. New ZBI item types specific to RISC-V needs will be
specified in the ZBI protocol as needed. In the future, we could use other
firmwares to boot into Zircon.

### Virtual memory

The RISC-V MMU supports multiple modes with different page sizes and maximum
page table levels. At first, we'll only concentrate on sv39 since it is the
baseline implemented in most hardware. In this mode, 39 bits of total address
space are available which Zircon will use to provide 38 bits of user address
space. As Sv48 becomes more widely available, we may extend Zircon to support
larger user address spaces.

### Target product

This RFC proposes targeting the "bringup" configuration followed by the
"minimal" enabled with Netstack 3 so there are no dependencies on Go. An "eng"
variant of the minimal configuration needs to be created such that standard
developer workflows are available without further modification.

### Drivers

For the targeted configuration, we need PLIC, PCI, Ethernet, Console, PCI and
RNG drivers. PLIC and PCI drivers need work but it is believed that the rest
can use the existing Virtio-based ARM drivers as-is, this needs to be verified.

### Conventions

Every time a RISC-V standard is provided, for example the
[ELF psABI](https://github.com/riscv-non-isa/riscv-elf-psabi-doc) or the
[interrupt stack handling recommendation](https://github.com/riscv/riscv-fast-interrupt/blob/master/clic.adoc#13-managing-interrupt-stacks-across-privilege-modes),
Fuchsia will follow the standard practices unless such standard and
recommendation precludes the agreed upon support level as defined below.

For example, we have already specified x18 (s2) as a reserved register for
shadow call stacks. We will endeavor to work with the community to update the
psABI spec to permit platforms to reserve x18 in particular.

### Support level

The support level should be the "base" level as described in
[RFC-0111](0111_fuchsia_hardware_specifications.md#experimental-hardware).
This requires LLVM, Rust and support upstream so that at least the "minimal"
configuration can be targeted. In addition, the SDK will be updated so
out-of-tree development targeting RISC-V is possible. Consult the RFC-0111 for
the full details. In addition, basic diagnostics like crash dump reports and
interactive debugging support are in scope.

## Performance

This RFC should not impact the performance of Fuchsia on existing
architectures, as there are no architecture-independent changes proposed. The
performance of Fuchsia on QEMU on RISC-V only affects the speed of the CI/CQ
test pipeline and as such performance work shall be limited to make this
scenario reasonably fast, which mostly entails running the test suites without
timing out.

## Security considerations

We propose to target the following baseline security mitigations:

- Stack canaries
- Shadow call stacks

Outside of the scope of this RFC are KASLR and architectural side-channel
mitigations.

## Testing

Automated testing of the `riscv64` architecture support will come from the
continuous integration where fuchsia will be booted in the QEMU `virt`
platform.

## Documentation

The fuchsia.dev documentation needs to be overhauled to include RISC-V in many
places, such that a regular developer can find the necessary information to
contribute RISC-V specific code. Some examples below:

- New entry in `concepts/architecture/architecture_support.md`
- Image and board support in `concepts/emulator/index.md`
- Supported configurations in `concepts/testing/sanitizers.md`
- All major targets in `contribute/contributing_to_zircon.md`

## Drawbacks, alternatives, and unknowns

This proposal adds a 3rd major architecture to Fuchsia and as such will incur
significant one-time engineering costs for:

- Toolchain & build system across the 3 supported languages
- Libc, firmware and low-level kernel support
- `zxdb` and `crashpad` work
- Architecture specific kernel work
- SDK and documentation work

In addition, it will require ongoing costs for:

- Build maintenance
- CI/CQ work for RISCV
- Incremental cost for Infra
- Ongoing higher-level library porting for full support

As RISC-V is still a relatively young architecture, we expect to encounter
unknown challenges as we work our way up the stack for full Fuchsia support.
Two such examples are Virtualization support and trusted execution environment.

Lack of server-class RISC-V hardware makes QEMU emulation slow and might make
it difficult to run all the relevant test suites or use more elaborate product
configurations.

In addition, some third-party tools and libraries may need additional work for
RISC-V compatibility which we have not accounted for here and that might
require convincing upstream projects to adopt RISC-V or force us to maintain
forks.

We could delay support for RISC-V until 64-bit hardware is more mature, but
delay would risk falling behind the introduction of hardware proposed for
production devices, making Fuchsia a less viable option as an OS. RISC-V being
the most viable competitor to Aarch64 and x86-64 will no doubt make it a
popular architecture, resulting in lower costs and more freedom to innovate.
The longer we wait, the more chance there is for RISC-V to evolve in ways that
make a later Fuchsia port more painful or prohibitive.

## Prior art and references

- [Proof of concept branch](https://fuchsia.googlesource.com/fuchsia/+log/d381548c6aef76926e6203a2ad2265dd510d1e9b)
- [Linux port to RISC-V](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/arch/riscv)
- [LK port to RISC-V](https://github.com/littlekernel/lk/tree/master/arch/riscv)
- [RISC-V SBI specification](https://github.com/riscv-non-isa/riscv-sbi-doc)
- [OpenSBI reference implementation](https://github.com/riscv-software-src/opensbi)
- [RISC-V calling conventions](https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-cc.adoc)
- [RISC-V instruction set manual volume II: privileged architecture](https://github.com/riscv/riscv-isa-manual/releases/download/Priv-v1.12/riscv-privileged-20211203.pdf)
- [RISC-V Profiles](https://github.com/riscv/riscv-profiles/blob/main/profiles.adoc)
