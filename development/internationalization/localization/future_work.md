# Future work

The development of Fuchsia's localization subsystem is heavily influenced by
the platform's immediate needs.  On the other hand, the field of localization
has a long history and a very well known set of issues that need resolving.  We
adopted a development approach where we tackle only the issues that the
Platform will need in the short term, such as the development of features for
the use of the Accessibility manager (a11y) and the Screen Reader.  As result,
there are issues that we deliberately deferred, until there are subsystems
that will need to use them.

This section lists such known, but as of yet unimplemented pieces of work, as a
record for future use, or as a way for an interested party to pick up some work
and contribute it to the open-source Fuchsia repository.  The mailing list
intl-dev@fuchsia.dev is the forum to go to to discuss any of the topic exposed
here.

The future work is classified roughly by area.

## API

1. There is no API support beyond C++.  There is low-level support for Rust,
   but it is not used today as there are currently no Rust clients.  Ideally
   there would be a single implementation (Rust)

1. There is no type safe API for messages.  Ideally we would be able to
   generate type-safe code in the target programming language based on the
   contents of `strings.xml`.  This is not implemented today: it is up to the
   author to verify, through automated testing or through other means, that the
   message formatting and argument ordering matches what the localized message
   expects.  This is because the underlying API is not type-aware and can not
   be fixed except by generating custom code on the fly.

## Security

1. The loaded messages are stored on the heap and can be written to.  This
   presents a potential security risk.  A better approach would be to load the
   messages into a continuous memory region that would subsequently be marked
   as read-only.  This would prevent any attacks that try to exploit the
   fundamental type-unsafety of the formatting API and confuse it into
   revealing information it is not meant to reveal.

## Locale fallback

The following desirable properties are not built into the fallbck mechanism
today but would be very nice to have.

1. There is no support for changing the locale dynamically.  The calling
   program must listen to [`OnChange`][fioc] events and update the `Lookup`
   accordingly.

   This is normally not an issue for Flutter programs, as they monitor for
   locale changes and redraw their UI.  However, in general, it is up to the
   program author to implement reacting to locale changes in a correct way.
   This should not be a big departure from the communication infrastructure
   that supposedly already exists, since we expect that the users of the
   `Lookup` API already integrate with [`fuchsia.intl`][fi] to be aware of the
   user's locale preferences.

[fioc]: https://fuchsia.dev/reference/fidl/fuchsia.intl#fuchsia.intl/PropertyProvider.OnChange
[fi]: https://fuchsia.dev/reference/fidl/fuchsia.intl

## Packaging

1. The localized resources need to be generated and packaged manually by
   registering them in the `BUILD.gn` file.  This could be automated in
   principle, since both the source and the target locations of the resources,
   as well as their respective locales, are all known to the build system.
   This work has not been done yet.

1. On-demand loading of localized resources is not implemented today.  With
   today's implementation, it is not possible to bundle localized resources
   into a package separate from the "main" binary package.  For
   space-constrained devices, and for applications localized in many languages,
   such as the Gmail application, or the Chrome browser, this becomes a
   significant factor that is sometimes a barrier to adoption.

   On-demand loading would allow downloading only the localized resources that
   the user will in fact be using on their device.

1. Third party contributions are not implemented today.  This feature, if
   available, would allow third parties to contribute localized resources to
   applications maintained by someone other than themselves.  This would allow
   independent support for a long tail of languages, which the original author
   has no incentive to maintain.

   Beyond the obvious mechanical support for slicing existing Fuchsia packages
   up into the package "proper" and independent "language pack" data sets, the
   feature gap also includes solving the trust and code verification issues
   between the parties involved in software distribution.  This has simply not
   been a prominent matter as of yet for Fuchsia software.

