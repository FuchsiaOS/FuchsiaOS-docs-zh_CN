# fidlcat: Guide

Once you have [launched `fidlcat`](/docs/development/monitoring/fidlcat) and
attached to a running process, the tool begins logging system calls sent and
received using FIDL.

See the following basic example output from `fidlcat`:

<pre><font color="#26A269">Monitoring </font><font color="#12488B">echo_client.cm</font> koid=<font color="#C01C28">193974</font>

echo_client.cm <font color="#C01C28">193974</font>:<font color="#C01C28">193976</font> zx_channel_create(options: <font color="#26A269">uint32</font> = <font color="#12488B">0</font>)
  -&gt; <font color="#26A269">ZX_OK</font> (out0: <font color="#26A269">handle</font> = <font color="#C01C28">d7e9f83b</font>(<font color="#26A269">channel</font>:<font color="#12488B">0</font>), out1: <font color="#26A269">handle</font> = <font color="#C01C28">d6c9fd5f</font>(<font color="#26A269">channel</font>:<font color="#12488B">1</font>))
</pre>

The example output contains the following information:

-   **echo_client.cm**: the name of the process that has generated this display.

-   **193974**: the process koid.

-   **193976**: the thread koid.

-   **zx_channel_create**: the name of the intercepted/displayed system call.

-   system call input parameters (such as **handle** and **options**) listed by
    name, type, and value.

-   system call return value (`ZX_OK`) and output parameters.

For system calls representing a FIDL transaction, `fidlcat` displays additional
input and output parameters. See the following example of a synchronous
`fuchsia.examples/Echo.EchoString` request:

<pre>echo_client.cm <font color="#C01C28">193974</font>:<font color="#C01C28">193976</font> zx_channel_call_etc(handle: <font color="#26A269">handle</font> = <font color="#C01C28">Channel:d089f8fb</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc/fuchsia.examples.Echo</font>), options: <font color="#26A269">uint32</font> = <font color="#12488B">ZX_CHANNEL_WRITE_USE_IOVEC</font>, deadline: <font color="#26A269">zx.time</font> = <font color="#12488B">ZX_TIME_INFINITE</font>, rd_num_bytes: <font color="#26A269">uint32</font> = <font color="#12488B">64</font>, rd_num_handles: <font color="#26A269">uint32</font> = <font color="#12488B">64</font>)
  <span style="background-color:#A347BA"><font color="#D0CFCC">sent request</font></span> <font color="#26A269">fuchsia.examples/Echo.EchoString</font> = { value: <font color="#26A269">string</font> = <font color="#C01C28">&quot;hello&quot;</font> }
  -&gt; <font color="#26A269">ZX_OK</font>
    <span style="background-color:#A347BA"><font color="#D0CFCC">received response</font></span> <font color="#26A269">fuchsia.examples/Echo.EchoString</font> = { response: <font color="#26A269">string</font> = <font color="#C01C28">&quot;hello&quot;</font> }
</pre>

Notice the FIDL request and response messages in the display output, including
the method name and parameters.

## Modifying the display

By default, `fidlcat` only displays process information on the first line of
each message. Use the flag `--with-process-info` to include these details on
each line:

<pre>echo_client.cm <font color="#C01C28">60014</font>:<font color="#C01C28">60016</font> zx_channel_call_etc(handle: <font color="#26A269">handle</font> = <font color="#C01C28">Channel:35272afb</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc/fuchsia.examples.Echo</font>), options: <font color="#26A269">uint32</font> = <font color="#12488B">ZX_CHANNEL_WRITE_USE_IOVEC</font>, deadline: <font color="#26A269">zx.time</font> = <font color="#12488B">ZX_TIME_INFINITE</font>, rd_num_bytes: <font color="#26A269">uint32</font> = <font color="#12488B">64</font>, rd_num_handles: <font color="#26A269">uint32</font> = <font color="#12488B">64</font>)
echo_client.cm <font color="#C01C28">60014</font>:<font color="#C01C28">60016</font>   <span style="background-color:#A347BA"><font color="#D0CFCC">sent request</font></span> <font color="#26A269">fuchsia.examples/Echo.EchoString</font> = { value: <font color="#26A269">string</font> = <font color="#C01C28">&quot;hello&quot;</font> }
echo_client.cm <font color="#C01C28">60014</font>:<font color="#C01C28">60016</font>   -&gt; <font color="#26A269">ZX_OK</font>
echo_client.cm <font color="#C01C28">60014</font>:<font color="#C01C28">60016</font>     <span style="background-color:#A347BA"><font color="#D0CFCC">received response</font></span> <font color="#26A269">fuchsia.examples/Echo.EchoString</font> = { response: <font color="#26A269">string</font> = <font color="#C01C28">&quot;hello&quot;</font> }
</pre>

