# Internationalization

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

Because Fuchsia is [designed to be extensible][inclusive] and to flexibly
support a variety of runtimes and target products, the core operating system
does not strictly specify how to implement i18n and l10n.

However, Fuchsia does include some basic conventions, shared libraries, and
services for i18n/l10n, and uses these in the components and runners that are
typically distributed with Fuchsia. For more information, see the
[Internationalization][i18n-guide] section in the Development guides.

<!--xrefs-->

[i18n-guide]: /docs/development/internationalization
[inclusive]: /docs/concepts/principles/inclusive.md#fuchsia_architecture_is_inclusive_by_design
