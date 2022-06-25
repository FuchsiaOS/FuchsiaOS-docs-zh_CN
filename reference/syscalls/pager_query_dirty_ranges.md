# zx_pager_query_dirty_ranges

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Query contiguous ranges of dirty pages in a pager owned vmo.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_pager_query_dirty_ranges(zx_handle_t pager,
                                        zx_handle_t pager_vmo,
                                        uint64_t offset,
                                        uint64_t length,
                                        void* buffer,
                                        size_t buffer_size,
                                        size_t* actual,
                                        size_t* avail);
```

## DESCRIPTION

Queries *pager_vmo* for contiguous runs of pages in the range [*offset*, *offset* + *length*) that
are dirty, i.e. have outstanding modifications that have not been written back to the pager source.
The *pager_vmo* must have previously been created from the *pager* by [`zx_pager_create_vmo()`].
*offset* and *length* need not be page aligned, but they will be rounded to page boundaries when
performing the query.

*buffer* should point to an array of `zx_vmo_dirty_range_t` struct that will hold the result of the
query, and *buffer_size* should accommodate the array.

```c
typedef struct zx_vmo_dirty_range {
  // Represents the range [offset, offset + length).
  uint64_t offset;
  uint64_t length;
  // Any options applicable to the range.
  // ZX_VMO_DIRTY_RANGE_IS_ZERO indicates that the range contains all zeros.
  uint64_t options;
} zx_vmo_dirty_range_t;

```

*actual* is an optional pointer to return the number of dirty ranges that were written to *buffer*.

*avail* is an optional pointer to return the number of dirty ranges that are available to read. If
*buffer* is insufficiently large, *avail* will be larger than *actual*.

Upon success, *actual* will contain the number of dirty ranges that were copied out to *buffer*.
The number of dirty ranges that are copied out to *buffer* is constrained by *buffer_size*, i.e. it
is possible for there to exist more dirty ranges in [*offset*, *offset* + *length*) that could not
be accommodated in *buffer*. The caller can assume than any range that had been made dirty prior to
making the call will either be contained in *buffer*, or will have a start offset strictly greater
than the last range in *buffer*. Therefore, the caller can advance *offset* and make another query
to discover further dirty ranges, until *avail* is zero.

Sample user code that wants to query all dirty ranges in a VMO might look like this:

```c

  zx_vmo_dirty_range_t ranges[5];
  size_t actual = 0;
  size_t avail = 0;
  uint64_t start = 0;
  uint64_t len = vmo_size;

  while (len > 0) {
    zx_status_t st = zx_pager_query_dirty_ranges(pager, vmo, start, len,
                                                 &ranges[0],
                                                 5 * sizeof(zx_vmo_dirty_range_t),
                                                 &actual, &avail);
    // Process the |ranges| returned as needed.
    ProcessDirtyRanges(&ranges[0], actual);

    // We've read all the dirty ranges that existed before the query.
    if (actual == avail) {
      break;
    }
    // We used up the entire |ranges| buffer, but there are more dirty ranges to be read.
    // Advance start beyond the last dirty range found.
    uint64_t new_start = ranges[4].offset + ranges[4].length;
    len -= (new_start - start);
    start = new_start;
  }

```

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*pager* must be of type **ZX_OBJ_TYPE_PAGER**.

*pager_vmo* must be of type **ZX_OBJ_TYPE_VMO**.

## RETURN VALUE

`zx_pager_query_dirty_ranges()` returns **ZX_OK** on success. In the event of failure, a negative
error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE** *pager* or *pager_vmo* is not a valid handle.

**ZX_ERR_WRONG_TYPE** *pager* is not a pager handle, or *pager_vmo* is not a vmo handle.

**ZX_ERR_INVALID_ARGS**  *pager_vmo* is not a vmo created from *pager*.

**ZX_ERR_OUT_OF_RANGE** The specified range in *pager_vmo* is invalid.

## SEE ALSO

 - [`zx_pager_create_vmo()`]
 - [`zx_pager_detach_vmo()`]
 - [`zx_pager_op_range()`]
 - [`zx_pager_query_vmo_stats()`]
 - [`zx_pager_supply_pages()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_pager_create_vmo()`]: pager_create_vmo.md
[`zx_pager_detach_vmo()`]: pager_detach_vmo.md
[`zx_pager_op_range()`]: pager_op_range.md
[`zx_pager_query_vmo_stats()`]: pager_query_vmo_stats.md
[`zx_pager_supply_pages()`]: pager_supply_pages.md
