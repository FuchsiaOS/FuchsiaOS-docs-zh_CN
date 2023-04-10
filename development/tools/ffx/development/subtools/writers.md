# Writers

The `ffx` subtool interface has the concept of a `Writer` that manages IO between
the tool and the user. This interface distinguishes between human and
machine users so that we can use our tools in other tooling more effectively.

We currently implement two kinds of writers:

* `SimpleWriter`: Errors if the program is run with `--machine json` and can
only output strings.
* `MachineWriter`: adapts it output to match whether the `--machine json`
argument is passed, outputting only JSON on stdout if it is and outputting
user-readable strings if not.

## How to specify your `Writer` type

In the legacy plugin interface you could add a `writer` argument to your plugin
definition using an `#[ffx(writer = OutputType)]` attribute on the writer
argument, and that would cause some generation of schema information from
the type you specified.

In this subtool interface, you specify this as an associated type on
the `FfxMain` trait for your tool, and the machine type is a generic
argument to the `MachineWriter` type. If your tool doesn't implement machine
output, it should use `SimpleWriter` instead of something like
`MachineWriter<String>` to avoid having people depend on your unstructured
output.

### Using the `Writer`

For the most part, you can use the writer as you would any implementation of
`std::io::Write`, and it's fine to just use `writeln!()` or similar built-in
macros against it for any unstructured string output.

All writers also implement some convenience functions for basic writing needs
like `print` and `line`. These functions will all only produce output if the
command wasn't run in machine mode. Otherwise they will be ignored.

All writers also implement `item`, which will either print the object
given in machine mode, or use its `Display` implementation to output it in
non-machine mode as text.

If you're using a machine writer, you also get a few more methods to help
with outputting structured output:

* `machine`: Only print the given object in machine mode.
* `machine_many`: Print the given objects in machine mode.
* `machine_or`: If in machine mode, print the object. Otherwise print the textual
information in the other argument (implementing `Display`).
* `machine_or_else`: If in machine mode, print the object. Otherwise print the
textual information resulting from the function in the other argument.
