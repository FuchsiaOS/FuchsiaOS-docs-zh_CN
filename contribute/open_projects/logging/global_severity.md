# Dismantle the global log severity file

## Goal & motivation

[Logs][logs] are commonly used to diagnose and troubleshoot system state,
whether in production or in tests.

A useful feature of logs in tests allows [restricting log severity][logs-tests]
under test. This gives test authors the confidence that their test isn't
encountering unexpected erroneous conditions, even though all expectations set
by the test are being met. Experience has shown that this is a useful feature
that detects subtle bugs and regressions.

By default, tests will fail if they log at severity level greater than
`WARNING`, meaning logs at `ERROR` or above.
To override this behavior, developers can set a higher threshold. For instance
if `ERROR`s are expected then developers will set `max_severity = "ERROR"` in
the test specification associated with the test.

When this feature was first introduced, all overrides were set in a
[single file][restrict-legacy]. Since then it became possible to set the
override specific to the test where the test itself is defined. The new approach
is easier to maintain.

## Technical background

Entry-level knowledge of editing `BUILD.gn` files is required.

## How to help

### Picking a task

Pick an entry from [max_severity_fuchsia.json][max-severity-json].
For instance:

```
      {
         "url": "fuchsia-pkg://fuchsia.com/logging-cpp-tests#meta/logging_cpp_unittests.cmx",
         "max_severity": "FATAL"
      },
```

You'll be deleting this part, and setting a similar configuration where
`logging_cpp_unittests.cmx` is used as needed.

### Doing a task

Find the definition for the associated test, such as by looking for a `BUILD.gn`
file that includes the strings `"logging-cpp-tests"` or
`"logging_cpp_unittests.cmx"`. Add the `max_severity` setting to mirror what you
deleted from `max_severity_fuchsia.json`, according to [the guide][logs-tests].

### Completing a task

Find reviewers by OWNERS and merge your change.

## Examples

*   [410049: [blobfs] blobfs stress tests v1](https://fuchsia-review.googlesource.com/c/fuchsia/+/410049)
*   [436337: [network/tests] Split up integration test binary](https://fuchsia-review.googlesource.com/c/fuchsia/+/436337)
*   [426214: [isolated-ota] Refactor into a library for integration tests.](https://fuchsia-review.googlesource.com/c/fuchsia/+/426214)
*   [440054: [run_test_component] Prune network test allowlist](https://fuchsia-review.googlesource.com/c/fuchsia/+/440054)

## Sponsors

Reach out for questions or for status updates:

*   <anmittal@google.com>
*   <shayba@google.com>

[logs]: /docs/concepts/diagnostics/logs/README.md
[logs-tests]: /docs/concepts/testing/logs.md
[max-severity-json]: /garnet/bin/run_test_component/max_severity_fuchsia.json
[restrict-legacy]: https://fuchsia.googlesource.com/fuchsia/+/66ed695f5c0fcf9ef642fb8736f3a85264e18bfd/docs/concepts/testing/test_component.md#restricting-log-severity
