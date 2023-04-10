# Hermetic build actions

Fuchsia's build system uses a tool to trace filesystem actions performed by
build actions in order to detect that build actions correctly and fully state
their inputs and outputs.

Continue reading this guide if you ran into an error that looks like this:

```
Unexpected file accesses building //some/target:label ...
(FileAccessType.READ /path/to/file/not/declared/as/input)
```

Or alternatively, if you're looking at an `action()` or `action_foreach()`
target that looks like this:

```gn
action("foo") {
  ...
  hermetic_deps = false
}
```

## Build graph correctness

The build is defined as a directed acyclic graph such that actions have their
inputs flowing into them and their outputs flowing from them. For instance, an
action that compiles a `.cc` file into a `.o` file will have the source file as
an input and an object file as an output. Any `.h` headers used in compilation
are considered as inputs to the same action.

This graph representation ensures that the build system can correctly perform
_incremental builds_. An incremental build is when a build was already
performed, but then some of the actions' inputs were changed, and now the build
system is being asked to rebuild. In an incremental build, the build system will
attempt to do the least amount of work needed, only rebuilding actions whose
inputs have changed, whether due to modifications done by the user to sources or
due to changes in the outputs of other actions that needed to be re-run.

For any action in the build graph, it's required that all inputs and outputs be
stated in order for the build graph to be correct and for actions to be
hermetic. However, this is not validated by the underlying build system, Ninja.
Build actions run in the user's local environment, with full access to the
entire filesystem, including all files in the source tree and in the `out/`
directory, so they're not sandboxed and they can reach anywhere.

Failing to declare an input would result in failing to re-run an action (and
everything downstream) when that input is updated. Failing to declare an output
that is an input to another action produces a race condition between related
actions, in which a single build invocation may miss a timestamp update, and
manifest as a failure to converge in a single invocation (see
[Ninja no-op][no_op]).

If you're reading this, you're probably dealing with a build action that did not
fully state one or more of its inputs or outputs.

## Extending the build with custom actions

Developers can use the GN metabuild system to define custom actions in their
`BUILD.gn` files. This can be done with [`action`][action] and
[`action_foreach`][action_foreach]. Custom actions allow developers to invoke
custom tools at build time, and to hook them up to the dependency graph, such
that the tools can be invoked at build time and correctly re-invoked for
incremental builds when their inputs have changed.

Actions state their inputs using the following parameters:

*   `script`: the tool to run. Often this is a Python script, but it can be any
    program that can be executed on the host.
*   `inputs`: files that are used as data inputs to the tool. For instance if
    the tool compresses a file, then the file to be compressed will be listed as
    an input.
*   `sources`: this is treated the same as `inputs`. The difference is only
    semantic, as `sources` are typically used for additional files used by the
    tool's `script`, e.g. dependent Python or script libraries.

Actions state their outputs using the following parameter:

*   `outputs`: each action must produce at least one output file. Actions that
    don't generate an output file, for instance actions that validate certain
    inputs for correctness, will typically generate a "stamp file", which acts
    as an indicator that the action ran and can be empty.

### Depfiles

If some of the inputs to an action are not known prior to running the action,
then additionally an action can specify a [`depfile`][depfile]. Depfiles list
inputs to the action's one or more outputs that were discovered at runtime. The
format of a depfile is one or more lines as follows:

```
[output_file1]: [input_file1] [input_file2...]
```

All paths in a depfile must be relative to `root_build_dir` (which is set as the
current working directory for actions). See also:
[prefer relative paths from `rebase_path()`][relative-paths].

Tools like compilers should (and do) support emitting a trace of all of the
files used in compilation in the form of a depfile.

## Filesystem action tracing for detecting non-hermetic actions

The Fuchsia build system uses a filesystem action tracing tool to detect if
actions read or wrote files that were not listed as inputs or outputs, either
explicitly in the `BUILD.gn` file or in a depfile, as shown above. This is done
in lieu of a sandbox for running actions, and as a runtime sanitizer of sorts.

If you are reading this page then you're likely contending with an error from
this system. The error will have listed precisely which files were read or
written but were not specified as inputs/outputs in `BUILD.gn` or in a depfile.
You should correct these omissions and attempt to rebuild until the error goes
away.

In order to reproduce this error in a local build, you will need to ensure that
action tracing is enabled:

