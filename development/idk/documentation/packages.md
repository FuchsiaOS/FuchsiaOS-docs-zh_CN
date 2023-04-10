# Packages

A package is the unit of installation on a Fuchsia system. This document describes
various workflows for building and installng a package.

Note: The majority of these workflows rely on the `ffx` tool or the legacy `pm`
tool, which are available in `//tools`.

The workflows are:

* [Build a package](#build-package)
* [Publish a package](#publish-package)
* [Install a package](#install-package)
* [Run a component from an installed package](#run-component)

Additionally, the following legacy workflows are supported:

* [Build a package using the legacy pm tool](#build-package-legacy-pm)
* [Publish a package using the legacy pm tool](#publish-package-legacy-pm)
* [Install a package using the legacy pm tool](#install-package-legacy-pm)

For more details, see the help messages from `ffx package build help`,
`ffx repository publish help`, or `pm`.

## Build a package {#build-package}

To build a package:

1. Create a `meta` directory:

   ```posix-terminal
   mkdir -p {{ '<var>' }}PACKAGE_DIR{{ '</var>' }}/meta
   ```

   Replace <var>PACKAGE_DIR</var> with the staging directory where the package
   is built.

1. Set the `$META_PACKAGE_FILE` environment variable:

   ```posix-terminal
   export META_PACKAGE_FILE={{ '<var>' }}PACKAGE_DIR{{ '</var>' }}/meta/package
   ```

1. Open a text editor and create the `$META_PACKAGE_FILE` file with
   the following content:

   ```none
   {
     "name": "<PACKAGE_NAME>",
     "version": "0"
   }
   ```

   The version number is required to be `0`.

1. Save the file and close the text editor.

1. Create a [package build manifest file][build-manifest-file] (`$BUILD_MANIFEST_FILE`),
   which provides the paths to all the package content files.

   Each line of a manifest file maps to a file contained in the package and
   is in the form of `destination=source` where:

   * `destination` is the path to the file in the final package.
   * `source` is the path to the file on the host machine.

   The manifest file must include at least one line for the package ID file,
   for example:

   ```none {:.devsite-disable-click-to-copy}
   meta/package=<PACKAGE_ID_FILE>
   ```

1. Go to the <var>PACKAGE_DIR</var> directory:

   ```posix-terminal
   cd {{ '<var>' }}PACKAGE_DIR{{ '</var>' }}
   ```

1. Generate a package manifest file, which creates the package metadata archive
   at <var>PACKAGE_DIR</var>`/meta.far`:

   ```posix-terminal
   ffx package build $BUILD_MANIFEST_FILE
   ```

   This command creates the package manifest file implicitly as
   {{ '<var>' }}PACKAGE_DIR{{ '</var>' }}`/package_manifest.json`.

1. Set the `$PACKAGE_MANIFEST_FILE` environment variable:

   ```posix-terminal
   export PACKAGE_MANIFEST_FILE="{{ '<var>' }}PACKAGE_DIR{{ '</var>' }}/package_manifest.json"
   ```

   If the contents of the package change, you need to re-run the
   `ffx package build $BUILD_MANIFEST_FILE` command.

1. Create a package archive, which gathers all the package contents into
   a single distributable file:

   ```posix-terminal
   ffx package archive create -o "{{ '<var>' }}PACKAGE_NAME{{ '</var>' }}.far" "$PACKAGE_MANIFEST_FILE"
   ```

   Replace <var>PACKAGE_NAME</var> with the name of the package.

   This command creates the package archive as <var>PACKAGE_NAME</var>`.far`.

1. Set the`$PACKAGE_ARCHIVE` environment variable:

   ```posix-terminal
   export PACKAGE_ARCHIVE={{ '<var>' }}PACKAGE_DIR{{ '</var>' }}/{{ '<var>' }}PACKAGE_NAME{{ '</var>' }}.far
   ```

   If the contents of the package change, you need to re-run the
   `ffx package build` and `ffx package archive create` commands.

You have successfully built a package. Now you are ready to publish the package.

## Publish a package {#publish-package}

Note: The workflow in this section uses the environment variables set in
the previous [Build a package](#build-package) section.

To publish a package:

1. Initialize a directory that serves as a packages repository:

   ```posix-terminal
   pm newrepo -repo {{ '<var>' }}REPO{{ '</var>' }}
   ```

   This creates a directory structure named <var>REPO</var> that is ready for
   publishing packages.

1. Publish package manifests to the repository:

   ```posix-terminal
   ffx repository publish --package-manifest $PACKAGE_MANIFEST_FILE {{ '<var>' }}REPO{{ '</var>' }}
   ```

   `ffx repository publish` parses `$PACKAGE_MANIFEST_FILE` and publishes the
   package in the provided <var>REPO</var> directory.

   The `--package-manifest` argument can be repeated. If you run this command
   multiple times with different package manifests, each instance will be
   published to the same repository. New versions of the same packages can be
   published using the same command.

1. (Optional) Publish package archives to the repository:

   ```posix-terminal
   ffx repository publish --package-archive $PACKAGE_ARCHIVE {{ '<var>' }}REPO{{ '</var>' }}
   ```

   `ffx repository publish` parses `$PACKAGE_ARCHIVE` and publishes the
   package in the provided <var>REPO</var> directory.

   The `--package-archive` argument can be repeated. If you run this command
   multiple times with different package archives, each instance will be
   published to the same repository. New versions of the same packages can be
   published using the same command.

You have successfully published a package. You are now ready to install a
package.

## Install a package {#install-package}

To install a package:

1. Start the package server:

   ```posix-terminal
   ffx repository server start
   ```

   By default, this starts an amber server on the host machine at port `8083`.

2. Add the repository:

   ```posix-terminal
   ffx repository add-from-pm --name "<REPO_NAME>" "{{ '<var>' }}REPO{{ '</var>' }}"
   ```

   This introduces the repository (with the `pm` directory format) to the
   `ffx repository server`. The `--name "<REPO_NAME>"` is optional, but helpful.

3. Add the new repository as an update source:

   ```posix-terminal
   ffx target repository register
   ```

   Providing a short name for the repository using `-n <REPO_NAME>` is optional,
   but helpful.

3. (On the target device) Download the package:

   ```
   pkgctl resolve fuchsia-pkg://{{ '<var>' }}REPO{{ '</var>' }}/{{ '<var>' }}PACKAGE_NAME{{ '</var>' }}
   ```

   If the component is not already present on the system, `pkgctl` downloads the
   package and places the blobs in the blobFS in the process of resolving. If
   the package already exists, the updates will be downloaded.

You have successfully installed or updated the package. You are now ready to
run a component from the installed package.

## Run a component from an installed package {#run-component}

(On the target device) run the component in a package:

```
run {{ '<var>' }}COMPONENT_URI{{ '</var>' }}
```

Replace <var>COMPONENT_URI</var> with a package URL in the form of
`fuchsia-pkg://<REPO>/<PACKAGE_NAME>#meta/<COMPONET_NAME>.cmx`.

You have successfully run a component from the installed package.

## Build a package using the legacy pm tool {#build-package-legacy-pm}

To build a package:

1. Create the package ID file:

   Note: `$PACKAGE_DIR` is a staging directory where the package
   is built.

   ```posix-terminal
   pm -o $PACKAGE_DIR -n $PACKAGE_NAME init
   ```

   This generates the package ID file implicitly as
   `$PACKAGE_DIR/meta/package`.  Set `$PACKAGE_ID_FILE` accordingly
   for use in subsequent steps:

   ```posix-terminal
   export PACKAGE_ID_FILE=${PACKAGE_DIR}/meta/package
   ```

   `$PACKAGE_ID_FILE` will contain the following data:

   ```
   {
     "name": "<package name>",
     "version": "<package version>"
   }
   ```

2. Create the manifest file, `$MANIFEST_FILE`, that provides the path to
   the package ID file.  Each line of a manifest file maps a single file that
   is contained in the package and is in the form of `destination=source` where:

   * `destination` is the path to the file in the final package
   * `source` is the path to the file on the host machine

   The manifest file must include at least one line for the package ID file like
   this:

   ```
   meta/package=<package ID file>
   ```

3. Generate the package metadata archive:

   ```posix-terminal
   pm -o $PACKAGE_DIR -m $MANIFEST_FILE build
   ```

   This creates the metadata archive at `$PACKAGE_DIR/meta.far`.

4. (Optional) Create the package archive `$PACKAGE_ARCHIVE`:

   ```posix-terminal
   pm -o $PACKAGE_DIR -m $MANIFEST_FILE archive
   ```

   This command creates the package archive implicitly as
   `$PACKAGE_DIR/$PACKAGE_NAME-0.far`.  Set `$PACKAGE_ARCHIVE` accordingly
   for use in subsequent steps:

   ```posix-terminal
   export PACKAGE_ARCHIVE=${PACKAGE_DIR}/${PACKAGE_NAME}-0.far
   ```

   If the contents of the package change, you need to re-run the
   `pm -o $PACKAGE_DIR -m $MANIFEST_FILE archive` command.

You have successfully built a package. You are now ready to publish the package.

## Publish a package using the legacy pm tool {#publish-package-legacy-pm}

To publish a package:

1. Initialize a directory, `$REPO`, that serves as a packages repository:

   ```posix-terminal
   pm newrepo -repo $REPO
   ```

   This creates a directory structure named `$REPO` that is ready for
   publishing packages.

2. Publish packages to the repository `$REPO`:

   ```posix-terminal
   pm publish -a -r $REPO -f $PACKAGE_ARCHIVE
   ```

   `pm publish` parses `$PACKAGE_ARCHIVE` and publishes the package in the
   provided `$REPO` directory. If you run this command multiple times with
   different package archives, `pm publish` publishes the packages to the same
   repository. New versions of a same package can be published using the same
   command.

You have successfully published a package. You are now ready to install a
package.

## Install a package using the legacy pm tool {#install-package-legacy-pm}

To install a package:

1. Start the package server:

   ```posix-terminal
   pm serve -repo $REPO
   ```

   By default, this starts an amber server on the host machine at port `8083`.

2. (On the target device) Add the new repository as an update source with
   `pkgctl`:

   ```
   pkgctl repo add url -f 1 -n $REPO http://$HOST_ADDRESS:8083/config.json
   ```

   The option `-f 1` must be set if `pm` is serving a component v1 config.json
   configuration file. (This is currently the case, but will change to serving
   component v2 configuration files in the future. Once this change has
   happened, the `-f 1` can be omitted.)

   Providing a short name for the repository using `-n $REPO` is optional, but
   helpful. If this short name is not provided, `pkgctl` will derive it from
   the provided config URL.

3. (On the target device) Get the package:

   ```
   pkgctl resolve fuchsia-pkg://$REPO/$PACKAGE_NAME
   ```

   If the component is not already present on the system, `pkgctl` downloads the
   package and places the blobs in the blobfs in the process of resolving. If
   the package already exists, the updates will be downloaded.

You have successfully installed or updated the package. You are now ready to
run a component from the installed package.

<!-- Reference links -->

[build-manifest-file]: /docs/development/build/build_system/internals/manifest_formats.md
