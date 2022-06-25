# CLI tool help requirements


## Overview

Command line help, as often provided through `--help` is an important means of
communication with the user. It provides a shorthand for more detailed
documentation and feature discovery.


## Guide

Help documentation must include:
* [Usage](#usage)
* [Description](#description)

Help documentation may additional include the following sections (as needed):

* [Options](#options)
* [Commands](#commands)
* [Examples](#examples)
* [Notes](#notes)
* [Error codes](#errorcodes)

### Example

Each section of this example is described in detail later in this document.
(Note that `blast` is not a real tool).

```text
Usage: blast [-f] [-s <scribble>] <command> [<args>]

Destroy the contents of <file>.

Options:
  -f                force, ignore minor errors. This description
                    is so long that it wraps to the next line.
  -s <scribble>     write <scribble> repeatedly
  -v, --verbose     say more. Defaults to $BLAST_VERBOSE.

Commands:
  blow-up         explosively separate
  grind           make smaller by many small cuts
  help            get help on other commands e.g. `blast help grind`

Examples:
  Scribble 'abc' and then run |grind|.
  $ blast -s 'abc' grind old.txt taxes.cp

Notes:
  Use `blast help <command>` for details on [<args>] for a subcommand.

Error codes:
  2 The blade is too dull.
  3 Out of fuel.
```

#### General layout and style

See above for an example that follows these requirements.

Several sections call for **English prose**. This means writing in proper
sentences using English grammar with US English spelling (as opposed to British
English or others). Use one space between sentences, and adhere to the "Oxford
comma" style. E.g. "The Description, Examples, and Notes sections are written in English
prose."

**Layout**

- Each section is separated by one blank line.
- Section contents are indented two spaces.
- Single line contents will be written on the same line as the label with one
  space separating the colon and the command name. E.g. "`Usage: blast <file>`".
- Multi-line sections will be written immediately following the label (without a
  blank line). Each line after the label will be indented two spaces.
- All output is a single column of text with the exception of Options, Commands,
  and Error codes, which are two column tables.
- Use spaces for indentation or alignment. Do not output tab characters.
- Wrap text at 80 columns. When wrapping Option or Command descriptions, align
  the subsequent lines with the start of the description (e.g. about 20
  characters in).

**Style**

- Section titles appear in title-case: capitalize the first and last words. All
  words in between are also capitalized except for articles (a, an, the),
  conjunctions (e.g., and, but, or), and prepositions (e.g., on, in, with).
- Short description fragments (the second column in Options or Commands) begin
  with a lowercase letter and are not expected to be full sentences. Any text
  beyond the short description should be complete sentences, with a period after
  the fragment.
- Try to keep each section concise. If there is more to say, direct the user to
  use `--verbose` when running `--help` or direct them to the full
  documentation.
- Unicode (UTF-8) characters are allowed in descriptive text (prose). The
  command name and usage text will only contain portable ASCII characters
  (without Unicode).

### Usage {#usage}

Usage is required and includes the "`Usage:`" header.

```text
Usage: blast [-f] [-s <scribble>] <command> [<args>]
```

Usage will commonly be a single line, though multiple lines can be used to
clarify when options are mutually exclusive. If the number of lines needed to
present all the mutually exclusive scenarios becomes excessive, limit the lines
to some common cases and give more details in the full docs. If there are many
mutually exclusive options, consider making subcommands or separate tools to
reduce the complexity.

#### Usage syntax

The command name is listed first. The command name is not hardcoded: it will be
dynamically pulled from the command name, i.e. the last element on the `argv[0]`
path. This allows a single binary to operate as multiple different tools.

The name and usage text will contain portable ASCII characters only. All long
form commands are entirely lowercase, i.e. never all-caps or mixed-case. Single
letter switches should prefer lowercase, but uppercase is allowed.

Use meaningful words for options and placeholders. Avoid abbreviations. Prefer
single words. When more than one word is used, separate words with a hyphen
(`-`), i.e. do not use underscores, camel-case, or run words together.

Aside from the command name there are several kinds of arguments (as described
in [Fuchsia Tool Requirements](http://go.corp.google.com/fuchsia-tool-requirements)).

- Exact text
- Arguments
- Options (Switches and Keys)
- Keyed options
- Option delimiter

##### Exact text syntax

Exact text is written as-is in the usage line. In the example "`Usage: copy
<from> to <destination>`", the word `to` is required exact text. If exact text
is optional, it will be enclosed in brackets (`[]`) in the usage line: "`Usage:
copy <from> [to] <destination>`".

##### Argument syntax

Arguments are enclosed in angle brackets (<>) to differentiate them from
explicit text. In the example `Usage: copy <from> <destination>`, both `<from>`
and `<destination>` are arguments. If an argument is optional, it will be
enclosed in brackets (`[]`) such as: `Usage: copy <from> [<destination>]`. See
also [Option Delimiter](./cli.md#option_delimiter).

##### Mutually exclusive option syntax

There are a couple choices when illustrating mutually exclusive options.

If more than one usage line is provided, each will show a mutually exclusive set
of commands. For example:

```text
Usage:
  swizzle [-z] <file>
  swizzle --reset
```

Indicates that `--reset` and usage with a `<file>` are mutually exclusive
options.

Another way to specify mutually exclusive options is using a vertical bar ('|')
between the options. Note that when a vertical bar is used to indicate data flow
between processes it is called a "Pipe." When used to separate options it is
read as "Or".

For example:

```text
Usage: froth [-y|-z] <file>
```

Indicates that `-y` **_or_** `-z` switches can be used (or neither, since they
are optional), but it's senseless to use both together (they are mutually
exclusive options). To indicate that either value must be used, but not both,
wrap the choices in parentheses, e.g. "`Usage: froth (-a|-b) <file>`" means that
`-a` **_or_** `-b` must be passed.

Note that it's common that `--version` or `--help` causes other arguments to be
ignored and is seldom listed as such. Listing them as separate usage lines is
considered unnecessary.

##### Grouping (implied) options

There is no specific syntax to indicate when enabling one option will also
affect another option. When an option implies that another option is enabled or
disabled, specify that in the Options. E.g. "`passing -e implies -f`" means that
if `-e` is enabled, `-f` will be enabled as if it were passed on the command
line (regardless of whether `-f` was explicitly passed). The redundant passing
of the implied value is harmless (not an error).

Document the implication in the primary switch. E.g. if `-x implies -c and -p`
place that note in the description of `-x` but not in `-c` and `-p`. This is to
keep the `--help` output concise (this rule can be relaxed in the full
documentation).

##### Optional keys

To create the appearance of a keyed option with an optional Key, create optional
exact text followed by an argument. For example "`Usage: copy [from] <from> [to]
<destination>`". In the example, all of these are valid: "`copy a b`", "`copy
from a b`", "`copy from a to b`", "`copy a to b`".

##### Repeating options

If the same positional argument may be repeated, indicate that with an ellipsis
('...'). Rather than a Unicode ellipsis, use three consecutive periods. For
example: "`Usage: copy <from> [<from>...] <to>`" means the last argument is
always interpreted as the `<to>`, while the preceding values are multiple
`<from>` entries. Note that "`<from> [<from>...]`" means there is one or more
`<from>` entries, while "`Usage: copy [<from>...] <to>`" means zero or more
`<from>` entries are accepted.

For Key/Value pairs that may be repeated, group them with brackets (if the pair
is optional) or parentheses (if the pair is not optional) and add an ellipsis to
the group, e.g. `[--input <file>]...` or `(--input <file>)...` respectively.

##### Brackets

Angle brackets (`<>`), brackets (`[]`), and parentheses (`()`) will not have
spaces immediately inside.

```text
[from] # correct
<to> # correct
(-a|-b) # correct

[ from ] # incorrect
< to > # incorrect
( -a|-b ) # incorrect
```

Angle brackets (<>) wrap Arguments or Key values.

Brackets (`[]`) wrap optional elements. With nested angle brackets, such as
`[<file>]`, interpret the `<file>` as an Argument that is optional. The nested
"`[<`" is not a separate bracket style, it is a "`[`" with a "`<`" within it.
When nesting, the brackets (`[`) will be outermost (do not use `<[file]>`).

Parentheses (`()`) are used to group elements. Use parentheses when they improve
clarity, such as with required mutually exclusive options.

Braces (`{}`) are reserved for future use. This guide intentionally leaves open
the possibility for braces to have special meaning in a future revision of this
document.

### Description{#description}

The description is required and does not include a header. I.e. the description
area is not labeled "description". E.g.

```text
Destroy the contents of <file>.
```

The description is written in US English prose (complete sentences using US
English grammar, spelling, and punctuation).

Every tool should tell you what it does and this is the section to do that.

The Description section should describe

- what the tool does (required)
- the platform configuration used
- schemes, data formats, or protocols used
- golden workflows (critical developer journeys)
- a broad URL to documentation (e.g. fuchsia.com/docs or similar, avoid deep
  links that go stale to easily)

The Description section can also contain a "see also" referring to another tool
by name (avoid using a URL).

What not to put in the Description section

- environment variables used, other than those already listed in Options
  (provide this in Options or Notes)
- security hazards (explain these in the Notes section)
- error codes (put those in an Error codes section)
- copyright (don't include this in the `--help`)
- author (don't include this in the `--help`)
- how to get help on subcommands (put this in the short description for the
  `help` subcommand)
- how to update the tool (that should be in the documentation for the tool
  package, if applicable)
- release notes (use a separate file)

### Options{#options}

An Options section is required if the program accepts arguments. E.g.

```text
Options:
  -f              force, ignore minor errors
  -s <scribble>   write <scribble> repeatedly. Defaults to $BLAST_SCRIBBLE.
```

The listed options apply to the tool itself and not to a subcommand. Options for
individual subcommands are listed when requesting help for that subcommand, e.g.
when using `blast help grind` or `blast grind --help`.

Try to keep options to a single, complete word. If two words are needed,
separate the words with a hyphen (`-`). Avoid uncommon abbreviations.

Present the list of options in alphabetical order.

Options will list each argument, switch, and keyed option on separate lines with
the exception of arguments that have both a short and long form. If an argument
has both a short and long form they are listed on the same line, short form
first, and separated by `, ` (comma space), e.g. `-f, --force`.

Exact text arguments will not be listed in the Options section. They are shown
in the Usage section.

Text that will be typed as-is is not wrapped in brackets, while variable entries
appear in angle brackets (`<>`) and optional entries appear in square brackets
(`[]`). When listing options, the Key is never optional. For example:

```text
  -a                   a good example
  [-b]                 a bad example, to use -b it must be typed as-is
```

A short description will follow each option. There's no limit on the length of
this description, but be concise. Try to put more details in the overall tool
description, the Examples, or the Notes instead of creating a lengthy option
description.

What to describe

- a brief reminder of what the option implies, e.g. `ignore minor errors`
- if the option overrides another option, e.g. `-x implies -c and -p`
- default value, e.g. `defaults to $BLAST_SCRIBBLE`

The column on which the description sentence fragment begins may vary depending
on the needs of the tool. Use 20 characters from the left edge if it looks okay,
but adjust if a bit more or less reads better.

If there is a large number of options, consider showing a useful subset and
explaining how to get further help to see all of them, e.g. by passing
`--verbose` along with `--help`.

### Commands{#commands}

A commands section is required if the program has subcommands. If present it
will be labeled, "Commands:". E.g.

```text
Commands:
  blow-up         explosively separate
  grind           make smaller by many small cuts
  help            get help on other commands e.g. `blast help grind`
```

If the program does not have subcommands, the commands section will not be
present.

When a tool has subcommands, it will also have a `help` command to get further
help on the subcommands, i.e.` blast help grind`.

Try to keep subcommands to a single, complete word. If two words are needed,
separate the words with a hyphen (`-`). Avoid uncommon abbreviations. Present
the list of commands in alphabetical order.

Each command name appears with a short description on a separate line. For a
more lengthy command description, the user will specifically ask for help on
that command. This description serves as a short reminder of the command and to
assist in discovery of commands.

If there is a large number of commands, consider showing a useful subset and
explaining how to get further help to see all of them, e.g. by passing
`--verbose` along with `--help`.

### Examples{#examples}

An examples section is optional. If present it will be labeled, "Examples:".
E.g.

```text
Examples:
  Scribble 'abc' and then run |grind|.
  $ blast -s 'abc' grind old.txt taxes.cp
```

Each example will have US English prose (i.e. complete sentences using US
English grammar, spelling, and punctuation) describing the example, followed by
an example command line. Each line that would be entered on the command line
literally will be prefixed with a "`$ `" to mimic a command prompt.

To wrap an example that is overly long, end the previous line with "`\ `" and
begin subsequent lines with "`  `" (spaces) to indicate line continuation.

```text
  This example wraps onto multiple lines.
  $ blast -s 2332 some/long/path/cats.o \
    more/long/path/dogs.o more/long/path/bears.o \
    more/long/path/deer.o
```

If it is helpful to show some of the output from the example command, write the
output immediately following the example.

Separate examples with one blank line.

If the Examples section is getting overly long, move examples to a help doc.
Interactive help examples are for quick reference and discoverability rather
than exhaustive documentation.

### Notes{#notes}

Notes are optional and begin with a "Notes:" header. E.g.

```text
Notes:
  Use `blast help <command>` for details on [<args>] for a subcommand.
```

The notes are written in US English prose (i.e. complete sentences using US
English grammar, spelling, and punctuation).

What to put in the Notes

- environment variables used, other than those already listed in Options (for
  default values)
- security hazards
- reminders to help the user

What not to put in the Notes

- error codes (put those in an Error codes section)
- copyright (don't include this in the `--help`)
- author (don't include this in the `--help`)
- how to get help on subcommands (put this in the short description for the
  `help` subcommand)
- how to update the tool (that should be in the documentation for the tool
  package, if applicable)
- release notes (use a separate file)

### Error codes{#errorcodes}

The Error codes section is required if codes other than `0` or `1` are
generated. E.g.

```text
Error codes:
  2  The blade is too dull.
  3  Out of fuel.
```

This section is omitted if only `0` or `1` results are generated by the program.

Error code `0` is always treated as "no error" and error code `1` is always a
"general error". Neither are documented in the `--help` output. Every error code
other than `0` or `1` that may be generated by the tool must be documented.

### Platform specifics

Some platforms (e.g. DOS) use a different option prefix (e.g. `/`) or may allow
case insensitive switches. Tools will use a dash prefix (`-`) and case sensitive
options regardless of the platform. This means that the documentation for a tool
generally doesn't need to consider the platform being used.

### What not to include in --help output

Do not show Key/Value pairs with an equals sign (`=`), e.g. `--scribble=abc`.
The Key and Value are parsed using whitespace as a delimiter (`--scribble abc`).
Showing the equals in the help is potentially confusing.

Do not implement a pager (something like the `more` program that pauses output
on each screenful of text).

Do not include

- A copyright in the help output (put that where legal specifies).
- Release notes (put that in release notes).
- Full documentation (put that in the markdown documentation).
- Version information (output that from `--version`).
- Documentation on result codes `0` or `1` (put in .md docs).
- Shell-specific help (such as how to redirect output or pipe to a pager).
