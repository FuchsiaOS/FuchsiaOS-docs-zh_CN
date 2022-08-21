<!-- ## Component manager -->
## 组件管理器

<!-- The heart of the component framework is the **component manager**. It is
responsible for coordinating the execution of all component instances,
providing them with their capabilities, and intermediating connections between
components. -->
组件框架的核心是**组件管理器**。它负责协调所有组件实例的执行，提供相应的功能，连接各个组件。

<!-- Components can be launched explicitly (from a URL, for example) or implicitly
from a request for a particular capability. Component manager performs the
necessary resolution to determine whether to launch a new component or route
the request to an existing instance. For this routing to take place, every
component must declare any capabilities that it **provides** to the system
and any it **consumes**. -->
组件可以显式地启动（例如，通过网址调用）或者隐式地通过响应特定功能的请求而启动。组件管理器采取必要的步骤以决定是启动一个新组件还是将请求路由到现有实例。为实现这种路由功能，每个组件必须先声明它能**提供**给系统和它需要**使用**的功能。

<aside class="key-point">
  <!-- <b>Does each component run in its own process?</b> -->
  <b>每个组件是否运行在各自单独的进程中？</b>
  <!-- <p>Zircon defines the common kernel objects for runnable code, such as
  processes. However, component instances do not always correlate directly
  with a single process. Often the policy for how these processes are used
  is defined by the runner. For example, the
  <a href="/concepts/components/v2/elf_runner.md">ELF runner</a> launches
  each component into a new job with a process running the executable code.</p> -->
  <p>Zircon 给可执行代码定义了通用内核对象，例如，进程。然而，组件实例并不总是与单一进程直接相关。通常，如何使用进程的策略是由运行器定义的。例如，<a href="/concepts/components/v2/elf_runner.md">ELF 运行器</a>会在一个全新的任务中启动各个组件，该任务包含一个进程以运行可执行代码。</p>

  <!-- <p>For more examples, see -->
  <p>更多实例请参看
  <!-- <a href="/concepts/components/v2/components_vs_processes.md">components
  vs. processes</a>.</p> -->
  <a href="/concepts/components/v2/components_vs_processes.md">组件与进程</a>。</p>
</aside>

<!-- Component manager parses each component's **declaration** to determine how to
run the component and supply the necessary capabilities. Components are
typically declared to the system through a **component manifest** file within
the component's package. -->
组件管理器解析每个组件的**声明**以决定如何运行组件并提供必要的功能。组件通常通过组件包中包含的**组件清单**
文件向系统声明自己。

<!-- Below is a simple example of a component manifest that describes an ELF
executable with some additional command arguments: -->
以下是一个组件清单的简单例子，它描述了一个包含一些附加命令参数的 ELF 可执行程序：

```json5
program: {
    runner: "elf",
    binary: "bin/hello",
    args: [ "Hello", "World!" ],
},
```

<!-- Notice the runtime declaration telling the component manager that this
component requires the [ELF runner](/concepts/components/v2/elf_runner.md).
**_This is an example of a capability!_** -->
留意这个运行时声明，它告诉组件管理器这个组件需要[ELF 运行器](/concepts/components/v2/elf_runner.md)。
**_这是一个功能示例！_**
