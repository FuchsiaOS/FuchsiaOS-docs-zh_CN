# Contributing to Zircon

Zircon is under active development and at this time Zircon is
not seeking major changes or new features from new contributors.
However, small bugfixes are encouraged.

Here are some general guidelines for patches to Zircon.

## Process

*   Follow the process for Fuchsia patches outlined in [Contribute changes](/docs/development/source_code/contribute_changes.md).

*   Patches are handled through [Gerrit Code Review](https://fuchsia-review.googlesource.com/#/q/project:zircon).

*   Additionally, make sure Zircon is buildable for all major targets (x86-64, arm64) at every
    change. Use `fx multi bringup-cq` so that Zircon is buildable.
    See [Building Zircon for all targets](/docs/development/kernel/getting_started.md#building_zircon_for_all_targets)
    for more information.

*   Avoid breaking the unit tests. Boot Zircon and [run the tests](/docs/development/testing/testing.md) to verify that
    they're all passing.

*   Avoid whitespace or style changes. Especially do not mix style changes with
    patches that do other things as the style changes are a distraction. Use `fx format-code`
    to format the code with the consistent style.

*   Avoid changes that touch multiple modules at once if possible. Most changes
    should be to a single library, driver, app, etc.

## Documentation for Zircon

Writing documentation is a great idea and is encouraged:

*   Documentation should be in Markdown files.
*   Zircon documentation is located in [/docs/concepts/kernel][googlesource-docs].
*   Before submitting documentation, make sure that the markdown renders correctly.

When editing or adding `syscalls` or `cmdlines`, update these documents:

*   A list of `syscalls` in [/docs/reference/syscalls/README.md][syscall-doc]
*   A list of kernel `cmdline` options in [/docs/reference/kernel/kernel_cmdline.md][cmdline-doc].

## Notes

## How to deprecate #define constants

You can create a deprecated `typedef` and have the constant definition
cast to that type.  The warning or error that is generated includes the name
of the deprecated `typedef`.

```
typedef int ZX_RESUME_NOT_HANDLED_DEPRECATION __attribute__((deprecated));
#define ZX_RESUME_NOT_HANDLED ((ZX_RESUME_NOT_HANDLED_DEPRECATION)(2))
```

[googlesource-docs]: /docs/concepts/kernel
[syscall-doc]: /docs/reference/syscalls/README.md
[cmdline-doc]: /docs/reference/kernel/kernel_cmdline.md
