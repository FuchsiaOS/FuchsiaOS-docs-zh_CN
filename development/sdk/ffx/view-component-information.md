# View component information

The [`ffx component`][ffx-component] commands can retrieve various types of
information about Fuchsia components on a device.

## Concepts

The [`ffx component list`][ffx-component-list] command prints
the list and hierarchy of all Fuchsia components (including v1 and v2 components)
available on your Fuchsia device. This command can help you answer the following
questions:

*   What are all the components on my device?
*   Which child components are under parent components on my device?

The [`ffx component show`][ffx-component-show] command prints the details of a
Fuchsia component on the device. This command can help you answer the following
questions (and more):

*   Which version of a component is on my device?
*   What's the instance ID of a component on my device?
*   What services are offered to and provided by a component on my device?

## Get the list of components {:#get-the-list-of-components}

To view the list of components available on your Fuchsia device,
run the following command:

```posix-terminal
ffx component list
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ tools/ffx component list
/
/bootstrap
/bootstrap/archivist
/bootstrap/base_resolver
/bootstrap/console
/bootstrap/console-launcher
/bootstrap/cr50_agent
/bootstrap/decompressor
/bootstrap/device_name_provider
/bootstrap/driver_index
/bootstrap/driver_manager
...
/core/wlancfg
/core/wlandevicemonitor
/core/wlanstack
/startup
```

## Get detailed information from a component {:#get-detailed-information-from-a-component}

To view the details of a specific component on your Fuchsia device,
run the following command:

Note: To see all available component monikers on the device,
run [`ffx component list`](#get-the-list-of-components).

```posix-terminal
ffx component show <COMPONENT>
```

Replace `COMPONENT` with a full or partial moniker of a Fuchsia component.
You may also replace `COMPONENT` with a full or partial component URL.
If there are multiple matches, the command prints the details of all the matching
components.

The example below prints the details of the `brightness_manager` component:

```none {:.devsite-disable-click-to-copy}
$ ffx component show brightness_manager
               Moniker: /core/brightness_manager
                   URL: fuchsia-pkg://fuchsia.com/brightness_manager#meta/brightness_manager.cm
                  Type: CML static component
       Component State: Resolved
           Instance ID: 8285f9d54645ea6baa7fd7b7905c7ae89a9daf8d5172be43b2587b166fd021af
 Incoming Capabilities: data
                        dev
                        fuchsia.logger.LogSink
                        pkg
  Exposed Capabilities: fuchsia.ui.brightness.Control
           Merkle root: a3b35183fbb3e49c450840f73043ee355d9e0d3673cc009c681f405816ea864e
       Execution State: Running
          Start reason: '/core/session-manager/session:session/workstation_session/login_shell/ermine_shell' requested capability 'fuchsia.ui.brightness.Control'
         Running since: 2022-04-11 20:54:05.139320126 UTC
                Job ID: 61155
            Process ID: 61188
 Outgoing Capabilities: fuchsia.ui.brightness.Control

```

<!-- Reference links -->

[ffx-component]: https://fuchsia.dev/reference/tools/sdk/ffx#component
[ffx-component-list]: https://fuchsia.dev/reference/tools/sdk/ffx#list_2
[ffx-component-show]: https://fuchsia.dev/reference/tools/sdk/ffx#show
