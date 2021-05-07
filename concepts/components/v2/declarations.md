# Component declarations (Components v2)

<<../_v2_banner.md>>

A component declaration describes what a component can do, the capabilities
it uses and exposes, its children, and other information needed to run the
component.

Every component has a declaration. For components that are distributed in
[packages][glossary-package], the declaration typically takes the form of
a [component manifest file][doc-component-manifests] that is located using a
[component URL][doc-component-urls].

Components can also be distributed in other forms such as web applications
with the help of a [resolver][doc-resolvers] to retrieve the component
declaration and a [runner][doc-runners] to run the component.

For example, the declaration for a calculator component might specify the
following information:

- The [location of the calculator program within its package][doc-component-manifests-program].
- The name of the [runner][doc-runners] used to run the program.
- A [request for persistent storage][doc-storage-capability] to save the
  contents of the calculator's accumulator across restarts.
- A request to [use capabilities][doc-component-manifests-use] to present
  a user interface.
- A request to [expose capabilities][doc-component-manifests-expose] to
  allow other components to access the calculator's accumulator register
  using inter-process communication.

[doc-component-urls]: /docs/concepts/components/component_urls.md
[doc-component-manifests]: /docs/concepts/components/v2/component_manifests.md
[doc-component-manifests-program]: /docs/concepts/components/v2/component_manifests.md#program
[doc-component-manifests-use]: /docs/concepts/components/v2/component_manifests.md#use
[doc-component-manifests-expose]: /docs/concepts/components/v2/component_manifests.md#expose
[doc-resolvers]: /docs/concepts/components/v2/capabilities/resolvers.md
[doc-runners]: /docs/concepts/components/v2/capabilities/runners.md
[doc-storage-capability]: /docs/concepts/components/v2/capabilities/storage.md
[glossary-package]: /docs/glossary.md#package
