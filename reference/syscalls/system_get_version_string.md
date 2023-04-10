<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_system_get_version_string

## Summary

Get version string for system.

## Declaration

```c
#include <zircon/syscalls.h>

zx_string_view_t zx_system_get_version_string(void);
```

## Description

`zx_system_get_version_string()` returns a string identifying the version of
the Zircon system currently running.

The returned object is a simple pair of C string pointer and length.  The
string is guaranteed to be NUL-terminated and to be valid UTF-8.  The length
does not include the NUL terminator.  In C++ the return value can be coerced
implicitly to `std::string_view`, `std::u8string_view`, or other types with
equivalent two-argument constructor signatures.

The string constant may be of any length.  It is in read-only memory provided
by the [vDSO](../../concepts/kernel/vdso.md).  Thus it is always accessible at
the same address for the life of the process and its contents never change.

The first four characters identify the version scheme. An example of the string
returned is "git-8a07d52603404521038d8866b297f99de36f9162".

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_system_get_version_string()` returns a `zx_string_view_t` object.

## Errors

`zx_system_get_version_string()` cannot fail.

## Notes

## See also
