# Guide to Configuring the Modular Framework

Note: The Modular framework is being deprecated in favor of
the [Session Framework](/docs/concepts/session/introduction.md).

## Requirements

To configure the modular framework, you will need to create a JSON file defining
the required configurations for `basemgr` and `sessionmgr` as detailed below.
The configuration file should be packaged via the build rule `modular_config`,
which will validate your file against a schema. You must then include the
`modular_config()` target in the product's base packages.

The file may contain (non-standard JSON) C-style comments
(`/* block */` and `// inline`).

## Reading configuration

The configuration provided to `basemgr` is available through
the [component inspection][docs-inspect] of the `basemgr` component.

Use [`iquery`][docs-iquery] or `fx iquery` to query the configuration
of a running `basemgr`:

```posix-terminal
iquery show 'basemgr.cmx:root:config'
```

When using a session launcher component, the launcher provides a different
configuration to `sessionmgr` that is used for the launched session.
For the launched session, you can query a running `sessionmgr` to get
this configuration:

```posix-terminal
iquery show 'sessionmgr.cmx:root:config'
```

## Launching Modular with custom configuration

Modular normally uses the configuration embedded in the product image
with the `modular_config` build rule. When developing agents and shells,
it may be useful to launch Modular with a custom configuration file.

`basemgr_launcher` is a shell utility that overrides the configuration with
one provided through stdin and starts `basemgr`. To launch a session,
run (from host machine):

```posix-terminal
cat myconfig.json | fx shell basemgr_launcher
```

### Persistent configuration

By default, configuration provided to `basemgr_launcher` is only used for a
single instance of the session. For example, rebooting the device launches
Modular with the default configuration from `modular_config`.

The `basemgr_launcher` configuration can be persisted across session restarts
and reboots by adding the `allow_persistent_config_override` build rule
to a non-production build:

```posix-terminal
fx set ... --with //src/modular/build:allow_persistent_config_override
```

When enabled, `basemgr_launcher` stores configuration provided to it in
its component cache, and `basemgr` uses it instead of the default
configuration. Subsequent invocations of `basemgr_launcher` overwrite existing
persistent configuration.

You can delete the persistent configuration by running (from host machine):

```posix-terminal
fx shell basemgr_launcher delete_config
```

## Example

The fields used in the startup configuration depend on whether a session launcher
component is specified in the `basemgr.session_launcher` field.

If `basemgr.session` is present, all other fields except for
`basemgr.enable_cobalt` are ignored, and the session launcher component is
responsible for instructing `basemgr` to launch a session with a complete
configuration file.

### Session launcher component

```json5
// Fields not specified here are ignored.
{
  "basemgr": {
    "enable_cobalt": false,
    "session_launcher": {
      "url": "fuchsia-pkg://fuchsia.com/custom_session#meta/custom_session.cmx",
      "args": [ "--foo", "--bar" ]
    }
  }
}
```

### Typical configuration

```json5
{
  /* This is a block comment.
     Comments are ignored. */
  // This is an inline comment. Comments are ignored.
  "basemgr": {
    "enable_cobalt": false,
    "use_session_shell_for_story_shell_factory": true,
    "session_shells": [
      {
        "url": "fuchsia-pkg://fuchsia.com/dev_session_shell#meta/dev_session_shell.cmx",
        "display_usage": "near",
        "screen_height": 50.0,
        "screen_width": 100.0
      }
    ]
  },
  "sessionmgr": {
    "startup_agents": [
      "fuchsia-pkg://fuchsia.com/startup_agent#meta/startup_agent.cmx"
    ],
    "session_agents": [
      "fuchsia-pkg://fuchsia.com/session_agent#meta/session_agent.cmx"
    ],
    "component_args": [
      {
        "uri": "fuchsia-pkg://fuchsia.com/startup_agent#meta/startup_agent.cmx",
        "args": [ "--foo", "--bar=true" ]
      }
    ],
    "agent_service_index": [
      {
        "service_name": "fuchsia.modular.SomeServiceName",
        "agent_url": "fuchsia-pkg://fuchsia.com/some_agent#meta/some_agent.cmx"
      }
    ],
    "restart_session_on_agent_crash": [
      "fuchsia-pkg://fuchsia.com/some_agent#meta/some_agent.cmx"
    ]
  }
}
```

## Basemgr fields

