# ffx

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx [-c <config>] [-e <env>] [-t <target>] [-T <timeout>] [-v] [<command>] [<args>]
Fuchsia's developer tool
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -c, --config      override default configuration
  -e, --env         override default environment settings
  -t, --target      apply operations across single or multiple targets
  -T, --timeout     override default proxy timeout
  -v, --verbose     use verbose output
  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  component         Discover and manage components
  config            View and switch default and user configurations
  daemon            Interact with/control the ffx daemon
  doctor            Run common checks for the ffx tool and host environment
  vdl               Start and manage Fuchsia emulators
  overnet           Interact with the Overnet mesh
  package           Create and publish Fuchsia packages
  platform          Manage platform build prerequisites
  scrutiny          Audit the security of Fuchsia
  sdk               Modify or query the installed SDKs
  self-test         Execute the ffx self-test (e2e) suite
  target            Interact with a target device or emulator
  trace             Interact with the tracing subsystem
  version           Print out ffx tool and daemon versions
  debug             Start a debugging session.
```

## component

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx component <command> [<args>]
Discover and manage components
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  knock             Connect to a service on the target
  list              List all components
  run               Run a component on the target
  select            Lists components matching a selector
  test              Run test suite
```

### knock

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx component knock <selector>
Connect to a service on the target
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
Examples:
  To connect to a service:
      $ ffx component knock 'core/appmgr:out:fuchsia.hwinfo.Product'
Notes:
  Knock verifies the existence of a service exposed by a component by
  attempting to connect to it. The command expects a <selector> with the
  following format:
  `<component moniker>:(in|out|exposed)[:<service name>].`
  Note that wildcards can be used but must match exactly one service.
  The `component select` command can be used to explore the component
  topology to compose the correct selector for use in `component knock`.
Error codes:
  1 Failed to connect to service
```

### list

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx component list
List all components
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
Examples:
  To list all components in the topology:
      $ ffx component list
Notes:
  Lists all the components on the running target. The command outputs a
  tree of all v1 and v2 components on the system.
  If the command fails or times out, ensure RCS is running on the target.
  This can be verified by running `ffx target list` and seeing the status
  on the RCS column.
Error codes:
  1 The command has timed out
```

### run

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx component run <url> [<args...>]
Run a component on the target
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
Examples:
  To run the 'hello_world_rust' component:
      $ ffx component run \
      fuchsia-pkg://fuchsia.com/hello_world_rust#meta/hello_world_rust.cmx
  To run the Remote Control Service:
      $ ffx component run \
      fuchsia-pkg://fuchsia.com/remote-control#meta/remote-control-runner.cmx
Notes:
  Runs a specified v1 component on the target. The <url> must follow the
  format:
  `fuchsia-pkg://fuchsia.com/<package>#meta/<component>.cmx`.
```

### select

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx component select <selector>
Lists components matching a selector
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
Examples:
  To show services exposed by remote-control:
      $ ffx component select remote-control:expose:*'
  Or to show all services offered by v1 components:
      $ ffx component select core/appmgr:out:*
Notes:
  Component select allows for looking up various services exposed by the
  component. The command expects a <selector> with the following format:
  `<component moniker>:(in|out|exposed)[:<service name>]`
  Wildcards may be used anywhere in the selector.
Error codes:
  1 No matching component paths found
```

### test

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx component test <test_url> [-t <timeout>] [--test-filter <test-filter>] [--list] [--run-disabled] [--parallel <parallel>] [--count <count>]
Run test suite
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -t, --timeout     test timeout
  --test-filter     a glob pattern for matching tests
  --list            list tests in the suite
  --run-disabled    run tests that have been marked disabled/ignored
  --parallel        run tests in parallel
  --count           number of times to run the test [default = 1]
  --help            display usage information
Notes:
  Runs a test or suite implementing the `fuchsia.test.Suite` protocol.
  Note that if running multiple iterations of a test and an iteration times
  out, no further iterations will be executed.
```

## config

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx config <command> [<args>]
View and switch default and user configurations
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  env               list environment settings
  get               display config values
  set               set config settings
  remove            remove config for a given level
  add               add config value the end of an array
  analytics         enable or disable analytics
```