## Stack frames

Note: If a program crashes while `fidlcat` is attached, the stack frames print
automatically to the display.

Using the flag `--stack` you can display the stack frames for every system
call. By default (`--stack=0`), the stack frames are not displayed.

With `--stack=1` only the call site (1 to 4 frames) is displayed:

<pre>echo_client.cm <font color="#C01C28">675407</font>:<font color="#C01C28">675409</font> <span style="background-color:#E9AD0C">at </span><span style="background-color:#E9AD0C"><font color="#C01C28">zircon/system/ulib/fidl/llcpp_message.cc</font></span><span style="background-color:#E9AD0C">:</span><span style="background-color:#E9AD0C"><font color="#12488B">243:12</font></span> fidl::OutgoingMessage::CallImpl
echo_client.cm <font color="#C01C28">675407</font>:<font color="#C01C28">675409</font> zx_channel_call_etc(handle: <font color="#26A269">handle</font> = <font color="#C01C28">Channel:8b745347</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc/fuchsia.examples.Echo</font>), options: <font color="#26A269">uint32</font> = <font color="#12488B">ZX_CHANNEL_WRITE_USE_IOVEC</font>, deadline: <font color="#26A269">zx.time</font> = <font color="#12488B">ZX_TIME_INFINITE</font>, rd_num_bytes: <font color="#26A269">uint32</font> = <font color="#12488B">64</font>, rd_num_handles: <font color="#26A269">uint32</font> = <font color="#12488B">64</font>)
  <span style="background-color:#A347BA"><font color="#D0CFCC">sent request</font></span> <font color="#26A269">fuchsia.examples/Echo.EchoString</font> = { value: <font color="#26A269">string</font> = <font color="#C01C28">&quot;hello&quot;</font> }
  -&gt; <font color="#26A269">ZX_OK</font>
    <span style="background-color:#A347BA"><font color="#D0CFCC">received response</font></span> <font color="#26A269">fuchsia.examples/Echo.EchoString</font> = { response: <font color="#26A269">string</font> = <font color="#C01C28">&quot;hello&quot;</font> }
</pre>

This option doesn't add any overhead (except for the display).

With `--stack=2` all the frames are displayed:

