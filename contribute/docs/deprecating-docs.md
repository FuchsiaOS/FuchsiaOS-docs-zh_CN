# Deprecating documentation

As Fuchsia evolves, there is a need to deprecate documentation for deprecated
features or out-of-date documentation.

## Establish a deprecation timeline

Before removing documentation, it is important to establish a deprecation
timeline and notify users. In general, it is recommended to remove documentation
after a deprecation period of 6 months.

To begin the deprecation process and notify users:

* [Deprecate a document in its markdown file](#deprecate-doc)
* [Deprecate a document in the navigation](#deprecate-toc)

## Deprecate a document in its markdown file {#deprecate-doc}

To mark a document as deprecated in markdown file:

1. Locate the title of the page which is prefixed with a `#`. For example:

   ```none {:.devsite-disable-click-to-copy}
   # Deprecating documentation
   ```

1. Add the following `include` statement below the title of the document. For
   example:

   ```none {:.devsite-disable-click-to-copy}
   # Deprecating documentation
   {% verbatim %}
   {% include "docs/_common/_deprecation_notice.md" %}
   {% endverbatim %}
   ```

1. Include information about the deprecation for users such as the
   reasoning behind the deprecation. Also, include any new tools or features
   that may now be in place instead of the deprecated feature.
1. [Mark the document as deprecated in the navigation](#deprecate-toc).

## Deprecate a document in the navigation {#deprecate-toc}

Note: For more information on working with the fuchsia.dev navigation, see
[Updating site navigation and TOC files](documentation-navigation-toc.md).

To mark a document as deprecated in a `_toc.yaml` file:

1. Locate the `_toc.yaml` file that references the documentation that you are
   deprecating. For example:

   Note: By convention, the `_toc.yaml` file should be in the same directory
   as the markdown file that you are deprecating.

   ```none {:.devsite-disable-click-to-copy}
   - title: "Deprecating documentation"
     path: contribute/docs/deprecating-docs.md
   ```

1. Add a key/value pair of `status: deprecated` under the title of the page. For
   example:

   ```none {:.devsite-disable-click-to-copy}
   - title: "Deprecating documentation"
     status: deprecated
     path: contribute/docs/deprecating-docs.md
   ```

1. Submit the changes (document and TOC deprecation) to the Fuchsia repository.

## Redirect the pages to the deprecation notice

After the deprecation timeline has passed, delete the pages and redirect them.

To delete the pages and redirect:

1. Search for any links that reference the pages that you are removing. For
   example:

   ```
   grep -r "contribute/docs/deprecating-docs.md" ~/fuchsia/docs/
   ```

   This lists all the documents that link to the page from the `grep` command.

1. Update or remove any links in documentation that references the pages that
   you are deprecating.

1. Use `git rm` to remove the documents that you are deprecating. For example:

   ```
   git rm docs/contribute/docs/deprecating-docs.md
   ```

1. Locate the `_toc.yaml` files where the documents are referenced and remove
   the entries for the deprecated documents.

1.  Ensure that `doc-checker` passes. Run `fx format-code` to run doc-checker:

    ```
    fx format-code
    ```

   Fix any issues that `doc-checker` may signal.

1. In the `[/docs/_common/_deprecate-docs.yaml][deprecate-docs]` file, create a redirect for
   the deprecated pages to the [deprecation notice][deprecation-notice] page.
   Also include a comment listing the deprecate feature with a deprecation date.
   For example:

   Note: # Using ? or * is not supported in the redirect links.

   ```
   # May 13th, 2022
   # Deprecating documentation around deprecation
   - from: contribute/docs/deprecating-docs.md
     to: contribute/docs/deprecation-notice.md
   ```

1. Submit the changes to the Fuchsia repository.


[deprecate-docs]: /docs/_common/_deprecate-docs.yaml
[deprecation-notice]: contribute/docs/deprecation-notice.md
