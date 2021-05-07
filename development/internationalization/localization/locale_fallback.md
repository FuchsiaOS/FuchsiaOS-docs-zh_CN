# Locale fallback

The [Lookup API](lookup_api.md) implements automatic language matching and
[locale fallback][fbk].  These mechanisms are available to in-tree uses
that call the C++ function [`intl::Lookup::New`][iln]

Locale fallback is the process of searching for translated content,
locale data, or other resources by "falling back" from more-specific
resources to more-general ones following a deterministic pattern.

As a simple example of fallback, a lookup request for a message with the locale
`en-US` falls back to using a message from the locale `en` if no
`en-US`-specific message was available to the system.

Fallbacks are not always straightforward.  As a simple example, `en-US` falls
back to `en`, but `en-IN` falls back go `en-GB`.  The [fallback
relationships][ld] are defined in CLDR, and Fuchsia's localization subsystem
uses this information to select the correct fallback localization.

Fuchsia's localized message lookup has two levels of fallback:

1. [*Language matching.*][langm]
   The user is allowed to specify up to 10 preferred locales, in order of
   preference.  This is supported as part of the type
   [`fuchsia.intl.Profile`][fip].

   This setting can be handed over to the [Lookup API](lookup_api.md) at
   `Lookup` construction time:

   ```c++
   auto lookup = intl::Lookup::New({"es", "en-US"});
   ```

   This example shows that the user sets a preference to see messages in the
   locale `es`, with a fall back to `en-US` It is expected that the locale
   information will come from `fuchsia.intl.Profile` normally at runtime.

1. *Fallback.*
   The lookup library automatically falls back from a more specific locale to a
   less specific locale even when not explicitly requested by the user.  In the
   prior example:

   ```c++
   auto lookup = intl::Lookup::New({"es", "en-US"});
   ```

   This example also defines that if `en-US` is not available, that the locale
   should fall back to `en`.  This approach allows us to provide a set of most
   general locales for products that may not need worldwide coverage while still
   covering the most common locales.

The combination of the two approaches uses the automated rules to find the best
available matching locale, but also allows the library caller to specify fine
grained sequence of preferred locales to use.

## Future work

See [Future work](future_work.md#locale_fallback) section for details on
planned but not yet implemented features related to fallback.

<!-- xrefs -->

[fbk]: https://www.w3.org/TR/ltli/#dfn-locale-fallback
[fip]: https://fuchsia.dev/reference/fidl/fuchsia.intl/#Profile
[iln]: /src/lib/intl/lookup/cpp/lookup.h#77
[langm]: http://unicode.org/reports/tr35/#LanguageMatching
[ld]: https://sites.google.com/site/cldr/development/development-process/design-proposals/languagedistance

