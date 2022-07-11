# zx_system_get_performance_info

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Get CPU performance parameters.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_system_get_performance_info(zx_handle_t resource,
                                           uint32_t topic,
                                           size_t count,
                                           void* info,
                                           size_t* output_count);
```

## DESCRIPTION

`zx_system_get_performance_info()` requests CPU performance parameters maintained by the kernel. The
*topic* parameter indicates what specific information is desired.

*count* indicates the number of items to query. Topics may impose requriements on this value.

*info* is a pointer to a buffer of sufficient size to accommodate *count* entries of the type
specified by *topic*. The values stored in this array are undefined if the syscall returns an error.

*output_count* is updated with the number of info entries populated by the kernel on success. The
value stored in this memory location is undefined if the syscall returns an error.

[TOC]

## TOPICS

### ZX_CPU_PERF_SCALE

*count*: Must be equal to the number of logical CPUs in the system.

*info* type: `zx_cpu_performance_scale_t[count]`

Returns an array of `zx_cpu_performance_scale_t` with entries indicating the current performance
scales (scalar values representing relative operating points) of each logical CPU in the system. The
values reflect the most recent call to `zx_system_set_performance_info`, even if the values have not
yet taken effect.

See [RFC 0123](/contribute/governance/rfcs/0123_cpu_performance_info.md)
for further details on values and update protocols.

### ZX_CPU_DEFAULT_PERF_SCALE

*count*: Must be equal to the number of logical CPUs in the system.

*info* type: `zx_cpu_performance_scale_t[count]`

Returns an array of `zx_cpu_performance_scale_t` with entries indicating the default performance
scales used during boot of each logical CPU in the system.

See [RFC 0123](/contribute/governance/rfcs/0123_cpu_performance_info.md)
for further details.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*resource* must have resource kind **ZX_RSRC_KIND_SYSTEM**.

## RETURN VALUE

`zx_system_get_performance_info()` returns **ZX_OK** on success. In the event of a failure, a
negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE** *resource* is not a valid handle.

**ZX_ERR_WRONG_TYPE** *resource* is not resource kind **ZX_RSRC_KIND_SYSTEM**.

**ZX_ERR_ACCESS_DENIED** *resource* is not in the range [**ZX_RSRC_SYSTEM_CPU_BASE**, **ZX_RSRC_SYSTEM_CPU_BASE**+1).

**ZX_ERR_INVALID_ARGS** *topic*, *info*, or *output_count* have invalid values. 

**ZX_ERR_OUT_OF_RANGE** *count* does not meet the requirements of the topic.

**ZX_ERR_NO_MEMORY** Failure due to lack of memory. There is no good way for userspace to handle this (unlikely) error. In a future build this error will no longer occur.

## SEE ALSO

- [RFC 0123: CPU performance info syscalls](/contribute/governance/rfcs/0123_cpu_performance_info.md)

 - [`zx_system_set_performance_info()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_system_set_performance_info()`]: system_set_performance_info.md
