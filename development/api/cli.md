# Command-line Tools Rubric


## Overview

This document is for command line interface (CLI) tools. Graphical User
Interfaces (GUI) are out of scope.

When developing tools for Fuchsia there are specific features and styles that
will be used to create consistency. This document walks through those
requirements.

The goal is to maintain a uniform fit and finish for Fuchsia developer tools so
that developers can know what to expect. They can most easily see how to
accomplish common tasks and there is a well lit path to discover rarer used
tools.


## Guide

The experience developers have writing software for Fuchsia will impact their
general feelings toward writing for the platform and our tools are a significant
part of that experience. Providing tools that are inconsistent (with one
another) creates a poor developer experience.

This guide provides a rubric that Fuchsia tools must follow.

> **IDK**
>
> Some sections have an "IDK" call-out, like this one. These detail specific
> rules that apply to tools included with the Fuchsia Integrator Development Kit distribution.

## Considerations

Before embarking on the creation of a new tool, consider these factors to
determine if the tool is a good fit for Fuchsia or the Fuchsia SDK.

> **IDK**
>
> IDK tools are specific to Fuchsia in some way. Generic tools or tools that are
> widely available should not be part of Fuchsia and will not be included in the
> Fuchsia IDK. For example, a tool that verifies generic JSON files would not be
> a good addition. However a tool that verifies Fuchsia `.cml` files, which
> happen to use the JSON format, would be okay.

> **ffx**
>
> [ffx](/development/tools/ffx/overview.md) is Fuchsia's unified CLI
> tool platform for host to target interactions. It provides a logical
> subcommand based grouping that maps to high-level Fuchsia workflows.
> It also provides a plugin framework to allow contributors to expand the
> `ffx` command surface. `ffx` is distributed as part of the Fuchsia IDK.

### Audience

Tools may be used for different development tasks. On a large team these roles
may be separate people. Some categories are:

- Component development
- Driver development (DDK)
- Fuchsia development (SDK)
- Build integration (GN, etc.)
- Quality assurance (QA)
- System integrators (e.g., on-device network tools)
- Publishing (from dev host to server)
- Deployment (from server to customers)

Consider which users may use a tool and cater the tool to the audience.

Tools may have different integration expectations. For example, a developer
doing component development may expect tools to integrate with their Integrated
Development Environment (IDE), while a build integration tool may be called from
a script.

### Grouping Related Tools

Prefer to put related commands under a common tool, such as `ffx`.
As an example, `git`, `ffx`, or `fx` present many features (or,
"sub-tools") under a single user-facing command. This helps encourage
the team toward a shared workflow and provides a single point of
discovery.

Prefer subcommands to multiple tools. E.g. don't create tools with hyphenated
names like `package-create` and `package-publish`, instead create a `package`
command that accepts create and publish subcommands.

Keep the number of commands under a tool organized and reasonable. I.e. avoid
adding unrelated commands to a tool and provide sensible organization of the
commands in the help and documentation.

### Scope of Work

Command line tools can be divided into two groups: simple single purpose tools
and larger more featureful tools. Create tools that are ergonomic for their
purpose. Simple tools should be quick to start up, while more complex tools will
lean toward the more featureful.

Larger tools will encompass an entire task at the user (developer) level. Avoid
making a tool that accomplishes one small step of a task; instead make a tool
that will perform a complete task.

For example, when:

- developing a C++ application: run the preprocessor, run the compiler, run the
linker, start the built executable.
- working on a unit test: build the tests and run the tests being worked on
- developing a mod: compile the code, move the code and resources to the device,
start the mod (or hot-reload)

Lean toward a tool that will accomplish all the steps needed by default, but
allow for an advanced user to do a partial step (for example, passing an
argument to ask the C++ compiler to only run the preprocessor).

> **IDK**
>
> For development environment integrators and EngProd teams, separate tools.
> The build integrators will learn each and piece them together to make a
> working system.

> **ffx**
>
> `ffx` introduces many subgroupings and related subcommands.
> In general for tools that fall in the categories such as host to target
> interaction, system integration, and publishing, prefer extending the
> existing `ffx` service instead of a new standalone tool. This can be accomplished
> by extending `ffx` via additional flags, options or subcommands to take
> advantage of shared code and functionality. For considerations and
> additional details refer to the `ffx` development
> [overview](/development/tools/ffx/overview.md).

