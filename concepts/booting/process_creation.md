# Process Creation

The kernel provides low-level facilities for creating and setting up processes.
However, these facilities are difficult to use because they involve directly
mapping memory for executables, shared libraries, and stacks. Instead, you should
use one of the higher-level mechanisms for creating processes.

## fuchsia.process.Launcher

Fuchsia provides a service, `fuchsia.process.Launcher`, that does the low-level
work of constructing processes for you. You provide this service with all the
kernel objects needed to construct the process (e.g., the job object in which
the process should be created, the executable image, and the standard input and
output handles), and the service does the work of parsing the ELF executable
format, configuring the address space for the process, and sending the process
the startup message.

Most clients do not need to use this service directly. Instead, most clients can
use the simple C frontend in the FDIO library called `fdio_spawn`. This
function, and its more advanced `fdio_spawn_etc` and `fdio_spawn_vmo`
companions, connect to the `fuchsia.process.Launcher` service and send the
service the appropriate messages to create the process.  The
`fdio_spawn_action_t` array passed to `fdio_spawn_etc` can be used to customize
the created process.

Regardless of whether you use the `fuchsia.process.Launcher` service directly
or the `fdio_spawn` frontend, this approach to creating processes is most
appropriate for creating processes within your own namespace because you need
to supply all the kernel objects for the new process.

## fuchsia.sys.Launcher

To create a process in its own namespace, Fuchsia provides the
`fuchsia.sys.Launcher` service. Rather than providing this process all the
kernel objects needed to construct the new process, you simply provide the
service a high-level description of the process you wish to create and the
`fuchsia.sys.Launcher` implementation supplies the new process with the
appropriate kernel objects. For example, if you provide the URL of a component
within a package, `fuchsia.sys.Launcher` will create a process for that
component in a namespace appropriate for that component with access to its own
package and whatever other resources are declared in the `sandbox` section of
its manifest.

Rather than returning a `zx::process` handle directly, `fuchsia.sys.Launcher`
returns a `fuchsia.sys.ComponentController` interface. This layer of
abstraction lets `fuchsia.sys.Launcher` create components that are not backed
by individual processes. For example, if you launch a component written in
Dart, the component might run in an instance of the Dart VM that is shared
between a number of components with compatible security constraints.

## Early boot

Early on in the boot process, the system does create a number of processes
manually. For example, the kernel manually creates the first userspace process,
`userboot`, which creates `devmgr` in turn. These low-level mechanisms use the
`liblaunchpad.so` shared library, which contains the logic for parsing ELF
files. Direct construction of processes is prohibited in the `fuchsia` job tree
using a job policy.

The `liblaunchpad.so` shared library is available in Zircon but should be used
only during early boot and for low-level tests of process creation. Libraries
or programs that might be used from the `fuchsia` job tree should use
`fdio_spawn` (or its companions) to conform to the security policy.
