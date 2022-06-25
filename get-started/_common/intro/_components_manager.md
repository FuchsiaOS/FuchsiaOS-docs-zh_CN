## Component manager

The heart of the component framework is the **component manager**. It is
responsible for coordinating the execution of all component instances,
providing them with their capabilities, and intermediating connections between
components.

Components can be launched explicitly (from a URL, for example) or implicitly
from a request for a particular capability. Component manager performs the
necessary resolution to determine whether to launch a new component or route
the request to an existing instance. For this routing to take place, every
component must declare any capabilities that it **provides** to the system
and any it **consumes**.


<aside class="key-point">
  <b>Does each component run in its own process?</b>
  <p>Zircon defines the common kernel objects for runnable code, such as
  processes. However, component instances do not always correlate directly
  with a single process. Often the policy for how these processes are used
  is defined by the runner. For example, the
  <a href="concepts/components/v2/elf_runner.md">ELF runner</a> launches
  each component into a new job with a process running the executable code.</p>

  <p>For more examples, see
  <a href="concepts/components/v2/components_vs_processes.md">components
  vs. processes</a>.</p>
</aside>

Component manager parses each component's **declaration** to determine how to
run the component and supply the necessary capabilities. Components are
typically declared to the system through a **component manifest** file within
the component's package.

Below is a simple example of a component manifest that describes an ELF
executable with some additional command arguments:

```json5
program: {
    runner: "elf",
    binary: "bin/hello",
    args: [ "Hello", "World!" ],
},
```

Notice the runtime declaration telling the component manager that this
component requires the [ELF runner](concepts/components/v2/elf_runner.md).
**_This is an example of a capability!_**
