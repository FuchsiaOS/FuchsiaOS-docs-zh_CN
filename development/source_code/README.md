Fuchsia 源代码
==============

Fuchsia 使用 `jiri` 工具管理 git 仓库
[https://fuchsia.googlesource.com/jiri](https://fuchsia.googlesource.com/jiri).
使用该工具来管理在清单中列出的仓库集合。

怎样构建，请看 Fuchsia 的 [Getting Started](/getting_started.md) 文档。

## 创建一个新的 Checkout

引导程序需要你安装 Go 1.6 （或更新）和 Git 并且能在你的 PATH 中找到。

这个脚本首先会创建一个 `fuchsia` 目录来引导一个开发环境。

```
curl -s "https://fuchsia.googlesource.com/scripts/+/master/bootstrap?format=TEXT" | base64 --decode | bash
```

该脚本会设置你的开发环境追踪 `topaz` 仓库的 HEAD，如果你想要追踪不同仓库的 HEAD，你可以使用 `fx set-petal` 命令改变。


### 设置环境变量
成功以后，引导脚本会打印一条消息提示你把 `.jiri_root/bin` 目录添加到 PATH。这会把 `jiri` 添加到你的 PATH 中，强烈推荐这么做，这样一来 `jiri` 会成为其它 Fuchsia 工具链的一部分。

另一个在 `.jiri_root/bin` 目录的工具是 `fx` ，用来帮助我们配置，构建，运行以及 debug Fuchsia。使用 `fx help` 查看所有可用的命令。

我们同样建议 source `scripte/fx-env.sh` 文件。它定义了一些文档中常用的变量，例如 `$FUCHSIA_DIR`，并且提供了一些有用的 shell 函数，如 `fd` 用来高效改变目录。更多详情，请查看 `scripts/fx-env.sh` 中的注释。

### 在不改变 PATH 的情况下工作

如果你不喜欢破坏你的环境变量，只想 `jiri` 能在当前目录下工作即可，只需要复制 `jiri` 到 PATH 中。然而你必须拥有对复制 `jiri` 到 **目标目录** 的  **写入权限** （无需 `sudo`) ，如果你没有，那么 `jiri` 将不会更新它自己。

```
cp .jiri_root/bin/jiri ~/bin
```

为了使用 `fx` 工具，你可以将它符号链接到 `~/bin` 目录：
```
ln -s `pwd`/scripts/fx ~/bin
```

或者如 `script/fx` 一样带工具目录运行，请确保 **jiri** 在你的 PATH。

## 谁会在代码上工作

在每个仓库的根目录和其它目录会有 MAINTAINERS 文件。这些列出来的都是那些熟悉并且能够为包含它的目录提供代码内容审查的人的邮件地址。查看 [maintainers.md](maintainers.md) 获得更多讨论。

## 如何处理第三方代码
有关编写 README.fuchsia 文件的信息，请查看 [指南](README.fuchsia.md)。

## 故障排除

### 身份认证错误
如果你在检出代码时看到一个错误，警告你 `Invalid authentication credentials`，你可能拥有一个叫 `$HOME/.gitcookies` 且适用于仓库的 cookie 文件，jiri 会尝试匿名检出它（就像域名中的 `.googlesource.com`）。你可以根据屏幕上的指示获得特定仓库的密码，或者你也可以在 `.gitcookies` 文件中删除错误的 cookie 。
