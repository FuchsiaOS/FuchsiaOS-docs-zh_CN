# Internationalization, localization, and input methods in Fuchsia

This is the entry page to the internationalization (I18N), localization (L10N)
and input methods (IM) subsystems in Fuchsia. You can find a higher-level
conceptual overview [in the Concepts section][i18n-concepts].

The topics you find here deal with the development of Fuchsia applications, and
the operating system itself, in multi-language environments.

-   **[Internationalization preferences](i18n_preferences.md):** Fuchsia has
    guidelines for how to encode, read, and write i18n settings.

-   **[ICU library](icu.md):** In-tree components that need common i18n
    functionality such as formatting and parsing dates, times, and numbers,
    working with time zones, displaying bidirectional text, or selecting locales
    and languages can use the third-party ICU library.

    <!-- "&colon;" is used to avoid Note callout -->

    Note&colon; Out-of-tree components can also use ICU, but they need to bring
    their own copy of ICU because the library is not part of the official
    Fuchsia SDK.

-   **[ICU time zone data](icu_data.md):** Both in-tree _and_ out-of-tree
    components that use ICU to deal with dates and times should consider using
    Fuchsia's shared ICU time zone data to ensure consistent time among
    components.

-   **Localizing components:** Fuchsia offers some basic guidance on
    [how to localize in-tree components](localizing_mods.md), as well as a
    [string lookup library and workflow](localization/message_translation.md)
    for C++ components.

-   **[Fonts](fonts):** Fuchsia provides a FIDL service for loading font assets,
    with an API that is particularly useful for `freetype2` clients.

<!--xrefs-->

[i18n-concepts]: /docs/concepts/internationalization/introduction.md
