{% include "docs/reference/tools/editors/vscode/_common/_vscode_header.md" %}

# Installing the Fuchsia extension for VS Code

The [Fuchsia extension for VS Code][extension-link]{: .external} provides support for debugging
Fuchsia devices, logging, and syntax highlighting. This extension is Fuchsia’s official extension
and can be used with the source tree and the SDK.

### Prerequisites

Before you begin:

* Download [Visual Studio Code][vscode]{: .external}.

For more information on configuring Vs Code, see
[VS Code][vscode-fuchsia].

### Installation

<img class="vscode-image-logo"
     alt="This figure shows the image of the fuchsia extension logo"
     src="images/extensions/extension-logo.png">

* Download the [Fuchsia extension][extension-link]{: .external} from the Visual
  Studio Marketplace.

The extension automatically detects the appropriate settings for each workspace that you use,
including the location of relevant tools such as ffx.  If these settings are incorrect or not set
automatically, follow the section below.

* {SDK}

    Note: For more information about the Fuchsia SDK and how to configure your environment,
    see [SDK fundamentals][sdk-fundamentals].

    1. Open your desired workspace. For example, open the
       [getting-started repository][sdk-fundamentals] in your VS Code workspace.
    1. The extension should automatically detect the path to ffx. If the path
       is not detected, follow these steps:
        1. In VS Code navigate to the main menu, click **Code**, then **Preferences**, then **Settings**.
        1. Under **Extensions** navigate to **Fuchsia SDK** then **Ffx Path**.
        1. Enter the path to `ffx` directory (for example, `~/fuchsia/getting-started/tools/ffx`).
        1. Verify the extension is working via the the button in the bottom right corner. Click said button, which lists a Fuchsia target device and ensure that your device is connected.
 
    If there is no Fuchsia device that is running, including the emulator, you
    will see the following in the **Output** tab:

    ```none {:.devsite-disable-click-to-copy}
    Running: ffxPath target,list,--format,json
    exit: 2: null
    ```

* {Source Tree}

    Note: For more information about the Fuchsia source tree and how to configure your environment, 
    see [source tree fundamentals][sourcetree-fundamentals].

    1. Open your desired workspace. For example, open the [sample repository][sourcetree-fundamentals]
    in your VS Code workspace.
    1. The extension should automatically detect the path to ffx. If not detected follow the following steps:
        1. In VS Code navigate to the main menu, click **Code**, then **Preferences**, then **Settings**.
        1. Under **Extensions** navigate to **Fuchsia SDK** then **Ffx Path**.
        1. Enter the path to `ffx` directory (for example, ` ~/fuchsia/tools/ffx`).
        1. Verify the extension is working via the the button in the bottom right corner. Click said button, which lists a Fuchsia target device and ensure that your device is connected.

    If there is no Fuchsia device that is running, including the emulator, you will see the following in the **Output** tab:

    ```none {:.devsite-disable-click-to-copy}
    Running: ffxPath target,list,--format,json
    exit: 2: null
    ```

You have successfully configured the Fuchsia extension! To see more
information on how to use the Fuchsia extension, see
[Using the Fuchsia extension for VS Code][using-fuchsia-ext].

<!-- Reference links -->
[sdk-fundamentals]: /docs/get-started/sdk/learn
[sourcetree-fundamentals]: /docs/get-started/learn
[vscode-fuchsia]: /docs/reference/tools/editors/README.md#vs-code
[vscode]: https://code.visualstudio.com/
[extension-link]: https://marketplace.visualstudio.com/items?itemName=fuchsia-authors.vscode-fuchsia
[using-fuchsia-ext]: /docs/reference/tools/editors/vscode/fuchsia-ext-using.md
