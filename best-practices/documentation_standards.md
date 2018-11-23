# Documentation Standards

A document about what to document and how to document it for people who create
things that need documentation.

## Why document?

Fuchsia is a new operating system.  As it grows and new people join the project
so grows the need to provide effective documentation resources.

## Who is the audience?

The documentation described here is intended to address a technical audience,
i.e. those who expect to implement or exercise APIs or understand the internal
dynamics of the operating system.  These standards are not intended for
end-user product documentation.

## What should I document?

In brief, document your interfaces, introduce essential concepts, explain how
everything fits together.

- Conventions: e.g. this document about documentation, code style
- System Design: e.g. network stack, compositor, kernel, assumptions
- APIs: e.g. FIDL interfaces, library functions, syscalls
- Protocols: e.g. schemas, encodings, wire formats, configuration files
- Tools: e.g. `bootserver`, `netcp`, `fx`
- Workflows: e.g. environment set up, test methodologies, where to find various
  parts, how to get work done

## Where should I put documents?  What goes where?

Keep your documentation assets in the source tree near the things they
describe.  Where this should go depends on the type of document and its topic.
(See following sections for details.)

### Prose Documentation

Prose documentation is especially effective at explaining big ideas and
demonstrating how to perform particular tasks.  It's great for documenting
system design, protocols, tools, conventions, workflows, and tutorials.

Prose documentation should be written in Markdown format and published as a
file with the extension `.md` in the source repository.

Preferred locations:

- Documents about a specific project should go into the `docs` folder at the
  root of that project's repository and be arranged by topic.
  e.g. `//apps/my-project/docs/my-feature.md`
- Documents about Fuchsia as a whole should go into the top-level `docs`
  repository itself.  e.g. `//docs/build_packages.md`

Alternate locations:

- Adding a `README.md` to the root of a project's repository may serve as a
  brief orientation to the project for first time visitors but this is not
  required.

Tips for writing effective prose documentation:

- Write plain English text.
- Optimize the experience of first time readers.
- Give each document a clear title.
- Briefly describe the purpose and underlying assumptions of each part.
- Be sure to define your jargon; refrain from excess abbreviations.
- Include links to other relevant documentation.
- Stay on topic.
- Use section headers to organize ideas.
- Keep the tone informal.
- Don't restate API documentation which is already published elsewhere (e.g. as
  documentation comments)

### Documentation Comments

Documentation comments are especially effective at describing the purpose of
interfaces, structures, methods, data types, and other elements of program
code.

Documentation comments should be applied consistently to all public APIs since
they are a valuable asset for SDK consumers.

Tips for writing effective documentation comments:

- Write plain English text.
- Write complete sentences and paragraphs.
- Keep comments clear and brief, no more than a few sentences.
- Follow the approved style guide for your programming language.
- Always add value; don't restate what is already indicated by the type
  signature.
- Describe units of measure and integrity constraints of variables.
- Link to prose documentation for more elaborate descriptions of how APIs fit
  together as a whole.

### Breadcrumbs

Documentation is only useful when your audience can find it.  Adding links to
or from existing documentation artifacts greatly improves the chances that
someone will read it.

Tips for leaving breadcrumbs:

- Top-down linkage: Add links from more broadly scoped documents to more
  detailed documents to help readers learn more about particular topics.  The
  [Fuchsia book](../the-book/README.md) is a good starting point for top-down
  linkage.
- Bottom-up linkage: Add links from more detailed documents to more broadly
  scoped documents to help readers develop more awareness of the overall
  context of the topics being discussed.  Adding links from module, class, or
  interface level documentation comments to higher level prose documentation
  overviews can be particularly effective.
- Sideways linkage: Add links to documents in related subject domains with
  which a reader should familiarize themselves in order to better understand
  the content of your document.
