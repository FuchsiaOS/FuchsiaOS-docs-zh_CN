# fidlc

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

usage: fidlc [--c-header HEADER_PATH]
             [--c-client CLIENT_PATH]
             [--c-server SERVER_PATH]
             [--tables TABLES_PATH]
             [--json JSON_PATH]
             [--convert-syntax CONVERTED_SYNTAX_PATH]
             [--name LIBRARY_NAME]
             [--experimental FLAG_NAME]
             [--werror]
             [--format=[text|json]]
             [--json-schema]
             [--files [FIDL_FILE...]...]
             [--help]
 * `--c-header HEADER_PATH`. If present, this flag instructs `fidlc` to output
   a C header at the given path.
 * `--c-client CLIENT_PATH`. If present, this flag instructs `fidlc` to output
   the simple C client implementation at the given path.
 * `--c-server SERVER_PATH`. If present, this flag instructs `fidlc` to output
   the simple C server implementation at the given path.
 * `--tables TABLES_PATH`. If present, this flag instructs `fidlc` to output
   coding tables at the given path. The coding tables are required to encode and
   decode messages from the C and C++ bindings.
 * `--json JSON_PATH`. If present, this flag instructs `fidlc` to output the
   library's intermediate representation at the given path. The intermediate
   representation is JSON that conforms to the schema available via --json-schema.
   The intermediate representation is used as input to the various backends.
 * `--convert-syntax CONVERTED_SYNTAX_PATH`. If present, this flag instructs `fidlc`
   to output the last file listed with updated syntax at the given path. The input
   file must be written in the old syntax for this to succeed.
 * `--name LIBRARY_NAME`. If present, this flag instructs `fidlc` to validate
   that the library being compiled has the given name. This flag is useful to
   cross-check between the library's declaration in a build system and the
   actual contents of the library.
 * `--experimental FLAG_NAME`. If present, this flag enables an experimental
    feature of fidlc.
 * `--files [FIDL_FILE...]...`. Each `--file [FIDL_FILE...]` chunk of arguments
   describes a library, all of which must share the same top-level library name
   declaration. Libraries must be presented in dependency order, with later
   libraries able to use declarations from preceding libraries but not vice versa.
   Output is only generated for the final library, not for each of its dependencies.
 * `--json-schema`. If present, this flag instructs `fidlc` to output the
   JSON schema of the intermediate representation.
 * `--format=[text|json]`. If present, this flag sets the output mode of `fidlc`.
    This specifies whether to output errors and warnings, if compilation fails, in
    plain text (the default), or as JSON.
 * `--werror`. Treats warnings as errors.
 * `--help`. Prints this help, and exit immediately.
All of the arguments can also be provided via a response file, denoted as
`@responsefile`. The contents of the file at `responsefile` will be interpreted
as a whitespace-delimited list of arguments. Response files cannot be nested.
See <https://fuchsia.dev/fuchsia-src/development/languages/fidl/reference/compiler>
for more information.
```