<pre>echo_client.cm <font color="#C01C28">717533</font>:<font color="#C01C28">717535</font> <span style="background-color:#E9AD0C">at </span><span style="background-color:#E9AD0C"><font color="#C01C28">3ac285b4811</font></span> _start
echo_client.cm <font color="#C01C28">717533</font>:<font color="#C01C28">717535</font> <span style="background-color:#E9AD0C">at </span><span style="background-color:#E9AD0C"><font color="#C01C28">zircon/third_party/ulib/musl/src/env/__libc_start_main.c</font></span><span style="background-color:#E9AD0C">:</span><span style="background-color:#E9AD0C"><font color="#12488B">215:5</font></span> __libc_start_main
echo_client.cm <font color="#C01C28">717533</font>:<font color="#C01C28">717535</font> <span style="background-color:#E9AD0C">at </span><span style="background-color:#E9AD0C"><font color="#C01C28">zircon/third_party/ulib/musl/src/env/__libc_start_main.c</font></span><span style="background-color:#E9AD0C">:</span><span style="background-color:#E9AD0C"><font color="#12488B">140:3</font></span> start_main
echo_client.cm <font color="#C01C28">717533</font>:<font color="#C01C28">717535</font> <span style="background-color:#E9AD0C">at </span><span style="background-color:#E9AD0C"><font color="#C01C28">examples/fidl/llcpp/client_sync/main.cc</font></span><span style="background-color:#E9AD0C">:</span><span style="background-color:#E9AD0C"><font color="#12488B">30:27</font></span> main
echo_client.cm <font color="#C01C28">717533</font>:<font color="#C01C28">717535</font> <span style="background-color:#E9AD0C">at </span><span style="background-color:#E9AD0C"><font color="#C01C28">fidling/gen/examples/fidl/fuchsia.examples/fuchsia.examples/llcpp/fidl/fuchsia.examples/cpp/wire_messaging.h</font></span><span style="background-color:#E9AD0C">:</span><span style="background-color:#E9AD0C"><font color="#12488B">2711:12</font></span> fidl::internal::WireSyncClientImpl&lt;fuchsia_examples::Echo&gt;::EchoString
echo_client.cm <font color="#C01C28">717533</font>:<font color="#C01C28">717535</font> <span style="background-color:#E9AD0C">at </span><span style="background-color:#E9AD0C"><font color="#C01C28">fidling/gen/examples/fidl/fuchsia.examples/fuchsia.examples/llcpp/fidl/fuchsia.examples/cpp/wire_messaging.cc</font></span><span style="background-color:#E9AD0C">:</span><span style="background-color:#E9AD0C"><font color="#12488B">1051:12</font></span> fidl::WireResult&lt;fuchsia_examples::Echo::EchoString&gt;::WireResult
echo_client.cm <font color="#C01C28">717533</font>:<font color="#C01C28">717535</font> <span style="background-color:#E9AD0C">at </span><span style="background-color:#E9AD0C"><font color="#C01C28">zircon/system/ulib/fidl/include/lib/fidl/llcpp/message.h</font></span><span style="background-color:#E9AD0C">:</span><span style="background-color:#E9AD0C"><font color="#12488B">205:3</font></span> fidl::OutgoingMessage::Call&lt;fidl::WireResponse&lt;fuchsia_examples::Echo::EchoString&gt;, zx::unowned&lt;zx::channel&gt; &gt;
echo_client.cm <font color="#C01C28">717533</font>:<font color="#C01C28">717535</font> <span style="background-color:#E9AD0C">at </span><span style="background-color:#E9AD0C"><font color="#C01C28">zircon/system/ulib/fidl/include/lib/fidl/llcpp/message.h</font></span><span style="background-color:#E9AD0C">:</span><span style="background-color:#E9AD0C"><font color="#12488B">196:5</font></span> fidl::OutgoingMessage::Call&lt;fidl::WireResponse&lt;fuchsia_examples::Echo::EchoString&gt; &gt;
echo_client.cm <font color="#C01C28">717533</font>:<font color="#C01C28">717535</font> <span style="background-color:#E9AD0C">at </span><span style="background-color:#E9AD0C"><font color="#C01C28">zircon/system/ulib/fidl/llcpp_message.cc</font></span><span style="background-color:#E9AD0C">:</span><span style="background-color:#E9AD0C"><font color="#12488B">243:12</font></span> fidl::OutgoingMessage::CallImpl
echo_client.cm <font color="#C01C28">717533</font>:<font color="#C01C28">717535</font> zx_channel_call_etc(handle: <font color="#26A269">handle</font> = <font color="#C01C28">Channel:f751d2fb</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc/fuchsia.examples.Echo</font>), options: <font color="#26A269">uint32</font> = <font color="#12488B">ZX_CHANNEL_WRITE_USE_IOVEC</font>, deadline: <font color="#26A269">zx.time</font> = <font color="#12488B">ZX_TIME_INFINITE</font>, rd_num_bytes: <font color="#26A269">uint32</font> = <font color="#12488B">64</font>, rd_num_handles: <font color="#26A269">uint32</font> = <font color="#12488B">64</font>)
  <span style="background-color:#A347BA"><font color="#D0CFCC">sent request</font></span> <font color="#26A269">fuchsia.examples/Echo.EchoString</font> = { value: <font color="#26A269">string</font> = <font color="#C01C28">&quot;hello&quot;</font> }
  -&gt; <font color="#26A269">ZX_OK</font>
    <span style="background-color:#A347BA"><font color="#D0CFCC">received response</font></span> <font color="#26A269">fuchsia.examples/Echo.EchoString</font> = { response: <font color="#26A269">string</font> = <font color="#C01C28">&quot;hello&quot;</font> }
