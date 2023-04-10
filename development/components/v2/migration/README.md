# Migrating legacy components

This guide provides instructions for migrating Fuchsia components from
Components v1 to [Components v2][glossary.components-v2].

For more details on the components migration effort, see
[State of the Components v2 Migration][components-migration-status].

{% dynamic if user.is_googler %}

Note: For important additional considerations and resources you should review
[go/tq-cf-v2-migration-resources](http://go/tq-cf-v2-migration-resources).

{% dynamic endif %}

## Before you begin {#background}

Before you begin migrating, consider familiarizing yourself with the
following topics:

-   [Introduction to the Fuchsia Component Framework][components-intro]:
    Components v2 comprises a set of concepts and APIs that are distinct from
    Components v1 or traditional OS program models.
-   [Introduction to the Test Runner Framework][trf-intro]: Test Runner
    Framework is built on the Component Framework. You need to be familiar with
    these concepts before you migrate tests.

## Get started {#get-started}

Begin your migration by following the guidance associated with your component's
role:

-   [Migrate system components](components.md):
    Components that provide services to other components in the system.
    Typically, in Components v1 the mapping of service to component is
    registered in a [sysmgr configuration file][sysmgr-config].
-   [Migrate test components](tests.md):
    Components related to integration testing such as test components,
    mock components, and test harnesses.

Next, explore the following sections for additional migration guidance on
specific features your components may support:

-   [Component sandbox features](features.md)
-   [Diagnostics capabilities](diagnostics.md)
-   [Other common situations](common.md)

[cf-dev-list]: https://groups.google.com/a/fuchsia.dev/g/component-framework-dev
[components-intro]: /concepts/components/v2/introduction.md
[components-migration-status]: /contribute/open_projects/components/migration.md
[glossary.components-v2]: /glossary/README.md#components-v2
[sysmgr-config]: /development/components/v2/migration/sysmgr.md
[trf-intro]: /development/testing/components/test_runner_framework.md