#### Sharing common functionality

If a small step of a task will be needed by several tools, it doesn't make sense
to duplicate that code. Consider making a small support tool or create a library
to share the code.

Making a small tool that performs one step of the task can make sense to promote
code reuse. If the user is not expected to run this small tool individually,
place the support tool in a directory that is not added to the `$PATH`. I.e.
avoid polluting the environment path unnecessarily.

Providing a library to share code may be preferable, so that a subprocess isn't
needed.


## Implementation

Here is some guidance for the nuts and bolts of creating a tool. We'll cover
which language to write the tool in, what style to use in that language, and so
on.

> **ffx**
>
> `ffx` follows the rubric and conventions laid out below and provides
> a reference implementation for the outlined recommendations.

### Naming

The following applies to names of binaries, tools, sub-commands, and long
parameter flags.

Use well-known US English terms or nouns for names. Well-known nouns includes
those in common use for the subject matter, or the names of whole subsystems.
If a name does not appear in documentation, it is likely not well-known. If
it does not appear in any implementation, it is definitely not well-known.

Only use lower-case letters (`a-z`) in the US-ASCII character set and hyphens.
A single hyphen (`-`) is used to separate words in a name. A Platform
required extension is an exception (such as `.exe`).

Name CLI tools with more than three characters. Keep the short file names
available for user shortcuts (aliases). If you believe a tool should have
a very short name, request approval from the Fuchsia API Council.

Keeping the points above in mind:

- Prefer whole words rather than abbreviations.
- Prefer shorter names where a user is expected type the name frequently. For
  less frequently typed names bias to more explicit names.
- Prefer a single word to multiple words
- Prefer subcommands to multiple tools that are hyphenated (e.g. avoid
  `foo-start`, `foo-stop`, `foo-reset`; instead have `foo` that accepts
  commands `start|stop|reset`).
- Prefer symmetry (particularly in verbs) with other similar commands or
  sub-systems, unless that introduces a broken metaphor.

### Programming Languages

Tools may be written in C++, Rust, and Go. For clarity, here are some languages
not approved: Bash, Python, Perl, JavaScript, and Dart (see exceptions below).

No language is preferred between C++, Rust, and Go. The choice between these
languages is up to the author of the tool.

> **IDK**
>
> If a SDK that is an integration of the Fuchsia IDK includes a specific language
> (e.g. Dart), that language may be used for tools that are distributed with
> that SDK. In other words, do not include a Dart tool in a SDK that
> wouldn't otherwise include the Dart runtime, but if it's already there,
> that's okay.

### Style Guides

Follow the corresponding [style guide](/development/languages/README.md)
for the language and area of Fuchsia being developed. For example, if the tool
is included with Zircon and written in C++, use the style guide for C++ in
Zircon. Specifically, avoid creating a separate style guide for tools.

### Runtime Link Dependencies

Try to minimize runtime link dependencies (statically link dependencies
instead). On Linux it is acceptable to runtime link against the glibc suite of
libraries (libm, etc.); other runtime link dependencies are not allowed.

### Building from Source

Keep in mind that some developers will want to build the tools from source. Use
the same build and dependency structure as the code in the Platform Source Tree.
Do not make a separate system to build tools.

## Host Platforms

Keep an eye on how resource heavy a tool becomes and what OSes it will be
expected to operate on.

### Run on a Variety of Hardware

Developer machines may range from a few CPU cores and moderate amount of RAM to
dozens of CPU cores and huge amounts of RAM. Don't assume that host machines are
very powerful or that a server cluster is available to offload work to.

### Supported OSes

This section is for the convenience of the reader. This document is not
authoritative on which platforms are supported.

We currently support

- Linux
- macOS

Tools written for developers must run on those platforms. There are other
platforms to consider, and while these are not required at this time, it's good
to keep the platforms listed below in mind.

Tools should be built in a way that makes them easy to port to the following
platforms:

- Fuchsia (self-hosted)
- Windows

This is not an exhaustive list, we may support others.

### Case Insensitive File Systems

Don't rely on case sensitivity in file paths. E.g. don't expect that `src/BUILD`
and `src/build` are different files. Conversely, don't rely on case
insensitivity since some platforms are case sensitive.

### Development Hosts Using a Non-English Locale

There are several aspects to consider for non-English developers:

