# Grammar

## Modified BNF rules

This is the grammar for FIDL source files. The grammar is expressed in
a modified BNF format.

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
present in the following grammar. Comments are C++-style `//` until
the end of the line.

TODO(US-238): Eventually comments will be read as part of a
documentation generation system.

## The grammar

`file` is the starting symbol.

```
file = library-header , ( using-list ) , declaration-list ;

library-header = ( attribute-list ) , "library" , compound-identifier , ";" ;

using-list = ( using | using-declaration )* ;

using-declaration = "using" , IDENTIFIER ,  "=" , primitive-type , ";" ;

declaration-list = ( declaration , ";" )* ;

compound-identifier = IDENTIFIER ( "." , IDENTIFIER )* ;

using = "using" , compound-identifier , ( "as" , IDENTIFIER ) , ";" ;

declaration = const-declaration | enum-declaration | interface-declaration |
              struct-declaration | union-declaration ;

const-declaration = ( attribute-list ) , "const" , type , IDENTIFIER , "=" , constant ;

enum-declaration = ( attribute-list ) , "enum" , IDENTIFIER , ( ":" , integer-type ) ,
                   "{" , ( enum-member , ";" )+ , "}" ;

enum-member = IDENTIFIER , ( "=" , enum-member-value ) ;

enum-member-value = IDENTIFIER | NUMERIC-LITERAL ;

interface-declaration = ( attribute-list ) , "interface" , IDENTIFIER ,
                        ( ":" , super-interface-list ) , "{" , ( interface-method , ";" )*  , "}" ;

super-interface-list = compound-identifier
                     | compound-identifier , "," , super-interface-list

interface-method = ordinal , ":" , interface-parameters

interface-parameters = IDENTIFIER , parameter-list , ( "->" , parameter-list )
                     | "->" , IDENTIFIER , parameter-list

parameter-list = "(" , ( parameters ) , ")" ;

parameters = parameter | parameter , "," , parameters ;

parameter = type , IDENTIFIER ;

struct-declaration = ( attribute-list ) , "struct" , IDENTIFIER , "{" , ( struct-field , ";" )* , "}" ;

struct-field = type , IDENTIFIER , ( "=" , constant ) ;

union-declaration = ( attribute-list ) , "union" , IDENTIFIER , "{" , ( union-field , ";" )+ , "}" ;

union-field = type , IDENTIFIER ;

attribute-list = "[" , attributes, "]" ;

attributes = attribute | attribute , "," , attributes ;

attribute = IDENTIFIER , ( "=", STRING-LITERAL ) ;

type = identifier-type | array-type | vector-type | string-type | handle-type
                       | request-type | primitive-type ;

identifier-type = compound-identifier , ( "?" ) ;

array-type = "array" , "<" , type , ">" , ":" , constant ;

vector-type = "vector" , "<" , type , ">" , ( ":" , constant ) , ( "?" ) ;

string-type = "string" , ( ":" , constant ) , ( "?" ) ;

handle-type = "handle" , ( "<" , handle-subtype , ">" ) , ( "?" ) ;

handle-subtype = "process" | "thread" | "vmo" | "channel" | "event" | "port" |
                 "interrupt" | "debuglog" | "socket" | "resource" | "eventpair" |
                 "job" | "vmar" | "fifo" | "guest" | "timer" ;

request-type = "request" , "<" , compound-identifier , ">" , ( "?" ) ;

primitive-type = integer-type | "bool" | "float32" | "float64" ;

integer-type = "int8" | "int16" | "int32" | "int64" |
               "uint8" | "uint16" | "uint32" | "uint64" ;

constant = compound-identifier | literal ;

ordinal = NUMERIC-LITERAL ;

literal = STRING-LITERAL | NUMERIC-LITERAL | TRUE | FALSE ;
```
