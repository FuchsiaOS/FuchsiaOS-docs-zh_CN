A *generated name* is the FIDL compiler assigned name for an anonymous type.
While anonymous types are not nameable in FIDL files themselves, there must be
some name that refers to them in the generated bindings output, so end users may
create instance of the type in the binding language.

Because the FIDL compiler name generation algorithm uses the local context
(member name, method name, and so on) to name the type, name collisions are
possible. To resolve such collisions, place an `@generated_name` attribute
directly before the type declaration, instructing the compiler which name it
should use instead.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_support_trees_tutorial.md>>
