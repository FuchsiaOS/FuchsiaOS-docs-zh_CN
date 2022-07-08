# Workflow tips and questions

## Workflow tips

This is a list of tips that should help you be more productive when working
on Fuchsia.

### Install Gerrit Monitor

Install the [Gerrit Monitor](https://chrome.google.com/webstore/detail/gerrit-monitor/leakcdjcdifiihdgalplgkghidmfafoh)
Chrome extension to include the list of Gerrit changes that require
your attention in your Chrome toolbar.

### Optimize your Gerrit settings

Review [Gerrit settings](https://fuchsia-review.googlesource.com/settings/)
and tweak them to your liking.
For instance you may want to enable "Publish comments on push", which will
automatically send draft comments when a new patch set is published, rather than
you having to do this manually from the web UI.

### Enable three-way diffs in Git

By default Git uses two-way diffs when presenting conflicts. It does not
display what the original text was before the conflict, which makes it [hard to
solve some conflicts](https://stackoverflow.com/questions/4129049/why-is-a-3-way-merge-advantageous-over-a-2-way-merge).

You can configure Git to show the original text by enabling three-way diffs:

```git config --global merge.conflictstyle diff3```

### Enable fuchsia-specific git commands

Add `$FUCHSIA_DIR/scripts/git` to your PATH to be able to use fuchsia-specific git
commands such as `git fuchsia-review [<commit ref>]`, which opens the current
or given commit in gerrit.

## Questions and answers

You are encouraged to add your own questions (and answers) here!

### Q: Is there a standard Git workflow for Fuchsia?

There are a wide variety of workflows used by the Fuchsia team. A daily
workflow to get you started is as follows:

```shell
$ jiri update -gc
# Start a new feature on a `myfeature` branch from the current stable commit
$ git checkout -b myfeature JIRI_HEAD
# Do work, making changes, etc.
$ git commit
# Upload your work to Gerrit for review
$ jiri upload
# OR
$ git push origin HEAD:refs/for/main
```

Congratulations, you made your first Gerrit change!

Suppose you want to start new work on an `otherfeature` branch
while you wait for review of the work located
on your `myfeature` branch:

```shell
# Start a new independent line of work on the `otherfeature` branch
# while waiting for review of `myfeature`:
$ git checkout -b otherfeature JIRI_HEAD
# OR
# Start a derivative line of work while waiting for review:
$ git checkout -b otherfeature
```

When you want to update your `myfeature` branch but you've been working on an
"independent" line of work on the `otherfeature` branch:

```shell
# Commit any present dirty work, then, switch to "myfeature":
$ git checkout myfeature
# Make any relevant edits to the code, then:
$ git commit --amend
# Now upload the new patchset to Gerrit:
$ jiri upload
# OR
$ git push origin HEAD:refs/for/main
```

When you want to update your `myfeature` branch because you got some review
comments, and you are using a "derivative" line of work:

```shell
# Now you get a review comment that needs a change in "myfeature"
# Commit your present work, if you aren't finished, maybe use a work-in-progress change:
$ git commit -a -m "work in progress"
# Start a rebase operation, so you can edit your first change:
$ git rebase -i JIRI_HEAD
# Replace "pick" with "edit" on the change you need to update and save and close the file
# Make the relevant code changes, then:
$ git add . && git rebase --continue
# You may need to make additional "rebase" steps if your edits need integration
# with later commits For each case, look at "git status" to see what files are
# in conflict, and make the relevant adjustments. The rebase is complete when
# git reports "Successfully rebased and updated ...." If you made a "work in
# progress" change and want to unwind that commit:
$ git reset HEAD
# Now you can upload your modified changes to Gerrit:
$ jiri upload
# OR
$ git push origin HEAD:refs/for/main
```

When you see "merge conflict" in Gerrit because your change can't cleanly be
integrated with the `main` branch:

```shell
# Checkout the branch for the change you need to update (e.g. "myfeature"):
$ git checkout myfeature
# Update your git repository:
$ git fetch
# Update your branch:
$ git rebase origin/main
# Fixup and continue the rebase as necessary, until you see "Successfully rebased ..."
# Then upload your newly updated code:
$ jiri upload
# OR
$ git push origin HEAD:refs/for/main
```

When you've been working for more than a day, and you need to "sync your
code" with upstream (you generally want to do this at least once per day):

```
# Commit any in-progress work, then
# Checkout the stable upstream you last sync'd
$ git checkout JIRI_HEAD
# Update your local repository (this will include updates for prebuilts, third
# party repositories, and so on):
$ jiri update -gc
# Now to switch back to, and update your working branch (e.g. "myfeature"):
$ git checkout myfeature
# Updating "myfeature" with the latest stable code:
$ git rebase JIRI_HEAD
# Perform fixups and "git rebase --continue" until you get to "Successfully rebased ..."
```

You can find more information on parts of workflows below.
You can find more information on general git workflows in [gitworkflows(7)](https://github.com/git/git/blob/HEAD/Documentation/gitworkflows.txt).
You can find more information on git in general at [git-scm.com/doc](https://git-scm.com/doc).

#### Rebasing

Update all projects simultaneously, and rebase your work branch on `JIRI_HEAD`:

```shell
$ jiri update -gc -rebase-untracked
$ git checkout <my_branch>
$ git rebase JIRI_HEAD
```

The `git rebase` to `JIRI_HEAD` should be done in *each* repo where you have
ongoing work. It's not needed for repos you haven't touched.

#### Uploading a new patch set (snapshot) of a change

You'll need to *upload* a patch set to
[Gerrit](https://fuchsia-review.googlesource.com/) to have it reviewed by
others. We do this with `jiri upload`.

Gerrit uses an auto-generated metadata marker in the change's description
to figure out which Gerrit review thread to upload a patch to, such
as: `Change-Id: I681125d950205fa7654e4a8ac0b3fee7985f5a4f`

This is different from a git commit's SHA hash, and can be considered stable
during review, as you make edits to your changes and commits. Use the same
Change-Id for a given review (in case you are
[squashing](https://git-scm.com/book/en/v2/Git-Tools-Rewriting-History) multiple
commits).

If you've made changes and want to upload a new patch set, then (assuming that
this is the latest change in your branch; use `git log` to find out) you can do
something like:

```shell
$ git commit -a --amend
# -a for all uncommitted files, --amend to amend latest commit
$ jiri upload
```

#### Resolving merge conflicts

Attempt a rebase:

```shell
$ git fetch origin && git rebase origin/main
# Resolve conflicts as needed...
$ jiri upload
```

But read below about how a `git rebase` can negatively interact with `jiri
update`.

#### Stashing

You can save all uncommitted changes aside, and re-apply them at a later time.
This is often useful when you're starting out with Git.

```shell
$ git stash # uncommitted changes will go away
# do stuff
$ git stash pop # uncommitted changes will come back
```

### Q: I use **fx** and **jiri** a lot. How are they related?

A: They are not related.
[`jiri`](https://fuchsia.googlesource.com/jiri/+/HEAD/) is a wrapper around
git that provides support for managing more than one git repository in sync
(the Fuchsia code base is composed of many git repositories), as well as
synchronizing a set of prebuilt artifacts, such as those found in
`//prebuilt`.
[`fx`](/scripts/fx) is a
convenience wrapper around many tools built in the Fuchsia tree, and helps
with many daily workflow tasks, such as building, running tests, consuming
logs, connecting to shells on devices, and many other operations.

### Q: Will a git rebase to origin/main mess up my jiri-updated (i.e. synchronized) view of the repository?

A: Yes, unless jiri is configured to sync the rebased repository/petal to HEAD
instead of the globally integrated version. This is not the case if you use the
current/new default bootstrap setup, which tracks global integration for all
repos, but may be the case if you set up your checkout in the past or used `fx
set-petal X`.

When working at petal X (accomplished with `fx set-petal X`), `jiri update` will
rebase the local branches in repo X onto HEAD of origin/main. But other
petals' repos will be synced to specific revisions that may be behind HEAD of
their origin/main.

Fuchsia's continuous integration system (specifically rollers) makes a new revision
of a petal available to other petals only after testing that the new revision
doesn't break other petals. `jiri update` will always leave other petals synced
to these successfully-tested revisions. But a git rebase to origin/main for a
petal may advance that repo beyond the tested revision, which has the potential
to introduce breaking changes. The result may be that you can build for a
certain petal, but not for other petals (e.g., correctly build garnet, but not
be able to build topaz).

If you have a particular commit that you want jiri to honor, download its
`jiri.update` file and feed it to `jiri update`.

### Q: What if I need an atomic commit across git repositories?

A: Can't, sorry. Try to arrange your changes so that they don't break each
petal during a transition (i.e., do a [soft
transition](working_across_petals.md#soft-transitions-preferred)). But sometimes
you will necessarily break things; aim to minimize the duration of breakage
(i.e., a [hard transition](working_across_petals.md#hard-transitions)).

Example scenario: I have an interface defined in stem, and it is implemented in
another petal. If I change the interface, am I doomed to break other petals?

Yes. But you can "babysit" the rollers so that the breakage range is minimized.
The caveat with babysitting is that others may *also* be babysitting a breakage,
and you may end up babysitting for longer than you had intended.

Alternatively, you *could* do something as follows:

1.  Introduce a new interface in `lower` that is a copy of the original
    interface.
1.  Wait for `lower-roller` to roll into `upper`, or roll yourself by updating
    the file `upper/manifest`.
1.  Change `upper` to use the new clone interface that maintains the old
    contract.
1.  Change `lower` such that the original interface’s contract is modified to
    the new, desired form.
1.  Wait for `lower-roller`, or roll yourself.
1.  Change `upper` to use the original interface name, now with its new
    contract. Make any changes required.
1.  Delete the clone interface in `lower`.

### Q: How do I do parallel builds from a single set of sources?

Note: this answer is subject to change/breakage shortly after authorship.

Let's assume you want to produce four builds:

 * a "bringup" product for x64
 * a "workstation" product for x64
 * a "core" product for vim3
 * a "workstation" product for vim3

First, one must build Zircon, as the Zircon build directory is shared across
Fuchsia build targets. It doesn't matter at this stage which product/board
combination you pick, we just need to start building Zircon.

```shell
# We start with bringup, because it's small, but it doesn't matter which you start with:
$ fx --dir out.bringup.x64 set bringup.x64
$ fx --dir out/bringup.x64 build
```

Now you have Zircon built, you can start building several other builds concurrently:

```shell
$ fx --dir out/workstation_eng.x64 set workstation_eng.x64
$ fx --dir out/workstation_eng.x64 build > workstation_eng.x64.build.log &

$ fx --dir out/core.vim3 set core.arm64
$ fx --dir out/core.vim3 build > core.vim3.build.log &

$ fx --dir out/workstation_eng.vim3 set workstation_eng.arm64
$ fx --dir out/workstation_eng.vim3 build > workstation_eng.vim3.build.log &
```

You can reference each of these builds while running `fx` tools by passing
`--dir` to your fx command, e.g. to run `fx serve` using the vim3 workstation
product, you would use:

```shell
$ fx --dir out/workstation_eng.vim3 serve
```

You can also change which build directory is your current default by using `fx use`:

```shell
$ fx use out/core.vim3
```

### Q: What if I want to build at a previous snapshot across the repos?

A: You'll need to `jiri update` against a *jiri snapshot file*, an XML file that
captures the state of each repo tracked by jiri.

### Q: How can I get a build that works for a particular fuchsia.git commit?

A: `fx sync-from-stem` will do this. It uses `jiri` under the hood. However,
instead of syncing fuchsia.git and dependencies to match the current integration
repo, it instead finds the integration commit that matches *currently checked
out* fuchsia.git, and syncs integration and dependencies to match that
fuchsia.git commit.

Put another way, fuchsia.git will be untouched, and everything else is synced to
match. This can be useful to bisect within fuchsia.git.

### Q: I'm building on Mac, how to do I stop getting spammed with 'incoming network connection' notifications?

A: You'll want to run `fx setup-macos`, which registers all the relevant Fuchsia
tools with the MacOS Application Firewall.

### Q: When/how do I make a soft vs hard transition when changing APIs?

See [this section](working_across_petals.md#hard-and-soft-transitions) about hard
and soft transitions.

### Q: How do I update a FIDL protocol?

A: The preferred method for updating a FIDL protocol is to use a *soft
transition*. In order for a soft transition to work, you need to create an
intermediate state that supports both the old and new versions of the protocol.

Use the following steps to execute a soft transition:

1.  Modify the FIDL definition in the Stem repository to support both the old
    and new protocol elements. Before landing the change, trigger the *global
    integration* tryjobs to validate that step 2 will succeed.

1.  Publish the Stem repository, either by waiting for the daily automatic
    publication or by manually publishing the repository.

1.  Update all the clients to use the new protocol elements.

1.  Publish all the clients.

1.  Remove the old protocol elements from the FIDL definition in the Stem
    repository.

1.  Publish the Stem repository, typically by waiting for the daily automatic
    publication.

### Q: How do I coordinate changes across multiple Petals?

A: Coordinating an atomic change across multiple Petals (or between the Stem
repository and one or more Petals) requires performing a *hard transition*.

Use the following steps to execute a hard transition:

1.  Prepare changes to all affected repositories. If all of these repositories
    are part of the Platform Source Tree:

    1.  Upload the relevant changes to fuchsia-review.googlesource.com.
    1.  Upload another change that modifies the *global integration*
        repository to reference the git revisions from your changes. Perform
        a "dry run" of the commit queue for this Gerrit change.

1.  Notify the team stating your intention to execute a hard transition.

1.  Land all the changes in the affected repositories. This step will break
    local integration in these repositories but will not break global
    integration because the changes have not been published yet.

1.  Land a change in the *global integration* repository that references the new
    versions of the affected repositories. This change will publish the new
    version of all the affected repositories and should not break global
    integration. This change should unbreak local integration in the affected
    repositories.

### Q: How do I bisect history to track down when something changed?

A: To bisect history, perform the following steps:

1.  Bisect the history in the configuration repository, which contains the
    revision history of global integration, before and after the observable
    change. The result of this bisect will be a single change to configuration
    repository, presumably that includes the publication of one or more
    repositories or prebuilt packages.

1.  If the change to the configuration repository is a publication of a single
    repository, bisect the history of that repository before and after the
    publication of global integration. The result of this bisect should be the
    revision at which the behavior changed.

1.  If the change to the configuration repository is a publication of prebuilt
    packages, switch to the source tree from which the prebuilt packages were
    created. Consult the documentation for that repository regarding how to
    bisect changes in that repository.

1.  If the change to the configuration repository is a publication of multiple
    repositories, bisecting history becomes complicated because the two
    repositories have likely been changed in concert and you will need to
    traverse their history in concert. Consider studying the history of the
    repositories to understand why they were published together.

### Q: Can I search the Fuchsia source code without cloning the repo?

A: Sure! The [base repository](https://fuchsia.googlesource.com/fuchsia)
provides a [Gitiles](https://gerrit.googlesource.com/gitiles/) UI for
navigation. You may also use the Google
[Open Source Code Search](https://cs.opensource.google/fuchsia/fuchsia) tool
to browse and search the Fuchsia codebase online.