### add

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx config add <name> <value> [-l <level>] [-b <build-dir>]
add config value the end of an array
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -l, --level       config level. Possible values are "user", "build", "global".
                    Defaults to "user".
  -b, --build-dir   an optional build directory to associate the build config
                    provided - use used for "build" configs
  --help            display usage information
Notes:
  This will always add to the end of an array.  Adding to a subtree is not supported. If the current value is not an array, it will convert the value to an array.  If you want to insert a value in a different position, consider editing the configuration file directly.  Configuration file locations can be found by running `ffx config env get` command.
```

### analytics

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx config analytics <command> [<args>]
enable or disable analytics
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  enable            enable analytics
  disable           disable analytics
  show              show analytics
```

#### disable

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx config analytics disable
disable analytics
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

#### enable

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx config analytics enable
enable analytics
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

#### show

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx config analytics show
show analytics
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

### env

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx config env [<command>] [<args>]
list environment settings
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  set               set environment settings
  get               list environment for a given level
```

#### get

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx config env get [<level>]
list environment for a given level
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

#### set

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx config env set <file> [-l <level>] [-b <build-dir>]
set environment settings
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -l, --level       config level. Possible values are "user", "build", "global".
                    Defaults to "user".
  -b, --build-dir   an optional build directory to associate the build config
                    provided - use used for "build" configs
  --help            display usage information
```

### get

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx config get [<name>] [-p <process>] [-s <select>] [-b <build-dir>] [-o <output>]
display config values
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -p, --process     how to process results. Possible values are "raw", "sub",
                    and "sub_flat".  Defaults to "raw". Currently only supported
                    if a name is given.
  -s, --select      how to collect results. Possible values are "first" and
                    "all".  Defaults to "first".  If the value is "first", the
                    first value found in terms of priority is returned. If the
                    value is "all", all values across all configuration levels
                    are aggregrated and returned. Currently only supported if a
                    name is given.
  -b, --build-dir   an optional build directory to associate the build config
                    provided - use used for "build" configs
  -o, --output      [DEPRECATED] The output type will always be JSON.
  --help            display usage information
Error codes:
  2 No value found
```

### remove

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx config remove <name> [-l <level>] [-b <build-dir>]
remove config for a given level
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -l, --level       config level. Possible values are "user", "build", "global".
                    Defaults to "user".
  -b, --build-dir   an optional build directory to associate the build config
                    provided - use used for "build" configs
  --help            display usage information
Notes:
  This will remove the entire value for the given name.  If the value is a subtree or array, the entire subtree or array will be removed.  If you want to remove a specific value from an array, consider editing the configuration file directly.  Configuration file locations can be found by running `ffx config env get` command.
```

### set

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx config set <name> <value> [-l <level>] [-b <build-dir>]
set config settings
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -l, --level       config level. Possible values are "user", "build", "global".
                    Defaults to "user".
  -b, --build-dir   an optional build directory to associate the build config
                    provided - use used for "build" configs
  --help            display usage information
```

## daemon

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx daemon <command> [<args>]
Interact with/control the ffx daemon
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  crash             crash the daemon
  echo              run echo test against the daemon
  log               Dumps the daemon log
  start             run as daemon
  stop              stops a running daemon
```

### crash

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx daemon crash
crash the daemon
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

### echo

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx daemon echo [<text>]
run echo test against the daemon
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

### log

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx daemon log
Dumps the daemon log
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

### start

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx daemon start
run as daemon
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

### stop

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx daemon stop
stops a running daemon
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

## debug

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx debug [<socket_location>]
Start a debugging session.
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

## doctor

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx doctor [--retry-count <retry-count>] [--retry-delay <retry-delay>] [--restart-daemon] [--record] [--record-output <record-output>]
Run common checks for the ffx tool and host environment
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --retry-count     number of times to retry failed connection attempts.
  --retry-delay     timeout delay when attempting to connect to the daemon or
                    RCS
  --restart-daemon
                    if true, forces a daemon restart, even if the connection
                    appears to be working
  --record          if true, generates an output zip file that can be attached
                    to a monorail issue
  --record-output   sets the output directory for doctor records. Only valid
                    when --record is provided. Defaults to the current directory
  --help            display usage information
```

## overnet

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx overnet <command> [<args>]
Interact with the Overnet mesh
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  list-peers        List known peer nodes
  list-links        List links on a particular peer
  host-pipe         Use stdin/stdout as a link to another overnet instance
  full-map          Construct a detailed graphviz map of the Overnet mesh - experts only!
```

