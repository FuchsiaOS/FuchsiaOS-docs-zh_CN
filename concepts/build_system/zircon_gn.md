<!-- 
# GN in Zircon
 -->
# Zircon 中的 GN

<!-- 
This discussion assumes basic familiarity with GN syntax and concepts.
[This introduction to GN](intro.md) can provide that background.
 -->
本文进行的讨论假设您对于 GN 的语法和概念已经基本熟悉。[这篇对于 GN 的介绍](intro.md)可以为您提供上述背景知识。

<!-- 
GN uses a templating structure to abstract many of the build details away from
the end user.  Below are a subset of the templates the Zircon GN defines,
focusing on the ones with which Zircon hackers are most likely to interact.
 -->
GN 使用一种模板结构将构建的细节对于终端用户进行抽象。下面是 Zircon GN 定义的模板的一个子集，它侧重于 Zircon 黑客最可能进行交互的那些模板。

<!-- 
## `//zircon/` prefix
 -->
## `//zircon/` 前缀

<!-- 
As discussed in [the introduction](intro.md), GN uses "source-absolute" paths
that look like `//a/b/c`.  In the Zircon GN files, we **never** use `//`.
Instead, use `//zircon/foo` to refer to `//zircon/foo`,
e.g. `"//zircon/system/ulib/zircon"`.
 -->
同在[介绍](intro.md)中讨论的一样，GN 使用“绝对于源的”路径，它们看起来形如 `//a/b/c`。在 Zircon GN 文件中，我们**从不**使用 `//`。作为替代，请使用 `//zircon/foo` 来指代 `//zircon/foo`，例如：`"//zircon/system/ulib/zircon"`。

<!-- 
## `executable()` and `test()`
 -->
## `executable()` 和 `test()`

<!-- 
The primary target type in producing a binary is `executable()`.  This produces
an executable binary from the listed sources.  The Zircon build also provides a
means to indicate the location in the image wherein that binary should be
installed via the `install_path` variable in the target scope.
`install_path` can be:

 * a string: the path relative to the root of the BOOTFS (with no leading `/`)
 * omitted: use the default path of `bin/<binary_name>`
 * `false`: do not install this file at all
 -->
产生二进制文件的主要目标格式是 `executable()`（可执行文件）。这会从列出的源生成一个可执行二进制文件。Zircon 构建也提供了一种方法来指示镜像中的位置，在此二进制文件应当通过目标域中的 `install_path` 变量安装。`install_path` 可以是：

* 字符串：与 BOOTFS 根目录有关的路径（无开头的 `/`）
* 忽略项：使用默认路径 `bin/<binary_name>`
* `false`：完全不安装该文件

<!-- 
The build also provides a `test()` target, which is identical to
`executable()` except that it sets `testonly = true` and that its default
`install_path` is `test/<binary_name>` instead of `bin/<binary_name>`.

