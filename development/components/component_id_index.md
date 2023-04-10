# Component ID index

## Terminology

[Moniker](/docs/glossary?style=box#moniker)
[Component instance identifier](/docs/glossary?style=box#component-instance-identifier)

## Scope

This document describes how to define an index that maps instance ids to
monikers. This mapping matters for components that use isolated storage
capabilities.

## Overview

The goal of an index of component instance IDs is to assign stable identifiers
to component instances. This is done by mapping an instance ID to a moniker.
When a component instance is assigned an ID, its persistent resources are
identified on disk using this instance ID. This allows the component's URL or
realm to be changed while its resources still remain attributed to it, so long
as this index is also updated.

When the component runtime discovers an instance ID -> moniker mapping, it
automatically moves the component instance's existing storage directory to be
keyed under its instance ID.

Only components that use storage capabilities must to be included in the
index. Test components should not be included in the index.

## Define a new index

An index file is a JSON5 formatted file, mapping a component's instance ID to
its moniker. There may be multiple index files in a build tree, but they will
be merged together into a single index file, and this merged file will be made
available to the component runtime. This merged index file is immutable, and
can only be updated through another system update.

The schema for an index file is described in the following example:

```json5
// Index files are written in JSON5, so you may use comments!
{
  // A list of entries, where each entry maps an instance ID to a moniker.
  instances: [
    // An entry, mapping an instance ID to a moniker.
    {
      // Instance IDs are randomly generated, 256-bits of base-16 encoded
      // strings (in lower case). To generate a new instance ID, omit this
      // field and run the build; the build will fail and suggest a new
      // instance ID which you can copy-paste here.
      instance_id: "2bd6cc2bd10243354b873a4ddb8a188b1d29171e26eebac06567bcdc36614af6",
      // The `instance_id` above is associated to the following moniker:
      moniker: "/core/account/credential_manager",
    },

    // More than one entry can be included. However, all entries must be distinct:
    // * Two entries cannot reference the same `instance_id`
    // * Two entries cannot reference the same `moniker`
    {
      instance_id: "7db7e88479772e241229682b47f1794e12ac5d692f8d67421acd9d7ff318a975",
      moniker: "/core/account/password_authenticator",
    }
  ]
}
```

To supply an index file to the build, use the
[component_id_index()](/build/component/component_id_index.gni) GN template:

```gn
component_id_index("my_component_id_index") {
  source = "my_component_id_index.json5"
}
```

## Add a component to the index {#add-to-index}

### Locate the appropriate index file

In order to add a component to the index, you must insert an entry into the
appropriate index file. Currently, `fuchsia.git`'s components are listed in the
[core_component_id_index.json5](/src/sys/core/core_component_id_index.json5)
index file.

{% dynamic if user.is_googler %}

Note: Outside of fuchsia.git, you can usually find a *component_id_index.json5
file in the `bundles/config` directory of a specific `//vendor` repository hosting
a product's configuration.

{% dynamic endif %}

### Add an entry to the index

The first step is to determine the component instance's moniker. You can find the
moniker of a component on a particular product's eng build by running `ffx component show`.

Then, append an entry to the `instances` list with the component's moniker.
Omit the `instance_id` field to have the build fail and suggest a new one you
can use.

#### Example

In this example, the component
`fuchsia-pkg://example.com/my_other_package#meta/my_other_component.cm` is added to the index.

To determine the component instance's moniker, you can run
`ffx component show fuchsia-pkg://example.com/my_other_package#meta/my_other_component.cm`:

```shell
$ ffx component show fuchsia-pkg://example.com/my_other_package#meta/my_other_component.cm
               Moniker:  /core/my_other_component
                   URL:  fuchsia-pkg://example.com/my_other_package#meta/my_other_component.cm
                      ...
```

The above output shows us that the moniker of this instance is `/core/my_other_component`

Add `/core/my_other_component` to the index by appending this entry to
[core_component_id_index.json5](/src/sys/core/core_component_id_index.json5)'s
`instances` list:

```json5
  {
    moniker: "/core/my_other_component"
  }
```

Now run the build.  The build should fail, suggesting a new instance ID:

```bash
$ fx build
.
.
Error: Could not merge index file ../../src/sys/core/core_component_id_index.json5

Caused by:
    Some entries are missing `instance_id` fields. Here are some generated IDs for you:
[
  {
    instance_id: "47c3bf08f3e560c4dee659c28fa8d863dbdc0b1dbb74065e6cb1f38441ac759c",
    moniker: "/core/my_other_component"
  }
]
```

Update the entry you've added by copying the suggested `instance_id` field. The
build should now pass.

## Include a Component ID Index in a system assembly {#system-assembly}

_The target audience for this section are product owners who are setting up a
system assembly_

This section describes how to include the component ID index in a system
assembly.

A system assembly should include a component ID index if it contains components
which use isolated storage. Any product that builds on top of the `core`
product already includes a component ID index in its assembly, so the following
instructions may not be necessary.

All `component_id_index()`s in a system build are merged together using the
`component_id_index_config()` template. This template is currently used in
`assembled_system.gni`, and assembly will fail if you define your own alongside
the one from `assembled_system.gni`.

### Steps

1. Define any `component_id_index()`s you want included in the system.
1. Add these targets as dependencies of `base_packages` in your `assembled_system()`
target.

