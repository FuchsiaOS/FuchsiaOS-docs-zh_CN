# Driver Logging

You can have a driver send log messages to the
[syslog](/docs/development/diagnostics/logs/recording.md) through the use of the
`zxlogf(<log_level>,...)` macro, which is defined in
[lib/ddk/debug.h](/src/lib/ddk/include/lib/ddk/debug.h).

Depending on the type of log level, by default, log messages are sent to the
following logs:

* [syslog](/docs/development/diagnostics/logs/recording.md#logsinksyslog):
  * `ERROR`
  * `WARNING`
  * `INFO`
* [debuglog](/docs/development/diagnostics/logs/recording.md#debuglog_handles):
  * `SERIAL`

To control which log levels are sent to the syslog (other than `SERIAL`), the
[kernel commandline](/docs/reference/kernel/kernel_cmdline.md#drivernamelogflags)
`driver.<driver_name>.log=<level>` can be used. For example,
`driver.sdhci.log=TRACE` additionally enables `DEBUG` and `TRACE` logs for the
sdhci driver, as we are setting a _minimum_ log level, and `TRACE` is lower than
`DEBUG`.

A driver's logs are tagged with the process name, "driver", and the driver name.
This can be used to filter the output of the syslog whilst searching for
particular logs.

Note: There is both producer and consumer filtering applied to logs. The above
covers the producer-side, for more information on the consumer-side, and on how
to view driver logs, see
[viewing logs](/docs/development/diagnostics/logs/viewing.md).
