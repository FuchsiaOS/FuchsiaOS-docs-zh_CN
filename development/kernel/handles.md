# <!-- Object usage -->

# 对象的使用

<!-- Processes create and use kernel objects to perform work. Just as memory can
be leaked or misused (e.g use-after-free), handles to kernel object can
be leaked or misused (e.g use-after-close). -->

进程创建并使用内核对象用于完成工作。内存可以泄漏或误用（例如，释放后使用），处理内核对象可以泄漏或误用（例如，关闭后使用）。

## <!-- Handles tool -->

## 句柄工具

<!-- To help developers diagnose handle issues use the `handles` tool, below
is a sample of process 29831 which is wlancfg.cmx : -->

为了帮助开发者诊断句柄问题可使用 `handles` 工具，下面是进程 29831（wlancfg.cmx ）的简单使用：

```
$ handles 29831
    handle   koid rkoid     rights type
0xa8d44a0f: 29973       0x0000d0ef vmo
0xa8e44aab: 29847 29846 0x0000f00e channel
0xa8d44a0b: 29972       0x0000d0ef vmo
0xa8e42413:  9931  9930 0x0000f00e channel
0xa8d44a07: 29971       0x0000d0ef vmo
0xa8f44a1f: 29969 29970 0x0000f00e channel
0xa8a44a3b: 29964       0x0000d0ef vmo
0xa8d44a17: 29962 29963 0x0000f00e channel
0xa8844a43: 29961       0x0000d0ef vmo
0xa8f44a4b: 29960       0x0000d0ef vmo
0xa8e44a3f: 29959       0x0000d0ef vmo
0xa8e44a23: 29958       0x0000800f port
0xa8f44a2f: 29957       0x0000d0ef vmo
0xa8644a53: 29911       0x0000d0ef vmo
0xa8a44a7f: 29908       0x0000d0ef vmo
0xa8844a6b: 29907       0x0000d0ef vmo
0xa8f44a63: 29906       0x0000d0ef vmo
0xa8844a6f: 29905       0x0000d0ef vmo
0xa8f44a8b: 29904       0x0000d0ef vmo
0xa8944a9f: 29903       0x0000d0ef vmo
0xa8444a83: 29900       0x0000800f vmar
0xa8e44a77: 29845       0x0000d0ef vmo
0xa8f44a8f:  1034       0x0000d0f7 vmo
0xa8d44aa3:  1129       0x0000d00b log
0xa8d44abf:  1129       0x0000d00b log
0xa8d44abb:  1129       0x0000d00b log
0xa8644aef: 29827 29828 0x0000f00e channel
0xa8844ac3: 29826  8711 0x0007dfcf job
0xa8144afb: 29825 29824 0x0000f00e channel
0xa8e44adb: 29816 29817 0x0000f00e channel
0xa8e44ad3: 29776 29777 0x0000f00e channel
0xa894496b: 29766 29767 0x0000f00e channel
0xa8d44a97: 29833 29831 0x0004d2cf thread
0xa8d44a93: 29832       0x0000801f vmar
0xa8d44aaf: 29831 29826 0x0006d3cf process
0xa8f44a73: 29850       0x0000d00b log
0xa8f44af3: 29768 29769 0x0000f00e channel
0xa8e44aa7: 29834 29835 0x0000f00e channel
38 handles
```

<!-- The `handles <pid>` tool dumps the process handle table, which holds all
accessible handles for that particular process at the moment of invocation. -->

`handles <pid>` 工具打印出来进程句柄表，在指定的进程中引用瞬间列出了全部可访问的句柄。

<!-- For each handle the tool prints the handle value, the koid of the object it
points to, the related koid (rkoid) if the object has a related object, the
rights of the handle and the type of object. -->

对于每个句柄工具打印出句柄的值，权限和对象类型，对象的 koid。如果有相关的对象会打印出 rkoid。

<!-- In the example above, it shows 38 unique handles, which map to 36 unique
objects; 3 of the handles point to the same "log" object with koid 1129. -->

在上面的例子中，显示了38个唯一的句柄，其中映射36个唯一对象。3个句柄指向相同的 1129「log」对象。

<!-- It should be noted that not all alive objects might be displayed by the tool.
For example, a thread can be alive even if there are not handles open to it and
VMOs can be held alive by the associated VMAR.-->

