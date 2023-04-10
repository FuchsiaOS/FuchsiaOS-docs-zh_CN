# Validator architecture

Validator applies automated interactive tests to a stateful library such as
Inspect or file systems - an interactive golden file framework.

The Validator architecture includes:

* A set of tests to validate functionality.
* A FIDL protocol to invoke operations to be tested.
* One or more puppet programs, which receive FIDL commands and invoke library
calls.
* A reference implementation or simulation of the desired behavior.
* Analysis of puppet results, comparison to local results, and reporting.

## Inspect validator

The Inspect Validator implementation includes:

* [Core Validator program](/src/diagnostics/validator/inspect/src)
    * [Tests](/src/diagnostics/validator/inspect/src/trials.rs)
    * [FIDL](/src/diagnostics/validator/inspect/fidl/validate.test.fidl)
    * [Reading the puppet's output](/src/diagnostics/validator/inspect/src/data/scanner.rs)
    * [Reference Behavior and comparison](/src/diagnostics/validator/inspect/src/data.rs)
    * [Analysis](/src/diagnostics/validator/inspect/src/runner.rs)
    and [more analysis](/src/diagnostics/validator/inspect/src/metrics.rs)
    * [Reporting](/src/diagnostics/validator/inspect/src/results.rs)
* [Rust Puppet](/src/diagnostics/validator/inspect/lib/rust/src/main.rs).
See also [Inspect Validator Puppet Architecture](puppet.md)
* [Dart Puppet](/sdk/dart/fuchsia_inspect/test/validator_puppet/lib/main.dart)
