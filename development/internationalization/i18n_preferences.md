# Internationalization preferences

Fuchsia has some conventions for how to communicate i18n preferences, whether
from an end user to components, or among components.

This guide covers the following:

-   Use Unicode BCP-47 Locale Identifiers to encode [locale IDs](#locale-ids).
-   Use `fuchsia.intl.PropertyProvider` in components to
    [read internationalization preferences](#access). Make sure that the product
    configuration includes [`intl_services`](#intl-services) or another
    implementation of this protocol.
-   If implementing a settings UI, use `fuchsia.settings.Intl` to
    [write internationalization settings](#store).

## Locale identifiers {#locale-ids}

The keystone of i18n preferences is the _locale identifier_, which is a string
that concisely conveys information such as:

-   Language (e.g. English, French, Arabic)
-   Country or region (e.g. United Kingdom, Morocco, South Korea)
-   Script (e.g. Latin, Cyrillic, Traditional Chinese, Simplified Chinese)

Locale identifiers can also convey more granular information, such as:

-   Calendar (e.g. Gregorian, Japanese, Hebrew)
-   First day of week
-   Collation (sort order, grouping strings for search)
-   Digit style (e.g. "Arabic" 012345, "Eastern Arabic" ٠١٢٣٤٥)
-   Number format (decimal separator, digit grouping)
-   Currency format
-   Time and date formats
-   Etc.

Specifying these details is particularly useful when overriding the default
values for a given language and region (see next section).

### Unicode BCP-47 Locale Identifiers {#unicode-bcp-47}

Fuchsia uses the
[Unicode BCP-47 Locale Identifier](http://www.unicode.org/reports/tr35/#BCP_47_Conformance)
standard for locale IDs.

For example, the following locale ID specifies the Serbian language (`sr`) as
spoken in Serbia (`RS`), written in a Cyrillic script (`Cyrl`):

```none {:.devsite-disable-click-to-copy}
"sr-Cyrl-RS"
```

You can use
[Unicode extension subtags](http://unicode.org/reports/tr35/#u_Extension) in the
locale ID to add overrides. Consider the following example:

```none {:.devsite-disable-click-to-copy}
"sr-Cyrl-RS-u-ca-hebrew-fw-monday-ms-ussystem-nu-deva-tz-usnyc"
```

This example specifies the following:

| Subtag(s)     | Meaning                                                      |
| ------------- | ------------------------------------------------------------ |
| `sr`          | Specifies the Serbian language.                              |
| `Cyrl`        | Specifies the Cyrillic script.                               |
| `RS`          | Specifies Serbia as the country/region.                      |
| `u`           | Marks the start of the Unicode extension data.               |
| `ca-hebrew`   | Specifies the Hebrew calendar.                               |
| `fw-monday`   | Specifies Monday as the first day of the week.               |
| `ms-ussystem` | Specifies the measurement system as "US", e.g. feet, ounces, |
:               : etc.                                                         :
| `nu-deva`     | Specifies Devanagari numerals.                               |
| `tz-usnyc`    | Specifies the time zone as `America/New_York`.               |

Not all internationalization properties that one might want to express have a
corresponding Unicode extension. For example, there is currently no extension
for temperature units, so there is no way to express "use metric units, but use
Fahrenheit for temperature" in a locale ID.

## Accessing i18n preferences {#access}

To send i18n preferences between Fuchsia
[components](/docs/glossary/README.md#component), use the
[`fuchsia.intl.Profile`](https://fuchsia.dev/reference/fidl/fuchsia.intl#Profile)
FIDL table:

```fidl {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/fidl/fuchsia.intl/intl.fidl" indented_block="type Profile" exclude_regexp="(//.*)|(^$)" %}
```

The locale ID is only a building block in the `Profile`. A profile contains a
ranked list of locale IDs (to express relative preference, priority, or degree
of support; see [Locale fallback](./localization/locale_fallback.md) for a
usage example), as well as other properties that cannot be fully expressed in a
single locale ID. When there is a conflict, explicit settings in the `Profile`
override the values in the locale ID (e.g. specifying US measurement units in
the locale ID but `CELSIUS` in the `temperature_unit` field).

When a component needs to provide i18n preferences to other components, it
should implement the
[`fuchsia.intl.PropertyProvider`](https://fuchsia.dev/reference/fidl/fuchsia.intl#PropertyProvider)
protocol, which serves the `Profile` and notifies of changes:

```fidl {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/fidl/fuchsia.intl/property_provider.fidl" indented_block="protocol PropertyProvider" exclude_regexp="(//.*)|(^$)" %}
```

This protocol offers a _read-only_ view of an internationalization profile.
Depending on the implementation of the service and the
[realm](/docs/concepts/components/v2/realms.md) in which it is intended to
serve, the contents of the internationalization profile may be derived from
[user settings](#store), a product's factory settings, a specific component's
requirements, or some combination of the above.

### Do not assume an ambient locale {#no-ambient-locale}

Fuchsia has no ambient or system locale. Locale and other i18n preferences
depend on the context in which a component is running. This is in contrast to
other operating systems, which may have APIs to obtain global or default locale
settings, following Fuchsia's design principle of
[no ambient authority](/docs/concepts/principles/secure.md)

In runtimes where the standard library offers access to some default locale (for
example, `Platform.localeName` in Dart and Flutter), it is the responsibility of
the [runner](/docs/concepts/components/v2/capabilities/runners.md) to retrieve
the needed values from the realm of the component being run. In most cases, the
runner should call `fuchsia.intl.PropertyProvider.GetProfile`. See
[Runner integrations](#runner-integrations) below.

#### Multiple i18n `Profile`s {#multiple-i18n-profiles}

Depending on a product's design, it is possible that two component instances
running concurrently on the same machine in different realms are connected to
different `PropertyProvider` instances and receive different `Profile`s.

For example, an encyclopedia component showing a Spanish-language (`es-ES`)
article about Mallorca may choose to launch a map component with an `es-ES` UI,
while at the same time an English-language (`en-US`) article launches the same
map component, but tells it to display an `en-US` UI. This can be accomplished
with two different
[sub-realms](/docs/concepts/components/v2/realms.md#definitions) that each
receives a different `PropertyProvider` instance.

### `intl_services` library {#intl-services}

A basic C++ library implementing `fuchsia.intl.PropertyProvider` is found at
[`//src/lib/intl/intl_property_provider_impl`](/src/lib/intl/intl_property_provider_impl).

The
[`core`](/docs/development/build/build_system/boards_and_products.md#key_product_configurations)
product configuration includes [`intl_services`](/src/intl/intl_services), a
component that wraps this implementation.

### Runner integrations {#runner-integrations}

#### dart_runner

In the future, accessing the field `Platform.localeName` in Dart will return the
_first_ `LocaleId` from the vector `fuchsia.intl.Profile.locales`. (This is
currently blocked by a
[limitation](https://github.com/dart-lang/sdk/issues/37586) in the Dart SDK.)

#### flutter_runner

The Flutter runner on Fuchsia is [wired up][flutter-source] to
`fuchsia.intl.PropertyProvider`, so using the standard
[Flutter localization API][flutter-l10n] should allow a Flutter application to
access the current context's locale preferences and detect changes. For details
and examples, see [Localizing mods](localizing_mods.md).

Note: Both Dart and Flutter components that are built _only for Fuchsia_ have
the option of directly accessing the `fuchsia.intl.PropertyProvider` FIDL
service — in addition to using the OS-agnostic APIs. Cross-platform apps should
use the properties provided by their runtimes.

#### web_runner

The list of preferred locales from `Profile` is sent to web serves in the HTTP
request header [`Accept-Language`][accept-language]. In the future, they may
also be made available in JavaScript in
[`navigator.languages` and `navigator.language`][navigator-languages].

## Storing i18n user settings {#store}

As with other user settings on Fuchsia, internationalization settings are
modified through the
[`fuchsia.settings`](https://fuchsia.dev/reference/fidl/fuchsia.intl/index) FIDL
protocols.

Specifically, the protocol
[`fuchsia.settings.Intl`](https://fuchsia.dev/reference/fidl/fuchsia.intl/index#Intl)
is used to write and monitor internationalization-related settings.

```fidl {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/fidl/fuchsia.settings/intl.fidl" indented_block="protocol Intl" exclude_regexp="(//.*)|(^$)" %}
```

```fidl {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="sdk/fidl/fuchsia.settings/intl.fidl" indented_block="type IntlSettings" exclude_regexp="(//.*)|(^$)" %}
```

This protocol is intended specifically for components that require direct access
to user settings, such as a system control panel, a taskbar locale selector, or
an _implementor_ of `fuchsia.intl.PropertyProvider`. In typical Fuchsia product
configurations, this access should be restricted to a narrow allowlist.

Most client components will instead use the [read-only view](#access) offered
through `fuchsia.intl.PropertyProvider`.

### Implementation: `setui_service` {#setui-service}

The protocol `fuchsia.settings.Intl` is implemented by the
[`setui_service`](/src/settings/service) (along with the other protocols under
`fuchsia.settings`). This service serves as the backend for settings UIs in
Fuchsia products.

<!--xrefs-->

[flutter-source]: https://cs.opensource.google/flutter/engine/+/master:shell/platform/fuchsia/flutter/engine.cc;?q=%5Cbintl_property_provider_%5Cb
[flutter-l10n]: https://flutter.dev/docs/development/accessibility-and-localization/internationalization
[accept-language]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept-Language
[navigator-languages]: https://developer.mozilla.org/en-US/docs/Web/API/NavigatorLanguage/languages
