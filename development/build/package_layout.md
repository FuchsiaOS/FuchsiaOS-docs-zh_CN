# 组件 和 包 目录的层次结构

Fuchsia 源代码树的每一个 [层级](/development/source_code/layers.md) 都包括了名为 `packages` 的顶层目录，包括了该层级所有的 [Build 包](packages-zh_CN.md)。当前的文档描述了一些所有层级都包含的包。每一个层级会对该层特有的包做额外介绍。

## 目录地图

以下内容中，“pkg” 指 Fuchsia 包 —— Fuchsia 系统的安装单位。

```
//<层级名>/products
    default          # 该层级的 default 构建配置
                     # 根据协议，default 配置预装了开发工具，
                     # 并使所有组件可用。
//<层级名>/packages
    <层级名>          # 所有该层级的生产环境用包
    buildbot         # 所有该层级声明的包； 在 CQ/CI 工程使用。
    default          # 该层的大量日常开发用包
    preinstall       # 该层的日常开发工具
    kitchen_sink     # 该层的所有包
    all              # 该层包大杂烩
    prod/            # 生产环境用包
    tests/           # 正确性测试(目标平台 & 主机)
    tools/           # 不适用于主机的开发工具(目标平台 & 主机)
    benchmarks/      # 性能测试
    examples/        # 功能演示
  * experimental/    # 实验性功能（不建议用于生产环境）
  * config/          # 系统配置文件(e.g. 引导信息)
    sdk/             # SDK 定义
    ...              # 每一层级都会定义额外的包
```

## 跨层级依赖关系

- `<层级名>(N)` 依赖于 `<层级名>(N-1)` 并增加所有 (N) 层的 组件配置。
	- 这定义了单纯的组件构建配置
- `buildbot(N)`依赖于 `<层级名>(N-1)` 并增加了该层级的特性
  + 这定义了适合验证（N）层完整性的构建配置
- `kitchen_sink(N)` 依赖于 `kitchen_sink(N-1)` 并增加了该层级的特性
  + 这定义了一个适合对（N）层及其依赖进行开发工作的构建配置



## 层级内依赖关系

存在于同一个 `packages` 目录中的大多数目录都包含了一个特殊的 `all` 包，它是该目录下所有包的合计。每一个 `all` 包都隶属于一个根的 `root` 包，因此这（构建一个层级中的根 `root` 包）是构建 “层级中所有包” 的一个捷径。

值得注意的是，那些不要求包括在合计中的包，已在上文用 `*` 标记出。



## 不构建一个包

在 Fuchsia 的代码库中，可能有一些包需要（暂时的）不进行构建。比如，如果不希望构建 `<层级名>/<type>/foo` ， 将这个包移动到  `<层级名>/<type>/<disabled>/foo` 并将它从 `<层级名>/<type>/all` 中删除。

值得注意的是，这对于不要求包括在合计中的包并不生效，这些包默认是严格的选择进入性包。



## 验证
[`//scripts/packages/verify_layer`][verify-layer] 工具的作用是验证一个层级的 `packages` 和 `products` 的目录结构是否与当前的文档描述相符。

值得注意的是，这样一个文件夹中，只允许组成包的文件存在，以及一个例外： `README.md`，用于作文档描述。

[verify-layer]: https://fuchsia.googlesource.com/scripts/+/master/packages/README.md