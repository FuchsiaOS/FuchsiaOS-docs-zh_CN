<!--
# Zircon System Interface Rubric

The Zircon system interface is expressed as the `libzircon.so` vDSO API surface.

Functions that are part of the interface must have names that start with `zx_`
and preprocessor macros must have names that start with `ZX_`.  Types defined as
part of the interface must have names that begin with `zx_` and end with `_t`.

Every function that is part of the interface must be documented with a markdown
file in https://fuchsia.googlesource.com/zircon/+/master/docs/syscalls/ and
linked from https://fuchsia.googlesource.com/zircon/+/master/docs/syscalls.md .
-->

# Zircon系统接口规范

Zircon系统接口代表“libzrone.so” vDSO API。

函数作为接口的一部分必须以"zx\_"开头, 预处理器宏的名称必须以“ZX\_”开头。 接口的类型定义部分必须以“zx\_”开头，以“\_t”结尾。

接口的所有函数都必须用Markdown记录到文档中。
文档位于<https://fuchsia.googlesource.com/zrone/+/master/docs/syscalls/>
和<https://fuchsia.googlesource.com/zrone/+/master/docs/syscalls.md。>

<!--
## Function Names

Functions must have names consisting entirely of lowercase letters and
underscores and that conform to the following grammar:

```
zx_<noun>_<verb>{_<direct-object>}
```

For example:

```
zx_handle_close, zx_channel_write, zx_object_signal_peer
```

Typically, the noun is a kernel object type but can be other nouns, such as
`clock` or `ticks` for which there is no corresponding kernel object. Other
functions use more abstract nouns, such as `system` or `status`.

The nouns and verbs must not contain underscores (to avoid confusing the
grammar). The noun and verb should each be single English words but acronyms (or
abbreviations) may be used if there is no suitable word or the word is too long.

The direct object may contain underscores.

Some functions perform composite operations. In such cases, the function may be
named by concatenating the names of the component operations.

Some functions operate on several types of kernel object, in which case the noun
is a more abstract object type. For example, functions with the noun `object`
operate on most kernel objects and functions with the noun `task` operate on
jobs, processes, and threads.
-->

## 函数名

函数名称必须完全由小写字母和下划线组成并符合以下要求：

```
zx_<noun>_<verb>{_<direct-object>}
```

例如:

```
zx_handle_close，zx_channel_write，zx_object_signal_peer
```

通常，核心对象类型的名字是名词也可以是其他的名词组，例如'clock'或'ticks'这种名词，没有相关的核心对象类型关联。其他
函数使用更抽象的名词，如“system”或“status”。

名词和动词不得含有下划线（以避免混淆语法）。名词和动词都应该是单独的英语单词，如果没有合适的单词或单词太长，则可以使用首字母缩写（或单词缩写）。

直接对象可以包含下划线。

如果函数有复合操作的情况，函数名可以把操作名字串联起来命名。

有些函数对几种类型的内核对象进行操作，在这种情况下，对象类型的名词更加抽象。例如，使用'object'的函数操作着大多的内核对象， 而用'task'的函数操作任务、进程和线程。

<!--
## Types

Use `zx_status_t` to represent success and failure.

Use fixed-size integer types. Functions must not use `short`, `int`, or
`unsigned long` (or similar types). Instead, use types such as `int16_t`,
`int32_t`, and `uint64_t`.

Use `size_t` for buffer lengths, element sizes, and element counts.

Use `void*` for pointers to arbitrary types in the caller's address space. Use
`zx_vaddr_t` / `zx_paddr_t` for addresses that might be in other address spaces.

Use `zx_time_t` for timeouts, which must be expressed as absolute deadlines in
nanoseconds in the `ZX_CLOCK_MONOTONIC` timebase. In scenarios were absolute
deadlines do not make sense (for example, timer slack), use `zx_duration_t` to
represent an amount of time in nanoseconds with no specific timebase.
-->

## 类型

使用“zx_status”表示成功和失败。