</pre>

This option adds some overhead because we need to ask zxdb for the full stack
for each system call (and fidlcat becomes even more verbose). You should use it
only when you need to understand what part of your code called the system calls.

## Filtering output

### Syscalls

By default, `fidlcat` only displays `zx_channel` syscalls.
The `--syscalls` option allows you to define a regular expression that selects
the syscalls to decode and display.

To display all the syscalls, use: `--syscalls=".\*"`

The `--exclude-syscalls` flag defines a regular expreission that excludes
syscalls from the set selected by `--syscalls`.

To be displayed, a syscall must satisfy the `--syscalls` pattern and not
satisfy the `--exclude-syscalls` pattern.

The following example displays all syscalls, **except** for `zx_handle`:

```none
--syscalls ".\*" --exclude-syscalls "zx_handle_.\*"
```

### Messages

By default, `fidlcat` displays all the messages.
You can specify the messages you want to display using:

*   `--messages` allows you to specify one or more regular expressions the
    messages must satisfy to be displayed.

*   `--exclude-messages` allows you to specify one or more regular expressions
    the messages must not satisfy to be displayed.

If both options are used at the same time, to be displayed, a message must
satisfy one of the regular expressions specified with `--messages` and not
satisfy any regular expression specified with `--exclude-messages`.

Message filtering works on the method's fully qualified name. For example,
the following flag:

```none
--messages=".*Open"
```

Matches methods like:

```none {:.devsite-disable-click-to-copy}
fuchsia.io/Directory.Open
fuchsia.io/Node.OnOpen
```

### Threads

When using the option `--thread=<thread koid>` only the events from the specified
thread are displayed. The option can be used several times to display several
threads.

## Grouping output

### Protocols

Use the options `--with=top` and `--with=top=<path>` to generate a view that
groups the output by process, protocol, and method. The groups are sorted by
number of events, so groups with more associated events are listed earlier.

### Threads

Use the options `--with=group-by-thread` and `--with=group-by-thread=<path>` to
generate a view that displays a short version of all the events for each thread.

## Postponing the message display

By default, `fidlcat` begins displaying messages immediately after it attaches
to the process.

You can use the `--trigger` option to defer the display until the provided
regular expression matches an incoming message.

This is really useful when you need to understand what's going on after you
received or emit a particular message.

## Summary view

To configure `fidlcat` to display a high level summary of the session instead of
listing individual messages, use the options `--with=summary` and
`--with=summary=<path>`.

