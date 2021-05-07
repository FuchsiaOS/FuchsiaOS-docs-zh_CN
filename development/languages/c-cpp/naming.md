Naming C/C++ objects
====================

## Include paths

There are four categories of headers: system, global, library, and
implementation.

#### System headers

```
#include <zircon/foo/bar.h>
```

###### Definition

These headers define the interface between the kernel and userspace, also known
as the vDSO interface. These headers define system calls, included related types
and structures.  These headers also define some basic C and C++ machinery, for
example for crashing in a well-defined sequence.

###### Notes

- System headers may be installed under `zircon/`, rather than `lib/zircon/`.
- System call wrappers, such as `zx`, are not considered system headers. They
  are library headers (see below) that depend on the system headers..
- Standard system headers (e.g., from the C and C++ standard librareis) have
  their   standard paths

###### Examples

- `#include <zircon/process.h>`
- `#include <zircon/syscalls/hypervisor.h>`
- `#include <stdint.h>`
- `#include <algorithm>`

#### Global headers

```
#include <fuchsia/foo/bar.h>
```

###### Definition

These headers define system-wide contracts between userspace components. These
headers are generated from FIDL definitions of these contracts.

###### Notes

- Hand-written code should be presented in library headers rather than global
  headers.

###### Examples

- `#include <fuchsia/sys/cpp/fidl.h>`
- `#include <fuchsia/sysmem/llcpp/fidl.h>`

#### Library headers

```
#include <lib/foo/bar.h>
```

###### Definition

Library headers are hand-written code that are used by applications. The
interfaces they define are local to that application. Some libraries are
Fuchsia-specific and provide an ergonomic wrapper around some lower-level
system facilities. Some libraries might not be tied directly to Fuchsia.

###### Notes

- All library headers are in the `lib/` directory to help avoid collisions with
  other header used by applications.
- Headers may not be placed straight under `lib/`. Subdirectories (`lib/foo/`)
  are mandatory.

###### Examples

- `#include <lib/fit/function.h>`
- `#include <lib/sys/cpp/component_context.h>`
- `#include <lib/zx/event.h>`

#### Implementation headers

```
#include "src/foo/bar.h"
```

###### Definition

Implementation headers are internal to the Fuchsia Platform Source Tree. They
are never included in SDKs and are referenced by absolute path from the root of
the source tree.

###### Notes

- Includes of implementation headers use `"` rather than `<` to indicate that
  the path is relative to the root of the source tree.

###### Examples

- `#include "src/ui/scenic/bin/app.h"`
- `#include "src/lib/fxl/observer_list.h"`
