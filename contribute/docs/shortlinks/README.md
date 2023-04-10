# Shortlinks

Note: Shortlinks on fuchsia.dev (fuchsia.dev/go/keyword) were implemented
based on [RFC-0125][RFC-0125].

## Create a shortlink

Before you create a shortlink, review the shortlink [rubric] to determine if
your shortlink proposal meets the requirements.

1. Review the [`/docs/go/_redirects.yaml`][redirects-file] file to
   make sure that the proposed keyword doesn't already exist.
1. If the keyword is available, append the following to
   [`/docs/go/_redirects.yaml`][redirects-file]:

   Note: You can use the [GIT source editor][git-source-editor] to propose a
   shortlink from a browser.

   ```yaml
   - from: /docs/go/{{ '<var>' }}keyword{{ '</var>' }}
     to: /docs/{{ '<var>' }}path_of_shortlink{{ '</var>' }}
   ```

1. Send your change for review.
1. Add a shortlink reviewer to review your change.

   Note: [See a list of available reviewers][OWNERS-cs].

If your proposal is rejected, a reviewer will list the reason for the
rejection and include a rationale.

If your proposal is approved, you may submit your shortlink. Once the change
is rolled into the build, the shortlink will go live on fuchsia.dev. The link
will resolve at both `fuchsia.dev/go/<keyword>` and
`fuchsia.dev/fuchsia-src/go/<keyword>`.

## Become a reviewer

Reviewers are expected to meet the following criteria:

 * Reviewers must be knowledgeable about Fuchsia. Typically,
   contributors acquire this knowledge by working on the project for a
   substantial amount of time and by interacting with multiple parts of the
   system.

 * Reviewers must enforce the shortlink criteria detailed in the [rubric].

If you meet the review criteria, you can create a change to propose yourself
as a member by editing the [OWNERS] file of the `/docs/go/` directory.


[git-source-editor]: https://ci.android.com/edit?repo=fuchsia/fuchsia/main&file=docs/go/_redirects.yaml
[redirects-file]: https://cs.opensource.google/fuchsia/fuchsia/+/main:docs/go/_redirects.yaml
[RFC-0125]: /docs/contribute/governance/rfcs/0125_shortlink_fuchsia_dev.md
[rubric]: /docs/contribute/docs/shortlinks/rubric.md
[OWNERS]: https://ci.android.com/edit?repo=fuchsia/fuchsia/main&file=docs/go/OWNERS
[OWNERS-cs]: https://cs.opensource.google/fuchsia/fuchsia/+/main:docs/go/OWNERS