使用固定大小的整数类型。函数不能使用“short”、“int”或`无符号long`（或类似类型）。相反，请使用“int16”等类型，`int32和uint64。

对缓冲区长度、元素大小和元素计数使用“size”。

对调用者地址空间中任意类型的指针使用“void*”。使用`zx_vaddr_t`/`zx_paddr_t`用于可能位于其他地址空间中的地址。

使用“zx\_time\_t”表示超时，基于纳秒级别的绝对超时必须使用"ZX\_CLOCK\_MONOTONIC"。 在场景中绝对的截止时间是没有意义的（例如，计时器延迟），请使用“zx\_duration”来表示没有特定时间单位的时间量（以纳秒为单位）。

<!--
## Parameters
-->
## 参数

<!--
### Receiver

The vast majority of functions act on a handle, which is a reference to a kernel
object of a type matching the *noun* in the function name. This handle is the
first argument to such functions and is referred to as the receiver.

Use the name `handle` for the receiver.

Object creation functions (eg, `zx_channel_create`, `zx_event_create`) may not
take a handle argument. These functions implicitly operate on the current
process.
-->
### 接收者

绝大多数函数作用于句柄，句柄是对内核的引用与函数名中的*名词*匹配的类型的对象。这个句柄是此类函数的第一个参数，称为接收者。

使用收件者的名称“handle”。

对象创建函数（例如，“zx\_channel\_create”、“zx\_event\_create”）不能带有句柄参数。这些函数在当前进程中做隐式操作。

<!--
### Options Parameter

Often functions include an `options` parameter to allow for flags which affect
the operation, and include room for further flags being added to future
revisions of the API.

Use the type `uint32_t` and the name `options` for the `options` parameter.

When present, an `options` parameter must be the first argument after the
receiver handle or the first argument overall if the function does not have a
receiver.

An `options` parameter is not required for all functions.

Individual option values must be defined as preprocessor macros that cast a
numeric literal to `uint32_t`. The options must be bit flags that can be
combined using the bitwise `|` operator.
-->

### 选项参数

函数通常包含一个“options”参数，以允许影响操作，并为将来添加更多标志留出空间API的修订版。

使用类型“uint32”和名称“options”作为“options”参数。

出现时，“options”参数必须接受句柄的第一个参数，如果函数没有接收者必须是所有参数的第一个。

并非所有函数都需要“options”参数。

单个选项值必须定义为“uint32”的变量类型。选项必须是位标志，可以是使用按位`|`运算符组合。


<!--
### Handles

When a function is given a handle as a parameter, the function must either
always consume the handle or never consume, with the following exceptions:

 * If the function takes an `options` parameter, the function may have a
   non-default option to avoid consuming handles in various error conditions.

 * If the function does not take an `options` parameter, the function may avoid
   consuming handles if/when it returns `ZX_ERR_SHOULD_WAIT`.
-->

### 句柄

当给函数一个句柄作为参数时，该函数必须始终使用句柄或从不使用，但以下情况除外：

- 如果函数采用“options”参数，则该函数可能具有避免在各种错误情况下使用句柄的非默认选项。

- 如果函数不采用“options”参数，则该函数可以避免使用句柄如果/当它返回“ZX_ERR_SHOULD_WAIT”时。

<!--
### Buffers with Data, Count/Size, and/or Actual

Always accompany arrays or buffers with a count or size (of type `size_t`),
including strings. If the buffer is written by the function, the function must
have an out parameter that returns the count or size of the data written.

For read and write style operations, the pointer(s) to the buffer(s) are
followed by the buffer count(s) or size(s), and if a short read or write is
possible, an out parameter provides the actual count(s) or size(s) on success:

```
zx_status_t zx_socket_write(zx_handle_t handle, uint32_t options,
                            const void* buffer, size_t size, size_t* actual);
