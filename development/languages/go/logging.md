# Logging in Go

Go programs on Fuchsia generally use the [syslog package] and its `syslog.Infof()` functions.

See the [language agnostic logging docs](concepts/components/diagnostics/logs/README.md) for more information
about recording and viewing logs.

## Requirements

### GN dependencies

The necessary packages can be included with an addition to `deps` in `BUILD.gn`:

```
deps = [
    "//src/lib/component",
    "//src/lib/syslog/go",
]
```

See [Go: Overview][go-dev] for more information about building Go within Fuchsia.

### Component manifest dependency

Ensure that your component has the required capabilities to log by including the
following in your component manifest:

   * {.cmx}

   ```json
   {
     "include": [
       "syslog/client.shard.cmx"
     ],
     ...
   }
   ```

   * {.cml}

   ```json5
   {
     include: [
       "syslog/client.shard.cml"
     ],
     ...
   }
   ```

The syslog library will fallback to `stderr` if the `LogSink` connection fails.

## Initialization

Initializing without any tags will default to using the process name.

```golang
import (
    "go.fuchsia.dev/fuchsia/src/lib/component"
    syslog "go.fuchsia.dev/fuchsia/src/lib/syslog/go"
)

func main() {
  ctx := component.NewContextFromStartupInfo()
  {
    // Global tags, max 4 tags can be passed. Every log message is tagged with these.
    l, err := syslog.NewLoggerWithDefaults(ctx.Connector(), "my_tag")
    if err != nil {
      panic(err)
    }
    syslog.SetDefaultLogger(l)
  }
}
```

## Recording messages

The log methods have two variants: `Levelf` and `LevelTf` (e.g. `Infof` and `InfoTf`). The variant
of each method with a `T` accepts an additional tag for the message.

```golang
syslog.Infof("my msg: %d", 10);          // maps to INFO

// Allow message specific tagging. This message is going to be tagged with
// this local tag and any global tag passed during initialization.
syslog.InfoTf("tag", "my msg: %d", 10);

syslog.Warnf("my msg: %d", 10);          // maps to WARN
syslog.WarnTf("tag", "my msg: %d", 10);

syslog.Errorf("my msg: %d", 10);         // maps to ERROR
syslog.ErrorTf("tag", "my msg: %d", 10);

syslog.Fatalf("my msg: %d", 10);         // maps to FATAL
syslog.FatalTf("tag", "my msg: %d", 10);
```

## Standard streams

`fmt.Printf()`, `fmt.Sprintf()` etc. go to standard out (`stdout`) and standard error (`stderr`).

See [`stdout` & `stderr`] in the language-agnostic logging docs for details on the routing of stdio
streams in the system.

[syslog package]: /src/lib/syslog/go
[go-dev]: development/languages/go/README.md
[`stdout` & `stderr`]: development/diagnostics/logs/recording.md#stdout-stderr