### full-map

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx overnet full-map --exclude-self <exclude-self>
Construct a detailed graphviz map of the Overnet mesh - experts only!
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --exclude-self    if set, exclude the onet tool from output
  --help            display usage information
```

### host-pipe

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx overnet host-pipe
Use stdin/stdout as a link to another overnet instance
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

### list-links

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx overnet list-links <nodes>
List links on a particular peer
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

### list-peers

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx overnet list-peers
List known peer nodes
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

## package

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx package <command> [<args>]
Create and publish Fuchsia packages
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  build             Builds a package.
Entries may be specified as:
-
                    <dst>=<src>: Place the file at path <src> into the package
                    at path <dst>.
- @<manifest-file>: Read each line of this
                    file as an entry. This is not recursive; you can't put
                    another @<manifest-file>
  export            export a package archive
  import            import a package archive
```

### build

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx package build [<entries...>] [--source-dir <source-dir>] [--hash-out <hash-out>] [--depfile <depfile>]
Builds a package.
Entries may be specified as:
- <dst>=<src>: Place the file at path <src> into the package at path <dst>.
- @<manifest-file>: Read each line of this file as an entry. This is not recursive; you can't put another @<manifest-file>
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --source-dir      base directory for the <src> part of entries; defaults to
                    the current directory
  --hash-out        write the package hash to this file instead of stdout
  --depfile         write a gcc-format depfile for use in build systems
  --help            display usage information
```

## platform

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx platform <command> [<args>]
Manage platform build prerequisites
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  preflight         Evaluate suitability for building and running Fuchsia
```

### preflight

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx platform preflight
Evaluate suitability for building and running Fuchsia
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

## scrutiny

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx scrutiny <command> [<args>]
Audit the security of Fuchsia
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  shell             Run the scrutiny shell
```

### shell

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx scrutiny shell [<command>]
Run the scrutiny shell
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

## sdk

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx sdk <command> [<args>]
Modify or query the installed SDKs
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  version           Retrieve the version of the current SDK
```

### version

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx sdk version
Retrieve the version of the current SDK
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

## self-test

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx self-test [--timeout <timeout>] [--case-timeout <case-timeout>] [--include-target <include-target>]
Execute the ffx self-test (e2e) suite
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --timeout         maximum runtime of entire test suite in seconds
  --case-timeout    maximum run time of a single test case in seconds
  --include-target  include target interaction tests
  --help            display usage information
```

## target

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target <command> [<args>]
Interact with a target device or emulator
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  add               Make the daemon aware of a specific target
  default           Manage the default target
  flash             Flash an image to a target device
  get-ssh-address   Get the target's ssh address
  list              List all targets
  log               
  off               Powers off a target
  reboot            Reboots a target
  remove            Make the daemon forget a specific target
  status            Display status information for the target
  update            Update base system software on target
Notes:
  The `target` subcommand contains various commands for target management
  and interaction.
  Typically, this is the entry workflow for users, allowing for target
  discovery and provisioning before moving on to `component` or `session`
  workflows once the system is up and running on the target.
  Most of the commands depend on the RCS (Remote Control Service) on the
  target.
```

### add

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target add <addr>
Make the daemon aware of a specific target
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
Examples:
  To add a remote target forwarded via ssh:
      $ ffx target add 127.0.0.1:8022
  Or to add a target using its IPV6:
      $ ffx target add fe80::32fd:38ff:fea8:a00a
Notes:
  Manually add a target based on its IP address. The command accepts IPV4
  or IPV6 addresses, including a port number: `<addr> = <ip addr:port>`.
  Typically, the daemon automatically discovers targets as they come online.
  However, manually adding a target allows for specifying a port number or
  address, often used for remote workflows.
```

### default

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target default <command> [<args>]
Manage the default target
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  get               Get the default configured target
  set               Set the default target
  unset             Clears the default configured target
Examples:
  For one-off overrides for the default use `--target` option:
      $ ffx --target <target name> <subcommand>
  Or use the `--config` option:
      $ ffx --config target.default=<target name> <subcommand>
Notes:
  Manages the default configured target for all operations. The default
  target is designated by a `*` next to the name. This is an alias for the
  `target.default` configuration key.
```

#### get

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target default get
Get the default configured target
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
Notes:
  Returns the default configured target from the 'User Configuration'.
  Returns an empty string if no default is configured.
