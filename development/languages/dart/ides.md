<!-- # IDEs for Dart in Fuchsia -->

# Fuchsia 中的 Dart IDEs

### Dart SDK

<!-- A prebuilt Dart SDK is available for IDE consumption at:
`prebuilt/third_party/dart/{linux|mac|windows}-x64/bin/dart`. -->

预构建的 Dart SDK 可用于 IDE 使用：`prebuilt/third_party/dart/{linux|mac|windows}-x64/bin/dart`。

## Visual Studio Code

<!-- 1.  Download and install [Visual Studio Code](https://code.visualstudio.com/)
1.  (Optional) Setup VS Code to launch from the command line

    *   For macOS: To allow running VS Code from the terminal using the `code`
        command, see
        [Launching from the command line](https://code.visualstudio.com/docs/setup/mac#_launching-from-the-command-line).

    *   For Linux and Windows: This should already be done as part of the
        installation

1.  Install the following extensions:

    *   [Dart Code](https://marketplace.visualstudio.com/items?itemName=Dart-Code.dart-code):
        Support for programming in Dart. It should automatically find the dart-sdk in the Fuchsia tree.
    *   [FIDL language support](https://marketplace.visualstudio.com/items?itemName=fuchsia-authors.language-fidl):
        Syntax highlighting support for Fuchsia's FIDL files
    *   [GN](https://marketplace.visualstudio.com/items?itemName=npclaudiu.vscode-gn):
        Syntax highlighting for GN build files
    *   Optional but helpful git extensions:
        *   [Git Blame](https://marketplace.visualstudio.com/items?itemName=waderyan.gitblame):
            See git blame information in the status bar
        *   [Git History](https://marketplace.visualstudio.com/items?itemName=donjayamanne.githistory):
            View git log, file history, etc.

1.  To improve your productivity for Dart in VS Code, you can set some useful
    settings.

    To add the settings:

    1. Open your user settings (Ctrl+,)
    1. Click the rotating page icon in the top left (or right for macOS) corner
    1. Add: -->

1.  下载并安装 [Visual Studio Code](https://code.visualstudio.com/)
1.  （可选）配置 VS Code 从命令行启动

    *   对于 macOS：为了允许从命令行运行 VS Code，可以查看
        [从命令行启动](https://code.visualstudio.com/docs/setup/mac#_launching-from-the-command-line)。

    *   对于 Linux 和 Windows：这是安装程序已经做好的一部分了。

1.  安装以下扩展插件:

    *   [Dart Code](https://marketplace.visualstudio.com/items?itemName=Dart-Code.dart-code)：
        支持 Dart 的编程语法，它将会自动的在 Fuchsia 树中查找到 dart-sdk。
    *   [FIDL language support](https://marketplace.visualstudio.com/items?itemName=fuchsia-authors.language-fidl)：
        支持 Fuchsia 的 FIDL 文件的语法高亮。
    *   [GN](https://marketplace.visualstudio.com/items?itemName=npclaudiu.vscode-gn)：
        GN 构建文件的语法高亮
    *   可选但有用的 git 拓展插件：
        *   [Git Blame](https://marketplace.visualstudio.com/items?itemName=waderyan.gitblame):
            在状态栏查看 git 逐行追溯信息
        *   [Git History](https://marketplace.visualstudio.com/items?itemName=donjayamanne.githistory):
            查看 git 日志，文件历史，或者其他

1.  为了提高在 VS Code 中使用 Dart 的效率，您可以设置一些有用的设置。

    增加设置：

    1. 打开你的用户设置 (Ctrl+,)
    2. 点击左上角的导航页图标 (macOS 是右上角)
    3. 添加:

<!-- Note: This configuration file is a JSON file. Make sure that you properly use
curly braces. -->

Note: 配置文件是一个 JSON 文件。请确保你正确的使用了花括号。

<!-- * Auto-format your files when you save: -->

* 当你保存的时候自动格式化你的文件：

```json
"editor.formatOnSave": true,
```

<!-- * Check for new SDK updates for Fuchsia: -->

* 检查新的 Fuchsia SDK 更新：

```json
"dart.checkForSdkUpdates": false,
```

<!-- * Configure VS Code to use the bundled Dart SDK -->

* 配置 VS Code 使用绑定的 Dart SDK：

```json
"dart.sdkPath": "/path/to/fuchsia/prebuilt/third_party/dart/linux-x64/bin/dart",
```

<!-- Note: For macOS, replace `linux-x64` with `mac-x64` in your supplied value for
`dart.sdkPath`. -->

Note: 对于 macOS，在你的 `dart.sdkPath` 提供值中使用 `mac-x64` 替换 `linux-x64`。

<!-- * Don't run pub with fuchsia. -->

* 不要在 fuchsia 上运行 pub。

```json
"dart.runPubGetOnPubspecChanges": false,
```

<!-- * Configure an 80 character ruler and a tab size of two spaces -->

* 配置单行上限为 80 个字符宽以及 tab 的宽为 2 个空格

```json
"[dart]": {
  "editor.rulers": [80],
  "editor.tabSize": 2
},
```

## CLion/IntelliJ

<!-- * Add the Dart plugin by going to `Settings > Plugins` then searching for
  Dart language support.
* Set the Dart path in `Settings > Languages & Frameworks > Dart` by
  * Check __Enable Dart support for the project <project name>.__
  * Enter the Dart SDK path "${FUCHSIA_SRC}/third_party/dart/tools/sdks/dart-sdk" -->

* 通过转到 `Settings > Plugins` 然后搜索 Dart 语言支持来添加 Dart 插件。
* 设置 Dart 路径在 `Settings > Languages & Frameworks > Dart` 通过以下步骤：
  * 检查 __Enable Dart support for the project <project name>__。
  * 输入 Dart SDK 路径 "${FUCHSIA_SRC}/third_party/dart/tools/sdks/dart-sdk"

<!-- ## Troubleshooting -->

## 疑难解答

<!-- If you find that the IDE is unable to find imports (red squigglies) that are
already correctly in your BUILD.gn dependencies, this is usually a sign that
Dart analysis is not working properly in your IDE. -->

如果您发现 IDE 无法在您的 BUILD.gn 依赖项中找到已经正确的导入（红色波浪线），这通常表明 Dart 分析在您的 IDE 中无法正常工作。

<!-- When this happens, try the following: -->

发生这种情况时，请尝试以下操作：

<!-- ### Open only the project directory you are working on -->
 
### 仅打开您正在处理的项目目录

<!-- E.g. only open `/topaz/shell/ermine` instead of `/topaz`. The analyzer can have
issues with really large source trees. -->

例如：只打开 `/topaz/shell/ermine` 来代替 `/topaz`。分析器在处理非常大的源码树时可能会出问题。

<!-- ### Remove pub output -->

### 删除 pub 输出

<!-- 1.  Delete the `.packages` and `pubspec.lock` files in your project (if
    present).
1.  Ensure that `"dart.runPubGetOnPubspecChanges": false,` is present in your
    VS Code preferences to prevent the files from reappearing whenever a
    `pubspec.yaml` file is edited.
1.  Reload VS Code to restart the Dart analyzer.
    1.  Press Ctrl+Shift+P to open the VS Code Command Palette
    1.  Select "Reload Window" -->

1. 在你的项目中删除 `.packages` 和 `pubspec.lock` 文件（如果存在的话）；
1. 确保在你的 VS Code 首选项中存在 `"dart.runPubGetOnPubspecChanges": false,` 以防止文件再次出现，无论 `pubspec.yaml` 文件何时被编辑；
1. 重载 VS Code 使 Dart 分析器重启；
   1. 按 Ctrl+Shift+P 打开 VS Code 的命令面板
   2. 选择 "Reload Window"

<!-- ### Rebuild -->

### 重构

<!-- Delete `/out` from your Fuchsia directory and rebuild. Dart FIDL bindings are
build-generated and may be absent. -->

在你的 Fuchsia 目录下删除 `/out` 并重构。
Dart FIDL 绑定是构建生成的，可能不存在。

<!-- ### Ensure that your build contains all packages -->

### 确保你的构建包含了所有的包

<!-- Any Dart code from packages not included in your build will not be available to
the analyzer, so ensure your build configuration (`fx set`) includes all
the packages you need (the `--with` flag can be helpful.) -->

分析器将无法使用未包含在您的构建中的包中的任何 Dart 代码，所以确保你的构建配置 (`fx set`) 包含你需要的所有包（`--with` 标志可能会有所帮助。）

<!-- For example, to view the `echo_client_async` example Dart code in VS Code, add
`--with examples/fidl/dart/echo_client_async_dart` to your `fx set`
command. Then, rebuild with `fx build examples/fidl/dart/echo_client_async_dart`. -->

举个例子：要在 VS Code 中查看 `echo_client_async` 示例的 Dart 代码，请将 `--with examples/fidl/dart/echo_client_async_dart` 添加到你的 `fx set` 命令之后。然后，使用 `fx build examples/fidl/dart/echo_client_async_dart` 重建。

<!-- ### Reload the Dart Analyzer -->

### 重载 Dart 分析器

<!-- Manually reloading the analyzer is often needed after modifying FIDLs. -->

修改 FIDL 后通常需要手动重新加载分析器。

#### VS Code

<!-- 1.  Open the Command Palette (Ctrl+Shift+P)
1.  Enter and select "Reload Window" -->

1.  打开命令面板 (Ctrl+Shift+P)
1.  输入并选择 "Reload Window"

<!-- This also restarts the Dart analyzer. -->

这将会重启 Dart 分析器

#### IntelliJ

<!-- 1.  Open Find Action (Ctrl+Shift+A)
1.  Enter and select "Restart Dart Analysis Server" -->

1.  打开查找操作 (Find Action) (Ctrl+Shift+A)
1.  输入并选择 "Restart Dart Analysis Server"

<!-- ### Check that the correct language has been detected for the current file type -->

### 检查是否已为当前文件类型检测到正确的语言

<!-- 1.  On VS Code use Ctrl+Shift+P then type "Change Language Mode" and ensure it is set to "Auto Detect".
1.  If this doesn't fix the issue you can try to manually fix via Ctrl+Shift+P and "Configure file association for .dart" -->

1.  在 VS Code 上使用 Ctrl+Shift+P 然后输入 "Change Language Mode" 并确保它设置为 "Auto Detect"。
1.  如果这不能解决问题，您可以尝试通过 Ctrl+Shift+P 并选择 “为 .dart 配置文件关联” （"Configure file association for .dart"）手动修复。

<!-- ### Manually specifying the Dart sdk path -->

### 手动指定 Dart sdk 路径

#### VS Code

<!-- _See the recommended VS Code options above._ -->

_请参阅上面推荐的 VS Code 选项。_

#### IntelliJ

1.  Open Settings
1.  Under *Languages & Frameworks* > *Dart*, enter "[YOUR FUCHSIA DIR LOCATION]/prebuilt/third_party/dart/{mac,linux}-x64"

1.  打开设置
1.  在 *Languages & Frameworks* > *Dart* 下面，输入 "[YOUR FUCHSIA DIR LOCATION]/prebuilt/third_party/dart/{mac,linux}-x64"
