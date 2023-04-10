# Fuchsia SDK Contributor Guide

This section includes documentation for contributing to the [Fuchsia API
Surface][fuchsia-api-surface] and the [IDK].

_Technically_, it may be more appropriate to call this the "IDK Contributor
Guide", as the APIs and libraries that make up the API Surface are first added
to the IDK, which is then turned into an SDK distribution. However,
colloquially and in code, we almost exclusively say "the SDK". Regardless, if
you're looking to "change the SDK" in some way, you're probably in the right
place.

## Contributing to an API in the SDK

To contribute to the [Fuchsia API Surface][fuchsia-api-surface], do the following:

*  Evaluate whether your change is large or small.

   *  If you have a small, incremental change to the API, contribute your
      change by completing the steps in
      [Create a change in Gerrit][create-a-change-in-gerrit], as you would for
      any Fuchsia source code change.
   *  If you have a large change to the API, that is, a change that
      significantly expands on the function of the API or modifies the
      API extensively, do the following:

      *  Create an [RFC][rfc] that explains the design of your modification
         to the API.
      *  This RFC should be reviewed through the normal [RFC process][rfc-process].
         The API reviewer for the relevant area should be a stakeholder in the RFC. See
         the [Fuchsia API Council Charter][api-council] to identify API reviewers.
      *  After your API RFC is approved, contribute your change by completing the steps
         in [Create a change in Gerrit][create-a-change-in-gerrit], as you would
         for any Fuchsia source code change.

* [Request a code review][request-a-code-review] from an API council member. Select
  your API council reviewer based on the area of the Fuchsia API that you're modifying.
  For a list of API council members and their areas of focus, see [Membership][membership]
  in the Fuchsia API Council Charter.

<!-- Reference links -->

[fuchsia-api-surface]: /docs/glossary/README.md#fuchsia-api-surface
[IDK]: /docs/development/idk/
[create-a-change-in-gerrit]: /docs/development/source_code/contribute_changes.md#create-a-change-in-gerrit
[request-a-code-review]: /docs/development/source_code/contribute_changes.md#request-a-code-review
[rfc]: /docs/contribute/governance/rfcs/TEMPLATE.md
[rfc-process]: /docs/contribute/governance/rfcs/rfc_process.md
[api-council]: /docs/contribute/governance/api_council.md#area
[membership]: /docs/contribute/governance/api_council.md#membership
