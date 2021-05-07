# Logging and tests

## Restricting log severity

By default, a test will fail if it [logs][syslog] a message with a severity of
`ERROR` or higher. This often indicates that an unexpected condition had
occurred during the test, so even if the test passes it's often useful to bring
this to the developer's attention.

This default behavior can be changed, for each test package, to allow logs at
higher severities, or to fail a test that logs at lower severities. For example,
a test might *expect* to log an `ERROR`, in order to cover a failure condition and
recovery steps.

A test might expect to log at `ERROR` severity. For example, the test might be
covering a failure condition & recovery steps. Other tests might expect not to
log anything more severe than `INFO`.

For instance, to allow a test to produce `ERROR` logs:

  * {Using fuchsia\_test\_package}

  ```gn
  fuchsia_component("my-package") {
    testonly = true
    manifest = "meta/my-test.cmx"
    deps = [ ":my_test" ]
  }

  fuchsia_test_package("my-package") {
    test_specs = {
        log_settings = {
          max_severity = "ERROR"
        }
    }
    test_components = [ ":my-test" ]
  }
  ```

  * {Using test\_package}

  ```gn
  test_package("my-package") {
    deps = [
      ":my_test",
    ]

    meta = []
      {
        path = rebase_path("meta/my-test.cmx")
        dest = "my-test.cmx"
      },
    ]

    tests = [
      {
        log_settings = {
          max_severity = "ERROR"
        }
        name = "my_test"
        environments = basic_envs
      },
    ]
  }
  ```

To make the test fail on any message more severe than `INFO`,
set `max_severity` to `"INFO"`.

Valid values for `max_severity`: `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`, `FATAL`.

See also: [choosing severity for log records][choose-severity].

### Legacy configuration

The section above shows a method for configuring allowed log severity as
metadata on the test package definition. An
[older approach][legacy-restrict-logs] defined a single configuration file per
repository. This was more difficult to maintain, and required running `fx ota`
before updates would take effect.

If your test has a legacy configuration, it is recommended to remove it from the
global configuration file and to add it to your build target instead as shown
above.

In the case of a conflict, the legacy configuration takes precedence and a
warning is printed when running the test.

[choose-severity]: /docs/development/diagnostics/logs/severity.md
[legacy-restrict-logs]: https://fuchsia.googlesource.com/fuchsia/+/1529a885fa0b9ea4867aa8b71786a291158082b7/docs/concepts/testing/test_component.md#restricting-log-severity
[syslog]: /docs/development/diagnostics/logs/README.md
