A component is a program that runs on Fuchsia in its own sandbox.
Each component is a composable module that interacts with other components
through their capabilities. All software on Fuchsia is defined as a component
except for the kernel image, bootstrapping processes, and the Component Manager.

Fuchsia's component framework is responsible for running nearly all user space
software in the system. The Component Manager is a system process that coordinates
the execution and lifecycle of all component instances, maintains the component
topology, provides components with their capabilities, and keeps them isolated
from one another.

Components obtain privileges to access various parts of the wider system through
capabilities. Each component can declare new capabilities that they offer to the
system and capabilities provided by other components (or the framework) that
they require to function. Component Manager resolves and validates all capability
requests between components to ensure they match capabilities that the component
has been properly granted.

<aside class="key-point">
<b>Legacy components</b>

<p>This section focuses on modern components whose manifest declarations are
written in component manifest language (CML). The legacy framework based on
<code>appmgr</code> and declared using CMX manifests is not covered here.

<p>For more details on the legacy component framework, see
<a href="/concepts/components/v1">legacy components</a>.
</aside>
