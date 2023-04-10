# Fuchsia's libc

Fuchsia's libc started as a fork of musl libc. It has since diverged
significantly, but the approximate source code layout remains the same. The
source can be found in
[`zircon/third_party/ulib/musl`](/zircon/third_party/ulib/musl).

TODO(fxbug.dev/31481) Type more here.

## Standards

### C11

Fuchsia's libc supports most of the [C11][c11std] standard. This
in particular includes the atomic and threading portions of the
standard library.

### POSIX

Fuchsia implements a subset of POSIX.

Things at least partially supported include the basics of POSIX I/O
(open/close/read/write/stat/...), and pthreads (threads and mutexes).

On Fuchsia, the portion of file paths beginning with a sequence of
`..` is resolved locally. See [this writeup][dotdot] for more
information.

Similarly, symlinks are not supported on Fuchsia.

Conspicuously not supported are UNIX signals, fork, and exec.

## FDIO

Fuchsia's libc does not directly support I/O operations. Instead it
provides weak symbols that another library can override. This is
typically done by [fdio.so][fdio].

## Linking

Statically linking libc is not supported. Everything dynamically links libc.so.

## Dynamic linking and loading

libc.so is also the dynamic linker.

[c11std]: https://en.wikipedia.org/wiki/C11_(C_standard_revision)
[dotdot]: /docs/concepts/filesystems/dotdot.md
[fdio]: /sdk/lib/fdio