<pre class="prettyprint">
<code class="devsite-terminal">fx set <var>what</var> --args=build_should_trace_actions=true</code>
</pre>

or interactively, run `fx args`, add a line `build_should_trace_actions=true`,
save and exit.

Note that if your action is not defined hermetically, and you haven't corrected
it, then upon attempting to rebuild the action you may not be encountering an
error. Because the action is not defined hermetically, it may not be correctly
picked up in an incremental build (which is part of the problem that you're
trying to solve). To force all build actions to run, you'll need to clean up
your build's output cache first:

```posix-terminal
fx clean
```

By default, CQ performs these hermeticity checks on all changes. It does so
using the `build_should_trace_actions=true` argument mentioned above, so
developers can reproduce the exact same traced builds locally.

## Suppressing hermetic action checks

Actions that are currently not hermetic have the following parameter set:

```gn
action("foo") {
  ...
  # TODO(fxbug.dev/xxxxx): delete the line below and fix this
  hermetic_deps = false
}
```

This suppresses the check that's described above. If you spot an action that has
this suppression, you should remove the suppression, attempt to reproduce the
issue as outlined above, and fix it.

If instead of fixing it right away, you file a bug, title the bug with
"[hermetic]" and include the output of tracing from the failed build action in
the description. Comment about the access violation if you know where it is
coming from.

## Common issues and how to fix them

### Missing inputs/outputs

Sometimes an input/output is well-known at build time but just isn't specified,
or it's specified incorrectly. These are common and straightforward to fix.
For instance:

