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

### Examine the system packages

List the package repositories configured in your package server:

```posix-terminal
ffx repository list
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
+--------------------------+------+-----------------------------------------------+
| NAME                     | TYPE | EXTRA                                         |
+==========================+======+===============================================+
| workstation-packages     | pm   | /home/alice/.local/share/Fuchsia/.../packages |
+--------------------------+------+-----------------------------------------------+
```

The `workstation-packages` repository is created when you run the
`ffx product-bundle get` command (previously in "Get started with the Fuchsia SDK"
quickstart guide). This repository contains additional system packages for the
`workstation_eng.qemu-x64` prebuilt image.

With the local package server running, you can explore the list of packages that
are available in the repository:

```posix-terminal
ffx repository package list -r workstation-packages
```

This command prints additional details about each package in the repository,
including the individual components.

### Monitor package loading

Packages are resolved and loaded on demand by a Fuchsia device. Take a look at
this in action with the `spinning-square` example package.

From the device shell prompt, you can confirm whether a known package is
currently on the device:

```posix-terminal
fssh pkgctl pkg-status fuchsia-pkg://fuchsia.com/spinning-square-rs
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
fssh pkgctl resolve fuchsia-pkg://fuchsia.com/spinning-square-rs
```

Notice the new lines added to the log output for `pkg-resolver`:

```none {:.devsite-disable-click-to-copy}
[pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://devhost/spinning-square-rs: [
    6b547fb59fda56866eea01cda90add0aabc1af7c7418c7850011ec6e99a996f1,
    7c1a9fd1c11e9b6b1d3c3184cf350cecfc91ec083b412d20c18b5187d0983d88,
]
[pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://fuchsia.com/spinning-square-rs as fuchsia-pkg://devhost/spinning-square-rs to 21967ecc643257800b8ca14420c7f023c1ede7a76068da5faedf328f9d9d3649 with TUF
```

From the device shell prompt, check the package status again on the device:

```posix-terminal
fssh pkgctl pkg-status fuchsia-pkg://fuchsia.com/spinning-square-rs
```

```none {:.devsite-disable-click-to-copy}
Package in registered TUF repo: yes (merkle=ef65e2ed...)
Package on disk: yes (path=/pkgfs/versions/ef65e2ed...)
```

Fuchsia resolved the package and loaded it from the local TUF repository on
demand!

### Explore package metadata

Now that the `spinning-square` package has successfully been resolved, you can
explore the package contents. Once resolved, the package is referenced on the
target device using its content address.

From the device shell prompt, use the `pkgctl get-hash` command to determine the
package hash for `spinning-square`:

```posix-terminal
fssh pkgctl get-hash fuchsia-pkg://fuchsia.com/spinning-square-rs
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
/bin/spinning_square
/lib/VkLayer_khronos_validation.so
/lib/ld.so.1
/lib/libasync-default.so
/lib/libbackend_fuchsia_globals.so
/lib/libc++.so.2
/lib/libc++abi.so.1
/lib/libfdio.so
/lib/librust-trace-provider.so
/lib/libstd-e3c06c8874beb723.so
/lib/libsyslog.so
/lib/libtrace-engine.so
/lib/libunwind.so.1
/lib/libvulkan.so
/meta/contents
/meta/package
/meta/spinning-square-rs.cm
/meta/spinning-square-rs.cmx
/data/fonts/RobotoSlab-Regular.ttf
/meta/fuchsia.abi/abi-revision
/data/vulkan/explicit_layer.d/VkLayer_khronos_validation.json
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
    href="/get-started/sdk/learn/components">Fuchsia components</a>
