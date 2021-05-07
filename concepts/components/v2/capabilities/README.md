# Capabilities

This directory contains documentation about the various capability types in the
component framework.

Capabilities can be created, routed, and used in a [component
manifest](../component_manifests.md) to control which parts of Fuchsia have
ability to connect to and access which resources.

- [Directory capabilities](directory.md): connect to directories provided by
  other components.
- [Event capabilities](event.md): receive lifecycle events about components at
  a certain scope.
- [Protocol capabilities](protocol.md): connect to FIDL protocols provided by
  other components or the framework itself.
- [Resolver capabilities](resolvers.md): which resolvers are available in an
  [environment](../environments.md) determines which URL schemes can be used
  when declaring child components.
- [Runner capabilities](runners.md): determines which runner is responsible
  for instantiating the component and assisting with its lifecycle.
- [Service capabilities](service.md): connect to FIDL services (groups of
  protocols) provided by other components or the framework itself.
- [Storage capabilities](storage.md): special-cased directories with different
  semantics.
- [Life of a protocol open](life_of_a_protocol_open.md): How
  components connect to protocols in their namespaces.