`test()` can be used for a test program that runs on Zircon or for a test
program that runs on the host side.  In fact, the same `test()` target can
serve to build the same test program for both situations with no extra work
required.  (It's just what dependency paths reach that target that will
determine whether it's built for host or for Zircon or for both.)
 -->
构建也提供了一个 `test()` 目标，它与 `executable()` 相同，除了它设置了 `testonly = true`，并且其默认 `install_path` 为 `test/<binary_name>` 而非 `bin/<binary_name>`。

`test()` 可以用于在 Zircon 上，或者在主机端运行的测试程序。事实上，相同的 `test()` 目标可以用于在两种情况下构建相同的测试程序而无需额外工作。（到达该目标的依赖会决定其为主机构建、为 Zircon 构建还是为两者同时构建。）


## `library()`

<!-- 
The `library()` template is for any kind of "library" in the Zircon tradition,
whether for the kernel, Zircon user code, or host-side code.  The basic thing
it means to be a "library" is that there is an `include/` subdirectory of
public header files.  Dependents that list this `library()` target in their
`deps` will automatically get `-I` switches for that `include/` directory.
 -->
`library()` 模板是针对 Zircon 传统中任何种类“库（library）”的，无论是内核、Zircon 用户代码还是主机端代码。“库”的基本含义是有一个公共头文件的 `include/` 子目录。将该 `library()` 对象列入它们 `deps` 内的项目将自动为其 `include/` 目录获得 `-I` 开关。

<!-- 
The default case with the most concise syntax is a static-only userland
library.  Making a library available as a shared library just requires adding
the line `shared = true`.  Likewise, making a library available for host-side
use just requires adding the line `host = true`.  These are in addition to the
default `static = true` that makes the library available for userland static
linking.  For a library that should *never* be statically linked (aside from
host-side or kernel uses), you can override the default with `static = false`.
 -->
语法最为简洁的默认情况是纯静态用户空间库（static-only userland library）。要使库可用作共享库，只需要添加行 `shared = true`。同样地，要使库可用于主机端，只需要添加行 `host = true`。这些都是对默认选项 `static = true` 的补充，它可以使库可用于用户空间静态链接。对于从不应当静态链接的库（除了主机端货内核使用），您可以使用 `static = false` 替换默认值。

<!-- 
For a library in the kernel, set `kernel = true`.  This is the same whether
it's a kernel-only library, or is code shared between kernel and user (and/or
host).  Setting `kernel = true` changes the default to `static = false`, so if
a library can be used either in the kernel or in userland, then you must set
`static = true` explicitly alongside `kernel = true` (unless you set `shared =
true` and want to prohibit static linking of that library in userland).
 -->
对于内核中的库，设置 `kernel = true`。不论是内核专用库还是内核与用户（或/与主机）共享的代码，都应如此。设置 `kernel = true` 会改变默认值为 `static = false`，因此如果要使库能够在内核或用户空间使用，那么除了 `kernel = true` 外，您还必须显式地设置 `static = true`（除非您设置 `shared = true` 并且希望禁止该库在用户空间的静态链接）。

