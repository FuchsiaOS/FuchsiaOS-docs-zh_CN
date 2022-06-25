# Download the Fuchsia source code

This guide provides instructions on how to download the
Fuchsia source code and set up the Fuchsia development
environment on your machine.

The steps are:

1. [Perform a preflight check](#perform-a-preflight-check).
2. [Install prerequisite packages](#install-prerequisite-packages).
3. [Download the Fuchsia source code](#download-the-fuchsia-source-code).
4. [Set up environment variables](#set-up-environment-variables).
5. [Configure firewall rules (Optional)](#configure-firewall-rules).


## 1. Perform a preflight check {#perform-a-preflight-check}

Fuchsia provides a preflight check tool
([`ffx platform preflight`][ffx-platform-preflight])
that examines your machine and informs you of any issues that may
affect building Fuchsia from source on the machine.

Note: The preflight tool only works for the x64 architecture. Fuchsia
is currently not guaranteed to build successfully on other host
architectures, such as Windows and ARM64.

Run the following command:

* {Linux}

  ```posix-terminal
  curl -sO https://storage.googleapis.com/fuchsia-ffx/ffx-linux-x64 && chmod +x ffx-linux-x64 && ./ffx-linux-x64 platform preflight
  ```

* {macOS}

  ```posix-terminal
  curl -sO https://storage.googleapis.com/fuchsia-ffx/ffx-macos-x64 && chmod +x ffx-macos-x64 && ./ffx-macos-x64 platform preflight
  ```

## 2. Install prerequisite packages {#install-prerequisite-packages}

Fuchsia requires `curl`, `file`, `unzip`, and `git` to be up to date. The version
of `git` needs to be 2.28 or higher.

* {Linux}

  Install (or update) the following packages:

  ```posix-terminal
  sudo apt install curl file git unzip
  ```

* {macOS}

  Install the Xcode command line tools:

  Note: Skip this step if `ffx platform preflight` shows that Xcode tools
  are already installed on your machine.

  ```posix-terminal
  xcode-select --install
  ```

## 3. Download the Fuchsia source code {#download-the-fuchsia-source-code}

Fuchsia provides a [bootstrap script](/scripts/bootstrap) that creates a
directory named `fuchsia` and downloads the Fuchsia source code in that
directory.

Downloading the Fuchsia source code requires about 2 GB of space
on your machine. Depending on your build configuration, you need
another 80 to 90 GB of space later when you build Fuchsia. Additionally,
the download process uses a substantial amount of memory. It is advisible
to close non-crucial processes during this time.

To download the Fuchsia source, do the following:

1.  Select a directory for downloading the Fuchsia source code, for example:

    Note: You can set up Fuchsia in any directory. This guide selects
    the `$HOME` directory as an example.

    ```posix-terminal
    cd $HOME
    ```

1.  Run the bootstrap script:

    Note: Downloading the Fuchsia source code can take up to 60 minutes.

    ```posix-terminal
    curl -s "https://fuchsia.googlesource.com/fuchsia/+/HEAD/scripts/bootstrap?format=TEXT" | base64 --decode | bash
    ```
    This script creates the `fuchsia` directory and downloads the source code.

    If you see the `Invalid authentication credentials` error during the
    bootstrapping process, see [Authentication error](#authentication-error) for
    help.

## 4. Set up environment variables {#set-up-environment-variables}

Fuchsia recommends that you update your shell profile to include the following
actions:

*   Add the `.jiri_root/bin` directory to your `PATH`.

    The `.jiri_root/bin` directory in the Fuchsia source contains the
    [`jiri`](https://fuchsia.googlesource.com/jiri) and
    [`fx`](development/build/fx.md) tools that are essential to
    Fuchsia workflows. Fuchsia uses the `jiri` tool to manage repositories in
    the Fuchsia project, and the `fx` tool helps configure, build, run, and
    debug Fuchsia. The Fuchsia toolchain requires that `jiri` is available in
    your `PATH`.

*   Source the `scripts/fx-env.sh` file.

    Though it's not required, sourcing the
    [`fx-env.sh`](/scripts/fx-env.sh) script enables a number of
    useful shell functions in your terminal. For instance, it creates the
    `FUCHSIA_DIR` environment variable and provides the `fd` command for
    navigating directories with auto-completion. (For more information, see
    comments in `fx-env.sh`.)

Note: If you don't wish to update your shell profile, see
[Work on Fuchsia without updating your PATH](#work-on-fuchsia-without-updating-your-path)
in Appendices instead.

To update your shell profile to configure Fuchsia's environment variables,
do the following:

1.  Use a text editor to open your `~/.bash_profile` file (in the example below,
    we use the [Nano][nano]{:.external} text editor):

    Note: This guide uses a `bash` terminal as an example. If you're
    using `zsh`, replace `~/.bash_profile` with `~/.zprofile` in the
    following steps:

    ```posix-terminal
    nano ~/.bash_profile
    ```

1.  Add the following lines to your `~/.bash_profile` file:

    Note: If your Fuchsia source code is not located in the `~/fuchsia`
    directory, replace `~/fuchsia` with your Fuchsia directory.

    ```sh
    export PATH=~/fuchsia/.jiri_root/bin:$PATH
    source ~/fuchsia/scripts/fx-env.sh
    ```

1.  Save the file and exit the text editor.

1.  To update your environment variables, run the following command:

    ```posix-terminal
    source ~/.bash_profile
    ```

1.  Verify that you can run the following commands inside your
    `fuchsia` directory without error:

    ```posix-terminal
    jiri help
    ```

    ```posix-terminal
    fx help
    ```

## 5. Configure firewall rules (Optional) {#configure-firewall-rules}

Note: This step is not required for building or running Fuchsia. But it is
recommended to ensure that Fuchsia's emulator instances run smoothly on Linux.

(**Linux only**) If you're planning on running Fuchsia on Linux, it is advised to
run the following command to allow Fuchsia-specific traffic on the host machine:

```posix-terminal
fx setup-ufw
```

This script requires `sudo` (which asks for your password) to set the appropriate
firewall rules. (For more information on this script, see [`setup-ufw`][setup-ufw].)

## Next steps

To build your first Fuchsia system image, see
[Configure and build Fuchsia](get-started/build_fuchsia.md).

## Appendices

### Authentication error {#authentication-error}

If you see the `Invalid authentication credentials` error during the bootstrap
process, your `~/.gitcookies` file may contain cookies from some repositories in
`googlesource.com` that the bootstrap script wants to check out anonymously.

To resolve this error, do one of the following:

*   Follow the onscreen directions to get passwords for the specified
    repositories.
*   Delete the offending cookies from the `.gitcookies` file.

### Work on Fuchsia without updating your PATH {#work-on-fuchsia-without-updating-your-path}

The following sections provide alternative approaches to the
[Set up environment variables](#set-up-environment-variables) section:

*   [Copy the tool to your binary directory](#copy-the-tool-to-your-binary-directory)
*   [Add a symlink to your binary directory](#add-a-symlink-to-your-binary-directory)

#### Copy the tool to your binary directory {#copy-the-tool-to-your-binary-directory}

If you don't wish to update your environment variables, but you want `jiri` to
work in any directory, copy the `jiri` tool to your `~/bin` directory, for
example:

Note: If your Fuchsia source code is not located in the `~/fuchsia` directory,
replace `~/fuchsia` with your Fuchsia directory.

```posix-terminal
cp ~/fuchsia/.jiri_root/bin/jiri ~/bin
```

However, you must have write access to the `~/bin` directory without `sudo`. If
you don't, `jiri` cannot keep itself up-to-date.

#### Add a symlink to your binary directory {#add-a-symlink-to-your-binary-directory}

Similarly, if you want to use the `fx` tool without updating your environment
variables, provide the `fx` tool's symlink in your `~/bin` directory, for
example:

Note: If your Fuchsia source code is not located in the `~/fuchsia` directory,
replace `~/fuchsia` with your Fuchsia directory.

```posix-terminal
ln -s ~/fuchsia/scripts/fx ~/bin
```

Alternatively, run the `fx` tool directly using its path, for example:

```posix-terminal
./scripts/fx help
```

In either case, you need `jiri` in your `PATH`.

<!-- Reference links -->

[ffx-platform-preflight]: https://fuchsia.dev/reference/tools/sdk/ffx#preflight
[nano]: https://www.nano-editor.org/docs.php
[setup-ufw]: https://fuchsia.dev/reference/tools/fx/cmd/setup-ufw
