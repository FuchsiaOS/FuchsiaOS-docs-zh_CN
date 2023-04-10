# Frequently Asked Questions

## What is Fuchsia CTF? {#what-is-cts}

Please see the [CTF overview] for an explanation of what CTF is.

## What is the CTF release schedule? {#schedule}

CTF has multiple releases with separate cadences:

| Release  | Schedule |
|----------|----------|
| Canary   | ~4 hrs   |
| Milestone| ~6 weeks |

The canary release is created when new canary releases of the Fuchsia platform
are created. Likewise, milestone releases are created when new milestone releases
of the Fuchsia platform are created.

Milestone branches (e.g. releases/f7) often receive cherry picks. When this
happens, a new CTF for that milestone is generated and automatically rolled
into CI/CQ.

{% dynamic if user.is_googler %}

Internal contributors: Look for builders named cts*prebuilt-roller in Milo
to monitor new releases.

{% dynamic endif %}

## When will my CTF test start running on CQ? {#wait-time}

The tip-of-tree version of your test will immediately begin running on CI/CQ.
This version of the test does not guarantee backward compatibility.

When the next CTF release is cut, it will contain a snapshot of your test from
tip-of-tree which will begin running as soon as that CTF release rolls into
CI/CQ.  This version of your test guarantees backward compatibility.

See [this section](#schedule) above for the release schedule.

## What test environment does CTF use in CQ? {#environments}

See go/fuchsia-builder-viz. Look for builders whose names end in "-cts".

At minimum, all CTF tests run on the core.x64 image in the Fuchsia emulator.

## How can I tell which version of a CTF test is failing? {#which-test-version}

CQ may run several versions of the same CTF test at a time: The version from
tip-of-tree, from the latest canary release, and from a previous milestone
release.

The tip-of-tree version of the test has a package name without a release version.
For example:

```
fuchsia-pkg://fuchsia.com/memfs-test-package#meta/memfs-component.cmx
```

The canary or milestone versions of the test include the release version. For
example:

```
fuchsia-pkg://fuchsia.com/memfs-test-package_6.20211109.1.3166058#meta/memfs-component.cmx
```


## How do I reproduce a CTF test failure locally? {#repro}

This depends on the [version](#which-test-version) of the test you'd like to run.

To run the tip-of-tree version locally, you can do:

```sh
fx set //sdk/ctf/tests
fx test TEST_NAME
```

For example:

```sh
fx set //sdk/ctf/tests
fx test memfs-test-package
```

To run the release version locally, you can do:

```sh
fx set //sdk/ctf/release:tests
fx test TEST_NAME
```

For example:

```sh
fx set //sdk/ctf/release:tests
fx test memfs-test-package_6.20211109.1.3166058
```

Please see [Run Fuchsia Tests] for more information about how to run
tests.

## What do I do if a CTF test is blocking my CL? {#broken-test}

This is a sign that your CL is breaking a part of the platform surface area.
Please verify that there are no projects in downstream repositories that rely
on the APIs and ABIs modified by your CL. If so, you will need to make a
soft transition. The general worklow is as follows:

1. Submit a CL that introduces the new behavior in your change and verify that
   the tip-of-tree version of the CTF test passes.
1. Notify any downstream SDK Users of the upcoming breaking change, ask them to
   migrate and depend on the new behavior.
1. Wait for the next CTF release to roll into CI/CQ.
1. Submit a CL to remove the old behavior.

## Are there any examples of CTF tests? {#examples}

Yes!  See [//sdk/ctf/examples] for some examples, or peruse the complete set
of tests under [//sdk/ctf/tests].

## When and why should I write a CTF test? {#why-cts}

You should write a CTF test if:

1. Your software is part of the public or partner SDKs.
2. You want CQ to prevent backward-incompatible changes to your software
   across multiple releases of the Fuchsia platform.

## How do I retire a CTF test? {#retire-a-test}

A CTF test should stop guarding against breaking changes once the SDK element
it covers is removed (deprecated and no longer supported by the platform, even
for legacy clients). This process is called test "retirement" and allows Fuchsia
contributors to remove things from the SDK.

To retire an entire CTF test, delete the test at HEAD before the upcoming
milestone release. The version of the test from the previous CTF release will
continue running in CQ until the next release is cut.

To retire a few test cases, follow the same procedure: Delete the test cases at
HEAD and wait for the next milestone release.

If you must immediately make changes to a previously released version of a test,
you'll need to get approval from the Release Team to have the change cherry
picked onto the appropriate release branch.

To verify that your change will succeed, you should sync your local Fuchsia
checkout to the release branch and test the change yourself, first.  After
verifying, submit the CL and file a bug against the Release Team.

## How do I temporarily disable a CTF test? {#disable-a-test}

You can disable a test by adding the test's archive name to the list of
`disabled_tests` on the appropriate `compatibility_test_suite` target in
`//sdk/ctf/release/BUILD.gn`.

For example:

```gn
compatibility_test_suite("canary") {
  {{ '<strong>' }}disabled_tests = [ "my_test" ]{{ '</strong>' }}
}
```

To find the archive name, check the //prebuilt/cts/$version>/$platform/cts/test_manifest.json
file for the archive name corresponding to the test.

Please include a comment with a bug ID as a reminder to enable the test again.
Tests should be enabled again within 72 hours.

## Additional questions

For questions and clarification on this document, please reach out to this
directory's owners or file a bug in the [CTF bug component].

[CTF bug component]: https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=Fuchsia+Compatibility+Test+Suite+%28CTS%29
[CTF overview]: /docs/development/testing/ctf/overview.md
[Run Fuchsia Tests]: /docs/development/testing/run_fuchsia_tests.md
[//sdk/ctf/examples]: /sdk/ctf/tests/examples/
[//sdk/ctf/tests]: /sdk/ctf/tests/
