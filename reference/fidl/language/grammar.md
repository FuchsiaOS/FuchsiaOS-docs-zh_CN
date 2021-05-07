# FIDL grammar

## Modified BNF rules

This is the grammar for FIDL source files. The grammar is expressed in a
modified BNF format.

A nonterminal symbol matches a sequence of other symbols, delimited by
commas.

```
nonterminal = list , of , symbols ;
```

Some symbols are terminals, which are either in all caps or are in
double quotes.

```
another-nonterminal = THESE , ARE , TERMINALS , AND , SO , IS , "this" ;
```

Alternation is expressed with a pipe.

```
choice = this | that | the-other ;
```

An option (zero or one) is expressed with parentheses.

```
optional = ( maybe , these ) , but , definitely , these ;
```

Repetition (zero or more) is expressed with parentheses and a star.

```
zero-or-more = ( list-part )* ;
```

Repetition (one or more) is expressed with parentheses and a plus.

```
one-or-more = ( list-part )+ ;

```

## Tokens

Whitespace and comments are ignored during lexing, and thus not
present in the following grammar.

Comments can start with two slashes ('//') or three slashes ('///'),
or they can be embodied within a [`[Doc]` attribute][doc_reference].
The three-slash variant and the `[Doc]` attribute behave the same way,
propagating the comments to the generated target.

The [fidldoc] tool processes comments propagated into the JSON-based
Intermediate Representation (IR) to generate reference
documentation pages for FIDL files.

See the [FIDL style guide][comment_style_guide] for more details on
comments.

## Grammar {#grammar}

`file` is the starting symbol.

```
file = library-header , ( using-list ) , declaration-list ;

library-header = ( attribute-list ) , "library" , compound-identifier , ";" ;

using-list = ( using , ";" )* ;

using = "using" , compound-identifier , ( "as" , IDENTIFIER ) ;

declaration-list = ( declaration , ";" )* ;

compound-identifier = IDENTIFIER ( "." , IDENTIFIER )* ;

declaration = bits-declaration | const-declaration | enum-declaration | protocol-declaration
            | struct-declaration | table-declaration | union-declaration
            | type-alias-declaration | resource-declaration | service-declaration ;

declaration-modifiers = "flexible" | "strict" | "resource"; [NOTE 1]

const-declaration = ( attribute-list ) , "const" , type-constructor , IDENTIFIER , "=" , constant ;

enum-declaration = ( attribute-list ) , ( declaration-modifiers )* , "enum" , IDENTIFIER ,
                   ( ":" , type-constructor ) , "{" , ( bits-or-enum-member , ";" )+ ,
                   "}" ; [NOTE 2]

bits-declaration = ( attribute-list ) , ( declaration-modifiers )* , "bits" , IDENTIFIER ,
                   ( ":" , type-constructor ) , "{" , ( bits-or-enum-member , ";" )+ ,
                   "}" ; [NOTE 3]

bits-or-enum-member = ( attribute-list ) , IDENTIFIER , "=" , bits-or-enum-member-value ;

bits-or-enum-member-value = IDENTIFIER | literal ; [NOTE 4]

protocol-declaration = ( attribute-list ) , "protocol" , IDENTIFIER ,
                       "{" , ( protocol-member , ";" )*  , "}" ;

protocol-member = protocol-method | protocol-event | protocol-compose ;

protocol-method = ( attribute-list ) , IDENTIFIER , parameter-list,
                  ( "->" , parameter-list , ( "error" type-constructor ) ) ; [NOTE 5]

protocol-event = ( attribute-list ) , "->" , IDENTIFIER , parameter-list ;

parameter-list = "(" , ( parameter ( "," , parameter )+ ) , ")" ;

parameter = ( attribute-list ) , type-constructor , IDENTIFIER ;

protocol-compose = "compose" , compound-identifier ;

struct-declaration = ( attribute-list ) , "struct" , IDENTIFIER , "{" , ( member-field , ";" )* ,
                     "}" ;

union-declaration = ( attribute-list ) , ( declaration-modifiers )* , "union" , IDENTIFIER , "{" ,
                    ( ordinal-member-field , ";" )+ , "}" ;

table-declaration = ( attribute-list ) , ( declaration-modifiers )* , "table" , IDENTIFIER , "{" ,
                    ( ordinal-member-field , ";" )* , "}" ;

member-field = ( attribute-list ) , type-constructor , IDENTIFIER , ( "=" , constant ) ;

ordinal-member-field = ( attribute-list ) , ordinal , ":" , ordinal-member-field-body ; [NOTE 6]

ordinal-member-field-body = member-field | "reserved";

type-alias-declaration = ( attribute-list ) , "alias" , IDENTIFIER ,  "=" , type-constructor ;

resource-declaration = ( attribute-list ) , "resource_definition" , IDENTIFIER , ":",
                       "uint32" , "{" , resource-properties ,  "}" ;

resource-properties = "properties" , "{" , ( type-constructor , IDENTIFIER  , ";" )* , "}" , ";"

service-declaration = ( attribute-list ) , "service" , IDENTIFIER , "{" ,
                      ( service-member , ";" )* , "}" ;

service-member = ( attribute-list ) , type-constructor , IDENTIFIER ; [NOTE 7]

attribute-list = "[" , attributes , "]" ;

attributes = attribute | attribute , "," , attributes ;

attribute = IDENTIFIER , ( "=" , STRING-LITERAL ) ;

type-constructor = compound-identifier ( "<" type-constructor ">" ) , (  type-constraint ) , ( "?" )
                 | handle-type ;

handle-type = "handle" , ( "<" , handle-subtype , ">" ) , ( "?" ) ;

handle-subtype = "bti" | "channel" | "clock" | "debuglog" | "event" | "eventpair" | "exception"
               | "fifo" | "guest" | "interrupt" | "iommu" | "job" | "pager" | "pcidevice"
               | "pmt" | "port" | "process" | "profile" | "resource" | "socket" | "suspendtoken"
               | "thread" | "timer" | "vcpu" | "vmar" | "vmo" ;

type-constraint = ":" , constant ;

constant = compound-identifier | literal ;

ordinal = NUMERIC-LITERAL ;

literal = STRING-LITERAL | NUMERIC-LITERAL | "true" | "false" ;
```

