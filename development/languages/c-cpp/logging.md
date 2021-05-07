# Syslog

This document explains how to get started with syslogger APIs.

### Component manifest dependency

Ensure that your component has the required capabilities to log by including the
following in your component manifest:

   * {.cmx}

   ```json
   {
     "include": [
       "sdk/lib/diagnostics/syslog/client.shard.cmx"
     ],
     ...
   }
   ```

   * {.cml}

   ```json5
   {
     include: [
       "sdk/lib/diagnostics/syslog/client.shard.cmx"
     ],
     ...
   }
   ```

Note: The above is only available for in-tree development.
This is tracked in [fxbug.dev/64207](http://fxbug.dev/64207).
Out of tree developers should copy the snippets shown below instead.

   * {.cmx}

   ```json
   {
     "sandbox": {
       "services": [
         "fuchsia.logger.LogSink"
       ]
     },
     ...
   }
   ```

   * {.cml}

   ```json5
   {
     use: [
       { protocol: "fuchsia.logger.LogSink" },
     ],
   }
   ```

## Default configuration

The global logger is lazily instantiated on the first use of the API (more
specifically, on the first call to `fx_log_get_logger`). The default
configuration for the global logger is:

- Use process name as the tag
- Write logs to `fuchsia.logger.LogSink`
- Min log level of `FX_LOG_INFO`

## In C

### BUILD.gn dependency

```gn
//zircon/public/lib/syslog
```

### Log messages

```C
FX_LOGF(INFO, "tag", "my msg: %d", 10);
FX_LOG(INFO, "tag", "my msg");
FX_LOGF(INFO, NULL, "my msg: %d", 10);
```

### Using non-default configuration

```C
#include <lib/syslog/global.h>

int main(int argc, char** argv) {
    fx_logger_config_t config = {.min_severity = FX_LOG_INFO,
                                 .console_fd = -1,
                                 .log_service_channel = ZX_HANDLE_INVALID,
                                 .tags = (const char * []) {"gtag", "gtag2"},
                                 .num_tags = 2};
    fx_log_reconfigure(&config);
}
```

### Reference

[C APIs](/zircon/system/ulib/syslog/include/lib/syslog/global.h)

## In C++

### BUILD.gn dependency

```gn
//sdk/lib/syslog/cpp
//sdk/lib/syslog/cpp:backend_legacy
```

### Log messages

```C++
FX_LOGS(INFO) << "my message";
FX_LOGST(INFO, "tag") << "my message";
```

### Set tags

By default, the process name is used as the tag, but this can be changed by
calling `syslog::SetTags`.

```C++
#include <lib/syslog/cpp/log_settings.h>

int main(int argc, char** argv) {
     syslog::SetTags({"tag1", "tag2"});
}
```

### Set settings

```C++
#include "<lib/syslog/cpp/log_settings.h>

int main(int argc, char** argv) {
     syslog::LogSettings settings = {.min_log_level = syslog::LOG_ERROR};
     syslog::SetLogSettings(settings, {"tag1", "tag2"});
}
```

### Set settings from command line

```C++
#include "src/lib/fxl/command_line.h"
#include "src/lib/fxl/log_settings_command_line.h"

int main(int argc, char** argv) {
    auto command_line = fxl::CommandLineFromArgcArgv(argc, argv);
    fxl::SetLogSettingsFromCommandLine(command_line, {"my_program"});
}
```

### GTest main with syslog initialized from command line

No initialization is required for using the default configuration of
syslog. If you would like your test suite to change the configuration
based on command line arguments (e.g. --verbose), use:

```gn
//src/lib/fxl/test:gtest_main
```

### Reference

[C++ APIs](/sdk/lib/syslog/cpp/macros.h)
<br/>
[Command line initialization API](/src/lib/fxl/log_settings_command_line.h)