```

#### set

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target default set <nodename> [-l <level>] [-b <build-dir>]
Set the default target
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -l, --level       config level, such as 'user', 'build', or 'global'
  -b, --build-dir   optional directory to associate the provided build config
  --help            display usage information
Examples:
  To set the default target:
     $ ffx target default set <target name>
  To set the 'target.default` key at the global configuration:
     $ ffx target default set -l global <target name>
  To specify a default target for a specific build directory:
     $ ffx target default set -l build -b ~/fuchsia/out <target name>
Notes:
  Sets the `target.default` configuration key. By default sets the key in
  the 'User Configuration'. Can be used in conjuction with `ffx target list`
  to list the names of the discovered targets.
  After setting the default target, `ffx target list` will mark the default
  with a `*` in the output list.
```

#### unset

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target default unset [-l <level>] [-b <build-dir>]
Clears the default configured target
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -l, --level       config level, such as 'user', 'build', or 'global'
  -b, --build-dir   optional directory to associate the provided build config
  --help            display usage information
Examples:
  To clear the default target:
      $ ffx target default unset
  To clear the `target.default` key from global configuration:
      $ ffx target default unset -l global
  To specify a specific build directory:
      $ ffx target default unset -l build -b ~/fuchsia/out
Notes:
  Clears the `target.default` configuration key. By default clears the
  'User Configuration'. Returns a warning if the key is already empty.
```

### flash

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target flash <manifest> [<product>] [--oem-stage <oem-stage>]
Flash an image to a target device
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --oem-stage       oem staged file - can be supplied multiple times
  --help            display usage information
Examples:
  To flash a specific image:
      $ ffx target flash ~/fuchsia/out/flash.json fuchsia
  To include SSH keys as well:
      $ ffx target flash
      --oem-stage add-staged-bootloader-file ssh.authorized_keys,
      ~/fuchsia/.ssh/authorized_keys
      ~/fuchsia/out/default/flash.json
       fuchsia
Notes:
  Flases an image to a target device using the fastboot protocol.
  Requires a specific <manifest> file and <product> name as an input.
  This is only applicable to a physical device and not an emulator target.
  The target device is typically connected via a micro-USB connection to
  the host system.
  The <manifest> format is a JSON file generated when building a fuchsia
  <product> and can be found in the build output directory.
  The `--oem-stage` option can be supplied multiple times for several OEM
  files. The format expects a single OEM command to execute after staging
  the given file.
  The format for the `--oem-stage` parameter is a comma separated pair:
  '<OEM_COMMAND>,<FILE_TO_STAGE>'
```

### get-ssh-address

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target get-ssh-address [-t <timeout>]
Get the target's ssh address
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -t, --timeout     the timeout in seconds [default = 1.0]
  --help            display usage information
Notes:
  Return the SSH address of the default target defined in the
  `target.default` key. By default this comes from the 'User Configuration'.
  The command takes a <timeout> value in seconds with a default of `1.0`
  and overrides the value in the `target.interaction.timeout` key.
Error codes:
  1 Timeout while getting ssh address
```

### list

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target list [<nodename>] [-f <format>]
List all targets
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -f, --format      determines the output format for the list operation
  --help            display usage information
Examples:
  To list targets in short form:
      $ ffx target list --format s
      fe80::4415:3606:fb52:e2bc%zx-f80ff974f283 pecan-guru-clerk-rhyme
  To list targets with only their addresses:
      $ ffx target list --format a
      fe80::4415:3606:fb52:e2bc%zx-f80ff974f283
Notes:
  List all targets that the daemon currently has in memory. This includes
  manually added targets. The daemon also proactively discovers targets as
  they come online. Use `ffx target list` to always get the latest list
  of targets.
  The default target is marked with a '*' next to the node name. The table
  has the following columns:
      NAME = The name of the target.
      TYPE = The product type of the target, currently always 'Unknown'.
      STATE = The high-level state of the target, currently always 'Unknown'.
      AGE = Shows the last time the daemon was able to discover the target.
      ADDRS/IP = The discovered and known addresses of the target.
      RCS = Indicates if the Remote Control Service is running on the target.
  The NAME column shows the target's advertised name. When the target is
  in early boot state such as fastboot, shows 'FastbootDevice' with the
  `product` and `serial` attributes instead.
  By default, the `list` command outputs in a tabular format. To override
  the format, pass `--format` and can take the following options: 'simple'
  , 'tabular|table|tab', 'addresses|addrs|addr', 'json|JSON' or in short form 's', 't',
   'a', 'j'.
