# Rights

## Basics

Rights are associated with handles and convey privileges to perform actions on
either the associated handle or the object associated with the handle.

The [`<zircon/rights.h>`](/zircon/system/public/zircon/rights.h) header defines
default rights for each object type, which can be reduced via
`zx_handle_replace()` or `zx_handle_duplicate()`.

| Right | Conferred Privileges |
| ----- | -------------------- |
| **ZX_RIGHT_DUPLICATE**      | Allows handle duplication via [`zx_handle_duplicate()`] |
| **ZX_RIGHT_TRANSFER**       | Allows handle transfer via [`zx_channel_write()`] |
| **ZX_RIGHT_READ**           | Allows reading of data from containers (channels, sockets, VM objects, etc.) |
|                             | Allows mapping as readable if **ZX_RIGHT_MAP** is also present |
|                             | **TO BE REMOVED** Allows inspection of object state |
| **ZX_RIGHT_WRITE**          | Allows writing of data to containers (channels, sockets, VM objects, etc.) |
|                             | Allows mapping as writeable if **ZX_RIGHT_MAP** is also present |
|                             | **TO BE REMOVED** Allows modification of object state |
| **ZX_RIGHT_EXECUTE**        | Allows mapping as executable if **ZX_RIGHT_MAP** is also present |
| **ZX_RIGHT_MAP**            | Allows mapping of a VM object into an address space. |
| **ZX_RIGHT_GET_PROPERTY**   | Allows property inspection via [`zx_object_get_property()`] |
| **ZX_RIGHT_SET_PROPERTY**   | Allows property modification via [`zx_object_set_property()`] |
| **ZX_RIGHT_ENUMERATE**      | Allows enumerating child objects via [`zx_object_get_info()`] and [`zx_object_get_child()`] |
| **ZX_RIGHT_DESTROY**        | Allows termination of task objects via [`zx_task_kill()`] |
| **ZX_RIGHT_SET_POLICY**     | Allows policy modification via [`zx_job_set_policy()`] |
| **ZX_RIGHT_GET_POLICY**     | Allows policy inspection |
| **ZX_RIGHT_SIGNAL**         | Allows use of [`zx_object_signal()`] |
| **ZX_RIGHT_SIGNAL_PEER**    | Allows use of [`zx_object_signal_peer()`] |
| **ZX_RIGHT_WAIT**           | Allows use of [`zx_object_wait_one()`], [`zx_object_wait_many()`], and other waiting primitives |
| **ZX_RIGHT_INSPECT**        | Allows inspection via [`zx_object_get_info()`] |
| **ZX_RIGHT_MANAGE_JOB**     | **NOT YET IMPLEMENTED** Allows creation of processes, subjobs, etc. |
| **ZX_RIGHT_MANAGE_PROCESS** | **NOT YET IMPLEMENTED** Allows creation of threads, etc. |
| **ZX_RIGHT_MANAGE_THREAD**  | **NOT YET IMPLEMENTED** Allows suspending/resuming threads, etc. |
| **ZX_RIGHT_APPLY_PROFILE**  | Allows a scheduling profile to be applied to a thread via [`zx_object_set_profile()`] |
| **ZX_RIGHT_MANAGE_SOCKET**  | Allows changing socket disposition via [`zx_socket_set_disposition()`] |

## ZX_RIGHTS_BASIC

The basic rights allow primitive manipulation of handles and are common to the
majority of handle types by default. These are **ZX_RIGHT_DUPLICATE**,
**ZX_RIGHT_TRANSFER**, **ZX_RIGHT_WAIT**, and **ZX_RIGHT_INSPECT**.

These four rights are referred to as **ZX_RIGHTS_BASIC** when used together.

## See also
[Objects](reference/kernel_objects/objects.md),
[Handles](concepts/kernel/handles.md)

[`zx_channel_write()`]: reference/syscalls/channel_write.md
[`zx_handle_duplicate()`]: reference/syscalls/handle_duplicate.md
[`zx_job_get_policy()`]: reference/syscalls/job_get_policy.md
[`zx_job_set_policy()`]: reference/syscalls/job_set_policy.md
[`zx_object_get_child()`]: reference/syscalls/object_get_child.md
[`zx_object_get_info()`]: reference/syscalls/object_get_info.md
[`zx_object_get_property()`]: reference/syscalls/object_get_property.md
[`zx_object_set_profile()`]: reference/syscalls/object_set_profile.md
[`zx_object_set_property()`]: reference/syscalls/object_set_property.md
[`zx_object_signal()`]: reference/syscalls/object_signal.md
[`zx_object_signal_peer()`]: reference/syscalls/object_signal_peer.md
[`zx_object_wait_many()`]: reference/syscalls/object_wait_many.md
[`zx_object_wait_one()`]: reference/syscalls/object_wait_one.md
[`zx_socket_set_disposition()`]: reference/syscalls/socket_set_disposition.md
[`zx_task_kill()`]: reference/syscalls/task_kill.md
