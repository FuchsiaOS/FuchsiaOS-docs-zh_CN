# Contributing to Zircon

Important: Zircon is under active development. At this time Zircon is
not seeking major changes or new features from new contributors.
However, small bug fixes are encouraged.

Here are some general guidelines for patches to Zircon.

## Process

*   Follow the process for Fuchsia patches outlined in
    [Contribute changes][contribute-changes].

*   Patches are handled through
    [Gerrit Code Review][gerrir-code-review]{:.external}.

*   Make sure Zircon is buildable for all major targets (`x86-64`, `arm64`)
    at every change. Use `fx multi bringup-cq` so that Zircon is buildable.
    For more information,  see
    [Building Zircon for all targets][build-zircon-for-all].

*   Avoid breaking the unit tests. Boot Zircon and
    [run the tests][run-the-tests] to verify that they're all passing.

*   Avoid whitespace or style changes. Especially, do not mix style changes
    with patches that contain other changes, as style changes are often
    a distraction. Use `fx format-code` to format the code with the
    consistent style.

*   Avoid changes that touch multiple modules at once if possible. Most changes
    should be to a single library, driver, app, and so on.

## Documentation for Zircon

Writing documentation is a great idea and is encouraged:

*   Documentation should be in Markdown files.
*   Zircon documentation is located in
    [`/docs/concepts/kernel`][googlesource-docs].
*   Before submitting documentation, make sure that the Markdown renders
    correctly.

When editing or adding `syscalls` or `cmdlines`, update the following:

*   A list of `syscalls` in [`/docs/reference/syscalls/README.md`][syscall-doc]
*   A list of kernel `cmdline` options in
    [`/docs/reference/kernel/kernel_cmdline.md`][cmdline-doc].

## Notes

### How to deprecate #define constants

You can create a deprecated `typedef` and have the constant definition
cast to that type.  The warning or error that is generated includes the name
of the deprecated `typedef`.

```none {:.devsite-disable-click-to-copy}
typedef int ZX_RESUME_NOT_HANDLED_DEPRECATION __attribute__((deprecated));
#define ZX_RESUME_NOT_HANDLED ((ZX_RESUME_NOT_HANDLED_DEPRECATION)(2))
```

<!-- Reference links -->

[contribute-changes]: /docs/development/source_code/contribute_changes.md
[gerrir-code-review]: https://fuchsia-review.googlesource.com/#/q/project:zircon
[build-zircon-for-all]: /docs/development/kernel/getting_started.md#building_zircon_for_all_targets
[run-the-tests]: /docs/development/testing/testing.md
[googlesource-docs]: /docs/concepts/kernel
[syscall-doc]: /docs/reference/syscalls/README.md
[cmdline-doc]: /docs/reference/kernel/kernel_cmdline.md
