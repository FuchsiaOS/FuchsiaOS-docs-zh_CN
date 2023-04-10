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

compound-identifier = IDENTIFIER ( "." , IDENTIFIER )* ;

declaration-list = ( declaration , ";" )* ;

declaration = const-declaration | layout-declaration | protocol-declaration
            | type-alias-declaration | resource-declaration | service-declaration ;

const-declaration = ( attribute-list ) , "const" , IDENTIFIER , type-constructor , "=" , constant ;

layout-declaration = ( attribute-list ) , "type" , IDENTIFIER , "=" , inline-layout ; [NOTE 1]

inline-layout = ( attribute-list ) , ( declaration-modifiers )* , layout-kind , ( layout-subtype ) ,
                layout-body ;

declaration-modifiers = "flexible" | "strict" | "resource" ; [NOTE 2]

layout-subtype = ":" , type-constructor ; [NOTE 3]

layout-kind = "struct" | "bits" | "enum" | "union" | "table" ;

layout-body = value-layout | struct-layout | ordinal-layout ;

value-layout = "{" , ( value-layout-member , ";" )+ , "}" ;

value-layout-member = ( attribute-list ) , IDENTIFIER , "=" , constant ; [NOTE 4]

struct-layout =  "{" , ( struct-layout-member, ";" )* , "}" ;

struct-layout-member = ( attribute-list ) , member-field ;

ordinal-layout =  "{" , ( ordinal-layout-member , ";" )* , "}" ; [NOTE 5]

ordinal-layout-member = ( attribute-list ) , ordinal , ":" , ordinal-layout-member-body ; [NOTE 6]

ordinal-layout-member-body = member-field | "reserved" ;

protocol-declaration = ( attribute-list ) , "protocol" , IDENTIFIER ,
                       "{" , ( protocol-member , ";" )*  , "}" ;

protocol-member = protocol-method | protocol-event | protocol-compose ;

protocol-method = ( attribute-list ) , IDENTIFIER , parameter-list,
                  ( "->" , parameter-list , ( "error" type-constructor ) ) ; [NOTE 7]

protocol-event = ( attribute-list ) , "->" , IDENTIFIER , parameter-list ;

parameter-list = "(" , ( type-constructor ) , ")" ; [NOTE 8]

protocol-compose = "compose" , compound-identifier ;

type-alias-declaration = ( attribute-list ) , "alias" , IDENTIFIER ,  "=" , type-constructor ;

resource-declaration = ( attribute-list ) , "resource_definition" , IDENTIFIER , ":",
                       "uint32" , "{" , resource-properties ,  "}" ;

resource-properties = "properties" , "{" , ( member-field  , ";" )* , "}" , ";"

service-declaration = ( attribute-list ) , "service" , IDENTIFIER , "{" ,
                      ( service-member , ";" )* , "}" ;

service-member = ( attribute-list ) , member-field ; [NOTE 9]

member-field = IDENTIFIER , type-constructor ;

attribute-list = ( attribute )* ;

attribute = "@", IDENTIFIER , ( "(" , constant | attribute-args, ")" ) ;

attribute-args = attribute-arg | attribute-arg, "," attribute-args ;

attribute-arg = IDENTIFIER , "=" , constant ;

type-constructor = layout , ( "<" , layout-parameters , ">" ) , ( ":" type-constraints ) ;

layout = compound-identifier | inline-layout ;

layout-parameters = layout-parameter | layout-parameter , "," , layout-parameters ;

layout-parameter = type-constructor | constant ;

type-constraints = type-constraint | "<" type-constraint-list ">" ;

type-constraint-list = type-constraint | type-constraint , "," , type-constraint-list ;

type-constraint = constant ;

constant = compound-identifier | literal ;

ordinal = NUMERIC-LITERAL ;

literal = STRING-LITERAL | NUMERIC-LITERAL | "true" | "false" ;
```

### `STRING-LITERAL`

The grammar for `STRING-LITERAL` is as follows:

```
STRING-LITERAL  = "\"" ( unicode-value )* "\"" ;
unicode-value   = literal-char | escaped-basic | escaped-unicode ;
literal-char    = any unicode character except CR, LF, "\" or "\"" ;
escaped-basic   = "\" ( "\" | "\"" | "n" | "r" | "t"  ) ;
escaped-unicode = "\u{" ( hex-digit ){1,6} "}" ;
```

----------

### NOTE 1

Attributes for an anonymous layout introduction can be placed in one of two
locations:

* before the `type` keyword, or
* as part of the `inline-layout`.

Placing attributes in both locations for a single layout definition is not
allowed by the compiler.

### NOTE 2

The grammar allows `( declaration-modifiers )*` on all declarations, but the
compiler limits this as follows:

* A modifier cannot occur twice on the same declaration.
* The `flexible` and `strict` modifiers cannot be used together.
* The `flexible` and `strict` modifiers can only be used when the `layout-kind`
  is `bits`, `enum`, or `union`.
* The `resource` modifier can only be when the `layout-kind` is `struct`,
  `table`, or `union`.

### NOTE 3

The grammar allows `( layout-subtype )` on all declarations, but the compiler limits
this to only be allowed when the `layout-kind` is `bits` or `enum`.

Further, `layout-subtype` allows the more liberal `type-constructor` in the
grammar, but the compiler limits this to signed or unsigned integer types
(see [primitives]) for enums and unsigned integer types for bits.

### NOTE 4

The `value-layout-member` allows the more liberal `constant` in the grammar, but
the compiler limits the values the `constant` may take:

* Any value that fits the specified `subtype`, in the context of an `enum`.
* Any value that fits the specified `subtype` and is a power of two, in the
  context of a `bits`.

### NOTE 5

The `ordinal-layout` grammar allows any number of members, but unions
specifically must at least one non-reserved member.

### NOTE 6

<!-- TODO(fxbug.dev/77958): remove when complete -->
Attributes cannot be placed on a reserved member.

Also, though ordinals can be any numeric literal, the compiler enforces that
the specified ordinals for any union or table cover a contiguous range starting
from 1.

### NOTE 7

The `protocol-method` error stanza allows the more liberal `type-constructor`
in the grammar, but the compiler limits this to an `int32`, `uint32`, or
an enum thereof.

### NOTE 8

The `parameter-list` allows the more liberal `type-constructor` in the grammar,
but the compiler only supports layouts that are structs, tables, or unions.

### NOTE 9

The `service-member` allows the more liberal `type-constructor` in the grammar,
but the compiler limits this to protocols.

<!-- xrefs -->
[primitives]: /docs/reference/fidl/language/language.md#primitives
[fidldoc]: /tools/fidl/fidldoc/
[doc_reference]: /docs/reference/fidl/language/attributes.md#doc
[comment_style_guide]: /docs/development/languages/fidl/guides/style.md#comments
