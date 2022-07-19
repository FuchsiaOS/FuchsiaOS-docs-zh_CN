# zx_system_set_performance_info

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Set CPU performance parameters.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_system_set_performance_info(zx_handle_t resource,
                                           uint32_t topic,
                                           const void* info,
                                           size_t count);
```

## DESCRIPTION

`zx_system_set_performance_info()` sets CPU performance parameters maintained by the kernel. The
*topic* parameter indicates what specific parameters are affected.

*count* indicates the number of items to set. Topics may impose requriements on this value.

*info* is a pointer to a buffer of sufficient size to accommodate *count* entries of the type
specified by *topic*.

[TOC]

## TOPICS

### ZX_CPU_PERF_SCALE

*count*: Must be non-zero and less than or equal to the number of logical CPUs in the system.

*info* type: `zx_cpu_performance_scale_t[count]`

An array of `zx_cpu_performance_scale_t` with entries specifying the performance scales (scalar
values representing relative operating points) of the respective logical CPUs in the system. Only
the CPUs specified in the array are updated, other CPUs are not affected.

Logical CPU numbers must be stored in increasing order in the array. The scale values are unsigned
fixed point Q32.32 format. The scale values should indicate the relative performance of the CPUs,
with 1.0 corresponding to the highest operating point of the fastest CPU in the system.

See [RFC 0123](/contribute/governance/rfcs/0123_cpu_performance_info.md)
for further details on values and update protocols.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*resource* must have resource kind **ZX_RSRC_KIND_SYSTEM**.

## RETURN VALUE

`zx_system_set_performance_info()` returns **ZX_OK** on success. In the event of a failure, a
negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE** *resource* is not a valid handle.

**ZX_ERR_WRONG_TYPE** *resource* is not resource kind **ZX_RSRC_KIND_SYSTEM**.

**ZX_ERR_OUT_OF_RANGE** *resource* is not in the range [**ZX_RSRC_SYSTEM_CPU_BASE**, **ZX_RSRC_SYSTEM_CPU_BASE**+1).

**ZX_ERR_INVALID_ARGS** *topic* or *info* have invalid values or *info* has out-of-order entries. 

**ZX_ERR_OUT_OF_RANGE** *count* or *info* entry values do not meet the requirements of the topic.

**ZX_ERR_NO_MEMORY** Failure due to lack of memory. There is no good way for userspace to handle this (unlikely) error. In a future build this error will no longer occur.


## SEE ALSO

- [RFC 0123: CPU performance info syscalls](/contribute/governance/rfcs/0123_cpu_performance_info.md)
 

 - [`zx_system_get_performance_info()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_system_get_performance_info()`]: system_get_performance_info.md
