# Bitfield diagram explanation

This document explains how to read the bitfield diagrams used in the
[logs][log_format] and [inspect][inspect_format] formats.

The following example bitfield diagram is broken down into its component parts:

```
.---------------------------------------------------------------.
|       |1|1|1|1|1|2|2|2|2|2|3|3|3|3|3|4|4|4|4|4|5|5|5|5|5|6|6|6|
|2|4|6|8|0|2|4|6|8|0|2|4|6|8|0|2|4|6|8|0|2|4|6|8|0|2|4|6|8|0|2|4|
|---+---+-------+-----------------------+-----------------------|
| O | R | Type  | Parent index          | Name index            |
|---------------------------------------------------------------|
| Reference count (optional)                                    |
'---------------------------------------------------------------'

O = Order
R = Reserved, must be 0
Type = {3,10}
```

## Top row

In the top row, each column denotes the bit index. For example, the first “2” in the left-most
column denotes that the space is reserved for the bits at index 0 and 1. The next “4” at index 2
and 3, and so on. The first column with two numbers should be read as "10", the next "12", and so
on.

```
.---------------------------------------------------------------.
|       |1|1|1|1|1|2|2|2|2|2|3|3|3|3|3|4|4|4|4|4|5|5|5|5|5|6|6|6|
|2|4|6|8|0|2|4|6|8|0|2|4|6|8|0|2|4|6|8|0|2|4|6|8|0|2|4|6|8|0|2|4|
|---+-----------+-----------------------------------------------|
```

## Bottom rows

The bottom rows of the diagram illustrate the physical layout of the fields. Each row consists of
64 bits.

```
|---+---+-------+-----------------------+-----------------------|
| O | R | Type  | Parent index          | Name index            |
|---------------------------------------------------------------|
| Reference count (optional)                                    |
'---------------------------------------------------------------'
```

In the example, we are representing 128 bits. The first 64 bits contain:

- `O`: 4 bits at indexes `[0, 4)` (left inclusive, right exclusive, so in this
  case the indexes `{0, 1, 2, 3}`).
- `R`: 4 bits at indexes `[4, 8)`
- `Type`: 8 bits at indexes `[8, 16)`
- `Parent index`: 24 bits at indexes `[16, 40)`
- `Name index`: 24 bits at indexes `[40, 64)`

The second 64 bits contain a single value, the `Reference count`.

## Notes

At the end of the diagram, there is a legend to make it clearer what some fields mean or what
restrictions they might have. In the example above, `R` means `Reserved` and `O` means `Order` but
there wasn’t enough space in the diagram to fit those words. `Type` will only contain values in the
set `{3, 10}`.


[inspect_format]: /docs/reference/diagnostics/inspect/vmo-format.md
[log_format]: /docs/reference/diagnostics/logs/encoding.md
