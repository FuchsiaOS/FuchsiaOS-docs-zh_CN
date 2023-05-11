<!--
# event_stream capabilities
 -->
# 事件流能力

<!--
event_stream capabilities allow components to subscribe to specific events that
occur during the various stages of the [component lifecycle][doc-lifecycle].
This includes information about component state changes as well as capability
routing.
 -->
事件流能力（event_stream capability）使组件可以定期收到[组件生命周期][doc-lifecycle]各个阶段发生的特定事件。其中包括有关组件状态变化和能力路由的信息。

<!--
For a complete list of supported events and their descriptions, see the
[`fuchsia.component.EventType`][event-type] reference documentation. See [RFC-121]
for the original RFC for event_stream capabilities.
 -->
要获取关于支持事件及其描述的完整列表，请参阅 [`fuchsia.component.EventType`][event-type] 参考文档。要获取事件流能力的原始 RFC，请参阅[RFC-121]。

<!--
## Providing event_stream capabilities {#provide}
 -->
## 提供事件流能力 {#provide}

<!--
event_stream capabilities always originate from component manager. They are
provided to components from AboveRoot. Individual components cannot declare
events in their manifest `capabilities`, nor can they be used `from:
"framework"`
 -->
事件流能力总是源自组件管理器，这项能力从 Aboveroot 提供给组件。单个组件无法在其清单的 `capabilities` 中声明事件，也不能通过 `from: "framework"` 使用。

<!--
Each event_stream capability has an optional `scope` which determines the
subtree of events that a connecting client would receive. `scope` may be either
a single sub-tree or refer to multiple children.
 -->
每项事件流能力都有一个可选的 `scope`（范围），它确定接入的客户端会收到的事件子树。`scope` 可以是单个子树，或引用多个子树。

<!--
## Routing event_stream capabilities {#route}
 -->
## 路由事件流能力 {#route}

<!--
Components can [offer](#offer) event_stream capabilities that they receive from
their parent to other components. event_stream capabilities cannot be exposed.
 -->
组件可其从父级收到的事件流能力[提供](#offer)至其他组件。事件流能力无法公开。

<!--
For more details on how the framework routes component capabilities, see
[capability routing][capability-routing].
 -->
要获取关于框架路由组件能力方式的更多细节，请参见[能力路由][capability-routing]。

<!--
### Offering {#offer}
 -->
### 提供 {#offer}

<!--
Offering an event capability gives a child component access to that capability:
 -->
提供事件流能力会给予子组件访问该能力的权限：

```json5
{
    offer: [
        {
            event_stream: "started",
            from: "parent",
            scope: ["#child_a"]
            to: [ "#child-a", "#child_b" ],
        },
        {
            event_stream: "stopped",
            from: "parent",
            to: "#child-c",
        }
    ]
}
```

<!--
Events can only be offered from parent.
 -->
事件仅可由父级提供。

<!--
## Consuming event_stream capabilities {#consume}
 -->
## 使用事件流能力{#consume}

<!--
To consume an event capability, the component must use the `event_stream`
capability. Event streams may be merged by specifying multiple streams in a
single `use`.
 -->
要使用（consume）事件能力，组件必须使用 `event_stream`（事件流）能力。事件流可通过在单个 `use`（使用）声明中指定多个流进行合并。

<!--
To request the capability, add a `use` declaration for it:
 -->
要请求该能力，请为其添加 `use`（使用）声明：

```json5
{
    use: [
        { event_stream: ["started", "stopped"] },
    ]
}
```

<!--
event_streams may only be used `from: "parent"`.
 -->
事件流仅可通过 `from: "parent"` 使用。

<!--
## event_stream routing example {#example}
 -->
## 事件流路由示例 {#example}

<!--
Consider the following example of a component topology:
 -->
考虑以下组件拓扑示例：

<!--
![event_stream example][example-img]
 -->
![事件流示例][example-img]

<!--
Notice the following key aspects of this example:
 -->
注意该示例的以下关键方面：

<!--
-   `core/archivist`: Receives `started` events for the entire topology through
    an events capability routed from `root`.
 -->
-   `core/archivist`：通过路由自 `root`（根）的事件能力接收整个拓扑的 `started`（启动）事件。

    ```json5
    // root.cml
    {
        offer: [
            {
                event_stream: "started",
                from: "parent",
                to: "#core",
            },
        ]
    }

    // core.cml
    {
        offer: [
            {
                event_stream: "started",
                from: "parent",
                to: "#archivist",
            },
            {
                event_stream: "started",
                from: "parent",
                to: "#test_manager",
                scope: ["#test_manager"],
            }
        ]
    }

    // archivist.cml
    {
        use: [
            { event_stream: "started" },
        ]
    }
    ```

<!--
-   `core/test_manager/archivist`: Receives `started` events for all test
    components through an events capability routed from `test_manager`.
 -->
-   `core/test_manager/archivist`：通过路由自 `test_manager` 的事件能力接收所有测试组件的 `started` 事件。

    ```json5
    // test_manager.cml
    {
        offer: [
            {
                event_stream: "started",
                from: "parent",
                to: "#archivist",
            },
            {
                event_stream: "started",
                from: "parent",
                to: "#tests",
                // Downscopes the event to the tests collection
                scope: ["#tests"],
            }
        ]
    }
    ```

<!-- 
    - The root Archivist gets `started` events for all components in the
    topology from the root, whereas the embedded Archivist only gets events from
    its own test collection.
 -->
    - 根归档器（root Archivist）从根获取拓扑中所有组件的 `started` 事件，而嵌入归档器（embedded Archivist）仅从其自己的测试集合中获取事件。
 <!-- 
    NOTE: An event_stream must be routed through the entire topology from the root
    component_manager all the way down to the component that wants to use the event.
    event_streams cannot be used `from: "framework"`, and they are not
    automatically made available in every component's environment.
 -->
    注意：事件流必须通过整个拓扑，从根组件管理器一直路由至要使用事件的组件。事件流不能通过 `from: "framework"` 使用，且不会自动在每个组件的环境中可用。

    ```json5
    // archivist.cml
    {
          use: [
            {
                event_stream: "started",
                from: "parent",
            },
        ]
    }
    ```

<!--
-   `core/test_manager/tests:test-12345`: Receives started events for things
    under its collection through an `event_stream` capability routed from
    `test_manager`.
 -->
-   `core/test_manager/tests:test-12345`：通过路由自 `test_manager` 的 `event_stream` 能力接收其集合成员的 `started` 事件。

    ```json5
    // test-12345.cml
    {
        use: [
            {
                event_stream: "started",
                from: "parent",
            },
        ]
    }
    ```

[RFC-121]: /contribute/governance/rfcs/0121_component_events.md
[capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[doc-lifecycle]: /concepts/components/v2/lifecycle.md
[doc-realms]: /concepts/components/v2/realms.md
[event-source]: https://fuchsia.googlesource.com/fuchsia/+/refs/changes/44/656544/21/sdk/fidl/fuchsia.sys2/events.fidl#283
[event-type]: https://fuchsia.googlesource.com/fuchsia/+/refs/changes/44/656544/21/sdk/fidl/fuchsia.sys2/events.fidl#21
[example-img]: ../images/example_topology.svg
