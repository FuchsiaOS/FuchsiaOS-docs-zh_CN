
# FIDL JSON intermediate representation

For all backends (except C), the FIDL compiler operates in two phases.
A first phase parses the FIDL file(s) and produces a JSON-based Intermediate
Representation (**IR**).
A second phase takes the IR as input, and produces the appropriate language-specific output.

The shape of the JSON IR is described in the [schema][json-schema]. All JSON IR files that
are produced during compilation using `fx build` are validated using this schema.

If you are interested in the JSON IR, you can generate it by [running
the FIDL compiler][fidl-cli].

<!-- xrefs -->
[json-schema]: /tools/fidl/fidlc/schema.json
[fidl-cli]: development/languages/fidl/guides/cli.md
