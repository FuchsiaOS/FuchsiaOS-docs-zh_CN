# Experimental plugins

When developing a new plugin, the experimental (or 'work in progress') nature of
the work is indicated by marking the plugin as experimental.

Experimental plugins:

- Experimental plugins may change their arguments inputs and outputs in every
  IDK release.
- Experimental plugins may _temporarily_ depend on APIs or ABIs that are not
  part of the Fuchsia platform surface area. All dependencies must still be
  defined and represented in the platform source tree.
- Plugin authors must demonstrate a roadmap to non-experimental status, for
  example stabilization of the dependencies of the plugin. It is a goal to
  remove experimental plugins over time to maximize the stability of the ffx
  surface and dependencies.
- Plugins may not introduce behavior that execute at times other than when a
  command from the plugin is invoked (i.e. an experimental plugin can not
  perform background operations in the ffx daemon or Remote Control Service).
- Experimental plugins are subject to the same code review and code quality
  process as other plugins. As experimental plugins are included in the standard
  ffx binary, they must adhere to the same quality standards so as to not affect
  the quality of the product as a whole.

## Mark a plugin as experimental

When defining a plugin function within the plugin lib.rs file, there will be an
entry point decorated with `ffx_plugin` such as:

```
#[ffx_plugin()]
```

or

```
#[ffx_plugin(MyProxy = "fuchsia.example.Service")]
```

To mark a plugin as experimental and simultaneously declare the token used to
enable the experiment, add an initial string parameter. In the following example
the tag "zippy" may (or may not) match the plugin command name. The label may be
arbitrary, but it's recommended to choose a value that relates to the plugin:

```
#[ffx_plugin("zippy")]
```

or

```
#[ffx_plugin("zippy", MyProxy = "fuchsia.example.Service")]
```

In both examples above, the plugin will be guarded by a feature token named
"zippy". When users try to execute `ffx zippy` they will be informed that zippy
is experimental, then instructions for enabling the zippy feature will be shown.

After following the instructions to enable zippy, future calls (for that user)
will operate as if zippy were not experimental, i.e. they have opted-in to using
the zippy feature.

## CL review of experimental code

Experimental plugins must not be used as a mechanism to bypass code quality
concerns, design, security, privacy and other concerns. Every element of ffx
must adhere to a common base standard for inclusion in the IDK.

## Removing the experimental status {#stable-plugin}

Before leaving the experimental state a plugin must have:

- The UX surface is reviewed and sufficiently stable as to adhere to the
  platform versioning RFC.
- The implementation dependencies are all part of the defined platform surface
  area, for example, all FIDL dependencies are sdk_category = 'public'.
- Design doc (accepted by stakeholders)
- User documentation
- Args that comply to the CLI rubric
- Unit tests
- Integration (e2e) tests