- Whether the tool itself can be localized
- Whether the documentation for the tool can be localized
- Whether the tool can work with path names and data that include non-ASCII
- Whether the tool works correctly on non-English OSes

Tools are provided in US English. It's not required that a tool be localized.
(This may change in the future.)

The documentation for a tool will support non-ASCII characters. Both HTML and
Markdown can support Unicode (UTF-8) characters, so these are both good choices
for documentation. Doing the translation is not required, merely allow for the
possibility.

Tools will function properly with file paths that contain binary sequences and
white space. Use a library to work with file paths rather than manipulating
paths as strings. (e.g. path.Join in Go.)

Tools will operate correctly on non-English platforms (e.g. Japanese or French).
This means handling binary (e.g. UTF-8) data without corrupting it. E.g. don't
assume a text file is just ASCII characters.


## Execution

At runtime (or execution time) consider how the tool should behave.

### Optimize for No Work Needed

When appropriate, such as with a build tool, have the tool exit quickly if there
is no work to do. If possible, go one step better by providing information to
the caller about the dependencies so that the caller can accurately determine
whether the tool needs to be called at all.

### Command Line Arguments

There are three types of command line arguments:

- exact text
- arguments
- options (i.e. switches and keys)

#### Exact text

Exact text is placed as-is on the command line. A piece of exact text may be
required or optional. Parsing exact text arguments should be restricted to cases
where they are needed for disambiguation (i.e. for correctly parsing other
arguments). For example if a `copy` command accepted multiple source and
destination arguments, an exact text argument may be used to clarify which is
which: `copy a b c` may be ambiguous; while `copy a to b c` may indicate that
'`a`' is copied to two destinations.

#### Arguments

Arguments are like function parameters or slots for data to fit into. Often,
their order matters. In the example `copy <from> <destination>`, both `<from>`
and `<destination>` are ordered arguments. In cases where a single logical
argument is repeated the order may not matter, such as remove `<files>...` where
the tool might process the `<files>` in an arbitrary order.

#### Options

Some arguments are known as options. Both switches and keyed (key/value pairs)
are options. Options tend to modify the behavior of the tool or how the tool
processes parameter arguments. Options consist of a dash prefixed letter or
word.

Options must start with either one ('`-`') or two ('`--`') dashes followed by an
alphanumeric label. In the case of a single dash, the length of the label must
be 1. If the length of the label is two or more, then two dashes must be used.
For example: `-v` or `--help` are correct; `-help` is not valid.

For option names with more than one word (for example, "foo bar"),
you must use a single dash ('`-`') between words. For example, "foo bar"
becomes `--foo-bar`.

All choices are required to have a (`--`) option. Providing single character
shorthand (`-`) is optional. E.g. it's okay to provide just `--output`, or both
`-o` and `--output`, but it's not ok to only provide an `-o` option without a
long option as well.

Do not create numeric options, such as `-1` or `-2`. E.g. rather than having
`-1` mean to do something once, add a `--once` option. If a numeric value is
needed, make a keyed option, like `--repeat <number>`.

One (`-`) or two (`--`) dashes on their own are special cases and are not
allowed as a key or switch.

#### Switches

The presence of a switch means the feature it represents is 'on' while its
absence means that it is 'off'. Switches default to 'off'. Unlike keyed options,
a switch does not accept a value. E.g. `-v` is a common switch meaning verbose;
it doesn't take a value, making it switch rather than a keyed value.

All switches must be documented (hidden switches are not allowed).

Running switches together is not allowed. E.g. `-xzf` or `-vv`, each must be
separate: "`-x -z -f`" or "`-v -v`".

#### Keyed Options

Keyed options consist of a key and a value. Keys are similar in syntax to
switches except that a keyed option expects a value for the key.
E.g. `-o <my_output_file>` has a key '`-o`' and a value of '`my_output_file`'.

Do not use an equals punctuation (or similar) to separate the key and value.
E.g. do not do `-o=<my_output_file>`.

Note about a rare case: Avoid making optional keys (where the value appears
without its key) or optional values (where the key appears without its
value). It's clearer to consider the key/value pair optional, but inseparable.
I.e. if the key is present a value is required and vice versa. Consider making
an argument instead of a keyed option with an optional key. E.g. rather than
"`do-something [--config [<config_file>]]`" where not passing `[<config_file>]`
means don't use a config file; instead do
"`do-something [--config <config_file>|--no-config]`" where passing
`--no-config` means don't load a config file.

