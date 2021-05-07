# fidlcat: Guide

## Launching fidlcat

For information about launching fidlcat: [fidlcat](/docs/development/monitoring/fidlcat).

{% dynamic if request.tld != "dev" %}

>> #  Notice
>>
>> This file only renders correctly from fuchsia.dev. Please navigate to
>> https://fuchsia.dev/fuchsia-src/development/monitoring/fidlcat/fidlcat_usage.md to see the examples correctly!

{% dynamic endif %}

## Default display

The default display for fidlcat is:


<pre>echo_client_cpp_synchronous <font color="#CC0000">180768</font>:<font color="#CC0000">180781</font> zx_channel_call(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">14b21e1b</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>, deadline:<font color="#4E9A06">time</font>: <font color="#3465A4">ZX_TIME_INFINITE</font>, rd_num_bytes:<font color="#4E9A06">uint32</font>: <font color="#3465A4">65536</font>, rd_num_handles:<font color="#4E9A06">uint32</font>: <font color="#3465A4">64</font>)
  <span style="background-color:#75507B"><font color="#D3D7CF">sent request</font></span> <font color="#4E9A06">fidl.examples.echo/Echo.EchoString</font> = {
    value: <font color="#4E9A06">string</font> = <font color="#CC0000">&quot;hello synchronous world&quot;</font>
  }
  -&gt; <font color="#4E9A06">ZX_OK</font>
    <span style="background-color:#75507B"><font color="#D3D7CF">received response</font></span> <font color="#4E9A06">fidl.examples.echo/Echo.EchoString</font> = {
      response: <font color="#4E9A06">string</font> = <font color="#CC0000">&quot;hello synchronous world&quot;</font>
    }
</pre>


We have the following information:

-   **echo_client_cpp_synchronous**: the name of the application that has
    generated this display.

-   **180768**: the process koid.

-   **180781**: the thread koid.

-   **zx_channel_call**: the name of the intercepted/displayed system call.

-   all the basic input parameters of the system call (here **handle** and
    **options**).

    For each one, we have:

    -   The name of the parameter in black.

    -   The type of the parameter in green.

    -   The value of the parameter (the color depends on the parameter type).

-   all the complex input parameters. Here we display a FIDL message. This is a
    request, which is sent by our application.

The display stops here. It will resume when the system call returns (sometimes
it can be a very long time). For one thread, there will be no other display
between the input arguments and the returned value. However, another thread
display may be interleaved. When the system call returns, we display:

-   The returned value (-> ZX_OK)

-   The basic output parameters (there is no basic output parameters in this
    example).

-   The complex output parameters. Here we display a FIDL message. This is the
    response we received to the request we sent.

For **zx_channel_read** we can have this display:


<pre>echo_client_rust <font color="#CC0000">256109</font>:<font color="#CC0000">256122</font> zx_channel_read(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">e4c7c57f</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>, num_bytes:<font color="#4E9A06">uint32</font>: <font color="#3465A4">48</font>, num_handles:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>)
  -&gt; <font color="#4E9A06">ZX_OK</font>
    <span style="background-color:#75507B"><font color="#D3D7CF">received response</font></span> <font color="#4E9A06">fidl.examples.echo/Echo.EchoString</font> = {
      response: <font color="#4E9A06">string</font> = <font color="#CC0000">&quot;hello world!&quot;</font>
    }
</pre>


But, if there is an error, we can have:

<pre>echo_client_rust <font color="#CC0000">256109</font>:<font color="#CC0000">256122</font> zx_channel_read(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">e4c7c57f</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>, num_bytes:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>, num_handles:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>)
  -&gt; <font color="#CC0000">ZX_ERR_SHOULD_WAIT</font>
</pre>

Or:

<pre>echo_client_rust <font color="#CC0000">256109</font>:<font color="#CC0000">256122</font> zx_channel_read(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">e4c7c57f</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>, num_bytes:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>, num_handles:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>)
  -&gt; <font color="#CC0000">ZX_ERR_BUFFER_TOO_SMALL</font> (actual_bytes:<font color="#4E9A06">uint32</font>: <font color="#3465A4">48</font>, actual_handles:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>)
</pre>

In this last case, even if the system call fails, we have some valid output
parameters. **actual_bytes** and **actual_handles** give the minimal values
which should have been used to call **zx_channel_read**.

## Modifying the display

By default, we only display the process information on the first line.

Eventually, we also display the process information before the returned value if
a system call from another thread has been displayed between the call and the
returned value:

<pre>ledger.cmx <font color="#CC0000">5859666</font>:<font color="#CC0000">5861991</font> zx_channel_write(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">035393df</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>)
  <span style="background-color:#75507B"><font color="#D3D7CF">sent request</font></span> <font color="#4E9A06">fuchsia.io/Directory.Open</font> = {
    flags: <font color="#4E9A06">uint32</font> = <font color="#3465A4">12582912</font>
    mode: <font color="#4E9A06">uint32</font> = <font color="#3465A4">0</font>
    path: <font color="#4E9A06">string</font> = <font color="#CC0000">&quot;29/cache/cached_db&quot;</font>
    object: <font color="#4E9A06">handle</font> = <font color="#CC0000">03f3b46b</font>
  }

ledger.cmx <font color="#CC0000">5859666</font>:<font color="#CC0000">5859693</font> zx_channel_write(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">035393df</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>)
  <span style="background-color:#75507B"><font color="#D3D7CF">sent request</font></span> <font color="#4E9A06">fuchsia.io/Directory.Open</font> = {
    flags: <font color="#4E9A06">uint32</font> = <font color="#3465A4">13107200</font>
    mode: <font color="#4E9A06">uint32</font> = <font color="#3465A4">0</font>
    path: <font color="#4E9A06">string</font> = <font color="#CC0000">&quot;.&quot;</font>
    object: <font color="#4E9A06">handle</font> = <font color="#CC0000">0053b5fb</font>
  }

ledger.cmx <font color="#CC0000">5859666</font>:<font color="#CC0000">5861991</font>   -&gt; <font color="#4E9A06">ZX_OK</font>

ledger.cmx <font color="#CC0000">5859666</font>:<font color="#CC0000">5859693</font>   -&gt; <font color="#4E9A06">ZX_OK</font>
</pre>

Using the flag **--with-process-info**, we can display the process information
on each line:

<pre>echo_client_rust <font color="#CC0000">305640</font>:<font color="#CC0000">305653</font> zx_channel_write(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">4446ec4b</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>)
echo_client_rust <font color="#CC0000">305640</font>:<font color="#CC0000">305653</font>   <span style="background-color:#75507B"><font color="#D3D7CF">sent request</font></span> <font color="#4E9A06">fidl.examples.echo/Echo.EchoString</font> = {
echo_client_rust <font color="#CC0000">305640</font>:<font color="#CC0000">305653</font>     value: <font color="#4E9A06">string</font> = <font color="#CC0000">&quot;hello world!&quot;</font>
echo_client_rust <font color="#CC0000">305640</font>:<font color="#CC0000">305653</font>   }
echo_client_rust <font color="#CC0000">305640</font>:<font color="#CC0000">305653</font>   -&gt; <font color="#4E9A06">ZX_OK</font>
</pre>

This is very useful if we want to do a **grep** on the output (for example, to
only select one thread).

## Interpreting the display

Most of the time we want to link several messages to be able to understand what
our program is doing.

In this example:

<pre>ledger.cmx <font color="#CC0000">5859666</font>:<font color="#CC0000">5859693</font> zx_channel_create(options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>)
  -&gt; <font color="#4E9A06">ZX_OK</font> (out0:<font color="#4E9A06">handle</font>: <font color="#CC0000">0243b493</font>, out1:<font color="#4E9A06">handle</font>: <font color="#CC0000">0163b42b</font>)

ledger.cmx <font color="#CC0000">5859666</font>:<font color="#CC0000">5859693</font> zx_channel_write(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">035393df</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>)
  <span style="background-color:#75507B"><font color="#D3D7CF">sent request</font></span> <font color="#4E9A06">fuchsia.io/Directory.Open</font> = {
    flags: <font color="#4E9A06">uint32</font> = <font color="#3465A4">12582912</font>
    mode: <font color="#4E9A06">uint32</font> = <font color="#3465A4">0</font>
    path: <font color="#4E9A06">string</font> = <font color="#CC0000">&quot;29&quot;</font>
    object: <font color="#4E9A06">handle</font> = <font color="#CC0000">0163b42b</font>
  }
  -&gt; <font color="#4E9A06">ZX_OK</font>

ledger.cmx <font color="#CC0000">5859666</font>:<font color="#CC0000">5859693</font> zx_channel_read(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">0243b493</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">1</font>, num_bytes:<font color="#4E9A06">uint32</font>: <font color="#3465A4">64</font>, num_handles:<font color="#4E9A06">uint32</font>: <font color="#3465A4">1</font>)
  -&gt; <font color="#4E9A06">ZX_OK</font>
    <span style="background-color:#75507B"><font color="#D3D7CF">received response</font></span> <font color="#4E9A06">fuchsia.io/Node.OnOpen</font> = {
      s: <font color="#4E9A06">int32</font> = <font color="#3465A4">-25</font>
      info: <font color="#4E9A06">fuchsia.io/NodeInfo</font> = <font color="#3465A4">null</font>
    }

ledger.cmx <font color="#CC0000">5859666</font>:<font color="#CC0000">5859693</font> zx_channel_read(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">0243b493</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">1</font>, num_bytes:<font color="#4E9A06">uint32</font>: <font color="#3465A4">64</font>, num_handles:<font color="#4E9A06">uint32</font>: <font color="#3465A4">1</font>)
  -&gt; <font color="#4E9A06">ZX_OK</font>
    <span style="background-color:#75507B"><font color="#D3D7CF">received response</font></span> <font color="#4E9A06">fuchsia.io/Node.OnOpen</font> = {
      s: <font color="#4E9A06">int32</font> = <font color="#3465A4">0</font>
      info: <font color="#4E9A06">fuchsia.io/NodeInfo</font> = { directory: <font color="#4E9A06">fuchsia.io/DirectoryObject</font> = {} }
    }

ledger.cmx <font color="#CC0000">5859666</font>:<font color="#CC0000">5859693</font> zx_channel_call(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">0243b493</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>, deadline:<font color="#4E9A06">time</font>: <font color="#3465A4">ZX_TIME_INFINITE</font>, rd_num_bytes:<font color="#4E9A06">uint32</font>: <font color="#3465A4">24</font>, rd_num_handles:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>)
  <span style="background-color:#75507B"><font color="#D3D7CF">sent request</font></span> <font color="#4E9A06">fuchsia.io/Node.Close</font> = {}
  -&gt; <font color="#4E9A06">ZX_OK</font>
    <span style="background-color:#75507B"><font color="#D3D7CF">received response</font></span> <font color="#4E9A06">fuchsia.io/Node.Close</font> = {
      s: <font color="#4E9A06">int32</font> = <font color="#3465A4">0</font>
    }
</pre>

We first create a channel. The two handles **0243b493** and **0163b42b** are
linked. That means that a write on one handle will result on a read on the other
handle.

We use handle **0163b42b** in the **Directory.Open** message. That means that
the associated handle (**0243b493**) is the handle that controls the directory
we just opened.

When we receive **Node.OnOpen** on **0243b493** we know that it's a response to
our **Directory.Open**. We also used the handle to call **Node.Close**.

## Stack frames

By default, only the system calls are displayed. However, it's sometime
interesting to know where a system call has been called. Using the flag
**--stack** we can display the stack frames for every system call.

By default (**--stack=0**), the stack frames are not displayed.

With **--stack=1** only the call site (1 to 4 frames) is displayed:

<pre>echo_client_cpp <font color="#CC0000">5231515</font>:<font color="#CC0000">5231528</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">zircon/system/ulib/fidl/message.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">62</font></span> fidl::Message::Read
echo_client_cpp <font color="#CC0000">5231515</font>:<font color="#CC0000">5231528</font> zx_channel_read(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">a0575917</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>, num_bytes:<font color="#4E9A06">uint32</font>: <font color="#3465A4">65536</font>, num_handles:<font color="#4E9A06">uint32</font>: <font color="#3465A4">64</font>)
  -&gt; <font color="#4E9A06">ZX_OK</font>
    <span style="background-color:#75507B"><font color="#D3D7CF">received response</font></span> <font color="#4E9A06">fidl.examples.echo/Echo.EchoString</font> = {
      response: <font color="#4E9A06">string</font> = <font color="#CC0000">&quot;hello world&quot;</font>
    }
</pre>

This option doesn't add any overhead (except for the display).

With **--stack=2** all the frames are displayed:

<pre>echo_client_cpp <font color="#CC0000">5234749</font>:<font color="#CC0000">5234762</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">zircon/third_party/ulib/musl/src/env/__libc_start_main.c</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">74</font></span> start_main
echo_client_cpp <font color="#CC0000">5234749</font>:<font color="#CC0000">5234762</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">garnet/examples/fidl/echo_client_cpp/echo_client.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">40</font></span> main
echo_client_cpp <font color="#CC0000">5234749</font>:<font color="#CC0000">5234762</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">zircon/system/ulib/async-loop/loop_wrapper.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">25</font></span> async::Loop::Run
echo_client_cpp <font color="#CC0000">5234749</font>:<font color="#CC0000">5234762</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">zircon/system/ulib/async-loop/loop.c</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">241</font></span> async_loop_run
echo_client_cpp <font color="#CC0000">5234749</font>:<font color="#CC0000">5234762</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">zircon/system/ulib/async-loop/loop.c</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">284</font></span> async_loop_run_once
echo_client_cpp <font color="#CC0000">5234749</font>:<font color="#CC0000">5234762</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">zircon/system/ulib/async-loop/loop.c</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">335</font></span> async_loop_dispatch_wait
echo_client_cpp <font color="#CC0000">5234749</font>:<font color="#CC0000">5234762</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">sdk/lib/fidl/cpp/internal/message_reader.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">165</font></span> fidl::internal::MessageReader::CallHandler
echo_client_cpp <font color="#CC0000">5234749</font>:<font color="#CC0000">5234762</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">sdk/lib/fidl/cpp/internal/message_reader.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">177</font></span> fidl::internal::MessageReader::OnHandleReady
echo_client_cpp <font color="#CC0000">5234749</font>:<font color="#CC0000">5234762</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">sdk/lib/fidl/cpp/internal/message_reader.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">201</font></span> fidl::internal::MessageReader::ReadAndDispatchMessage
echo_client_cpp <font color="#CC0000">5234749</font>:<font color="#CC0000">5234762</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">zircon/system/ulib/fidl/message.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">62</font></span> fidl::Message::Read
echo_client_cpp <font color="#CC0000">5234749</font>:<font color="#CC0000">5234762</font> zx_channel_read(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">a95c4cdf</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>, num_bytes:<font color="#4E9A06">uint32</font>: <font color="#3465A4">65536</font>, num_handles:<font color="#4E9A06">uint32</font>: <font color="#3465A4">64</font>)
  -&gt; <font color="#4E9A06">ZX_OK</font>
    <span style="background-color:#75507B"><font color="#D3D7CF">received response</font></span> <font color="#4E9A06">fidl.examples.echo/Echo.EchoString</font> = {
      response: <font color="#4E9A06">string</font> = <font color="#CC0000">&quot;hello world&quot;</font>
    }
</pre>

This option adds some overhead because we need to ask zxdb for the full stack
for each system call (and fidlcat becomes even more verbose). You should use it
only when you need to understand what part of your code called the system calls.

## Exceptions

Sometimes, your program crashes. If it's monitored by **fidlcat** you
automatically have a stack where it crashed.

For example:

<pre>echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> zx_channel_read(handle:<font color="#4E9A06">handle</font>: <font color="#CC0000">ca322b6f</font>, options:<font color="#4E9A06">uint32</font>: <font color="#3465A4">0</font>, num_bytes:<font color="#4E9A06">uint32</font>: <font color="#3465A4">65536</font>, num_handles:<font color="#4E9A06">uint32</font>: <font color="#3465A4">64</font>)
  -&gt; <font color="#4E9A06">ZX_OK</font>
    <span style="background-color:#75507B"><font color="#D3D7CF">received request</font></span> <font color="#4E9A06">fidl.examples.echo/Echo.EchoString</font> = {
      value: <font color="#4E9A06">string</font> = <font color="#CC0000">&quot;hello world&quot;</font>
    }

echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">zircon/third_party/ulib/musl/src/env/__libc_start_main.c</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">93</font></span> start_main
echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">garnet/examples/fidl/echo_server_cpp/echo_server.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">15</font></span> main
echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">zircon/system/ulib/async-loop/loop_wrapper.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">21</font></span> async::Loop::Run
echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">zircon/system/ulib/async-loop/loop.c</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">194</font></span> async_loop_run
echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">zircon/system/ulib/async-loop/loop.c</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">236</font></span> async_loop_run_once
echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">zircon/system/ulib/async-loop/loop.c</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">277</font></span> async_loop_dispatch_wait
echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">sdk/lib/fidl/cpp/internal/message_reader.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">165</font></span> fidl::internal::MessageReader::CallHandler
echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">sdk/lib/fidl/cpp/internal/message_reader.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">177</font></span> fidl::internal::MessageReader::OnHandleReady
echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">sdk/lib/fidl/cpp/internal/message_reader.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">228</font></span> fidl::internal::MessageReader::ReadAndDispatchMessage
echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">sdk/lib/fidl/cpp/internal/stub_controller.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">32</font></span> fidl::internal::StubController::OnMessage
echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">fidling/gen/garnet/examples/fidl/services/fidl/examples/echo/cpp/fidl.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">152</font></span> fidl::examples::echo::Echo_Stub::Dispatch_
echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <span style="background-color:#FCE94F">at </span><span style="background-color:#FCE94F"><font color="#CC0000">garnet/examples/fidl/echo_server_cpp/echo_server_app.cc</font></span><span style="background-color:#FCE94F">:</span><span style="background-color:#FCE94F"><font color="#3465A4">22</font></span> echo::EchoServerApp::EchoString
echo_server_cpp.cmx <font color="#CC0000">1707964</font>:<font color="#CC0000">1707977</font> <font color="#CC0000">thread stopped on exception</font>
</pre>

You have the stack frames for the exception even if you didn't ask for the stack
frames with the **--stack** options.

## Syscalls

By default, fidlcat only displays the zx_channel syscalls.

You can display all the syscalls and choose which ones you want to display.

The **--syscalls** option let you define a regular expression that selects the
syscalls to decode and display.

It can be passed multiple times.

To display all the syscalls, use: **--syscalls=".\*"**

The **--exclude-syscalls** flag lets you exclude some syscalls selected by
**--syscalls**. It's a regular expression that selects the syscalls to not
decode and display.

It can be passed multiple times.

To be displayed, a syscall must satisfy the **--syscalls** pattern and not
satisfy the **--exclude-syscalls** pattern.

To display all the syscalls but the zx_handle syscalls, use:

--syscalls ".\*" --exclude-syscalls "zx_handle_.\*"

## Filtering messages

By default, fidlcat displays all the messages.

You can specify the messages you want to display using:

*   **--messages** allows you to specify one or more regular expressions the messages must satisfy
to be displayed.

*   **--exclude-messages** allows you to specify one or more regular expressions the messages must
not satisfy to be displayed.

If both options are used at the same time, to be displayed, a message must satisfy one of the
regular expressions specified with **--messages** and not satisfy any regular expression specified
with **--exclude-messages**.

Message filtering works on the method's fully qualified name. For example, with:

```
--messages=".*Open"
```

Methods like:

```
fuchsia.io/Directory.Open
fuchsia.io/Node.OnOpen
```

Will match.

## Postponing the message display

By default, everything is displayed as soon as an application is monitored. You can differ the
display using **--trigger**. With this option the display will start only if one message satisfying
one of the regular expressions specified with **--trigger** is encountered.

This is really useful when you need to understand what's going on after you received or emit a
particular message.

## Filter threads

When using the option `--thread=<thread koid>` only the events from the specified thread are
displayed. The option can be used several times to display several threads.

## High level summary

Sometime, you don't need to display all the messages exchanged but only a high level view of the
session. The options `--with=summary` and `--with=summary=&lt;path&gt;` generate a high level
summary of the session.

With those options, fidlcat displays a list of all the monitored processes.

For each process, fidlcat displays a list of all the handles found for the process.

For each handle, fidlcat displays the information we have about the process. That includes the data
fidlcat has been able to infer. It also display if the handle is a startup handle (inherited while
the process has been created) or a handle created during the process life.

For channels, fidlcat tries to display the other end of the channel (even if the other end is own
by another process).

For non startup handles, fidlcat displays how the handle was created:

*   by calling zx_channel_create, zx_port_create, zx_timer_create, ...

*   by receiving a handle within a message.

Then fidlcat displays a list of all the messages sent and received (only for channels).

Finally fidlcat displays how the handle was closed:

*   by calling zx_handle_close or zx_handle_close_many.

*   by sending the handle to another process.

If fidlcat doesn't display that a handle is closed, that probably means that the program forgot to
close it.

In addition to `--with=summary` you can use `--stack=2`. In that case, the stack frame is displayed
for each channel creation.

## Continuous monitoring

By default, fidlcat ends when all the monitored processes are ended.

With the option `--stay-alive`, fidlcat doesn't automatically end (which means a control-c to end
it).

This is useful to monitor a service you want to restart. When launched, fidlcat will monitor the
current version of the service. When the service is restarted, fidlcat stops monitoring the old
process and then automatically monitors the new process. This way you can have all the messages
exchanged when the service is restarted.

## Top protocols

The options `--with=top` and `--with=top=&lt;path&gt;` generate a view that groups the output by
process, protocol, and method. The groups are sorted by number of events, so groups with more
associated events are listed earlier.

## Group by thread

The options `--with=group-by-thread` and `--with=group-by-thread=&lt;path&gt;` generate a view that, for each thread, displays a short version of all the events.
