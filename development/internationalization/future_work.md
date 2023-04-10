# Future work

## Localization

For information about the future work related to localization, see
[Future work](./localization/future_work.md).

## Internationalization preferences

-   Migrate `fuchsia.intl.PropertyProvider` change watcher API to a
    [hanging get](/docs/development/api/fidl.md#hanging-get) design and migrate all
    existing clients.

-   Instead of having a single `fuchsia.intl.PropertyProvider` implemented by
    the `intl` component, demonstrate best practices for embedding custom
    `PropertyProvider`s in [session component](/docs/glossary#session-component)
    implementations, ideally with multi-user use cases.

-   Wire up Dart's `Platform.localeName` (blocked by Dart SDK
    [#37586](https://github.com/dart-lang/sdk/issues/37586)).

-   Wire up and verify Chromium's
    [`navigator.language` and `navigator.languages`][navigator-languages].

## Internationalization UI

-   For the Workstation product, design and implement an i18n settings UI.

## Fonts

-   Complete support for ephemeral fonts, i.e. loading fonts as ephemeral
    packages that are temporarily cached. This requires production support for
    ephemeral packages in Fuchsia.

    -   Configure a set of ephemeral fonts to cover all common scripts supported
        by Unicode. (On target devices with lots of storage space, these don't
        necessarily need to be ephemeral.)

-   Add support for font subsetting by allowing single typefaces to be split
    across multiple files, with metadata mapping code point ranges to files. For
    large fonts, this would improve loading performance.

-   Make font service multi-threaded to better handle workloads where a single
    component is displaying text in many fonts at once, or when multiple
    components are being displayed side by side.

-   Use ICU4X to implement real language and script ID matching in font service.

-   Implement support for async local font loading in Flutter.

-   Implement support for async local font loading in Chromium.

-   For Workstation product, design and implement a font management UI for
    reviewing and testing out fonts available in a given build.

## Time zones

-   Implement a FIDL service that provides a listing of time zone IDs and
    localized display names. This will initially be used in the Workstation
    product's settings UI.

## Input methods

-   Design and implement APIs for switching keyboard layouts.

-   For Workstation product, design and implement UI for switching keyboard
    layouts.

<!--xrefs-->

[navigator-languages]: https://developer.mozilla.org/en-US/docs/Web/API/NavigatorLanguage/languages
