<!-- ## Jobs, processes and threads -->
## 作业（job），进程（process）与线程（thread）

<!-- Zircon exposes three main kernel objects for running code: -->
Zircon 为运行中的代码暴露三个主要内核对象：

<!-- 
* [Thread](reference/kernel_objects/thread.md):
  Thread of execution within a given address space.
* [Process](reference/kernel_objects/process.md):
  Set of executable instructions run in a private, isolated address space.
* [Job](reference/kernel_objects/job.md):
  Group of related processes and jobs. All jobs form a single rooted tree.
 -->
* [线程](reference/kernel_objects/thread.md):
  给定地址空间中的执行序列。
* [进程](reference/kernel_objects/process.md):
  在私有、隔离的地址空间中运行的一组可执行指令。
* [作业](reference/kernel_objects/job.md):
  一组相关的进程和作业。所有作业形成一棵单根树。

<!-- 
![Tree diagram illustrating Fuchsia's process hierarchy. Processes are
grouped into jobs, which are ultimately owned by the Root Job.]
(get-started/images/intro/processes-jobs.png){: width="549"}
 -->
![阐明 Fuchsia 进程的层次结构的树形图。进程被分组到作业，作业最终由根作业所有。]
(get-started/images/intro/processes-jobs.png){: width="549"}

<!-- 
Processes form the basis for system capabilities. Each process is granted a set
of capabilities through the various handles it holds.
 -->
进程构成了系统能力的基础。每个进程通过其持有的各种句柄被授予一组能力。

<!-- 
Fuchsia software may or may not run within the confines of a single process.
Jobs allow "applications" that are composed of more than one process to be
controlled as a single entity.
 -->
Fuchsia 软件的运行不局限于单个进程内。
作业允许由多个进程组成的“应用”作为单个实体被控制。