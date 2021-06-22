<!-- 
# Lifecycle events
 -->
# 生命周期事件

<!-- 
The [Archivist][archivist] consumes lifecycle events to ingest diagnostics data. Additionally, it provides an
interface to read those lifecycle events for diagnostics purposes. This document explains what
these events are and through which interface they can be accessed for diagnostics.
 -->
归档器[archivist]（Archivist）根据生命周期（lifecycle）事件来获取诊断数据。另外，它还提供了一个读取这些生命周期事件的接口，用于诊断目的。本文档解释了这些事件的内容，以及通过哪个界面才能将其用于诊断。

{#archivist-consumption}
<!-- 
## Archivist consumption of lifecycle events
 -->
## 归档器对于生命周期事件的获取

<!-- 
The archivist ingests events from both the v1 and v2 component framework. The
main difference between them is the protocol it uses to consume the events.

The following diagram shows a very high level overview of the three lifecycle events (started,
capability_ready and stopped) the archivist is interested on:
 -->
归档器获取来自 v1 和 v2 组件框架的事件。其中主要的不同在于它获取事件时所使用的协议。

下面的图表展示了归档器所关注的三种生命周期事件（已启动（started），功能就绪（capability_ready）和已停止（stopped））的高度概览：

- {Components v1}

  ![Figure: Flow of lifecycle events under appmgr](appmgr_lifecycle_flow.png)

<!-- 
  The archivist consumes the following lifecycle events in components v1 through
  [`fuchsia.sys.internal.ComponentEventProvider`][component_event_provider]:
 -->
  归档器通过 [`fuchsia.sys.internal.ComponentEventProvider`][component_event_provider] 获取如下 components v1 中的生命周期事件：

<!-- 
  - **Started**: Sent by appmgr when a component starts, the [runner][runner] might still need to
    launch the component. This event is synthesized for all components that exist at the moment the
    archivist starts listening for events.
  - **Stopped**: Sent by appmgr when a component stops. The runner might still need to tear down the
    component, but the component is gone from the framework perspective.
  - **Diagnostics ready**: Sent by appmgr when a component's `out/diagnostics` directory is being
    served by the component.
 -->
  - **已启动**（**Started**）：由 appmgr 在组件启动时发送，[启动器][runner]（runner）可能仍需启动该组件。该事件在归档器开始侦听事件时，针对当时所有的组件进行合成。
  - **已停止**（**Stopped**）：由 appmgr 在组件停止时发送。启动器可能仍需拆除（tear down）该组件，但是从框架角度来说，组件已经消失。
  - **诊断就绪**（**Diagnostics ready**）：由 appmgr 在组件为其 `out/diagnostics` 目录服务时发送。

- {Components v2}

  ![Figure: Flow of lifecycle events under component manager](component_manager_lifecycle_flow.png)

<!-- 
  The archivist consumes the following lifecycle events in components v2 through
  [`fuchsia.sys2.EventSource`][event_source]:
 -->
  归档器通过 [`fuchsia.sys2.EventSource`][event_source] 获取如下 components v2 中的生命周期事件：

<!-- 
  - **Started**: Sent by component manager when a component starts, the [runner][runner] might still
    need to launch the component, but the component has started from the framework perspective.
  - **Stopped**: Sent by component manager when a component stops, the runner might still need to to
    tear down the component, but the component is gone from the framework perspective.
  - **Existing**: Sent by component manager for all components that are running at the moment the
    archivist starts listening for events. In other words, a synthesized started event. This event
    is provided to the reader as **Running**, but consumed from the framework as “Existing”.
  - **Capability ready**: The archivist listens for capability ready of the `out/diagnostics`
    directory. When the component starts serving this directory, the component manager sends this
    event to the Archivist.
 -->
  - **已启动**（**Started**）：由组件管理器（component manager）在组件启动时发送，[启动器][runner]可能仍需启动该组件，但是从框架角度来看，组件已经启动。
  - **已停止**（**Stopped**）：由组件管理器在组件停止时发送。启动器可能仍需拆除该组件，但是从框架角度来看，组件已经消失。
  - **已存在**（**Existing**）：由组件管理器在归档器开始侦听事件时针对所有正在运行的组件发送。即合成的已启动事件。
  - **功能就绪**（**Capability ready**）：归档器侦听 `out/diagnostics` 目录的功能就绪。当组件开始服务该目录时，组件管理器向归档器发送该事件。

<!-- 
## Reading lifecycle events
 -->
## 读取生命周期事件

<!-- 
Lifecycle events can be read through the ArchiveAccessor. Only the `snapshot` mode is supported.
 -->
生命周期事件能够通过档案访问器（ArchiveAccessor）读取。只有 `snapshot` 模式受支持。

<!-- TODO(fxbug.dev/60763): link to ArchiveAccessor documentation where each mode is explained -->

<!-- 
Results are returned as a `vector<FormattedContent>` with each entry's variant matching the
requested `Format`, although JSON is the only supported format.
 -->
结果以 `vector<FormattedContent>` 形式返回，每个条目的变体与请求的 `Format` 匹配，尽管 JSON 是唯一受支持的格式。

<!-- 
### JSON object contents
 -->
### JSON 对象内容

<!-- 
Each JSON object in the array is one event entry. Like other data types in ArchiveAccessor,
each object consists of several fields, although the contents of metadata and payload differ
from other sources. The following is an example of a JSON object entry:
 -->
数组中的每个 JSON 对象都是一个事件条目。同档案访问器的其他数据类型一样，每个对象由几个字段（field）组成，尽管元数据和装载的内容与其他源不同。下面是一个 JSON 对象条目的示例：

```
{
    "version": 1,
    "moniker": "netstack.cmx",
    "data_source": "LifecycleEvent",
    "metadata": {
        "timestamp": 1234567890,
        "lifecycle_event_type": "Started",
        "component_url": "fuchsia-pkg://fuchsia.com/netstack#meta/netstack.cmx",
        “errors”: []
    },
    "payload": null,
}

```

<!-- 
#### Monikers
 -->
#### 昵称

<!-- 
Monikers identify the component related to the triggered event.

As explained in [Archivist consumption of lifecycle events](#archivist-consumption), there are two
systems that provide the archivist with events, appmgr and component manager. The monikers reflect
this. One simple way to distinguish between them is if the moniker has a `.cmx` extension then it's
a v1 component, otherwise it's a v2 component.
 -->
昵称（moniker）标明与触发事件相关联的组件。

正如在[归档器对于生命周期事件的获取](#archivist-consumption)中解释的那样，有两个系统都能向归档器提供事件，它们是 appmgr 和组件管理器。昵称反映了这一点。一个简单的区分方法是，如果昵称拥有一个 `.cmx` 扩展名，那么它是 v1 组件，否则是 v2 组件。

<!-- 
#### Timestamp
 -->
#### 时间戳

<!-- 
The time is recorded using the kernel's monotonic clock (nanoseconds) and conveyed without
modification as an unsigned integer. The time is when the event was created by the component
manager and appmgr, which also provide the time.
 -->
时间通过使用内核的单调时钟（纳秒）记录，并且不加修改地以无符号整型传送。在该时间，事件由组件管理器和 appmgr 创建，它们也提供该时间。

<!-- 
#### Lifecycle event type
 -->
#### 生命周期事件类型

<!-- 
These are the valid values for lifecycle event types:

- DiagnosticsReady
- Started
- Stopped
- Running
 -->
下列是生命周期事件的有效类型：

- 诊断就绪
- 已启动
- 已停止
- 运行中

<!-- 
#### Component URL
 -->
#### 组件 URL

<!-- 
The URL with which the component related to this event was launched.
 -->
用于启动与该事件相关组件的 URL。

<!-- 
#### Errors
 -->
#### 错误

<!-- 
Optional vector of errors encountered by the platform when handling this event.
Usually, no errors are expected for lifecycle events, so in most cases this is empty.
 -->
平台在处理该事件时遇到的可选错误（error）向量。通常，生命周期事件不会出现错误，因此大多数情况下它是空的。

<!-- 
#### Payload
 -->
#### 装载

<!-- 
The payload is always be empty for lifecycle events. Other types of data sources, like logs and
inspect, contain a payload. For more information, refer to the
[ArchiveAccessor documentation][archive_accessor].
 -->
生命周期事件的装载（payload）总是空的。其他类型的数据源，诸如日志（log）和审视（inspect），包含装载。要获取更多信息，请参阅[档案访问器文档][archive_accessor]。

<!-- 
## Related docs
 -->
## 相关文档

<!-- 
- [Event capabilities][event_capabilities]
- [Inspect discovery and hosting - Archivist section][inspect_discovery_hosting]
 -->
- [事件功能][event_capabilities]
- [审视发现与托管 - 归档器部分][inspect_discovery_hosting]


[archivist]: /docs/reference/diagnostics/inspect/tree.md#archivist
[event_source]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#EventSource
[component_event_provider]: https://fuchsia.dev/reference/fidl/fuchsia.sys.internal#ComponentEventProvider
[event_capabilities]: /docs/concepts/components/v2/capabilities/event.md
[inspect_discovery_hosting]: /docs/reference/diagnostics/inspect/tree.md#archivist
[component_runner]: /docs/glossary#runner
