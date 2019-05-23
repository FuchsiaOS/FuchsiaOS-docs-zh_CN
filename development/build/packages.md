### 包

**Build 包** 在以下几处的 JSON 文件中可找到定义：

* Garnet 层包列表： [`//garnet/packages/`][garnet-packages-source].
* Peridot 层包列表： [`//peridot/packages/`][peridot-packages-source].
* Topaz 层包列表： [`//topaz/packages/`][topaz-packages-source].

Build 包是 Fuchsia 基于 GN 构建的特有的功能，能够帮助定制 Fuchsia 构建过程。

一个 Build 包文件包括用（`imports`）添加的对其他可构建包的依赖，和用（`packages`）添加的 Fuchsia 包列表两部分。
```json
{
    "imports": [
        "garnet/packages/prod/network”,
    ],
    "packages": [
        "//garnet/examples/http/wget"
    ]
}
```

Build 包是一个或多个标签的合集，比如引用其他的包，或者引用构建标签。在构建的前期，GN 会 [解析 JSON 格式的包定义文件][preprocess-build-packages-py] 。
（构建标签引用经常包括对 [组件定义][products] 的引用，这些定义会决定 GN 的额外任务。）

**Fuchsia 包**，区别于 Build 包，是 Fuchsia 构建系统的构建配置文件（Artifacts），由在 `packages` 列表中的 GN 构建目标生成。一个 Fuchsia 包 包括一个声明内容的 manifest 文件、零或更多个可执行文件，以及它们的资源文件。

例如，上述 Build 包文件向构建内容添加了一个名为 “wget” 的 Fuchsia 包。这个 Fuchsia 包 包括了一个二进制文件。这个二进制文件由同名构建目标 `//garnet/examples/http/wget/BUILD.gn` 生成：

```py
package("wget") {
  deps = [
    # The path to the dependency that creates the wget binary.
    ":bin"
  ],
  binaries = [{
    name = "wget"
  }]
}
```

其中，包所定义的 `binaries` 域指明，这个包包含了单个名为 “wget” 的二进制文件。但 `binaries` 域并不负责创建这个二进制文件 —— 那是依赖列表的工作。依赖列表在 `deps` 域中定义：

```py
# Executable defines a c++ binary, the label of the executable target will
# be the same as the name of the produced binary file.
executable("bin") {
  output_name = "wget"

  sources = [
    "wget.cc",
  ]

  deps = [
    # This executable also has its own dependencies.
    "//garnet/public/lib/app/cpp",
    "//garnet/public/lib/fidl/cpp",
  ]
}
```

`package` 目标中的 `binaries` 域指示 GN 在你引导你的 Fuchsia OS 镜像时部署二进制文件，这使你能够在 Fuchsia 系统中通过 shell 运行你的二进制文件：

```bash
$ wget google.com
--2018-06-13 17:04:44--  http://google.com/
$
```

[garnet-packages-source]: https://fuchsia.googlesource.com/garnet/+/master/packages/
[peridot-packages-source]: https://fuchsia.googlesource.com/peridot/+/master/packages/
[topaz-packages-source]: https://fuchsia.googlesource.com/topaz/+/master/packages/
[preprocess-build-packages-py]: https://fuchsia.googlesource.com/build/+/master/gn/prepreprocess_build_packages.py
[products]: products.md