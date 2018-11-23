# Syslog

This document explains how to get started with syslogger APIs.

## In C

### BUILD.gn dependency

```gn
//zircon/public/lib/syslog
```

### Initialization

Logger can only be initialized once.

#### Basic initialization

```C
#include <lib/syslog/global.h>

int main(int argc, char** argv) {
    fx_log_init();
}
```

#### Initialization with tags

```C
#include <lib/syslog/global.h>

int main(int argc, char** argv) {
    fx_logger_config_t config = {.min_severity = FX_LOG_INFO,
                                 .console_fd = -1,
                                 .log_service_channel = ZX_HANDLE_INVALID,
                                 .tags = (const char * []) {"gtag", "gtag2"},
                                 .num_tags = 2};
    fx_log_init_with_config(&config);
}
```

### Log messages

```C
FX_LOGF(INFO, "tag", "my msg: %d", 10);
FX_LOG(INFO, "tag", "my msg");
FX_LOGF(INFO, NULL, "my msg: %d", 10);
```

### Reference

[C APIs](https://fuchsia.googlesource.com/zircon/+/master/system/ulib/syslog/include/syslog/global.h)

## In C++

From garnet and above layers.

### BUILD.gn dependency

```gn
//garnet/public/lib/syslog/cpp
```

### sandboxing dependency

```
{
    "sandbox": {
        "services": [
            "fuchsia.logger.LogSink"
        ]
    }
}
```

### Initialization

Logger can only be initialized once.

#### Basic initialization

```C++
#include "lib/syslog/cpp/logger.h"

int main(int argc, char** argv) {
    syslog::InitLogger();
}
```

#### Initialization with tags

```C++
#include "lib/syslog/cpp/logger.h"

int main(int argc, char** argv) {
     syslog::InitLogger({"tag1", "tag2"});
}
```

#### Initialization using command line

```C++
#include "lib/fxl/command_line.h"
#include "lib/fsl/syslogger/init.h"

int main(int argc, char** argv) {
    auto command_line = fxl::CommandLineFromArgcArgv(argc, argv);
    fsl::InitLoggerFromCommandLine(command_line, {"my_program"});
}
```

### Log messages

```C++
FX_LOGS(INFO) << "my message";
FX_LOGST(INFO, "tag") << "my message";
```

### Reference

[C++ APIs](https://fuchsia.googlesource.com/garnet/+/master/public/lib/syslog/cpp/logger.h)
<br/>
[FSL initialization API](https://fuchsia.googlesource.com/garnet/+/master/public/lib/fsl/syslogger/init.h)
