# Hermetic actions

## Goal & motivation

When defining a build action in any build system, it's important to correctly
and fully specify what are the inputs and outputs of that action.
Incremental builds (partial rebuilds following a small change to sources) rely
on this information being present and correct in the build graph to correctly
identify and execute rebuilds. If this information is incorrect or incomplete,
rebuilds might produce wrong outcomes, or otherwise different outcomes from full
clean builds.

Actions that only read from their declared inputs and only write to their
declared outputs are hermetic to the build system.
Today we don't have a fully hermetic build, but rather we have dozens of actions
and tools that read/write files that are not in their declared inputs/outputs.
This keeps us from ever fully relying on incremental rebuilds in production,
forcing us to do full clean builds, which are ~10x slower.

Furthermore, these non-hermetic actions often manifest themselves as incoherent
behavior from the build system that engineers run into and waste their time
trying to troubleshoot.

The goal of this project is to investigate and fix all instances of non-hermetic
build actions in our build graph.

## Technical background

Familiarity with GN and `BUILD.gn` files is important, particularly with the
following target types:

*   [`action`](https://gn.googlesource.com/gn/+/master/docs/reference.md#func_action)
*   [`action_foreach`](https://gn.googlesource.com/gn/+/master/docs/reference.md#func_action_foreach)
*   [`template`](https://gn.googlesource.com/gn/+/master/docs/reference.md#func_template)

In many cases you will also need to understand how depfiles work:

*   [`depfiles`](https://gn.googlesource.com/gn/+/master/docs/reference.md#var_depfile)

Additional important information can be found in this guide:

*   [Hermetic build actions][hermetic-build-actions]

## How to help

### Picking a task

Pick any build target that is marked as non-hermetic. They look as follows:

```gn
action("foo") {
  ...
  # TODO(fxbug.dev/xxxxx): delete the line below and fix this
  hermetic_deps = false
}
```

### Doing a task

#### Reproducing the issue

In order to understand why an action is non-hermetic, you need to build it with
the action tracer tool enabled. This tool runs all build actions while tracing
their filesystem accesses, then compares those accesses against the declared
inputs / outputs / depfile of those actions.

To reproduce the issue locally, first remove `hermetic_deps = false` and then
set up your build as follows:

<pre class="prettyprint">
<code class="devsite-terminal">fx set <var>what</var> --args=build_should_trace_actions=true</code>
</pre>

Running a build with the above setting should produce an error with
actionable troubleshooting information.

CQ runs action tracing by default, so you can use it to get the same
troubleshooting information.

If there's already a bug filed for the issue then the bug should include the
information that's in the error. If there isn't a bug, please file one and
record the information that you've gathered.

Example: [fxbug.dev/68307](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=68307)

#### Fixing the issue

There are several common reasons for hermeticity issues. You can find some of
them in [this guide][common-issues].
If you run across an issue that's not covered in the guide, please consider
improving the guide.

If you're able to remove `hermetic_deps = false` and still successfully build
locally with tracing or the traced tryjob passed, then your change is ready for
review.

### Completing a task

Find reviewers by OWNERS and merge your change.

## Examples

*   [472565: [build] Generate depfile in generate_fidl_json.py](https://fuchsia-review.googlesource.com/c/fuchsia/+/472565)
*   [472657: [build] Fix hermeticity of hotsort_target_internal](https://fuchsia-review.googlesource.com/c/fuchsia/+/472657)
*   [473980: [build] Fix hermeticity of fidl-c-header](https://fuchsia-review.googlesource.com/c/fuchsia/+/473980)
*   [472658: [build] Make go_library build hermetically](https://fuchsia-review.googlesource.com/c/fuchsia/+/472658)
*   [472637: [build] Fix hermeticity of flatbuffer](https://fuchsia-review.googlesource.com/c/third_party/flatbuffers/+/472637)

## Sponsors

Reach out for questions or for status updates:

*   <digit@google.com>
*   <fangism@google.com>
*   <jayzhuang@google.com>
*   <shayba@google.com>

[hermetic-build-actions]: development/build/hermetic_actions.md
[common-issues]: development/build/hermetic_actions.md#common_issues_and_how_to_fix_them
