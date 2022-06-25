# Commit message style guide

The [Git project provides guidelines](https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project),
including how to compose commit messages.
When writing a commit message, follow these additional guidelines:

+   [Add a required tag and optional subtag(s).](#add-required-tag)
+   [Add a paragraph.](#add-paragraph)
+   [Add an associated bug.](#add-bug)
+   [Optionally indicate multiple steps.](#indicate-multiple-steps)
+   [Add tests and run them automatically multiple times.](#add-tests)
+   [Add a buffer line before the Change-Id.](#add-buffer)
+   [Use Change-Id to refer to related changes](#use-change-id)

## Add required tag {#add-required-tag}

The required `[tag]` helps readers of the commit tell what subject your change is
about. The format is simply a keyword between brackets, for example, `[docs]`. The
keyword has no specific meaning, but should help readers identify the subject
easily. More specific tags or multiple tags can also be used to specify more
fine-grained subjects, for example,`[docs][fidl]`.
The following example shows required tags in the commit message subject:

```none {:.devsite-disable-click-to-copy}
[parent][component] Update component in Fuchsia

Write the details of a commit message here.

Bug: <issue-tracker-ID>

Test: Added test X.
```

You can view the commit history of the files you've edited to check for the tags
used previously. See these examples:

*   [https://fuchsia-review.googlesource.com/c/fuchsia/+/441776](https://fuchsia-review.googlesource.com/c/fuchsia/+/441776){:.external}
*   [https://fuchsia-review.googlesource.com/c/topaz/+/114013](https://fuchsia-review.googlesource.com/c/topaz/+/114013){:.external}

Commit message tags are required. If the subject of a commit message
doesn't include tags, Gerrit flags your
change with `Needs Label: Commit-Message-has-tags`.

## Add paragraph {#add-paragraph}

The paragraph underneath the header line describes in better detail what the
reason for the change is, and in general brief terms describe what it is
intended to do,
[for example](https://fuchsia-review.googlesource.com/c/fuchsia/+/569681):

```none {:.devsite-disable-click-to-copy}
[docs] Adding Fuchsia Commit message style guide

This change centralizes all commit message style guide into one style
guide. It also removes duplicate content from existing pages and points
to the new style guide instead.

Change-Id: I307e5b24df4273661d22c52c81038de50600c76c
```

## Add bug {#add-bug}

If you want Fuchsia Gerrit to know what issue this change is associated with,
you need to add the `Bug: <issue-tracker-ID>` line. To associate multiple issues
with a change, list each bug in a separate line. For example:

```none {:.devsite-disable-click-to-copy}
[parent][component] Update component in Fuchsia

Write the details of a commit message here.

Bug: 82657
Bug: 82658

Test: Added test X.
```

The difference between `Bug:` and `Fixed:` is that `Fixed:` automatically closes
the issue for you once your change is submitted, whereas `Bug:` only comments on
your issue once submitted. If you have multiple changes attached to your issue, use
the `Bug:` tag for all the changes up until the final change. Use `Fixed:` on
the final change, so that the issue is closed.

## Indicate multiple steps {#indicate-multiple-steps}

When executing a change that requires multiple steps across various repositories
(for instance, to soft transition APIs defined in one repository and used in
others), indicate multiple steps by referencing the last step taken and the next
step taken.

This enables reviewers and those looking at the log to understand and navigate
the totality of the change. When possible, it is encouraged to provide all steps
to complete the migration in each commit log (but that may be impractical in
some cases).

Here's an example of a [commit
message](https://fuchsia-review.googlesource.com/c/fuchsia/+/423314) with
multiple steps:

```none {:.devsite-disable-click-to-copy}
[fidl][go] Support for flexible enums (1/3)

Step 1 of 3. Adds support for flexible enums to fidlgen_go:

* For all enums, emit `IsUnknown()` method indicating whether the
  value is unknown (or known). While relevant only for flexible enums,
  it is possible to construct unknown strict enums using type casts.
* Emit an internal method `I_EnumIsStrict` indicating whether the enum
  is strict or flexible. This method is read by the runtime, when
  creating enum marshaler.
* For flexible enums, we generate a default unknown placeholder

Step 2: I1102f244aa5ab4545fab21218c1da90be08604ec
Step 3: If0a047a4db804a183e984676217b31e17b4af0ea

Test: fx test fidl_go_conformance at If0a047a4db804a183e984676217b31e17b4af0ea

Change-Id: Id71eb879e4d7dfabe228cc7b4e2fedb7f52db7b7
```

## Add tests and multiply line {#add-tests}

The `Test:` line is necessary to indicate what type of test to run to make sure
your change is working. You can add multiple different tests in this line, for
example, `fx test setui_service_tests, setui_client_interface_test`. You can
also add explanations of what tests you added below. If you did not add or
modify tests, you can specify `None:`, with an explanation of why it doesn't need
to be tested, for example, `None: documentation change only`.

If you added new tests, you can get deflake runs automatically by adding the
`Multiply:` line with the test to run multiple times.

The following example shows `Test:` and `Multiply:` in the [commit
message](https://fuchsia-review.googlesource.com/c/fuchsia/+/537303):

```none {:.devsite-disable-click-to-copy}
[SetUI] Correct internal items' visibility Part II

This CL marks some internal items with pub(crate), pub(super) or leaves
as private. Related CL: fxr/535942

Bug: 72941
Test: fx test -o setui_service_tests setui_client_interface_test
sample-setui-config-test setting-service-config-test
Multiply: setui_service_tests
Multiply: setui_client_interface_test

Change-Id: I67e061edee1e81a6875bf26b752ba5687c4ced71
```

Note: To add multiple to more than one test, you can either separate multiple
entries with commas on a single line, or add multiple `Multiply:` lines as shown
in the above example.

If the testing instructions are complex,
create an issue and provide a link to that issue in
the change description. If the change doesn't intend to change behavior,
indicate that fact in the commit message.

In some cases, certain behavior changes cannot be tested because Fuchsia lacks
some particular piece of infrastructure. If so, create an issue in the tracker
about the necessary infrastructure support and provide the bug number in the
change description, in addition to describing how the change is tested manually,
for example:

```none
Test: Manually tested that [...]. Automated testing needs US-XXXX.
```

Developers are responsible for high-quality automated testing of their code.
Reviewers are responsible for pushing back on changes that do not include
sufficient tests. See
[Fuchsia testability rubrics](development/testing/testability_rubric.md) for
more information on how to introduce testable and tested code in the Fuchsia
project.

## Add a buffer line before Change-Id {#add-buffer}

The Change-Id gets added automatically when you commit your changes. You may
want to add a buffer line between your other lines and the Change-Id, in case
you use `git commit --amend`, so that Gerrit doesn't parse the Change-Id as a
regular line, and add a second ID to it.

For more specific details, see the [Git interpret trailer
rules](https://git-scm.com/docs/git-interpret-trailers).

## Use Change-Id to refer to related changes {#use-change-id}

To reference another Gerrit change in a commit message,
always use the Change-Id.

Using the Change-Id is preferred since:

The git SHA is only known after a change is merged,
and while guidance could be given to use the Change-Id in one case,
and the git SHA in the other, prefer uniform guidance.
Furthermore, you cannot reference other repositories using the git SHA.

The link to the change is assigned by Gerrit,
and is not part of the persistent history of the repository.
Should the review mechanism change,
the Change-Id will continue to be part of the recorded history,
whereas the change's number will not.
There are also rare occurrences where change numbers may be lost,
for example, due to re-indexing issues.

For instance, to refer to the change that added
[RFC-0042](contribute/governance/rfcs/0042_non_nullable_types.md),
use `I32b966810d21a249647887fa45b61720ad01714c`,
and not the git SHA `5d40ee8c42d1b0e4d8b690786da12a0a947c1aaa`
or the link to the change,
https://fuchsia-review.googlesource.com/c/fuchsia/+/284569.