##### Mutually Exclusive Options

Some options don't make sense with other options. We call the options mutually
exclusive.

Passing mutually exclusive options is considered a user error. When this occurs
the tool will do one of the following:

- Write an error message explaining the issue and exit with a non-zero result
  code; doing no work (i.e. there was no data changed as a result of the call).
  This is the expected handling, so no further documentation or notes are
  required.
- Prioritize one option over another. E.g. "`passing -z will override -y`". In
  this case the handling will be documented in the `--help` output.
- Other handling is possible (first takes precedence or last takes precedence or
  something else) though this is discouraged. In this case the handling will
  be documented in the Description, Options, ***and*** Notes; though
  "`See Notes`" may be used in Description and Options with the full write-up in
  `Notes`.

##### Grouping Options

There is no specific syntax to indicate when enabling one option will also
affect another option. When an option implies that another option is enabled or
disabled, specify that in the Options. E.g. "`passing -e implies -f`" means that
if `-e` is enabled, `-f` will be enabled as if it were passed on the command
line (regardless of whether `-f` was explicitly passed). The redundant passing
of the implied value is harmless (not an error).

##### Option Delimiter

Two dashes ('`--`') on their own indicates the end of argument options. All
subsequent values are given to the tool as-is. For example, with
"`Usage: foo [-a] <file>`", the command line "`foo -- -a`" may interpret `-a` as
a file name rather than a switch. Further, "`foo -a -- -a`" enables the switch
`-a` (the first `-a`, before the `--`) and passes the literal text `-a` (the
second `-a`).

##### Repeating Options

Repeating switches may be used to apply more emphasis (what more emphasis means
is up to the tool, the description here is intentionally vague). A common
example is increasing verbosity by passing more `-v` switches.

Repeating keyed options may be used to pass multiple values to the same command.
Often this is done to avoid calling the same command multiple times. Common
commands that accept repeating options are `cp`, `rm`, `cat`. Care must be taken
to ensure that repeating commands are unambiguous and clear. E.g. `cp` always
interprets the last argument as the destination; if `cp` accepted multiple
source and destination arguments the parsing would become ambiguous or unclear.

#### Standard Input Alias

In Fuchsia tools a single dash (`-`) is not interpreted as an alias to stdin. Use
pipes to direct data into stdin or use `/dev/stdin` as an alias for stdin.
(Note: `/dev/stdin` is not available on Fuchsia or Windows).

#### Single Dash

A single dash ('-') on its own is reserved for future use.

#### Subcommands

Tools may contain sub-command that accept independent command line arguments.
(Similar to the `git` tool). Subcommands do not begin with any dashes. E.g. in
`fx build` the `build` argument is a subcommand.

When a tool has many subcommands, it should also have a help subcommand that
display help about other subcommands. E.g. "`fx help build`" will provide help
on the build subcommand.

Subcommands may have their own arguments that are not handled by the main tool.
Arguments between the tool name and the subcommand are handled by the tool and
arguments that follow the subcommand are handled by the subcommand. E.g. in
`fx -a build -b` the `-a` is an argument for the `fx` tool, while the `-b`
argument is handled by the `build` subcommand.

### Common Features

Command line tools are expected to support some common switches:

- `--help`
- `--quiet`
- `--verbose`
- `--version`

#### Interactive Help (--help)

A tool must accept a `--help` switch and provide usage information to the
command line in that case. The layout and syntax of the help text is described
in [CLI tool help requirements](/development/api/cli_help.md).

The tool must not do other work (i.e. have side effects) when displaying help.

Use a library that can parse the arguments as well as present help information
from the same source. Doing so keeps the two in sync. I.e. avoid writing command
line help as an independent paragraph of text.

Keep the interactive help reasonably concise. Plan for a skilled reader, i.e.
someone looking for a reminder on how to use the tool or a developer experienced
in reading interactive help. For the novice, provide a note referring them to
the Markdown documentation.

Provide an option to generate machine parsable output.

#### Verbosity (--quiet and --verbose)

The `--quiet` and `--verbose` switches decrease or increase informational output
to the user. Their implementation is optional, but all tools will accept them as
arguments and must not use those terms for other purposes, e.g. don't use
`--quiet` to turn off the audio output (use `--silence` or `--volume 0` or some
other synonym).

#### Interactive Version (--version)

