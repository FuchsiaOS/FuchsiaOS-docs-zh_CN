# Register debug symbols

The [`ffx debug symbol-index`][ffx-debug-symbol-index] commands help
manage Fuchsia components’ debug symbols in your development environment.

## Concepts

During development, debug symbols for a Fuchsia component are generated
as part of build artifacts. A file containing debug symbols has
`.symbol-index.json` as a postfix (for example,
`my-component.symbol-index.json`).

Registering a Fuchsia component's debug symbols in your development
environment enables the following workflows:

*   View the component's logs in [symbolized][symbolize-logs] format.
*   Use the [Fuchsia debugger][start-the-fuchsia-debugger] to step through the
    code while the component is running on a device.
*   Monitor the component's [FIDL traffic][monitor-fidl] on a device in real time.

In addition to registering locally generated debug symbols, you can also
configure your environment to retrieve debug symbols from an online storage
(for instance, Google Cloud Storage). Once an online storage is added in
your [global `symbol-index` configuration](#list-registered-debug-symbols),
various Fuchsia debugging tools (such as [`ffx log`][symbolize-logs] and
[`ffx debug connect`][start-the-fuchsia-debugger]) can automatically
download and use the debug symbols available in the online storage.

In a Fuchsia development environment, a registration of debug symbols
often takes place in the background. Some Fuchsia tools are configured
to invoke the registration of debug symbols
(using [`ffx debug symbol-index add`](#manually-register-debug-symbols))
upon detection of debug symbols, so you may never need to manually
register the debug symbols of your Fuchsia component during development.
For instance, if you're using Fuchsia's Bazel rules, the debug symbols get
registered automatically as part of the build. However, when necessary,
you could use the `ffx debug symbol-index`
commands to manage debug symbols directly.

## List registered debug symbols {:#list-registered-debug-symbols}

To view your global `symbol-index` configuration, run the following command:

```posix-terminal
ffx debug symbol-index list
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx debug symbol-index list
SymbolIndex {
    includes: [
        "/usr/alice/home/my-fuchsia-project/my-component.symbol-index.json",
    ],
    build_id_dirs: [],
    ids_txts: [],
    gcs_flat: [
        GcsFlat {
           url: "gs://our-fuchsia-project/debug",
           require_authentication: false,
        },
    ],
}
```

## Manually register debug symbols {:#manually-register-debug-symbols}

Important: A registration of debug symbols is invoked automatically
by other tools in a Fuchsia development environment. A manual registration
of debug symbols is required only when your environment doesn't support
this automatic setup.

To register debug symbols in your environment, run the following command:

```posix-terminal
ffx debug symbol-index add <PATH_TO_DEBUG_SYMBOLS>
```

Replace `PATH_TO_DEBUG_SYMBOLS` with an absolute path to a debug symbols
file.

The example below registers the debug symbols of the `my-component`
component:

```none {:.devsite-disable-click-to-copy}
$ ffx debug symbol-index add /usr/alice/home/my-fuchsia-project/my-component.symbol-index.json
```

When the registration is successful, the command exits silently without output.

If you want to verify the registration,
see [List registered debug symbols](#list-registered-debug-symbols).

## Remove registered debug symbols {:#remove-registered-debug-symbols}

To remove debug symbols from your global `symbol-index` configuration, run the
following command:

Note: Run `ffx debug symbol-index list` to check the registered paths in your
global `symbol-index` configuration first. Then use the exact path as input to
the command below.

```posix-terminal
ffx debug symbol-index remove <PATH_TO_DEBUG_SYMBOLS>
```

Replace `PATH_TO_DEBUG_SYMBOLS` with the exact path to a registered debug
symbols file to be removed.

The example below removes the debug symbols of the `my-component` component:

```none {:.devsite-disable-click-to-copy}
$ ffx debug symbol-index remove /usr/alice/home/my-fuchsia-project/my-component.symbol-index.json
```

When the removal is successful, the command exits silently without output.

## Clean up misplaced debug symbols {:#clean-up-misplaced-debug-symbols}

Deleting a registered debug symbols file on the host machine does not mean that
your global `symbol-index` configuration also gets updated automatically.

To remove any stale paths (pointing to deleted debug symbols files) from
the global `symbol-index` configuration, run the following command:

```posix-terminal
ffx debug symbol-index clean
```

For instance, if the `/usr/alice/home/my-fuchsia-project` directory no longer
exists on the host machine, this command removes the
`/usr/alice/home/my-fuchsia-project/my-component.symbol-index.json` entry from
the global `symbol-index` configuration.

<!-- Reference links -->

[ffx-debug-symbol-index]: https://fuchsia.dev/reference/tools/sdk/ffx#symbol-index
[symbolize-logs]: ./symbolize-logs.md
[start-the-fuchsia-debugger]: ./start-the-fuchsia-debugger.md
[monitor-fidl]: ./monitor-fidl-messages-on-a-device.md
