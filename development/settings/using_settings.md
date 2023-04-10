# Using Settings

Settings is part of the [Fuchsia SDK][sdk] and is available on products with the
proper supporting packages. Applications on these products can interact with
Settings if they have the appropriate permissions. The interaction follows a
common pattern for accessing and modifying settings across the Settings
protocols.

This guide walks through the steps for incorporating Settings into an
application and interacting with Settings.

## Prerequisites

The Setting Service supports the Settings protocols in Fuchsia. The service's
package `//src/settings/service:setui_service` and one of its core shard
such as `//src/settings/service:setui_service_core_shard` must be present in the
product definition in order to use Settings. The following product definition
includes Settings:

```gn
import("//products/bringup.gni")

base_package_labels += [
  "//src/settings/service:setui_service",
]
core_realm_shards += [
  "//src/settings/service:setui_service_core_shard",
]
```

For more information about Fuchsia's build system, see [The Fuchsia build
system][build].

### Permissions

Any application that accesses Settings must declare usage through its component
manifest. For example, the following manifest declares access to the
[fuchsia.settings.accessibility][accessibility] protocol:

```json5
{
    program: {
        runner: "elf",
        binary: "bin/example",
    },
    use: [
        { protocol: "fuchsia.settings.Accessibility" },
    ],
}
```

For more information about Fuchsia components, see
[Component manifests][manifest].

## Connecting

Applications access Settings through the runtime bindings found in the Fuchsia
SDK. This guide will use Rust for examples, but bindings are available for a
variety of other languages, such as C++ and Dart.

Like other [FIDL][fidl] protocols, the first step to accessing Settings is to
connect to the Setting Service. The following example connects to
[fuchsia.settings.accessibility][accessibility]:

```rust
let proxy = connect_to_protocol::<AccessibilityMarker>().context("failed to connect to Settings");
```

In the above example, `connect_to_protocol` and `AccessibilityMarker` are
provided through the SDK.

Clients should communicate with each Setting protocol over a single connection.
Ongoing communication must occur over the same connection to ensure
the consistency and continuity of responses, as explored in later sections.

## Reading

Each Setting protocol defines a [table][fidl_table] for conveying relevant
details, such as state and status. Organizing data under a single structure
allows Settings to succinctly convey information. The structure also facilitates
communicating changes, as discussed [later](#writing). In the Accessibility
example, `AccessibilitySettings` captures relevant details:

<a name="a11y-table"></a>

```fidl
/// Supported accessibility settings.
type AccessibilitySettings = table {
    /// For videos, use an alternative audio track (akin to changing languages)
    /// that explains what is happening visually while there is no dialogue.
    1: audio_description bool;

    /// Read aloud elements of the screen selected by the user.
    2: screen_reader bool;

    /// Invert colors on the screen.
    3: color_inversion bool;

    /// Interpret triple-tap on the touchscreen as a command to zoom in.
    4: enable_magnification bool;

    /// What type of color-blindness, if any, to correct for.
    5: color_correction ColorBlindnessType;

    /// What kind of sources get closed captions, and how they look.
    6: captions_settings CaptionsSettings;
};
```

A method, called Watch, is present in each protocol to provide access to this
information. This is the declaration for
[fuchsia.settings.accessibility][accessibility]:

```fidl
Watch() -> (struct {
    settings AccessibilitySettings;
});
```

`Watch` follows the [hanging get pattern][hanging-get], returning the current
information on the initial call. Responses to subsequent invocations are
deferred until there is an update to the last returned value. Using the same
proxy connection across these requests is critical for this behavior as Settings
tracks the delivered responses based on the channel. If an error occurs,
Settings will close the FIDL channel with a relevant [epitaph][epitaph].

In the Accessibility example, call `Watch` to determine if the screen reader is
enabled:

```rust
let settings = proxy.watch().expect("settings retrieved");
let screen_reader_enabled = settings.screen_reader.ok_or(false);
```

## Writing

Applications can affect Settings by utilizing the same table structure found
for reading data. Each mutable protocol offers a counterpart method to `Watch`
called `Set`, which takes [AccessibilitySettings](#a11y-table) as an argument:

```fidl
Set(struct {
    settings AccessibilitySettings;
}) -> () error Error;
```

Changes are conveyed by specifying the desired final state in the table fields.
Since each field is optional, only affected fields need to be specified.
By defining changes as deltas, race conditions from multiple callers are
avoided. If successful, the change will be persisted and applied across boots.
Continuing the previous example, a caller can enable the screen reader with the
following code:

```rust
let new_settings = AccessibilitySettings::EMPTY;
new_settings.screen_reader = Some(true);
proxy.set(new_settings).await.expect("request completed").expect("request succeeded");
```

## Debugging

Settings offers a Fuchsia CLI tool ([`ffx setui`][ffx-setui]) for interacting
with its protocols. This tool gives developers real-time access to Settings,
enabling them to see how their application affects and is affected by Settings.
The `ffx setui` tool comes with the Fuchsia source code and SDK. To use it, run
the following command first to opt in:

```posix-terminal
ffx config set setui true
```

To retrieve a Setting protocol's current information (except Accessibility and
VolumePolicy), you can call `ffx setui` with the protocol's name as an argument.
For example, the following command retrieves information about Privacy:

```posix-terminal
ffx setui privacy
```

For Accessibility, add the keyword `watch` after the protocol's name:

```
ffx setui accessibility watch
```

For VolumePolicy, add the keyword `get` after the protocol's name:

```
ffx setui volume_policy get
```

`ffx setui` can also modify Settings. The utility's `help` command details the
specific modification syntax per protocol:

```posix-terminal
ffx setui privacy help
```

Here is an example of setting the user data sharing consent
(`user-data-sharing-consent`) to true:

```posix-terminal
ffx setui privacy -u true
```

<!-- link labels -->
[sdk]: /sdk/fidl/fuchsia.settings/
[fidl]: /docs/concepts/fidl/overview.md
[build]: /docs/development/build/build_system/fuchsia_build_system_overview.md
[accessibility]: /sdk/fidl/fuchsia.settings/accessibility.fidl
[manifest]: /docs/concepts/components/v2/component_manifests.md
[hanging-get]: /docs/development/api/fidl.md#hanging-get
[fidl_table]: /docs/reference/fidl/language/language.md#tables
[epitaph]: /docs/contribute/governance/rfcs/0053_epitaphs.md
[ffx-setui]: https://fuchsia.dev/reference/tools/sdk/ffx#setui
