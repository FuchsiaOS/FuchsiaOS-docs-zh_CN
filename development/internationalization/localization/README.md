# Localization

[Localization](https://en.wikipedia.org/wiki/Language_localisation) (L10N for
short) is a process for adapting the software so that it is usable in a
particular region; or a number of regions. Fuchsia has a workflow that allows
program authors to equip their programs with language-specific resources
(a.k.a _localized assets_).

At present, Fuchsia's localization support is limited, when compared to the
scope of all localization features out there. However, even in its limited
scope (though the scope will grow) enough small scale decisions have been made
that it is useful to document them in the form of a specification.

This specification is by no means complete or final. We reserve the right to
modify it in the future, and though we will make a best effort to evolve it in
backward compatible ways, there may be cases in which breaking changes could be
introduced if the benefits outweigh the potential downsides.
