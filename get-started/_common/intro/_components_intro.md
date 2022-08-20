**Components** are the foundational building blocks of software running in
Fuchsia. Each component is a composable, sandboxed module that interacts with
other components through capabilities. This promotes system security and
creates clear interfaces between individual components, making them easier to
update or replace.

In Fuchsia, **everything is a component** (almost). Recall from the previous
discussion of Zircon that the surface area of the kernel is intentionally small,
with most core services being implemented in user space. This means that most
software running on Fuchsia is implemented using the component framework,
including:

*   User-facing applications
*   Device drivers
*   Filesystems
*   Media codecs
*   Network stacks

Outside the kernel there are only a few low-level exceptions not using the
component framework, such as bootloaders and the `userboot` process.
