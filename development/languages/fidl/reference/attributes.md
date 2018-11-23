
# FIDL Attributes

The following FIDL attributes are discussed:

* [Discoverable]
* [Doc]
* [Internal]
* [Layout]

An attribute preceeds a FIDL element, for example:

```fidl
[Layout = "Simple"]
struct MyStruct { ...
```

It's used to either modify the characteristics of the element, or provide documentation.

> Note that the attribute applies *only* to the *next* element, not all subsequent ones.
> Elements after the current one revert to having no attributes.

## Discoverable

Causes the service's name to be made available for lookup.
A service with a `[Discoverable]` attribute can be found at run-time.
That is to say, you can "request" this service, and zircon will locate it and provide access to it.

## Doc

In FIDL, comments can start with two ("`//`") or three slashes ("`///`"), or they can be
embodied within a `[Doc]` attribute.
The two-slash variant does not propagate the comments to the generated target, whereas
both the three-slash and `[Doc]` variants do.

That is:

```fidl
/// Foo
struct MyFooStruct { ...
```

and

```fidl
[Doc = "Foo"]
struct MyFooStruct { ...
```

have the same effect &mdash; the text of the comment is
emitted into the generated code, in a manner compatible with the syntax of the target language.

## Internal

This marks internal libraries, such as library `zx`.
It should be used only by Fuchsia developers.

## Layout

This attribute currently has one valid value, `Simple`, and is meaningful only on interfaces.

It's used to indicate that all arguments and returns must contain objects that are of a fixed size.
The arguments and returns themselves, however, can be dynamically sized string or vector of primitives.

To clarify with an example, the following is valid:

```fidl
[Layout = "Simple"]
interface MyInterface {
    1: DynamicCountOfFixedArguments(vector<uint8>:1024 inputs);
};
```

Here, the argument is a dynamically sized `vector` of unsigned 8-bit integers called `inputs`, with a maximum bound of 1024 elements.

The benefit of `[Layout = "Simple"]` is that the data can be directly mapped without having to be copied and assembled.

