# Frequently Asked Questions

## What is Fuchsia CTS? {#what-is-cts}

Please see the [CTS overview] for an explanation of what CTS is.

## What is the CTS release schedule? {#schedule}

CTS has multiple releases with separate cadences:

| Release  | Schedule |
|----------|----------|
| Canary   | ~4 hrs   |
| Milestone| ~6 weeks |

The canary release is created when new canary releases of the Fuchsia platform
are created. Likewise, milestone releases are created when new milestone releases
of the Fuchsia platform are created.

Milestone branches (e.g. releases/f7) often receive cherry picks. When this
happens, a new CTS for that milestone is generated and automatically rolled
into CI/CQ.

{% dynamic if user.is_googler %}

Internal contributors: Look for builders named cts*prebuilt-roller in Milo
to monitor new releases.

{% dynamic endif %}

## When will my CTS test start running on CQ? {#wait-time}

The tip-of-tree version of your test will immediately begin running on CI/CQ.
This version of the test does not guarantee backward compatibility.

When the next CTS release is cut, it will contain a snapshot of your test from
tip-of-tree which will begin running as soon as that CTS release rolls into
CI/CQ.  This version of your test guarantees backward compatibility.

See [this section](#schedule) above for the release schedule.

## What test environment does CTS use in CQ? {#environments}

See go/fuchsia-builder-viz. Look for builders whose names end in "-cts".

At minimum, all CTS tests run on the core.x64 image in the Fuchsia emulator.

## How can I tell which version of a CTS test is failing? {#which-test-version}

CQ may run several versions of the same CTS test at a time: The version from
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


## How do I reproduce a CTS test failure locally? {#repro}

This depends on the [version](#which-test-version) of the test you'd like to run.

To run the tip-of-tree version locally, you can do:

```sh
fx set //sdk/cts/tests
fx test TEST_NAME
```

For example:

```sh
fx set //sdk/cts/tests
fx test memfs-test-package
```

To run the release version locally, you can do:

```sh
fx set //sdk/cts/release:tests
fx test TEST_NAME
```

For example:

```sh
fx set //sdk/cts/release:tests
fx test memfs-test-package_6.20211109.1.3166058
```

Please see [Run Fuchsia Tests] for more information about how to run
tests.

## What do I do if a CTS test is blocking my CL? {#broken-test}

This is a sign that your CL is breaking a part of the platform surface area.
Please verify that there are no projects in downstream repositories that rely
on the APIs and ABIs modified by your CL. If so, you will need to make a
soft transition. The general worklow is as follows:

1. Submit a CL that introduces the new behavior in your change and verify that
   the tip-of-tree version of the CTS test passes.
1. Notify any downstream SDK Users of the upcoming breaking change, ask them to
   migrate and depend on the new behavior.
1. Wait for the next CTS release to roll into CI/CQ.
1. Submit a CL to remove the old behavior.

## Are there any examples of CTS tests? {#examples}

Yes!  See [//sdk/cts/examples] for some examples, or peruse the complete set
of tests under [//sdk/cts/tests].

## When and why should I write a CTS test? {#why-cts}

You should write a CTS test if:

1. Your software is part of the public or parnter SDKs.
2. You want CQ to prevent backward-incompatible changes to your software
   across multiple releases of the Fuchsia platform.

## How do I retire a CTS test? {#retire-a-test}

A CTS test should stop guarding against breaking changes once the SDK element
it covers is removed (deprecated and no longer supported by the platform, even
for legacy clients). This process is called test "retirement" and allows Fuchsia
contributors to remove things from the SDK.

To retire an entire CTS test, delete the test at HEAD before the upcoming
milestone release. The version of the test from the previous CTS release will
continue running in CQ until the next release is cut.

To retire a few test cases, follow the same procedure: Delete the test cases at
HEAD and wait for the next milestone release.

If you must immediately make changes to a previously released version of a test,
you'll need to get approval from the Release Team to have the change cherry
picked onto the appropriate release branch.

To verify that your change will succeed, you should sync your local Fuchsia
checkout to the release branch and test the change yourself, first.  After
verifying, submit the CL and file a bug against the Release Team.

## How do I temporarily disable a CTS test? {#disable-a-test}

You can disable a test by adding the test's package and component name to the list
of `disabled_tests` on the appropriate `compatibility_test_suite` target in
`//sdk/cts/release/BUILD.gn`.

For example, a test running in Fuchsia's canary release might have the package
URL:

```
fuchsia-pkg://fuchsia.com/my_test_canary#meta/my_test_component.cm
```

This can be disabled as follows:

```
compatibility_test_suite("canary") {
  {{ '<strong>' }}disabled_tests = [
    {
      package = "my_test_canary"
      component_name = "my_test_component"
    },
  ]{{ '</strong>' }}
}
```

Please include a comment with a bug ID as a reminder to enable the test again.
Tests should be enabled again within 72 hours.

If you need to disable a test for an extended period of time, please instead
remove the test from the CTS release by submitting a change like the following:

```
cts_fuchsia_test_package("my_test") {
  test_components = [ ":my_test_component" ]
  {{ '<strong>' }}internal_only_skip_on_cq = true{{ '</strong>' }}
}
```

Then ask the release team to cherry pick this CL onto the appropriate release
branch which will generate a new CTS without the test.

## Additional questions

For questions and clarification on this document, please reach out to this
directory's owners or file a bug in the [CTS bug component].


[CTS bug component]: https://bugs.fuchsia.dev/p/fuchsia/templates/detail?saved=1&template=Fuchsia%20Compatibility%20Test%20Suite%20%28CTS%29&ts=1627669234
[CTS overview]: /docs/development/testing/cts/overview.md
[Run Fuchsia Tests]: /docs/development/testing/run_fuchsia_tests.md
[//sdk/cts/examples]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/sdk/cts/examples/
[//sdk/cts/tests]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/sdk/cts/tests/
