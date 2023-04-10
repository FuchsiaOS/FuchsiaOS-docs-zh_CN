Every component has a declaration that describes the component's attributes and
capabilities. For components that are distributed in packages, the declaration
is expressed using a **component manifest file** and loaded with the help of a
**component resolver**.

![Diagram showing how components are declared using a "component manifest." The
manifest is compiled by the developer tools and resolved onto the device at
runtime.](/docs/get-started/images/components/component-manifest.png){: width="836"}

You declare components using component manifest language (CML) files. At build
time, the Component Manifest Compiler (`cmc`) tool validates and compiles the
manifest source into a binary format (`.cm`) and stores it in the component's
package. At runtime, component resolvers load the binary manifest into a
[ComponentDecl](https://fuchsia.dev/reference/fidl/fuchsia.component.decl#Component)
FIDL structure for [Component Manager](/docs/glossary/README.md#Component-Manager).
