<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_pager_op_range

## Summary

Perform an operation on a range of a pager owned vmo.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_pager_op_range(zx_handle_t pager,
                              uint32_t op,
                              zx_handle_t pager_vmo,
                              uint64_t offset,
                              uint64_t length,
                              uint64_t data);
```

## Description

Performs a pager operation, specified by *op* on *pager_vmo* in the range [*offset*, *offset* +
*length*). The *pager_vmo* must have previously been created from the *pager* by
[`zx_pager_create_vmo()`]. *offset* and *length* must be page aligned. *data* is an optional
parameter, if the specified *op* supports one.

Operations that can be performed, i.e. values *op* can take:

**ZX_PAGER_OP_DIRTY** - The userspace pager wants to transition pages in the range [*offset*,
*offset* + *length*) from clean to dirty. This will unblock any writes that were waiting on
**ZX_PAGER_VMO_DIRTY** page requests for the specified range.

**ZX_PAGER_OP_FAIL** - The userspace pager failed to fulfill page requests for *pager_vmo* in the
range [*offset*, *offset* + *length*) with command **ZX_PAGER_VMO_READ** or **ZX_PAGER_VMO_DIRTY**.
*data* contains the error encountered (a `zx_status_t` error code sign-extended to a `uint64_t`
value) - permitted values are **ZX_ERR_IO**, **ZX_ERR_IO_DATA_INTEGRITY**, **ZX_ERR_BAD_STATE**,
**ZX_ERR_NO_SPACE**, and **ZX_ERR_BUFFER_TOO_SMALL**.

This will signal threads that might be waiting on page requests in that range, unblocking them. If
the blocked thread was requesting pages through a [`zx_vmo_read()`] / [`zx_vmo_write()`] or a
[`zx_vmo_op_range()`] with **ZX_VMO_OP_COMMIT**, the call will fail and the error status (*data*)
will be returned. If the blocked thread was requesting pages through a VMAR mapping, the thread will
take a fatal page fault exception.

**ZX_PAGER_OP_WRITEBACK_BEGIN** - The userspace pager wants to begin writing back pages in the range
[*offset*, *offset* + *length*). This indicates an intent to clean any dirty pages in the specified
range once the writeback is completed (signaled with **ZX_PAGER_OP_WRITEBACK_END**). Refer to the
sample code below for suggested usage.

*data* can optionally be set to **ZX_VMO_DIRTY_RANGE_IS_ZERO** to indicate that the caller wants to
write back the specified range as zeroes. This is intended to be used when the caller is processing
a range that was returned by [`zx_pager_query_dirty_ranges()`] with its `options` set to
**ZX_VMO_DIRTY_RANGE_IS_ZERO**. It ensures that any non-zero content that was created in the range
after the query but before the writeback was started is not lost, by incorrectly assuming it is
still zero and marking it clean (hence evictable).

**ZX_PAGER_OP_WRITEBACK_END** - The userspace pager is done writing back pages in the range
[*offset*, *offset* + *length*). This indicates that any dirty pages in the specified range that
were previously signaled with **ZX_PAGER_OP_WRITEBACK_BEGIN** can be marked clean. Refer to the
sample code below for suggested usage.

Sample code (modulo error handling) to discover and clean any dirty pages might look something like
this.

```c
  zx_vmo_dirty_range_t ranges[kMaxRanges];
  uint64_t num_ranges;

  zx_status_t st =
      zx_pager_query_dirty_ranges(pager, vmo, 0, vmo_size, &ranges[0],
                                  kMaxRanges * sizeof(zx_vmo_dirty_range_t), &num_ranges, nullptr);

  for (uint64_t i = 0; i < num_ranges; i++) {
    uint64_t start = ranges[i].offset;
    uint64_t len = ranges[i].length;
    st = zx_pager_op_range(pager, ZX_PAGER_OP_WRITEBACK_BEGIN, vmo, start, len, 0);
    WritebackToDisk(vmo, start, len);
    st = zx_pager_op_range(pager, ZX_PAGER_OP_WRITEBACK_END, vmo, start, len, 0);
  }
```

## Rights

*pager* must be of type **ZX_OBJ_TYPE_PAGER**.

*pager_vmo* must be of type **ZX_OBJ_TYPE_VMO**.

## Return value

`zx_pager_op_range()` returns ZX_OK on success, or one of the following error codes on failure.

## Errors

**ZX_ERR_BAD_HANDLE** *pager* or *pager_vmo* is not a valid handle.

**ZX_ERR_WRONG_TYPE** *pager* is not a pager handle, or *pager_vmo* is not a vmo handle.

**ZX_ERR_INVALID_ARGS** under any of these conditions:
- *pager_vmo* is not a vmo created from *pager*.
- *offset* or *length* is not page aligned.
- *op* is **ZX_PAGER_OP_FAIL** and *data* is not one of **ZX_ERR_IO**, **ZX_ERR_IO_DATA_INTEGRITY**
  or **ZX_ERR_BAD_STATE**.
- *op* is **ZX_PAGER_OP_WRITEBACK_BEGIN** and *data* is not 0 or **ZX_VMO_DIRTY_RANGE_IS_ZERO**.

**ZX_ERR_OUT_OF_RANGE** The specified range in *pager_vmo* is invalid.

**ZX_ERR_NOT_SUPPORTED**  *op* is not supported on the specified range in *pager_vmo*.

**ZX_ERR_NOT_FOUND** *op* is **ZX_PAGER_OP_DIRTY** and the range denoted by *offset* and
*length* contains unsupplied regions.

## See also

 - [`zx_pager_create_vmo()`]
 - [`zx_pager_detach_vmo()`]
 - [`zx_pager_query_dirty_ranges()`]
 - [`zx_pager_supply_pages()`]

[`zx_pager_create_vmo()`]: pager_create_vmo.md
[`zx_pager_detach_vmo()`]: pager_detach_vmo.md
[`zx_pager_query_dirty_ranges()`]: pager_query_dirty_ranges.md
[`zx_pager_supply_pages()`]: pager_supply_pages.md
[`zx_vmo_op_range()`]: vmo_op_range.md
[`zx_vmo_read()`]: vmo_read.md
[`zx_vmo_write()`]: vmo_write.md
