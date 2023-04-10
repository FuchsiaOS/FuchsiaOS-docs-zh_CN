# Best practices for using GN toolchains

## Overview

In GN, toolchains provide a way to build targets in multiple ways. To understand
and debug GN code, you need to know what toolchain you're in. Because GN code
can be conditional on `current_toolchain`, a target that does one thing in
toolchain **A** might do something completely different in toolchain **B**, and
it might not exist at all in toolchain **C**.

This document details the best practices for using toolchains to solve common
problems in GN code (`.gn` and `.gni` files). These best practices are in
addition to the best practices outlined in [Fuchsia build system
policies](policies.md).

See [GN toolchains and the Fuchsia Build][gn-toolchains-overview] to learn more
about how toolchains work, or run `fx gn help toolchain` to see GN's built-in
documentation.

## Goals

The [best practices](#best-practices) in this document are based on the
following goals:

* **Consistency**. Prefer to have one way of doing things.
* **Clarity**. Communicate intent clearly with assertions.
* **Performance**. Avoid unnecessary work in the build.

## Best practices {#best-practices}

### Assert on the expected toolchain {#assert-on-the-expected-toolchain}

If a file is only expected to be used in one toolchain or in certain toolchains,
put an assertion at the top.

<span class="compare-better">Recommended</span>: Asserting `is_host` in a
`BUILD.gn` file that only builds host executables.

```gn
assert(is_host)

# ...
```

<span class="compare-better">Recommended</span>: Asserting `current_toolchain ==
default_toolchain` in a template that only makes sense in the default toolchain.

```gn
template("foo") {
  assert(current_toolchain == default_toolchain,
         "The foo template can only be used in the default toolchain")
}
```

### Wrap targets in conditionals {#wrap-targets-in-conditionals}

If you can't [assert on the expected
toolchain](#assert-on-the-expected-toolchain) because a file needs to use more
than one toolchain, wrap targets in conditional blocks to avoid unnecessary
expansion. This makes it easier to understand what targets are used where, and
it also helps reduce GN gen time.

<span class="compare-better">Recommended</span>: Wrapping targets in `is_host`
and `is_fuchsia` checks.

```gn
# example/BUILD.gn

executable("built_everywhere") {
  # ...
}

if (is_host) {
  executable("only_on_host") {
    # ...
  }
}

if (is_fuchsia) {
  executable("only_on_fuchsia") {
    # ...
  }
}
```

<span class="compare-worse">Not recommended</span>: Defining all targets
unconditionally.

```gn
# example/BUILD.gn

executable("built_everywhere") {
  # ...
}

executable("only_on_host") {
  # ...
}

executable("only_on_fuchsia") {
  # ...
}
```

This approach increases the number of targets, slowing down both GN and ninja.
For example, when GN sees a reference to `example:only_on_fuchsia` in the
default toolchain, it evaluates all of example/BUILD.gn in the default
toolchain, including the `only_on_host` target. Since this cascades transitively
through the target's dependencies, getting this wrong in certain places can lead
to [tens of thousands of unwanted targets](/bundles/README.md).

### Use `is_*` variables to check the toolchain {#use-is-variables-to-check-the-toolchain}

When [asserting](#assert-on-the-expected-toolchain) or writing a
[conditional](#wrap-targets-in-conditionals) on the current toolchain, use one
of the `is_*` variables defined in
[BUILDCONFIG.gn](/build/config/BUILDCONFIG.gn) if one meets your needs:

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="build/config/BUILDCONFIG.gn" region_tag="toolchain_is_variables" adjust_indentation="auto" exclude_regexp="^$" %}
```

<span class="compare-better">Recommended</span>: Using `is_host` to check for
host toolchains.

```gn
if (is_host) {
  # ...
}
```

<span class="compare-worse">Not recommended</span>: Using `current_toolchain ==
host_toolchain` to check for host toolchains.

```gn
if (current_toolchain == host_toolchain) {
  # ...
}
```

Checking `current_toolchain == host_toolchain` is usually wrong because there
are multiple host toolchains when [variants](variants.md) are involved.

Only check the value of `current_toolchain` if you have a reason for doing so.
For example, one valid use case is checking if `current_toolchain ==
default_toolchain` to define a [toolchain-agnostic
action](#put-toolchain-agnostic-actions-in-the-default-toolchain).

### Prefer fewer, earlier toolchain redirections {#prefer-fewer-earlier-toolchain-redirections}

To get to a non-default toolchain, you have to redirect to it at some point.
Push these redirections as far up the build graph as possible. This results in
fewer redirections, and allows you to [assert on the expected toolchain]
(#assert-on-the-expected-toolchain).

<span class="compare-better">Recommended</span>: Redirecting to `host_toolchain`
once earlier in the build.

```gn
# example/BUILD.gn

group("tests") {
  testonly = true
  deps = [ "foo:tests($host_toolchain)" ]
}
```

```gn
# examples/foo/BUILD.gn

assert(is_host)

test("foo_unit_tests") {
  # ...
}

test("foo_integration_tests") {
  # ...
}

group("tests") {
  testonly = true
  deps = [
    ":foo_unit_tests",
    ":foo_integration_tests",
  ]
}
```

<span class="compare-worse">Not recommended</span>: Redirecting to
`host_toolchain` multiple times later in the build.

```gn
# example/BUILD.gn

group("tests") {
  testonly = true
  deps = [ "foo:tests" ]
}
```

```gn
# examples/foo/BUILD.gn

if (is_host) {
  test("foo_unit_tests") {
    # ...
  }

  test("foo_integration_tests") {
    # ...
  }
}

group("tests") {
  testonly = true
  deps = [
    ":foo_unit_tests($host_toolchain)",
    ":foo_integration_tests($host_toolchain)",
  ]
}
```

This approach needlessly processes examples/foo/BUILD.gn twice, once in the
default toolchain and again in the host toolchain.

### Avoid automatic toolchain forwarding {#avoid-automatic-toolchain-forwarding}

If a target only makes sense in a particular toolchain, simply [assert on the
expected toolchain](#assert-on-the-expected-toolchain).

<span class="compare-better">Recommended</span>: Asserting on the expected
toolchain and defining the target once.

```gn
assert(current_toolchain == desired_toolchain)

action(target_name) {
  # ...
}
```

<span class="compare-worse">Not recommended</span>: Hiding the toolchain
requirement with a GN group that automatically redirects all other toolchains.

```gn
if (current_toolchain == desired_toolchain) {
  action(target_name) {
    # ...
  }
} else {
  group(target_name) {
    public_deps = [ ":$target_name($desired_toolchain)" ]
  }
}
```

While it might seem convenient to make the target work in any toolchain, this
practice makes it harder to understand what's really going on.

### Put toolchain-agnostic actions in the default toolchain {#put-toolchain-agnostic-actions-in-the-default-toolchain}

Some actions behave the same no matter what the toolchain is, so it's wasteful
to repeat them in multiple toolchains. The most common example is code
generation: while we might build the resulting code in multiple toolchains, we
shouldn't have to generate the code again every time. To solve this, ensure the
action is only defined in `default_toolchain`.

<span class="compare-better">Recommended</span>: Running code generation once in
the default toolchain.

```gn
if (current_toolchain == default_toolchain) {
  action("codegen") {
    visibility = [ ":*" ]
    outputs = [ "$target_gen_dir/main.cc" ]
    # ...
  }
}

executable("program") {
  deps = [ ":codegen($default_toolchain)" ]
  sources = get_target_outputs(deps[0])
  # ...
}
```

<span class="compare-worse">Not recommended</span>: Redoing code generation in
every toolchain.

```gn
action("codegen") {
  visibility = [ ":*" ]
  outputs = [ "$target_gen_dir/main.cc" ]
  # ...
}

executable("program") {
  deps = [ ":codegen" ]
  sources = get_target_outputs(deps[0])
  # ...
}
```

### Use the `:anything` label to get output directories {#use-the-anything-label-to-get-output-directories}

When you call `get_label_info` with "target_gen_dir" or "target_out_dir", only
the label's directory matters, not its target name. If there is no specific
target that makes sense, use a fake target called "anything".

<span class="compare-better">Recommended</span>: Naming the fake target "anything".

```gn
codegen_dir = get_label_info(":anything($default_toolchain)", "target_gen_dir")
```

<span class="compare-worse">Not recommended</span>: Naming the fake target something other than "anything".

```gn
codegen_dir = get_label_info(":bogus($default_toolchain)", "target_gen_dir")
```

### Avoid language-specific toolchains {#avoid-language-specific-toolchains}

Do not create a toolchain for a particular programming language. We did this
early on, and it turned out to be a bad idea. For example, we used to have
`rust_toolchain` but later removed it. We are also [planning to
remove](https://fxbug.dev/108355) the `fidl_toolchain`.

[gn-toolchains-overview]: /docs//development/build/build_system/internals/toolchains/gn_toolchains_overview.md
