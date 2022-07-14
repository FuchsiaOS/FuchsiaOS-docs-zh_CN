{% set rfcid = "RFC-0058" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-013.

## Summary

Note: This FTP is superseded by [RFC-0083: FIDL Versioning][rfc-0083].


Use a new attribute `[Deprecated]` to indicate deprecation of types (enums,
structs, unions, using declarations), consts, methods, or whole interfaces.
Carry this over to target languages in the best possible way.

### Relation to other RFCs

This RFC was superseded by:

* [RFC-0083: FIDL versioning](0083_fidl_versioning.md)

## Motivation

We have a number of comments indicating that a type, method, or interface
should not be used anymore.
Examples [here][donotuse1], or [here][donotuse2].
By standardizing on a way to communicate deprecation, exposing this in the JSON
IR, and leveraging this information in the various language backends, we can
turn these notes into warnings, in target languages, which will more easily guide
developers using APIs.

### Survey of Uses Today
Surveyed with "`ack --type=fidl -i 'deprecated' -A 2 -B 2`"

* On methods
    * fuchsia.io/Ioctl
    * fuchsia.tracelink/RegisterTraceProviderDeprecated
    * fuchsia.modular/GetAuthenticationContext
    * fuchsia.modular/GetActiveLinks
    * fuchsia.modular/Duplicate
    * fuchsia.modular.module/ use StartOngoingActivity instead
    * fuchsia.mediaplayer/SetReaderSource
    * fuchsia.ui.viewsv1/AddChild
    * fuchsia.ui.viewsv1/RemoveChild
    * fuchsia.ui.viewsv1/CreateView
    * fuchsia.testing.runner/Fail
    * fuchsia.netstack/GetNodeName
    * fuchsia.netstack/SetRouteTable
* On fields
    * fuchsia.modular/CreateStory -- module_url argument
    * fuchsia.modular/CreateStoryWithInfo -- module_url argument
    * fuchsia.modular.intent/ json -> entity_reference
* On interface
    * fuchsia.simplecamera.SimpleCamera
    * fuchsia.sys.ServiceProvider
    * fuchsia.media.AudioOut
    * fuchsia.media.AudioIn

## Design

Recommend and document the use of the `[Deprecated]` attribute.
Optionally, a note can be added to provide an explanation about the deprecation
and preferred alternatives, e.g.
`[Deprecated = "explanation"]`.

No change to the FIDL compiler.
While we may want to have deprecation warnings appear for uses of deprecated
types or messages, especially across library boundaries, we are choosing a
minimal implementation to start.
This is motivated by both wanting to see how these `[Deprecated]` attributes are
used in practice, and avoiding complexity in the compiler that we are not
certain we need in the future.

Change the various language backends, as described in detail in the
[next section](#specifics-on-placement-of-deprecation-in-target-languages):

* In Rust, add `#[deprecated]` or `#[deprecated(note = "explanation")]` where
  appropriate.

* In Dart, add `@Deprecated` where appropriate.
  Consider also adding automatic comment if an explanation is provided.

* In Go, add comment `// Deprecated.` or `// Deprecated: explanation.` where appropriate.
  (See the [three recommended forms][three-recommended-forms].)

* In C++, add `[[deprecated]]` or `[[deprecated("explanation")]]` where
  appropriate.

Lastly, we would want to document this feature.
A good place would be to discuss this under "Good Design Patterns" in the
[API Rubric][fidl-api].

### Specifics on Placement of Deprecation In Target Languages

In FIDL                                | In Target Language
---------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------
Type alias (e.g. using foo = bar;)     | No impact, currently type aliases are a frontend only concern, and are not represented in the JSON IR.
Const declaration                      | On the constant being defined, so as to warn on uses of the constant.
Message (e.g. struct, union, table)    | Annotation on the top-level type (class/struct) representing the FIDL message, i.e. the type used by end-developers.
Field (e.g. struct field, table field) | On the field of the type representing the specific FIDL field, and/or on all accessor methods for this field (e.g. ClearXxx, SetXxx, etc.)
Method, Event, or Interface            | Place on client facing objects/function (e.g. proxy) but not on service client facing objects/functions (e.g. not on stub); see [note below](#note).

#### Note

We could introduce `[DeprecatedForClients]` and `[DeprecatedForServices]`
attributes to control which side is deprecated, but uses show deprecation is
mostly to inform consumers.


### Deprecation as an Error

Depending on build settings and pragmas, the deprecation annotation in target
languages are raised to errors.

In Rust for instance, the [`#deprecated`][rust-deprecated] attribute raises a warning.
However, this is often coupled with a crate-level [`#![deny(warnings)]`][rust-deny-warnings]
which elevates all warnings to errors.
This in turn forces users of deprecated functions, variables, and methods to
specify an [`#allow(deprecated)`][rust-allow-deprecated] at point of use.
This specific use-site properly documents the intention to knowingly use
deprecated code.

As another example, support for deprecation warnings in Go is not as direct,
and one needs to turn to third-party tools, such as [staticcheck.io][go-staticcheck].

As a result, FIDL library authors should be aware that the introduction of a
`[Deprecated]` attribute is a source-level breaking change, in most cases
requiring new annotations on the part of users of the library.

## Implementation strategy

Two observations:

* Each language backend can be implemented independently.
* The `[Deprecated]` attribute can be introduced in various .fidl files
  independently of backend support.

The suggested strategy would be to start using the `[Deprecated]` attribute in
various .fidl files, by converting ad-hoc annotations to this proposed
attribute.

In separate changes, tackle Dart, Rust, and C++ since they have some target
language support.

For Go, we would want to implement this change along with the use of [doc
comments][go-doc-comments].
(Especially since a deprecation notice would need to be properly fused with a
doc comment, the general style is to have doc comments, a line break, and then
the deprecation notice.)

As for documentation, this change should occur soon after the use of this
attribute in .fidl files, or after the first implementation in one language
backend.

## Documentation and examples

Add a 'Deprecation' sub-section under the "Good Design Patterns"
section of the [API Rubric][fidl-api].
In addition, [document this attribute along with others][attributes].

## Backwards compatibility

No impact.

## Performance

No impact.

## Security

No impact.

## Testing

Testing would be done at each backend code gen level.

## Drawbacks, alternatives, and unknowns

The implementation cost of this proposal is measured, and can be done in each
language backend one at a time.
Furthermore, the existence of this convention will already clarify how to
indicate deprecation, and provide guidance on annotate existing FIDL files.

As an alternative, we can choose not to implement anything, and not provide
support for any deprecation indication.
Not doing anything allows us to avoid committing at this time to one specific
way to indicate deprecation, especially before we see more usage of this.
(A quick ack search turns up on the order of 20-25 places.)

We could also introduce a language keyword for deprecation, and have that as
part of the grammar.
This seems overly restrictive, and complicated, especially for a feature that
doesn't have semantic meaning beyond documentation.

## Prior art and references

Being able to describe deprecation, and point to alternatives, is a common
feature in multiple programming languages (some noted above).

In protobufs, deprecation is allowed [only on fields][protobufs-only-on-fields]: "If set to true,
indicates that the field is deprecated and should not be used by new code.
In most languages this has no actual effect.
In Java, this becomes a @Deprecated annotation.
In the future, other language-specific code generators may generate deprecation
annotations on the field's accessors, which will in turn cause a warning to be
emitted when compiling code that attempts to use the field.
If the field is not used by anyone and you want to prevent new users from using
it, consider replacing the field declaration with a reserved statement."

Mojo and Thrift do not seem to have any feature like this.

Flatbuffers, [only on fields][flatbuffers-only-on-fields]: "do not generate accessors
for this field anymore, code should stop using this data."

<!-- XREFS -->

[rfc-0083]: /contribute/governance/rfcs/0083_fidl_versioning.md
[attributes]: /reference/fidl/language/attributes.md
[donotuse1]: https://fuchsia.googlesource.com/fuchsia/+/caa3f20aa7b64240f4265ede5e6deddf0f2d0cf7/garnet/public/fidl/fuchsia.media/audio_renderer.fidl#7
[donotuse2]: https://fuchsia.googlesource.com/fuchsia/+/ce931e090d0c54030a80397bd24f217132983794/peridot/public/fidl/fuchsia.modular/module/module_context.fidl#79
[three-recommended-forms]: https://github.com/golang/go/issues/10909#issuecomment-136492606
[fidl-api]: /development/api/fidl.md#Good-Design-Patterns
[rust-deprecated]: https://doc.rust-lang.org/reference/attributes.html#deprecation
[rust-deny-warnings]: https://doc.rust-lang.org/reference/attributes.html#lint-check-attributes
[rust-allow-deprecated]: https://doc.rust-lang.org/reference/attributes.html#lint-check-attributes
[go-doc-comments]: /contribute/governance/rfcs/0055_documentation_comments.md
[go-staticcheck]: https://staticcheck.io/docs/checks#checks
[protobufs-only-on-fields]: https://developers.google.com/protocol-buffers/docs/proto3
[flatbuffers-only-on-fields]: https://google.github.io/flatbuffers/md__schemas.html
