# Contribute changes

This guide provides instructions on how to submit your contribution to the
Fuchsia project.

## Prerequisites

Fuchsia manages git commits and code reviews through
[Gerrit's](https://fuchsia-review.googlesource.com){:.external} web UI.

Before you begin, you need to:

*   [Download the Fuchsia source code](/docs/get-started/get_fuchsia_source.md).

    Note: You can complete the next prerequisite items while downloading the
    Fuchsia source code.

*   [Sign the Google Contributor License Agreements (CLA)](#sign-the-google-cla).

*   [Generate a cookie to authenticate you in Gerrit](#generate-a-cookie).

### Sign the Google CLA {#sign-the-google-cla}

Do the following:

1.  Go to the Google Developers'
    [Contributor License Agreements](https://cla.developers.google.com/){:.external}
    page.
1.  Sign the agreement on behalf of **Only Yourself** or **Your Employer**.

### Generate a cookie {#generate-a-cookie}

Do the following:

1.  Log into [Gerrit](https://fuchsia-review.googlesource.com){:.external}.
1.  Go to
    [https://fuchsia.googlesource.com](https://fuchsia.googlesource.com){:.external}.
1.  At the top of the page, click **Generate Password**.
1.  Copy the generated code and run it in a terminal of your workstation.

## Create a change in Gerrit {#create-a-change-in-gerrit}

The Fuchsia project uses Gerrit's web-based UI to manage code and
documentation reviews. When a commit is uploaded to Gerrit, it is referred to as
as a **change**.

To create a
[change](https://gerrit-review.googlesource.com/Documentation/concept-changes.html){:.external}
in Gerrit, do the following:

1.  Go to your Fuchsia directory, for example:

    ```posix-terminal
    cd ~/fuchsia
    ```

1.  Create a new branch:

    ```posix-terminal
    git checkout -b <branch_name>

    ```

1.  Create or edit files in the new branch.

1.  Add the updated files:

    ```posix-terminal
    git add <files>
    ```

1.  Commit the updated files and
    [write a change message](#write-a-change-message):

    ```posix-terminal
    git commit
    ```

1.  Upload the commit to Gerrit:

    ```posix-terminal
    jiri upload
    ```

    If you want to use the `git` command instead, run the following command:

    ```posix-terminal
    git push origin HEAD:refs/for/master
    ```

See the
[Gerrit documentation](https://gerrit-documentation.storage.googleapis.com/Documentation/2.12.3/intro-user.html#upload-change){:.external}
for more information.

### Request a code review

After creating a change, to request a code review, do the following:

1. Go to your [Fuchsia Gerrit dashboard](https://fuchsia-review.googlesource.com/dashboard/self).
1. Click your change, which appears in the *Outgoing reviews* section.
1. Click **ADD REVIEWER**.
1. Add reviewers by email address. You can refer to the `OWNERS` file, located in the directory
where you're making your change or in one of its parents to find the best reviewers for your change.
1. Click **SEND**.

#### Track your review

To track the progress of your code review, use
[Gerrit](https://fuchsia-review.googlesource.com){:.external}. For more
information on how to use the Gerrit code review tool, see
[Review UI documentation](https://gerrit-review.googlesource.com/Documentation/user-review-ui.html){:.external}.

After you request a code review for your change, reviewers can score
your change. Reviewers can label your change with a
score of **-2**, **-1**, **0**, **+1**, or **+2**. For more information on
review label definitions see, [Gerrit Code Review - Review Labels](https://gerrit-review.googlesource.com/Documentation/config-labels.html){:.external}.

In order for your change to be submitted, you need a **Code Review Label +2**.
A **Code Review Label +2** score can only be applied by a directory owner.

#### Submit your change {#submit-a-change}

A change can be submitted after a repository owner applies the
**Code Review Label +2** to your change. When a change is submitted, the change
is submitted to the Commit Queue (CQ). The Commit Queue verifies and
merges changes to the `main` branch.

**Only contributors with [commit access](/docs/contribute/community/contributor-roles.md#committer)** can submit code directly
through the Gerrit interface. Regular members need to ask a Committer to submit code for them.

### Create and upload a patch

After creating a change, to upload a patch to your change, do the following:

1.  Create or edit files in the same branch.
1.  Add the updated files:

    ```posix-terminal
    git add <files>
    ```

1.  Include the patch in the same commit using the `--amend` option:

    ```posix-terminal
    git commit --amend
    ```

1.  Upload the patch to Gerrit:

    ```posix-terminal
    jiri upload
    ```

### Resolve merge conflicts {#resolve-merge-conflicts}

When Gerrit warns you of merge conflicts in your change, do the following:

1.  Rebase from `origin/master`, which reveals the files that cause merge
    conflicts:

    ```posix-terminal
    git rebase origin/master
    ```

1.  Edit those files to resolve the conflicts and finish the rebase:

    ```posix-terminal
    git add <files_with_resolved_conflicts>
    ```

    ```posix-terminal
    git rebase --continue
    ```

1.  Upload the patch to your change:

    ```posix-terminal
    git commit --amend
    ```

    ```posix-terminal
    jiri upload
    ```

### Delete your local branch

After the change is submitted, you may delete your local branch:

```posix-terminal
git branch -d <branch_name>
```

## Write a change message {#write-a-change-message}

When writing a change message, follow these guidelines:

*   [Add commit message tags](#add-commit-message-tags)
*   [Add test instructions](#add-test-instructions)

### Add commit message tags {#add-commit-message-tags}

Include `[tags]` in the subject of a commit message to indicate which module,
library, and app are affected by your change. For instance, use `[docs]` for
documentation, `[zircon]` for zircon, and `[fidl]` for FIDL.

The following example of a commit message shows the tags in the subject:

<pre>
<b>[parent][component]</b> Update component in Topaz.

Write the details of a commit message here.

Test: Added test X.
</pre>

You can view the commit history of the files you've edited to check for the tags
used previously. See these examples:

*   [https://fuchsia-review.googlesource.com/c/fuchsia/+/441776](https://fuchsia-review.googlesource.com/c/fuchsia/+/441776){:.external}
*   [https://fuchsia-review.googlesource.com/c/topaz/+/114013](https://fuchsia-review.googlesource.com/c/topaz/+/114013){:.external}

Commit message tags are required. If the subject of a commit message
doesn't include tags, Gerrit flags your
change with `Needs Label: Commit-Message-has-tags`.

### Add test instructions {#add-test-instructions}

If a change requires non-obvious manual testing for validation, describe those
testing steps in the change description beginning with `Test:`, for example:

```none
Test: Write the test instructions here.
```

If the instructions are complex, create a bug and provide a link to that bug in
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
[Fuchsia testability rubrics](/docs/concepts/testing/testability_rubric.md) for
more information on how to introduce testable and tested code in the Fuchsia
project.

## Contribute a change to the API

To contribute to the
[Fuchsia API Surface](/docs/glossary.md#fuchsia-api-surface), do the following:

* Evaluate whether your change is large or small.

    * If you have a small, incremental change to the API, contribute your
    change by completing the steps in
    [create a change in Gerrit](#create-a-change-in-gerrit), as you would for
    any Fuchsia source code change.
    * If you have a large change to the API, that is, a change that
    significantly expands on the fuction of the API or modifies the
    API extensively, do the following:
        * Create an [API Design Document](/docs/contribute/governance/api-design-template.md)
        that explains the design of your modification to the API.
        * Request a review of your API Design Document.
        To read about the API Design Document and the API Design Document review
        process, see [Decision process](/docs/contribute/governance/api_council.md#decision_process)
        in the Fuchsia API Council Charter.
        * After your API Design Document is approved, contribute your change by
        completing the steps in
        [create a change in Gerrit](#create-a-change-in-gerrit), as you would
        for any Fuchsia source code change.

* [Request a code review](#request_a_code_review) from an API council
member. Select your API council reviewer based on the area of the
Fuchsia API that you're modifying. For a list of API council members and their
areas of focus, see
[Membership](/docs/contribute/governance/api_council.md#membership) in the
Fuchsia API Council Charter.

## Manage changes that span multiple repositories

To understand how to manage changes that span different repositories (petals),
see the following pages:

*   [Working across different petals](/docs/development/source_code/working_across_petals.md)
*   [Upload changes from multiple repositories](/docs/development/source_code/upload_changes_from_multiple_repositories.md)

See [Source code layout](/docs/concepts/source_code/layout.md) for more
information on the structure of the Fuchsia repository.