```

When there are multiple buffers, the buffers, lengths, and out parameters appear
interleaved in a consistent order. For example, see `zx_channel_read`:

```
zx_status_t zx_channel_read(zx_handle_t handle, uint32_t options,
                            void* bytes, zx_handle_t* handles,
                            uint32_t num_bytes, uint32_t num_handles,
                            uint32_t* actual_bytes, uint32_t* actual_handles);
```
-->

### 缓冲区的数据、计数/大小和以及实际值

数组或缓冲区始终伴随着计数或大小（类型为“size_t”）包括字符串。如果缓冲区是由函数写入的，则函数必须
有一个out参数，返回所写数据的计数或大小。

对于读写式操作，指向缓冲区的指针是后跟缓冲区计数或大小，如果短读或短写是可能，out参数提供成功时的实际计数或大小：

```
zx_status_t zx_socket_write（zx_handle_t handle，uint32_t options，const void*buffer，size_t size，size_t *actual);
```

当存在多个缓冲区时，将显示缓冲区、长度和输出参数以一致的顺序交错。例如，请见“zx_channel_read”：

```
zx_status_t zx_channel_read（zx_handle_t handle，uint32_t optiona，void*bytes
zx_handle_t* handles，uint32_t num_bytes，uint32_t num_handles，uint32_t* actual_bytes，uint32_t* actual_handles）；
```

<!--
### Outputs

An out parameter is a scalar value written by the function. For example, a
function that returns the number of CPUs by writing to a `uint32_t` has an out
parameter. If the function populates a buffer provided by the client, the buffer
isn’t an out parameter.

Out parameters always come at the end of the parameter list.

An out parameter must not also be an in parameter. For example, if a function
has an out parameter through which it returns the number of bytes written to a
buffer, that parameter must not also be used by the function to receive the
length of the buffer from the caller.
-->

### 输出

输出参数是由函数写入的标量值。例如，一个通过写入“uint32”返回CPU数量的函数具有输出参数。如果函数填充客户端提供的缓冲区，则不是输出参数。

输出参数总是出现在参数列表的末尾。

输出参数不能也是输入参数。例如，如果一个函数有一个输出参数，它通过该参数返回写入缓冲区，该参数不能也被函数用来接收调用者缓冲区的长度。

<!--
## Return Type

The vast majority of functions have a return type of `zx_status_t`, which is
`ZX_OK` on success and `ZX_ERR_...` on failure.

Do not return other values through `zx_status_t`, for example using the
positive value range. Instead, use an out parameter.

Other return types may be used for functions that cannot fail. For example,
`zx_thread_exit` never fails to exit the thread and has a return type of void.
Similarly, `zx_clock_get` cannot fail to get the current time and has a return
type of `zx_time_t`.
-->

##返回类型

绝大多数函数的返回类型为“zx\_status\_t”，即`成功时“ZX\_OK”，失败时“ZX\_ERR…”。

不要通过“zx\_status\_t”返回其他值，例如使用正值范围。而是使用输出参数。

其他返回类型可用于不能失败的函数。例如，`zx_thread_exit`从不用在线程中的失败或退出，返回类型为void。类似地，“zx\_clock\_get”也不能失败的获取当前时间并返回“zx\_time\_t”的类型。

<!--
## Function-specific rules

### zx_object_get_property versus zx_object_get_info

There are two similar mechanisms for exposing data about objects:
`zx_object_get_property` and `zx_object_get_info`. Prefer exposing data through
`zx_object_get_property` if (a) the property can be set using
`zx_object_set_property` or (b) the property exist across multiple types of
objects. In other case, consider including the data in the general
`zx_object_get_info` topic for the type of object that has the property.
-->
## 特定函数的规则

### zx\_object\_get\_property与zx\_object\_get\_info

公开对象数据有两个函数：
`zx_object_get_property`和`zx_object_get_info`。如果`zx_object_get_property`可以使用，则更倾向于选它。`zx_object_set_property`或（b）该属性存在多种类型的对象，在其他情况下，考虑将数据包含在具有属性的对象类型`zx_object_get_info`。
