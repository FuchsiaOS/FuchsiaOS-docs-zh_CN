# Documentation standards overview

This document outlines standards, structure, tone, and best practices for Fuchsia documentation.

## Document locations

  * **Documentation specific to developing a specific Fuchsia feature:**
    Documentation for developers creating or maintaining a specific part of the Fuchsia codebase
    should be kept in the same directory as the source code. These docs are usually in the form of
    `README.md` files embedded throughout the Fuchsia codebase.
  * **General documentation for Fuchsia developers:** Fuchsia documentation should
    be created in <code>/HEAD/docs/</code>.
    In the `/docs/` directory, you should create documentation in one of these sub-directories:

    * <code>get-started</code>:
       Specific guidance to download, set up, and start developing on Fuchsia should go in
       `/get-started`. This content should contain opinionated, short tutorials that help new
       users get started on Fuchsia, and link to additional documentation in Fuchsia.dev.
    *  <code>development</code>:
        The `/development/` directory (which displays on the site as "Guides") contains
        instructions and tutorials for developers
        working on Fuchsia. This directory includes documentation
        on how to build, run, and test Fuchsia.
    *  <code>concepts</code>:
        The `/concepts` directory contains in-depth explanations of specific features of
        Fuchsia and how they work, including operating system overviews, frameworks, architecture,
        and packages.
    *  <code>reference</code>:
        The `/reference/` directory contains generated reference docs on Fuchsia tools and APIs,
        including FIDL and kernel reference.
    *  <code>contribute</code>:
        The `/contribute/` directory contains code and documentation contribution processes and
        best practices, including documentation code and style guides, code polcies, and governance.
    *  `images`
        The `/images/` directory contains images used in the documentation. You should
        place images in this common directory.

## Document types

Most documentation can be divided into the following categories:

- [Procedural](documentation-types.md#procedural-documentation)
    - Getting started - Initial setup documentation
    - Guides - Task-oriented documentation
- [Conceptual](documentation-types.md#conceptual-documentation) - Foundational
  documentation focused on teaching more about Fuchsia, Fuchsia architecture, and Fuchsia components
- [Reference](documentation-types.md#reference-documentation) - Documentation focused on
  detailing the syntax and parameters of Fuchsia APIs and tools. This documentation is usually
  auto-generated.

See [Documentation Types](documentation-types.md) for more information.

## Documentation and code style guides

It's important to follow documentation style guidelines to ensure that the documentation
created by a large number of contributors remains consistent. See the
[Documentation style guide](documentation-style-guide.md) for specific documentation guidance and
[Code sample style guide](code-sample-style-guide.md) for code sample guidance.

## Search best practices

Documentation is only useful when users can find it. Some findability and search best practices
include the following:

- Add your document to the table of contents: Add links to documentation in the left sided
  navigation on fuchsia.dev. See [Site navigation and TOC files](documentation-navigation-toc.md)
  for more information.
- Cross-link documentation: Add links to documents on subjects that help readers better understand the
  content of your document. For example, the conceptual document for the [Fuchsia emulator](/development/build/emulator.md)
  links to relevant guides and getting started documents about the Fuchsia emulator.
- Use consistent terminology: If you're writing about a specific concept in Fuchsia, verify that you are
  using consistent terminology. Use the [glossary](/glossary/README.md) to verify terms.

## Documentation file formats and file names

All documentation for Fuchsia is written in Markdown (`.md`), and Fuchsia.dev
uses the [Hoedown Markdown Parser](https://github.com/hoedown/hoedown).

The site's navigation is configured by `_toc.yaml` files, which are included in every documentation
directory. Use the guidance in
[Site navigation and TOC files](documentation-navigation-toc.md) to update these files.

File and directory names should be lowercase, and separate words with hyphens, not underscores.
Use only standard ASCII alphanumeric characters in file and directory names. If the file name
contains a command with an underscore, then you can include the underscore.

## General guidance on style and tone

- **Write in plain U.S. English.** Write in clear, direct U.S. English that makes content
  easy to understand. Use simple words, be concise, and use contractions like _it's_ or _don't_.

- **Be respectful.** Follow the guidelines set forth in [Respectful Code](/contribute/respectful_code.md).

- **Write in second-person ("you").** Fuchsia documentation is written to the user ("you"). When
  For example, "You can install Fuchsia by doing the following...". Do not refer to the reader in the
  third person ("Fuchsia users can install Fuchsia by...") or use
  "We" ("We can install Fuchsia by...").

- **Write in present tense.** Always document the system as it is, not as it will be. Words such
  as "will" are very ambiguous. For example "you will see" leads to questions like "when will I see
  this?" In 1 minute or in 20 minutes? In addition, do not refer to future product features unless
  necessary. Mentioning future plans that might not happen becomes a maintenance burden.

- **Keep sentences short and concrete.** Using punctuation allows your reader to follow
  instructions and understand concepts. Also, short sentences are easier to translate.

- **Know your audience.** Define your audience before you write a document. Knowing your audience
  allows you to understand what information your audience should be familiar with. When a document
  is meant for a more advanced audience, state that up front and let users know that as a
  prerequisite before reading your document.

- **Use active voice.** Try to write in active voice since passive voice can
  make sentences ambiguous and hard to understand. Here's an example:
  - Active voice: "The operating system runs a process." In this case, the subject performs the
    action denoted by the verb.
  - Passive voice: "A process is being run." The subject is no longer _active_, but is being acted
    upon by the verb — it's _passive_.
  In most cases, if you use "by" this indicates that your sentence might be still be in passive
  voice.

- **If you use acronyms, define them the first time you write about them.** For
  example, looks good to me (LGTM). Don't assume that everyone will understand all acronyms. You do
  not need to define acronyms that are industry standards such as TCP/IP.

- **Define technical terms and avoid jargon.** Fuchsia documentation should be accessible
  to all levels of developers. Avoid overcomplicating documentation with uncommon or highly
  technical words. If you're using Fuchsia-specific terms, define them in
  the [glossary](/glossary/README.md). Avoid invented words.

- **Avoid colloquial phrases or regional idioms.** Keep in mind that many Fuchsia users
  may not be native English speakers. Avoid difficult to translate idioms, like
  "that's the way the cookie crumbles." While it might make sense to you, it doesn't translate
  well into other languages.

- **Avoid referencing proprietary information.** This can refer to any potential terminology or
  product names that may be trademarked or any internal information (API keys, machine names, etc…)
  internal to your company.

- **Use gender-neutral pronouns.** Don't use _he, him, his, she,_ or _her,_ and don't use _he/she_ or
  _(s)he_ or other such punctuational approaches. Instead, use the singular _they._

- **Use consistent terminology.** Ensure that terms are consistent in code, UI, and documentation.
  Use common terms when possible, and use the [glossary](/glossary/README.md) to verify terminology.