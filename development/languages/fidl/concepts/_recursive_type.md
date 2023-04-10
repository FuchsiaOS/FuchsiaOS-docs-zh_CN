*Recursive types* are types that transitively refer to themselves in their own
definition. This can happen when a type makes a direct reference to itself, or
when it refers to some chain of types that transitively include it. For
instance, in the classic
[tree](https://en.wikipedia.org/wiki/Tree_(data_structure)) data structure, each
node may either contain only data (a "leaf"), or data and references to more
child nodes (a "branch"). In the latter case, the node recursively contains a
nested tree definition, repeating to as great of a depth as necessary.

Warning: Support for recursive types is currently a [work in
progress](https://fxbug.dev/35218), with only some forms of cycle breakage being
recognized today.

FIDL supports recursive types, as long as at least one chain in the includes
cycle (in other words, the chain of type definitions that leads back to the
original type) is optional. If no type in the chain were optional, the type
would be unencodable, as each instance of the type would require at least one
more inside of it, ad infinitum.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_support_trees_tutorial.md>>
