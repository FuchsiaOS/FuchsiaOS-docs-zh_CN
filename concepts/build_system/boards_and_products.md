<!-- 
# Products and Boards
 -->
# 产品和板型

<!-- 
[**Products**][products-source] and [**Boards**][boards-source] are GN
includes used in combination to provide a baseline configuration for a
Fuchsia build.

It is expected that a GN build configuration include exactly one board GNI
file, and one product GNI file. In [fx][fx] this pair is the primary argument
to the `fx set` command.

In a Fuchsia GN build configuration the board is always included first. The
board starts the definition of three dependency lists that are then augmented
by the imported product (and later, optional GN inclusions). Those list are
[Base](#base), [Cache](#cache) and [Universe](#universe) respectively, and
are defined below.
 -->
[**产品**][products-source]（product）和[**板型**][boards-source]（board）是 GN 包含项，二者组合使用以为 Fuchsia 构建提供基线标准。

一份 GN 构建配置最好仅包含一个板型的 GNI 文件和一个产品的 GNI 文件。在 [fx][fx] 中，这个配置对是 `fx set` 命令的主要参数。

在 Fuchsia GN 构建配置中，板型总是最先被包含（include）。板型定义了三个依赖列表，它们可以被之后导入的产品（和再后来可选的 GN 包含项）所扩充。这些列表分别为[Base](#base)、[Cache](#cache) 和 [Universe](#universe)。

<!-- 
## Boards
 -->
## 板型

<!-- 
A board defines the architecture that the build produces for, as well as key
features of the device upon which the build is intended to run. This
configuration includes what drivers are included, and may also influence
device specific kernel parameters.

The available boards can be listed using `fx list-boards`.
 -->
板型（board）定义了构建制作所面向的架构，也定义了构建所计划运行的设备的关键特性。该配置中包括哪些驱动包含在内，并且也可能影响设备特定的（device specific）内核参数。

<!-- 
## Products
 -->
## 产品

<!-- 
A product defines the software configuration that a build will produce. Most
critically, a product typically defines the kinds of user experiences that
are provided for, such as what kind of graphical shell the user might
observe, whether or not multimedia support is included, and so on.

The available products can be listed using `fx list-products`.
 -->
产品（product）定义了构建将会产生的软件配置。至关重要的是，产品通常定义了所提供的用户体验类型，例如用户将查看到哪种图形外壳（graphical shell），多媒体支持是否包含在内，等等。

<!-- 
## Dependency Sets
 -->
## 依赖集合

<!-- 
Boards define, and products augment three lists of dependencies, Base, Cache
and Universe. These dependencies are GN labels that ultimately contribute
packages to various system artifacts, such as disk images and signed package
metadata, as well as various development artifacts such as host tools and
tests.
 -->
板型定义、产品扩充了三个依赖列表：Base、Cache
和 Universe。这些依赖是 GN 标签，它们最终将包（package）提交给各种系统部件（例如磁盘镜像和签名的包元数据），以及各种开发部件（例如主机工具和测试）。

### Base

<!-- 
The `base` dependency list contributes packages to disk images and system
updates as well as the package repository. A package included by the `base`
dependency set takes precedence over a duplicate membership in the `cache`
dependency set. Base packages in a system configuration are considered system
and security critical. They are updated as an atomic unit and are never
evicted at runtime regardless of resource pressure.
 -->
`base` 依赖列表将包提交给磁盘镜像、系统更新和包仓库。包含在 `base` 依赖集合内的包优先于 `cache` 依赖集合中的副本成员。系统配置中的 base 包被认定为对系统和安全性至关重要。它们作为原子单元升级，并且在运行时期，无论资源压力有多大，都绝不会被剔除。

### Cache

<!-- 
The `cache` dependency list contributes packages that are pre-cached in the
disk image artifacts of the build, and will also be made available in the
package repository. These packages are not added to the system updates, but
would instead be updated ephemerally. Cached packages can also be evicted
from running systems in order to free resources based on runtime resource
demands.
 -->
`cache` 依赖列表提交预先缓存在构建的磁盘镜像部件中的包，这些包在包仓库中也会变得可用。这些包没有添加至系统更新，但会在短期内更新。cache 包在系统运行时，会根据资源需要被剔除以释放资源。

### Universe

<!-- 
The `universe` dependency list contributes packages to the package repository
only. These packages will be available for runtime caching and updating, but
are not found in system update images nor are they pre-cached in any disk
images. All members of `base` and `cache` are inherently also members of
`universe`.
 -->
`cache` 依赖列表仅将包提交至包仓库。这些包将在运行时缓存和更新中可用，但在系统更新镜像中无法找到，也不会被预先缓存在任何磁盘镜像中。所有 `base` 成员和 `cache` 成员内在地也都是 `universe` 成员。

<!-- 
## Key Product Configurations
 -->
## 关键产品配置

<!-- 
There are many more than below, but the following three particularly
important configurations to be familiar with:
 -->
实际上的种类远多于下列三个，但是它们尤为重要，需要熟悉：

### Bringup {#bringup-product}

<!-- 
The `bringup` product is the most minimal viable target for development.
Because it lacks most network capabilities, the `bringup` product
cannot use the `fx` commands, such as
<code>[fx serve](/docs/development/build/fx.md#serve-a-build)</code> and
<code>[fx shell](/docs/development/build/fx.md#connect-to-a-target-shell)</code>,
that require network connectivity.
 -->
`bringup` 产品是开发的最小可行目标。因为缺少大多数网络功能，所以 `bringup` 产品无法使用 `fx` 命令，例如 <code>[fx serve](/docs/development/build/fx.md#serve-a-build)</code> 和
<code>[fx shell](/docs/development/build/fx.md#connect-to-a-target-shell)</code>，它们都需要网络连接。

<!-- 
For more see [Bringup Product Definition](/docs/concepts/build_system/bringup.md)
 -->
更多信息请参阅 [Bringup 产品定义](/docs/concepts/build_system/bringup.md)。

### Core {#core-product}

<!-- 
`core` is a minimal feature set that can install additional software (such as
items added to the "universe" dependency set). It is the starting point for
all higher-level product configurations. It has common network capabilities
and can update a system over-the-air.
 -->
`core` 是能够安装附加软件（例如添加至“universe”依赖集合的项目）的最小特性集合。它是所有高级（high-level）产品配置的起点。它拥有常规网络功能，能够空中（over-the-air）更新系统。

### Workstation {#workstation-product}

<!-- 
`workstation` is a basis for a general purpose development environment, good
for working on UI, media and many other high-level features. This is also
the best environment for enthusiasts to play with and explore.
 -->
`workstation` 是综合目的开发环境的基础，其用户界面、媒体和许多其他高级特性有助于工作进行。这也是爱好者进行体验与探索的最佳环境。

[products-source]: /products/
[boards-source]: /boards/
[fx]: /docs/development/build/fx.md
[fx-netboot]: /docs/development/build/fx.md#what-is-netbooting
[fx-paving]: /docs/development/build/fx.md#what-is-paving
[fx-serve]: /docs/development/build/fx.md#serve-a-build
[fx-shell]: /docs/development/build/fx.md#connect-to-a-target-shell