*   [472657: [build] Fix hermeticity of hotsort_target_internal](https://fuchsia-review.googlesource.com/c/fuchsia/+/472657)

### Inputs not known until action runtime

As explained above, sometimes not all inputs are known at build time and so
cannot be specified in `BUILD.gn` definitions. This is what [depfiles][depfile]
are for.

You can find an example for fixing a build action to generate a depfile here:

*   [472565: [build] Generate depfile in generate_fidl_json.py](https://fuchsia-review.googlesource.com/c/fuchsia/+/472565)
*   [472657: [build] Fix hermeticity of hotsort_target_internal](https://fuchsia-review.googlesource.com/c/fuchsia/+/472657)
*   [473980: [build] Fix hermeticity of fidl-c-header](https://fuchsia-review.googlesource.com/c/fuchsia/+/473980)
*   [472658: [build] Make go_library build hermetically](https://fuchsia-review.googlesource.com/c/fuchsia/+/472658)
*   [472637: [build] Fix hermeticity of flatbuffer](https://fuchsia-review.googlesource.com/c/third_party/flatbuffers/+/472637)

### Action arguments missing from inputs/outputs

Build actions are often scripts that take certain file paths as arguments.

```gn
action("foo") {
  script = "concatenate.py"
  outputs = [ "$target_out_dir/file1_file2.txt" ]
  args = [
    "--concat-from",
    rebase_path("data/file1.txt", root_build_dir),
    rebase_path("data/file2.txt", root_build_dir),
    "--output",
  ] + outputs
}
```

In the above case you'll get an action tracer error that `concatenate.py`
read from `data/file1.txt` and `data/file2.txt`. The mistake is easy to spot,
because you can see that these paths are passed as args to the script but are
not listed as inputs or outputs. While it's technically possible to pass paths
as args and not actually have the script read/write to those paths, it's very
unlikely.

The fix is as follows:

```gn
action("foo") {
  script = "concatenate.py"
  sources = [
    "data/file1.txt",
    "data/file2.txt",
  ]
  outputs = [ "$target_out_dir/file1_file2.txt" ]
  args = [
    "--concat-from",
  ] + rebase_path(sources, root_build_dir) + [
    "--output",
  ] + outputs
}
```

### Expanding arguments from a file

There is a common pattern used especially in Python scripts to expand the
contents of a file as arguments (also known as a "response file"). In `BUILD.gn`
you will find:

```gn
action("foo") {
   script = "myaction.py"
   args = [ "@" + rebase_path(args_file, root_build_dir) ]
   ...
}
```

Then in the associated Python file `myaction.py` you will find
an argument parser with [`fromfile_prefix_chars`][fromfile_prefix_chars]:

```python
def main():
    parser = argparse.ArgumentParser(fromfile_prefix_chars='@')
    args = parser.parse_args()
    ...
```

The problem with the above is that `args_file` is read at runtime by the Python
script, and should be specified as an input. To fix:

```gn
action("foo") {
   script = "myaction.py"
   inputs = [ args_file ]
   args = [ "@" + rebase_path(args_file, root_build_dir) ]
   ...
}
```

If you need to quickly populate such a file from a list in GN, you can use
[`write_file()`][write_file]:

```gn
action("foo") {
  args_file = "${target_gen_dir}/${target_name}.args"
  write_file(args_file, a_very_long_list_of_args)
  args = [ "@" + rebase_path(args_file, root_build_dir) ]
  ...
}
```

Note GN provides [`response_file_contents`][response_file_contents] as a
convenient alternative, instead of `write_file`, for this purpose. However due
to a [bug][response_file_bug] rooted in Ninja, we currently don't allow
`response_file_contents` in our builds.

### Creating and deleting temporary files

It is a common pattern in build actions to create temporary files.
It's ok to not list temporary files as outputs so long as the same action that
creates the temporary files also deletes them before returning.

Temporary files should be saved under `target_out_dir` or `target_gen_dir`.
Use of global temporary storage such as `/tmp` or `$TMPDIR`, or any reads and
writes outside of the checkout or output directories, is discouraged because
it can make troubleshooting of build failures more difficult as files may need
to be recovered from other places in the filesystem to indicate what went
wrong.

### Creating and deleting temporary directories

Sometimes temporary files need to be created in temporary directories.
Again this is fine as long as the action that creates the temporary directory
also recursively deletes it before returning.

[`shutil.rmtree`][shutil_rmtree] is a common function used to delete temporary
directories. However, due to a limitation in our tracer, this would sometimes
result in spurious unexpected reads. See also: [Issue 75057: Properly handle
directory deletion from shutil.rmtree in action tracer][shutil_rmtree_bug].

One way to get around this limitation is to only create temporary files, not
temporary directories. Temporary files should be written under `target_out_dir`
or `target_gen_dir`.

Sometimes this is not possible, for instance when the temporary directory is
created by an external build tool that cannot be modified. In this case, an
alternative is to give your temporary directories a special name, for instance
`__untraced_foo_tmp_outputs__`, and allowlist them in the
[action tracer][action_tracer_ignored_path_parts]. Accesses to files in this
special directory will be ignored by the tracer. __Because of this, this
feature should not be used lightly.__

For example, assuming `bar.py` always deletes all files in the `--tmp-dir`
passed to it, then re-populates:

```gn
action(target_name) {
  script = "bar.py"
  args = [
    "--tmp-dir"
    rebase_path("${target_gen_dir}/${target_name}/__untraced_bar_tmp_outputs__", root_build_dir)
  ]
  ...
}
```

Then in the action tracer, add an entry in `ignored_path_parts`:

```py
ignored_path_parts = {
  # Comment with clear explanation on why this is necessary,
  # preferably with a link to an associated bug for more context.
  "__untraced_bar_tmp_outputs__",
  ...
}
```

See also: [hermetic actions in open projects][hermetic-actions-bb]

[action]: https://gn.googlesource.com/gn/+/master/docs/reference.md#func_action
[action_foreach]: https://gn.googlesource.com/gn/+/master/docs/reference.md#func_action_foreach
[action_tracer_ignored_path_parts]: https://cs.opensource.google/fuchsia/fuchsia/+/main:build/tracer/action_tracer.py;l=873;drc=57a94b356da70385d8439b1dd3f355d9850c2db2
[depfile]: https://gn.googlesource.com/gn/+/master/docs/reference.md#var_depfile
[fromfile_prefix_chars]: https://docs.python.org/3/library/argparse.html#fromfile-prefix-chars
[hermetic-actions-bb]: /docs/contribute/open_projects/build/hermetic_actions.md
[no_op]: /docs/development/build/ninja_no_op.md
[relative-paths]: /docs/development/build/build_system/best_practices_templates.md#prefer-relative-paths-from-rebase-path
[response_file_bug]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=76068
[response_file_contents]: https://gn.googlesource.com/gn/+/master/docs/reference.md#var_response_file_contents
[shutil_rmtree]: https://docs.python.org/3/library/shutil.html#shutil.rmtree
[shutil_rmtree_bug]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=75057
[write_file]: https://gn.googlesource.com/gn/+/master/docs/reference.md#func_write_file
