<!-- 
# Best practices for writing GN templates
 -->
# 编写 GN 模板的最佳实践

<!-- 
## Overview {#overview}
 -->
## 概述 {#overview}

<!-- 
In GN, templates provide a way to add on to GN’s built-in target types. Basically,
templates are GN’s primary way to build reusable functions. Template definitions go
in `.gni` (GN import) files that can be imported into target `.gn` files.

This document details the best practices for creating GN templates, and each best
practice includes an example. These best practices are in addition to the best
practices outlined in [Fuchsia build system policies](policies.md).

Run `gn help template` for more information and more complete examples, and
[GN Language and Operation](https://gn.googlesource.com/gn/+/HEAD/docs/language.md#templates)
for more information on GN features.
 -->
在 GN 中，模板（template）提供了一种添加到 GN 内置目标类型的方法。根本上讲，模板是 GN 构建可重用功能的主要方式。模板定义放在 `.gni`（GN import）文件内，这种文件可以导入 `.gn` 目标文件。

本文档详细介绍了创建 GN 模板的最佳做法，它们每一条都包含了一个示例。这些最佳实践是对 [Fuchsia 构建系统策略](policies.md)中概述的最佳实践的补充。

运行 `gn help template` 以获取更多信息和更完整的示例，查看 [GN 语言与操作](https://gn.googlesource.com/gn/+/HEAD/docs/language.md#templates)以获取有关 GN 特性的更多信息。

<!-- 
##  Templates {#templates}
 -->
##  模板 {#templates}

<!-- 
### Define templates in `.gni`, targets in `BUILD.gn` {#define-templates-in-gni-targets-in-build-gn}
 -->
### 在 `.gni` 中定义模板，在 `BUILD.gn` 中定义目标 {#define-templates-in-gni-targets-in-build-gn}

<!-- 
Technically, it’s possible to import both `.gni` and `BUILD.gn` files. The best
practice, however, is to define templates in `.gni` files, and
targets in `.gn` files. This makes it clear to users what’s a template. Users
want to import templates so they can use them, and never want to import targets.
 -->
技术上是可以做到同时导入 `.gni` 和 `BUILD.gn` 文件的。不过，最佳实践是在 `.gni` 文件内定义模板，并在 `.gn` 文件内定义目标。这样一来，用户就可以清楚地知道模板是什么。用户希望导入模板以便使用，而从不想要导入目标。

<!-- 
### Document templates and args {#document-templates-and-args}
 -->
### 为模板和参数编写说明文档 {#document-templates-and-args}

<!-- 
Document both your templates and args, including:

*   A general explanation of the template’s purpose and concepts introduced. A practical usage example is recommended.
*   All parameters should be documented. Parameters that are common and simply forwarded (such as `deps` or `visibility`), where the meaning is consistent with their meaning on built-in GN rules, can be listed with no additional information.
*   If a template generates `metadata,` then `data_keys` should be listed.

To document your template, insert a comment block in front of your template definition
to specify your public contract.
 -->
请为您的模板和参数编写说明文档，其中包括：

* 一段模板用途和概念的简单介绍。建议使用一个实际运用的示例。

* 所有参数都应介绍说明。有些参数比较简单常见（如 `deps` 或 `visibility`），并且它们的含义与内置 GN 规则中的含义一致，这样的参数可以直接列出而无需附加信息。

* 如果模板生成 `metadata`，那么应当列出 `data_keys`。

要为您的模板编写说明文档，请在模板定义之前插入注释块以指定您的公共契约（public contract）。

<!-- 
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
 -->
```gn
declare_args() {
  # 创建磁盘镜像时分配的字节数。
  disk_image_size_bytes = 1024
}

# 定义一个磁盘镜像文件。
#
# 镜像磁盘文件用于启动虚拟机 bar。
#
# 示例：
# ```
# disk_image("my_image") {
#   sources = [ "boot.img", "kernel.img" ]
#   sdk = false
# }
# ```
#
# 参数
#
#  sources（必需）
#    包含在该镜像中源文件的列表。
#    类型：list(path)
#
#  sdk（可选）
#    该镜像导出至SDK。
#    类型：bool
#    默认：false
#
#  data_deps
#  deps
#  public_deps
#  testonly
#  visibility
#
# 元数据
#
#  files
#    该镜像中显示的文件名。
template("disk_image") {
  ...
}
```

<!-- 
### Wrap tools with a single action template {#wrap-tools-with-a-single-action-template}
 -->
### 使用单一 action 模板包装工具 {#wrap-tools-with-a-single-action-template}

<!-- 
For every tool, have a canonical template that wraps it with an `action`.
This template’s job is to turn GN parameters into `args` for the tool, and
that’s it. This sets an encapsulation boundary around the tool for details
such as translating parameters to args.

Note that in this example we define the `executable()` in one file and the
`template()` in another, because
[templates and targets should be separated](#define-templates-in-gni-targets-in-build-gn).
 -->
每个工具都有一个规范模板，通过一个 `action` 就可以将其打包。该模板的工作仅仅是将 GN 参数转化为对应工具的 `args`。这在工具上对于细节设置了一个封装界限，例如将参数转换为args。

请注意，在下面的示例中我们在一个文件中定义了 `executable()`，在另一个文件中定义了 `template`，因为[模板和目标应当分离](#define-templates-in-gni-targets-in-build-gn)。

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


<!-- 
### Consider making templates private {#consider-making-templates-private}
 -->
### 考虑将模板设为私有 {#consider-making-templates-private}

<!-- 
Templates and variables whose name begins with an underscore (e.g. `template("_private")`)
are considered private and won’t be visible to other files that `import()` them, but can be
used in the same file that they’re defined. This is useful for internal helper templates or
“local global variables” that you might define for instance to share logic between two templates,
where the helper is not useful to the user.
 -->
请考虑将名称以下划线开头的模板和变量（例如：`template("_private")`）设为私有，他们将对 `import()`（导入）它们的其他文件不可见，但可在定义它们的那个文件中使用。这对于内部帮助模板（此处的“帮助”并不是对用户有所帮助）或您（比如为了在两个模板间共享逻辑而）定义的“本地全局变量”而言有所帮助。

<!-- 
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
 -->
```gn
template("coffee") {
  # 例如 coffee 参数有 roast 和 sugar
  ...
  _beverage(target_name) {
    # 以 beverage 的术语来说，如 ingredients 和 temperature
    ...
  }
}

template("tea") {
  # 例如 tea 参数有 loose leaf 和 cream
  ...
  _beverage(target_name) {
    # 以 beverage 的术语来说，如 ingredients 和 temperature
    ...
  }
}

# 我们不想让人直接定义新的 beverage。
# 比如他们可能向 ingredient 列表中既加入 sugar 又加入 salt。
template("_beverage") {
  ...
}
```

<!-- 
Sometimes you can’t make a template private because it actually needs to be used
from different files, but you’d still like to hide it because it’s not meant to
be used directly. In situations like this you can swap enforcement for signaling, by
putting your template in a file under a path such as `//build/internal/`.
 -->
有时您可能无法将模板设为私有，因为它确实需要在其他文件中用到，但您仍希望将其隐藏，因为使用并不意味着是直接的。在这样的情况下，您可以更换方案，通过将您的模板放入形如路径 `//build/internal/` 下的文件内的方式达意。

<!-- 
### Test your templates {#test-your-templates}
 -->
### 测试您的模板 {#test-your-templates}

<!-- 
Write tests that use your templates to build, or use files generated by your
templates in the course of the test.

You should not rely on other people’s builds and tests to test your template.
Having your own tests makes your template more maintainable, since it’s faster
to validate future changes to your template and it’s easier to isolate faults.
 -->
请编写需要使用您模板进行构建的测试，或者使用在测试过程中由您模板生成的文件。

您不应当依赖他人的构建和测试来测试您的模板。拥有您自己的测试能使您的模板更容易维护，因为更新对您自己的模板进行的后续改动会变得更快，并且查错会变得更加容易。

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

<!-- 
## Parameters {#parameters}
 -->
## 参数 {#parameters}

<!-- 
### Assert on required parameters {#assert-on-required-parameters}
 -->
### 对必需的参数 `assert` {#assert-on-required-parameters}

<!-- 
If you have required parameters in your template, `assert` that they’re defined.

If a user forgets to specify a required parameter, and there’s no assert defined,
they won’t get a clear explanation for their error. Using an assert allows you to
provide a useful error message.
 -->
如果您的模板中有必需参数，对他们被定义一事进行 `assert`（强调说明）。

如果用户忘记指定一个必需的参数，而又没有已定义的“assert”，那么用户将无法得到清楚的错误解释。

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

<!-- 
### Always forward `testonly` {#always-forward-testonly}
 -->
### 总是传递 `testonly` {#always-forward-testonly}

<!-- 
Setting `testonly` on a target guards it against being used by non-test targets.
If your template doesn’t forward `testonly` to inner targets then:

1. Your inner targets might fail to build, because your users might pass you `testonly` dependencies.
2. You’ll surprise your users when they find that their `testonly` artifacts end up in production artifacts.

The following example shows how to forward `testonly`:
 -->
在目标上设置 `testonly` 以防它被非测试目标使用。

如果您的模板没有向内部目标传递 `testonly`，那么：

1. 您的内部目标有可能构建失败，因为您的用户可能向您传送 `testonly` 依赖。
2. 您将使您的用户发现他们的测试级产品最终变成了生产级产品。

下例示范了如何传递`testonly`：

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

<!-- 
Note that if the parent scope for the inner action defines `testonly`
then `forward_variables_from(invoker, "*")` won’t forward it, as it
avoids clobbering variables. Here are some patterns to work around this:
 -->
请注意，如果内部操作的父域定义了 `testonly`，那么 `forward_variables_from(invoker, "*")` 为避免破坏变量将不会传递它。以下是一些解决方式：

<!-- 
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
 -->
```gn
# 损坏，不传递 `testonly`
template("my_template") {
  testonly = ...
  action(target_name) {
    forward_variables_from(invoker, "*")
    ...
  }
}

# 有效
template("my_template") {
  testonly = ...
  action(target_name) {
    forward_variables_from(invoker, "*")
    testonly = testonly
    ...
  }
}

# 有效
template("my_template") {
  testonly = ...
  action(target_name) {
    forward_variables_from(invoker, "*", [ "testonly" ])
    forward_variables_from(invoker, [ "testonly" ])
    ...
  }
}
```

<!-- 
The one exception to this are templates that hard-code `testonly = true` because
they should never be used in production targets. For example:
 -->
这里的一个例外情况是硬编码 `testonly = true` 的模板，因为它们从不应当在生产级目标中使用。例如：

```gn
template("a_test_template") {
  testonly = true
  ...
}
```

<!-- 
### Forward `visibility` to the main target and hide inner targets {#forward-visibility-to-the-main-target-hide-inner-targets}
 -->
### 向主要目标传递 `visibility` 并隐藏内部目标 {#forward-visibility-to-the-main-target-hide-inner-targets}

<!-- 
GN users expect to be able to set `visibility` on any target.

This advice is similar to [always forward testonly](#heading=h.fk6w1as9tkpx), except that
it only applies to the main target (the target named `target_name`). Other targets should
have their `visibility` restricted, so that your users can’t depend on your inner targets
that are not part of your contract.
 -->
GN 用户希望能够对任何目标设置 `visibility`。

这一建议与[总是传递“testonly”](#heading=h.fk6w1as9tkpx)类似，除了前者只应用于主要目标（命名为 `target_name` 的目标）。其他目标应当限制其`visibility`，以使您的用户无法依赖您契约之外的内部目标。

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

<!-- 
### If forwarding `deps`, also forward `public_deps` and `data_deps` {#if-forwarding-deps-also-forward-public_deps-and-data_deps}
 -->
### 如果传递了 `deps`，那么也要传递 `public_deps` 和 `data_deps` {#if-forwarding-deps-also-forward-public_deps-and-data_deps}

<!-- 
All built-in rules that take `deps` take `public_deps` and `data_deps`.
Some built-in rules don’t differentiate between types of deps (e.g. `action()`
treats `deps` and `public_deps` equally). But dependants on your generated
targets might (e.g. an `executable()` that deps on your generated `action()`
treats transitive `deps` and `public_deps` differently).
 -->
所有带有 `deps` 的内置规则也有 `public_deps` 和 `data_deps`。一些内置规则并不区分 deps 的类型（例如：`action()`
将 `deps` 和 `public_deps` 同等对待），而依赖于您生成的目标的则会进行区分（例如：一个依赖于您生成的 `action()` 的 `executable()` 会区别对待传递性的 `deps` 和 `public_deps`）。

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

<!-- 
## Target Names {#target-names}
 -->
## 目标名称 {#target-names}

<!-- 
### Define an inner target named  {#define-an-inner-target-named-target_name}
 -->
### 定义一个名为 `target_name` 的内部目标

<!-- 
Your template should define at least one target that is named `target_name`.
This allows your users to invoke your template with a name, and then use that
name in their deps.
 -->
您的模板应当定义至少一个名为 `target_name` 的目标。这允许您的用户通过一个名称来饮用您的模板，再在他们的依赖项中使用该名称。

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

<!-- 
### `target_name` is a good default for an output name, but offer an override {#target_name-is-a-good-default-for-an-output-name-but-offer-an-override}
 -->
### `target_name` 是良好的输出名称默认值，但也应提供覆盖功能 {#target_name-is-a-good-default-for-an-output-name-but-offer-an-override}

<!-- 
If your template produces a single output then using the target name to select
the output name is good default behavior. However, target names must be unique
in a directory, so your users won’t always be able to use the name that they
want both for the target and the output.

It’s a good best practice to offer users an override:
 -->
如果您的模板生成了单一输出，那么选用目标名称作为输出名称是良好的默认行为。但是，由于同一目录下目标名称必须唯一，因此您的用户并不总是能够将他们想用的名字用在目标和输出两者上。

为用户提供覆盖功能是一个很好的最佳实践：

```gn
template("image") {
  forward_variables_from(invoker, [ "output_name", ... ])
  if (!defined(output_name)) {
    output_name = target_name
  }
  ...
}
```

<!-- 
### Prefix internal target names with `$target_name` {#prefix-internal-target-names-with-$target_name}
 -->
### 为内部目标名称加上 `$target_name` 前缀 {#prefix-internal-target-names-with-$target_name}

<!-- 
GN labels must be unique, or else you’ll get a gen-time error. If everyone on
the same project follows the same naming convention then collisions are less
likely to happen and it becomes easier to associate internal target names
with the targets that created them.
 -->
GN 标签必须唯一，否则您将会收到生成时错误（gen-time error）。如果同一项目中的所有人都遵循了相同的命名约定，那么将减少发生冲突的可能，并且关联目标和其创建的标签这一操作将变得更加容易。

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

<!-- 
### Do not infer output names from target labels {#do-not-infer-output-names-from-target-labels}
 -->
### 请勿从目标标签推断输出名称 {#do-not-infer-output-names-from-target-labels}

<!-- 
It’s tempting to assume a relationship between target names and output names.
For instance, the following example will work:
 -->
猜测目标名称和输出名称间的关系这一做法非常具有诱惑性。比如，下面的示例将正常运作：

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

<!-- 
However this example will product a gen-time error:
 -->
然而下面的示例将产生生成时错误：

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

<!-- 
Here’s one way of fixing this problem:
 -->
下面是修复此问题的一种方法：

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

<!-- 
## GN functions and generation {#gn-functions-and-generation}
 -->
## GN 功能和生成 {#gn-functions-and-generation}

<!-- 
### Only use `read_file()` with source files {#only-use-read_file-with-source-files}
 -->
### 只能在对源文件使用 `read_file()`

<!-- 
`read_file()` occurs during generation and can not be safely used to read from generated
files or build outputs. It can be used to read source files, for example to read
a manifest file or a json file with which to populate build dependencies.
Notably `read_file()` can not be used with `generated_file()` or `write_file()`.
 -->
`read_file()` 出现在生成过程中，它不能安全地用于读取生成的文件和构建的输出。它可以用于读取源文件，例如导入构建依赖时读取 manifest 文件或 json 文件。注意 `read_file()` 不能与 `generated_file()` 或 `write_file()` 一同使用。

<!-- 
### Prefer `generated_file()` over `write_file()` {#prefer-generated_file-over-write_file}
 -->
### 多用 `generated_file()`，少用 `write_file()` {#prefer-generated_file-over-write_file}

<!-- 
In general, it’s recommended that you use `generated_file()` over `write_file()`.
`generated_file()` provides additional features and addresses some of the challenges
of `write_file()`. For instance, `generated_file()` can be executed in parallel,
while `write_file()` is done serially at gen time.

The structure of both commands is very similar. For instance, you can turn
this instance of `write_file()`:
 -->
一言以蔽之，推荐您使用`generated_file()` 而非 `write_file()`。`generated_file()` 提供了附加特性，并且解决了一些 `write_file()` 的弊端。比如，`generated_file()` 可以并行执行，而 `write_file()` 在生成期间只能按序执行。

两个命令的结构非常相似。例如，您可以将这个 `write_file()` 的示例：

```gn
write_file("my_file", "My file contents")
```

<!-- 
Into this instance of `generated_file()`:
 -->
转换为这个使用 `generated_file()` 的示例：

```gn
generated_file("my_file") {
  outputs = [ "my_file" ]
  contents = "My file contents"
}
```

<!-- 
### Prefer relative paths from `rebase_path()` {#prefer-relative-paths-from-rebase-path}
 -->
### 多用 `rebase_path()` 的相对路径 {#prefer-relative-paths-from-rebase-path}

<!-- 
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
 -->
总是在 `rebase_path()` 中指定一个 `new_base`（新的基准位置），例如 `rebase_path("foo/bar.txt", root_build_dir)`。避免其单参数形式，即 `rebase_path("foo/bar.txt")`。

GN 的 `rebase_path()` 拥有三个参数，其中后两个可选。它的单参数形式返回一个绝对路径，这种做法[不推荐][rebase-path-thread]。在构建模板和目标中应当避免。`new_base` 的值会根据实际情况发生变化，而 `root_build_dir` 则是其常用选项，因为它是构建脚本执行的地方。请在 `rebase_path()` 的 [GN 参考手册][gn-reference-rebase-path]中参阅更多信息。

<!-- 
Relative paths can stay unchanged when paths to project or build output
directory changes. It has a few advantages over absolute paths:

*   Protects user privacy by not leaking potentially sensitive information from
    paths in build outputs.
*   Improves efficiency of content-addressed caches.
*   Makes interactions between bots possible, for example, one bot performs an
    action following another bot.

See also:
[`rebase_path(x)` returning absolute paths considered harmful?][rebase-path-thread]
 -->
相对路径可以在项目路径或构建输出目录发生改变时保持不变。相较于绝对路径，相对路径有几点优势：

*   不通过构建输出路径泄露潜在敏感信息，保护用户隐私。
*   提升内容定址缓存（content-addressed caches）的效率。
*   使得 bot 间的交互成为可能，例如，一个 bot 跟随另一 bot 的操作运行。

参阅：
[`rebase_path(x)` 返回绝对路径，被认定是有害的？][rebase-path-thread]

<!-- 
## Patterns and anti-patterns {#patterns-and-anti-patterns}
 -->
## 模式与反面模式 {#patterns-and-anti-patterns}

<!-- 
### Target outputs {#target-outputs}
 -->
### 标签输出 {#target-outputs}

<!-- 
When working with `get_target_outputs()` to extract a single element, GN won’t
let you subscript a list before assignment. To work around this issue,
you can use the less than elegant workaround below:
 -->
在使用 `get_target_outputs()` 提取单一元素时，GN 不会允许您对未分配的列表进行下标操作。要解决此问题，您可以使用下面这种不怎么优雅的方法：

<!-- 
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
 -->
```gn
# 向列表尾插元素很优雅
deps += get_target_outputs(":some_target")

# 提取单一元素以在变量代换中使用——丑陋但是可靠
_outputs = get_target_outputs(":other_target")
output = _outputs[0]
message = "My favorite output is $output"

# 该表达式是无效的：`output = get_target_outputs(":other_target")[0]`
# GN 不会允许您对右值进行下标操作。
```

<!-- 
### Set operations {#set-operations}
 -->
### 设置操作 {#set-operations}

<!-- 
GN offers lists and scopes as aggregate data types, but not associative
types like maps or sets. Sometimes lists are used instead of sets. The
example below has a list of build variants, and checks if one of them
is the “profile” variant:
 -->
GN 提供的聚合数据类型为列表（list）和 域（scope），但不提供诸如地图（map）和集合（set）这样的关联类型。有时列表被用来代替集合。下面的示例含有一个构建变量的列表，并检查其中之一是否是“profile”变量：

```gn
if (variants + [ "profile" ] - [ "profile" ] != variants) {
  # Do something special for profile builds
  ...
}
```

<!-- 
This is an anti-pattern. Rather, variants could be defined as follows:
 -->
这是一种反面模式（anti-pattern，意近“反面教材”）。相反地，变量可以按照如下方式定义：

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

<!-- 
### Forwarding `"*"` {#forwarding-*}
 -->
### 传递 `"*"` {#forwarding-*}

<!-- 
`forward_variables_from()` copies specified variables to the current
scope from the given scope _or any enclosing scope_. Unless you
specify `"*"`, in which case it will only directly copy variables
from the given scope. And it will never clobber a variable that’s
already in your scope - that’s a gen-time error.

Sometimes you want to copy everything from the invoker, except for
a particular variable that you want to copy from any enclosing
scope. You’ll encounter this pattern:
 -->
`forward_variables_from()` 将从给定域_或任何外封闭域_中将指定的变量复制到当前域下。除非指定 `"*"`——这种情况下它将仅从给定域下复制变量。并且它绝不会替换您域中已经存在的变量——那是一个生成时错误。

有时您希望从主调函数复制一切，除了某个你想从任何外封闭域中复制的特定变量。您将会用到这样的模式：

```gn
forward_variables_from(invoker, "*", [ "visibility" ])
forward_variables_from(invoker, [ "visibility" ])
```

### `exec_script()` {#exec-script}

<!-- 
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
 -->
GN 的内置函数 [exec_script](https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#func_exec_script) 是增强 GN 能力的有力工具。与 `action()` 相同的是，`exec_script()` 可以调用外部工具。与 `action()` 不同的是，`exec_script()` 与构建生成**同步地**调用外部工具，这意味着您能够在您的 `BUILD.gn` 逻辑中使用该工具的输出。

由于这造成了生成时期的性能瓶颈（即：`fx set` 耗时更长），因此该特性必须小心使用。要获取更多信息，请参阅由 Chromium 团队撰写的[这篇评论](https://chromium.googlesource.com/chromium/src/+/ab1c69b1814d3c905fdab7b0d177b478eecf40a3/.gn#291)。

一份允许列表已被建立在 `//.gn`。请向 `OWNERS` 咨询针对该允许列表所做的改动。

[rebase-path-thread]: https://groups.google.com/a/chromium.org/g/gn-dev/c/WOFiYgcGgjw
[gn-reference-rebase-path]: https://gn.googlesource.com/gn/+/master/docs/reference.md#func_rebase_path
