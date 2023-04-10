### Accessing this command {#access}

<!-- TODO(kayce@): Move this section to the hardware testing guide
     once it's done and just link to the hardware testing guide section
     instead. -->

This command is only available on certain diagnostic and testing
Fuchsia builds.

This is a device-side command, not a host-side command like `fx` or `ffx`.
In other words before calling this command you must first access the
shell of the Fuchsia device.

To interactively access the Fuchsia device's shell:

```none
ffx component explore <component>
```

Replace `<component>` with the name of the component that has access to
the command documented on this page.

To call a single command and return the output to the host:

```none
ffx component explore <component> -c "<command>"
```

Replace `<command>` with one of the commands documented on this page.

[shell]: /reference/tools/fx/cmd/shell
