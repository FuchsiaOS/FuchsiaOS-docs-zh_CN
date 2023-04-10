<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0203" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC proposes a mechanism by which a userspace agent may interact with the
kernel to access the energy consumption information across a set of power
domains on platforms that support Running Average Power Limit (RAPL) interfaces.

## Motivation

Energy efficiency and consumption are a key design element to laptop computers
which are not permanently tethered to a power supply. X86 RAPL is an interface
on modern X86 CPUs that accurately estimates energy consumption across a set of
power domains by using hardware performance counters and I/O models. If we have
a mechanism to make energy consumption information accessible outside the
kernel, we can build developer-facing and fleet-tracking tools to inspect the
run-time power draw on an device in an ergonomic way, which is useful for
optimizing power footprint during development and identifying power regression
across the fleet.

## Design

### Background

#### Intel x86 RAPL MSRs {#intel-rapl-msrs}

In RAPL, platforms are divided into domains for fine grained reports and
control. A RAPL domain is a physically meaningful domain for power management.
The specific RAPL domains available in a platform vary across processor
families/models, including:

- PSys: Entire SoC.
- Package: The processor die (all cores, integrated graphics, caches, memory
  controller).
- Power Plane PP0: All processor cores and private caches.
- Power Plane PP1: The power plane of a specific device in the uncore, which is
  usually the integrated GPU.
- Memory: DRAM attached to the integrated memory controller.

RAPL interfaces consist of non-architectural MSRs. Each RAPL domain supports the
following set of capabilities, some of which are optional as stated below:

- `Power Limit`: MSR interfaces to specify power limit, time window; lock bit,
  clamp bit etc.
- `Energy Status`: Power metering interface providing energy consumption
  information.
- `Perf Status` (Optional): Interface providing information on the performance
  effects (regression) due to power limits. It is defined as a duration metric
  that measures the power limit effect in the respective domain. The meaning of
  duration is domain specific.
- `Power Info` (Optional): Interface providing information on the range of
  parameters for a given domain, minimum power, maximum power etc.
- `Policy` (Optional): 4-bit priority information that is a hint to hardware for
  dividing budget between sub-domains in a parent domain.

Each of the above capabilities requires specific units in order to describe
them. Units are exposed in the read-only `MSR_RAPL_POWER_UNIT` MSR. Scaling
factors are supplied to each unit to make the information presented meaningful
in a finite number of bits.

The MSRs storing energy consumption information across different domains are the
read-only `Energy Status` MSRs:

- MSR_PLATFORM_ENERGY_STATUS (0x0000064d)
- MSR_PKG_ENERGY_STATUS (0x00000611)
- MSR_DRAM_ENERGY_STATUS (0x00000619)
- MSR_PP0_ENERGY_STATUS (0x00000639)
- MSR_PP1_ENERGY_STATUS (0x00000641)

The energy unit of above MSRs is specified by the `Energy Status Units` field of
`MSR_RAPL_POWER_UNIT`.

The `Energy Status` MSRs are supported for a finite range of processor
families/models. Code that accesses a non-architectural MSR and that is executed
on a processor that does not support that MSR will generate an exception. The
following are non-exhaustive lists of processors models that support each MSR:

`MSR_PLATFORM_ENERGY_STATUS` is available on the following processors:

- Skylake, family: 6, model: 0x4e, 0x5e
- Kaby Lake, family: 6, model: 0x8e, 0x9e
- Cannon Lake, family: 6, model: 0x66

`MSR_PKG_ENERGY_STATUS` is available on the following processors:

- Sandy Bridge, family: 6, model: 0x2a, 0x2d
- Ivy Bridge, family: 6, model: 0x3a, 0x3e
- Haswell, family: 6, model: 0x3c, 0x3f, 0x45, 0x46
- Broadwell, family: 6, model: 0x3d, 0x47, 0x56, 0x4f
- Skylake, family: 6, model: 0x4e, 0x5e
- Kaby Lake, family: 6, model: 0x8e, 0x9e
- Skylake X/SP, family: 6, model: 0x55
- Cannon Lake, family: 6, model: 0x66
- Ice Lake-SP, family: 6, model: 0x6a

`MSR_DRAM_ENERGY_STATUS` is available on the following processors:

- Haswell, family: 6, model: 0x3c, 0x3f, 0x45, 0x46
- Broadwell, family: 6, model: 0x3d, 0x47, 0x56, 0x4f
- Skylake, family: 6, model: 0x4e, 0x5e
- Kaby Lake, family: 6, model: 0x8e, 0x9e
- Skylake X/SP, family: 6, model: 0x55
- Cannon Lake, family: 6, model: 0x66
- Ice Lake-SP, family: 6, model: 0x6a

`MSR_PP0_ENERGY_STATUS` is available on the following processors:

- Sandy Bridge, family: 6, model: 0x2a, 0x2d
- Ivy Bridge, family: 6, model: 0x3a, 0x3e
- Haswell, family: 6, model: 0x3c, 0x3f, 0x45, 0x46
- Broadwell, family: 6, model: 0x3d, 0x47, 0x56, 0x4f
- Skylake, family: 6, model: 0x4e, 0x5e
- Kaby Lake, family: 6, model: 0x8e, 0x9e
- Skylake X/SP, family: 6, model: 0x55
- Cannon Lake, family: 6, model: 0x66
- Ice Lake-SP, family: 6, model: 0x6a

`MSR_PP1_ENERGY_STATUS` is available on the following processors:

- Skylake, family: 6, model: 0x4e, 0x5e
- Kaby Lake, family: 6, model: 0x8e, 0x9e
- Cannon Lake, family: 6, model: 0x66

