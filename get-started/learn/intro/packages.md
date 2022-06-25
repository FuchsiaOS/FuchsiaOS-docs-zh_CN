# Software delivery

<<../../_common/intro/_packages_intro.md>>

<<../../_common/intro/_packages_serving.md>>

<<../../_common/intro/_packages_storing.md>>

## Exercise: Packages

So far in this codelab, you've been experiencing on demand software delivery
to your device and you probably didn't even know it! In this exercise, you'll
peel back the covers and see the details of how packages are delivered and stored
on a Fuchsia device.

<<../_common/_restart_femu.md>>

### Start a local package server

Run the following command to start a package server and enable the emulator to
load software packages:

```posix-terminal
fx serve-updates
```

The command prints output similar to the following, indicating the server is
running and has successfully registered the emulator as a target device:

```none {:.devsite-disable-click-to-copy}
[serve-updates] Discovery...
[serve-updates] Device up
[serve-updates] Registering devhost as update source
[serve-updates] Ready to push packages!
[serve-updates] Target uptime: 139
[pm auto] adding client: [fe80::5888:cea3:7557:7384%qemu]:46126
[pm auto] client count: 1
```

### Examine the package server

The `fx serve-updates` command runs a **local package server** used to deliver
packages to the target devices. By default, this server runs at on port 8083.

Open a browser to `http://localhost:8083`. This loads an HTML page listing all
the packages currently available in the package repository. Each one of these
are packages that can be delivered to the device.

### Monitor package loading

Packages are resolved and loaded on demand by a Fuchsia device. Take a look at
this in action with the `bouncing_ball` example package.

From the device shell prompt, you can confirm whether a known package is
currently on the device:

```posix-terminal
fx shell pkgctl pkg-status fuchsia-pkg://fuchsia.com/bouncing_ball
```

```none {:.devsite-disable-click-to-copy}
Package in registered TUF repo: yes (merkle=ef65e2ed...)
Package on disk: no
```

Open a new terminal and begin streaming the device logs for `pkg-resolver`:

```posix-terminal
ffx log --filter pkg-resolver
```

This shows all the instances where a package was loaded from the package
server.

From the device shell prompt, attempt to resolve the package:

```posix-terminal
fx shell pkgctl resolve fuchsia-pkg://fuchsia.com/bouncing_ball
```

Notice the new lines added to the log output for `pkg-resolver`:

```none {:.devsite-disable-click-to-copy}
[core/pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://google3-devhost/bouncing_ball: [
    e57c05aa909bcb38ca452d31abfbf9cc1d099751c9cd644b4d40cbf64e2af48b,
]
[core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://fuchsia.com/bouncing_ball as fuchsia-pkg://google3-devhost/bouncing_ball to 4ca324998ae9679241c74d2d9d9779fe86c79e2fa1f1627d941a37e987215895 with TUF
```

From the device shell prompt, check the package status again on the device:

```posix-terminal
fx shell pkgctl pkg-status fuchsia-pkg://fuchsia.com/bouncing_ball
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
fx shell pkgctl get-hash fuchsia-pkg://fuchsia.com/bouncing_ball
```

The command returns the unique package hash:

```none {:.devsite-disable-click-to-copy}
ef65e2ed...
```

Provide the full package hash to the `pkgctl open` command to view the package
contents:

```posix-terminal
fx shell pkgctl open {{ '<var>' }}ef65e2ed...{{ '</var>' }}
```

```none {:.devsite-disable-click-to-copy}
opening ef65e2ed... with the selectors []
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
```

This lists the package metadata and each of the content BLOBs in the package.
You can `bin/` entries for executables, `lib/` entries for shared library
dependencies, additional metadata and resources.

## What's Next?

Congratulations! You now have a better understanding of what makes Fuchsia
unique and the goals driving this new platform's design.

In the next module, you'll learn more about the Fuchsia open source project and
the tools used to build and customize the system:

<a class="button button-primary"
    href="/docs/get-started/learn/build">Building Fuchsia</a>
