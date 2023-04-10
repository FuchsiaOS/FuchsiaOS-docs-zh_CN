<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_cprng_draw

## Summary

Draw from the kernel's CPRNG.

## Declaration

```c
#include <zircon/syscalls.h>

void zx_cprng_draw(void* buffer, size_t buffer_size);
```

## Description

`zx_cprng_draw()` draws random bytes from the kernel CPRNG.  This data should
be suitable for cryptographic applications.

Clients that require a large volume of randomness should consider using these
bytes to seed a user-space random number generator for better performance.
"Large" here would mean a large multiple of **ZX_CPRNG_DRAW_MAX_LEN**, which is
currently 256. As always, test the actual latency of your call site before
optimizing.

## Rights

None.

## Notes

`zx_cprng_draw()` terminates the calling process if **buffer** is not a valid
userspace pointer.

There are no other error conditions.  If its arguments are valid,
`zx_cprng_draw()` will succeed.

## See also

 - [`zx_cprng_add_entropy()`]

[`zx_cprng_add_entropy()`]: cprng_add_entropy.md
