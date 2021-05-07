# log_listener CLI

The `log_listener` binary is included in all device builds that include `pkgfs` and is currently
the primary tool used by engineers to interactively read a live log stream. In-tree developers
usually invoke it with [`fx log`](https://fuchsia.dev/reference/tools/fx/cmd/log).

<table>
  <tr>
   <td>Flag
   </td>
   <td>Notes
   </td>
  </tr>
  <tr>
   <td><code>--only &lt;comma-separated-words></code>
   </td>
   <td>Include lines containing at least one of the specified words. If this is not set, it has no effect.
   </td>
  </tr>
  <tr>
   <td><code>--suppress &lt;comma-separated-words></code>
   </td>
   <td>Exclude lines containing any of the specified words.
   </td>
  </tr>
  <tr>
   <td><code>--begin &lt;comma-separated-words></code>
   </td>
   <td>Include blocks starting with at least one of the specified words. Pair with the <code>--end</code> flag. Defaults to off.
   </td>
  </tr>
  <tr>
   <td><code>--end &lt;comma-separated-words></code>
   </td>
   <td>Exclude blocks starting with at least one of the specified words. Pair with the <code>--begin</code> flag. Defaults to off.
   </td>
  </tr>
  <tr>
   <td><code>--tag &lt;string></code>
   </td>
   <td>Tag(s) to include. Use multiple times for multiple tags. By default no tag filtering is performed, only specified tags are allowed if any are specified.
   </td>
  </tr>
  <tr>
   <td><code>--ignore-tag &lt;string></code>
   </td>
   <td>Tag(s) to ignore. Use multiple times for multiple tags.
   </td>
  </tr>
  <tr>
   <td><code>--pid &lt;integer></code>
   </td>
   <td>Only print logs from the given process koid.
   </td>
  </tr>
  <tr>
   <td><code>--tid &lt;integer></code>
   </td>
   <td>Only print logs from the given thread koid.
   </td>
  </tr>
  <tr>
   <td><code>--pretty yes</code>
   </td>
   <td>Activate colors.
   </td>
  </tr>
  <tr>
   <td><code>--severity &lt;level></code>
   </td>
   <td>Minimum severity to include. Defaults to <code>INFO</code>.
<p>
Does not have any impact on the logs produced by components.
<p>
<code><level></code> must be one of <code>TRACE|DEBUG|INFO|WARN_ERROR|FATAL</code>.
   </td>
  </tr>
  <tr>
   <td><code>--file &lt;string></code>
   </td>
   <td>Path where logs will be written. By default logs are written to stdout.
   </td>
  </tr>
  <tr>
   <td><code>--file_capacity &lt;integer></code>
   </td>
   <td>The maximum allowed amount of disk space to consume. Once the file being written to reaches
   half of the capacity, it is moved to <code>FILE.old</code> and a new log file is created.
<p>
Defaults to 64000. If --file is not specified, it has no effect.
<p>
Setting to 0 disables this functionality.
   </td>
  </tr>
  <tr>
   <td><code>--startup_sleep &lt;integer></code>
   </td>
   <td>Sleep for this number of milliseconds on program startup.
   </td>
  </tr>
  <tr>
   <td><code>--clock &lt;Monotonic|UTC|Local></code>
   </td>
   <td>Clock to use when printing timestamps.
<p>
Monotonic (default): Monotonic time reported by the kernel.
<p>
UTC: UTC time as reported by the runtime.
<p>
Local: Localized wall time.
   </td>
  </tr>
  <tr>
   <td><code>--time_format &lt;format></code>
   </td>
   <td>If --clock is not MONOTONIC, specify timestamp format.
<p>
See <a href="https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html">the chrono crate's docs</a>
for format specifiers.
<p>
Defaults to "%Y-%m-%d %H:%M:%S".
   </td>
  </tr>
  <tr>
   <td><code>--since_now yes</code>
   </td>
   <td>Ignore all logs from before this command is invoked.
   </td>
  </tr>
  <tr>
   <td><code>--dump_logs yes</code>
   </td>
   <td>Exit after printing already-available logs.
   </td>
  </tr>
  <tr>
   <td><code>--help</code> | <code>-h</code>
   </td>
   <td>Prints usage.
   </td>
  </tr>
</table>
