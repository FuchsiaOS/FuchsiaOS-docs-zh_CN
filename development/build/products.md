## 组件

**组件(Products)** 在 如下位置的 JSON 文件中定义：

* Garnet 层组件：[`//garnet/products/`][garnet-products-source].
* Peridot 层组件：[`//peridot/products/`][peridot-products-source].
* Topaz 层组件：[`//topaz/products/`][topaz-products-source].

组件是一个基于 GN 构建的 Fuchsia 特有的功能，用于帮助定制 Fuchsia 构建过程。组件引用包并粗略定义了将包添加到哪一个构建工件（Artifacts）。

## 包集合

一个组件可以包含一个或多个包，分置与三个不同的构建工件，如下所定义。这些包集合将影响构建输出的不同部分中将包含的包。

### monolith

**组件** 中的 `monolith` 节定义了将包含在硬盘镜像、系统更新镜像以及包仓库（package repository）中的 [build 包](packages.md)。`monolith` 中包含的包将比其他包集合中的包拥有更高的优先级。

### preinstall

**组件** 中的 `preinstall` 节定义了将在硬盘镜像中预装的 [build 包](packages.md) ，这些包也将可以从包仓库中访问。这些包将不会被添加到系统更新镜像（或更新包）中。

### available

**组件** 中的 `available` 节定义了将只添加至包仓库的 [build 包](packages.md) 。这些包将可以进行运行时安装，但不会被预装在硬盘镜像中，也不会包含在系统更新镜像中。`monolith` 与 `preinstall` 包集合内的成员都拥有 `available` 包的特性。

## 默认配置 & 约定俗成

### 默认组件配置

一个层级的 `default` 组件配置，按照约定存储在 `//<层级名>/products/default`，其中包括：

* `monolith` - 该层的最小可用基础，并支持系统更新。
* `preinstall` - 该层常用开发工具的集合和其他常用的项目。
* `available` - 该层的所有 `prod` 包。

根据约定，更高层的 `default` 组件配置应该从低一层的配置增量形成。

## 查看组件配置

由于组件依赖于 [包](packages.md) 而包又依赖于其他的包，查看构成组件的每个包集合中的构建标签（的拓展和过滤集合）是很必要的。[预处理组件][preprocess-products-py] 脚本就是这样一个工具，可以如下手动运行：

```bash
$ python build/gn/preprocess_products.py --products '["garnet/products/default"]'
```

[garnet-products-source]: https://fuchsia.googlesource.com/garnet/+/master/products/
[peridot-products-source]: https://fuchsia.googlesource.com/peridot/+/master/products/
[topaz-products-source]: https://fuchsia.googlesource.com/topaz/+/master/products/
[preprocess-products-py]: https://fuchsia.googlesource.com/build/+/master/gn/preprocess_products.py