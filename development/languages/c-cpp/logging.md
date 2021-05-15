# Syslog

本文介绍如何使用 syslogger API。

### 组件清单依赖项

通过在组件清单中包含以下内容，确保组件具有所需的日志功能：

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

注意：以上内容仅适用于树内开发。在[fxbug.dev/64207](http://fxbug.dev/64207)处对此进行了跟踪。树外的开发人员应该复制下面显示的代码片段。

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

## 默认配置

首次使用 API 时会延迟实例化全局 logger（更具体地说，是首次调用 `fx_log_get_logger` 时）。全局 logger 的默认配置为：

- 使用进程名称作为标签
- 将日志写入 `fuchsia.logger.LogSink`
- `FX_LOG_INFO` 最低日志级别 

## 在 C 中

### BUILD.gn 依赖项

```gn
//zircon/public/lib/syslog
```

### 日志消息

```C
FX_LOGF(INFO, "tag", "my msg: %d", 10);
FX_LOG(INFO, "tag", "my msg");
FX_LOGF(INFO, NULL, "my msg: %d", 10);
```

### 使用非默认配置

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

### 参考

[C API](/zircon/system/ulib/syslog/include/lib/syslog/global.h)

## 在 C++ 中

### BUILD.gn 依赖项

```gn
//sdk/lib/syslog/cpp
//sdk/lib/syslog/cpp:backend_legacy
```

### 日志消息

```C++
FX_LOGS(INFO) << "my message";
FX_LOGST(INFO, "tag") << "my message";
```

### 设置标签

默认情况下，进程名称用作标签，但是可以通过调用 `syslog::SetTags` 进行更改。

```C++
#include <lib/syslog/cpp/log_settings.h>

int main(int argc, char** argv) {
     syslog::SetTags({"tag1", "tag2"});
}
```

### 设置日志设置

```C++
#include "<lib/syslog/cpp/log_settings.h>

int main(int argc, char** argv) {
     syslog::LogSettings settings = {.min_log_level = syslog::LOG_ERROR};
     syslog::SetLogSettings(settings, {"tag1", "tag2"});
}
```

### 从命令行设置日志设置

```C++
#include "src/lib/fxl/command_line.h"
#include "src/lib/fxl/log_settings_command_line.h"

int main(int argc, char** argv) {
    auto command_line = fxl::CommandLineFromArgcArgv(argc, argv);
    fxl::SetLogSettingsFromCommandLine(command_line, {"my_program"});
}
```

### 从命令行初始化 syslog 的 GTest main

使用 syslog 的默认配置不需要初始化。如果您希望测试套件根据命令行参数（例如 --verbose）更改配置，请使用：

```gn
//src/lib/fxl/test:gtest_main
```

### 参考

[C++ API](/sdk/lib/syslog/cpp/macros.h)
<br/>
[命令行初始化 API](/src/lib/fxl/log_settings_command_line.h)
