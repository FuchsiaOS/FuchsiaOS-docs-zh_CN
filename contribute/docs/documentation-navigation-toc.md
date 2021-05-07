# Updating site navigation and TOC files

The table of contents is the list of documents that is displayed on the left
side of every page on fuchsia.dev. It is represented by a hierarchical set of
`_toc.yaml` files. The top level `_toc.yaml` file is
[`_toc.yaml`](https://fuchsia.googlesource.com/fuchsia/+show/HEAD/docs/_toc.yaml).

If you add a new document to fuchsia.dev, or if you move existing documentation within the site,
you need to change the site navigation, defined in
`_toc.yaml` files. These files are located in the same directory of
the documentation file or in a parent directory.

## Existing document

To change the documentation navigation for an existing document:

1. Locate the corresponding `_toc.yaml` file for the document in the source code
   tree.

   For example, if you want to modify the navigation for the
   [concepts page of Zircon](/docs/concepts/kernel/concepts.md),
   you can see that there is a
   [`_toc.yaml`](https://fuchsia.googlesource.com/fuchsia/+show/HEAD/docs/concepts/kernel/_toc.yaml)
   file in the same directory.

1. Edit the `_toc.yaml` file.
   You have to specify the published location of the document in the
   `_toc.yaml` files instead of the actual path in the Fuchsia source
   code. See [`_toc.yaml` reference](#toc-reference).

## New document

To add navigation for a new document:

1. Locate the closest `_toc.yaml` file for the document.
   If the directory where you created
   the document has a `_toc.yaml` file, use that file. If not, navigate through
   the parent directories until you locate the closest `_toc.yaml` file.

1. Edit the `_toc.yaml` file.
   See [`_toc.yaml` reference](#toc-reference).

## `_toc.yaml` reference {#toc-reference}

A `_toc.yaml` file can contain single entries or expandable sections
with multiple entries:

* Single entry

  A single entry in the table of contents navigation is represented by a title
  and a path in the corresponding `_toc.yaml` file. Each entry must also use
  the correct indentation like the other entries in `_toc.yaml`.

  Paths must follow these requirements:

  * Paths to files should be the full path from the root of the project. For
    example, `/docs/development/api/cli.md`.
  * Paths to directories should not include a trailing slash, and the directory
    must have a file named `README.md`.

  For example, to add an entry for the Zircon `concepts.md`
  page in its respective [`_toc.yaml`](https://fuchsia.googlesource.com/fuchsia/+show/HEAD/docs/concepts/kernel/_toc.yaml),
  you should add an entry:

  ```
  - title: "Kernel concepts"
    path: /docs/concepts/kernel/concepts.md
  ```

* Expandable section

  An expandable section is an expandable group of multiple entries in a table
  of contents. For example, see the expandable sections, such as Networking
  and Graphics, in the
  [Concepts section](/docs/concepts/index.md). Each expandable
  section has an arrow to the left of the section name.

  You can create a group of entries with a `section` element. Each section must
  also use the correct indentation like the other entries in `_toc.yaml`. Then,
  you can add single entries to the section.

  For example, to add a section in the "System" table of contents
  [`_toc.yaml`](https://fuchsia.googlesource.com/fuchsia/+show/HEAD/docs/concepts/_toc.yaml),
  add a `section` group and its corresponding entries. Usually the entries are included from another _toc.yaml file:

  ```yaml
  - title: "Zircon kernel"
    section:
    - include: /docs/concepts/kernel/_toc.yaml
  ```

Once you have made these changes, you can submit your changes for review.

