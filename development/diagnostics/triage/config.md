# Configuring Triage

Triage analyzes Diagnostics data according to config files.

## Overview {#triage-overview}

Triage allows anyone to easily add new ways to analyze `fx snapshot` data for
off-nominal conditions.

By default, the config files are read from
`//src/diagnostics/config/triage/*.triage`. Just add a new config file there.

Config file syntax is JSON5.

Each config file specifies four kinds of configuration: Selectors and Evals
(collectively called "Metrics"), Actions, and Tests.

*   Selectors load values for use by Evals and Actions.
*   Evals calculate values for use by Evals and Actions.
*   Actions determine how to act upon certain specified values.
*   Tests include sample data to verify that specified Actions trigger
    correctly.

Each Select, Eval, Test, and Action has a name. Thus, the structure of a config
file is:

```json5
{
    "select": {
        "select1": "type:component:node/path:property",
        "select2": "type:component:node/path:property"
    },
    "eval": {
        "name1": "select1+select2",
        "name2": "select1 - select2"
    },
    "act": {
        "action1": { .... },
        "action2": { .... }
    },
    "test": {
        "test1": { .... },
        "test2": { .... }
    }
}
```

## Names and namespaces {#triage-names}

Select, Eval, Action, Test, and config file names consist of one
lowercase-alphabetic-or-underscore character followed by zero or more
alphanumeric-or-underscore characters. Thus, "abc123" and "_abc_123" are valid
names, but "123abc", "a.b.c", and "abc-123" are not. In particular, file names
are not allowed to contain periods except for the `.triage` extension.

Evals, Tests, and Actions in one file can refer to Selectors, Evals, and Actions
in another file. The file basename is used as a namespace. `::` is used as the
separator. For example, if file `foo.triage` is loaded and contains a Metric
named `bar` then any config file may refer to `foo::bar`.

Names may be reused between Metrics, Tests, and Actions, but not between Select
and Eval.

User-defined names must start with a lowercase letter. Language-supplied
functions start with uppercase letters.

NOTE: The current version of the program is not guaranteed to enforce these
restrictions.

### Selectors {#triage-selectors}

Selectors use the [Selector] format. The text before the first `:` selects the
component name from the `inspect.json` file. The `.`-separated middle section
specifies Inspect Node names forming a path to the Property named after the
second `:`.

[Selector]: /docs/reference/diagnostics/selectors.md

### Calculation {#triage-calculation}

Eval strings are infix math expressions with normal operator precedence.

() may be used.

Arithmetic operators are `+` `-` `*` `/` `//` `/?` `//?`. `/` is float division; `//` is int
division. Division operators with a `?` return Problem::Ignore on division by 0.

Functions are a function name, '(', comma-separated expression list, ')'.
Provided functions include:

*   Boolean
    *   `And(1+ args)`
    *   `Or(1+ args)`
    *   `Not(1 arg)`
    *   `Missing(value)` returns true if the value is a `Missing` type error
        indication.
    *   `Problem(value)` returns true if the value is any kind of error.
    *   `True()` returns true
    *   `False()` returns false
*   Numeric
    *   `Min(1+ args)`
    *   `Max(1+ args)`
    *   `Abs(1 arg)`
*   Functional
    *   `Fn([name1, name2, ...], expression)`
    *   `Map(function, vector1, vector2, ...)`
    *   `Fold(function, vector, optional_start_value)`
    *   `Filter(function, vector)`
    *   `Apply(function, [arg1, arg2, ...])`
    *   `Count(vector)`
*   Time
    *   `Days()`, `Hours()`, `Minutes()`, `Seconds()`, `Millis()`, `Micros()`,
        and `Nanos()` calculate values for comparison with monotonic timestamps.
    *   `Now()` returns the approximate timestamp when the Diagnostic data was
        created.
*   Other
    *   `Option(value1, value2, value3...)` returns the first useful value, to
        support selector migrations and defaults: the first non-empty-list,
        non-Missing value if any; or empty list if one was given; or Missing.
    *   `Annotation(string)` fetches the corresponding key from
        the annotations.json file, if present.
    *   `SyslogHas(regex)`, `KlogHas(regex)`, `BootlogHas(regex)` return `true`
        if the syslog, kernel log, or previous-boot log contain a line matching
        the regex.
    *   `StringMatches(value, regex)` applies the given regex to the given
        value and returns true if there is a match. The regex syntax is that
        supported by the Rust [regex crate].