```

### log

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target log <command> [<args>]
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  watch             Watches for and prints logs from a target. Optionally dumps
                    recent logs first.
  dump              Dumps all logs from a target.
```

#### dump

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target log dump
Dumps all logs from a target.
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

#### recent

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Unrecognized argument: recent
```

#### watch

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target log watch [--dump <dump>]
Watches for and prints logs from a target. Optionally dumps recent logs first.
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --dump            if true, dumps recent logs before printing new ones.
  --help            display usage information
```

### off

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target off
Powers off a target
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
Notes:
  Power off a target. Uses the 'fuchsia.hardware.power.statecontrol.Admin'
  FIDL API to send the power off command.
  The 'fuchsia.hardware.power.statecontrol.Admin' is exposed via the 'appmgr'
  component. To verify that the target exposes this service, `ffx component
  select` or `ffx component knock` can be used.
Error codes:
  1 Timeout while powering off target.
```

### reboot

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target reboot [-b] [-r]
Reboots a target
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -b, --bootloader  reboot to bootloader
  -r, --recovery    reboot to recovery
  --help            display usage information
Notes:
  Reboot a target. Uses the 'fuchsia.hardware.power.statecontrol.Admin'
  FIDL API to send the reboot command.
  By default, target boots fully. This behavior can be overrided by passing
  in either `--bootloader` or `--recovery` to boot into the bootloader or
  recovery, respectively.
  The 'fuchsia.hardware.power.statecontrol.Admin' is exposed via the 'appmgr'
  component. To verify that the target exposes this service, `ffx component
  select` or `ffx component knock` can be used.
Error codes:
  1 Timeout while powering off target.
```

### remove

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target remove <id>
Make the daemon forget a specific target
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
Examples:
  To remove a target by its target name:
      $ ffx target remove correct-horse-battery-staple
  Or to remove a target using its IP address:
      $ ffx target remove fe80::32fd:38ff:fea8:a00a
Notes:
  IP addresses are matched by their full string representation.
  for best results, copy the exact address from ffx target list.
```

### status

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target status [--desc] [--label] [--json] [--version]
Display status information for the target
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --desc            display descriptions of entries
  --label           display label of entries
  --json            formats output as json objects
  --version         display version
  --help            display usage information
Notes:
  Displays a detailed runtime status information about the target.
  The default output is intended for a human reader. This output can be
  decorated with machine readable labels (--label) and descriptions of
  each field (--desc).
  The 'label' fields in the machine readable output (--json) will remain
  stable across software updates and is not localized (compare to 'title'
  which may change or be localized). The 'value' field will be one of:
  'null', 'bool', 'string', or a list of strings.
Error codes:
  1 Timeout retrieving target status.
```

### update

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target update <command> [<args>]
Update base system software on target
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  channel           View and manage update channels
  check-now         Check and perform the system update operation
  force-install     Trigger the system updater manually
Notes:
  This command interfaces with system update services on the target.
```

#### channel

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target update channel <command> [<args>]
View and manage update channels
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  get-current       Return the currently configured update channel
  get-next          Return the next or target update channel
  set               Sets the update channel
  list              List the known update channels
Notes:
  Channel management commands and operations. Interfaces directly with
  the 'fuchsia.update.channelcontrol.ChannelControl' service on the target
  system.
```

##### get-current

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target update channel get-current
Return the currently configured update channel
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
Notes:
  For developer product configurations, this is by default 'devhost'.
Error codes:
  1 Timeout while getting update channel.
```

##### get-next

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target update channel get-next
Return the next or target update channel
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
Notes:
  Returns the next or target channel. This differs from `get` when the
  next successful update changes the configured update channel on the
  system.
Error codes:
  1 Timeout while getting update channel.
```

##### list

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target update channel list
List the known update channels
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
Notes:
  This lists all the known next or target update channels on the system.
  Returns an empty list if no other update channels are configured.
Error codes:
  1 Timeout while getting list of update channel.