<pre>echo_client.cm <font color="#C01C28">1505832</font>: 16 handles

  <font color="#C01C28">Process:ac4ce043</font>(<font color="#26A269">proc-self</font>)

  <font color="#C01C28">startup Vmar:a43cfe53</font>(<font color="#26A269">vmar-root</font>)

  <font color="#C01C28">startup Thread:d5dce00f</font>(<font color="#26A269">thread-self</font>)

  <font color="#C01C28">startup Channel:91cce2f3</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc</font>)
      write <span style="background-color:#A347BA"><font color="#D0CFCC">request </font></span> <font color="#26A269">fuchsia.io/Directory.Open</font>(<font color="#C01C28">&quot;.&quot;</font>)
        -&gt; <font color="#C01C28">Channel:c65ce1c3</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc</font>)

  <font color="#C01C28">startup Channel:daece3fb</font>(<font color="#26A269">dir</font>:<font color="#12488B">/pkg</font>)

  <font color="#C01C28">startup Socket:cb8ce31f</font>(<font color="#26A269">fd</font>:<font color="#12488B">1</font>)
    closed by <font color="#26A269">zx_handle_close</font>

  <font color="#C01C28">startup Socket:df8ce687</font>(<font color="#26A269">fd</font>:<font color="#12488B">2</font>)
    closed by <font color="#26A269">zx_handle_close</font>

  <font color="#C01C28">startup Channel:93ccfcf7</font>(<font color="#26A269">directory-request</font>:<font color="#12488B">/</font>)

  <font color="#C01C28">startup Clock:b7ecfe9b</font>()

  <font color="#C01C28">startup Job:674ce17f</font>(<font color="#26A269">job-default</font>)

  <font color="#C01C28">startup Vmo:adbcfc9f</font>(<font color="#26A269">vdso-vmo</font>)

  <font color="#C01C28">startup Vmo:ef2ce06f</font>(<font color="#26A269">stack-vmo</font>)

  <font color="#C01C28">Channel:c65ce1c3</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc</font>)
    linked to <font color="#C01C28">Channel:da9cebcb</font>(<font color="#26A269">channel</font>:<font color="#12488B">1</font>)
    created by <font color="#26A269">zx_channel_create</font>
      write <span style="background-color:#A347BA"><font color="#D0CFCC">request </font></span> <font color="#26A269">fuchsia.io/Directory.Open</font>(<font color="#C01C28">&quot;fuchsia.examples.Echo&quot;</font>)
        -&gt; <font color="#C01C28">Channel:767ce3f3</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc/fuchsia.examples.Echo</font>)
    closed by <font color="#26A269">zx_handle_close</font>

  <font color="#C01C28">Channel:da9cebcb</font>(<font color="#26A269">channel</font>:<font color="#12488B">1</font>)
    linked to <font color="#C01C28">Channel:c65ce1c3</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc</font>)
    created by <font color="#26A269">zx_channel_create</font>
    closed by <font color="#C01C28">Channel:91cce2f3</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc</font>) sending <font color="#26A269">fuchsia.io/Directory.Open</font>

  <font color="#C01C28">Channel:767ce3f3</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc/fuchsia.examples.Echo</font>)
    linked to <font color="#C01C28">Channel:f4bce307</font>(<font color="#26A269">channel</font>:<font color="#12488B">3</font>)
    created by <font color="#26A269">zx_channel_create</font>
      call  <span style="background-color:#A347BA"><font color="#D0CFCC">request </font></span> <font color="#26A269">fuchsia.examples/Echo.EchoString</font>
      write <span style="background-color:#A347BA"><font color="#D0CFCC">request </font></span> <font color="#26A269">fuchsia.examples/Echo.SendString</font>
      read  <span style="background-color:#A347BA"><font color="#D0CFCC">event   </font></span> <font color="#26A269">fuchsia.examples/Echo.OnString</font>
    closed by <font color="#26A269">zx_handle_close</font>

  <font color="#C01C28">Channel:f4bce307</font>(<font color="#26A269">channel</font>:<font color="#12488B">3</font>)
    linked to <font color="#C01C28">Channel:767ce3f3</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc/fuchsia.examples.Echo</font>)
    created by <font color="#26A269">zx_channel_create</font>
    closed by <font color="#C01C28">Channel:c65ce1c3</font>(<font color="#26A269">dir</font>:<font color="#12488B">/svc</font>) sending <font color="#26A269">fuchsia.io/Directory.Open</font>
</pre>

This displays a list of all the monitored processes, handles, and channels in
the session with additional summary details:

*   **Handles**: Whether the handle is a startup handle (inherited during
    process creation) or created during the process life.
    For non-startup handles, `fidlcat` also displays information about the
    syscalls used to create and close each handle.

*   **Channels**: Displays the handle responsible for the other end of the
    channel and the list of FIDL messages sent and received.

## Continuous monitoring

By default, the `fidlcat` session terminates when all the attached processes
exit.

Use the option `--stay-alive` to keep the session running until you manually
exit `fidlcat` (for example, using Ctrl-C).

This allows you to restart a program multiple times within the same monitoring
session. With each restart, the `fidlcat` session attaches to the new process
automatically.