Metric type follows the type read from the Inspect file. Currently, UInt is
converted to Int upon reading. Operating on mixed Int and Float promotes the
result to Float.

Boolean operations are `>` `<` `>=` `<=` `==` `!=`. The equality tests `==` and
`!=` compare
numbers, Booleans, strings, and vectors. `>` `<` `>=` `<=` only compare numbers.

Whitespace is optional everywhere, but recommended around infix operators.

Metric names, including namespaced names, do not need to be specially delimited.

[regex crate]: https://docs.rs/regex/latest/regex/

#### Functional programming and vectors {#functional-programming}

Every selector actually returns a vector, but one-item vectors are
automatically unwrapped for the purposes of arithmetic and boolean
calculations. Inspect selectors without wildcards return a one-item
vector unless the same moniker occurs multiple times in inspect.json.

Selectors with wildcards, selectors for `bootstrap/driver_manager` and
`netstack.cmx`, and (eventually) selectors on logs, may return
multiple items in a vector. To process such values, Triage provides the
following functions:

*   Fn(parameters, expression) - for example, "Fn([a, b], a+b)"
*   Map(function, vector1, vector2...)
*   Fold(function, vector) or Fold(function, vector, start_value)
*   Filter(function, vector)
*   Count(vector)

Vectors of values are written `[ expr, expr, expr ]`.

If a `values` argument to Map is not a vector, its value is applied to each
iteration. If all `values` are not vectors, or no `values` are supplied, an
empty vector is returned. If the vector `values` are different lengths, the
shortest one determines the result length and remaining values are not used.

Count() does not check the type of items in a vector `values`. Count() of a
non-vector `values` returns Missing.

If a Fn expression is the entirety of an 'eval' expression, the name of that
expression can be used as the first argument to Map, Fold, or Filter.

If a function has the wrong arity for its arguments, the function it was passed
to returns Missing. If the function's evaluation fails, for example due to
inappropriate types, the function it was passed to may return a partial value:

*   Map returns a vector, some elements of which may be Missing.
*   Fold returns Missing.
*   Filter expects its filter function to return Boolean true or false. If that
    function returns anything else, including Missing, Filter adds a Missing
    value at that point in its result list.

## Actions {#triage-actions}

Each Action determines how to surface information for a given selector.
Currently, there are two types of actions, "Warning" and "Gauge". Actions are
specified by providing the appropriate value for the `type` field.

### Warning Type {#triage-warnings}

A `Warning` is an action that is used to raise an alert when a boolean condition
is met.

`Warning` supports the following fields:

*   `trigger`, a required field, specifies the name of a Metric that supplies a
    Boolean value.
*   `print`, a required field, specifies a string to output when the warning is
    raised.
*   `tag`, an optional field, associates a tag with this Action.
*   `file_bug`, an optional string field, specifies that a bug should be filed
    and where. Triage does not file bugs directly; this field simply informs the
    consumer (a human user or an automated pipeline). It can be a Monorail
    component (e.g. "I18N>Fonts") or references to other issue trackers
    understood by the consumer.

```json5
    "actions": {
        "disk_usage_high": {
            "type": "Warning", "trigger": "disk_used / disk_total > 0.95", "print": "Disk usage is high!"
        }
    }
```

### Gauge Type {#triage-gauges}

A `Gauge` is a snapshot of a particular value at the time Triage is invoked.
`Gauge` supports the following fields:

* `value`, a required field, specifies a value to display.
* `format`, an optional field, specifies formatting rules for the gauge's value.

#### Format {#triage-gauge-format}

The `format` field allows users to control how the gauge value is displayed. If
this field isn't provided, or if an invalid value is given, then value will be
displayed as is. `format` supports the following values:

* `percentage`: prints a float as a percentage value.

```json5
    "actions": {
        "disk_usage": {
            "type": "Gauge", "value": "disk_used / disk_total", "format": "percentage"
        }
    }
```

## Tests {#triage-tests}

Each Test specifies:

*   Sample data, keyed by `inspect`
*   A list of actions that should trigger given that data, keyed by `yes`
*   A list of actions that should not trigger given that data, keyed by `no`

The sample data is in the same format as an inspect.json file: an array of maps
where each map contains `path` and `contents` fields.

```json5
    "tests": {
        "test1": {
            "yes": ["action1", "action2"],
            "no": ["action3"],
            "inspect": [
                {
                    "path": "global_data",
                    "contents": {"root": {"stats":
                        {"total_bytes": 10, "used_bytes": 9}}}
                }
            ]
        }
    }
```
