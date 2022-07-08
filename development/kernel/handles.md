# Object usage

Processes create and use kernel objects to perform work. Just as memory can
be leaked or misused (e.g use-after-free), handles to kernel object can
be leaked or misused (e.g use-after-close).

## Handles tool

To help developers diagnose handle issues use the `handles` tool, below
is a sample of process 29831 which is wlancfg.cmx :

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

The `handles <pid>` tool dumps the process handle table, which holds all
accessible handles for that particular process at the moment of invocation.

For each handle the tool prints the handle value, the koid of the object it
points to, the related koid (rkoid) if the object has a related object, the
rights of the handle and the type of object.

In the example above, it shows 38 unique handles, which map to 36 unique
objects; 3 of the handles point to the same "log" object with koid 1129.

It should be noted that not all alive objects might be displayed by the tool.
For example, a thread can be alive even if there are not handles open to it and
VMOs can be held alive by the associated VMAR.

The `handles` tool supports filtering and reverse filtering by object type; use
`handles --help` to see all the options.

## Handles in the debugger

You can view handle information using the [debugger](/development/debugger/kernel_objects.md).
To do this, attach to the process in question and run the `handles` command. This shows the handle
value, object type, and object koid:

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

You can also view basic information about a handle by calling `handle` and specifying a handle
value:

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

If the object referenced by the handle is related to another object (such as the other end of a
channel, or the parent of a job) then `related_koid` is the koid of that other object. If there is
no other related object, this value is zero. In this example, the related koid is the other end of
the channel. This relationship is immutable: an object's `related_koid` does not change even if the
related object no longer exists.

## Bad handle policy

Using a handle after it has been [closed](/reference/syscalls/handle_close.md)
or closing a handle that has been already closed are mistakes that can create
hard to diagnose errors.

In order to help developers find these issues, the "bad handle" Job policy can
be activated using [zx_job_set_policy](/reference/syscalls/job_set_policy.md)
with the condition **ZX_POL_BAD_HANDLE** and the action
**ZX_POL_ACTION_ALLOW_EXCEPTION**. When a process is launched under a job with
this policy, any use of an already closed handle will generate an exception
that if not handled will terminate the process and log the offending call stack
or that can be trapped by the [debugger](/development/idk/documentation/debugger.md)
for interactive troubleshooting.

