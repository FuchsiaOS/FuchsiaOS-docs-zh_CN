Naming C/C++ objects
====================

## Include paths

The following guidelines apply to libraries which are meant to be used
extensively, e.g. in an upper layer of the Fuchsia codebase or via an SDK,
where "upper layer of the Fuchsia codebase" means "garnet" and above
(peridot, topaz, vendor/foo).

There are three categories of headers: system, fuchsia, other.

#### For system headers

```
<zircon/foo/bar.h>
```

###### Rationale

These headers describe kernel interfaces (syscalls, related structs and
defines), shared definitions and data structures between kernel and userspace
(and bootloader), that are often useful to higher layers as well.

###### Notes

- Headers may be installed straight under `zircon/`.
- This does not include things like wrappers on syscall interfaces like zx.

###### Examples

- `zircon/process.h`
- `zircon/syscalls/hypervisor.h`


#### For global headers

```
<fuchsia/foo/bar.h>
```

###### Rationale

These are libraries that define a low level ABI/API in Fuchsia but are not
specific to the kernel.

###### Notes

- FIDL-generated code for Fuchsia APIs in that very namespace,
  as well as C/C++ wrapper libraries around these APIs are installed here.
- Headers may be installed straight under `fuchsia/`.

###### Examples

- `fuchsia/fdio/fdio.h`
- `fuchsia/pixelformat.h`


#### For other headers

```
<lib/foo/bar.h>
```

###### Rationale

Some libraries in that space are not necessarily Fuchsia-specific, or they
may be Fuchsia-specific but do not fall into either of the above categories.
We use a rather bland namespace that will likely not cause any collisions in
the outside world: "lib".

###### Notes

- Headers may not be placed straight under `lib/`. Subdirectories (`lib/foo/`)
  are mandatory.

###### Examples

- `lib/app/cpp/startup_context.h`
- `lib/fbl/array.h`
- `lib/zx/event.h`
