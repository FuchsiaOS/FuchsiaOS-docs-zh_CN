# Device firmware

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

Device firmware are binary blobs containing code that are executed by device
hardware. The binary blob is available in the driver's namespace for loading.

Device firmware are stored in CIPD (Chrome Infrastructure Package Deployment)
and mirrored in Google Storage.

## Create a Firmware Package

To create a firmware package, create a directory containing the following
files:

* One or more firmware files
* A license file
* [README.fuchsia](/docs/development/source_code/third-party-metadata.md)

README.fuchsia must contain at least the following directives:

* `Name`
* `Version`
* `Upstream Git`
* `License`
* `License File`

If this is the first time you uploaded to CIPD from the host system,
authenticate with CIPD:

```
fx cipd auth-login
```

Upload and tag the package in CIPD using the following command:

```
fx cipd create -in <package-directory> -install-mode copy \
    -name <package-name> \
    -tag git_repository:<source-git-repositry> \
    -tag git_revision:<source-git-revision>
```

`package-name` has the format `fuchsia/firmware/<name>`.

`<name>` should be a string that identifies the firmware. It may contain
any non-whitespace character. It is helpful to identify the driver that will
use the firmware in the name.

After this step, the package is uploaded to CIPD. Check the
[CIPD browser here](https://chrome-infra-packages.appspot.com/#/?path=fuchsia/firmware)
for packages under `fuchsia/firmware`.

## Adding the Firmware Package to the Build

This must be done in the `integration.git` repository.  See the `firmware` file
in the open-source repository for examples.