----------

### NOTE 1
The grammar allows `( declaration-modifiers )*` on all declarations, but the
compiler limits this as follows:

* A modifier cannot occur twice on the same declaration.
* The `flexible` and `strict` modifiers cannot be used together.
* The `flexible` and `strict` modifiers can only be used on `bits`, `enum`, and `union`.
* The `resource` modifier can only be used on `struct`, `table`, and `union`.

### NOTE 2
The `enum-declaration` allows the more liberal `type-constructor` in the
grammar, but the compiler limits this to signed or unsigned integer types,
see [primitives].

### NOTE 3
The `bits-declaration` allows the more liberal `type-constructor` in the grammar, but the compiler
limits this to unsigned integer types, see [primitives].

### NOTE 4
The `bits-or-enum-member-value` allows the more liberal `literal` in the grammar, but the compiler limits this to:

* A `NUMERIC-LITERAL` in the context of an `enum`;
* A `NUMERIC-LITERAL`, which must be a power of two, in the context of a `bits`.

### NOTE 5
The `protocol-method` error stanza allows the more liberal `type-constructor`
in the grammar, but the compiler limits this to an `int32`, `uint32`, or
an enum thereof.

### NOTE 6
Attributes cannot be placed on a reserved member.

### NOTE 7
The `service-member` allows the more liberal `type-constructor` in the grammar, but the compiler
limits this to protocols.


<!-- xrefs -->
[primitives]: /docs/reference/fidl/language/language.md#primitives
[fidldoc]: /tools/fidl/fidldoc/
[doc_reference]: /docs/reference/fidl/language/attributes.md#doc
[comment_style_guide]: /docs/development/languages/fidl/guides/style.md#comments