#### AMD x86 RAPL MSRs

Recent AMD processors support an MSR interface that is semi-compatible with
[Intel RAPL MSRs](#intel-rapl-msrs). The supported RAPL MSRs have the same
contents, but the MSR numbers are different. Specifically, AMD Zen (family:
0x17, 0x19) processors support the following `Energy Status` MSRs:

- MSR_AMD_CORE_ENERGY_STATUS (0xc001029a)

  - Equivalent to Intel `MSR_PP0_ENERGY_STATUS` of the `Power Plane PP0` domain.

- MSR_AMD_PKG_ENERGY_STATUS (0xc001029b)

  - Equivalent to Intel `MSR_PKG_ENERGY_STATUS` of the `Package` domain.

### Syscall: zx_system_energy_info

#### Signature

This syscall allows a userspace agent to access the energy consumption
information stored in the RAPL MSRs of the specified power domain:

```c
zx_status_t zx_system_energy_info(
    zx_handle_t resource,
    uint32_t domain,
    uint64_t* energy_uj
);
```

Its arguments are:

`resource`: A resource that grants permission to this call. Must be
`ZX_RSRC_SYSTEM_ENERGY_INFO_BASE`, a new resource introduced specifically for
this API, or the call will fail.

`domain`: The RAPL domain referenced by this call. It takes any of the following
value to specify the RAPL domain:

- ZX_ENERGY_X64_PLATFORM_DOMAIN
- ZX_ENERGY_X64_PKG_DOMAIN
- ZX_ENERGY_X64_DRAM_DOMAIN
- ZX_ENERGY_X64_PP0_DOMAIN
- ZX_ENERGY_X64_PP1_DOMAIN

The above domains will be defined upon proposal implementation.The domain
determines the content written to `energy_uj`, described below.

`energy_uj`: If the call succeeds, then upon return `energy_uj` contains the
energy value in uj. The energy value is determined from the value stored in the
`Energy Status` MSR of the specified domain and the unit stored in the
`Energy Status Units` field of `MSR_RAPL_POWER_UNIT`. If the call fails, its
value is unspecified.

#### Error conditions

`ZX_ERR_NOT_SUPPORTED`

- The processor is not a [supported x86-64 processor](/docs/contribute/governance/rfcs/0073_x86_64_platform_requirement.md).

- The processor is a supported x86-64 processor, but the `Energy Status` MSR of
  the specified domain is not available on this processor.
- The processor is a supported x86-64 processor and  the `Energy Status` MSR of
  the specified domain is available on this processor, but the reading operation
  is not enabled.

`ZX_ERR_BAD_HANDLE`

- Resource is not a valid handle.

`ZX_ERR_WRONG_TYPE`

- Resource is not a valid resource handle or is not of kind
  `ZX_RSRC_KIND_SYSTEM`.

`ZX_ERR_INVALID_ARGS`

- `domain` is not one of the following:

    - ZX_ENERGY_X64_PLATFORM_DOMAIN
    - ZX_ENERGY_X64_PKG_DOMAIN
    - ZX_ENERGY_X64_DRAM_DOMAIN
    - ZX_ENERGY_X64_PP0_DOMAIN
    - ZX_ENERGY_X64_PP1_DOMAIN

- `energy_uj` is an invalid pointer.

`ZX_ERR_OUT_OF_RANGE`

- Resource is of kind `ZX_RSRC_KIND_SYSTEM` but is not equal to
  `ZX_RSRC_SYSTEM_ENERGY_INFO_BASE`.

## Implementation

### Kernel

The new syscalls must be implemented, gated by a new resource
`ZX_RSRC_SYSTEM_ENERGY_INFO_BASE`.
Fuchsia kernel already has APIs to query the manufacturer id and processor
signature/id using CPUID. DisplayFamily and DisplayModel have been mapped to
processor ids for various x86 processors, and can be used to determine the
availability of the MSRs.

### Component manager

A new protocol `EnergyInfoResource` must be defined and must be implemented by
Component Manager to provide the `ZX_RSRC_SYSTEM_ENERGY_INFO_BASE` resource.
This follows a pre-existing pattern for resources that gate syscalls.

## Performance

There is no known impact on performance. The new syscall will take a negligible
amount of time to execute, as it simply touches a small amount of data.

## Security considerations

The syscall is gated by the new resource handle
`ZX_RSRC_SYSTEM_ENERGY_INFO_BASE`. This protection addresses the concern of
malicious interference with the kernel.

## Privacy considerations

This proposal has no meaningful impact on privacy.

## Testing

- Unit tests can be added to validate the returned `energy_uj` is converted
correctly from the values stored in the MSRs.

## Documentation

The Zircon syscall documentation will be updated to include the new API.

## Drawbacks, alternatives, and unknowns

A more general interface was considered, such as a `zx_get_msr` syscall that
would allow farming out other useful MSR features. However, non-architectural
MSR access needs to be gated by its availability on the processor. Requirements
and scope of a more general interface would be largely guesswork at this point.
Eventually, we opted for a narrow interface to keep the implementation fairly
easy and the cost of future changes to the proposed interface relatively small.

## Prior art and references

`zx_system_powerctl` syscall called with cmd
`ZX_SYSTEM_POWERCTL_X86_SET_PKG_PL1` manipulates the CPU power level using the
RAPL `Power Limit` and `Power Info` MSRs. The resource used to validate this
syscall has the resource kind `ZX_RSRC_KIND_SYSTEM` with base
`ZX_RSRC_SYSTEM_POWER_BASE`.
