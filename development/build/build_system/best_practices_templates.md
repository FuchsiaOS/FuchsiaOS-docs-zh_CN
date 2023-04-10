# Best practices for writing GN templates

## Overview {#overview}

In GN, templates provide a way to add on to GN’s built-in target types. Basically,
templates are GN’s primary way to build reusable functions. Template definitions go
in `.gni` (GN import) files that can be imported into target `.gn` files.

This document details the best practices for creating GN templates, and each best
practice includes an example. These best practices are in addition to the best
practices outlined in [Fuchsia build system policies](policies.md).

Run `fx gn help template` for more information and more complete examples, and see
[GN Language and Operation](https://gn.googlesource.com/gn/+/HEAD/docs/language.md#templates)
for more information on GN features.

##  Templates {#templates}

### Define templates in `.gni`, targets in `BUILD.gn` {#define-templates-in-gni-targets-in-build-gn}

Technically, it’s possible to import both `.gni` and `BUILD.gn` files. The best
practice, however, is to define templates in `.gni` files, and
targets in `.gn` files. This makes it clear to users what’s a template. Users
want to import templates so they can use them, and never want to import targets.

### Document templates and args {#document-templates-and-args}

Document both your templates and args, including:

*   A general explanation of the template’s purpose and concepts introduced. A practical usage example is recommended.
*   All parameters should be documented. Parameters that are common and simply forwarded (such as `deps` or `visibility`), where the meaning is consistent with their meaning on built-in GN rules, can be listed with no additional information.
*   If a template generates `metadata,` then `data_keys` should be listed.

To document your template, insert a comment block in front of your template definition
to specify your public contract.

```gn
declare_args() {
  # The amount of bytes to allocate when creating a disk image.
  disk_image_size_bytes = 1024
}

# Defines a disk image file.
#
# Disk image files are used to boot the bar virtual machine.
#
# Example:
# ```
# disk_image("my_image") {
#   sources = [ "boot.img", "kernel.img" ]
#   sdk = false
# }
# ```
#
# Parameters
#
#  sources (required)
#    List of source files to include in the image.
#    Type: list(path)
#
#  sdk (optional)
#    This image is exported to the SDK.
#    Type: bool
#    Default: false
#
#  data_deps
#  deps
#  public_deps
#  testonly
#  visibility
#
# Metadata
#
#  files
#    Filenames present in this image.
template("disk_image") {
  ...
}
```

### Wrap tools with a single action template {#wrap-tools-with-a-single-action-template}

For every tool, have a canonical template that wraps it with an `action`.
This template’s job is to turn GN parameters into `args` for the tool, and
that’s it. This sets an encapsulation boundary around the tool for details
such as translating parameters to args.

Note that in this example we define the `executable()` in one file and the
`template()` in another, because
[templates and targets should be separated](#define-templates-in-gni-targets-in-build-gn).

```gn
# //src/developer_tools/BUILD.gn
executable("copy_to_target_bin") {
  ...
}

# //src/developer_tools/cli.gni
template("copy_to_target") {
  compiled_action(target_name) {
    forward_variables_from(invoker, [
                                      "data_deps",
                                      "deps",
                                      "public_deps",
                                      "testonly",
                                      "visibility"
                                    ])
    assert(defined(invoker.sources), "Must specify sources")
    assert(defined(invoker.destinations), "Must specify destinations")
    tool = "//src/developer_tools:copy_to_target_bin"
    args = [ "--sources" ]
    foreach(source, sources) {
      args += [ rebase_path(source, root_build_dir) ]
    }
    args += [ "--destinations" ]
    foreach(destination, destinations) {
      args += [ rebase_path(destination, root_build_dir) ]
    }
  }
}
```

### Consider making templates private {#consider-making-templates-private}

Templates and variables whose name begins with an underscore (e.g. `template("_private")`)
are considered private and won’t be visible to other files that `import()` them, but can be
used in the same file that they’re defined. This is useful for internal helper templates or
“local global variables” that you might define for instance to share logic between two templates,
where the helper is not useful to the user.

```gn
template("coffee") {
  # Take coffee parameters like roast and sugar
  ...
  _beverage(target_name) {
    # Express in beverage terms like ingredients and temperature
    ...
  }
}

template("tea") {
  # Take tea parameters like loose leaf and cream
  ...
  _beverage(target_name) {
    # Express in beverage terms like ingredients and temperature
    ...
  }
}

# We don't want people directly defining new beverages.
# For instance they might add both sugar and salt to the ingredients list.
template("_beverage") {
  ...
}
```

Sometimes you can’t make a template private because it actually needs to be used
from different files, but you’d still like to hide it because it’s not meant to
be used directly. In situations like this you can swap enforcement for signaling, by
putting your template in a file under a path such as `//build/internal/`.

### Test your templates {#test-your-templates}

Write tests that use your templates to build, or use files generated by your
templates in the course of the test.

You should not rely on other people’s builds and tests to test your template.
Having your own tests makes your template more maintainable, since it’s faster
to validate future changes to your template and it’s easier to isolate faults.

```gn
# //src/drinks/coffee.gni
template("coffee") {
  ...
}

# //src/drinks/tests/BUILD.gni
import("//src/drinks/coffee.gni")

coffee("coffee_for_test") {
  ...
}

test("coffee_test") {
  sources = [ "taste_coffee.cc" ]
  data_deps = [ ":coffee_for_test" ]
  ...
}
```

## Parameters {#parameters}

### Assert on required parameters {#assert-on-required-parameters}

If you have required parameters in your template, `assert` that they’re defined.

If a user forgets to specify a required parameter, and there’s no assert defined,
they won’t get a clear explanation for their error. Using an assert allows you to
provide a useful error message.

```gn
template("my_template") {
  forward_variables_from(invoker, [ "sources", "testonly", "visibility" ])
  assert(defined(sources),
      "A `sources` argument was missing when calling my_template($target_name)")
}

template("my_other_template") {
  forward_variables_from(invoker, [ "inputs", "testonly", "visibility" ])
  assert(defined(inputs) && inputs != [],
      "An `input` argument must be present and non-empty " +
      "when calling my_template($target_name)")
}
```

### Always forward `testonly` {#always-forward-testonly}

Setting `testonly` on a target guards it against being used by non-test targets.
If your template doesn’t forward `testonly` to inner targets then:

1. Your inner targets might fail to build, because your users might pass you `testonly` dependencies.
2. You’ll surprise your users when they find that their `testonly` artifacts end up in production artifacts.

The following example shows how to forward `testonly`:

```gn
template("my_template") {
  action(target_name) {
    forward_variables_from(invoker, [ "testonly", "deps" ])
    ...
  }
}

my_template("my_target") {
  visibility = [ ... ]
  testonly = true
  ...
}
```

Note that if the parent scope for the inner action defines `testonly`
then `forward_variables_from(invoker, "*")` won’t forward it, as it
avoids clobbering variables. Here are some patterns to work around this:

```gn
# Broken, doesn't forward `testonly`
template("my_template") {
  testonly = ...
  action(target_name) {
    forward_variables_from(invoker, "*")
    ...
  }
}

# Works
template("my_template") {
  testonly = ...
  action(target_name) {
    forward_variables_from(invoker, "*")
    testonly = testonly
    ...
  }
}

# Works
template("my_template") {
  testonly = ...
  action(target_name) {
    forward_variables_from(invoker, "*", [ "testonly" ])
    forward_variables_from(invoker, [ "testonly" ])
    ...
  }
}
```

The one exception to this are templates that hard-code `testonly = true` because
they should never be used in production targets. For example:

```gn
template("a_test_template") {
  testonly = true
  ...
}
```

### Forward `visibility` to the main target and hide inner targets {#forward-visibility-to-the-main-target-hide-inner-targets}

GN users expect to be able to set `visibility` on any target.

This advice is similar to [always forward testonly](#heading=h.fk6w1as9tkpx), except that
it only applies to the main target (the target named `target_name`). Other targets should
have their `visibility` restricted, so that your users can’t depend on your inner targets
that are not part of your contract.

```gn
template("my_template") {
  action("${target_name}_helper") {
    forward_variables_from(invoker, [ "testonly", "deps" ])
    visibility = [ ":*" ]
    ...
  }

  action(target_name) {
    forward_variables_from(invoker, [ "testonly", "visibility" ])
    deps = [ ":${target_name}_helper" ]
    ...
  }
}
```

### If forwarding `deps`, also forward `public_deps` and `data_deps` {#if-forwarding-deps-also-forward-public_deps-and-data_deps}

All built-in rules that take `deps` take `public_deps` and `data_deps`.
Some built-in rules don’t differentiate between types of deps (e.g. `action()`
treats `deps` and `public_deps` equally). But dependants on your generated
targets might (e.g. an `executable()` that deps on your generated `action()`
treats transitive `deps` and `public_deps` differently).

```gn
template("my_template") {
  action(target_name) {
    forward_variables_from(invoker, [
                                       "data_deps",
                                       "deps",
                                       "public_deps",
                                       "testonly",
                                       "Visibility"
                                    ])
    ...
  }
}
```

## Target Names {#target-names}

### Define an inner target named `target_name` {#define-an-inner-target-named-target_name}

Your template should define at least one target that is named `target_name`.
This allows your users to invoke your template with a name, and then use that
name in their deps.

```gn
# //build/image.gni
template("image") {
  action(target_name) {
    ...
  }
}

# //src/some/project/BUILD.gn
import("//build/image.gni")

image("my_image") {
  ...
}

group("images") {
  deps = [ ":my_image", ... ]
}
```

### `target_name` is a good default for an output name, but offer an override {#target_name-is-a-good-default-for-an-output-name-but-offer-an-override}

If your template produces a single output then using the target name to select
the output name is good default behavior. However, target names must be unique
in a directory, so your users won’t always be able to use the name that they
want both for the target and the output.

It’s a good best practice to offer users an override:

```gn
template("image") {
  forward_variables_from(invoker, [ "output_name", ... ])
  if (!defined(output_name)) {
    output_name = target_name
  }
  ...
}
```

### Prefix internal target names with `$target_name` {#prefix-internal-target-names-with-$target_name}

GN labels must be unique, or else you’ll get a gen-time error. If everyone on
the same project follows the same naming convention then collisions are less
likely to happen and it becomes easier to associate internal target names
with the targets that created them.

```gn
template("boot_image") {
  generate_boot_manifest_action = "${target_name}_generate_boot_manifest"
  action(generate_boot_manifest_action) {
    ...
  }

  image(target_name) {
    ...
    deps += [ ":$generate_boot_manifest_action" ]
  }
}
```

### Do not infer output names from target labels {#do-not-infer-output-names-from-target-labels}

It’s tempting to assume a relationship between target names and output names.
For instance, the following example will work:

```gn
executable("bin") {
  ...
}

template("bin_runner") {
  compiled_action(target_name) {
    forward_variables_from(invoker, [ "testonly", "visibility" ])
    assert(defined(invoker.bin), "Must specify bin")
    deps = [ invoker.bin ]
    tool = root_out_dir + "/" + get_label_info(invoker.foo, "name")
    ...
  }
}

bin_runner("this_will_work") {
  bin = ":bin"
}
```

However this example will product a gen-time error:

```gn
executable("bin") {
  output_name = "my_binary"
  ...
}

template("bin_runner") {
  compiled_action(target_name) {
    forward_variables_from(invoker, [ "testonly", "visibility" ])
    assert(defined(invoker.bin), "Must specify bin")
    tool = root_out_dir + "/" + get_label_info(invoker.bin, "name")
    ...
  }
}

# This will produce a gen-time error saying that a file ".../bin" is needed
# by ":this_will_fail" with no rule to generate it.
bin_runner("this_will_fail") {
  bin = ":bin"
}
```

Here’s one way of fixing this problem:

```gn
executable("bin") {
  output_name = "my_binary"
  ...
}

template("bin_runner") {
  compiled_action(target_name) {
    forward_variables_from(invoker, [ "testonly", "visibility" ])
    assert(defined(invoker.bin), "Must specify bin")
    tool = bin
    ...
  }
}

bin_runner("this_will_work") {
  bin = "$root_out_dir/my_binary"
}
```

## GN functions and generation {#gn-functions-and-generation}

### Only use `read_file()` with source files {#only-use-read_file-with-source-files}

`read_file()` occurs during generation and can not be safely used to read from generated
files or build outputs. It can be used to read source files, for example to read
a manifest file or a json file with which to populate build dependencies.
Notably `read_file()` can not be used with `generated_file()` or `write_file()`.

### Prefer `generated_file()` over `write_file()` {#prefer-generated_file-over-write_file}

In general, it’s recommended that you use `generated_file()` over `write_file()`.
`generated_file()` provides additional features and addresses some of the challenges
of `write_file()`. For instance, `generated_file()` can be executed in parallel,
while `write_file()` is done serially at gen time.

The structure of both commands is very similar. For instance, you can turn
this instance of `write_file()`:

```gn
write_file("my_file", "My file contents")
```

Into this instance of `generated_file()`:

```gn
generated_file("my_file") {
  outputs = [ "my_file" ]
  contents = "My file contents"
}
```

### Prefer relative paths from `rebase_path()` {#prefer-relative-paths-from-rebase-path}

Always specify a `new_base` in `rebase_path()`, for example
`rebase_path("foo/bar.txt", root_build_dir)`. Avoid its one-parameter form, that
is `rebase_path("foo/bar.txt")`.

GN's `rebase_path()` has three parameters, with the latter two being optional.
Its one-parameter form returns an absolute path, and it is
[being deprecated][rebase-path-thread]. Avoid it in build templates and targets.
The value of `new_base` varies case-by-case, with `root_build_dir` being a
common choice, because it is where build scripts are executed. See more
information about `rebase_path()` in its
[GN reference][gn-reference-rebase-path].

Relative paths can stay unchanged when paths to project or build output
directory changes. It has a few advantages over absolute paths:

*   Protects user privacy by not leaking potentially sensitive information from
    paths in build outputs.
*   Improves efficiency of content-addressed caches.
*   Makes interactions between bots possible, for example, one bot performs an
    action following another bot.

See also:
[`rebase_path(x)` returning absolute paths considered harmful?][rebase-path-thread]

## Patterns and anti-patterns {#patterns-and-anti-patterns}

### Target outputs {#target-outputs}

When working with `get_target_outputs()` to extract a single element, GN won’t
let you subscript a list before assignment. To work around this issue,
you can use the less than elegant workaround below:

```gn
# Appending to a list is elegant
deps += get_target_outputs(":some_target")

# Extracting a single element to use in variable substitution - ugly but reliable
_outputs = get_target_outputs(":other_target")
output = _outputs[0]
message = "My favorite output is $output"

# This expression is invalid: `output = get_target_outputs(":other_target")[0]`
# GN won't let you subscript an rvalue.
```

Additionally, `get_target_outputs()` has a few painful limitations:

*   Only `copy()`, `generated_file()`, and `action()` targets are supported.
*   Only targets defined in the same `BUILD.gn` file may be queried.

As a result, often you will find one target's output path hard-coded in another
`BUILD.gn` file. This creates a brittle contract. When the contract breaks,
troubleshooting the breakage is difficult. Please try to avoid doing this when you
can, and add a lot of inline documentation when you can't.

### Checking if type is string

Though GN doesn't allow for comprehensive type checking, you can check that a variable
is a string, and only a string, by writing the following line:

```gn
if (var == "$var") {
  # Execute code conditional on `var` type being string
}
```

### Check if var is a singleton list

Similarly, you can check if a variable is a singleton list like this:

```gn
if (var == [var[0]]) {
  # Execute code conditional on `var` type being a singleton list
}
```

Though note, that this will crash if the type is *not* a list or is empty.

### Set operations {#set-operations}

GN offers lists and scopes as aggregate data types, but not associative
types like maps or sets. Sometimes lists are used instead of sets. The
example below has a list of build variants, and checks if one of them
is the “profile” variant:

```gn
if (variants + [ "profile" ] - [ "profile" ] != variants) {
  # Do something special for profile builds
  ...
}
```

This is an anti-pattern. Rather, variants could be defined as follows:

```gn
variants = {
  profile = true
  asan = false
  ...
}

if (variants.profile) {
  # Do something special for profile builds
  ...
}
```

### Forwarding `"*"` {#forwarding-*}

`forward_variables_from()` copies specified variables to the current
scope from the given scope _or any enclosing scope_. Unless you
specify `"*"`, in which case it will only directly copy variables
from the given scope. And it will never clobber a variable that’s
already in your scope - that’s a gen-time error.

Sometimes you want to copy everything from the invoker, except for
a particular variable that you want to copy from any enclosing
scope. You’ll encounter this pattern:

```gn
forward_variables_from(invoker, "*", [ "visibility" ])
forward_variables_from(invoker, [ "visibility" ])
```

### `exec_script()` {#exec-script}

GN's built-in function
[exec_script](https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#func_exec_script)
is a powerful tool for augmenting GN's abilities. Like `action()`,
`exec_script()` can invoke an external tool. Unlike `action()`, `exec_script()`
can invoke the tool **synchronously** with build generation, meaning that you
can use the output of the tool in your `BUILD.gn` logic.

Since this creates a performance bottleneck in gen time (i.e. `fx set` takes
longer), this feature must be used with care.
For more information, refer to
[this writeup](https://chromium.googlesource.com/chromium/src/+/ab1c69b1814d3c905fdab7b0d177b478eecf40a3/.gn#291)
by the Chromium team.

An allowlist has been set up in `//.gn`. Please consult `OWNERS` for changes
made to this allowlist.

[rebase-path-thread]: https://groups.google.com/a/chromium.org/g/gn-dev/c/WOFiYgcGgjw
[gn-reference-rebase-path]: https://gn.googlesource.com/gn/+/master/docs/reference.md#func_rebase_path
