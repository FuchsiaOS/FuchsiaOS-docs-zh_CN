<!-- ## Sandboxing -->
## 沙箱化（sandboxing）

<!-- 
When a new process is created, it has no capabilities. The process relies
entirely on its creator to provide capabilities through the set of
[handles][glossary.handle] passed to it. One might also say that an empty
process has no **ambient authority**.
 -->
当一个新进程被创建时，它没有任何能力。
进程完全依赖于其创建者通过传给它的那些[句柄][glossary.handle]来提供能力。
我们也可以说，空进程没有**环境权限**（ambient authority）。

<!-- 
Because of this, processes are usually created with some initial resources
and capabilities. The `fuchsia.process.Launcher` protocol provides the
low-level interface to create new processes on the system from an executable
and a set of kernel object handles. Most software uses the component framework,
which simplifies the work of setting up a new process to execute some code with
a standard set of initial capabilities. You will explore components in more
detail later on.
 -->
因此，进程被创建时通常被赋予一些初始资源和能力。
`fuchsia.process.Launcher` 协议提供了从一个可执行文件和一组内核对象句柄来在系统上创建新进程的低级接口。
大多数软件使用组件框架，它简化了创建一个有一组标准的初始能力的新进程来执行一些代码的工作。
您将在稍后更详细地探索组件。

<aside class="key-point">
  <!-- <b>Handles have rights</b> -->
  <b>句柄具有权利</b>
  <!-- 
  <p>Previously you saw that handles are unique references to objects in the
  kernel. Each handle also contains the rights the handle has to perform
  certain actions, such as <code>ZX_RIGHT_READ</code>,
  <code>ZX_RIGHT_WRITE</code>, or <code>ZX_RIGHT_EXECUTE</code>.</p>
 -->
  <p>之前您曾看到句柄是对内核中对象的唯一引用。每个句柄也包含其具有的执行特定操作的权利，
  比如 <code>ZX_RIGHT_READ</code>，<code>ZX_RIGHT_WRITE</code> 或 
  <code>ZX_RIGHT_EXECUTE</code>。</p>
<!-- 
  <p>During process creation, the rights of each handle can be reduced to suit
  the requirements (and restrictions) of the new process using the
  <code>zx_handle_replace()</code> or <code>zx_handle_duplicate()</code>
   operations.
 -->
   <p>在进程创建期间，使用 <code>zx_handle_replace()</code> 或 <code>zx_handle_duplicate()</code> 操作，
   每个句柄的权利可以被减小以适应新进程的需求（和限制）。</p>
<!-- 
  <p>The creating process can then write the new handles across the IPC channel
  to set the initial capabilities of the new process.</p>
 -->
  <p>创建者进程就可以将新句柄写进IPC通道来设置新进程的初始能力。</p>
</aside>

<!-- 
Some initial handles given to a process are directories that the process mounts
into its **namespace**.
 -->
<p>一些给予进程的初始句柄是被进程挂载到**命名空间**（namespace）中的目录。</p>

[glossary.handle]: /glossary/README.md#handle