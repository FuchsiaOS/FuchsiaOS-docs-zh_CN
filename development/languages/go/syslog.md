<!-- # Syslog

This document explains how to use golang syslogger library. -->
# 系统日志
该文档向开发者展示了如何使用golang下的系统日志库。


<!--## gn dependency-->
## gn 依赖项

```
deps = [
    "//garnet/public/lib/component/go/src/app",
    "//garnet/public/lib/syslog/go/src/syslog",
]
```

<!-- ### Initialization
Logger can only be initialized once. -->

### 初始化
日志记录器只能被初始化一次

<!-- #### Basic initialization -->
#### 基本的初始化

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

<!-- #### Initialization with tags -->
#### 使用用标签初始化

```golang
import (
    "app/context"
    "syslog/logger"
)

func main() {
    ctx := context.CreateFromStartupInfo()
    // 全局标签，最多能传递4个标签。每一条日志消息都将被打上这些标签。
    err := logger.InitDefaultLoggerWithTags(ctx.Connector(), tag1, tag2)
}
```

<!-- ### Log messages -->
### 日志消息

```golang
logger.Infof("my msg: %d", 10);

// 可以对某一条消息指定标签。这条消息将被打上这个局部标签（tag）以及在初始化中被传递的全局标签。
logger.InfoTf("tag", "my msg: %d", 10);

logger.Warnf("my msg: %d", 10);
logger.WarnTf("tag", "my msg: %d", 10);

logger.Errorf("my msg: %d", 10);
logger.ErrorTf("tag", "my msg: %d", 10);

logger.Fatalf("my msg: %d", 10);
logger.FatalTf("tag", "my msg: %d", 10);

logger.VLogf(1, "my msg: %d", 10); // 详细日志
logger.VLogTf(1, "tag", "my msg: %d", 10); // 详细日志
```

<!-- ### Reference -->
### 参考
[Golang APIs](https://fuchsia.googlesource.com/garnet/+/master/public/lib/syslog/go/src/syslog/logger/logger.go)
