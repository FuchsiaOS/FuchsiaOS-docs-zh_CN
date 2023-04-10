# Triage codelab

Contributors: cphoenix@

This codelab explains the Triage utility:

*   What it's for.
*   How to run it, including command line options.
*   How to add and test configuration rules to detect problems in Fuchsia
    snapshots.

The source files and examples to which this document refers are available at:

*   [//examples/diagnostics/triage/snapshot/inspect.json][triage-inspect-example].
*   [//examples/diagnostics/triage/rules.triage][triage-rules-example].
*   [//examples/diagnostics/triage/solution][triage-codelab-solution].

## What is Triage?

Triage allows you to scan Fuchsia snapshots (snapshot.zip files) for predefined
conditions.

The Triage system makes it easy to configure new conditions, increasing the
usefulness of Triage for everyone.

## Prerequisites

Before you start on this codelab, make sure you have completed the following:

*   [Getting started](/docs/get-started/README.md).
*   Made a Fuchsia build with `fx build`.

## Running Triage from the command line

*   To run Triage:

```shell
fx triage
```

This command downloads a fresh `snapshot.zip` file using the `fx snapshot`
command. This command runs the default rules, which are located in the source
tree:

*   `//src/diagnostics/config/triage/*.triage`

To analyze a specific snapshot.zip file, use `--data`.

*   You can specify at most one `--data` argument.
*   The argument to `--data` can also be a path to a directory containing an
    unzipped snapshot.
*   If you run `fx triage` without specifying a `--data` option, it runs a fresh
    `fx snapshot` and analyzes its files.

```shell
$ fx triage --data my/foo/snapshot.zip
```

To use a specific configuration file or all `*.triage` files in a specific
directory, use `--config`.

*   You can use multiple `--config` arguments.
*   If a `--config` argument is used, the default rules will not be
    automatically loaded.

```shell
fx triage --config my/directory --config my/file.triage
```

## Adding Triage rules

The rest of this codelab explains how to configure new behavior in Triage.

### Overview

Triage condenses the mass of Diagnostic data into useful information, through
the following steps:

1.  Select values from the `inspect.json` file using _selector_ strings in the
    `select` section of the config file.
1.  Perform computations and comparisons to generate new values, as specified in
    the `eval` section of the config file.
1.  Take actions according to entries in the `act` section of the config file.
    1.  Warn if an error condition (Boolean expression) is true.
    1.  Display a value to the user.
1.  Support unit-testing the actions and computation via entries in the `test`
    section.

### Find the codelab's sample files

Navigate to the `examples/diagnostics/triage` directory in your source tree.
The following command is intended to run from that directory:

```shell
fx triage --config . --data snapshot
```

Running this command in the sample directory with the unmodified codelab files
prints a line from a pre-supplied action:

```none
Warning: 'always_triggered' in 'rules' detected 'Triage is running': 'always_true' was true
```

#### inspect.json

This codelab includes an `inspect.json` file with Inspect data to make the
exercises work predictably. This file is in the sample directory under
`snapshot/inspect.json`.

Note: `inspect.json` files are normally packaged in the `snapshot.zip` file
produced by `fx snapshot`. Either use `unzip` to unpack these files or give the
.zip file as the argument to `--data`. For this codelab the snapshot has already
been unzipped.

#### rules.triage

The Triage program uses configuration loaded from one or more .triage files.
This codelab uses the `rules.triage` file located in the sample directory.

.triage files use JSON5 which is easier to write and read than JSON:

*   It's good style to put a comma after the last list item.
*   Most keys (including all valid Triage names) don't need to be wrapped in
    quotation marks.
*   /* Multiline */ and // single-line comments can be used.

### Add selectors for the Inspect values

The `inspect.json` file in the sample directory indicates a couple of problems
with the system. You're going to configure the triage system to detect those
problems.

This step configures Triage to extract values from the data in the
`inspect.json` file.

The `rules.triage` file contains a key-value section called `select`. Each key
(name) can be used in the body of other config entries. Each value is a selector
string. In effect, each entry in the `select` section (and the `eval` section,
described below) defines a variable.

The selector string is a colon-separated string that tells where in the Inspect
data to find the value you need.

```json5
select: {
    disk_total: "INSPECT:bootstrap/fshost:root/data_stats/stats:total_bytes",
    // "data_stat" is an intentional typo to fix later in the codelab.
    disk_used: "INSPECT:bootstrap/fshost:root/data_stat/stats:used_bytes",
}
```

Inspect data published by a component is organized as a tree of nodes with
values (properties) at the leaves. The `inspect.json` file is an array of these
trees, each with a moniker that identifies the source component.

The portion of the selector string between the `INSPECT:` and the second colon
should match one of the moniker strings in the `inspect.json` file.

The portion between the second and third colons is a `/`-separated list of node
names.

The portion after the last colon is the property name.

The above selector string indicates a component whose `moniker` matches the
string `bootstrap/fshost` and whose inspect tree contains the path
`root/data_stat/stats`. It also indicates the `used_bytes` property from the
`stats` subnode of the `root` node of that component's Inspect Tree.

Put the above selectors into the "select" section of the rules.triage file.

#### Generating selectors

Entering selectors by hand is error-prone, so `fx triage --select` can be used
to output valid selectors.

```shell
$ fx triage --data snapshot --select total_bytes
INSPECT:bootstrap/fshost:root/data_stats/stats:total_bytes
```

Multiple `--select` parameters can be used. The program will generate all
possible selectors for the snapshot's Diagnostic data, then filter (grep)
through all `--select` parameters.

### Add a computation

After selecting values from the `inspect.json` file, you need to do some logic,
and probably some arithmetic, to see whether those values indicate a condition
worth flagging.

Copy and add the following line to the `eval` section of the `rules.triage` file
to calculate how full the disk is:

```json5
eval: {
    ....
    disk_percentage: "disk_used / disk_total",
}
```

`eval` entries use ordinary infix math expressions. See the [Details](#details)
section for more information.

### Add an action

In the "act" part of the config file, add an action that prints a warning when
the disk is 98% full. Use the following lines:

```json5
act: {
    ...
    disk_full: {
        type: "Warning",
        trigger: "disk_percentage > 0.98",
        print: "Disk reached 98% full",
    },
}
```

Note the following:

*   The "trigger" is an expression that evaluates to a Boolean value. This may
    be the name of a Boolean-type selector or computation, or any suitable math
    expression.
*   See the [Details](#details) section for more information about comparisons.
*   See the [config reference][triage-config-reference] for other supported
    actions.

### Add a gauge

```json5
act: {
    ...
    disk_display: {
        type: "Gauge",
        value: "disk_percentage",
        format: "percentage",
    }
}
```

Gauges always display the supplied value.

The `format` field is optional. A value of "percentage" tells the output
formatter to expect a value between 0 and 1 and display it as a percentage.

### Try it out

The following command will run Triage against the local config file.

```shell
fx triage --config . --data snapshot
```

You will get an error that looks like the following:

```none
[ERROR] In config 'rules': No value found matching selector
bootstrap/fshost:root/data_stat/stats:used_bytes
```

There was a typo in the selector rules. Triage could not find values needed to
evaluate a rule. In fact, the correct selector is "data_stats" not "data_stat."
Fix it in your selector rules and try again.

```shell
fx triage --config . --data snapshot
```

Now what happened? Nothing, right? So, how do you know whether there was no
problem in the `inspect.json` file, or a bug in your rule?

### Test your rule

You can (and should!) add tests for your actions. For each test, specify values
and whether or not those values should trigger your rule.

To test the rule you've added, add the following to the `test` section of the
`rules.triage` file:

```json5
test: {
    ....
    is_full: {
        yes: ["disk_full"],
        values: {
            disk_used: 98,
            disk_total: 100,
        }
    }
}
```

Note: Unlike the right hand side of `eval` entries, the `values` entries are
parsed as JSON values, not expression strings. Numbers should not be quoted.

The keys in the `values` section should be the names of `eval` or `select`
entries. The values supplied will override the value that the entry would have
selected or calculated.

You can also test conditions in which actions should not trigger:

```json5
test: {
    ....
    not_full: {
        no: ["disk_full"],
        values: {
            disk_used: 97,
            disk_total: 100,
        }
    }
}
```

To run the test, just run Triage. It automatically self-tests each time it's
run.

```shell
fx triage --config . --data snapshot
```

Whoops! That should signal an error:

`Test is_full failed: trigger 'disk_percentage > 0.98' of action disk_full
returned Bool(false), expected true`

### Fix your rule

You want to trigger when the disk is 98% or more full, but that's not quite what
you wrote, and your test caught the problem. Modify the `>` in your action to be
a `>=`:

```json5
        trigger: "disk_percentage >= 0.98",
```

Run Triage again. The error should disappear, replaced by a warning that your
`inspect.json` file does in fact indicate a full disk.

`Warning: 'disk_full' in 'rules' detected 'Disk is 98% full': 'disk98' was true`

### Log file scanning

You can write expressions to test whether a line of a log file (syslog.txt,
klog.txt, bootlog.txt) is matched by a regular expression. It looks like this:

```json5
    eval: {
        syslog_has_not_found: "SyslogHas('ERROR.*not found')",
        ...
    }
    act: {
        something_not_found: {
            trigger: "SyslogHas('ERROR.*not found')",
            ...
        }
    }
```

Note: To nest quotation marks you can use either single quote `'` or escaped
double quote `\"`.

The functions are SyslogHas(), KlogHas(), BootlogHas(). If a log file is missing
(for example, some snapshots contain no bootlog.txt) it is treated the same as
an empty file.

To test this, you can include entries in the test like this:

```json5
test: {
    test_error: {
        yes: [error_scan],
        syslog: "ERROR: file Foo not found\nSecond line OK",
    }
}
```

### annotations.json

The snapshot contains a file `annotations.json`, which contains information on
the build, board, uptime, and so on.

Values can be fetched from this file by using the function `Annotation()` with a
single string parameter, which is a key of the JSON object in the file. For
example,

```json5
eval: {
    using_chromebook: "Annotation('build.board') == 'chromebook-x64'",
}
```

### Use multiple configuration files

You can add any number of Triage configuration files, and even use variables
defined in one file in another file. This has lots of applications:

*   One file for disk-related variables and actions, and another for
    network-related variables and actions.
*   A file to define product-specific numbers.
*   Separate files for particular engineers or teams.

Add a file "product.triage" containing the following:

```json5
{
    eval: {
        max_components: "4",
    },
}
```

Note the following:

*   Empty sections may be omitted from .triage files. This file contains no
    `select`, `act`, or `test` entries.
*   Although numeric values in JSON are not quoted, `4` is a math expression
    string so it does need to be quoted.

Add the following entries to the rules.triage file:

```json5
select: {
    ...
    actual_components: "INSPECT:bootstrap/archivist:root/event_stats:components_started",
}
```

That will extract how many components were active in the device.

```json5
eval: {
    ...
    too_many_components: "actual_components > product::max_components",
```

That compares the actual components with the theoretical maximum for the
product.

Note: To use variable names from another file, combine the file name, two
colons, and the variable name.

Finally, add an action:

```json5
act: {
    ...
    component_overflow: {
        type: "Warning",
        trigger: "too_many_components",
        print: "Too many components!",
    },
}
```

Unfortunately, this device tried to use too many components, so this warning
should trigger when "fx triage" is run.

Note: The `trigger` of an action can also use `file::name` syntax to refer to a
variable from another file.

In a production environment, several "product.triage" files could be maintained
in different directories, and Triage could be directed to use any of them with
the "--config" command line argument.

#### Tests and namespaces

Tests use only the metrics within the file where the test occurs, plus the
values supplied by the test. An expression (eval or test trigger) that uses
namespaced values like "a::b" must have those values supplied by an "a::b" entry
in the test's values.

Note: Unlike most keys in .triage files, namespaced names must be double-quoted
when used as keys.

```json5
test: {
    component_max_ok: {
        no: [
            "component_overflow",
        ],
        values: {
            actual_components: 17,
            "product::max_components": 17,
        },
    },
},
```

### Details {#details}

#### Names

Names (of selectors, expressions, actions, and tests, as well as the basenames
of config files) can be any letter or underscore, followed by any number of
letters, numbers, or underscores.

Names beginning with underscores may have special meaning in future versions of
Triage. They're not forbidden, but it's best to avoid them.

The name of each .triage file establishes its namespace. Loading two .triage
files with the same name from different directories is not allowed.

#### Math expressions

*   Variables can be 64-bit float, signed 64-bit int, or Boolean.
*   Arithmetic expressions use `+ - * / //` operators, with ordinary order and
    precedence of operations.
*   The division operator `/` produces a float value.
*   The division operator `//` produces an int value, truncating the result
    toward 0, even with float arguments. (Note this is different from Python 3
    where // truncates downward.)
*   `+ - *` preserve the type of their operands (mixed promotes to float).
*   Comparison operators are `> >= < <= == !=`
*   Comparisons have Boolean result type and can be used to trigger actions.
*   You can combine computations and comparisons in a single `eval` rule.
*   You can use parentheses.
*   You can use the key names of `eval` and `select` entries as variables.
*   Spaces are optional everywhere, and allowed everywhere except inside
    `filename::variable` namespaced variables.

#### Predefined functions

Triage provides predefined functions for use in `eval` expressions:

*   `Max(value1, value2, value3...)` returns the largest value, with type
    promotion to float.
*   `Min(value1, value2, value3...)` returns the smallest value, with type
    promotion to float.
*   `And(value1, value2, value3...)` takes Boolean arguments and returns the
    logical AND of the values.
*   `Or(value1, value2, value3...)` takes Boolean arguments and returns the
    logical OR of the values.
*   `Not(value)` takes one Boolean argument and returns the logical NOT of it.
*   `SyslogHas(matcher)`, `KlogHas(matcher)`, `BootlogHas(matcher)` return true if the
    corresponding log file has a line matching matcher, which is a string
    containing a regex expression.
*   `Annotation(key)` returns the corresponding value from the annotations.json
    file.
*   `Option(value1, value2, value3...)` returns the first useful value, to
    support selector migrations and defaults: the first non-empty-list,
    non-Missing value if any; or empty list if one was given; or Missing.
*   `Missing(value)` returns true if the value is an error indication.
*   `Days()`, `Hours()`, `Minutes()`, `Seconds()`, `Millis()`, `Micros()`,
    and `Nanos()` calculate values for comparison with monotonic timestamps.
*   `Now()` returns the approximate timestamp when the Diagnostic data was
    created.
*   `StringMatches(value, regex)` applies the given regex to the given
    value and returns true if there is a match. The regex syntax is that
    supported by the Rust [regex crate](https://docs.rs/regex/latest/regex/).

Note: Since logs are not structured, selectors can't be applied to them, so we
supply regex matching functions instead.

#### Functional programming

Triage can apply functions to vectors of values. Vectors have the format
`"[expr, expr, expr...]"`. Some selectors return multi-element vectors.

Triage provides the functions `Map()`, `Fold()`, `Filter()`, and `Count()` to
process vectors, `Fn()` to define functions or lambdas for Map, Fold, and
Filter to apply, and `Apply()` to apply a Fn() to arguments.

For more information see [Configuring fx triage][triage-config-reference].


## Further Reading

See [`fx triage`][fx-triage] for the latest features and options - Triage will
keep improving!

[fx-triage]: https://www.fuchsia.dev/reference/tools/fx/cmd/triage
[triage-inspect-example]: /examples/diagnostics/triage/snapshot/inspect.json
[triage-rules-example]: /examples/diagnostics/triage/rules.triage
[triage-codelab-solution]: /examples/diagnostics/triage/solution
[triage-config-reference]: /docs/development/diagnostics/triage/config.md
