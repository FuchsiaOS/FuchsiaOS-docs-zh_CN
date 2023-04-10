# Detect snapshot filing

Detect runs on-device, periodically gathering and evaluating Diagnostic data to decide whether to generate a crash report / snapshot.

## How it works {#how-detect-works}

Every few minutes Detect reads the Inspect state, evaluates trigger conditions,
and files crash reports to the `FuchsiaDetect` product. The frequency of
diagnostic scans is set by command line arguments in
`//src/diagnostics/detect/meta/triage-detect.cml`.

Diagnostic state other than Inspect is not currently read. SyslogHas(),
KlogHas(), and BootlogHas() will always return false.

## Enabling crash reports {#enabling-detect-reports}

By default, Detect runs in all products and builds, but only creates or files
crash reports in some builds. Otherwise it logs a warning:
`Detect would have filed {}` where `{}` is replaced by the snapshot signature.

Detect only generates crash reports if
`"//src/diagnostics/detect:enable-report-filing"` is included in
`--with-base`.

Crash reports are only sent to the server in some build configurations, or if
explicitly enabled as explained in [Crash reporting].

[Crash reporting]: /src/developer/forensics/crash_reports/README.md

## Snapshot triggers {#snapshot-triggers}

Snapshot triggers live in .triage files in the
`//src/diagnostics/config/triage/detect` directory. Files added to that
directory must also be added to `detect_unconditional_files` in
`//src/diagnostics/config/triage/BUILD.gn`.

The .triage files are standard [Triage] format. Detect adds a new `act` type:

```json5
    snapshot_action: {
        type: "Snapshot",
        trigger: "something > something_else",
        repeat: "Hours(6)",
        signature: "a-string-with-restrictions",
    }
```

* `trigger` is evaluated by Triage-lib just like in `Warning`-type actions.
* `repeat` gives the minimum delay before re-filing if a condition persists
    or repeats. It can use `Days()`, `Hours()`, and `Minutes()`. If the
    `repeat` value is less than `MINIMUM_SIGNATURE_INTERVAL_NANOS` the latter
    will be used and a warning will be logged.
* `signature` will be reported in the crash report.
    * `fuchsia-detect-` will be prepended to the signature.
    * Signatures must contain only lowercase letters and hyphens
      (no spaces or underscores). Any characters
      outside this set will be lowercased or converted to hyphens and an error
      will be logged.
    * Several Snapshot actions may have the same signature. Crash reports filed
      by each action will be identical. `repeat` throttling will be applied
      per-signature not per-action, using the value from the action that
      triggered.

[Triage]: /docs/development/diagnostics/triage/config.md

## Conditional file inclusion

Detect configuration files can be included in only a subset of builds or
products. This can be done in `//src/diagnostics/config/triage/BUILD.gn` by:

1. Adding the file to `detect_conditional_files` rather
than `detect_unconditional_files`
1. Creating a new `config_data()` entry; by convention its target name should
be the same as the file name
1. Adding that target to the appropriate group so the file is included along
with whatever component(s) it should be analyzing


See [here for examples](https://fuchsia-review.googlesource.com/c/fuchsia/+/542996).

## The fine print

Read this section if you're using Detect for debugging on your desktop.

`repeat` is actually a Triage eval expression like `trigger` and could be
calculated dynamically from Inspect data. The expression must evaluate to an
integer number of nanoseconds.

The frequency with which Detect checks diagnostic data is configured by the
`--check-every` command line parameter which is set in
`//src/diagnostics/detect/meta/triage-detect.cml`. (This expression must
contain only constants.)

Detect has built-in limits on its time delays; it will exit with error if
`--check-every` is less than `MINIMUM_CHECK_TIME_NANOS` and it will
delay re-filing snapshots at least `MINIMUM_SIGNATURE_INTERVAL_NANOS`.
These limits can be overridden by adding a `--test-only` command line argument.
