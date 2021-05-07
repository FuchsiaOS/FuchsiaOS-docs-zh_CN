# Documentation Types

Documentation is an important part of any product or feature because it lets users know how to
use a feature that has been implemented. This document is a quick and easy
reference for types of documentation.

For information on general documentation standards, including file types, locations, and general
tone, see the [Fuchsia documentation standards][doc-standard].
For specific guidance on word choice, style, and structure, see the
[Fuchsia documentation style guide][style-guide].

## Procedures (guides), concepts, or reference documentation

Most documentation can be divided into these categories:

- [Procedures (guides)](#procedural-documentation)
    - Get-started - Documentation that provides a step-by-step process for setting up a part of
      the Fuchsia developer environment, such as downloading and building Fuchsia. These are located
      under `/docs/get-started`.
    - Development, or Guides - Documentation that provides a step-by-step process for completing
      any task related to Fuchsia. These are all located under `/docs/development/`.
- [Concepts](#conceptual-documentation) - Documentation that helps you understand a concept such
  as mods in Fuchsia. This type of documentation is located under `/docs/concepts`.
- [Reference](#reference-documentation) - Documentation that provides a source of information about
  parts of a system such as API parameters or FIDL. These are located under `/docs/reference/`. Much
  of the reference documentation is auto-generated.

**You should write a procedural document** if you plan on explaining to a user how to use a specific
feature and are able to guide a user through simple numbered steps. Procedural documents tend to
reinforce the concepts that were explained in a conceptual document by giving one or more
examples that might be useful for users.

**You should write a conceptual document** if you plan on explaining a concept about a product.
Conceptual documents explain a specific concept, but for the most part they do not include actual
examples. They provide essential facts, background, and diagrams to help your readers build a
foundational understanding of a product or topic. You should not explain industry standards that
your audience should be familiar with, for example, TCP/IP. You might explain how this concept ties
in with your feature, but you should not explain the basics behind that industry standard concept.

**You should write a reference document** if you need to provide information about parts of a system
including, but not limited to APIs and CLIs. Reference documentation should allow the user to
understand how to use a specific feature quickly and easily.


Note: A feature may require more than one type of document. You may decide that your feature
requires just reference documentation or that you need several procedural docs, a conceptual doc,
and reference documentation.


## Procedural documentation {#procedural-documentation}

Procedural documentation should try to be brief and each task within your documentation should try
to avoid going above 10 steps. You should divide long procedures into multiple sub-tasks to try to keep tasks
manageable for a user.

For example, if you wanted to write a procedural document for taking care of
a dog, you might have a table of content that looks like this:

How to take care of a dog:

- Feed a dog
- Wash a dog
- Trim a dog's nails
- Brush a dog's hair


### General procedural documentation guidelines

- Each task or subtask should have a paragraph that lets a user know what the task is about and
  what a user should be able to do after performing the task or subtask.
- Use screenshots or graphics to assist a user in navigating a user interface (UI).
- A procedural document should not have to explain any concepts to a user, but should reference
  conceptual documents in case a user does not know about a certain concept. For example, a
  procedure with a reference to a conceptual document might look like this:

   Configure the server with the appropriate configuration. For more information about server
   configurations, see "server configuration".

- Avoid giving the users multiple paths to select when working through procedures. When you avoid
  giving the user choices, your documentation should lead all users to the same end result.
- If a procedural document is meant for beginner users, avoid adding procedures that you might
  consider better suited for advanced users. If your document is intended for advanced users, state
  that up front and give them a list of prerequisites before they go through your how to or codelab.
- If you are incorporating code samples in your procedural documentation,
  review the best practices detailed in [Code sample style guidelines][code-sample-style-guide]

## Conceptual documentation {#conceptual-documentation}

Conceptual documentation should try to be brief and for the most part should not go above 1 page.
If you need to write more than one page to describe a concept, consider breaking that concept into
sub-concepts by using headings. By keeping your document brief you achieve the following:

- You do not overwhelm your reader with a wall of text.
- You avoid losing the reader while they read your document.

The first paragraph should try to be a brief summary of your document, this should allow the user to
quickly read through it, determine what the document covers, and if this is relevant to what they
want to learn. If your document has multiple headings, you should include a bulleted list with the
high-level headings after this first paragraph.

You should use graphics, images, or diagrams to reinforce certain concepts. The text that comes
before and after the graphic should explain what the graphic shows. Images should be saved in
a feature specific 'images/' directory or a common 'images/' directory. You should also save
the source file of your images in a 'images/src/' directory.

Good conceptual documentation usually includes:

- **Description** rather than instruction
- **Background** concepts
- **Diagrams** or other visual aids (preferably in .png format)
- **Links** to procedureal and/or reference docs

After writing your document, it is good practice to proofread the document, put yourself in the
user's shoes (no longer being the expert that developed the feature), and try to answer these
questions:

- Does the information in the document explain the concept completely?
- Is there information that is not needed for this concept? If so, remove it.
- Is there unnecessary detail about how things might work in the background?
- If I am the user, is there additional I would have liked to know?

Then, if these questions aren't fully answered, edit your document again.

## Reference documentation {#reference-documentation}

Reference documentation should provide information about parts of a system including, but not
limited to APIs and CLIs. The style of reference documentation should be the same for all reference
documentation of that type. For example, API documentation should define all of the API's parameters,
indicate if a parameter is required or optional, and show examples of the use of the API. These
examples should be very generic and simple. If you feel like you need a more elaborate example,
consider creating a procedural document to reinforce your reference documentation.

For the style guide for API documentation, see the
[API style guide][api-style].


<!-- Reference links -->

[doc-standard]: /docs/contribute/docs/documentation-standards.md
[style-guide]: /docs/contribute/docs/documentation-style-guide.md
[code-sample-style-guide]: /docs/contribute/docs/code-sample-style-guide.md
[api-style]: /docs/concepts/api/documentation.md
