# Syslog

This document explains how to use golang syslogger library.


## gn dependency

```
deps = [
    "//garnet/public/lib/component/go/src/app",
    "//garnet/public/lib/syslog/go/src/syslog",
]
```

### Initialization

Logger can only be initialized once.

#### Basic initialization

```golang
import (
    "app/context"
    "syslog/logger"
)

func main() {
    ctx := context.CreateFromStartupInfo()
    err := logger.InitDefaultLogger(ctx.Connector())
}
```

#### Initialization with tags

```golang
import (
    "app/context"
    "syslog/logger"
)

func main() {
    ctx := context.CreateFromStartupInfo()
    // Global tags, max 4 tags can be passed. Every log message would be tagged using these.
    err := logger.InitDefaultLoggerWithTags(ctx.Connector(), tag1, tag2)
}
```

### Log messages

```golang
logger.Infof("my msg: %d", 10);

// Allow message specific tagging. This message is going to be tagged with
// this local tag and any global tag passed during initialization.
logger.InfoTf("tag", "my msg: %d", 10);

logger.Warnf("my msg: %d", 10);
logger.WarnTf("tag", "my msg: %d", 10);

logger.Errorf("my msg: %d", 10);
logger.ErrorTf("tag", "my msg: %d", 10);

logger.Fatalf("my msg: %d", 10);
logger.FatalTf("tag", "my msg: %d", 10);

logger.VLogf(1, "my msg: %d", 10); // verbose logs
logger.VLogTf(1, "tag", "my msg: %d", 10); // verbose logs
```

### Reference
[Golang APIs](https://fuchsia.googlesource.com/garnet/+/master/public/lib/syslog/go/src/syslog/logger/logger.go)
