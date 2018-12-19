<!--
# Editors
-->
# 编辑器

<!--
Several editors have support for FIDL.
-->
一些编辑器已经支持 FIDL。

<!--
## Atom
-->
## Atom

<!--
To install
-->
安装

    pushd ~/.atom/packages
    git clone https://fuchsia.googlesource.com/atom-language-fidl
    popd

<!--
## IntelliJ / Android Studio
-->
## IntelliJ / Android Studio

<!--
There is an IntelliJ plugin available for FIDL.  It adds syntax and parsing
support.  To install it, choose **Settings > Plugins**, then click **Browse
Repositories** and search for **FIDL**.
-->
在 IntelliJ 中，有一个可用于 FIDL 的插件。它增加了对语法和解析的支持。要安装它，请选择 **Settings > Plugins**，然后点击 **Browse Repositories** 并搜索 **FIDL**。

<!--
## Sublime Text
-->
## Sublime Text

<!--
[Sublime syntax highlighting support can be found here](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/tools/sublime).
-->
[可以在此处找到 Sublime 对语法高亮显示的支持](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/tools/sublime)。

<--
To install, go to **Sublime Text > Preferences > Browse Packages** and copy the
files ``FIDL.sublime-syntax``, and ``Comments (FIDL).tmPreferences`` into the
**User** package.
-->

要安装它，请转到 **Sublime Text > Preferences > Browse Packages** 并将文件 ``FIDL.sublime-syntax`` 和 ``Comments(FIDL).tmPreferences`` 复制到 **User** 包中。

<!--
## Vim
-->
## Vim

[可以在此处找到 Vim 对语法高亮显示的支持和说明](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/tools/vim)。

<!--
## Visual Studio Code
-->

<!--
There is a (Visual Studio Code extension available) at
<https://marketplace.visualstudio.com/items?itemName=fuchsia-authors.language-fidl>.
-->

请参阅（Visual Studio Code可用的扩展）<https://marketplace.visualstudio.com/items?itemName=fuchsia-authors.language-fidl>.

<!--
## Contributing
-->
## 贡献

<!--
Contributions to the various plugins are welcome. Their respective code lives at
-->
欢迎大家对各种插件做出贡献。查看源代码请参阅以下链接。

* **Atom**: <https://fuchsia.googlesource.com/atom-language-fidl/>
* **IntelliJ**: <https://fuchsia.googlesource.com/intellij-language-fidl/>
* **Sublime**: <https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/tools/sublime>
* **vim**: <https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/tools/vim>
* **Visual Studio Code**: <https://fuchsia.googlesource.com/vscode-language-fidl/>