A tool must accept a `--version` switch and provide an indication of the code
used to build the tool in that case. The layout and syntax is not specified, but
the version will include a version number of some kind.

The tool must not do other work (have side effects) when reporting its version.

### Logging

Logging is distinct from normal output. The audience for logging is normally the
tool developer or a power user trying to debug an issue. Logging may go to
stdout in special cases, such as when `--verbose` output is requested.

Logging from multiple threads will not interlace words within a line, i.e. the
minimum unit of output is a full text line. Each line will be prefixed with an
indication of the severity of the line. The severity will be one of: detail,
info, warning, error, fatal.

## Metrics

Every tool must file a Privacy Design Document (PDD) in order to collect usage
metrics.

Metrics are important to drive quality and business decisions. Questions we want
to answer with metrics include:

- Which OS are our users using? - so we know how to prioritize work for various
  platforms
- Which tools are they using? - so we know how to prioritize investments, and to
  learn which workflows are currently being used so we can prioritize
  investments or identify weak spots
- How often do they use a tool? - so we know how to prioritize investments, and
  to learn which workflows are currently being used so we can prioritize
  investments or identify weak spots
- Do our tools crash in the wild? How often? - so we know how to prioritize
  maintenance of tools
- How do they use a tool? - assuming that a tool can do one or more things, we'd
  like to learn how to prioritize investments in particular workflows of a tool

The type and content of the metrics collected must be carefully chosen. We will
go through the Google-standard PDD review process to ensure we are compliant
with Google's practices and policies. Tools must get approval on which metrics
are collected before collection.

## Configuration and Environment

Tools often need to know something about the context they are running. Let's
look at how that context should be gathered or stored.

#### Reading Information

Tools should not attempt to gather or intuit settings or other state directly
from the environment. Information such as an attached target's IP address, the
out directory for build products, or a directory for writing temporary files
will be gathered from a platform independent source. Separating out the code that
performs platform-specific work will allow tools to remain portable between
disparate platforms.

Where practical, configuration information should be stored in a way familiar to
the user of the host machine (e.g. on Windows, use the registry). Tools should
gather information from SDK files or platform-specific tools that encapsulate
the work of reading from the Windows registry, Linux environment, or Mac
settings.

Tools will be unbiased towards any build system or environment as well.
Accessing a common file such as build input dependency file is okay.

#### Writing Information

Tools will not modify configuration or environment settings, except when the
tool is clearly for the purpose of modifying an expected portion of the
environment.

If modifying the environment outside of the tool's normal scope may help the
user, the tool may do so with the express permission of the user.


## Execution Success and Failure

Command line tools return an integer value in the range [0..127] when they exit.
A zero represents success (no error) and 1-127 are various forms of error. The
value 1 is used as a general error. Any values other than 0 and 1 that may be
returned must be documented for the user.

### Succeed with Grace

If there were no errors encountered, return a result code of zero.

Avoid producing unnecessary output on success. Don't print "succeeded" (unless
the user is asking for verbose output).

### If Something is Unclear, Stop

If the tool encounters an ambiguous situation or is in danger of corrupting
data, do not continue. E.g. if the path to the directory you're being asked to
delete comes back as just "`/`", there was likely an error trying to get that
configuration information, avoid 'soldiering on' and removing everything under
"`/`".

### Do Not Fail Silently

Tools must clearly indicate failure by returning a non-zero error code. If
appropriate (if it makes sense for the tool or if the user explicitly asked for
verbose output) print an error message explaining what went wrong.

### Provide Direction on Failure

When a tool execution fails, be clear about whether the error came from bad
inputs, missing dependencies, or bugs within the tool. Make error reports
comprehensible and actionable.

If the error came from bad inputs

