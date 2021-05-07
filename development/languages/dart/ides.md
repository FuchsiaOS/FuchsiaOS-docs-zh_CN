# IDEs for Dart in Fuchsia

### Dart SDK

A prebuilt Dart SDK is available for IDE consumption at:
`prebuilt/third_party/dart/{linux|mac|windows}-x64/bin/dart`.

## Visual Studio Code

1.  Download and install [Visual Studio Code](https://code.visualstudio.com/)
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
    1. Add:

Note: This configuration file is a JSON file. Make sure that you properly use
curly braces.

* Auto-format your files when you save:

```json
"editor.formatOnSave": true,
```

* Check for new SDK updates for Fuchsia:

```json
"dart.checkForSdkUpdates": false,
```

* Configure VS Code to use the bundled Dart SDK


```json
"dart.sdkPath": "/path/to/fuchsia/prebuilt/third_party/dart/linux-x64/bin/dart",
```

Note: For macOS, replace `linux-x64` with `mac-x64` in your supplied value for
`dart.sdkPath`.

* Don't run pub with fuchsia.

```json
"dart.runPubGetOnPubspecChanges": false,
```

* Configure an 80 character ruler and a tab size of two spaces

```json
"[dart]": {
  "editor.rulers": [80],
  "editor.tabSize": 2
},
```

## CLion/IntelliJ

* Add the Dart plugin by going to `Settings > Plugins` then searching for
  Dart language support.
* Set the Dart path in `Settings > Languages & Frameworks > Dart` by
  * Check __Enable Dart support for the project <project name>.__
  * Enter the Dart SDK path "${FUCHSIA_SRC}/third_party/dart/tools/sdks/dart-sdk"


## Troubleshooting

If you find that the IDE is unable to find imports (red squigglies) that are
already correctly in your BUILD.gn dependencies, this is usually a sign that
Dart analysis is not working properly in your IDE.

When this happens, try the following:

### Open only the project directory you are working on

E.g. only open `/topaz/shell/ermine` instead of `/topaz`. The analyzer can have
issues with really large source trees.

### Remove pub output

1.  Delete the `.packages` and `pubspec.lock` files in your project (if
    present).
1.  Ensure that `"dart.runPubGetOnPubspecChanges": false,` is present in your
    VS Code preferences to prevent the files from reappearing whenever a
    `pubspec.yaml` file is edited.
1.  Reload VS Code to restart the Dart analyzer.
    1.  Press Ctrl+Shift+P to open the VS Code Command Palette
    1.  Select "Reload Window"

### Rebuild

Delete `/out` from your Fuchsia directory and rebuild. Dart FIDL bindings are
build-generated and may be absent.

### Ensure that your build contains all packages

Any Dart code from packages not included in your build will not be available to
the analyzer, so ensure your build configuration (`fx set`) includes all
the packages you need (the `--with` flag can be helpful.)

For example, to view the `echo_client_async` example Dart code in VS Code, add
`--with examples/fidl/dart/echo_client_async_dart` to your `fx set`
command. Then, rebuild with `fx build examples/fidl/dart/echo_client_async_dart`.

### Reload the Dart Analyzer

Manually reloading the analyzer is often needed after modifying FIDLs.

#### VS Code

1.  Open the Command Palette (Ctrl+Shift+P)
1.  Enter and select "Reload Window"

This also restarts the Dart analyzer.

#### IntelliJ

1.  Open Find Action (Ctrl+Shift+A)
1.  Enter and select "Restart Dart Analysis Server"

### Check that the correct language has been detected for the current file type
1.  On VS Code use Ctrl+Shift+P then type "Change Language Mode" and ensure it is set to "Auto Detect".
1.  If this doesn't fix the issue you can try to manually fix via Ctrl+Shift+P and "Configure file association for .dart"

### Manually specifying the Dart sdk path

#### VS Code

_See the recommended VS Code options above._

#### IntelliJ

1.  Open Settings
1.  Under *Languages & Frameworks* > *Dart*, enter "[YOUR FUCHSIA DIR LOCATION]/prebuilt/third_party/dart/{mac,linux}-x64"
