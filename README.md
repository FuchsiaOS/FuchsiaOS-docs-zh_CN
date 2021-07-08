# Fuchsia 中文社区简体中文文档

2021 年新版文档翻译进行中, 欢迎 [加入翻译](https://fuchsia-china.com/2021-document-call-for-volunteers/) 或 [加入校稿](https://fuchsia-china.com/fuchsia-chinese-documents-start-publish/)

<!-- 
 - [Code of conduct](/CODE_OF_CONDUCT.md)
 - [Glossary](glossary.md) - definitions of commonly used terms
 - [Getting started](/docs/get-started/README.md) - everything you need to get started with Fuchsia
 - [Development](development/README.md) - instructions for building, running and
   testing Fuchsia and software that runs on Fuchsia
 - [System](/docs/concepts/index.md) - documentation for how Fuchsia works
 - [Zircon](/docs/concepts/kernel/README.md) - documentation for the Zircon kernel
 - [Run an example component](/docs/development/run/run-examples.md) - instructions for running examples
   on a device
 - [Contributing changes](/CONTRIBUTING.md)

Other files in this repository are **system-wide** documentation articles for
Fuchsia. **Individual subprojects** have their own documentation within each
project repository. The articles above link to Individual documents both within
the system-wide repository and within Individual project repositories.
-->

- [行为准则](CODE_OF_CONDUCT.md)
- [术语表](/glossary/README.md) - 定义常用的专业术语
- [入门](/get-started/README.md) - 任何人都可以从这开始入门 Fuchsia
- [开发](development/README.md) - 构建，运行和测试 Fuchsia 以及在 Fuchsia 上运行的软件的说明
- [系统概念](concepts/index.md) - 关于 Fuchsia 是如何工作的
- [Zircon](concepts/kernel/README.md) - 关于 Zircon 内核的说明
 - [运行组件例子](development/run/run-examples.md) - 运行例子在设备的说明
 - [术语表](glossary/README.md) - 翻译术语 和 Fuchsia术语说明
 - [贡献社区](CONTRIBUTING.md)

仓库的其他文件是 **系统范围** 的文档，个别子项目都在子仓库中有各自的文档。上面文档会链接到系统范围和个别子项目的仓库。



## 贡献者名单(2021版)
 - 翻译小组成员: Logincat, Lua, Lumos Maxima, Roy, Salted Fish, Tanging, Vseann, Whyto, Yangxuan
 - 翻译项目协调人: Dongchan, N0B8D1

**感谢所有为 FuchsiaOS 中文社区作出贡献的开发者**

<a href="https://github.com/FuchsiaOS/FuchsiaOS-docs-zh_CN/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=FuchsiaOS/FuchsiaOS-docs-zh_CN" />
</a>


## Fuchsia 中文社区

 - 网站  https://fuchsia-china.com
 - 论坛  https://forum.fuchsia-china.com/
 - 电报  https://t.me/FuchsiaOSzh
 - QQ群：788645873
 - 微信：https://fuchsia-china.com/join

 ## Fuchsia 开发者社区

 - 电报  https://t.me/FuchsiaDevelopers
 - QQ群：241234421
 - 微信：https://fuchsia-china.com/join



## 文档校对

### 为什么要进行校对

文档初稿翻译难免会有不太理想的地方，所以我们希望能有更多人志愿参与校对工作，进一步完善中文文档。

### 文档的构成

文档由若干 `md` 和 `html` 文档构成，翻译即是将原始 `md` 和 `html`文件中需要翻译的文字用 tag 注释包起来，然后再拷贝一份进行翻译。原始英文用符号 `<!-- -->` 注释掉，每一段英文，对应一段中文，方便其他译者 review，如下例：

```
<!-- 
# Fuchsia getting started

Welcome to Fuchsia! This guide walks you through the steps to get Fuchsia source code,
build Fuchsia, and run Fuchsia on an emulator or hardware device. 

-->

# Fuchsia 入门

欢迎来到 Fuchsia 的世界！本文将通过源代码的获取、构建，再到虚拟机或者真机上运行 Fuchsia ，带领你一步一步地探索 Fuchsia 操作系统。
```



## 翻译规范

- 翻译之前，需参考[约定与术语表](./glossary/README.md)以规范翻译一致性。
- 译文中的英文与中文建议用空格分隔,可以考虑找个[自动化中英文格式化 md 的软件](https://pypi.org/project/zhlint/)
- 专业名称不需要翻译，尽可能用原始英文。
- 翻译的中英文间隔不宜过长，尽可能一段英文注释，一段中文翻译，可以前后对应，方便其他译者协助 review。
- 保持原始 `md` 或 `html` 格式不变，例如 **_Server_** 翻译成 **_服务器_**
- 对于长文章翻译要注意锚点链接不要移除，例如 **[Server](#Client)** 翻译成 **[服务器](#Client)** 锚点链接保留，但不翻译。
- `md` 代码块与代码输出内容也不要翻译
- 如果是多人合译的文章，需要同步好翻译进度



## 翻译文档贡献流程（提PR的方式）

1. 在Github上找到 [Fuchsia中文文档](https://github.com/FuchsiaOS/FuchsiaOS-docs-zh_CN)，点击右上角的**Fork**按钮
2. Clone 你自己的 fork 库
3. 当在自己本地的文档更改后，push 到自己的 fork 库。
4. 打开自己的 fork 库，创建 **pull request**

[提PR详情](https://www.dataschool.io/how-to-contribute-on-github/)

