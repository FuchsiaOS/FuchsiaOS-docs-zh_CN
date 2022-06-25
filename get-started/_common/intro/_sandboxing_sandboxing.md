## Sandboxing

When a new process is created, it has no capabilities. The process relies
entirely on its creator to provide capabilities through the set of
[handles][glossary.handle] passed to it. One might also say that an empty
process has no **ambient authority**.

Because of this, processes are usually created with some initial resources
and capabilities. The `fuchsia.process.Launcher` protocol provides the
low-level interface to create new processes on the system from an executable
and a set of kernel object handles. Most software uses the component framework,
which simplifies the work of setting up a new process to execute some code with
a standard set of initial capabilities. You will explore components in more
detail later on.


<aside class="key-point">
  <b>Handles have rights</b>
  <p>Previously you saw that handles are unique references to objects in the
  kernel. Each handle also contains the rights the handle has to perform
  certain actions, such as <code>ZX_RIGHT_READ</code>,
  <code>ZX_RIGHT_WRITE</code>, or <code>ZX_RIGHT_EXECUTE</code>.</p>

  <p>During process creation, the rights of each handle can be reduced to suit
  the requirements (and restrictions) of the new process using the
  <code>zx_handle_replace()</code> or <code>zx_handle_duplicate()</code>
   operations.

  <p>The creating process can then write the new handles across the IPC channel
  to set the initial capabilities of the new process.</p>
</aside>


Some initial handles given to a process are directories that the process mounts
into its **namespace**.

[glossary.handle]: glossary/README.md#handle