<!--
# Lifecycle events
 -->
# 生命周期事件

<!--
The [Archivist][archivist] consumes lifecycle events to ingest diagnostics data. This document
explains what these events are.
 -->
[归档器][archivist]（Archivist）使用生命周期事件来获取诊断数据。本文档解释了这些事件。

<!--
## Archivist consumption of lifecycle events {#archivist-consumption}
 -->
## 归档器对生命周期事件的使用 {#archivist-consumption}

<!--
The archivist ingests events from the component framework.
The following diagram shows a very high level overview of the three lifecycle events
(started, directory_ready and stopped) the archivist is interested in.
 -->
归档器从组件框架中提取事件。下图展示了归档器关注的三个生命周期事件（启动、目录就绪和停止）的高级概述。

<!--
![Figure: Flow of lifecycle events under component manager](component_manager_lifecycle_flow.png)
 -->
![图：组件管理器下生命周期事件的流程](component_manager_lifecycle_flow.png)

<!--
The archivist consumes the following lifecycle events under component manager through
[`fuchsia.sys2.EventSource`][event_source]:
 -->
归档器通过 [`fuchsia.sys2.EventSource`][event_source] 在组件管理器下使用以下生命周期事件：

<!--
- **Started**: Sent by component manager when a component starts, the [runner] might still
  need to launch the component, but the component has started from the framework perspective.
- **Stopped**: Sent by component manager when a component stops, the runner might still need to to
  tear down the component, but the component is gone from the framework perspective.
- **Running**: Sent by component manager for all components that are running at the moment the
  archivist starts listening for events. In other words, a synthesized started event. This event
  is provided to the reader as **Started**, but consumed from the framework as “Running”.
- **Directory ready**: The archivist listens for directory ready of the `out/diagnostics`
  directory. When the component starts serving this directory, the component manager sends this
  event to the Archivist.
 -->
- **启动**（started）：由组件管理器发送，当组件启动，[运行器][runner]可能仍需启动组件，但从框架的角度出发组件已启动时。
- **停止**（stopped）：由组件管理器发送，当组件停止，运行器可能仍需拆解组件，但从框架的角度出发组件已停止时。
- **运行**（running）：由组件管理器发送，针对在归档器开始侦听事件时正在运行的所有组件。换句话说，这是一个合成的启动事件。该事件作为**启动**提供给读取者，但由框架将其作为“运行”使用。
- **目录就绪**（directory_ready）：归档器监听 `out/diagnostics` 目录的“目录就绪”状态。当组件开始为该目录提供服务时，组件管理器将该事件发送给归档器。

<!--
## Related docs
 -->
## 相关文档

<!--
- [Event capabilities][event_capabilities]
- [Inspect discovery and hosting - Archivist section][inspect_discovery_hosting]
 -->
- [事件能力][event_capabilities]
- [审视的发现与托管——归档器部分][inspect_discovery_hosting]


[archivist]: /reference/diagnostics/inspect/tree.md#archivist
[event_source]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#EventSource
[component_event_provider]: https://fuchsia.dev/reference/fidl/fuchsia.sys.internal#ComponentEventProvider
[event_capabilities]: /concepts/components/v2/capabilities/event.md
[inspect_discovery_hosting]: /reference/diagnostics/inspect/tree.md#archivist
[runner]: /glossary#runner
