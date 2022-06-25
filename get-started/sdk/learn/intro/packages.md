# Software delivery

<<../../../_common/intro/_packages_intro.md>>

<<../../../_common/intro/_packages_serving.md>>

<<../../../_common/intro/_packages_storing.md>>

## Exercise: Packages

So far in this codelab, you've been experiencing on demand software delivery
to your device and you probably didn't even know it! In this exercise, you'll
peel back the covers and see the details of how packages are delivered and stored
on a Fuchsia device.

<<../_common/_start_femu.md>>

### Register a new package server

List the package repositories configured in your package server:

```posix-terminal
ffx repository list
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
+----------------------+------+-----------------------------------------------+
| NAME                 | TYPE | EXTRA                                         |
+======================+======+===============================================+
| fuchsiasamples.com   | pm   | /home/alice/.package_repos/sdk-samples        |
+----------------------+------+-----------------------------------------------+
| workstation.qemu-x64 | pm   | /home/alice/.local/share/Fuchsia/.../packages |
+----------------------+------+-----------------------------------------------+
```

The workstation.qemu-x64 repository is created when you run the
`ffx product-bundle get` command (previously in "Get started with the Fuchsia SDK"
quickstart guide).
This repository contains additional system packages for the workstation.qemu-x64 prebuilt image.

Start a local package server instance to begin serving these packages:

```posix-terminal
ffx repository server start
```

```none {:.devsite-disable-click-to-copy}
server is listening on [::]:8083
```

Configure the emulator to resolve package URLs for `fuchsia.com` from the local
package server:

```posix-terminal
ffx target repository register -r workstation.qemu-x64 --alias fuchsia.com
```

### Examine the package server

With the local package server running, you can explore the list of packages that
are available in the repository:

```posix-terminal
ffx repository package list -r workstation.qemu-x64
```

This command prints additional details about each package in the repository,
including the individual components.

### Monitor package loading

Packages are resolved and loaded on demand by a Fuchsia device. Take a look at
this in action with the `bouncing_ball` example package.

From the device shell prompt, you can confirm whether a known package is
currently on the device:

```posix-terminal
fssh pkgctl pkg-status fuchsia-pkg://fuchsia.com/bouncing_ball
```

```none {:.devsite-disable-click-to-copy}
Package in registered TUF repo: yes (merkle=ef65e2ed...)
Package on disk: no
fssh ERROR: Error running ssh: exit status 2
```

Note: The `pkgctl` command returns an error status when the package is not
present on disk.

Open a new terminal and begin streaming the device logs for `pkg-resolver`:

```posix-terminal
ffx log --filter pkg-resolver
```

This shows all the instances where a package was loaded from the package
server.

From the device shell prompt, attempt to resolve the package:

```posix-terminal
fssh pkgctl resolve fuchsia-pkg://fuchsia.com/bouncing_ball
```

Notice the new lines added to the log output for `pkg-resolver`:

```none {:.devsite-disable-click-to-copy}
[128.297][core/pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://workstation.qemu-x64/bouncing_ball: [
    9575f44e2e3eaa25d4e97864abc9e308ee83d2abfda836e3fd4454999b2166a9,
]
[128.386][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://fuchsia.com/bouncing_ball as fuchsia-pkg://workstation.qemu-x64/bouncing_ball to bb0515ee231c3b07da82234d015508f8799ed26b828e8dae16b3e9c59bd87cf2 with TUF
```

From the device shell prompt, check the package status again on the device:

```posix-terminal
fssh pkgctl pkg-status fuchsia-pkg://fuchsia.com/bouncing_ball
```

```none {:.devsite-disable-click-to-copy}
Package in registered TUF repo: yes (merkle=ef65e2ed...)
Package on disk: yes (path=/pkgfs/versions/ef65e2ed...)
```

Fuchsia resolved the package and loaded it from the local TUF repository on
demand!

### Explore package metadata

Now that the `bouncing_ball` package has successfully been resolved, you can
explore the package contents. Once resolved, the package is referenced on the
target device using its content address.

From the device shell prompt, use the `pkgctl get-hash` command to determine the
package hash for `bouncing_ball`:

```posix-terminal
fssh pkgctl get-hash fuchsia-pkg://fuchsia.com/bouncing_ball
```

```none {:.devsite-disable-click-to-copy}
ef65e2ed...
```

Provide the full package hash to the `pkgctl open` command to view the package
contents:

```posix-terminal
fssh pkgctl open ef65e2ed...
```

```none {:.devsite-disable-click-to-copy}
opening ef65e2ed...
package contents:
/bin/bouncing_ball
/lib/ld.so.1
/lib/libasync-default.so
/lib/libbackend_fuchsia_globals.so
/lib/libc++.so.2
/lib/libc++abi.so.1
/lib/libfdio.so
/lib/libsyslog.so
/lib/libunwind.so.1
/meta/bouncing_ball.cmx
/meta/contents
/meta/package
/meta/fuchsia.abi/abi-revision
```

This lists the package metadata and each of the content BLOBs in the package.
You can see `bin/` entries for executables, `lib/` entries for shared library
dependencies, additional metadata and resources.

## What's Next?

Congratulations! You now have a better understanding of what makes Fuchsia
unique and the goals driving this new platform's design.

In the next module, you'll learn more about building Fuchsia's fundamental unit
of software:

<a class="button button-primary"
    href="get-started/sdk/learn/components">Fuchsia components</a>