注意，并不是全部的存活的对象都可以通过工具显示出来。例如，一个存活的线程即使没有句柄打开它，VMOs 也可以通过关联的 VMAR 保持存活

<!-- The `handles` tool supports filtering and reverse filtering by object type; use
`handles --help` to see all the options. -->

`handles` 工具通过对象类型支持过滤和排除过滤。执行`handles --help` 可以看到全部的选项详情。

## <!-- Handles in the debugger -->

## 调试句柄

<!-- You can view handle information using the [debugger](/docs/development/debugger/kernel_objects.md).
To do this, attach to the process in question and run the `handles` command. This shows the handle
value, object type, and object koid:-->

你可以使用 [调试器](/development/debugger/kernel_objects.md) 查看句柄信息。获取进程并运行 `handles` 命令，它就显示句柄值，对象类型和对象koid：

```
[zxdb] handles
  504103211  ZX_OBJ_TYPE_VMO        27851
  504103271  ZX_OBJ_TYPE_VMO        27719
  505151859  ZX_OBJ_TYPE_VMO        27720
  505151867  ZX_OBJ_TYPE_VMO        27718
  506200511  ZX_OBJ_TYPE_PORT       27976
  507249163  ZX_OBJ_TYPE_VMAR       27716
  508297363  ZX_OBJ_TYPE_VMO        28200
  508297379  ZX_OBJ_TYPE_VMO        28187
  508297387  ZX_OBJ_TYPE_SOCKET     28189
  508297731  ZX_OBJ_TYPE_CLOCK       1263
  508297735  ZX_OBJ_TYPE_LOG         1275
  508297755  ZX_OBJ_TYPE_LOG         1275
```

<!-- You can also view basic information about a handle by calling `handle` and specifying a handle
value:-->

你也可以通过调用 `handle` 指定的句柄值来查看关于句柄的基本信息：

```
[zxdb] handle 508302371
          Type  ZX_OBJ_TYPE_CHANNEL
         Value  508302371
        Rights  ZX_RIGHT_TRANSFER
                ZX_RIGHT_READ
                ZX_RIGHT_WRITE
                ZX_RIGHT_SIGNAL
                ZX_RIGHT_SIGNAL_PEER
                ZX_RIGHT_WAIT
                ZX_RIGHT_INSPECT
          Koid  31062
  Related koid  31061

```

<!-- If the object referenced by the handle is related to another object (such as the other end of a
channel, or the parent of a job) then `related_koid` is the koid of that other object. If there is
no other related object, this value is zero. In this example, the related koid is the other end of
the channel. This relationship is immutable: an object's `related_koid` does not change even if the
related object no longer exists.-->

如果句柄引用的对象是关联到另一个对象（比如，管道的尾部或者父作业），`related_koid` 是其他对象的koid，如果这没有相关的其他对象，该值是零。在这个例子中，`related_koid` 是另一个管道尾部。这个关系是不可变的：一个对象的 `related_koid` 不能改变甚至如果关联对象不再存在了。

## <!-- Bad handle policy -->

### 糟糕的句柄策略

<!-- Using a handle after it has been [closed](/docs/reference/syscalls/handle_close.md)
or closing a handle that has been already closed are mistakes that can create
hard to diagnose errors.-->

在 [关闭](/reference/syscalls/handle_close.md) 后使用句柄或者已关闭的句柄再次关闭它会出现糟糕的诊断错误。

<!-- In order to help developers find these issues, the "bad handle" Job policy can
be activated using [zx_job_set_policy](/docs/reference/syscalls/job_set_policy.md)
with the condition **ZX_POL_BAD_HANDLE** and the action
**ZX_POL_ACTION_ALLOW_EXCEPTION**. When a process is launched under a job with
this policy, any use of an already closed handle will generate an exception
that if not handled will terminate the process and log the offending call stack
or that can be trapped by the [debugger](/docs/development/idk/documentation/debugger.md)
for interactive troubleshooting.-->

为了帮助开发者找到问题，「糟糕的句柄」作业策略可以通过 **ZX_POL_BAD_HANDLE** 和 **ZX_POL_ACTION_ALLOW_EXCEPTION** 来激活使用 [Zircon作业设置策略](/reference/syscalls/job_set_policy.md)。 当一个进程运行在这个策略的作业下，任何使用已经关闭的句柄会生成异常，如果没有句柄中断进程并记录调用栈或者通过[调试器](/development/idk/documentation/debugger.md)收集。

