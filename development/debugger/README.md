# zxdb: The Fuchsia debugger

Zxdb is a console-mode debugger for native code running on Fuchsia.

It primarily supports C, C++ and Rust code, but any language that compiles natively and exports
DWARF symbols should work to some extent (such as Go). Interpreted code such as Dart and JavaScript
are not supported.

User guide:

  * [Set up and run the debugger](running.md)
  * [Commands and interaction model](commands.md)
  * [Debugging a process, component, or crash dump](attaching.md)
  * [Controlling thread execution](execution.md) (pausing, stepping, and resuming)
  * [Using breakpoints](breakpoints.md)
  * [Evaluating and printing expressions](printing.md)
  * [Inspecting memory](memory.md)
  * [Working with assembly language](assembly.md)
  * [Looking at handles](kernel_objects.md)


Advanced usage:

  * [Diagnosing symbol problems](symbols.md)
  * [Working with exceptions](exceptions.md)

Other topics:

  * [Developing and debugging the debugger](developing.md)

  * [Report a zxdb bug](https://bugs.fuchsia.dev/p/fuchsia/issues/entry?components=Tools%3Ezxdb)
