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

```gn
fuchsia_component("my-package") {
  testonly = true
  manifest = "meta/my-test.cml"
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

To make the test fail on any message more severe than `INFO`,
set `max_severity` to `"INFO"`.

Valid values for `max_severity`: `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`, `FATAL`.

See also: [choosing severity for log records][choose-severity].

[choose-severity]: /development/diagnostics/logs/severity.md
[syslog]: /development/diagnostics/logs/README.md
