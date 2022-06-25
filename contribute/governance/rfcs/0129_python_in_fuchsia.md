<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0129" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC defines the Python versioning and script requirements for Python
sources within the Fuchsia project.

## Motivation

On the host, not all Python scripts in the tree are executed in a deterministic
manner. Some scripts assume a vendored prebuilt interpreter, others are left to
the system Python. Further, there is no consistency with respect to the version
of Python supported on a per-script basis.

The lack of a deterministic Python environment causes scripts to occasionally
break when environmental changes are made (for example, removing the
/usr/bin/python symlink). This requires the user to either restore the local
environment to the expected state, or make local modifications to the script(s)
in question to be compatible with their environment.

Lastly, as of January 1st, 2020, Python 2.x has been officially
[sunset](https://www.python.org/doc/sunset-python-2/) by
[python.org](http://python.org).

## Design

This design replaces the user's local host environment with a vendored prebuilt
Python interpreter for all Fuchsia Project Python sources. This vendored Python
SHALL be the supported Python language revision.

The current vendored Python version SHALL be kept within a current
[python.org](http://python.org) support window. As the version of Python is
updated, a reasonable transition period will be granted to allow sources to be
made compatible with the new language version.

For Fuchsia Project Python sources, this RFC terminates support for Python
language versions prior to 3.8

* Python 2.7 is henceforth deprecated.
* Backwards compatibility prior to 3.8 is not required.

### Python Sources as Scripts
A Python source file may be invoked as a script by (for example) invoking it on
the command line or supplying it as an argument to an invocation of the Python
interpreter. Scripts are denoted by those runtime environments whose module
[`__name__`](https://docs.python.org/3/library/__main__.html) is set
equal to `"__main__"`.

Executable Python sources are REQUIRED to defer to the vendored Python for
execution. This may be accomplished using different means:

* Through documentation, instruct users to directly invoke the vendored
  interpreter supplying the candidate script as an interpreter argument.
* Defer to the vendored Python using a wrapping script or other mechanism which
  contains logic sufficient to invoke the interpreter (e.g. using something like
  `//scripts/hermetic-env`).
* Assuming a script interpreter source directive (i.e. shebang) which ultimately
  causes the script to be executed using the vendored interpreter.

### Python Shebang
Python scripts intended to be directly invoked MUST contain a shebang which
ultimately references the vendored Python. The appropriate shebang line to use
is:

```bash
#!/usr/bin/env fuchsia-vendored-python
```

As the use of a shebang implies a dependency on the host environment, this
shebang is intentionally selected to meet the following hermetic-driven goals:

* Be less-likely to collide with an existing tool, alias, or macro in the user's
  host environment.
* Be agnostic to the ordering by which the user added `//.jiri_root/bin` to
  their `$PATH`.

### Hermetic Python Environments

Tools such as [venv](https://docs.python.org/3/library/venv.html) and
[vpython](https://chromium.googlesource.com/infra/infra/+/refs/heads/main/doc/users/vpython.md)
serve to assemble an ephemeral and hermetic Python environment. Packages
installed are local to these hermetic environments and do not affect the local
system installation base. The use of hermetic Python tools is permitted with the
stipulation that they are derived from the vendored Python.

#### Python VirtualEnv (venv)

A venv may be instantiated by invoking the vendored Python interpreter with the
args of `-m venv my_venv`. This will assemble a venv in the local directory
`my_venv`. The venv may then be activated and interacted with per typical venv
usage. Installing venv-local packages is permitted.

#### Chromium vpython

The vpython utility is another form of hermetic Python environment manager. It
sources its interpreter from the `$PATH` and assumes the first interpreter whose
`--version` is compatible with the input `vpython.Spec` protobuf. To use
vpython, ensure the vendored Python is configured in the `$PATH` at the time
vpython is invoked. Further, the `vpython.Spec` protobuf must reflect the
current supported language version. Use of vpython wheels is permitted to the
extent they comply with the current supported language version.

## Implementation

All Python sources in Fuchsia Project repositories will be updated to language
version 3.8 by end of Q3 2021. Those sources that are meant to be directly
invoked will have their shebang point to the vendored Python according to the
semantics below.

To facilitate a vendored Python interpreter as the shebang target, the following
will be modified:

1. To `//.jiri_root/bin`, a new `fuchsia-vendored-python` symlink will be added
   which links to `//scripts/fuchsia-vendored-python`.
1. To `//scripts`, a new `fuchsia-vendored-python` symlink will be added which
   links to `//scripts/fuchsia-vendored-python3.8`.
1. The `fuchsia-vendored-python3.8` script will be added to `//scripts` and be
   implemented as:

```bash
#!/bin/bash
hermetic-env python3.8 "$@"
```

Assuming `//.jiri_root/bin` is among the host environment's `PATH` (an assumed
environmental configuration), this will dispatch to the prebuilt interpreter
regardless of what interpreter version(s) are installed on the host.

If `//.jiri_root/bin` is not part of the host environment's path, it will be
necessary to symlink `//.jiri_root/bin/fuchsia-vendored-python` to an
appropriate location (e.g. `~/bin` or similar).

It is expected that new Python versions will occasionally need to be rolled in
via `//integration`. The procedure for doing this is straightforward:

1. Via `//integration`, download and install the prebuilt binaries to their
   appropriate locations.
1. Add a new bootstrapping script (e.g. `fuchsia-vendored-python3.9`) to
   `//scripts`.
1. Move the `//scripts/fuchsia-vendored-python` symlink to the new
   bootstrapping script.
1. (optionally) delete the old bootstrapping script(s).

## Performance

N/A

## Ergonomics

Ergonomically, a deterministic Python language version, and vendored prebuilt
interpreter will much more comfortable of an experience. Users will not be
bogged down by the class of problems caused by relying on their host
environment.

## Backwards Compatibility

Backwards compatibility for Python language version 2.x is no longer required,
and this RFC terminates support for Python language version 2.x. The intent of
this RFC is to eliminate the maintenance burden from Python 2 backwards
compatibility.

## Security considerations

[python.org](http://python.org) has sunset Python 2.x as end-of-life, and ceased
all maintenance, including security fixes. The Fuchsia project no longer
supports Python language version 2.x.

Security is improved by migrating to a maintained Python distribution (version
3.8+) and the more strict typing between byte and character array
types present in the newer Python language versions.

Additionally, having assumed an exact prebuilt version rather than arbitrary
host system installation will reduce maintenance burden and developer
frustration due to version skew and local installation variations.

## Privacy considerations

N/A

## Testing

N/A

## Documentation

The following documentation will need to be updated to reflect the policy here:

* `//docs/development/build/build_system/policies.md`
* `//docs/development/languages/python/python_style.md`
* `//docs/get-started/get_fuchsia_source.md`

## Drawbacks, alternatives, and unknowns

Sun-setting a programming language revision in a backwards-incompatible manner
can be problematic during migration. Not all users will anticipate this change,
and some may not have themselves moved on from Python 2.x.

However, the industry has been aware of the need to migrate from 2.7 to Python
3.x for multiple years now, and this change shouldn't be unexpected.

## Prior art and references

* [Sunsetting Python 2](https://www.python.org/doc/sunset-python-2/)
* [Python VirtualEnv](https://docs.python.org/3/library/venv.html)
* [Chromium vpython](https://chromium.googlesource.com/infra/infra/+/refs/heads/main/doc/users/vpython.md)
