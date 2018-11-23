# Workflow: Questions and Answers

You are encouraged to add your own questions (and answers) here!

[TOC]

## Q: Is there a standard Git workflow for Fuchsia?

A: No. Instead, the Git tool offers infinite control and variety for defining
your own workflow. Carve out the workflow you need.

### Rebasing

Update all projects simultaneously, and rebase your work branch on `JIRI_HEAD`:

```shell
$ jiri update -gc -rebase-untracked
$ cd garnet  # go into a petal
$ git checkout <my_branch>
$ git rebase JIRI_HEAD
```

The `git rebase` to `JIRI_HEAD` should be done in *each* repo where you have
ongoing work. It's not needed for repos you haven't touched.

### Uploading a new patch set (snapshot) of a change

You'll need to *upload* a patch set to
[Gerrit](https://fuchsia-review.googlesource.com/) to have it reviewed by
others. We do this with `jiri upload`.

Gerrit uses an auto-generated metadata marker in the CL description to figure
out which Gerrit review thread to upload a patch to, such as: `Change-Id:
I681125d950205fa7654e4a8ac0b3fee7985f5a4f`

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

### Resolving merge conflicts

Attempt a rebase:

```shell
$ git fetch origin && git rebase origin/master
# Resolve conflicts as needed...
$ jiri upload
```

But read below about how a `git rebase` can negatively interact with `jiri
update`.

### Stashing

You can save all uncommitted changes aside, and re-apply them at a later time.
This is often useful when you're starting out with Git.

```shell
$ git stash # uncommitted changes will go away
# do stuff
$ git stash pop # uncommitted changes will come back
```

## Q: I use **fx** and **jiri** a lot. How are they related?

A: [`jiri`](https://fuchsia.googlesource.com/jiri/+/master/) is source
management for multiple repositories.
[`fx`](https://fuchsia.googlesource.com/scripts/+/master) is a convenience
wrapper for configuring and running the build system (Make for Zircon,
[GN](https://fuchsia.git.corp.google.com/docs/+/HEAD/glossary.md#gn) and
[Ninja](https://fuchsia.git.corp.google.com/docs/+/HEAD/glossary.md#ninja) for
everything else), as well as facilities to help with day-to-day engineering (`fx
boot`, `fx log`, etc).

## Q: Will a git rebase to origin/master mess up my jiri-updated (i.e. synchronized) view of the repository?

A: No, if jiri is managing up to the *same petal* as your repository.

When working at petal X (accomplished with `fx set-petal X`), `jiri update` will
rebase the local branches in repo X onto HEAD of origin/master. But other
petals' repos will be synced to specific revisions that may be behind HEAD of
their origin/master.

Our continuous integration system (specifically rollers) makes a new revision of
a petal available to other petals only after testing that the new revision
doesn't break other petals. `jiri update` will always leave other petals synced
to these successfully-tested revisions. But a git rebase to origin/master for a
petal may advance that repo beyond the tested revision, which has the potential
to introduce breaking changes. The result may be that you can build for a
certain petal, but not for other petals (e.g., correctly build garnet, but not
be able to build topaz).

If you have a particular commit that you want jiri to honor, download its
`jiri.update` file and feed it to `jiri update`.

## Q: What if I need an atomic commit across git repositories?

A: Can't, sorry. Try to arrange your CLs to not break each petal during a
transition (i.e., do a [soft
transition](multilayer_changes.md#soft-transitions-preferred)). But sometimes
you will necessarily break things; aim to minimize the duration of breakage
(i.e., a [hard transition](multilayer_changes.md#hard-transitions)).

Example scenario: I have an interface defined in stem, and it is implemented in
another petal. If I change the interface, am I doomed to break other petals?

Yes. But you can "babysit" the rollers so that the breakage range is minimized.
The gotcha with babysitting is that others may *also* be babysitting a breakage,
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

## Q: How do I do parallel builds from a single set of sources?

A: Currently, this is not possible. The vanilla GN + Ninja workflow should allow
this, but `fx` maintains additional global state.

Another slight limitation is that GN files for Zircon are currently generated at
build-time, and running multiple parallel builds which both try to generate GN
files may confuse Ninja. It's unclear whether this is a real issue or not.

## Q: What if I want to build at a previous snapshot across the repos?

A: You'll need to `jiri update` against a *jiri snapshot file*, an XML file that
captures the state of each repo tracked by jiri.

## Q: I'm building on Mac, how to do I stop getting spammed with 'incoming network connection' notifications?

A: You'll want to run `fx setup-macos`, which registers all the relevant Fuchsia
tools with the MacOS Application Firewall.

## Q: When/how do I make a soft vs hard transition when changing APIs?

See [this section](/development/workflows/multilayer_changes.md#hard-and-soft-transitions)
about hard and soft transitions.

## Q: How do I update a FIDL protocol?

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

## Q: How do I coordinate changes across multiple Petals?

A: Coordinating an atomic change across multiple Petals (or between the Stem
repository and one or more Petals) requires performing a *hard transition*.

Use the following steps to execute a hard transition:

1.  Prepare changes to all affected repositories. If all of these repositories
    are part of the Fuchsia source tree:

    1.  Upload CLs containing the changes to fuchsia-review.googlesource.com.
    1.  Upload another CL that modifies the *global integration* repository to
        reference the git revisions from your CLs. Perform a "dry run" of the
        commit queue for this CL.

1.  Notify the team stating your intention to execute a hard transition.

1.  Land all the changes in the affected repositories. This step will break
    local integration in these repositories but will not break global
    integration because the changes have not been published yet.

1.  Land a change in the *global integration* repository that references the new
    versions of the affected repositories. This change will publish the new
    version of all the affected repositories and should not break global
    integration. This change should unbreak local integration in the affected
    repositories.

## Q: How do I bisect history to track down when something changed?

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
