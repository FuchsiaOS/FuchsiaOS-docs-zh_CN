# Editors

Several editors have support for FIDL.

## Atom

To install

    pushd ~/.atom/packages
    git clone https://fuchsia.googlesource.com/atom-language-fidl
    popd

## IntelliJ / Android Studio

There is an IntelliJ plugin available for FIDL.  It adds syntax and parsing
support.  To install it, choose **Settings > Plugins**, then click **Browse
Repositories** and search for **FIDL**.

## Sublime Text

[Sublime syntax highlighting support can be found here](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/tools/sublime).

To install, go to **Sublime Text > Preferences > Browse Packages** and copy the
files ``FIDL.sublime-syntax``, and ``Comments (FIDL).tmPreferences`` into the
**User** package.

## Vim

[Vim syntax highlighting support and instructions can be found here](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/tools/vim).

## Visual Studio Code

There is a (Visual Studio Code extension available) at
<https://marketplace.visualstudio.com/items?itemName=fuchsia-authors.language-fidl>.

## Contributing

Contributions to the various plugins are welcome. Their respective code lives at

* **Atom**: <https://fuchsia.googlesource.com/atom-language-fidl/>
* **IntelliJ**: <https://fuchsia.googlesource.com/intellij-language-fidl/>
* **Sublime**: <https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/tools/sublime>
* **vim**: <https://fuchsia.googlesource.com/garnet/+/master/public/lib/fidl/tools/vim>
* **Visual Studio Code**: <https://fuchsia.googlesource.com/vscode-language-fidl/>

