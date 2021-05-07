# zx_system_get_version_string

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Get version string for system.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_string_view_t zx_system_get_version_string(void);
```

## DESCRIPTION

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

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_system_get_version_string()` returns a `zx_string_view_t` object.

## ERRORS

`zx_system_get_version_string()` cannot fail.

## NOTES

## SEE ALSO