```

##### set

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target update channel set <channel>
Sets the update channel
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
Examples:
  To list all the known update channels:
      $ ffx target update channel list
  Then, use a valid channel from the list:
      $ ffx target update channel set <channel>
Notes:
  Sets the next or target update channel on the device. When paired with
  `ffx target update check-now`, ensures the update is check against the
  next or target channel. When the update is successful, next or target
  channel becomes the current channel.
  Use `ffx target update channel list` to list known system update
  channels.
Error codes:
  1 Timeout while setting update channel.
```

#### check-now

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target update check-now [--service-initiated] [--monitor]
Check and perform the system update operation
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --service-initiated
                    the update check was initiated by a service, in the
                    background.
  --monitor         monitor for state update.
  --help            display usage information
Examples:
  To check for update and monitor progress:
      $ ffx target update check-now --monitor
Notes:
  Triggers an update check operation and performs the update if available.
  Interfaces using the 'fuchsia.update Manager' protocol with the system
  update service on the target.
  The command takes in an optional `--monitor` switch to watch the progress
  of the update. The output is displayed in `stdout`.
  The command also takes an optional `--service-initiated` switch to indicate
  a separate service has initiated a check for update.
```

#### force-install

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx target update force-install <update_pkg_url> [--reboot <reboot>]
Trigger the system updater manually
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --reboot          automatically trigger a reboot into the new system
  --help            display usage information
Examples:
  With a known update package URL, trigger an update:
      $ ffx target update force-install fuchsia-pkg://fuchsia.com/update
  Also trigger a reboot after update:
      $ ffx target update force-install
      fuchsia-pkg://fuchsia.com/update
      --reboot
Notes:
  Directly invoke the system updater to install the provided update,
  bypassing any update checks.
  Interfaces using the 'fuchsia.update.installer' protocol to update the
  system. Requires an <update_pkg_url> in the following format:
  `fuchsia-pkg://fuchsia.com/update`
  Takes an optional `--reboot <true|false>` to trigger a system reboot
  after update has been successfully applied.
```

## trace

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx trace <command> [<args>]
Interact with the tracing subsystem
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  list-providers    List the target's trace providers
  record            Record a trace
```

### list-providers

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx trace list-providers
List the target's trace providers
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --help            display usage information
```

### record

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx trace record [--buffer-size <buffer-size>] [--categories <categories>] [--duration <duration>] [--output <output>]
Record a trace
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --buffer-size     size of per-provider trace buffer in MB.  Defaults to 4.
  --categories      comma-separated list of categories to enable.  Defaults to
                    "app,audio,benchmark,blobfs,gfx,input,kernel:meta,
                    kernel:sched,ledger,magma,minfs,modular,view,flutter,
                    dart,dart:compiler,dart:dart,dart:debugger,dart:embedder,
                    dart:gc,dart:isolate,dart:profiler,dart:vm"
  --duration        duration of trace capture in seconds. Defaults to 10
                    seconds.
  --output          name of output trace file.  Defaults to trace.fxt.
  --help            display usage information
```

## vdl

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx vdl [--sdk] <command> [<args>]
Start and manage Fuchsia emulators
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --sdk             running in fuchsia sdk (not inside the fuchsia code
                    repository)
  --help            display usage information
```

__Commands:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  start             Starting Fuchsia Emulator
  kill              Killing Fuchsia Emulator
```

### kill

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx vdl kill [-d <vdl-path>] [--launched-proto <launched-proto>]
Killing Fuchsia Emulator
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -d, --vdl-path    device_launcher binary location. Defaults to looking in
                    prebuilt/vdl/device_launcher
  --launched-proto  required, file containing device_launcher process artifact
                    location.
  --help            display usage information
