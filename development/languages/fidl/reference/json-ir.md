
# FIDL JSON Internal Representation

For all backends (except C), the FIDL compiler operates in two phases.
A first phase parses the FIDL file(s) and produces a JSON-based Intermediate
Representation (**IR**).
A second phase takes the IR as input, and produces the appropriate language-specific output.

This section documents the JSON IR.

> If you're curious about the JSON IR, you can generate it by running
> the FIDL compiler with the `json` output directive:
>
> `fidlc --json outputfile.json --files inputfile.fidl`

## A simple example

To get started, we can see how a simple example looks.
We'll use the `echo2.fidl` ["Hello World Echo Interface"](../tutorial/README.md)
example from the tutorial:

```fidl
library echo2;

[Discoverable]
interface Echo {
    1: EchoString(string? value) -> (string? response);
};
```

The tutorial goes through this line-by-line, but the summary is that we create a
discoverable interface called `Echo` with a method called `EchoString`.
The `EchoString` method takes an optional string called `value` and returns
an optional string called `response`.

Regardless of the FIDL input, the FIDL compiler generates a JSON data set with
the following overall shape:

```json
{
  "version": "0.0.1",
  "name": "libraryname",
  "library_dependencies": [],
  "const_declarations": [],
  "enum_declarations": [],
  "interface_declarations": [],
  "struct_declarations": [],
  "table_declarations": [],
  "union_declarations": [],
  "declaration_order": [],
  "declarations": {}
}
```

The JSON members (name-value pairs) are as follows:

Name                    | Meaning
------------------------|-----------------------------------------------------------------------
version                 | A string indicating the version of the JSON IR schema
name                    | A string indicating the given `library` name
library_dependencies    | A list of dependencies on other libraries
const_declarations      | A list of consts
enum_declarations       | A list of enums
interface_declarations  | A list of interfaces provided
struct_declarations     | A list of structs
table_declarations      | A list of tables (reserved for future use)
union_declarations      | A list of unions
declaration_order       | A list of the object declarations, in order of declaration
declarations            | A list of declarations and their types

Not all members have content.

So, for our simple example, here's what the JSON IR looks like (line
numbers have been added for reference; they are not part of the generated code):

```json
[01]    {
[02]      "version": "0.0.1",
[03]      "name": "echo2",
[04]      "library_dependencies": [],
[05]      "const_declarations": [],
[06]      "enum_declarations": [],
[07]      "interface_declarations": [

(content discussed below)

[53]      ],
[54]      "struct_declarations": [],
[55]      "table_declarations": [],
[56]      "union_declarations": [],
[57]      "declaration_order": [
[58]        "echo2/Echo"
[59]      ],
[60]      "declarations": {
[61]        "echo2/Echo": "interface"
[62]      }
[63]    }
```

Lines `[01]` and `[63]` wrap the entire JSON object.

Line `[02]` is the version number of the JSON IR schema.

Line `[03]` is the name of the library, and is copied from the FIDL `library` directive.

Lines `[04]`, `[05]` and `[06]` are the library dependencies, constant declarations,
and enumeration declarations.
Our simple example doesn't have any, so they just have a zero-sized (empty) array
as their value ("`[]`").
Similarly, there are no structs (line `[54]`), tables (`[55]`) or unions (`[56]`).
The declaration order (`[57]`..`[59]`) isn't that interesting either,
because there's only the one declaration, and, finally, the
declarations member (`[60]`..`[62]`) just indicates the declared object (here, `echo2/Echo`)
and its type (it's an `interface`).

## The interface

Where things are interesting, though, is starting with line `[07]` &mdash; it's the interface
declaration for all interfaces in the file.

Our simple example has just one interface, the `Echo` interface, so there's just one
array element:

```json
[07]      "interface_declarations": [
[08]        {
[09]          "name": "echo2/Echo",
[10]          "maybe_attributes": [
[11]            {
[12]              "name": "Discoverable",
[13]              "value": ""
[14]            }
[15]          ],
[16]          "methods": [
[17]            {
[18]              "ordinal": 1,
[19]              "name": "EchoString",
[20]              "has_request": true,
[21]              "maybe_request": [
[22]                {
[23]                  "type": {
[24]                    "kind": "string",
[25]                    "nullable": true
[26]                  },
[27]                  "name": "value",
[28]                  "size": 16,
[29]                  "alignment": 8,
[30]                  "offset": 16
[31]                }
[32]              ],
[33]              "maybe_request_size": 32,
[34]              "maybe_request_alignment": 8,
[35]              "has_response": true,
[36]              "maybe_response": [
[37]                {
[38]                  "type": {
[39]                    "kind": "string",
[40]                    "nullable": true
[41]                  },
[42]                  "name": "response",
[43]                  "size": 16,
[44]                  "alignment": 8,
[45]                  "offset": 16
[46]                }
[47]              ],
[48]              "maybe_response_size": 32,
[49]              "maybe_response_alignment": 8
[50]            }
[51]          ]
[52]        }
[53]      ],
```

Each interface declaration array element contains:

*   Line `[09]`: the name of the object (`echo2/Echo` &mdash; this gets matched
    up with the `declarations` member contents starting on line `[60]`),
*   Lines `[10]`..`[15]`: an optional list of attributes (we had marked it as
    `Discoverable` &mdash; if we did not specify any attributes then we wouldn't
    see lines `[10]` through `[15]`), and
*   Lines `[16]`..`[51]`: an optional array of methods.

The methods array lists the defined methods in declaration order (giving details
about the ordinal number, the name of the method, whether it has a request
component and a response component, and indicates the sizes and alignments of
those componenets).

The JSON output has two `bool`s, `has_request` and `has_response`,
that indicate if the interface defines a request and a response, respectively.

Since the string parameters within the request and response are both optional,
the parameter description specifies `"nullable": true` (line `[25]` and `[40]`).

### What about the sizes?

The `size` members might be confusing at first; it's important here to note
that the size refers to the size of the *container* and not the *contents*.

> You may wish to refer to the [on-wire](wire-format/index.md) format document when
> reading this part.

Lines `[36]` through `[47]`, for example, define the `response` string container.
It's 16 bytes long, and consists of two 64-bit values:

*   a size field, indicating the number of bytes in the string (we don't rely
    on NUL termination), and
*   a data field, which indicates presence or pointer, depending on context.

For the data field, two interpretations are possible.
In the "wire format" version (that is, as the data is encoded for transmission),
the data field has one of two values: zero indicates the string is null,
and `UINTPTR_MAX` indicates that the data is present.
(See the [Wire Format](wire-format/index.md) chapter for details).

However, when this field has been read into memory and is decoded for consumption,
it contains a 0 (if the string is null), otherwise it's a pointer
to where the string content is stored.

The other fields, like `alignment` and `offset`, also relate to the
[on-wire](wire-format/index.md) data marshalling.

