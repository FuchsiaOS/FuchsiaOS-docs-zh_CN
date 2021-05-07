# Tracing: C and C++ macros

You can use these C and C++ macros to record tracing data after
you have set your component as a tracing provider. For more information,
see [Adding tracing in your code](/docs/development/tracing/tutorial/adding-tracing-in-code.md).

These macros are defined in
[//zircon/system/ulib/trace/include/lib/trace/internal/event_common.h](/zircon/system/ulib/trace/include/lib/trace/internal/event_common.h).

Warning: This reference document is created manually and may be incomplete.

## Encoding macros {#encoding-macros}

When you want to record a `value` with tracing macros, it is important
that you properly encode each value. Use the respective macro based on
the data type that you want to record.

Note: C++ can infer most data types, but in the cases marked as required for C++,
you must use an encoding macro.

<table>
  <tr>
    <th>Macro</th>
    <th>Description</th>
  </tr>
  <tr>
    <td><code>TA_NULL</code></td>
    <td><p><b>Optional for C++</b></p>
      <p>A <code>NULL</code> value.</p>
      <p>This macro does not take any arguments. You can use it as
      <code>TA_NULL()</code>.</p>
      <p>In C++, you can alternatively use <code>nullptr</code>.</p>
    </td>
  <tr>
  <tr>
    <td><code>TA_BOOL</code></td>
    <td><p><b>Optional for C++</b></p>
      <p>A boolean value.</p>
    </td>
  <tr>
  <tr>
    <td><code>TA_INT32</code></td>
    <td><p><b>Optional for C++</b></p>
      <p>A signed 32-bit integer value.</p>
    </td>
  <tr>
  <tr>
    <td><code>TA_UINT32</code></td>
    <td><p><b>Optional for C++</b></p>
      <p>An unsigned 32-bit integer value.</p>
    </td>
  <tr>
  <tr>
    <td><code>TA_INT64</code></td>
    <td><p><b>Optional for C++</b></p>
      <p>A signed 64-bit integer value.</p>
    </td>
  <tr>
  <tr>
    <td><code>TA_UINT64</code></td>
    <td><p><b>Optional for C++</b></p>
      <p>An unsigned 32-bit integer value.</p>
    </td>
  <tr>
  <tr>
    <td><code>TA_DOUBLE</code></td>
    <td><p><b>Optional for C++</b></p>
      <p>A double precision floating point value.</p>
    </td>
  <tr>
  <tr>
    <td><code>TA_CHAR_ARRAY</code></td>
    <td><p><b>Required for C++</b></p>
      <p>A character array with a length, which is copied rather than cached.</p>
      <p>This macro takes two arguments. The first argument is a pointer to
      the character array. The second argument is the length of the array.</p>
    </td>
  <tr>
  <tr>
    <td><code>TA_STRING</code></td>
    <td><p><b>Optional for C++</b></p>
      <p>A NULL terminated dynamic string, which is copied rather than cached.</p>
    </td>
  <tr>
  <tr>
    <td><code>TA_STRING_LITERAL</code></td>
    <td><p><b>Required for C++</b></p>
      <p>A NULL terminated dynamic string, which is cached.</p>
    </td>
  <tr>
  <tr>
    <td><code>TA_POINTER</code></td>
    <td><p><b>Optional for C++</b></p>
      <p>A pointer value that records the memory address
       and not the target.</p>
    </td>
  <tr>
  <tr>
    <td><code>TA_KOID</code></td>
    <td><p><b>Required for C++</b></p>
      <p>A kernel object ID. For more information,
      see <a href="/docs/reference/kernel_objects/objects.md">Zircon kernel objects
      </a>.</p>
    </td>
  <tr>
</table>

### C++ notes

In C++, when you use a literal constant, type inference needs a hint
in order to get the size, signedness, and type right.

For example, is the value `77` a signed 32-bit integer? An unsigned 32-bit
integer? Or maybe even a 64-bit integer of some kind?

Type inference in the tracing macros works according to the standard C++ rules:

*   `77` is a signed 32-bit integer, `TA_INT32`
*   `77U` is an unsigned 32-bit integer, `TA_UINT32`
*   `77L` is a signed 64-bit integer, `TA_INT64`
*   `77LU` is an unsigned 64-bit integer, `TA_UINT64`

This also means that floating point needs to be explicitly noted if it's an
integer value.

For example:

* `77` is a `TA_INT32`.
* `77.` is a `TA_DOUBLE`.

If you are using constants, you should consider retaining the
encoding macros if you are expressing the values directly, or you should use
the appropriate `const` type. The following examples all do the same thing,
but show the use of a well defined type, a type macro, and a discouraged example.

* {Defined type}

  ```cpp
  const int32_t my_id = 77;                       // well defined type
  TRACE_INSTANT("category", "name", "int", my_id);
  ```

* {Type macro}

  ```cpp
  #define MY_ID   (TA_INT32(77))                  // uses the typing macro
  TRACE_INSTANT("category", "name", "int", MY_ID);
  ```

* {Discouraged}

  ```cpp
  TRACE_INSTANT("category", "name", "int", 77);   // discouraged
  ```

## TRACE_ENABLED {#TRACE_ENABLED}

```
 Returns true if tracing is enabled.

 Usage:

     if (TRACE_ENABLED()) {
         // do something possibly expensive only when tracing is enabled
     }

```

## TRACE_CATEGORY_ENABLED {#TRACE_CATEGORY_ENABLED}

```
 Returns true if tracing of the specified category has been enabled (which
 implies that |TRACE_ENABLED()| is also true).

 |category_literal| must be a null-terminated static string constant.

 Usage:

     if (TRACE_CATEGORY_ENABLED("category")) {
         // do something possibly expensive only when tracing this category
     }

```
## TRACE_NONCE {#TRACE_NONCE}

```
 Returns a new unique 64-bit unsigned integer (within this process).
 Each invocation returns a different non-zero value.
 Useful for generating identifiers for async and flow events.

 Usage:

     trace_async_id_t async_id = TRACE_NONCE();
     TRACE_ASYNC_BEGIN("category", "name", async_id);
     // a little while later...
     TRACE_ASYNC_END("category", "name", async_id);

```
## TRACE_INSTANT {#TRACE_INSTANT}

```
 Writes an instant event representing a single moment in time (a probe).

 0 to 15 arguments can be associated with the event, each of which is used
 to annotate the moment with additional information.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |scope| is |TRACE_SCOPE_THREAD|, |TRACE_SCOPE_PROCESS|, or |TRACE_SCOPE_GLOBAL|.
 |args| is the list of argument key/value pairs.

 Usage:

     TRACE_INSTANT("category", "name", TRACE_SCOPE_PROCESS, "x", TA_INT32(42));

```

## TRACE_COUNTER {#TRACE_COUNTER}

```
 Writes a counter event with the specified id.

 The arguments to this event are numeric samples are typically represented by
 the visualizer as a stacked area chart.  The id serves to distinguish multiple
 instances of counters that share the same category and name within the
 same process.

 1 to 15 numeric arguments can be associated with the event, each of which is
 interpreted as a distinct time series.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |counter_id| is the correlation id of the counter.
              Must be unique for a given process, category, and name combination.
 |args| is the list of argument key/value pairs.

 Usage:

     trace_counter_id_t counter_id = 555;
     TRACE_COUNTER("category", "name", counter_id, "x", TA_INT32(42), "y", TA_DOUBLE(2.0))

```

## TRACE_DURATION {#TRACE_DURATION}

```
 Writes a duration event that ends when the current scope exits.

 Durations describe work that is happening synchronously on one thread.
 They can be nested to represent a control flow stack.

 0 to 15 arguments can be associated with the event, each of which is used
 to annotate the duration with additional information.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |args| is the list of argument key/value pairs.

 Usage:

     void function(int arg) {
         TRACE_DURATION("category", "name", "arg", TA_INT32(arg));
         // do something useful here
     }

```

## TRACE_DURATION_BEGIN {#TRACE_DURATION_BEGIN}

```
 Writes a duration begin event only.
 This event must be matched by a duration end event with the same category and name.

 Durations describe work that is happening synchronously on one thread.
 They can be nested to represent a control flow stack.

 0 to 15 arguments can be associated with the event, each of which is used
 to annotate the duration with additional information.  The arguments provided
 to matching duration begin and duration end events are combined together in
 the trace; it is not necessary to repeat them.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |args| is the list of argument key/value pairs.

 Usage:

     TRACE_DURATION_BEGIN("category", "name", "x", TA_INT32(42));

```

## TRACE_DURATION_END {#TRACE_DURATION_END}

```
 Writes a duration end event only.

 Durations describe work that is happening synchronously on one thread.
 They can be nested to represent a control flow stack.

 0 to 15 arguments can be associated with the event, each of which is used
 to annotate the duration with additional information.  The arguments provided
 to matching duration begin and duration end events are combined together in
 the trace; it is not necessary to repeat them.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |args| is the list of argument key/value pairs.

 Usage:

     TRACE_DURATION_END("category", "name", "x", TA_INT32(42));

```

## TRACE_ASYNC_BEGIN {#TRACE_ASYNC_BEGIN}

```
 Writes an asynchronous begin event with the specified id.
 This event may be followed by async instant events and must be matched by
 an async end event with the same category, name, and id.

 Asynchronous events describe work that is happening asynchronously and that
 may span multiple threads.  Asynchronous events do not nest.  The id serves
 to correlate the progress of distinct asynchronous operations that share
 the same category and name within the same process.

 0 to 15 arguments can be associated with the event, each of which is used
 to annotate the asynchronous operation with additional information.  The
 arguments provided to matching async begin, async instant, and async end
 events are combined together in the trace; it is not necessary to repeat them.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |async_id| is the correlation id of the asynchronous operation.
            Must be unique for a given process, category, and name combination.
 |args| is the list of argument key/value pairs.

 Usage:

     trace_async_id_t async_id = 555;
     TRACE_ASYNC_BEGIN("category", "name", async_id, "x", TA_INT32(42));

```

## TRACE_ASYNC_INSTANT {#TRACE_ASYNC_INSTANT}

```
 Writes an asynchronous instant event with the specified id.

 Asynchronous events describe work that is happening asynchronously and that
 may span multiple threads.  Asynchronous events do not nest.  The id serves
 to correlate the progress of distinct asynchronous operations that share
 the same category and name within the same process.

 0 to 15 arguments can be associated with the event, each of which is used
 to annotate the asynchronous operation with additional information.  The
 arguments provided to matching async begin, async instant, and async end
 events are combined together in the trace; it is not necessary to repeat them.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |async_id| is the correlation id of the asynchronous operation.
            Must be unique for a given process, category, and name combination.
 |args| is the list of argument key/value pairs.

 Usage:

     trace_async_id_t async_id = 555;
     TRACE_ASYNC_INSTANT("category", "name", async_id, "x", TA_INT32(42));

```

## TRACE_ASYNC_END {#TRACE_ASYNC_END}

```
 Writes an asynchronous end event with the specified id.

 Asynchronous events describe work that is happening asynchronously and that
 may span multiple threads.  Asynchronous events do not nest.  The id serves
 to correlate the progress of distinct asynchronous operations that share
 the same category and name within the same process.

 0 to 15 arguments can be associated with the event, each of which is used
 to annotate the asynchronous operation with additional information.  The
 arguments provided to matching async begin, async instant, and async end
 events are combined together in the trace; it is not necessary to repeat them.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |async_id| is the correlation id of the asynchronous operation.
            Must be unique for a given process, category, and name combination.
 |args| is the list of argument key/value pairs.

 Usage:

     trace_async_id_t async_id = 555;
     TRACE_ASYNC_END("category", "name", async_id, "x", TA_INT32(42));

```

## TRACE_FLOW_BEGIN {#TRACE_FLOW_BEGIN}

```
 Writes a flow begin event with the specified id.
 This event may be followed by flow steps events and must be matched by
 a flow end event with the same category, name, and id.

 Flow events describe control flow handoffs between threads or across processes.
 They are typically represented as arrows in a visualizer.  Flow arrows are
 from the end of the duration event that encloses the beginning of the flow
 to the beginning of the duration event that encloses the next step or the
 end of the flow.  The id serves to correlate flows that share the same
 category and name across processes.

 This event must be enclosed in a duration event that represents where
 the flow handoff occurs.

 0 to 15 arguments can be associated with the event, each of which is used
 to annotate the flow with additional information.  The arguments provided
 to matching flow begin, flow step, and flow end events are combined together
 in the trace; it is not necessary to repeat them.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |flow_id| is the correlation id of the flow.
           Must be unique for a given category and name combination.
 |args| is the list of argument key/value pairs.

 Usage:

     trace_flow_id_t flow_id = 555;
     TRACE_FLOW_BEGIN("category", "name", flow_id, "x", TA_INT32(42));

```

## TRACE_FLOW_STEP {#TRACE_FLOW_STEP}

```
 Writes a flow step event with the specified id.

 Flow events describe control flow handoffs between threads or across processes.
 They are typically represented as arrows in a visualizer.  Flow arrows are
 from the end of the duration event that encloses the beginning of the flow
 to the beginning of the duration event that encloses the next step or the
 end of the flow.  The id serves to correlate flows that share the same
 category and name across processes.

 This event must be enclosed in a duration event that represents where
 the flow handoff occurs.

 0 to 15 arguments can be associated with the event, each of which is used
 to annotate the flow with additional information.  The arguments provided
 to matching flow begin, flow step, and flow end events are combined together
 in the trace; it is not necessary to repeat them.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |flow_id| is the correlation id of the flow.
           Must be unique for a given category and name combination.
 |args| is the list of argument key/value pairs.

 Usage:

     trace_flow_id_t flow_id = 555;
     TRACE_FLOW_STEP("category", "name", flow_id, "x", TA_INT32(42));

```

## TRACE_FLOW_END {#TRACE_FLOW_END}

```
 Writes a flow end event with the specified id.

 Flow events describe control flow handoffs between threads or across processes.
 They are typically represented as arrows in a visualizer.  Flow arrows are
 from the end of the duration event that encloses the beginning of the flow
 to the beginning of the duration event that encloses the next step or the
 end of the flow.  The id serves to correlate flows that share the same
 category and name across processes.

 This event must be enclosed in a duration event that represents where
 the flow handoff occurs.

 0 to 15 arguments can be associated with the event, each of which is used
 to annotate the flow with additional information.  The arguments provided
 to matching flow begin, flow step, and flow end events are combined together
 in the trace; it is not necessary to repeat them.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |flow_id| is the correlation id of the flow.
           Must be unique for a given category and name combination.
 |args| is the list of argument key/value pairs.

 Usage:

     trace_flow_id_t id = 555;
     TRACE_FLOW_END("category", "name", flow_id, "x", TA_INT32(42));

```

## TRACE_BLOB_EVENT {#TRACE_BLOB_EVENT}

```
 Writes a large blob record with the given blob data and metadata.
 Here metadata includes timestamp, thread and process information, and arguments,
 which is what most event records contain.

 Blobs that exceed |TRACE_ENCODED_RECORD_MAX_TOTAL_LENGTH| will be silently
 ignored, as will blobs that cannot fit within the remaining space in the
 trace buffer.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |blob| is a pointer to the data.
 |blob_size| is the size, in bytes, of the data.
```

## TRACE_BLOB_ATTACHMENT {#TRACE_BLOB_ATTACHMENT}

```
 Writes a large blob record with the given blob data, with only a
 category and name associated with the blob. This will not contain much
 additional metadata. This means timestamp, thread and process information,
 and arguments are not included with the record.

 Blobs that exceed |TRACE_ENCODED_RECORD_MAX_TOTAL_LENGTH| will be silently
 ignored, as will blobs that cannot fit within the remaining space in the
 trace buffer.

 |category_literal| and |name_literal| must be null-terminated static string constants.
 |blob| is a pointer to the data.
 |blob_size| is the size, in bytes, of the data.
```

## TRACE_KERNEL_OBJECT {#TRACE_KERNEL_OBJECT}

```
 Writes a description of a kernel object indicated by |handle|,
 including its koid, name, and the supplied arguments.

 0 to 15 arguments can be associated with the record, each of which is used
 to annotate the handle with additional information.

 |handle| is the handle of the object being described.
 |args| is the list of argument key/value pairs.

 Usage:

     zx_handle_t handle = ...;
     TRACE_KERNEL_OBJECT(handle, "description", TA_STRING("some object"));

```

<!-- # Commented out for the time being
## TRACE_INSTANT {#trace_instant}

The `TRACE_INSTANT` macro records a trace with a single timestamp that is
scoped to a process, thread, or global.

### Arguments

```
TRACE_INSTANT("category", "name", SCOPE, "key", value);
```

This macro accepts the following arguments:

<table>
  <tr>
    <th>Name</th>
    <th>Description</th>
    <th>Valid values</th>
  </tr>
  <tr>
    <td><code>category</code></td>
    <td><p><b>Required</b></p>
    <p>Specifies the category of the trace event.</p>
    <p>For example, a category could be: <code>demo</code></p>
      <p>The category namespace is global to all components that are running,
      so if there is another component that also uses <code>demo</code>, you may see trace
      data from another trace provider.</p>
    </td>
    <td>Null terminated string.</td>
  </tr>
  <tr>
    <td><code>name</code></td>
    <td><p><b>Required</b></p>
    <p>Specifies a description of the event data.</p>
      <p>In many cases, you may use the same categories for various trace points,
      but you can use the <code>name</code> to distinguish these various trace
      points.</p>
    </td>
    <td>Null terminated string.</td>
  </tr>
  <tr>
    <td><code>SCOPE</code></td>
    <td><p><b>Required</b></p>
    <p>Specifies if the trace is scoped to a process, thread, or globally
      scoped.</p>
    </td>
    <td><p>Valid values are:</p>
        <p><code>TRACE_SCOPE_PROCESS</code></p>
        <p>The event is only relevant to
        the process in which it occurred.</p>
        <p><code>TRACE_SCOPE_THREAD</code></p>
        <p>The event is only relevant to
        the thread it occurred on.</p>
        <p><code>TRACE_SCOPE_GLOBAL</code></p>
        <p>The event is globally relevant.</p>
    </td>
  </tr>
  <tr>
    <td><code>key</code></td>
    <td><p><b>Optional</b></p>
    <p>The number of keys and values must match as they are used in pairs.</p>
    <p>Specifies a key for the <code>value</code> that your trace provider
    records.</p>
    </td>
    <td>Null terminated string.</td>
  </tr>
  <tr>
    <td><code>value</code></td>
    <td><p><b>Optional</b></p>
    <p>The number of keys and values must match as they are used in pairs.</p>
    <p>Specifies a value for the respective <code>key</code> of the event data.</p>
    </td>
    <td><p>This value may be <code>NULL</code>.</p>
    <p>The values that you specify must be properly encoded values, see
    <a href="#encoding-macros">Encoding macros</a>.</td>
  </tr>
</table>

### Examples

#### No key and value pairs

In some cases, you may need to record trace data, but do not need to record
any key and value pairs:

```
TRACE_INSTANT("helloworld", "hello_world_test", TRACE_SCOPE_PROCESS);
```

#### Single key and value pair

In some cases, you may want to only record trace data for a single key and value pair:

```
TRACE_INSTANT("helloworld", "hello_world_test", TRACE_SCOPE_PROCESS, "message", TA_STRING("Hello, World!"));
```

#### Multiple key and value pairs

In some cases, you may want to only record trace data for a multiple key and value pairs:

```
TRACE_INSTANT("helloworld", "hello_world_test", TRACE_SCOPE_PROCESS, "message1", TA_STRING("First: Hello, World!"), "message2", TA_STRING("Second: Hello, World!"), "message3", TA_STRING("Third: Hello, World!"));
```

-->
