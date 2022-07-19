<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0125" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

Allow the use of shorter, meaningful, and durable links to specific Fuchsia
artifacts. These shortlinks ensure links won't degrade over time.

The proposed format is fuchsia.dev/go/<keyword> and would map to any
fuchsia.dev document or a specific file in the Fuchsia source tree.

For example, fuchsia.dev/go/components could redirect to a page
such as fuchsia.dev/fuchsia-src/concepts/components/v2.

In most cases, these links should not be changed; however, there may be
exceptions, such as when the Component framework version change. The
go/link allows for a seamless transition between these version changes.

## Motivation

Shorter links are useful for things such as rubrics, specifications,
documentation, FAQs, supported hardware, and more. They are also helpful
with version updated, for example, Components v2.

This can also help with versioning of certain core parts of Fuchsia such
as Components.

## Stakeholders

Facilitator: hjfreyer.

Reviewers: mkearney, curtisgalloway.

Consulted: Members of FEC.

Socialization: RFC draft was shared with the technical writing team.

## Design

The design involves creating a directory in the source tree in `docs/go/`
to place a `_redirects.yaml` file.

This directory will include a `README.md` file to explain how to use the
redirects. It will also have an `OWNERS` file to review and approve the new
shortlinks. The `OWNERS` would also need to approve any possible changes to
existing `go` links. These changes should only be approved in very particular
cases, such as a replacement of a core concept, such as a new version of the
component framemwork.

The format of redirects would be in the following format:

```
- from: /go/<keyword>
  to: <path in source tree>
```

Example:

Note: This syntax can also point to source files outside of the `docs/`
directory.

```
- from: /go/drivers
  to: /reference/hardware/drivers.md
```

This redirect results in <docs/go/drivers> and points to
<docs/reference/hardware/drivers>.

## Implementation

Create three changes:

* A gerrit change that includes the new `docs/go` directory along with
  `_redirects.yaml`, `README.md`, and `OWNERS` files.
* A gerrit change that updates `doc_checker` for testing. See
  [Testing](#Testing).
* An overall redirect for fuchsia.dev to redirect fuchsia.dev/fuchsia-src/go/
  to fuchsia.dev/go. fuchsia/src/go/ would be the path for `docs/go` on
  fuchsia.dev.

## Performance

Note: Only applies to fuchsia.dev.

This would result in a minor latency increase due to the redirect.

## Ergonomics

This enables contributors to make easy to remember keywords for Fuchsia
documentation.

## Backwards Compatibility

N/A.

## Security considerations

Technically, a `_redirects.yaml` file can be added in any of the
sub-directories of the `/docs/` directory to make redirects. However, the scope
of a `_redirects.yaml` file is limited to the directory in which the file
lives; there should not be an issue where other parts of fuchsia.dev get
redirected.

This proposal creates a `/go/_redirects.yaml` file which limits the
redirect scope to only fuchsia.dev/go/. In the background, a higher-level
redirect takes care of redirecting fuchsia.dev/fuchsia-src/go to
fuchsia.dev/go. Keep in mind that once the git repo is moved into fuchsia.dev,
the `fuchsia-src/` directory is mapped to the `docs/` directory.

## Privacy considerations

N/A.

## Testing

[`doc_checker`](https://cs.opensource.google/fuchsia/fuchsia/+/main:tools/doc_checker/)
can be expanded to verify that:

* The `- from:` fields only have `/go` paths to avoid issues.
* The `to:` links exist.
* The other .md files in `/docs/` are not referencing broken `/go/` links.
* Problematic words or phrases are not used in the names of links.
* No duplicate `go` keywords exist.

## Documentation

Create a `README.md` that explains the syntax for the file along with the
following items:

* Rubric for making a shortlink. See [rubric](#rubric) For example, things
  that should have a permanent reference in case of a reorganization. Any
  reviewer could consult this rubric to determine the validity of proposed
  links.

### Rubric {#rubric}

Short links (fuchsia.dev/go) are intended to be durable, have a meaningful
name, and be of general interest. The links are durable and use a
flat namespace, so it is desirable to carefully consider each proposal
to limit the number of created short links.

Reviewers should consider the following factors to approve pending short
link requests:

* Will the proposed link be relevant for a long time?
  Links are intended to be durable, so you should be reasonably confident
  that a link will continue to be of general interest for a long time.

  * <span class="compare-better">Positive example</span>:
    A general Fuchsia FAQ page or the current list of supported hardware. Even
    though they will change over time, the topics will be relevant
    indefinitely.

  * <span class="compare-worse">Negative example</span>:
    An RFC proposal that is under consideration. While the proposal is
    currently relevant, it will be either accepted or rejected. If accepted,
    it may be a suitable short link topic for reference.

* Does the link cover a general topic or concept? Is it relevant for a wide
  audience?

  * <span class="compare-better">Positive example</span>:

      * An overview of FIDL. This is a fundamental technology used by Fuchsia.
      * The Fuchsia security model. This is of interest to both users and
      developers.

  * <span class="compare-worse">Negative example</span>:

      * Documentation for a single system call. A link to the list of system
        calls would be more relevant.
      * Instructions for booting a specific model of NUC over Ethernet. A link
        to a more general page on booting Fuchsia on categories of devices
        would be of interest to a wider audience.

* Does the link duplicate an existing easy-to-use link that is unlikely to
  change?

  * <span class="compare-better">Positive example</span>:
    <fuchsia.dev/fuchsia-src/get-started> is unlikely to change, so
    <fuchsia.dev/go/get-started> is probably redundant.

* Is the link name meaningful, and does it comply with the
  [Respectful Code policy](/contribute/respectful_code.md)?
  Short links should be, of course, short, but not at the expense of being
  understood.

  * <span class="compare-better">Positive example</span>:
    Both of these examples are short and descriptive:

      * `/go/faq` (renders as fuchsia.dev/go/faq).
      * `/go/hardware-specs` (renders as fuchsia.dev/go/hardware-specs).

* Does the `to` link point to a document inside of the `/docs/` directory?

  * <span class="compare-better">Positive example</span>:

      * `to: /concepts/software_model.md`.

  * <span class="compare-worse">Negative example</span>:

      * `to: http://google.com/`.

## Drawbacks, alternatives, and unknowns

N/A

## Prior art and references

Examples from link shortners such as:

* [http://bit.ly/](http://bit.ly/){: .external}
