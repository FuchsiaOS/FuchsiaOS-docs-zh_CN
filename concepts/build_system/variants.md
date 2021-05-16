<!-- 
# Fuchsia Build System: Variants
 -->
# Fuchsia 构建系统：变体

<!-- 
The Fuchsia GN build machinery allows for separate components to be built
in different "variants".  A variant usually just means using extra compiler
options, but they can do more than that if you write some more GN code.
The variants defined so far enable things like
[sanitizers](https://github.com/google/sanitizers/wiki) and
[LTO](https://llvm.org/docs/LinkTimeOptimization.html).
 -->
Fuchsia GN 构建体系允许独立组件（component）以不同“变体”（variant）构建。一个变体通常仅意味着使用额外编译器选项，但如果您编写更多的 GN 代码，您会发现它们的功效不止于此。目前为止定义的变体启用了诸如 [sanitizer](https://github.com/google/sanitizers/wiki) 和 [LT0](https://llvm.org/docs/LinkTimeOptimization.html) 的功能。

<!-- 
The GN build argument
[`select_variant`](/docs/gen/build_arguments.md#select_variant)
controls which components are built in which variants.  It applies
automatically to every `executable`, `loadable_module`, or `driver_module`
target in GN files.  It's a flexible mechanism in which you give a list of
matching rules to apply to each target to decide which variant to use (if
any).  To support this flexibility, the value for `select_variant` uses a
detailed GN syntax.  For simple cases, this can just be a list of strings.

Using `fx set`:
 -->
GN 构建参数 [`select_variant`](/docs/gen/build_arguments.md#select_variant) 控制在哪些变量中构建哪些组件。它会自动应用于 GN 文件中的每个 `executable`、`loadable_module` 或 `driver_module` 目标。这是一种灵活的机制，您可以在其中提供一系列匹配的规则来应用于每个目标，以决定要使用哪个变体（若有）。为了支持这种灵活性，select_variant 的值使用了详细的 GN 语法。 对于简单情况，它可以仅是一个字符串列表。

使用`fx set`：

```sh
fx set core.x64 --variant=host_asan --variant=asan/cat --variant=asan/ledger
```

<!-- 
Alternatively, you can add or modify the variants on an existing build by
editing the GN args (substituting your build's GN output directory
for `out/default` as necessary):
 -->
或者，您可以通过编辑 GN 参数对现存构建进行添加或修改（按需将您的构建的 GN 输出目录替换为 `out/default`）。

```sh
gn args out/default
```

<!-- 
That command will bring up an editor. Append to that file:
 -->
该命令将打开编辑器。向文件尾添加：

```
select_variant = [ "host_asan", "asan/cat", "asan/ledger" ]
```

<!-- 
 1. The first switch applies the `host_asan` matching rule, which enables
    [AddressSanitizer](https://clang.llvm.org/docs/AddressSanitizer.html)
    for all the executables built to run on the build host.

 2. The second switch applies the `asan` matching rule, which enables
    AddressSanitizer for executables built to run on the target (i.e. the
    Fuchsia device).  The `/cat` suffix constrains this matching rule only
    to the binary named `cat`.

 3. The third switch is like the second, but matches the binary named `ledger`.
 -->
 1. 第一个开关应用 `host_asan` 匹配规则，该规则对所有为在构建主机上运行而构建的可执行文件启用 [AddressSanitizer](https://clang.llvm.org/docs/AddressSanitizer.html)。

 2. 第二个开关应用 `asan` 匹配规则，该规则对 为在目标设备（即 Fuchsia 设备）上运行而构建的可执行文件启用 AddressSanitizer。`/cat` 后缀将此匹配规则仅限制在名为 `cat` 的二进制文件上。

 3. 第三个开关与第二个相同，但匹配名为 `ledger` 的二进制文件。

<!-- 
The GN code supports much more flexible matching rules than just the binary
name, but there are no shorthands for those. See the
[`select_variant`](/docs/gen/build_arguments.md#select_variant)
build argument documentation for more details.

To see the list of variants available and learn more about how to define
new ones, see the
[`known_variants`](/docs/gen/build_arguments.md#known_variants)
build argument.
 -->
GN 代码不仅支持二进制名称，还支持更加灵活的匹配规则，但是这些规则没有简写形式。参阅 [`select_variant`](/docs/gen/build_arguments.md#select_variant) 构建参数文档以获取更多信息。

要查看可用的变体列表，了解有关如何定义新变体的更多信息，请参阅 [`known_variants`](/docs/gen/build_arguments.md#known_variants) 构建参数。

<!-- 
## Troubleshooting notes
 -->
## 故障排除说明

<!-- 
### Replicating ASan failures
 -->
### 复制 ASan 故障

<!-- 
Our commit queue runs tests in an ASan-enabled configuration. To replicate the
build in this configuration, use the following `args.gn` file:
 -->
我们的记录队列（commit queue）在启用 ASan 的配置中运行测试。要在该配置中复制该构建，请使用下面的 `args.gn` 文件：


```sh
import("//boards/<x64-or-arm64>.gni")
import("//products/core.gni")

base_package_labels+=[ "//bundles/buildbot:core" ]
goma_dir="<path-to-goma-dir>"
is_debug=true
select_variant=["asan","host_asan"]
target_cpu="<x64-or-arm64>"
use_goma=true
```

<!-- 
Replace `x64-or-arm64` with your desired target architecture, and replace
`<path-to-goma-dir>` with the path to your goma dir (for those who use goma). This
can also be generated from the command line with:
 -->
将 `x64-or-arm64` 替换为您所希望的目标架构，并将 `<path-to-goma-dir>` 替换为您的 goma 文件夹（对于使用 goma 的用户）。这也可以通过命令行生成：

```sh
fx set core.x64 --with-base //bundles/buildbot:core --variant host_asan --variant asan --goma
```

<!-- 
Note that this will build all of the tests that are run by the commit queue and
install them in the system image. This may be undesirable for two reasons:

 * Building all of the tests is typically slow and unnecessary. Developers may
   find it more effective to limit the package labels to the tests they need.
 * Installing all of the tests in the system image ahead of time means that the
   software deployment workflow does not get exercised.
 -->
注意，这将会构建由记录队列运行的所有测试，并将它们安装在系统镜像中。出于以下两个原因，这可能是令人讨厌的：

 * 构建所有测试经常又慢又没有必要。开发者可能会发现将包（package）标签限制到他们所需的测试会更加有效。
 * 提前将所有的测试安装在系统镜像中意味着软件部署工作流未实施。

<!-- 
### Launching executables from within ASan-enabled binaries
 -->
### 从启用 ASan 的二进制文件中启动可执行文件

<!-- 
If you are trying to use the ASan variant, you may encounter an error that looks
like this:
 -->
您如果正试图使用 ASan 变体，则可能遇到类似这样的错误：

```sh
launcher: error: Launch: elf_load: handle_interp failed
dlsvc: could not open 'asan/ld.so.1'
```

<!-- 
Fuchsia is structured around packages and components. Each component contains
all of the shared libraries it needs to run. This helps Fuchsia avoid library
versioning issues that plague other operating systems. It also means that, if
you want to run a binary from within a component, you must provide the
appropriate shared library loader for that binary.
 -->
Fuchsia 是围绕包和组件构造的。每一个组件包含了它运行时所需要的所有共享库。这帮助 Fuchsia 规避了困扰了其他操作系统的库版本控制问题。这也意味着，如果您想从一个组件中运行一个二进制文件，您必须为该二进制文件提供正确的共享库加载器。

<!-- 
There are a set of command line programs located in the `/boot/` directory of
Fuchsia installs that are not contained in packages, but in the boot filesystem.
These programs do not have their own shared library loader, and will use
whatever shared libraries the component executing them provides. This normally
works, as programs like `sh` and `ls` have very minimal, very common
dependencies. However, there's no guarantee that the component's package will
have sufficient or compatible shared libraries for the command line program's
needs. ASan-enabled packages usually do not contain the right launcher for these
programs, so most ASan-enabled components cannot run executables out of
`/boot`. If an ASan-enabled component tries to do so, it gets the error above.
 -->
在 Fuchsia 安装目录的 `/boot/` 目录中有一套命令行程序，它们不包含在包中，而是包含在引导文件系统（boot filesystem）中。这些程序没有自己的共享库加载器，而将使用执行它们的组件提供的任何共享库。这通常能够运作，因为诸如 `sh` 和 `ls` 的程序具有非常少、非常常见的依赖关系。但是，不能保证组件的包会有足够的或兼容的共享库以满足命令行程序的需求。启用 ASan 的包通常不包含这些程序的正确启动器，因此大多数启用 ASan 的组件无法在 `/boot` 之外运行可执行文件。启用了 ASan 的组件如果尝试这样做，则会得到上面的错误。

<!-- 
Fortunately, it turns out that the fix involves doing what all packages should
do anyway, which is to declare their dependencies explicitly. If your package
depends on a binary, it should declare it as a dependency, and then use that
declared dependency instead of the one in the `/boot` directory. In the case of
our build system, the `zircon_extras_manifest` rule defined in
`//build/config/fuchsia/zircon_images.gni` will allow you to depend on any of
the binaries found in the `/boot` directory. They will be installed in
`/pkg/bin/`, and you should execute them from there.
 -->
幸运的是，事实证明，其修复涉及的是所有包该做的事情，即显式地声明其依赖。如果您的包依赖于一个二进制文件，则应将其声明为依赖，然后使用这个已经声明了的依赖，而不要使用 `/boot` 目录中的依赖项。在我们构建系统的案例中，在 `//build/config/fuchsia/zircon_images.gni` 中定义的 `zircon_extras_manifest` 规则将允许您依赖在 `/boot` 目录中的任何二进制文件。它们将被安装在
`/pkg/bin/`，您应当从那里执行它们。