<!-- 
Note: For kernel modules that do not provide an `include/` subdirectory,
use [`source_set()`](#source_set) instead of `library()`.

Here’s an exemplar showing all the essential options.  Most actual targets
will be little more than a `sources` list and a `deps` list.
 -->
注意：对于不提供 `include/` 子目录的内核模块，请使用 [`source_set()`](#source_set) 代替 `library()`。

下面是一个范例，它展示了所有必要的选项。多数实际目标并不会比 `sources` 列表和 `deps` 列表复杂多少。

<!-- 
```gn
library("foo") {
  # Builds "libfoo.a" when static, "libfoo.so" when shared.

  static = true  # default, omitted unless kernel = true: build userland libfoo.a
  shared = true  # false if omitted: build userland libfoo.so
  kernel = true  # false if omitted: can be used from kernel
  host = true  # false if omitted: can be used in host tools

  sources = [
    "foo.c",
    "bar.cpp",
  ]

  deps = [
    # Can refer here to `source_set()` or other `library()` targets defined
    # locally.
    ":foo_minimal",  # Defined in this same BUILD.gn file.
    "foobar_subsystem",  # Defined in foobar_subsystem/BUILD.gn relative to here.

    # Explicitly link in static libbar.a even if libbar.so is available.
    "//zircon/system/ulib/bar:static",

    # Be explicit about getting libbaz.so as a shared library.
    "//zircon/system/ulib/baz:shared",

    # Compile with -Isystem/ulib/bozo/include, but don't link anything in.
    # This should usually not be used in `deps`, but only in `public_deps`.
    # See below.
    "//zircon/system/ulib/bozo:headers",

    # Let system/ulib/quux/BUILD.gn decide whether static or shared is the
    # norm for that library.  (So far the defining `library()` will always
    # prefer the shared library if it's enabled; it would be easy to add the
    # option to build shared but default to static if that's ever useful.)
    "//zircon/system/ulib/quux",

    # `library("quextras")` appears in system/ulib/quux/BUILD.gn because quux
    # and quextras want to share some private source code or for whatever
    # reason we've decided putting them in a single directory is right.
    # Because we're not using the target with the name of its directory,
    # the `:name` syntax selects the specific target within that BUILD.gn file.
    # For the derived target names, we use `.` before the suffix.
    # In fact, "quux:headers" is just an alias for "quux:quux.headers", etc.
    "//zircon/system/ulib/quux:quextras",
    "//zircon/system/ulib/quux:quextras_more.static",
    "//zircon/system/ulib/quux:quextras_way_more.shared",

    # This is a `library()` that will set `static=false shared=true`
    # so `zircon:static` here wouldn't work but `zircon:shared` would work.
    "//zircon/system/ulib/zircon",
  ]

  # Per-module compilation flags are always optional.
  # *Note*: For cases where the flag order matters, it may be necessary
  # to use a config() instead.
  cflags = [ "-Wfoo", "-fbar" ]
  cflags_cc = [ "-fonly-for-c++" ]
  cflags_c = [ "-fonly-for-c" ]
  asmflags = [ "-Wa,--some-as-switch" ]
  ldflags = [ "-Wl,--only-affects-shlib-link" ]
}
```
 -->
```gn
library("foo") {
  # 静态时构建“libfoo.a”，共享时构建“libfoo.so”。

  static = true  # 默认，除非 kernel = true，否则忽略：构建用户空间 libfoo.a
  shared = true  # 如果忽略则为 false：构建用户空间 libfoo.so
  kernel = true  # 如果忽略则为 false：能从内核使用
  host = true  # 如果忽略则为 false：能在主机工具中使用

  sources = [
    "foo.c",
    "bar.cpp",
  ]

  deps = [
    # 可以将此指向本地定义的 `source_set()` 或其他 `library()` 目标。
    ":foo_minimal",  # 在相同的 BUILD.gn 文件中定义。
    "foobar_subsystem",  # 相对于此处，在 foobar_subsystem/BUILD.gn 中定义。

    # 显式地链入 libbar.a，即使 libbar.so 可用。
    "//zircon/system/ulib/bar:static",

    # 显式地将 libbaz.so 作为共享库。
    "//zircon/system/ulib/baz:shared",

    # 带 -Isystem/ulib/bozo/include 编译，但不链入任何内容。
    # 这不应当常用在 `deps` 中，而是仅用在 `public_deps` 中。
    "//zircon/system/ulib/bozo:headers",

    # 让 system/ulib/quux/BUILD.gn 决定该库的基准是静态的还是共享的。
    # （目前为止，如果共享库启用了，那么定义 `library()` 将总是偏向于共享库；
    # 默认为静态，但是如果有用，也可以很容易地添加构建共享选项。）
    "//zircon/system/ulib/quux",

    # `library("quextras")` 出现在 system/ulib/quux/BUILD.gn，因为 quux 和 quextras
    # 希望共享一些私有代码，或者出于任何原因，我们断定将它们放入单独的目录是正确的做法。
    # 由于我们不使用具有其目录名称的目标，因此 `:name` 语法在 BUILD.gn 文件中仅选择特定的目标。
    # 对于派生目标名称，我们在前缀前使用 `.`。
    # 实际上，“quux:headers” 仅是“quux:quux.headers”等的一个别名。
    "//zircon/system/ulib/quux:quextras",
    "//zircon/system/ulib/quux:quextras_more.static",
    "//zircon/system/ulib/quux:quextras_way_more.shared",

    # 这是一个进行 `static=false shared=true` 设置的 `library()`，因此这里的 `zircon:static`
    # 不会起作用，而 `zircon:shared` 会起作用。
    "//zircon/system/ulib/zircon",
  ]

  # 各模块编译标志（flag）总是可选的。
  # *注意*：对于标志顺序有关的情况，有必要使用 config() 代替。
  cflags = [ "-Wfoo", "-fbar" ]
  cflags_cc = [ "-fonly-for-c++" ]
  cflags_c = [ "-fonly-for-c" ]
  asmflags = [ "-Wa,--some-as-switch" ]
  ldflags = [ "-Wl,--only-affects-shlib-link" ]
}
```

<!-- 
A heavily abridged real-world example of a kernel module:
 -->
下面是一个高度删节的内核模块实际用例：

```gn
# deps = [ "//zircon/kernel/object" ] gets -Ikernel/object/include
library("object") {
  kernel = true
  sources = [
    "buffer_chain.cpp",
    "process_dispatcher.cpp",
  ]
  deps = [
    "//zircon/kernel/dev/interrupt",
    "//zircon/system/ulib/fbl",
  ]
}
```

<!-- 
Note `system/ulib/fbl` is not `kernel/lib/fbl`: the one `fbl` serves
all. Here's a heavily abridged example for that case:
 -->
注意 `system/ulib/fbl` 不是 `kernel/lib/fbl`：这一 `fbl` 适用于全部。下面是一个针对该情况的高度删节的用例：

```gn
library("fbl") {
  kernel = true
  static = true
  sources = [
    "alloc_checker.cpp",
  ]
  if (is_kernel) {
    sources += [
      "arena.cpp",
      "arena_tests.cpp",
    ]
  } else {
    sources += [ "string.cpp" ]
  }
}
```

<!-- 
The actual `fbl` is a bad example because it has other complications, but this
demonstrates how a library of shared code can be maintained in one place with
one `BUILD.gn` file using one library target to describe both the kernel and
userland incarnations.  They share everything, but can differ as needed based
on `is_kernel` conditionals.
 -->
实际的 `fbl` 是一个糟糕的示例，因为它还存在其他问题，但是这说明了共享代码的库怎样才能在使用一个库目标来描述内核和用户空间化身（incarnation）的情况下通过一个 `BUILD.gn` 文件获得维护。它们共享一切，但可以根据 `is_kernel` 条件按需要产生区别。

<!-- 
Libraries define a standard set of targets (if relevant):

 * `$target_name.headers`
   is always provided, for just getting the headers and not linking it in
 * `$target_name.static`
   is provided if `static = true` (the default)
 * `$target_name.shared`
   is provided if `shared = true`
 -->
库定义目标的一个标准集合（如果相关）：

 * `$target_name.headers` 总是提供的，仅是为了获取头部而不将其链入
 * `$target_name.static` 是提供的，如果 `static = true`（默认值）
 * `$target_name.shared` 是提供的，如果 `shared = true`

<!-- 
If the library is the main target in the file (e.g. `//zircon/foo:foo`)--the common
case--the `static`, `shared`, and `headers` sub-targets are aliased into
`//zircon/foo:static`, `//zircon/foo:shared`, and `//zircon/foo:headers`.
 -->
如果库是文件中的主目标（例如：`//zircon/foo:foo`）（也是通常情况），那么 `static`、`shared` 和 `headers` 子目标也有别名 `//zircon/foo:static`、`//zircon/foo:shared` 和 `//zircon/foo:headers`。

<!-- 
### `public_deps` for header dependencies
 -->
### 针对头部依赖的 `public_deps`

<!-- 
In addition to `deps` and `data_deps`, GN also has `public_deps`. This is used
when a target exposes a dependency in its public header files and needs to
forward that dependency's settings up the dependency chain. Every use of
`public_deps` should have a comment explaining why it's needed:

For example, `library("async-loop")` contains this:
 -->
作为对 `deps` 和 `data_deps` 的补充，GN 也拥有 `public_deps`。它用在目标将依赖置于其公共头部文件中，并需要将该依赖的设置传递至依赖链中的时候。`public_deps` 的每处使用都应当附上注释，解释其必要性：

例如，`library("async-loop")` 含有下面的内容：

```gn
  public_deps = [
    # <lib/async-loop/loop.h> has #include <lib/async/dispatcher.h>.
    "//zircon/system/ulib/async:headers",
  ]
```

<!-- 
## `source_set()` and `static_library()`
 -->
## `source_set()` 和 `static_library()`

<!-- 
Some code that doesn't have an include directory can just use the
native GN `source_set()` or `static_library()` targets.

A source set (see `gn help source_set`) is a way to create a logical grouping
of files or to scope compilation switches narrowly. The object files will be
linked directly into final binaries without going through any intermediate
libraries. In contrast, the files in a static library are only pulled in
as-needed to resolve symbols.
 -->
一些代码没有包含（include）目录，它们可以直接使用原生 GN `source_set()`（源集合）或 `static_library()`（静态库）目标。

源集合（source set）（参阅 `gn help source_set`）是一种创建文件逻辑分组或缩小编译开关范围（to scope compilation switches narrowly）。目标文件将会直接链入最终的二进制文件，而无需通过任何中间库。相比之下，静态库中的文件只是按需取用以解析符号。

<!-- 
  * Code in the kernel itself should always use `source_set`. Static libraries
    currently interact poorly with inline assembly.

  * A `source_set` *must* be used when creating groups of tests since the
    test harness depends on static initializers while the static library
    linking rules will strip the tests. All kernel code.

  * A `static_library` should be used for a higher-level thing that looks like
    a library or a part of one. Dead code stripping is more efficient, and can
    produce faster links and smaller binaries in cases where some code isn't
    needed.
 -->
  * 内核自身中的代码应当总是使用 `source_set`。当前，静态库和内联汇编的交互很差。

  * 创建测试组时，*必须使用* `source_set`，因为测试工具依赖于静态初始化器，而静态库链接规则会剥除测试（strip the tests）。所有内核代码。

  * `static_library` 应当用于形如库或其一部分的高层次事物中。死代码剥除（dead code stripping）更有效率，并且在一些代码不需要之时能够产生更快的链接和更小的二进制程序。

```gn
source_set("some_code") {
  sources = [
    "this.c",
    "that.cpp",
  ]
}
```


## `loadable_module()`

<!-- 
This is not really used in the Zircon build so far, but could be. A loadable
module is a shared object that's not linked directly but rather loaded
dynamically via `dlopen()` or the like.

`loadable_module()` takes the `install_path` parameter like `executable()`
does.  But it has no default path, so it's like `install_path = false` unless
you supply a path explicitly.

Zircon device drivers are loadable modules, but they have their own special
templates that should be used instead of `loadable_module()`.
 -->
目前为止，它并没有在 Zircon 构建中用到，但尚有可能。可加载模块（loadable module）是一种共享对象，它不直接链接，而是通过 `dlopen()` 之类方式动态加载。

可加载模块对待 `install_path` 参数的方式与 `executable()`（可执行文件）相同。但是它没有默认路径，因此除非您显式地提供路径，否则就等同于 `install_path = false`。

Zircon 设备驱动是可加载模块，但是它们有自己的特殊模板可供使用，而非 `loadable_module()`。

<!-- 
## `driver()` and `test_driver()`
 -->
## `driver()` 和 `test_driver()`

<!-- 
Drivers are loadable modules with some special support and constraints.

 * They get a default `install_path` appropriate for drivers, so they will be
   found by `devmgr`.
 * They implicitly depend on `libdriver` so it shouldn't be listed in `deps`.
 * They implicitly use the static C++ standard library.

`test_driver()` is to `driver()` as `test()` is to `executable()`.
 -->
驱动（driver）是具有一些特殊支持和限制的可加载模块。

 * 它们为驱动将默认 `install_path`（安装路径）处理得当，这样它们就可以被 `devmgr` 发现。
 * 它们隐式地依赖 `libdriver`，因此不应将其列入 `deps`。
 * 它们隐式地使用静态 C++ 标准库。

`test_driver()`（测试驱动）之于 `driver()`（驱动）就如同 `test()`（测试）之于 `executable()`。

```gn
driver("fvm") {
  sources = [
    "fvm.cpp",
  ]
  deps = [
    "//src/lib/storage/fs/cpp",
    "//zircon/system/ulib/ddktl",
    "//zircon/system/ulib/zircon",
  ]
}
```

<!-- 
### `resources()` and `firmware()`
 -->
### `resources()` 和 `firmware()`

<!-- 
A `resource()` target declares some file that might be needed in the BOOTFS
image, but doesn’t directly cause anything to happen in the build.  The style
of the rule is as if it’s a copy from a source file to an output file in the
build; it’s modelled on GN’s native `copy()` rule, and `gn help copy` explains
why its syntax is exactly the way it is.  `outputs` is single-element list
containing a path in the BOOTFS.
 -->
`resource()`（资源）目标声明了某个可能在 BOOTFS 镜像中需要但不会在构建中直接造成后果的文件。规则的样式仿佛它是从源文件到构建输出文件的一份拷贝一样；它以 GN 原生的 `copy()` 规则为模型，而 `gn help copy` 解释了其语法的成因。`outputs` 是单元素列表，其中包含了 BOOTFS 中的路径。

```gn
import("//zircon/public/gn/resource.gni")

resource("tables") {
  sources = [
    "data.tbl",
  ]
  outputs = [
    "data/some_lib/data_v1.tbl",
  ]
}
```
<!-- 
The purpose of `resource()` is to be listed in the `data_deps` of the target
that uses the data:
 -->
`resource()` 得目的是用来列入是用来该数据的目标的 `data_deps` 中：

```gn
library("uses_tables") {
  sources = [
    "read_table.cc",
  ]
  data_deps = [
    ":tables",
  ]
}
```

<!-- 
This can be a `library()`, an `executable()`, a `source_set()`, etc.  Good
practice is to put the `data_deps` in the finest-grained target that holds the
code that uses the file at runtime.  Doing so ensures that the relevant
resource will be available at runtime.

If the resource is generated by the build, then the path in the `sources` list
identifies its location in the build directory, usually using
`$target_out_dir` or `$target_gen_dir`.  In that case, the `resource()` must
also have a `deps` list that includes the target that generates that file.
 -->
这可以是 `library()`、`executable()`、`source_set`等等。好的做法是将 `data_deps` 放入最细粒度的包含在运行时使用该文件代码的目标中。这样做能够确保相关资源在运行时可用。

如果资源由构建生成，那么 `sources` 列表中的路径将标明其于构建目录下的位置，通常会使用 `$target_out_dir` 或 `$target_gen_dir`。这种情况下，`resource()` 也必须拥有包含生成该文件的目标的 `deps` 列表。

<!-- 
The build also allows for a special type of resource that is generated from
the dependency graph.  Using `generated_resource()` creates a resource file
that is intended for use in `data_deps`, as in a normal `resource()`, but
instead of using an existing source file it will generate a file at `gn gen`
time with fixed contents or based on a metadata collection (see `gn help
generated_file` for details).

`firmware()` is a special-case variant of `resource()`, intended for drivers.
It places the resource in `/lib/firmware/$path`, where `$path` is a relative
path to the resource in the `/lib/firmware` root.  This mimics the calling
convention in `devhost`, where a driver calls `load_firmware(...)` on a
relative path.
 -->
构建也允许一种特殊类型的资源，它从依赖图中生成。使用 `generated_resource()` 会创建一个资源文件，它用于 `data_deps` 中，和通常的 `resource()` 一样，但是它不会使用现存源文件，而是在 `gn gen` 时期使用固定内容或基于元数据集（metadata collection）生成一个文件（参阅 `gn help
generated_file` 以获取细节）。

`firmware()` 是 `resource()` 的一种特殊情况变体，它面向驱动。它将资源文件放在 `/lib/firmware/$path`，其中 `$path` 是相对于 `/lib/firmware` 根目录中资源的相对路径。这模仿了 `devhost` 中的调用传统，`devhost` 中的驱动是用相对路径调用 `load_firmware(...)`。

## `fidl_library()`

<!-- 
This template allows the definition of a FIDL library and its associated
bindings.  Declaring a `fidl_library()` target will cause the build to
generate bindings for all supported languages.

Note: To use this template, you must import the `fidl.gni` file scope.
 -->
该模板允许 FIDL 库的定义和与其相关的绑定。声明 `fidl_library()` 目标将导致构建为所有支持的语言生成绑定。

注意：要是用该模板，您必须导入 `fidl.gni` 文件域。

<!-- 
```gn
import("//zircon/public/gn/fidl.gni")

# Defined in //zircon/system/fidl/fuchsia-io/BUILD.gn
fidl_library("fuchsia-io") {
  sources = [
    "io.fidl",
  ]
  public_deps = [
    "//zircon/system/fidl/fuchsia-mem",
  ]
}
```
 -->
```gn
import("//zircon/public/gn/fidl.gni")

# 定义在 //zircon/system/fidl/fuchsia-io/BUILD.gn 中
fidl_library("fuchsia-io") {
  sources = [
    "io.fidl",
  ]
  public_deps = [
    "//zircon/system/fidl/fuchsia-mem",
  ]
}
```

<!-- 
Note the use of [`public_deps`](#public_deps).  When a FIDL library's source
files have `using other_library;` that's equivalent to a C/C++ library using
`#include <other_library/header>` in its public headers.  Since this is very
common for FIDL (and Banjo) libraries, we don't require comments on every case
when it follows this simple pattern.

Depending on which bindings are defined, the above example will generate a set
of targets of the form `//zircon/system/fidl/fuchsia-io:fuchsia-io.<language>`, or,
in the case where the target name is the same as the directory name as above,
`//zircon/system/fidl/fuchsia-io:<language>`.

The common case today is `"//zircon/system/fidl/fuchsia-io:c"`.
 -->
注意 [`public_deps`](#public_deps) 的使用。当 FIDL 库的源文件具有 `using other_library;` 时，就等同于在公共头部使用了 `#include <other_library/header>` 的 C/C++ 库。因为这对于 FIDL（和 Banjo）库非常常见，所以我们不需要在每种类似的简单模式情况下都进行注释。

根据定义的绑定，上述示例将产生形如 `//zircon/system/fidl/fuchsia-io:fuchsia-io.<language>` 的目标集合，或者在目标名称与上述目录名称相同的情况下，则为 `//zircon/system/fidl/fuchsia-io:<language>`。

如今常见的情况是 `"//zircon/system/fidl/fuchsia-io:c"`。

## `banjo_library()`

<!-- 
The definition of Banjo libraries is similar to that of FIDL libraries.  A
`banjo_libary()` target will generate bindings for all supported languages,
though the set of supported languages will be different from that of FIDL.
 -->
Banjo 库定义与 FIDL 库相似。`banjo_library()` 目标将为所有支持的语言产生绑定，尽管其支持语言集与 FIDL 的会有所不同。

```gn
import("//zircon/public/gn/banjo.gni")

banjo_library("ddk-driver") {
  sources = [
    "driver.banjo",
  ]
}
```

<!-- 
Currently, listing the plain target with no `:<language>` suffix in `deps`
gets both the C and C++ bindings.  This will probably change in the near
future to more closely follow the FIDL model: specify exactly which bindings
you depend on.

See above about `public_deps`.  Its use in `banjo_library()` is exactly like
its use in `fidl_library()`.
 -->
目前，在 `deps` 中不使用 `:<language>` 前缀直接列出目标会同时得到 C 和 C++ 绑定。这一情况在近期可能会发生改变，以跟进 FIDL 模型：确切指定您所依赖的绑定。

参阅前文关于 `public_deps` 的内容。其在 `banjo_library()` 中的用法与其在 `fidl_library()` 中的用法完全相同。
