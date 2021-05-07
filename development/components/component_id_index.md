# Define an index of components that use storage

Note: This guide uses the [components v1](/docs/glossary.md#components-v1)
architecture.

## Terminology

- [Component Moniker](/docs/glossary.md#moniker)
- [Component Instance ID](/docs/glossary.md#component-instance-id)

## Scope

This document describes how to define an index that maps instance ids to
monikers, for components that use isolated-persistent-storage.

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
index. The following class of components should not be included in the
index:

* Test components
* Components whose storage is not managed by appmgr.

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
      instance_id: "11601233aef81741f7251907d4d2a1a33aa6fec6b2e54abffc21bec29f95fec2",
      // The `instance_id` above is associated to the following moniker:
      appmgr_moniker: {
        // This the URL of the component.
        url: "fuchsia-pkg://example.com/my_package#meta/my_component.cmx",

        // This is the realm path where this component runs.
        realm_path: [
          "sys",     // This the parent realm of "session"
          "session"  // This is the realm the component runs under
        ]
      }
    },

    // More than one entry can be included. However, all entries must be distinct:
    // * Two entries cannot reference the same `instance_id`
    // * Two entries cannot reference the same `realm`
    {
      instance_id: "644a7f0f66f8994d894c5f78b5b879911fee6c185c6aadd29d52888812d20ac4",
      appmgr_moniker: {
        url: "fuchsia-pkg://example.com/my_other_package#meta/my_other_component.cmx",
        realm_path: [
          "sys"
        ]
      }
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
[core_component_id_index.json5](/src/sys/appmgr/config/core_component_id_index.json5)
index file.

{% dynamic if user.is_googler %}

Note: Outside of fuchsia.git, you can usually find a *component_id_index.json5
file in the `bundles/config` directory of a specific `//vendor` repository hosting
a product's configuration.

{% dynamic endif %}

### Add an entry to the index

The first step is to determine the component instance's moniker, which is its
URL and realm path. You can find the the realm path of a component on a
particular product's eng build by checking `ffx component list` and collecting
"(realm)" labels under appmgr leading up to the component.

Then, append an entry to the `instances` list with the component's moniker.
Omit the `instance_id` field to have the build fail and suggest a new one you
can use.

#### Example

In this example, component `fuchsia-pkg://example.com/my_other_package#meta/my_other_component.cmx`
is added to the index.

To determine the component instance's realm_path, you can look at the output of
`ffx component list`:

```shell
$ ffx component list
<root>
  .
  .
  core
    appmgr
      app (realm)
        sysmgr.cmx
        sys (realm)
          my_other_component.cmx
          .
          .
  .
  .
```

The above output tells us that my_other_component.cmx runs under the
`[app, sys]` realm path.

Add `fuchsia-pkg://example.com/my_other_package#meta/my_other_component.cmx` to
the index by appending this entry to [core_component_id_index.json5](/src/sys/appmgr/config/core_component_id_index.json5)'s
`instances` list:

```json5
  {
    appmgr_moniker: {
      // The component's URL
      url: "fuchsia-pkg://example.com/my_other_package#meta/my_other_component.cmx",
      // The realm the component is run under.
      realm_path: [
        "app",
        "sys"
      ]
    }
  }
```

Now run the build.  The build should fail, suggesting a new instance ID:

```bash
$ fx build
.
.
Error: Could not merge index file ../../src/sys/appmgr/config/core_component_id_index.json5

Caused by:
    Some entries are missing `instance_id` fields. Here are some generated IDs for you:
[
  {
    instance_id: "47c3bf08f3e560c4dee659c28fa8d863dbdc0b1dbb74065e6cb1f38441ac759c",
    appmgr_moniker: {
      url: "fuchsia-pkg://example.com/my_other_package#meta/my_other_component.cmx",
      realm_path: [
        "app",
        "sys"
      ]
    }
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

### `component_id_index_config()`
All component_id_index()s in a system build are merged together using the
`component_id_index_config()` template.

`component_id_index_config()` produces a `resource()` target containing a
a FIDL-wireformat encoded index, along with a `config_data(for_pkg=appmgr)`
sub-target with a "-config-data" suffix containing a JSON-encoded index.

The `resource()` copy of the index is used by `component_manager`, while the
`config_data()` copy is used by `appmgr`. Although they use different formats,
they carry the same information.

To include a `component_id_index_config()` target in a system assembly:

**a)** Define it with a dependency on any `component_id_index()` targets
you want included in the system. For example, `//build/images:universe_packages`
is a good dependency candidate because it transitively includes all
`component_id_index()` specified in the build.

**b)** Add both the `component_id_index_config()` target and the `-config-data`
suffixed subtarget to the system assembly. Currently, a good method is to
include the target in the bootfs_labels, and make the `-config-data` sub-target
a dependency to your system assembly's `config_package()`.
