Build and run the [C++ Hello World component][hello-world-component]{:.external}
included in the SDK samples repository. [Components][fuchsia-component] are the
basic unit of executable software on Fuchsia.

The tasks include:

- Build and run the sample Hello World component.
- Make a change to the component.
- Repeat the build and run steps.
- Verify the change.

Do the following:

1. Build and run the sample component:

   ```posix-terminal
   tools/bazel run //src/hello_world:pkg.component
   ```

   When the build is successful, this command generates build artifacts in a
   temporary Fuchsia package repository, which is then removed after the
   component runs.

   The command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/bazel run //src/hello_world:pkg.component
   ...
   INFO: Build completed successfully, 155 total actions
   Running workflow: pkg.component_base
   Running task: pkg.debug_symbols_base (step 1/2)
   Running task: pkg.component.run_base (step 2/2)
   added repository bazel.pkg.component.runnable
   URL: fuchsia-pkg://bazel.pkg.component.runnable/hello_world#meta/hello_world.cm
   Moniker: /core/ffx-laboratory:hello_world.cm
   Creating component instance...
   Resolving component instance...
   Starting component instance...
   Started component instance!
   ```
1. Check the status of the `hello_world` component:

   ```posix-terminal
   tools/ffx component show hello_world
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx component show hello_world
                  Moniker:  /core/ffx-laboratory:hello_world.cm
                      URL:  fuchsia-pkg://bazel.pkg.component.runnable/hello_world#meta/hello_world.cm
              Instance ID:  None
                     Type:  CML Component
          Component State:  Resolved
    Incoming Capabilities:  /svc/fuchsia.logger.LogSink
     Exposed Capabilities:
              Merkle root:  eebd529bd8ac6d2fd8a467279719f74c76643ebee2e94ebf594ffcbaac02fe8f
          Execution State:  Stopped
   ```

   The output shows that the `hello_world` component has run and is now
   terminated (`Stopped`).

1. Verify the `Hello, World!` message in the device logs:

   ```posix-terminal
   tools/ffx log --filter hello_world dump
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --filter hello_world dump
   [2022-11-30 02:32:28.122][<ffx>]: logger started.
   [183.252][pkg-resolver][pkg-resolver][I] updated local TUF metadata for "fuchsia-pkg://bazel.pkg.component.runnable" to version RepoVersions { root: 1, timestamp: Some(1669775711), snapshot: Some(1669775711), targets: Some(1669775711) } while getting merkle for TargetPath("hello_world/0")
   [183.347][pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://bazel.pkg.component.runnable/hello_world as fuchsia-pkg://bazel.pkg.component.runnable/hello_world to eebd529bd8ac6d2fd8a467279719f74c76643ebee2e94ebf594ffcbaac02fe8f with TUF
   [183.362][pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://bazel.pkg.component.runnable/hello_world as fuchsia-pkg://bazel.pkg.component.runnable/hello_world to eebd529bd8ac6d2fd8a467279719f74c76643ebee2e94ebf594ffcbaac02fe8f with TUF
   {{ '<strong>' }}[183.397][ffx-laboratory:hello_world.cm][I] Hello, World!{{ '</strong>' }}
   ```

1. Use a text editor to edit the `src/hello_world/hello_world.cc` file, for
   example:

   ```posix-terminal
   nano src/hello_world/hello_world.cc
   ```

1. Change the message to `"Hello again, World!"`.

   The `main()` method now should look like below:

   ```none {:.devsite-disable-click-to-copy}
   int main() {
     {{ '<strong>' }}std::cout << "Hello again, World!\n";{{ '</strong>' }}
     return 0;
   }
   ```

1. Save the file and exit the text editor.

1. Build and run the sample component again:

   ```posix-terminal
   tools/bazel run //src/hello_world:pkg.component
   ```

1. Verify the `Hello again, World!` message in the device logs:

   ```posix-terminal
   tools/ffx log --filter hello_world dump
   ```

   This command prints output similar to the following;

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --filter hello_world dump
   ...
   [280.088][pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://bazel.pkg.component.runnable/hello_world as fuchsia-pkg://bazel.pkg.component.runnable/hello_world to 03405c9f712b2db800194d496ce90372845a8f4bbcb1df4a9abfe9c5bdfc40fb with TUF
   {{ '<strong>' }}[280.113][ffx-laboratory:hello_world.cm][I] Hello again, World!{{ '</strong>' }}
   ```

<!-- Reference links -->

[fuchsia-component]: /docs/concepts/components/v2/README.md
[hello-world-component]: https://fuchsia.googlesource.com/sdk-samples/getting-started/+/refs/heads/main/src/hello_world/
