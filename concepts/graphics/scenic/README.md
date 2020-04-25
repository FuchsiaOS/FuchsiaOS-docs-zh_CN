<!-- # Fuchsia UI services -->

# Fuchsia UI 服务

<!-- The Fuchsia tree does not provide a full-fledged end-user UI.  Instead, it provides services that provide a foundation upon which to build secure, performant, multi-process UIs. -->

Fuchsia 代码树中不包含完备的用户界面。但它提供了一系列服务，这些服务为创建安全、高效、多进程的用户界面提供了基础。

<!-- These services include: -->

这些服务包括：

<!-- ## Scenic, the Fuchsia graphics engine -->

## Scenic，Fuchsia 的图形引擎

<!-- Scenic ([doc](scenic.md)) provides a retained-mode scene graph that allows graphical objects from multiple processes to be composed and rendered within a unified lighting environment. -->

Scenic （[文档](scenic.md)）提供了内部保存的场景图（scene graph），使得我们可以组合来自各个进程的图形对象，并在统一的光照环境下进行渲染。

<!-- ## Input -->

## Input（输入）

<!-- The input subsystem ([doc](input.md)) is responsible for discovering the available input devices, and allowing clients to register for events from these devices. -->

Input 输入子系统（[文档](input.md)）负责发现可用的输入设备，并允许客户端注册获取来自这些设备的输入事件。