# Dismantle the global log severity file

<<_banner.md>>

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
single file. Since then it became possible to set the
override specific to the test where the test itself is defined. The new approach
is easier to maintain.

## Technical background

Entry-level knowledge of editing `BUILD.gn` files is required.

## How to help

### Picking a task

Pick an entry from [max_severity_fuchsia.json][max-severity-json].
For instance:

```json
      {
           "max_severity": "FATAL"
           "url": "fuchsia-pkg://fuchsia.com/audio_core_unittests#meta/audio_core_unittests.cm"
      },
```

### Doing a task

You'll be deleting this part, and setting a similar configuration on the build
definition for the test above.

```gn
fuchsia_test_package("audio_core_unittests") {
  test_specs = {
      log_settings = {
        max_severity = "FATAL"
      }
  }
  ...
}
```

You may also refer to [the guide][logs-tests].

Note that while most tests are defined with the `fuchsia_test_package()`
template, some tests are defined with other wrapper templates. Often times the
wrappers accept `test_specs` and forward them to the underlying
`fuchsia_test_package` template.

### Completing a task

Find reviewers by OWNERS and merge your change.

## Examples

* [555759: [log] Move intl_services log severity config to test definition](https://fuchsia-review.googlesource.com/c/fuchsia/+/555759)
* [410049: [blobfs] blobfs stress tests v1](https://fuchsia-review.googlesource.com/c/fuchsia/+/410049)
* [436337: [network/tests] Split up integration test binary](https://fuchsia-review.googlesource.com/c/fuchsia/+/436337)
* [426214: [isolated-ota] Refactor into a library for integration tests.](https://fuchsia-review.googlesource.com/c/fuchsia/+/426214)
* [440054: [run_test_component] Prune network test allowlist](https://fuchsia-review.googlesource.com/c/fuchsia/+/440054)

## Sponsors

Reach out for questions or for status updates:

* <anmittal@google.com>
* <shayba@google.com>

[logs]: /docs/concepts/components/diagnostics/logs/README.md
[logs-tests]: /docs/development/diagnostics/test_and_logs.md
[max-severity-json]: https://fuchsia.googlesource.com/fuchsia/+/589aecf5a99689e33621137355b79dcebf6e0c48/garnet/bin/run_test_component/max_severity_fuchsia.json