1. If the user gave the tool bad data, give context about the error and guide
   the user toward fixing the input, for example, by printing the input file
   (and line number if that's appropriate for the input) where the input error occurred.
   - Prefer output that follows this format (for easy regex use):
     `file_name:line:column:description`. This is a common format used by many
     tools. Other formats are acceptable, but try to use something that is easy
     for both humans and tools to parse.
2. Provide a reference to further information. If documentation is
   available, provide a link to documentation about the tool in general or to
   documentation regarding the specific error. If the tool has the capacity to
   provide more details, describe that (like how `gn` can explain how to run the
   tool to get more help).

If the error came from missing dependencies

1. Be clear that the error is from missing dependencies. Don't leave the
   user trying to debug their input data if that is not the issue.
2. Provide instruction on how to satisfy the dependencies. This can be an
   example command to run (`apt-get install foo`) or a link to further
   instructions (`see: http:example.com/how-to-install-foo`).

If the error came from an unexpected state (i.e. a bug) in the tool

1. Apologize. Explain that the tool got into an unexpected state. Don't leave
   the user trying to guess whether their input data was bad or they were
   missing dependencies.
2. Suggest a mailing list or forum to get help. Help the user find out if the
   bug is fixed in the next tool version; or someone has found a workaround.
3. Invite the user to enter a bug report and make that as easy as possible.
   Provide a link that goes to the bug database with the tool and platform
   information prepopulated.


## Include Tests

Tools must include tests that guarantee its correct behavior. Include both unit
tests and integration tests with each tool. Tests will run in Fuchsia continuous
integration.

> **IDK**
>
> It's especially important that IDK tools imported from the Fuchsia build (pm,
> etc.) have tests that run in Fuchsia continuous integration because the IDK
> bot does not currently prevent breaking changes.

> **ffx**
> The `ffx` platform provides a framework for introducing tests that are
> run automatically in Fuchsia continuous integration. Contributors can
> see examples of plugin tests and end-to-end self-tests in the `ffx`
> [source](/src/developer/ffx).

## Documentation

The Markdown documentation is the right place to put more verbose usage examples
and explanations.

> **IDK**
>
> All tools included in the IDK and intended to be executed directly by an end
> user must have a corresponding Markdown documentation file.

## User vs. Programmatic Interaction

A tool may be run interactively by a human user or programmatically via a script
(or other tool).

While each tool will default to interactive or non-interactive mode if they can
glean which is sensible, they must also accept explicit instruction to run in a
given mode (e.g. allow the user to execute the programmatic interface even if
they are running in an interactive shell).

### Stdin

For tools that are not normally interactive, avoid requesting user input
e.g. readline or linenoise). Don't suddenly put up an unexpected prompt to
ask the user a question.

For interactive tools (e.g. `zxdb`) prompting the user for input is expected.

### Stdout

When sending output to the user on stdout use proper spelling, grammar, and
avoid unusual abbreviations. If an unusual abbreviation is used, be sure it has
an entry in the [glossary](/glossary/README.md).

Try to check for output to terminal, i.e. see if a user is there or whether the
receiver is a program.

#### ANSI Color

Use of color is allowed, with the following caveats:

- Suppressing color:
  - When possible, check whether the terminal supports color, and suppress color
    output if not.
  - Always allow the user to manually suppress color output, e.g. with a
    `--no-color` flag and/or by setting the `NO_COLOR` environment variable
    ([no-color.org](http://no-color.org)).
- When using color, be sure to use colors that are distinct for readers who may
  not be able to see a full range of color (e.g. color-blindness).
  - The best way to do this is to stick to the standard [8/16 colors](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797#8-16-colors). It's easy for users to remap these, unlike the [256 colors](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797#256-colors).
- Never rely solely on color to convey information. Only use color as an enhancement.
  Seeing the color must not be needed for correct interpretation of the output.

### Stderr

Use stderr for reporting invalid operation (diagnostic output) i.e. when the
tool is misbehaving. If the tool's purpose is to report issues (like a linter,
where the tool is not failing) output those results to stdout instead of stderr.

See Success and Failure for more information on reporting errors.

### Full-Screen

Avoid creating full-screen terminal applications. Use a GUI application for such
a tool.

### Non-interactive (Programmatic)

Include a programmatic interface where reasonable to allow for automation.

If there is an existing protocol for that domain, try to follow suit (or have a
good reason not to). Otherwise consider using manifest or JSON files for
machine input.

### IDE (Semi-Programmatic)

Allow for tools to be used by an Integrated Development Environment. This
generally involves accepting a manifest for input and generating a manifest.

### Interactive (User)

Interacting with the user while the tool is running is an uncommon case for many
tools. Some tools may run interactively as an option, e.g. `rm -i` will prompt
the user before each removal.

## State Files

State files encode information for data sharing between tools. PID file and lock
files are examples of state files.

Avoid using a PID file to contain the process ID of a running executable.

Avoid using a lock file to manage mutual exclusion of resource access (i.e. a
mutex).
