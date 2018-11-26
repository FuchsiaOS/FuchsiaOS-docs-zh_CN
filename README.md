# Fuchsia OS 中文社区简体中文文档

原始英文文档 fork 自 https://fuchsia.googlesource.com/docs/

 - [贡献者名单](CREDITS.md)
 - [术语表](DICT.md)
 - [开发](development/README.md) - 一份详细的说明文档，介绍如何参与开发、编译、运行、以及测试 Fuchsia OS 和 Fuchsia OS 软件的方法与细节。
 - [系统](the-book/README.md) - 介绍 Fuchsia OS 如何运作的文档
 - [仓库结构](map.md)
 - [Contributing changes](CONTRIBUTING.md)
 
Fuchsia OS 中文社区
 
 - 网站  https://fuchsia-china.com
 - 论坛  https://forum.fuchsia-china.com/
 - 电报  https://t.me/FuchsiaOSzh
 - QQ群：788645873
 - 微信：https://fuchsia-china.com/join

Other files in this repository are **system-wide** documentation articles for
Fuchsia. **Individual subprojects** have their own documentation within each
project repository. The articles above link to Individual documents both within
the system-wide repository and within Individual project repositories.

## 文档校对

### 为什么要进行校对

文档初稿翻译难免会有不太理想的地方，所以我们希望能有更多人志愿参与校对工作，进一步完善中文文档。

### 文档的构成

文档由若干 `md` 和 `html` 文档构成，翻译即是将原始 `md` 和 `html`文件中需要翻译的文字用 tag 注释包起来，然后再拷贝一份进行翻译。原始英文用符号 `<!-- -->` 注释掉，每一段英文，对应一段中文，方便其他译者 review，如下例：

```
<!--
#### Kubernetes is

* **Portable**: public, private, hybrid, multi-cloud
* **Extensible**: modular, pluggable, hookable, composable
-->

#### Kubernetes 具有如下特点:

* **便携性**: 无论公有云、私有云、混合云还是多云架构都全面支持
* **可扩展**: 它是模块化、可插拔、可挂载、可组合的，支持各种形式的扩展
```

### 翻译规范

- 翻译之前，需参考[术语表](DICT.md)以规范翻译一致性。
- 译文中的英文与中文建议用空格分隔,可以考虑找个[自动化中英文格式化 md 的软件](https://pypi.org/project/zhlint/)
- 专业名称不需要翻译，尽可能用原始英文。
- 翻译的中英文间隔不宜过长，尽可能一段英文注释，一段中文翻译，可以前后对应，方便其他译者协助 review。
- 保持原始 `md` 或 `html` 格式不变，例如 **\_Server\_** 翻译成 **\_服务器\_**
- 对于长文章翻译要注意锚点链接不要移除，例如 **\[Server](#Client)** 翻译成 **\[服务器](#Client)** 锚点链接保留，但不翻译。
- `md` 代码块与代码输出内容也不要翻译
- 如果是多人合译的文章，需要同步好翻译进度
