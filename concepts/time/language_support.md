# Time language support

This page details how you can handle time when developing software on Fuchsia.

In general, you may use any time library available for your language. Most
libraries are able to link against [Fuchsia's libc implementation][libc] to
obtain time. It is recommended that you use a platform-agnostic library unless
you need Fuchsia specific time operations.

Various time options are available, such as:

Language | Monotonic | UTC
-------- | --------- | ---
C | [`clock_gettime`][c-clock-gettime]{:.external} | [`time`][c-time]{:.external}
C++ | [`std::chrono::steady_clock`][cpp-steady-clock]{:.external} | [`std::chrono::system_clock`][cpp-system-clock]{:.external}
Rust | [`std::time::Instant`][rust-instant]{:.external} | [`std::time::SystemTime`][rust-system-time]{:.external}

## Fuchsia specific time operations

In some cases, you will need to handle time in a Fuchsia specific manner. This
is necessary when you need to handle Fuchsia's representation of time directly,
or when you need to handle
[Fuchsia specific UTC behavior][fuchsia-utc-behavior].

### Monotonic time

Monotonic time on Fuchsia is represented as a signed 64 bit integer, which
contains the number of nanoseconds since the system was powered on. See
[`zx_clock_get_monotonic`][zx-monotonic] for more details.

* {C }

  Monotonic time is accessible through [libzircon][c-libzircon].

  ```c
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/time/c/main.c" region_tag="common_imports" adjust_indentation="auto" %}

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/time/c/main.c" region_tag="monotonic" adjust_indentation="auto" %}
  ```

* {C++}

  Monotonic time is accessible through [libzx][cpp-libzx].

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/time/cpp/main.cc" region_tag="common_imports" adjust_indentation="auto" %}

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/time/cpp/main.cc" region_tag="monotonic" adjust_indentation="auto" %}
  ```

* {Rust}

  Monotonic time is accessible through the [fuchsia_zircon][rust-zircon] crate.
  This crate is only available in-tree.

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/time/rust/src/main.rs" region_tag="monotonic" adjust_indentation="auto" %}
  ```

### UTC time

UTC time on Fuchsia is represented as a signed 64 bit integer that contains
the number of nanoseconds since the Unix epoch (January 1st, 1970).

Operations on the UTC clock require obtaining a handle to the UTC clock
provided to the runtime.

Handling the UTC clock directly enables a few Fuchsia specific operations,
including:

* Inspecting the UTC clock's properties.
* Waiting for the UTC clock to begin running, which indicates it has been
synchronized, before reading it.

* {C }

  You can obtain a handle to the UTC clock using `zx_utc_reference_get` and use
  the syscalls exposed in [libzircon][c-libzircon].

  ```c
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/time/c/main.c" region_tag="common_imports" adjust_indentation="auto" %}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/time/c/main.c" region_tag="utc_imports" adjust_indentation="auto" %}

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/time/c/main.c" region_tag="utc" adjust_indentation="auto" %}
  ```

* {C++}

  You can obtain a handle to the UTC clock using `zx_utc_reference_get` and use
  the syscall wrappers in [libzx][cpp-libzx].

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/time/cpp/main.cc" region_tag="common_imports" adjust_indentation="auto" %}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/time/cpp/main.cc" region_tag="utc_imports" adjust_indentation="auto" %}

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/time/cpp/main.cc" region_tag="utc" adjust_indentation="auto" %}
  ```

* {Rust}

  You can obtain a handle to the UTC clock using the
  [fuchsia_runtime][rust-runtime] crate and use the syscall wrappers in the
  [fuchsia_zircon][rust-zircon] crate. The [fuchsia_async][rust-async] crate
  contains utilities to aid waiting for the clock to start. Note that these
  crates are only available in-tree.

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/time/rust/src/main.rs" region_tag="utc" adjust_indentation="auto" %}
  ```

[libc]: /docs/development/languages/c-cpp/libc.md
[c-clock-gettime]: https://linux.die.net/man/3/clock_gettime
[c-time]: https://linux.die.net/man/2/time
[cpp-steady-clock]: https://en.cppreference.com/w/cpp/chrono/steady_clock
[cpp-system-clock]: https://en.cppreference.com/w/cpp/chrono/system_clock
[rust-instant]: https://doc.rust-lang.org/std/time/struct.Instant.html
[rust-system-time]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
[fuchsia-utc-behavior]: utc/behavior.md#differences_from_other_operating_systems
[zx-monotonic]: /docs/reference/syscalls/clock_get_monotonic.md
[c-libzircon]: /docs/concepts/process/core_libraries.md#libzircon
[cpp-libzx]: /docs/concepts/process/core_libraries.md#libzx
[rust-runtime]: /src/lib/fuchsia-runtime
[rust-zircon]: /src/lib/zircon/rust
[rust-async]: /src/lib/fuchsia-async
