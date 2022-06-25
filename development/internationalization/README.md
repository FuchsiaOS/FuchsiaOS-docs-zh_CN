# Internationalization, localization, and input methods in Fuchsia

This is the entry page to the internationalization (I18N), localization (L10N)
and input methods (IM) subsystems in Fuchsia.

## Introduction

Modern user-facing operating systems need to support users across a variety of
regions, languages, and cultures. Localization and internationalization are
critical to this goal.

**Localization** (L10N) is the process of translating user interfaces into local
languages and conventions. This includes text and image content, fonts, regional
variations for components like date, time, text and imagery direction, currency,
and number formats.

**Internationalization** (I18N) is the process of designing locale-independent
software that makes _localization_ in a wide range of languages, regions, and
cultures easy.

Typically, localization refers to the work done by language experts, designers,
tech writers, and other content producers. Internationalization refers to the
work done by software engineers.

## Fuchsia's approach {#fuchsias-approach}

Because Fuchsia is designed to be extensible and to flexibly
support a variety of runtimes and target products, the core operating system
does not strictly specify how to implement i18n and l10n.

However, Fuchsia does include some basic conventions, shared libraries, and
services for i18n/l10n, and uses these in the components and runners that are
typically distributed with Fuchsia.

The following topics discuss the development of Fuchsia applications and
the operating system itself, in multi-language environments.

-   **[Internationalization preferences](i18n_preferences.md):** Fuchsia has
    guidelines for how to encode, read, and write i18n settings.

-   **[ICU library](icu.md):** In-tree components that need common i18n
    functionality such as formatting and parsing dates, times, and numbers,
    working with time zones, displaying bidirectional text, or selecting locales
    and languages can use the third-party ICU library.

    Out-of-tree components can also use ICU, but they need to bring
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