- `base_shell` **object** _(optional)_
  - **WARNING:** Basemgr no longer launches base shells. This section is unused.
  - `url`: **string** _(optional)_
    - **This field is unused.**
    - **default**: `fuchsia-pkg://fuchsia.com/auto_login_base_shell#meta/auto_login_base_shell.cmx`
  - `keep_alive_after_login` **boolean** _(optional)_
    - **This field is unused.**
    - **default**: `false`
  - `args` **string[]** _(optional)_
    - **This field is unused.**
    - **default**: []
- `session_shells` **array** _(optional)_
  - List of exactly one session shell containing the following
    fields (is an Array type for backwards compatibility):
    - `url`: **string** _(required)_
      - The fuchsia component url for which session shell to use.
    - `display_usage`: **string** _(optional)_
      - The display usage policy for this session shell.
      - Options:
        - `handheld`: the display is used well within arm's reach.
        - `close`: the display is used at arm's reach.
        - `near`: the display is used beyond arm's reach.
        - `midrange`: the display is used beyond arm's reach.
        - `far`: the display is used well beyond arm's reach.
    - `screen_height`: **float** _(optional)_
      - The screen height in millimeters for the session shell's display.
    - `screen_width`: **float** _(optional)_
      - The screen width in millimeters for the session shell's display.
  - **default**: A single session shell with the following properties:
    - `url`: `fuchsia-pkg://fuchsia.com/dev_story_shell#meta/dev_session_shell.cmx`
    - `display_usage`: `unknown`
    - `screen_height`: `0`
    - `screen_width`: `0`
- `story_shell_url`: **string** _(optional)_
  - The fuchsia component url for which story shell to use.
  - **default**: `fuchsia-pkg://fuchsia.com/dev_story_shell#meta/dev_story_shell.cmx`
- `enable_cobalt`: **boolean** _(optional)_
  - When set to false, Cobalt statistics are disabled.
  - **default**: `true`
- `use_session_shell_for_story_shell_factory`: **boolean** _(optional)_
  - Create story shells through StoryShellFactory exposed by the session shell
    instead of creating separate story shell components. When set,
    `story_shell_url` and any story shell args are ignored.
  - **default**: `false`
- `session_launcher` **object** _(optional)_
  - When set, basemgr will launch this component instead of sessionmgr
    on startup and ignore all other configuration properties, except
    `basemgr.enable_cobalt`. This component can use the `fuchsia.session.Launcher`
    protocol to launch sessionmgr.
    - `url`: **string** _(required)_
      - The Fuchsia component URL for the session component.
    - `args` **string[]** _(optional)_
      - A list of arguments to be passed to the session component specified by `url`.

## Sessionmgr fields

- `enable_cobalt`: **boolean** _(optional)_
  - When set to false, Cobalt statistics are disabled. This is used for
    testing.
  - **default**: `true`
- `startup_agents`: **string[]** _(optional)_
  - A list of fuchsia component urls that specify which agents to launch at
    startup.
- `session_agents`: **string[]** _(optional)_
  - A list of fuchsia component urls that specify which agents to launch at
    startup with PuppetMaster and FocusProvider services.
- `component_args`: **array** _(optional)_
  - A list of key/value pairs to construct a map from component URI to
    arguments list for that component. Presence in this list results in the
    given arguments passed to the component as its argv at launch.
    - `uri`: The component's uri.
    - `args`: A list of arguments to be passed to the component specified by
      `uri`. Arguments must be prefixed with --.
- `agent_service_index`: **array** _(optional)_
  - A list of key/value pairs mapping from service name to the serving component's
    URL. Agents and the session shell are both valid components to specify
    here.  Service names must be unique: only one component can provide any
    given service. These services are provided to modules, the session shell,
    and agents, in their incoming namespace (i.e. at the path
    "/svc/fully.qualified.ServiceName").
    - `service_name`: The name of a service offered by `agent_url`.
    - `agent_url`: A fuchsia component url that specifies which agent/shell will
      provide the named service.
- `restart_session_on_agent_crash`: **array** _(optional)_
  - A list of agent URLs that will cause the session to be restarted
    when they terminate unexpectedly. If an agent is not in this list,
    sessionmgr will restart it individually, preserving the session.

    The session shell is automatically added to this list.
- `disable_agent_restart_on_crash`: **boolean** _(optional)_
  - When set to true, disables any automatic restarts of agents listed in
    `session_agents` if they crash.
  - **default**: `false`

[docs-inspect]: /docs/development/diagnostics/inspect/README.md
[docs-iquery]: /docs/reference/diagnostics/consumers/iquery.md
