# Legacy components

This section contains documentation about components using the legacy component
framework ([Components v1][glossary.components-v1]) implemented by `appmgr`.
The Fuchsia platform team is currently [migrating legacy components][migration]
to the modern component framework ([Components v2][glossary.components-v2]).

Note: New component development should use the modern component framework.
For more details, see the [Components overview][components-overview].

## Architectural concepts

- [sysmgr](sysmgr.md): The component responsible for hosting the `sys` realm.

## Developing components

- [Component manifests](component_manifests.md): Declaring legacy components.

## Debugging and troubleshooting

- [Hub](hub.md): A portal for introspection of `appmgr` components.

## Testing

- [Test components][test-component]: Implementing component tests with the
  legacy component framework.

[components-overview]: concepts/components/v2/introduction.md
[glossary.components-v1]: glossary/README.md#components-v1
[glossary.components-v2]: glossary/README.md#components-v2
[migration]: contribute/open_projects/components/migration.md
[test-component]: concepts/testing/v1_test_component.md