```

### start

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx vdl start [-H] [-N] [--host-gpu] [--software-gpu] [--hidpi-scaling] [-u <upscript>] [--packages-to-serve <packages-to-serve>] [-p <pointing-device>] [-w <window-width>] [-h <window-height>] [-s <image-size>] [-f <device-proto>] [-e <aemu-path>] [--aemu-version <aemu-version>] [-d <vdl-path>] [--vdl-version <vdl-version>] [-x <grpcwebproxy>] [-X <grpcwebproxy-path>] [--grpcwebproxy-version <grpcwebproxy-version>] [-v <sdk-version>] [--gcs-bucket <gcs-bucket>] [--image-name <image-name>] [-l <emulator-log>] [--port-map <port-map>] [--vdl-output <vdl-output>] [--nointeractive]
Starting Fuchsia Emulator
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -H, --headless    bool, run emulator in headless mode.
  -N, --tuntap      bool, run emulator with emulated nic via tun/tap.
  --host-gpu        bool, run emulator with host GPU acceleration, this doesn't
                    work on remote-desktop with --headless.
  --software-gpu    bool, run emulator without host GPU acceleration, default.
  --hidpi-scaling   bool, enable pixel scaling on HiDPI devices.
  -u, --upscript    path to tun/tap upscript, this script will be executed
                    before booting up FEMU.
  --packages-to-serve
                    comma separated string of fuchsia package urls, extra
                    packages to serve after starting FEMU.
  -p, --pointing-device
                    set pointing device used on emulator: mouse or touch screen.
                    Allowed values are "touch", "mouse". Default is "touch".
  -w, --window-width
                    emulator window width. Default to 1280.
  -h, --window-height
                    emulator window height. Default to 800.
  -s, --image-size  extends storage size to <size> bytes. Default is "2G".
  -f, --device-proto
                    path to fuchsia virtual device configuration, if not
                    specified a generic one will be generated.
  -e, --aemu-path   path to aemu location. When running in fuchsia repo,
                    defaults to looking in prebuilt/third_party/aemu/PLATFORM.
                    When running in fuchsia sdk, defaults to looking in
                    $HOME/.fuchsia/femu.
  --aemu-version    label used to download AEMU from CIPD. Default is
                    "integration". Download only happens if aemu binary cannot
                    be found from known paths.
  -d, --vdl-path    device_launcher binary location. When running in fuchsia
                    repo, defaults to looking in prebuilt/vdl/device_launcher.
                    When running in fuchsia sdk, defaults to looking in
                    directory containing `fvdl`.
  --vdl-version     label used to download vdl from CIPD. Default is "latest".
                    Download only happens if vdl (device_launcher) binary cannot
                    be found from known paths.
  -x, --grpcwebproxy
                    enable WebRTC HTTP service on port, if set to 0 a random
                    port will be picked
  -X, --grpcwebproxy-path
                    location of grpcwebproxy, When running in fuchsia repo,
                    defaults to looking in prebuilt/third_party/grpcwebproxy
                    When running in fuchsia sdk, defaults to looking in
                    $HOME/.fuchsia/femu.
  --grpcwebproxy-version
                    label used to download grpcwebproxy from CIPD. Default is
                    "latest". Download only happens if --grpcwebproxy is set and
                    grpcwebproxy binary cannot be found from known paths or path
                    specified by --grpcwebproxy_path.
  -v, --sdk-version fuchsia sdk ID used to fetch from gcs, if specified, the
                    emulator will launch with fuchsia sdk files fetched from
                    gcs. To find the latest version run `gsutil cat
                    gs://fuchsia/development/LATEST_LINUX`.
  --gcs-bucket      gcs bucket name. Default is "fuchsia".
  --image-name      image file name used to fetch from gcs. Default is
                    "qemu-x64". To view availabe image names run `gsutil ls -l
                    gs://fuchsia/development/$(gsutil cat
                    gs://fuchsia/development/LATEST_LINUX)/images`.
  -l, --emulator-log
                    file path to store emulator log. Default is a temp file that
                    is deleted after `fvdl` exits.
  --port-map        host port mapping for user-networking mode. This flag will
                    be ignored if --tuntap is used. If not specified, an ssh
                    port on host will be randomly picked and forwarded. ex:
                    hostfwd=tcp::<host_port>-:<guest_port>,hostfwd=tcp::<host_port>-:<guest_port>
  --vdl-output      file destination to write `device_launcher` output. Required
                    for --nointeractive mode. Default is a temp file that is
                    deleted after `fvdl` exits. Specify this flag if you plan to
                    use the `kill` subcommand.
  --nointeractive   bool, turn off interactive mode. if turned off, fvdl will
                    not land user in ssh console. A ssh port will still be
                    forwarded. User needs to specify --vdl-output flag with this
                    mode, and manually call the `kill` subcommand to perform
                    clean shutdown.
  --help            display usage information
```

## version

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: ffx version [-v]
Print out ffx tool and daemon versions
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  -v, --verbose     if true, includes details about both ffx and the daemon
  --help            display usage information
```

