# Debugger (zxdb)

Zxdb is a console debugger for native code compiled with DWARF symbols (C, C++
and Rust). The frontend runs on the host computer and connects to the on-device
`debug_agent`. This document describes how to set up these processes.

## Running the agent

The `debug_agent` is run on the target device along with the port number that
it should listen to for incoming client connections. Typically this command
will be run from a console after [ssh-ing](ssh.md) in to the system:

```
run fuchsia-pkg://fuchsia.com/debug_agent#meta/debug_agent.cmx --port=2345
```

## Connecting the client

The `zxdb` client program is run on the host computer. It can be connected to
the `debug_agent` via the interactive `connect` debugger command or it can
automatically connect based on a command-line flag. Both IPv4 and IPv6
addresses are supported (see [device discovery](device_discovery.md) to find
the address). The port should match the port number passed to the agent.

```
zxdb -c "[f370::5051:ff:1e53:589a%qemu]:2345"
```

### Connecting via a script

Scripts may want to automatically launch the agent and client automatically.
The script should wait for the port to be open on the target system before
launching the client. Automatic retry is not yet implemented in the client.

To clean up the debug agent gracefully when the client exits, pass the
`--quit-agent-on-exit` command-line flag to the client.

## Specifying symbol paths

The debugger expects unstripped ELF files to be available on the local host
system. Symbols on the target are not used. The location where the local build
stores symbols must be passed to the `zxdb` client.

For more information on how to specify symbol paths, see [set the symbol
location](/docs/development/debugger/README.md#set-symbol-location)
