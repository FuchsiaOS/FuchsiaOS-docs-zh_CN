# 2022 年度 Fuchsia 文档简体中文翻译项目

## 加入我们

***

<p align="center"><a href="https://fuchsia-china.com/2022-document-group-recruitment/">📢</a>&nbsp;<strong>2022 年度翻译小组志愿者持续招募中！详情请查看<a href="https://fuchsia-china.com/2022-document-group-recruitment/">招募公告</a>。</strong>&nbsp;<a href="https://fuchsia-china.com/2022-document-group-recruitment/">🙌</a></p>

***

您还可以通过以下方式获取最新消息：

 - [Fuchsia 中文社区网站](https://fuchsia-china.com)
 - [Fuchsia 中文论坛](https://forum.fuchsia-china.com)
 - TG 群聊：[Fuchsia OS 中文社区](https://t.me/FuchsiaOSzh)、[Fuchsia 开发者社区](https://t.me/FuchsiaDevelopers)
 - QQ 群聊：
   - Fuchsia OS 中文社区：788645873
   - Fuchsia 开发者社区：241234421
 - [微信群聊](https://fuchsia-china.com/join)

![Fuchsia中文文档翻译小组](https://fuchsia-china.com/wp-content/uploads/2022/07/fuchsia-new-trans-outlined.png)

## README

> 英文文档来源：https://fuchsia.googlesource.com/fuchsia/+/main/docs

英文原版 README 请查阅 [README_en.md](README_en.md)，其中部分介绍了文档的大致构成。下面是本项目中大致的文件构成。

 - [行为准则](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/CODE_OF_CONDUCT.md)（英文）
 - [术语表](/glossary/README.md) - 常用术语的定义
 - [使用入门](/get-started/README.md) - 您要入门 Fuchsia 所需要的一切
 - [开发](/development/README.md) - 针对进行构建、运行和测试 Fuchsia 及运行 Fuchsia 的软件的说明
 - [系统](/concepts/index.md) - 针对 Fuchsia 运作机理的文档
 - [Zircon](/concepts/kernel/README.md) - 针对 Zircon 内核的文档
 - [运行示例组件](/development/run/run-examples.md) - 针对在设备上运行示例的说明
 - [贡献修改](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/CONTRIBUTING.md)（英文）

本仓库内的其他文件是 Fuchsia 的**系统级**文档。**个体子项目**在其项目仓库内拥有各自的文档。上述文档链接至系统级仓库内和个体项目仓库内的个体文档。

## 翻译与校对概述

除原文档的概述内容外，请在进行翻译或校对时，务必参考[翻译术语表](glossary_translation.md)，并在对翻译方式有所犹豫时及时查阅 Google 公司现有的帮助文档，进行借鉴。

原文档中同时使用了 markdown 和 html 标记，在翻译时需要将英文文段使用 `<!-- -->` 注释掉，具体格式如下：

原文：

```markdown
Most users control the build through [fx](/development/build/fx.md).
The following documents provide details of build configuration and internal
structure.
```

译文（**请注意将注释标记单独成行**，以便 GitHub 给出可读性更强的文件差异）：

```markdown
<!-- 
Most users control the build through [fx](/development/build/fx.md).
The following documents provide details of build configuration and internal
structure.
 -->
大多用户通过 [fx](/development/build/fx.md) 来控制构建。下面的文档提供了构建配置和内部结构的细节。
```

更多 markdown 和 html 语法请自行查阅，建议在翻译中针对不懂的位置进行查询即可，不必完全掌握。

在翻译时，应尽可能考虑校对的可读性，即尽量分段翻译，不要对过多段落一次性进行注释和翻译，这样会降低校对可读性和效率，也不利于翻译工作本身的进行。

在翻译时，请不要忘记翻译代码块内的可读英文**注释**部分，不要翻译文档中的超链接内容，即下文中的 `B` 处：

```markdown
[A](B)
[A][B]
```

在翻译时，对于内联代码区的内容，如果本身是代码内容，则不译；如果是具有可读性的非代码内容，则可以考虑进行翻译，并在代码区后使用括号标注原文。

在翻译时，要求英文和中文非标点、数字和中文非标点间用 1 个空格分割；英文或数字与中文标点见不空格。可以考虑使用[该软件](https://pypi.org/project/zhlint/)或类似程序进行自动化排版。另外，在非代码区域，应当尽量使用中文标点符号。

## PR 提交要求

 1. 多个翻译请**分文件提交 PR**，即新建分支（branch），在每个分支中只更改一个文件并提交；
 1. PR 提交时请在**标题区域包含文件名**，以便进行记录；
 1. PR 的标题和内容最好使用中文；
 1. 为避免做重复性工作，请在翻译时**先试译若干节**，并将 PR **[以草稿形式提交](https://docs.github.com/cn/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests#draft-pull-requests)**（[创建草稿 PR](https://docs.github.com/cn/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request-from-a-fork) 或[提交 PR 后进入并在右侧选择转换为草稿](https://docs.github.com/cn/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/changing-the-stage-of-a-pull-request#converting-a-pull-request-to-a-draft)），以便校对人员提前开始校对，及时修正当前翻译中存在的可能会在下文中重复出现的格式或内容问题。
 
