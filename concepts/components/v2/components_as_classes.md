<!--
# Understanding components through object-oriented design
 -->
# 通过面向对象的设计了解组件

<!--
[Fuchsia Components][component-intro] are composable units of software
execution that emphasize reuse, isolation, and testability.
 -->
[Fuchsia组件][component-intro] 是软件执行的可复合单元，强调重用、隔离和可测试性。

<!--
This document provides an analogy between Fuchsia
Components and [Object-Oriented][oop] Design with [Dependency
Injection][dependency-injection]. This analogy allows Fuchsia developers
to apply their existing knowledge of Object-Oriented Design to develop
Fuchsia components using familiar terms and design patterns.
 -->
本文档提供了 Fuchsia 组件和具有[依赖注入][dependency-injection]（Dependency Injection）的[面向对象][oop]设计之间的类比。这种类比使 Fuchsia 开发人员能够应用其面向对象设计的现有知识，以使用熟悉的术语和设计模式开发 Fuchsia 组件。

<!--
## Introduction
 -->
## 引言

<!--
In [Object-Oriented Programming][oop] (OOP), an **object** is an entity
that contains both *data* and *methods* that operate on that that data. A
**class** defines the data and methods associated with a particular type
of object. An object is an instantiation of a class.
 -->
在[面向对象程序设计][oop]（Object-Oriented Programming，OOP）中，**对象**（object）是包含“数据”（data）和操作该数据“方法”（method）的实体。**类**（class）定义与特定类型的对象关联的数据和方法。对象是类的实例化。

<!--
Similarly, **components** contain internal program state (data) and expose
protocols (groups of methods) that operate on their internal state. Where
a class declares the callable methods on an object, a **[component
manifest][component-manifests]** declares the callable protocols on a
component. Components are instantiated as **component instances**.
 -->
类似地，**组件**包含内部程序状态（数据）并公开在内部状态下运行的协议（方法组）。与类在对象上声明可调用方法的情况相对应，**[组件清单][component-manifests]**在组件上声明可调用协议。组件被实例化为**组件实例**（component instance）。

<!--
**Protocols**, defined using [FIDL][fidl], declare interfaces between
components. Providing a protocol means that a component implements that
protocol, similar to how classes may *implement* an interface or trait.
 -->
**协议**，使用 [FIDL][fidl] 定义，声明了组件之间的接口。提供一个协议意味着有一个组件实现了该协议，类似于类可以“实现”接口（interface）或特征（trait）的方式。

<!--
Note: Generally in Component Framework, "[capabilities][capabilities]" are
exposed, used, and offered. A common representation of a capability is a
Zircon channel that speaks a particular FIDL protocol, and for this reason
this document uses "protocol" instead of the "capability" terminology.
 -->
注意：通常，在组件框架中，“[能力][capabilities]”是被公开、使用和提供的。能力的常见表示是一个使用特定 FIDL 协议的 Zircon 通道，因此，该文档使用“协议”而不是“能力”术语。

<!--
This document explores the analogy between components implementing
protocols and classes implementing interfaces, and this analogy extends
to the ways in which components and objects relate to other components
or objects.
 -->
本文档探讨了组件实现协议和类实现接口之间的类比，该类比又扩展到组件和对象与其他组件或对象沟通的方式。

<!--
Two important relationships are "Has-A" (in which one object is *composed
of* other objects), and "Depends-On/Uses-A" (in which one object requires
another object be present to properly operate).
 -->
两个重要的关系是“复合”（Has-A）（该关系下一个对象由其他对象“组成”）和“依赖/使用”（Depends-On/Uses-A）（该关系下一个对象需要另一个对象存在才能正确运转）。

<!--
Components may exhibit these same relationships. A single component may
be composed of multiple child components, and like in OOP the presence
of these children is an implementation detail of the component. Similar
to a class constructor that takes in a required object, component
manifests declare the protocols they depend on. The Component Framework
is concerned with how these dependent protocols are routed and satisfied
so that a component may execute, which is analogous to how [Dependency
Injection][dependency-injection] works for OOP.
 -->
组件可能清单相同的关系。单个组件可以由多个子组件组成，就像在 OOP 中一样，这些子组件的存在是该组件的实现细节。与类构造函数接收需要的对象相类似，组件清单声明其依赖的协议。组件框架关注这些依赖协议的路由和满足方式，以便组件可以执行，而这类似于[依赖注入][dependency-injection]在 OOP 中的工作方式。

<!--
The other common relationship in OOP is "Is-A" (Inheritance), where a
class can extend another class' data and logic. In Component Framework,
there is no analog to Inheritance.
 -->
OOP 中的另一个常见关系是“继承”（Is-A）（Inheritance），该关系下一个类可以扩展另一个类的数据和逻辑。在组件框架中，没有类似于继承的关系。

<!--
Together, these similarities provide the following mapping between OOP
and Component Framework concepts:
 -->
这些相似性共同提供了下列 OOP 和组件框架概念之间的映射：

<!--
Object-Oriented Concept | Component Concepts
-----                   | -----
Interface | FIDL protocol
Class Definition | Component manifest
Object | Component instance
Inner / Associated Class | FIDL Data
Depends-On/Uses-A Relationship (Dependency) | Capability routed from parent
Has-A Relationship (Composition) | Child component
Implements Interface | Expose capability from self
Is-A Relationship (Inheritance) | N/A, prefer "Implements"
 -->
| 面向对象概念                | 组件概念                   |
| --------------------------- | -------------------------- |
| 接口                        | FIDL 协议                  |
| 类定义                      | 组件清单                   |
| 对象                        | 组件实例                   |
| 内部/关联类                 | FIDL 数据                  |
| 依赖/使用关系（Dependency） | 路由自父组件的能力         |
| 复合关系（Composition）     | 子组件                     |
| implements 接口             | 自身的 expose 能力         |
| 继承关系（Inheritance）     | 不适用，请使用“implements” |

<!--
Component Framework provides abilities that go above and beyond OOP,
but starting from OOP principles will give you a reasonable approximation
of your final component design.
 -->
组件框架提供了超越 OOP 的能力，但是从 OOP 原则起步会让您获得一个对最终组件设计的合理近似。

<!--
The rest of this document provides more detail and examples of how to
map the above OOP concepts into Component concepts.
 -->
本文档的其余部分提供了更多的详细信息和示例，以将上述 OOP 概念映射到组件概念中。

<!--
## FIDL protocols as interfaces
 -->
## FIDL 协议作为接口

<!--
Many OOP languages have the concept of an **interface** or a **trait**
that can be *implemented* by an object. Where classes define data and
behavior, interfaces only *declare* behavior that may be present on
a class. Implementers of an interface can be grouped and used interchangeably.
 -->
许多 OOP 语言具有**接口**（interface）或**特征**（trait）的概念，可以通过对象“实现”（implement）。其中类定义数据和行为，接口仅“声明”（declare）类中可能存在的行为。接口的不同实现者的分组和使用是可交换的。

<!--
On Fuchsia, FIDL defines interfaces between components. Similar to OOP
interfaces, implementers of FIDL Protocols can be used interchangeably.
 -->
在 Fuchsia 中，FIDL 定义了组件之间的接口。与 OOP 接口类似，FIDL 协议的实现者可以互换使用。

<!--
### Example
 -->
### 示例

<!--
Consider an interface called `Duck`, with a method called `Quack`.
 -->
考虑一个名为 `Duck`（鸭）的接口，使用名为 `Quack`（嘎）的方法。

<!--
The FIDL protocol is as follows:
 -->
FIDL协议如下：

```fidl
library fuchsia.animals;

@discoverable
protocol Duck {
  Quack();
}
```

<!--
To implement this protocol, a component would include the following
snippet in their component manifest:
 -->
为了实现该协议，组件将在其组件清单中包括以下片段：

<!-- 
```json5
{
  // ...

  // Declare that this component implements a protocol.
  capabilities: [
    {
      protocol: ["fuchsia.animals.Duck"],
    }
  ],

  // Expose this protocol so that other components may call it.
  expose: [
    {
      protocol: ["fuchsia.animals.Duck"],
      from: "self",
    }
  ],
}
```
 -->
```json5
{
  // ...

  // 声明该组件实现了一个协议。
  capabilities: [
    {
      protocol: ["fuchsia.animals.Duck"],
    }
  ],

  // 公开该协议，以便其他组件调用。
  expose: [
    {
      protocol: ["fuchsia.animals.Duck"],
      from: "self",
    }
  ],
}
```

<!--
In various OOP languages, you would write this as follows:
 -->
在各种 OOP 语言中，您将编写以下内容：

* {C++}

<!-- 
  ```cpp
  // Duck is an abstract class with a pure virtual method "Quack".
  class Duck {
    public:
      virtual void Quack() = 0;
  };

  // Actually implement Duck.
  class MallardDuck : public Duck {
    public:
      void Quack() override { /* ... */ }
  }
  ```
 -->
  ```cpp
  // Duck 是拥有纯虚方法“Quack”的抽象类。
  class Duck {
    public:
      virtual void Quack() = 0;
  };

  // Duck 的实际实现。
  class MallardDuck : public Duck {
    public:
      void Quack() override { /* ... */ }
  }
  ```


* {Dart}

<!-- 
  ```dart
  // All classes in Dart define an interface.
  // Omitting the definition of Quack means it can be overridden in a child.
  abstract class Duck {
    Quack();
  }

  MallardDuck implements Duck {
    @override
    Quack() { /* ... */ }
  }
  ```
 -->
  ```dart
  // Dart 中的所有类都定义了接口。
  // 忽略 Quack 的定义表示其可被子类覆盖。
  abstract class Duck {
    Quack();
  }

  MallardDuck implements Duck {
    @override
    Quack() { /* ... */ }
  }
  ```

* {Rust}

<!-- 
  ```rust
  // Rust uses "traits" rather than interfaces.
  // A trait may be explicitly implemented for any type.
  trait Duck {
    fn quack(&self);
  }

  // The type MallardDuck implements the "Duck" trait.
  struct MallardDuck {
  }

  impl Duck for MallardDuck {
    fn quack(&self) { /* ... */ }
  }
  ```
 -->
  ```rust
  // Rust 使用“特征”（trait）而非接口（interface）。
  // 对于任何类而言，特征都可以显式实现。
  trait Duck {
    fn quack(&self);
  }

  // MallardDuck 类实现了“Duck”特征。
  struct MallardDuck {
  }

  impl Duck for MallardDuck {
    fn quack(&self) { /* ... */ }
  }
  ```

* {Java}

<!-- 
  ```java
  // Explicitly define a public Duck interface.
  public interface Duck {
    public void Quack();
  }

  // Create a class that implements the interface.
  public class MallardDuck {
    @Override
    public void Quack() { /* ... */ }
  }
  ```
 -->
  ```java
  // 显式定义了公共的 Duck 接口。
  public interface Duck {
    public void Quack();
  }

  // 创建实现该接口的类。
  public class MallardDuck {
    @Override
    public void Quack() { /* ... */ }
  }
  ```

<!--
## Component manifests as classes
 -->
## 组件清单作为类

<!--
The core concept of OOP is the **object**, which contains both data
and methods that operate on that data. [Class-based OOP][class-oop]
languages define hierarchies of *classes of objects* to describe data
and their relationships. A class may be instantiated into an object
multiple times, and they may be used modularly.
 -->
OOP 的核心概念是**对象**，它包含数据和操作该数据的方法。[基于类的 OOP][class-oop] 语言定义“对象的类”的层次结构来描述数据及其关系。类可以多次实例化为对象，并且可以模块化地使用它们。

<!--
Similarly, a Fuchsia system is defined as a hierarchy of components,
each of which is defined by its **component manifest**. A component
manifest defines a *class of component* that can be instantiated and
used modularly.
 -->
同样，Fuchsia 系统被定义为组件的层次结构，其中每个组件都由其**组件清单**定义。组件清单定义了一个可以实例化并模块化使用的“组件的类”。

<!--
Both objects and components represent a reusable unit of behavior and
data, grouped by interfaces that they implement.
 -->
对象和组件都代表一个可重用的行为和数据单元，按它们实现的接口分组。

<!--
### Example
 -->
### 示例

<!--
Consider a component that rolls an N-sided die. That is, the component
returns a value in the range `[1, N]` when it is requested.
 -->
考虑一个掷 N 面骰子的组件。也就是说，组件在收到请求时返回一个范围在 `[1, N]` 的值。

<!--
In Fuchsia, we define a protocol to be part of the component's interface:
 -->
在 Fuchsia 中，我们将协议定义为组件接口的一部分：

<!-- 
```fidl
library fuchsia.dice;

// fuchsia.dice.Roller supports rolling a single die
protocol Roller {

  // Method "Roll" takes no arguments, and it returns an "outcome" that is a 64-bit unsigned integer.
  Roll() -> (struct {
    outcome uint64
  });
}
```
 -->
```fidl
library fuchsia.dice;

// fuchsia.dice.Roller 支持掷出单个骰子
protocol Roller {

  // “Roll” 方法不接受参数，并返回 64 位无符号整型“结果”。
  Roll() -> (struct {
    outcome uint64
  });
}
```

<!--
The manifest for your component is as follows:
 -->
组件的清单如下所示：

<!-- 
```json5
// dice_roller.cml

{
  // The execution details for the component.
  //
  // This section says the component should be run as a normal ELF binary
  // (e.g. C++ or Rust) by executing the file at path "bin/dice_roller"
  // in the containing package.
  //
  // It also says to pass the command line arguments '"--sides" "6"' to
  // the program. It is the program's responsibility to parse its command
  // line parameters. In this case we want a 6-sided die.
  program: {
    runner: "elf",
    binary: "bin/dice_roller",
    args = [
      "--sides",
      "6",
    ],
  },

  // Declare the protocols the component implements (see previous section)
  capabilities: [{
    protocol: ["fuchsia.dice.Roller"],
  }],

  // Expose the protocols the component implements.
  expose: [{
    protocol: ["fuchsia.dice.Roller"],
    from: "self",
  }],
}
```
 -->
```json5
// dice_roller.cml

{
  // 组件的执行细节。
  //
  // 本节表示组件应作为普通 ELF 二进制文件（例如 C++ 或 Rust）运行，
  // 方法是执行包含包中路径“bin/dice_roller”内的文件。
  //
  // 它还表示将命令行参数“"--sides" "6"”传递给程序。解析其命令行参数是由程序负责的。
  // 本例中我们需要一个 6 面骰子。
  program: {
    runner: "elf",
    binary: "bin/dice_roller",
    args = [
      "--sides",
      "6",
    ],
  },

  // 声明组件实现的协议（见前文）
  capabilities: [{
    protocol: ["fuchsia.dice.Roller"],
  }],

  // 公开组件实现的协议。
  expose: [{
    protocol: ["fuchsia.dice.Roller"],
    from: "self",
  }],
}
```

<!-- TODO: We should show a C++/Rust component implementing the protocol as well -->

<!--
An analogous class definition would be as follows:
 -->
类似的类定义如下所示：

* {C++}

  ```cpp
  class DiceRoller : public Roller {
    public:
      DiceRoller(uint64_t number_of_sides) : number_of_sides(number_of_sides) {}
      uint64_t Roll() override;

    private:
      const uint64_t number_of_sides;
  };
  ```

* {Dart}

  ```dart
  class DiceRoller implements dice.Roller {
      final int numberOfSides;
      DiceRoller({required this.numberOfSides});

      @override
      int Roll() { /* ... */ }
  }
  ```

* {Rust}

  ```rust
  pub struct DiceRoller {
      number_of_sides: u64,
  }

  impl DiceRoller {
    pub fn new(number_of_sides: u64) -> Self {
      Self{ number_of_sides }
    }
  }

  impl Roller for DiceRoller {
    pub fn roll(&self) -> u64 { /* ... */ }
  }
  ```

* {Java}

  ```java
  class DiceRoller implements Roller {
      private long numberOfSides;

      public DiceRoller(long numberOfSides) {
        this.numberOfSides = numberOfSides;
      }

      @Override
      public long Roll() { /* ... */ }
  }
  ```

<!--
In each of these examples, there is a `DiceRoller` that implements the
`Roll` method from a `Roller` interface. `DiceRoller` is parameterized by
its input argument that specifies the number of sides the die will have.
 -->
上述的每个示例中都有一个 `DiceRoller`（掷骰器），它从 `Roller`（投掷器）接口实现 `Roll`（投掷）方法。`DiceRoller` 的参数化，是通过输入参数指定骰子面数的方式进行的。

<!--
In the OOP examples it is possible to define a `DiceRoller` with any
arbitrary number of sides, but the component manifest specifies the
value 6.
 -->
在 OOP 示例中，是可以用任意面数定义 `DiceRoller` 的，不过该组件清单指定了值为 6。

<!--
## Component instances as objects, children as composition
 -->
## 组件实例作为对象，子组件作为构图

<!--
An object is an instantiation of a class in an OOP language, and a
*component instance* is an instantiation of a *component* as defined by
its manifest. Both objects and components can be instantiated multiple
times, which allows them to be reused in different contexts.
 -->
对象是 OOP 语言中类的实例化，“组件实例”是由其清单定义的“组件”的实例化。对象和组件都可以多次实例化，这允许它们在不同的上下文中重用。

<!--
Objects and component instances primarily differ in how they are instantiated.
 -->
对象和组件实例主要在实例化的方式上有所不同。

<!--
In OOP languages, objects may be created by calling their
*constructor*, and in some languages objects are destroyed by calling a
*destructor*. Various strategies and design patterns exist to abstract
away object creation (such as the [Factory Pattern][factory-method]),
but in the end the object is always explicitly created somewhere.
 -->
在 OOP 语言中，对象可以通过调用它们的“构造函数”（*constructor*）来创建，而在某些语言中，对象可以通过调用“析构函数”（*destructor*）来销毁。存在各种策略和设计模式来抽象对象创建（例如[工厂模式][factory-method]（Factory Pattern）），但最终对象总是在某处显式创建。

<!--
By contrast, component instances are typically defined in a static
hierarchy. Simply specifying a component as a child of another component
is sufficient to [make the child *exist*][component-creation]. Existence
does not imply that the component is actually running, however. Typically
a component runs only when a something [binds][component-binding]
to a capability it exposes. In OOP terms, it would be as if an object
came into existence the first time a method was called on it (a type
of [Late Binding][late-binding] or [Lazy Initialization][lazy-init]).
Components have their own [lifecycle][lifecycle] which largely does not need to
be observed.
 -->
相比之下，组件实例通常在静态层次结构中定义。简单地将一个组件指定为另一个组件的子组件就足以[使子组件“存在”][component-creation]。但是，存在并不意味着该组件实际上正在运行。通常，只有当某物[绑定][component-binding]到它公开的功能时，组件才会运行。在 OOP 术语中，就好像一个对象在第一次调用方法时就已经存在（[后期绑定][late-binding]（Late Binding）或[延迟初始化][lazy-init]（Lazy Initialization）的一种）。组件有自己的[生命周期][lifecycle]，在很大程度上不需要观察。

<!--
The exception to static component initialization is [dynamic
component collections][component-collections]. The collection itself is
statically defined, but components in the collection may be dynamically
[created][realm-create], bound to by [opening][realm-open] an exposed capability,
and [destroyed][realm-destroy]. This would be represented as a collection
holding objects in OOP, though the Component Framework gives you lazy binding
for free.
 -->
静态组件初始化的例外是[动态组件集合][component-collections]（dynamic component collection）。集合本身是静态定义的，但其中的组件可以动态[创建][realm-create]，通过[打开][realm-open]一项公开的能力来绑定，以及[销毁][realm-destroy]。这会表示为 OOP 中包含对象的集合，尽管组件框架为您无偿提供了延迟绑定。

<!--
The state of a component is composed of its own state and that of their
children, similar to how object state is composed of its own state
and that of contained objects. The behavior of a component consists of
its own behavior and its interaction with children through protocols,
similar to how object behavior consists of its own behavior and its
interaction with contained objects through methods.
 -->
组件的状态由其自身的状态及其子组件的状态组成，这类似于对象状态由其自身的状态及所包含对象的状态组成。组件的行为包括其自身的行为以及其通过协议与子组件进行的交互，这类似于对象行为包括其自身的行为以及其通过方法与所包含对象进行的交互。

<!--
### Example
 -->
### 示例

<!--
In this example there exists a hierarchy of objects representing a "user
session." `UserSession` consists of one `User` and multiple `Apps`.
 -->
本示例中存在表示“用户会话”（user session）的对象层次结构。`UserSession` 由一个 `User` 和多个 `Apps` 组成。

<!--
This structure may be implemented using components as follows:
 -->
可以使用如下所示的组件实现该结构：

<!-- 
```json5
// user_session.cml
{
  // ...
  // user_session has a static child called "user", declared in user.cml
  children: [
    {
      name: "user",
      url: "fuchsia-pkg://fuchsia.com/session_example#meta/user.cm",
    },
  ],
  // user_session has a collection for dynamic components, called "apps"
  collections: [
    {
      name: "apps",
    }
  ],
  // access the User protocol from the child called "user".
  use: [
    {
      protocol: "fuchsia.session_example.User",
      from: "#user",
    },
  ],
}
```
 -->
```json5
// user_session.cml
{
  // ...
  // user_session 拥有一个名为“user”的静态子组件，声明于 user.cml
  children: [
    {
      name: "user",
      url: "fuchsia-pkg://fuchsia.com/session_example#meta/user.cm",
    },
  ],
  // user_session 拥有一个名为“apps”的动态组件集合
  collections: [
    {
      name: "apps",
    }
  ],
  // 访问来自“user”子组件的 User 协议
  use: [
    {
      protocol: "fuchsia.session_example.User",
      from: "#user",
    },
  ],
}
```

<!-- 
```json5
// user.cml
{
  // ..

  // Expose the User capability, which provides information and actions on the current user.
  capabilities: [
    { protocol: "fuchsia.session_example.User" },
  ],
  expose: [
    {
      protocol: "fuchsia.session_example.User",
      from: "self",
    },
  ],
}
```
 -->
```json5
// user.cml
{
  // ..

  // 公开 User 能力，该能力提供当前用户的信息和动作。
  capabilities: [
    { protocol: "fuchsia.session_example.User" },
  ],
  expose: [
    {
      protocol: "fuchsia.session_example.User",
      from: "self",
    },
  ],
}
```

<!-- 
```cpp
// C++-like pseudocode for interacting with the child components from the session.

// Create any arbitrarily named app in the apps collection with just a URL to execute.
CreateChild("apps", "my_app", "fuchsia-pkg://..." /* url to the app to run */);

// Accessing exposed protocols causes the component to actually run. The
// output parameter is a Directory handle over which capabilities are accessed.
OpenExposedDir("apps", "my_app", &out_dir);

// Open any arbitrary capability on the bound component.
// Assuming that the "Controller" protocol has a method called "ExecuteCommand".
out_dir.Open("fuchsia.my_app.Controller").ExecuteCommand();

// Connect to the protocol from the static child.
// This is available in the incoming namespace for session, since it
// "uses" the capability.
incoming_namespace.Open("fuchsia.session_example.User").GetName();
```
 -->
```cpp
// 用于从会话中与子组件交互的类 C++ 伪代码。

// 仅通过要执行的网址，在 apps 集合中创建任意名称应用。
CreateChild("apps", "my_app", "fuchsia-pkg://..." /* 要执行的 app 的网址 */);

// 访问公开协议会导致组件实际运行。
// 输出参数是访问功能的目录句柄。
OpenExposedDir("apps", "my_app", &out_dir);

// 在绑定组件上打开任意功能。
// 假设“Controller”（控制器）协议有一个名为“ExecuteCommand”（执行命令）的方法。
out_dir.Open("fuchsia.my_app.Controller").ExecuteCommand();

// 从静态子组件连接到协议。
// 这在会话的传入命名空间中可用，因为它“使用”（use）了该能力。
incoming_namespace.Open("fuchsia.session_example.User").GetName();
```

<!--
The Component Framework allows any arbitrary component to be started in
the "apps" collection, so long as its dependencies are satisfied
(see later section).
 -->
组件框架允许在“apps”集合中启动任意组件，只要满足其依赖关系即可（请参阅后文）。

* {C++}

<!-- 
  ```cpp
  // User is an abstract class representing a user of the session.
  // It declares the "GetName" method all users must implement.
  class User {
    public:
      virtual std::string GetName()  = 0;
  };

  // App is class representing the interface to apps.
  class App {
    /* ... */
  };

  class Session {
    public:
      Session() : user(/* initialize user */) {}

      void AddApp(App app) {
        apps.push_back(std::move(app));
      }

    private:
      std::unique_ptr<User> user;

      // Note that in C++ the collection needs to be typed, while in component
      // terms all components share a base type.
      std::vector<App> apps;
  };
  ```
 -->
  ```cpp
  // User 是一个表示会话用户的抽象类。
  // 它声明了所有用户必须实现的“GetName”（获取名称）方法。
  class User {
    public:
      virtual std::string GetName()  = 0;
  };

  // App 是表示 apps 接口的类。
  class App {
    /* ... */
  };

  class Session {
    public:
      Session() : user(/* 初始化 user */) {}

      void AddApp(App app) {
        apps.push_back(std::move(app));
      }

    private:
      std::unique_ptr<User> user;

      // 请注意，在 C++ 中，集合需要类型化（typed），而在组件术语中，所有组件共享一个基类型。
      std::vector<App> apps;
  };
  ```

* {Dart}

<!-- 
  ```dart

  interface User {
    String GetName();
  }

  class Session {
      final User user;
      final List<Object> apps = [];
      Session() :  user = /* initialize user */;

      // Similar to how all components share a "base type", Dart's Object
      // type can be dynamically cast to a desired interface.
      //
      // Casting will fail if the Object does not implement the type
      // requested, similar to how connecting to a non-exposed capability
      // fails for a component.
      void AddApp(Object app) {
        apps.add(app);
      }
  }
  ```
 -->
  ```dart

  interface User {
    String GetName();
  }

  class Session {
      final User user;
      final List<Object> apps = [];
      Session() :  user = /* 初始化 user */;

      // 与所有组件共享“基类型”的方式类似，Dart 的对象类型可以动态转换为所需的接口。
      //
      // 如果对象未实现所请求的类型，则转换失败，这类似于组件连接到非公开功能而失败。
      void AddApp(Object app) {
        apps.add(app);
      }
  }
  ```

* {Rust}

<!-- 
  ```rust
  pub trait User {
      fn get_name() -> String;
  }

  pub trait App {
    /* ... */
  }

  pub struct Session {
    user: User,

    // Note that in Rust the collection needs to be typed, while in component
    // terms all components share a base type.
    apps: Vec<Box<dyn App>>;
  }

  impl Session {
    pub fn new() -> Self {
      Self{ user: /* initialize user */, apps: Vec::new() }
    }

    pub fn add_app(&mut self, app: Box<dyn App>) {
      self.apps.push(app);
    }
  }
  ```
 -->
  ```rust
  pub trait User {
      fn get_name() -> String;
  }

  pub trait App {
    /* ... */
  }

  pub struct Session {
    user: User,

    // 请注意，在 Rust 中，集合需要类型化（typed），而在组件术语中，所有组件共享一个基类型。
    apps: Vec<Box<dyn App>>;
  }

  impl Session {
    pub fn new() -> Self {
      Self{ user: /* 初始化 user */, apps: Vec::new() }
    }

    pub fn add_app(&mut self, app: Box<dyn App>) {
      self.apps.push(app);
    }
  }
  ```

* {Java}

<!-- 
  ```java
  interface User {
    String GetName();
  }

  class Session {
      private User user;
      private List<Object> apps = new ArrayList<Object>();

      public Session() {
        user = /* initialize user */;
      }

      // Similar to how all components share a "base type", Java's Object
      // type can be dynamically cast to a desired interface.
      //
      // Casting will fail if the Object does not implement the type
      // requested, similar to how connecting to a non-exposed capability
      // fails for a component.
      public void AddApp(Object app) {
        apps.add(app);
      }
  }
  ```
 -->
  ```java
  interface User {
    String GetName();
  }

  class Session {
      private User user;
      private List<Object> apps = new ArrayList<Object>();

      public Session() {
        user = /* 初始化 user */;
      }

      // 与所有组件共享“基类型”的方式类似，Java 的对象类型可以动态转换为所需的接口。
      //
      // 如果对象未实现所请求的类型，则转换失败，这类似于组件连接到非公开功能而失败。
      public void AddApp(Object app) {
        apps.add(app);
      }
  }
  ```

<!--
## FIDL data as inner or associated classes
 -->
## FIDL 数据作为内部或关联类

<!--
It is common in OOP to have objects that act upon other objects. Previous
sections of this document focused on cases where the object uses a
dependency for additional behavior, but also important are cases where
an object depends on data stored in other objects. This is common in
container interfaces, where one object maintains a collection of other
objects and exposes an interface to manipulate the collection in some way.
 -->
在 OOP 中，对象作用于其他对象很常见。本文档中前文重点介绍了对象使用依赖项来实现其他行为的情况，但对象依赖于存储在其他对象中数据的情况也很重要。这在容器接口中很常见，其中一个对象维护其他对象的集合，并公开一个接口以某种方式操作该集合。

<!--
Components are best suited to represent objects with complex behaviors
rather than act as data containers. FIDL provides the ability to express
extensible data structures that can be passed to and from protocols,
and these types are more suitable for representing data than components.
 -->
组件最适合用于表示具有复杂行为的对象，而非充当数据容器。FIDL 提供了表达可扩展数据结构的能力，这些数据结构可以传入和传出协议，这些类型比组件更适合用于表示数据。

<!--
Generally, if an interface calls for acting upon [plain old data
types][plain-data], the data should be stored within the component,
declared using FIDL `tables`, and exposed by a protocol providing
accessors and mutators on the `table`.
 -->
一般地，如果一个接口要求对[普通旧数据类型][plain-data]进行操作，那么数据应该存储在组件中，使用 FIDL `tables` 来声明，并通过 `table` 上提供访问器（accessor）和修改器（mutator）的协议来公开。

<!--
[Builder interfaces][builder-pattern] that imperatively construct a data
type before executing an operation can also be represented best in FIDL.
 -->
[构建器接口][builder-pattern]（builder interface）在执行操作之前强制构造数据类型，它也最好使用 FIDL 表示。

<!--
### Example
 -->
### 示例

<!--
In this example we will create a `Store` interface that contains a number
of `Items` for sale. Customers can create a `Cart` to which they add
items and eventually `Checkout()`.
 -->
本示例中，我们将创建一个 `Store`（商店）接口，其中包含许多待售 `Items`（商品）。顾客可以创建一个 `Cart`（购物车），在其中添加商品，并最后 `Checkout()`（结帐）。

<!-- 
```fidl
library fuchsia.store;

// An Item is a plain data type describing individual items in the store.
type Item = table {
  // Each Item has a unique ID, which is used to reference the object.
  1: id uint64;
  2: name string;
  3: price_in_cents uint32;
  4: quantity_in_stock: uint32;
}

type StoreError = strict enum {
  ITEM_NOT_FOUND = 1;
  INVALID_QUANTITY = 2;
};

protocol Store {
  // Add new items to the store.
  // No return code, so this operation is asynchronous and can fail silently.
  AddItem(struct {
    item: Item;
  });

  // Set the price on an existing item, by id.
  // Fails if the item is not found.
  SetPrice(struct {
    item_id: uint64;
    new_price: uint32;
  }) error StoreError;

  // Add (or subtract) additional stock of an item.
  // Fails if the item is not found or if you would be left with an
  // invalid quantity of the item.
  AddStock(struct {
    item_id: uint64;
    additional_quantity: int32;
  }) error StoreError;

  // Create a new Cart interface to shop at the store.
  // Note that this takes a "resource" struct, because request is a
  // Zircon handle.
  CreateCart(resource struct {
    request: server_end:Cart;
  });
};

type CartError = strict enum {
  PAYMENT_FAILURE = 1;
  NOT_ENOUGH_IN_STOCK = 2;
};

// Cart uses the builder pattern to create a set of items and checkout atomically.
protocol Cart {
  // Add a specific quantity of an item by id to the cart.
  AddItem(struct {
    item_id: uint64;
    quantity: uint32;
  });

  // Add a coupon code to the cart.
  AddCouponCode(struct {
    code: string;
  });

  // Checkout all previously added items atomically.
  // Fails if payment fails or if there are not enough items in stock
  // to satisfy the request.
  Checkout() error CartError;
};
```
 -->
```fidl
library fuchsia.store;

// Item 是描述商店中各种商品的普通数据类型。
type Item = table {
  // 每个 Item 都有一个唯一的 ID，用于引用该对象。
  1: id uint64;
  2: name string;
  3: price_in_cents uint32;
  4: quantity_in_stock: uint32;
}

type StoreError = strict enum {
  ITEM_NOT_FOUND = 1;
  INVALID_QUANTITY = 2;
};

protocol Store {
  // 向商店添加新商品。
  // 无返回，所以该操作是异步的，可以静默失败。
  AddItem(struct {
    item: Item;
  });

  // 通过 ID 设置现有商品的价格。
  // 如果找不到该商品，则失败。
  SetPrice(struct {
    item_id: uint64;
    new_price: uint32;
  }) error StoreError;

  // 添加（或减去）一种商品的额外库存。
  // 如果找不到该商品或者您留下无效的商品数量，则失败。
  AddStock(struct {
    item_id: uint64;
    additional_quantity: int32;
  }) error StoreError;

  // 创建新的 Cart（购物车）接口，用于在商店购物。
  // 请注意，这需要一个“资源”（resource）结构，因为请求是一个 Zircon 句柄。
  CreateCart(resource struct {
    request: server_end:Cart;
  });
};

type CartError = strict enum {
  PAYMENT_FAILURE = 1;
  NOT_ENOUGH_IN_STOCK = 2;
};

// Cart 使用构建器模式创建一组商品并以原子方式结帐。
protocol Cart {
  // 通过 ID 添加特定数量的一种商品到购物车。
  AddItem(struct {
    item_id: uint64;
    quantity: uint32;
  });

  // 将优惠券代码（coupon code）添加到购物车。
  AddCouponCode(struct {
    code: string;
  });

  // 以原子方式将所有以前添加的商品结帐。
  // 如果付款失败或库存商品不足以满足请求，则失败。
  Checkout() error CartError;
};
```

<!-- 
```cpp
// Pseudo-code for interacting with the store.

StoreProxy store = connect_to<Store>();
store.AddItem(Item{.id = 1, .name = "Fuchsia Coffee Mug", .price_in_cents = 1299, .quantity_in_stock = 30});
store.AddItem(Item{.id = 2, .name = "Fuchsia Blanket", .price_in_cents = 3499, .quantity_in_stock = 10});
store.SetPrice({.item_id = 2, .new_price = 2999});
store.AddStock({.item_id = 1, .additional_quantity = -10});

CartProxy cart;
store.CreateCart(cart.NewRequest());
cart.AddItem({.item_id = 2, .quantity = 1});
cart.AddItem({.item_id = 1, .quantity = 5});
cart.AddCouponCode("FUCHSIA-ROCKS");
cart.Checkout();
```
 -->
```cpp
// 与商店（store）交互的伪代码。

StoreProxy store = connect_to<Store>();
store.AddItem(Item{.id = 1, .name = "Fuchsia Coffee Mug", .price_in_cents = 1299, .quantity_in_stock = 30});
store.AddItem(Item{.id = 2, .name = "Fuchsia Blanket", .price_in_cents = 3499, .quantity_in_stock = 10});
store.SetPrice({.item_id = 2, .new_price = 2999});
store.AddStock({.item_id = 1, .additional_quantity = -10});

CartProxy cart;
store.CreateCart(cart.NewRequest());
cart.AddItem({.item_id = 2, .quantity = 1});
cart.AddItem({.item_id = 1, .quantity = 5});
cart.AddCouponCode("FUCHSIA-ROCKS");
cart.Checkout();
```

<!--
The component implementing the `Store` interface is responsible for
maintaining the set of items according to the protocol's contract.
 -->
实现 `Store` 接口的组件负责根据协议的合约（contract）维护一组商品。

* {C++}

<!-- 
  ```cpp

  // Create a plain old data type for Item.
  struct Item {
    uint64_t id;
    std::string name;
    uint32_t price_in_cents;
    uint32_t quantity_in_stock;
  };

  // Enumerate the return values of cart operations.
  enum CartResult {
    OK = 0,
    PAYMENT_FAILURE = 1,
    NOT_ENOUGH_IN_STOCK = 2,
  };

  class Cart {
    public:
      // Cart is owned by a store, and it requires the pointer back to
      // its owner to implement Checkout.
      Cart(Store* store);

      // Adding items and coupon codes cannot fail.
      void AddItem(uint64_t item_id, uint32_t quantity);
      void AddCouponCode(std::string code);

      // Perform the checkout operation by acting upon store_ in some way.
      CartResult Checkout();

    private:
      // Create an inner class representing the pair of item id and quantity.
      struct ItemQuantity {
        uint64_t item_id;
        uint32_t quantity;
      };

      // The parent store, not owned.
      Store* store_;
      std::vector<ItemQuantity> item_requests_;
      std::vector<std::string> coupon_codes_;
  };

  // Enumerate return values of store operations.
  enum StoreResult {
    OK = 0,
    ITEM_NOT_FOUND = 1,
  };

  class Store {
    public:
      // Add new items to the store.
      void AddItem(Item item);

      // Set properties of items based on id.
      StoreResult SetPrice(uint64_t item_id, uint32_t new_price);
      StoreResult AddStock(uint64_t item_id, int32_t additional_quantity);

      // Create a new Cart for this store, referencing the Store that owns the Cart.
      // Carts are owned by a store, and must be deleted before the Store is.
      Cart* CreateCart() {
        carts_.emplace_back(Cart(this));
        return &cards_.back();
      }
    private:
      std::vector<Item> items_;
      std::vector<Cart> carts_;
  };
  ```
 -->
  ```cpp

  // 创建 Item 的普通旧数据类型。
  struct Item {
    uint64_t id;
    std::string name;
    uint32_t price_in_cents;
    uint32_t quantity_in_stock;
  };

  // 枚举购物车操作的返回值。
  enum CartResult {
    OK = 0,
    PAYMENT_FAILURE = 1,
    NOT_ENOUGH_IN_STOCK = 2,
  };

  class Cart {
    public:
      // Cart 由商店拥有，并需要指回其所有者的指针才能实现 Checkout。
      Cart(Store* store);

      // 添加商品和优惠券代码不能失败。
      void AddItem(uint64_t item_id, uint32_t quantity);
      void AddCouponCode(std::string code);

      // 通过以某种方式作用于 store_ 来执行结帐操作。
      CartResult Checkout();

    private:
      // 创建表示商品 ID-数量对的内部类。
      struct ItemQuantity {
        uint64_t item_id;
        uint32_t quantity;
      };

      // 父商店，未被拥有。
      Store* store_;
      std::vector<ItemQuantity> item_requests_;
      std::vector<std::string> coupon_codes_;
  };

  // 枚举商店操作的返回值。
  enum StoreResult {
    OK = 0,
    ITEM_NOT_FOUND = 1,
  };

  class Store {
    public:
      // 向商店添加新商品。
      void AddItem(Item item);

      // 基于 ID 设置商品属性。
      StoreResult SetPrice(uint64_t item_id, uint32_t new_price);
      StoreResult AddStock(uint64_t item_id, int32_t additional_quantity);

      // 为该商店创建新 Cart，引用了拥有 Cart 的 Store。
      // Cart 由商店所有，并必须在 Store 之前删除。
      Cart* CreateCart() {
        carts_.emplace_back(Cart(this));
        return &cards_.back();
      }
    private:
      std::vector<Item> items_;
      std::vector<Cart> carts_;
  };
  ```

* {Dart}

<!-- 
  ```dart

  // Create a class containing the data for items.
  class Item {
    final int id;
    final String name;
    int priceInCents;
    int quantityInStock;

    Item({
      required this.id,
      required this.name,
      required this.priceInCents,
      this.quantityInStock = 0
    });
  }

  // Since Dart doesn't have tuples, create a pair type for id and quantity.
  class ItemQuantity {
    final int itemId;
    int quantity;

    ItemQuantity({required this.itemId, required this.quantity});
  }

  // Represent the various results for cart operations.
  enum CartResult {
    ok,
    paymentFailure,
    notEnoughInStock,
  }

  class Cart {
    final Store store;

    final List<ItemQuantity> _items = [];
    final List<String> _couponCodes = [];

    // A Cart needs to refer back to its Store to implement Checkout.
    Cart({required this.store});

    void AddItem(int itemId, int quantity) {
      _items.add(ItemQuantity(itemId: itemId, quantity: quantity);
    }

    void AddCouponCode(String code) {
      _couponCodes.add(code);
    }

    CartResult Checkout() { /* ... */ }
  }

  // Represent the results for store operations.
  enum StoreResult {
    ok,
    itemNotFound,
  }

  class Store {
    final List<Item> _items = [];
    final List<Cart> _carts = [];

    void AddItem(Item item) { _items.add(item); }

    StoreResult SetPrice(int item_id, int new_price) { /* ... */ }
    StoreResult AddStock(int item_id, int additional_quantity) { /* ... */ }

    // Create a cart that refers back to this owning store.
    Cart CreateCart() {
      var ret = Cart(this);
      _carts.add(ret);
      return ret;
    }
  }
  ```
 -->
  ```dart

  // 创建包含商品数据的类。
  class Item {
    final int id;
    final String name;
    int priceInCents;
    int quantityInStock;

    Item({
      required this.id,
      required this.name,
      required this.priceInCents,
      this.quantityInStock = 0
    });
  }

  // 由于 Dart 中没有元组（tuple），因此为 ID 和数量创建成对类型。
  class ItemQuantity {
    final int itemId;
    int quantity;

    ItemQuantity({required this.itemId, required this.quantity});
  }

  // 表示购物车操作的各种结果。
  enum CartResult {
    ok,
    paymentFailure,
    notEnoughInStock,
  }

  class Cart {
    final Store store;

    final List<ItemQuantity> _items = [];
    final List<String> _couponCodes = [];

    // Cart 需要引用回其 Store 以实现 Checkout。
    Cart({required this.store});

    void AddItem(int itemId, int quantity) {
      _items.add(ItemQuantity(itemId: itemId, quantity: quantity);
    }

    void AddCouponCode(String code) {
      _couponCodes.add(code);
    }

    CartResult Checkout() { /* ... */ }
  }

  // 表示商店操作的结果。
  enum StoreResult {
    ok,
    itemNotFound,
  }

  class Store {
    final List<Item> _items = [];
    final List<Cart> _carts = [];

    void AddItem(Item item) { _items.add(item); }

    StoreResult SetPrice(int item_id, int new_price) { /* ... */ }
    StoreResult AddStock(int item_id, int additional_quantity) { /* ... */ }

    // 创建引用回其拥有者商店的购物车。
    Cart CreateCart() {
      var ret = Cart(this);
      _carts.add(ret);
      return ret;
    }
  }
  ```

* {Rust}

<!-- 
  ```rust

  // Create a data struct for Item information.
  pub struct Item {
    pub id: u64,
    pub name: String,
    pub price_in_cents: u32,
    pub quantity_in_stock: u32,
  }

  pub struct Cart {
    // Carts need to act on their parent Store, but we want to avoid cyclic references.
    // Use a Weak pointer so that the Store can be deleted independent of its Carts.
    // Mutex is used for interior mutability.
    store: Weak<Mutex<Store>>,
    items: Vec<(u64, u32)>,
    coupon_codes: Vec<String>,
  }

  impl Cart {
    pub fn new(store: Weak<Mutex<Store>>) -> Self {
      Self {
        store,
        items: vec![],
        coupon_codes: vec![],
      }
    }

    pub fn add_item(&mut self, item_id: u64, quantity: u32) {
      self.items.push((item_id, quantity));
    }

    pub fn add_coupon_code(&mut self, code: String) {
      self.coupon_codes.push(code);
    }

    // Checkout consumes the Cart builder and returns the result.
    pub fn checkout(self) -> Result<(), Error> { /* ... */ }
  }

  pub struct Store {
    items: Vec<Item>,

    // Note that we do not need to maintain ownership over Carts, since
    // they can exist independent of the Store they are from. Checkout will
    // presumably fail if the Store was deleted before it is called.
  }

  impl Store {
    pub fn new() -> Arc<Mutex<Self>> {
      Arc::new(Mutex::new(Self {
        items: vec![],
        carts: vec![],
      }));
    }

    pub fn add_item(&mut self, item: Item) { items.push(item); }
    pub fn set_price(&mut self, item_id: u64, new_price: u32) -> Result<(), Error> { /* ... */ }
    pub fn add_stock(&mut self, item_id: u64, additional_quantity: i32) -> Result<(), Error> { /* ... */ }

    pub fn create_cart(self: Arc<Mutex<Self>>) -> Cart {
      Cart::new(self.downgrade())
    }
  }
  ```
 -->
  ```rust

  // 为 Item 信息创建数据结构。
  pub struct Item {
    pub id: u64,
    pub name: String,
    pub price_in_cents: u32,
    pub quantity_in_stock: u32,
  }

  pub struct Cart {
    // Cart 需要作用于其父 Store，但我们希望避免循环引用。
    // 使用一个弱指针，这样 Store 就可以独立于其 Cart 被删除。
    // Mutex 用于内部可变性。
    store: Weak<Mutex<Store>>,
    items: Vec<(u64, u32)>,
    coupon_codes: Vec<String>,
  }

  impl Cart {
    pub fn new(store: Weak<Mutex<Store>>) -> Self {
      Self {
        store,
        items: vec![],
        coupon_codes: vec![],
      }
    }

    pub fn add_item(&mut self, item_id: u64, quantity: u32) {
      self.items.push((item_id, quantity));
    }

    pub fn add_coupon_code(&mut self, code: String) {
      self.coupon_codes.push(code);
    }

    // Checkout 使用 Cart 构建器并返回结果。
    pub fn checkout(self) -> Result<(), Error> { /* ... */ }
  }

  pub struct Store {
    items: Vec<Item>,

    // 请注意，我们不需要维护对 Cart 的所有权，因为购物车可以独立于其来源的商店而存在。
    // 如果 Store 在调用之前被删除，那么结帐可能会失败。
  }

  impl Store {
    pub fn new() -> Arc<Mutex<Self>> {
      Arc::new(Mutex::new(Self {
        items: vec![],
        carts: vec![],
      }));
    }

    pub fn add_item(&mut self, item: Item) { items.push(item); }
    pub fn set_price(&mut self, item_id: u64, new_price: u32) -> Result<(), Error> { /* ... */ }
    pub fn add_stock(&mut self, item_id: u64, additional_quantity: i32) -> Result<(), Error> { /* ... */ }

    pub fn create_cart(self: Arc<Mutex<Self>>) -> Cart {
      Cart::new(self.downgrade())
    }
  }
  ```

* {Java}

<!-- 
  ```java
  // Create a class containing the data for items.
  public class Item {
    public int id;
    public String name;
    public int priceInCents;
    public int quantityInStock;
  }

  // Since Java doesn't have tuples, create a pair type for id and quantity.
  class ItemQuantity {
    public int item_id;
    public int quantity;

    public ItemQuantity(int item_id, int quantity) {
      this.item_id = item_id;
      this.quantity = quantity;
    }
  }

  // Represent the various results for cart operations.
  public enum CartResult {
    OK,
    PAYMENT_FAILURE,
    NOT_ENOUGH_IN_STOCK,
  }

  // Represent the results for store operations.
  enum StoreResult {
    ok,
    itemNotFound,
  }

  class Store {
    private final List<Item> items = new ArrayList<Item>();
    private final List<Cart> carts = new ArrayList<Cart>();

    public void AddItem(Item item) { items.add(item); }

    public StoreResult SetPrice(int item_id, int new_price) { /* ... */ }
    public StoreResult AddStock(int item_id, int additional_quantity) { /* ... */ }

    public Cart CreateCart() {
      Cart ret = new Cart();
      carts.add(ret);
      return ret;
    }

    // Inner classes in Java can refer to their containing class.
    // This is needed to Checkout can act upon Store.this.
    public class Cart {
      private final List<ItemQuantity> items = new ArrayList<ItemQuantity>();
      private final List<String> couponCodes = new ArrayList<String>();

      void AddItem(int item_id, int quantity) {
        _items.add(ItemQuantity(item_id, quantity));
      }

      void AddCouponCode(String code) {
        _couponCodes.add(code);
      }

      CartResult Checkout() { /* ... */ }
    }
  }

  ```
 -->
  ```java
  // 创建包含商品数据的类。
  public class Item {
    public int id;
    public String name;
    public int priceInCents;
    public int quantityInStock;
  }

  // 由于 Java 中没有元组（tuple），因此为 ID 和数量创建成对类型。
  class ItemQuantity {
    public int item_id;
    public int quantity;

    public ItemQuantity(int item_id, int quantity) {
      this.item_id = item_id;
      this.quantity = quantity;
    }
  }

  // 表示购物车操作的各种结果。
  public enum CartResult {
    OK,
    PAYMENT_FAILURE,
    NOT_ENOUGH_IN_STOCK,
  }

  // 表示商店操作的结果。
  enum StoreResult {
    ok,
    itemNotFound,
  }

  class Store {
    private final List<Item> items = new ArrayList<Item>();
    private final List<Cart> carts = new ArrayList<Cart>();

    public void AddItem(Item item) { items.add(item); }

    public StoreResult SetPrice(int item_id, int new_price) { /* ... */ }
    public StoreResult AddStock(int item_id, int additional_quantity) { /* ... */ }

    public Cart CreateCart() {
      Cart ret = new Cart();
      carts.add(ret);
      return ret;
    }

    // Java 中的内部类可以引用其包含类。
    // 这是 Checkout 可以对 Store.this 进行操作所必需的。
    public class Cart {
      private final List<ItemQuantity> items = new ArrayList<ItemQuantity>();
      private final List<String> couponCodes = new ArrayList<String>();

      void AddItem(int item_id, int quantity) {
        _items.add(ItemQuantity(item_id, quantity));
      }

      void AddCouponCode(String code) {
        _couponCodes.add(code);
      }

      CartResult Checkout() { /* ... */ }
    }
  }

  ```

<!--
## Capability routing as dependency injection
 -->
## 能力路由作为依赖注入

<!--
[Dependency Injection][dependency-injection] is a technique in which an
object's dependencies are passed as arguments to the object rather than
constructed or found by the object itself. This gives the creator of the
object control over the implementation of behavior the object depends
on. It is especially powerful to allow testing of objects that interact
with external services without actually calling those services in test
settings. One popular usage of dependency injection is passing a Time
interface to objects that would otherwise read the system clock. The
caller then has the ability to pass in an implementation that provides
a fixed time value for testing, and in production they pass in an
implementation that reads the real time.
 -->
[依赖注入][dependency-injection]是一种技术，其中对象的依赖项作为参数传递给对象，而非由对象本身构造或查找。这使对象的创建者可以控制对象所依赖行为的实现。它非常强大，允许测试与外部服务交互的对象，而无需在测试设置中实际调用这些服务。依赖注入的一种流行用法是将时间（Time）接口传递给读取系统时钟的对象。之后，调用者可以传入一个为测试提供固定时间值的实现，而在生产环境中，他们可以传入一个读取实时时间的实现。

<!--
The use of protocols between components fundamentally builds on top
of dependency injection techniques. Each component explicitly defines
the protocols it `uses`, and these protocols must be provided for the
component to be instantiated (similar to how OOP classes declare all
needed dependencies in their constructor).
 -->
组件之间的协议的使用根本上构建在依赖注入技术之上。每个组件显式定义了它所“使用”的协议，并且必须将这些协议提供给组件，以进行实例化（类似于 OOP 类声明其构造函数中的所有必要依赖项）。

<!--
Unlike some dependency injection frameworks where dependencies can be
constructed by a registry, all capabilities are explicitly routed from
sources to destinations by component manifests. Previous sections of this
document show how a component can implement an interface by `exposing` a
protocol. This gives the parent of that component the ability to `offer`
that protocol to other components in order to satisfy their dependencies
(the protocols they `use`).
 -->
不同于某些依赖注入框架可以通过注册表项来构造依赖关系，所有能力均通过组件清单显式地从源路由到目标。本文档中前文展示了组件通过“公开”（expose）协议的方式实现接口的方法。这使得该组件的父组件可以将该协议“提供”给其他组件，以满足其依赖关系（其“使用”的协议）。

<!--
The reasons for constructing components in this manner are similar to
that of OOP dependency injection: dependent behavior can be swapped as
needed for testing, evolution, and extensibility.
 -->
以这种方式构造组件的原因与 OOP 依赖注入产生的原因类似：可以根据测试、演化和扩展性需要来交换依赖行为。

<!--
### Example
 -->
### 示例

<!--
This example implements a `Purchaser` that needs to process credit
cards as part of a purchase flow. In some settings (such as testing)
you don't want to actually charge credit cards (which could get
very expensive)! Instead, we will provide a `CreditCardCharger` that
`Purchaser` uses to charge credit cards. In testing scenarios we provide
a fake `CreditCardCharger` that doesn't actually charge cards.
 -->
本示例实现了在购买流程中需要处理信用卡的 `Purchaser`（买家）。在某些设置（例如测试）中，您不想信用卡产生实际扣费（代价可能非常高昂）！相反，我们将提供一个由 `Purchaser` 用来对信用卡扣费的 `CreditCardCharger`（信用卡扣费器）。在测试场景中，我们提供了不会产生实际卡片扣费的假 `CreditCardCharger`。

<!--
Note: A fake or mock implementation can store the values of
parameters so they can be verified. If you were to use something like
`charge_credit_card = false` and forgot to set the parameter, you could
cause some expensive bugs.
 -->
注意：伪造或模拟实现可以存储参数值，以便对其进行验证。您如果要使用例如 `charge_credit_card = false` 之类而忘记设置参数，那么可能会导致一些代价高昂的故障。

<!-- 
```fidl
// fuchsia.store.fidl

type PurchaseError = enum {
  CREDIT_CARD_FAILURE = 1;
  ITEM_NOT_FOUND = 2;
};

protocol Purchaser {
  // Purchase an item by name with a specific credit card.
  // Fails if the item is not found or if the credit card failed to charge.
  Purchase(struct {
    item_name: string,
    credit_card: string,
  }) error PurchaseError;
};

protocol CreditCardCharger {
  // Charge a specific credit card a specific amount.
  // Returns whether the charge is successful.
  Charge(struct {
    credit_card: string,
    amount: int,
  }) -> (struct { success: bool });
};
```
 -->
```fidl
// fuchsia.store.fidl

type PurchaseError = enum {
  CREDIT_CARD_FAILURE = 1;
  ITEM_NOT_FOUND = 2;
};

protocol Purchaser {
  // 利用特定信用卡根据名称购买商品。
  // 如果找不到商品或信用卡无法扣费，则失败。
  Purchase(struct {
    item_name: string,
    credit_card: string,
  }) error PurchaseError;
};

protocol CreditCardCharger {
  // 将特定信用卡扣除特定数额费用。
  // 返回扣费是否成功。
  Charge(struct {
    credit_card: string,
    amount: int,
  }) -> (struct { success: bool });
};
```

<!-- 
```json5
// purchaser.cml
{
  program: {
    // Instructions for how to run this component.
    /* ... */
  },

  capabilities: [
    { protocol: "fuchsia.store.Purchaser" }
  ],

  // Purchaser is a public interface implemented by this component.
  expose: [
    {
      protocol: "fuchsia.store.Purchaser",
      from: "self",
    }
  ],

  // CreditCardCharger is an interface required by this component to function.
  use: [
    { protocol: "fuchsia.store.CreditCardCharger" }
  ]
}
```
 -->
```json5
// purchaser.cml
{
  program: {
    // 运行该组件方法的说明。
    /* ... */
  },

  capabilities: [
    { protocol: "fuchsia.store.Purchaser" }
  ],

  // Purchaser 是由该组件实现的公共接口。
  expose: [
    {
      protocol: "fuchsia.store.Purchaser",
      from: "self",
    }
  ],

  // CreditCardCharger 是该组件运转所需的接口。
  use: [
    { protocol: "fuchsia.store.CreditCardCharger" }
  ]
}
```

<!-- 
```json5
// real_credit_card_charger.cml
// Implements CreditCardCharger and actually charges credit cards.
{
  // ...
  capabilities: [
    { protocol: "fuchsia.store.CreditCardCharger" }
  ],

  expose: [
    {
      protocol: "fuchsia.store.CreditCardCharger",
      from: "self",
    }
  ],
}
```
 -->
```json5
// real_credit_card_charger.cml
// 实现 CreditCardCharger，并对信用卡实际扣费。
{
  // ...
  capabilities: [
    { protocol: "fuchsia.store.CreditCardCharger" }
  ],

  expose: [
    {
      protocol: "fuchsia.store.CreditCardCharger",
      from: "self",
    }
  ],
}
```

<!-- 
```json5
// fake_credit_card_charger.cml
// Implements CreditCardCharger, but does not really charge anything.
{
  // ...
  capabilities: [
    {
      protocol: [
        "fuchsia.store.CreditCardCharger",
        // Interface to control the output of this fake component (FIDL not pictured here).
        "fuchsia.store.testing.CreditCardChargerController",
      ]
    }
  ],

  expose: [
    {
      protocol: [
        "fuchsia.store.CreditCardCharger",
        "fuchsia.store.testing.CreditCardChargerController",
      ],
      from: "self",
    }
  ],
}
```
 -->
```json5
// fake_credit_card_charger.cml
// 实现 CreditCardCharger，但并不对信用卡实际扣费。
{
  // ...
  capabilities: [
    {
      protocol: [
        "fuchsia.store.CreditCardCharger",
        // 控制该虚假组件输出的接口（此处未描述 FIDL）。
        "fuchsia.store.testing.CreditCardChargerController",
      ]
    }
  ],

  expose: [
    {
      protocol: [
        "fuchsia.store.CreditCardCharger",
        "fuchsia.store.testing.CreditCardChargerController",
      ],
      from: "self",
    }
  ],
}
```

<!-- 
```json5
// core.cml
//
// Actually add a "Purchaser" to the system, its dependencies, and route
// its protocol to some component implementing a purchase flow.
{
  children: [
    // ...
    {
      name: "purchaser",
      url: "fuchsia-pkg://fuchsia.com/purchaser#meta/purchaser.cml",
    },
    {
      // We want to use the real credit card charger so that we actually charge customers.
      name: "credit_card_charger"
      url: "fuchsia-pkg://fuchsia.com/real_credit_card_charger#meta/real_credit_card_charger.cml",
    },
    {
      name: "real_graphical_purchase_flow"
      url: /* ... */,
    },
  ],
  offer: [
    // Route protocols to satisfy every component's dependencies.
    {
      protocol: "fuchsia.store.CreditCardCharger",
      from: "#credit_card_charger",
      to: "#purchaser",
    },
    {
      protocol: "fuchsia.store.Purchaser",
      from: "#purchaser",
      to: "#real_graphical_purchase_flow",
    },
  ]
}
```
 -->
```json5
// core.cml
//
// 向系统实际添加一个“买家”（Purchaser）及其依赖项，并将其协议路由至实现购买流程的某个组件。
// Actually add a "Purchaser" to the system, its dependencies, and route
// its protocol to some component implementing a purchase flow.
{
  children: [
    // ...
    {
      name: "purchaser",
      url: "fuchsia-pkg://fuchsia.com/purchaser#meta/purchaser.cml",
    },
    {
      // 我们希望使用真正的信用卡扣费器，以便实际对顾客收费。
      name: "credit_card_charger"
      url: "fuchsia-pkg://fuchsia.com/real_credit_card_charger#meta/real_credit_card_charger.cml",
    },
    {
      name: "real_graphical_purchase_flow"
      url: /* ... */,
    },
  ],
  offer: [
    // 将协议路由以满足每个组件的依赖关系。
    {
      protocol: "fuchsia.store.CreditCardCharger",
      from: "#credit_card_charger",
      to: "#purchaser",
    },
    {
      protocol: "fuchsia.store.Purchaser",
      from: "#purchaser",
      to: "#real_graphical_purchase_flow",
    },
  ]
}
```

<!-- 
```json5
// test_purchaser.cml
{
  children: [
    {
      // We're going to test the real purchaser component, which is safe since we are mocking its dependency.
      name: "purchaser",
      url: "fuchsia-pkg://fuchsia.com/purchaser#meta/purchaser.cml",
    },
    {
      // We want to use the fake credit card charger so that we don't actually charge cards in tests.
      name: "credit_card_charger"
      url: "fuchsia-pkg://fuchsia.com/fake_credit_card_charger#meta/fake_credit_card_charger.cml",
    },
  ],
  offer: [
    {
      // Inject the fake charger as a dependency to purchaser.
      protocol: "fuchsia.store.CreditCardCharger",
      from: "#credit_card_charger",
      to: "#purchaser",
    }
  ],
  use: [
    {
      // Use Purchaser so we can test it
      protocol: "fuchsia.store.Purchaser",
      from: "#purchaser",
    },
    {
      // Use test charger so we can control what the credit card charger returns.
      protocol: "fuchsia.store.testing.CreditCardChargerController",
      from: "#credit_card_charger",
    },
  ]
}
```
 -->
```json5
// test_purchaser.cml
{
  children: [
    {
      // 我们将要测试真正的买家组件，这是安全的，因为我们正在模拟其依赖关系。
      name: "purchaser",
      url: "fuchsia-pkg://fuchsia.com/purchaser#meta/purchaser.cml",
    },
    {
      // 我们希望使用虚假信用卡扣费器，以便在测试中不会产生实际的卡片扣费。
      name: "credit_card_charger"
      url: "fuchsia-pkg://fuchsia.com/fake_credit_card_charger#meta/fake_credit_card_charger.cml",
    },
  ],
  offer: [
    {
      // 注入虚假扣费器作为买家的依赖。
      protocol: "fuchsia.store.CreditCardCharger",
      from: "#credit_card_charger",
      to: "#purchaser",
    }
  ],
  use: [
    {
      // 使用 Purchaser 以便测试
      protocol: "fuchsia.store.Purchaser",
      from: "#purchaser",
    },
    {
      // 使用测试扣费器，从而我们可以控制信用卡扣费器的返回值。
      protocol: "fuchsia.store.testing.CreditCardChargerController",
      from: "#credit_card_charger",
    },
  ]
}
```

<!-- 
```cpp
// Pseudo-code for test_purchaser

PurchaserProxy purchaser = open_service<Purchaser>();
CreditCardChargerController charger = open_service<CreditCardChargerController>();

// Make the card charger always return true, then test successful charge for an existing item.
charger.SetChargeResponse(true);
assert(purchaser.Purchase("existing item", "fake-card"), isNotError);

// Now test what happens when an item is missing.
// Depending on how advanced the mock charger is, we could even check
// that it was not called as a result of this invalid Purchase call.
assert(purchaser.Purchase("missing item", "fake-card"), PurchaseError.ITEM_NOT_FOUND);

// Make the charger return false and try again with an existing item.
// This allows us to test our error handling code paths.
charger.SetChargeResponse(false);
assert(purchaser.Purchase("existing item", "fake-card"), PurchaseError.CREDIT_CARD_FAILURE);
```
 -->
```cpp
// test_purchaser 的伪代码

PurchaserProxy purchaser = open_service<Purchaser>();
CreditCardChargerController charger = open_service<CreditCardChargerController>();

// 使卡片扣费器总返回真，之后对已有商品测试扣费是否成功。
charger.SetChargeResponse(true);
assert(purchaser.Purchase("existing item", "fake-card"), isNotError);

// 现在测试商品缺失时会发生什么情况。
// 根据模拟扣费器的高级程度，我们甚至可以检查出是购买（Purchase）调用的无效导致了其未被调用。
assert(purchaser.Purchase("missing item", "fake-card"), PurchaseError.ITEM_NOT_FOUND);

// 使卡片扣费器返回假并再次对已有商品测试。
// 这允许我们测试错误处理代码路径。
charger.SetChargeResponse(false);
assert(purchaser.Purchase("existing item", "fake-card"), PurchaseError.CREDIT_CARD_FAILURE);
```

<!--
The above system would be implemented in an OOP language as follows:
 -->
上述系统使用 OOP 语言将实现为以下内容：

* {C++}

<!-- 
  ```cpp
  class Purchaser final {
    public:
      // Purchaser takes as input the credit card charger to use.
      Purchaser(CreditCardCharger* credit_card_charger) :
        credit_card_charger_(credit_card_charger) {}
      PurchaseError Purchase(std::string item_name, std::string credit_card) {
        /* ... */
        // Use the injected credit card charger when needed.
        credit_card_charger_->Charge(std::move(credit_card), /* amount */);
        /* ... */
      }
    private:
      CreditCardCharger* credit_card_charger_;
  };

  // Abstract base class for concrete credit card chargers.
  class CreditCardCharger {
    public:
      virtual bool Charge(std::string credit_card, int amount) = 0;
  };

  class RealCreditCardCharger : public CreditCardCharger {
    public:
      bool Charge(std::string credit_card, int amount) override {
        /* actually charge credit cards somehow */
      }
  };

  class MockCreditCardCharger : public CreditCardCharger {
    public:
      // Mock implementation of CreditCardCharger::Charge that returns
      // a configurable error value and records the arguments of its
      // previous call.
      bool Charge(std::string credit_card, int amount) override {
        calls_++;
        last_credit_card_ = std::move(credit_card);
        last_amount_ = amount;
        return return_value_;
      }

      // Set the value that will be returned when calling Charge
      void SetReturnValue(bool return_value) { return_value_ = return_value; }

      // Get the parameters of the last call to Charge.
      const std::string& GetLastCreditCard() const { return last_credit_card_; }
      int GetLastAmount() const { return last_amount_; }
      size_t GetCallCount() const { return calls_; }

    private:
      bool return_value_ = true;
      size_t calls_ = 0;
      std::string last_credit_card_;
      int last_amount_ = 0;
  };


  // Production code
  int main() {
    auto charger = std::make_unique<RealCreditCardCharger>();
    Purchaser purchaser(charger.get());
    // use purchaser in the program flow

    /* ... */
  }

  // Test code (assuming GoogleTest)
  TEST(Purchaser, Success) {
    // Test that a purchase can succeed.
    // We expect that when a purchase is completed for an item costing
    // $100 that the CreditCardCharger is called with amount = 100.
    auto charger = std::make_unique<MockCreditCardCharger>();
    Purchaser purchaser(charger.get());
    EXPECT_EQ(PurchaseResult::OK, purchaser.Purchase("Item costing $100", "1234567890"));
    EXPECT_EQ(1, charger->GetCallCount());
    EXPECT_EQ("1234567890", charger->GetLastCreditCard());
    EXPECT_EQ(100, charger->GetLastAmount());
  }

  TEST(Purchaser, ItemNotFound) {
    // Test that we do not actually try to charge a credit card if the item is not found.
    auto charger = std::make_unique<MockCreditCardCharger>();
    Purchaser purchaser(charger.get());
    EXPECT_EQ(PurchaseResult::ITEM_NOT_FOUND, purchaser.Purchase("Not found item", "1234567890"));
    EXPECT_EQ(0, charger->GetCallCount());
  }

  TEST(Purchaser, CardChargeFailure) {
    // Test that a purchase can fail.
    auto charger = std::make_unique<MockCreditCardCharger>();
    Purchaser purchaser(charger.get());
    charger->SetReturnValue(false);
    EXPECT_EQ(PurchaseResult::CREDIT_CARD_FAILURE,
              purchaser.Purchase("Item costing $100", "1234567890"));
    EXPECT_EQ(1, charger->GetCallCount());
    EXPECT_EQ("1234567890", charger->GetLastCreditCard());
    EXPECT_EQ(100, charger->GetLastAmount());
  }
  ```
 -->
  ```cpp
  class Purchaser final {
    public:
      // Purchaser 将需要使用的信用卡扣费器作为输入。
      Purchaser(CreditCardCharger* credit_card_charger) :
        credit_card_charger_(credit_card_charger) {}
      PurchaseError Purchase(std::string item_name, std::string credit_card) {
        /* ... */
        // 需要时使用注入的信用卡扣费器。
        credit_card_charger_->Charge(std::move(credit_card), /* 数额 */);
        /* ... */
      }
    private:
      CreditCardCharger* credit_card_charger_;
  };

  // 具体信用卡扣费器的抽象基类。
  class CreditCardCharger {
    public:
      virtual bool Charge(std::string credit_card, int amount) = 0;
  };

  class RealCreditCardCharger : public CreditCardCharger {
    public:
      bool Charge(std::string credit_card, int amount) override {
        /* 以某种方式对信用卡实际扣费 */
      }
  };

  class MockCreditCardCharger : public CreditCardCharger {
    public:
      // CreditCardCharger::Charge 的模拟实现，返回一个可配置的错误值及其先前调用参数的记录。
      bool Charge(std::string credit_card, int amount) override {
        calls_++;
        last_credit_card_ = std::move(credit_card);
        last_amount_ = amount;
        return return_value_;
      }

      // 设置调用 Charge 时会返回的值
      void SetReturnValue(bool return_value) { return_value_ = return_value; }

      // 获取对 Charge 最后一次调用的参数。
      const std::string& GetLastCreditCard() const { return last_credit_card_; }
      int GetLastAmount() const { return last_amount_; }
      size_t GetCallCount() const { return calls_; }

    private:
      bool return_value_ = true;
      size_t calls_ = 0;
      std::string last_credit_card_;
      int last_amount_ = 0;
  };


  // 生产代码
  int main() {
    auto charger = std::make_unique<RealCreditCardCharger>();
    Purchaser purchaser(charger.get());
    // 在程序流程中使用 purchaser

    /* ... */
  }

  // 测试代码（假设使用 GoogleTest）
  TEST(Purchaser, Success) {
    // 测试购买能够成功。
    // 我们希望当一件价值 100 美元的商品的购买完成时，CreditCardCharger 被调用，且 amount = 100。
    auto charger = std::make_unique<MockCreditCardCharger>();
    Purchaser purchaser(charger.get());
    EXPECT_EQ(PurchaseResult::OK, purchaser.Purchase("Item costing $100", "1234567890"));
    EXPECT_EQ(1, charger->GetCallCount());
    EXPECT_EQ("1234567890", charger->GetLastCreditCard());
    EXPECT_EQ(100, charger->GetLastAmount());
  }

  TEST(Purchaser, ItemNotFound) {
    // 测试如果找不到该项目，我们实际上不会尝试从信用卡中扣费。
    auto charger = std::make_unique<MockCreditCardCharger>();
    Purchaser purchaser(charger.get());
    EXPECT_EQ(PurchaseResult::ITEM_NOT_FOUND, purchaser.Purchase("Not found item", "1234567890"));
    EXPECT_EQ(0, charger->GetCallCount());
  }

  TEST(Purchaser, CardChargeFailure) {
    // 测试购买可能失败。
    auto charger = std::make_unique<MockCreditCardCharger>();
    Purchaser purchaser(charger.get());
    charger->SetReturnValue(false);
    EXPECT_EQ(PurchaseResult::CREDIT_CARD_FAILURE,
              purchaser.Purchase("Item costing $100", "1234567890"));
    EXPECT_EQ(1, charger->GetCallCount());
    EXPECT_EQ("1234567890", charger->GetLastCreditCard());
    EXPECT_EQ(100, charger->GetLastAmount());
  }
  ```

* {Dart}

<!-- 
  ```dart
  class Purchaser {
    final CreditCardCharger creditCardCharger;

    // Purchaser takes as input the credit card charger to use.
    Purchaser({required this.creditCardCharger});
    PurchaseError Purchase(String itemName, String creditCard) {
      /* ... */
      // Use the injected credit card charger when needed.
      creditCardCharger.Charge(creditCard, /* amount */);
      /* ... */
    }
  };

  // Abstract base class for concrete credit card chargers.
  abstract class CreditCardCharger {
    bool Charge(String creditCard, int amount);
  };

  class RealCreditCardCharger implements CreditCardCharger {
    @override
    bool Charge(String creditCard, int amount) {
      /* actually charge credit cards somehow */
    }
  };

  class MockCreditCardCharger implements CreditCardCharger {
    bool _returnValue = true;
    int _calls = 0;
    String _lastCreditCard = '';
    int _lastAmount = 0;

    // Mock implementation of CreditCardCharger::Charge that returns
    // a configurable error value and records the arguments of its
    // previous call.
    @override
    bool Charge(String creditCard, int amount) {
      _calls++;
      _lastCreditCard = creditCard;
      _lastAmount = amount;
      return _returnValue;
    }

    // Set the value that will be returned when calling Charge
    void set returnValue(int v) {
      _returnValue = v;
    }

    // Get the parameters of the last call to Charge.
    int get calls => _calls;
    String get lastCreditCard => _lastCreditCard;
    int get lastAmount => _lastAmount;
  };


  // Production code
  void main() {
    final charger = RealCreditCardCharger();
    Purchaser purchaser(creditCardCharger: charger);
    // use purchaser in the program flow

    /* ... */
  }

  // Test code (assuming package:test)
  import 'package:test/test.dart';

  void main() {
    group('Purchaser', () {
      test('succeeds', () {
        // Test that a purchase can succeed.
        // We expect that when a purchase is completed for an item costing
        // $100 that the CreditCardCharger is called with amount = 100.
        final charger = MockCreditCardCharger();
        final purchaser = Purchaser(creditCardCharger: charger);

        expect(purchaser.Purchase("Item costing $100", "1234567890"), PurchaseResult.ok);
        expect(charger.calls, 1);
        expect(charger.lastCreditCard, "1234567890");
        expect(charger.amount, 100);
      });

      test('fails when item is not found', () {
        // Test that we do not actually try to charge a credit card if the item is not found.
        final charger = MockCreditCardCharger();
        final purchaser = Purchaser(creditCardCharger: charger);

        expect(purchaser.Purchase("Not found item", "1234567890"), PurchaseResult.itemNotFound);
        expect(charger.calls, 0);
      });

      test('fails when card cannot be charged', () {
        // Test that a purchase can fail.
        final charger = MockCreditCardCharger();
        final purchaser = Purchaser(creditCardCharger: charger);

        charger.returnValue = false;
        expect(purchaser.Purchase("Item costing $100", "1234567890"), PurchaseResult.creditCardFailure);
        expect(charger.calls, 1);
        expect(charger.lastCreditCard, "1234567890");
        expect(charger.amount, 100);
      });
    });
  }
  ```
 -->
  ```dart
  class Purchaser {
    final CreditCardCharger creditCardCharger;

    // Purchaser 将需要使用的信用卡扣费器作为输入。
    Purchaser({required this.creditCardCharger});
    PurchaseError Purchase(String itemName, String creditCard) {
      /* ... */
      // 需要时使用注入的信用卡扣费器。
      creditCardCharger.Charge(creditCard, /* 数额 */);
      /* ... */
    }
  };

  // 具体信用卡扣费器的抽象基类。
  abstract class CreditCardCharger {
    bool Charge(String creditCard, int amount);
  };

  class RealCreditCardCharger implements CreditCardCharger {
    @override
    bool Charge(String creditCard, int amount) {
      /* 以某种方式对信用卡实际扣费 */
    }
  };

  class MockCreditCardCharger implements CreditCardCharger {
    bool _returnValue = true;
    int _calls = 0;
    String _lastCreditCard = '';
    int _lastAmount = 0;

    // CreditCardCharger::Charge 的模拟实现，返回一个可配置的错误值及其先前调用参数的记录。
    @override
    bool Charge(String creditCard, int amount) {
      _calls++;
      _lastCreditCard = creditCard;
      _lastAmount = amount;
      return _returnValue;
    }

    // 设置调用 Charge 时会返回的值
    void set returnValue(int v) {
      _returnValue = v;
    }

    // 获取对 Charge 最后一次调用的参数。
    int get calls => _calls;
    String get lastCreditCard => _lastCreditCard;
    int get lastAmount => _lastAmount;
  };


  // 生产代码
  void main() {
    final charger = RealCreditCardCharger();
    Purchaser purchaser(creditCardCharger: charger);
    // 在程序流程中使用 purchaser

    /* ... */
  }

  // 测试代码（假设使用 package:test）
  import 'package:test/test.dart';

  void main() {
    group('Purchaser', () {
      test('succeeds', () {
        // 测试购买能够成功。
        // 我们希望当一件价值 100 美元的商品的购买完成时，CreditCardCharger 被调用，且 amount = 100。
        final charger = MockCreditCardCharger();
        final purchaser = Purchaser(creditCardCharger: charger);

        expect(purchaser.Purchase("Item costing $100", "1234567890"), PurchaseResult.ok);
        expect(charger.calls, 1);
        expect(charger.lastCreditCard, "1234567890");
        expect(charger.amount, 100);
      });

      test('fails when item is not found', () {
        // 测试如果找不到该项目，我们实际上不会尝试从信用卡中扣费。
        final charger = MockCreditCardCharger();
        final purchaser = Purchaser(creditCardCharger: charger);

        expect(purchaser.Purchase("Not found item", "1234567890"), PurchaseResult.itemNotFound);
        expect(charger.calls, 0);
      });

      test('fails when card cannot be charged', () {
        // 测试购买可能失败。
        final charger = MockCreditCardCharger();
        final purchaser = Purchaser(creditCardCharger: charger);

        charger.returnValue = false;
        expect(purchaser.Purchase("Item costing $100", "1234567890"), PurchaseResult.creditCardFailure);
        expect(charger.calls, 1);
        expect(charger.lastCreditCard, "1234567890");
        expect(charger.amount, 100);
      });
    });
  }
  ```
 -->

* {Rust}

<!-- 
  ```rust
  pub struct Purchaser {
    credit_card_charger: Box<dyn CreditCardCharger>,
  }

  impl Purchaser {
    // Purchaser takes as input the credit card charger to use.
    pub fn new(credit_card_charger: Box<dyn CreditCardCharger>) -> Self {
      Self { credit_card_charger }
    }

    pub fn purchase(&mut self, item_name: String, credit_card: String) {
      /* ... */
      // Use the injected credit card charger when needed.
      self.credit_card_charger.charge(creditCard, /* amount */);
      /* ... */
    }

    // For testing only, allow a Purchaser to be destroyed and converted
    // back to it CreditCardCharger.
    //
    // Alternatively, we could take a non-owning reference to the dependency.
    #[cfg(test)]
    pub fn to_charger(mut self) -> Box<dyn CreditCardCharger> {
      self.credit_card_charger
    }
  }

  // Trait implemented by concrete credit card chargers.
  trait CreditCardCharger {
    fn charge(credit_card: String, amount: i32) -> bool;
  }

  struct RealCreditCardCharger {}

  impl CreditCardCharger for RealCreditCardCharger {
    fn charge(&mut self, credit_card: String, amount: i32) -> bool {
      /* actually charge credit cards somehow */
    }
  };

  // Mock implementation of CreditCardCharger that returns
  // a configurable error value and records the arguments of its
  // previous call.
  pub struct MockCreditCardCharger {
    return_value: bool,
    calls: usize,
    last_credit_card: Option<String>,
    last_amount: Option<i32>,
  }

  impl MockCreditCardCharger {
    pub fn new() -> Self {
      Self {
        return_value: true,
        calls: 0,
        last_credit_card: None,
        last_amount: None,
      }
    }

    // Set the value that will be returned when calling charge
    pub fn set_return_value(&mut self, return_value: bool) {
      self.return_value = return_value;
    }

    // Get the parameters of the last call to charge.

    pub fn get_last_credit_card<'a>(&'a self) -> Option<&'a str> {
      self.last_credit_card.as_deref()
    }

    pub fn get_last_amount(&self) -> Option<i32> {
      self.last_amount.clone()
    }

    pub fn get_calls(&self) -> usize {
      self.calls
    }
  }

  impl CreditCardCharger for MockCreditCardCharger {
    fn charge(&mut self, credit_card: String, amount: i32) -> bool {
      self.calls += 1;
      self.last_credit_card = Some(credit_card);
      self.last_amount = Some(amount);
      self.return_value
    }
  }

  // Production code
  fn main() {
    let mut purchaser = Purchaser::new(Box::new(RealCreditCardCharger::new()));
    // use purchaser in the program flow
    /* ... */
  }

  // Test code (assuming Rust tests)

  #[cfg(test)]
  mod tests {
    #[test]
    fn success() {
      // Test that a purchase can succeed.
      // We expect that when a purchase is completed for an item costing
      // $100 that the CreditCardCharger is called with amount = 100.
      let mut purchaser = Purchaser::new(Box::new(MockCreditCardCharger::new()));
      assert_eq!(purchaser.purchase("Item costing $100", "1234567890"), PurchaseResult::OK);
      let charger = purchaser.to_charger();
      assert_eq!(charger.get_calls(), 1);
      assert_eq!(charger.get_last_credit_card(), Some("1234567890"));
      assert_eq!(charger.get_last_amount, Some(100i32));
    }

    #[test]
    fn item_not_found() {
      // Test that we do not actually try to charge a credit card if the item is not found.
      let mut purchaser = Purchaser::new(Box::new(MockCreditCardCharger::new()));
      assert_eq!(purchaser.purchase("Item costing $100", "1234567890"), PurchaseResult.ok);
      let charger = purchaser.to_charger();

      assert_eq!(purchaser.purchase("Not found item", "1234567890"), PurchaseResult::ITEM_NOT_FOUND);
      let charger = purchaser.to_charger();
      assert_eq!(charger.get_calls(), 0);
    }

    #[test]
    fn card_charge_fails() {
      // Test that a purchase can fail.

      let mut charger = Box::new(MockCreditCardCharger::new());
      charger.set_return_value(false);
      let mut purchaser = Purchaser::new(charger);
      assert_eq!(purchaser.purchase("Item costing $100", "1234567890"), PurchaseResult::CREDIT_CARD_FAILURE);
      let charger = purchaser.to_charger();

      assert_eq!(charger.get_calls(), 1);
      assert_eq!(charger.get_last_credit_card(), Some("1234567890"));
      assert_eq!(charger.get_last_amount, Some(100i32));
    }
  }
  ```
 -->
  ```rust
  pub struct Purchaser {
    credit_card_charger: Box<dyn CreditCardCharger>,
  }

  impl Purchaser {
    // Purchaser 将需要使用的信用卡扣费器作为输入。
    pub fn new(credit_card_charger: Box<dyn CreditCardCharger>) -> Self {
      Self { credit_card_charger }
    }

    pub fn purchase(&mut self, item_name: String, credit_card: String) {
      /* ... */
      // 需要时使用注入的信用卡扣费器。
      self.credit_card_charger.charge(creditCard, /* 数额 */);
      /* ... */
    }

    // 仅用于测试，允许销毁 Purchaser 并将其转换回 CreditCardCharger。
    //
    // 或者，我们可以对依赖项进行非拥有的（non-owning）引用。
    #[cfg(test)]
    pub fn to_charger(mut self) -> Box<dyn CreditCardCharger> {
      self.credit_card_charger
    }
  }

  // 由具体信用卡扣费器实现的特征（trait）。
  trait CreditCardCharger {
    fn charge(credit_card: String, amount: i32) -> bool;
  }

  struct RealCreditCardCharger {}

  impl CreditCardCharger for RealCreditCardCharger {
    fn charge(&mut self, credit_card: String, amount: i32) -> bool {
      /* 以某种方式对信用卡实际扣费 */
    }
  };

  // CreditCardCharger 的模拟实现，返回一个可配置的错误值及其先前调用参数的记录。
  pub struct MockCreditCardCharger {
    return_value: bool,
    calls: usize,
    last_credit_card: Option<String>,
    last_amount: Option<i32>,
  }

  impl MockCreditCardCharger {
    pub fn new() -> Self {
      Self {
        return_value: true,
        calls: 0,
        last_credit_card: None,
        last_amount: None,
      }
    }

    // 设置调用 charge 时会返回的值
    pub fn set_return_value(&mut self, return_value: bool) {
      self.return_value = return_value;
    }

    // 获取对 charge 最后一次调用的参数。

    pub fn get_last_credit_card<'a>(&'a self) -> Option<&'a str> {
      self.last_credit_card.as_deref()
    }

    pub fn get_last_amount(&self) -> Option<i32> {
      self.last_amount.clone()
    }

    pub fn get_calls(&self) -> usize {
      self.calls
    }
  }

  impl CreditCardCharger for MockCreditCardCharger {
    fn charge(&mut self, credit_card: String, amount: i32) -> bool {
      self.calls += 1;
      self.last_credit_card = Some(credit_card);
      self.last_amount = Some(amount);
      self.return_value
    }
  }

  // 生产代码
  fn main() {
    let mut purchaser = Purchaser::new(Box::new(RealCreditCardCharger::new()));
    // 在程序流程中使用 purchaser
    /* ... */
  }

  // 测试代码（假设使用 Rust test）

  #[cfg(test)]
  mod tests {
    #[test]
    fn success() {
      // 测试购买能够成功。
      // 我们希望当一件价值 100 美元的商品的购买完成时，CreditCardCharger 被调用，且 amount = 100。
      let mut purchaser = Purchaser::new(Box::new(MockCreditCardCharger::new()));
      assert_eq!(purchaser.purchase("Item costing $100", "1234567890"), PurchaseResult::OK);
      let charger = purchaser.to_charger();
      assert_eq!(charger.get_calls(), 1);
      assert_eq!(charger.get_last_credit_card(), Some("1234567890"));
      assert_eq!(charger.get_last_amount, Some(100i32));
    }

    #[test]
    fn item_not_found() {
      // 测试如果找不到该项目，我们实际上不会尝试从信用卡中扣费。
      let mut purchaser = Purchaser::new(Box::new(MockCreditCardCharger::new()));
      assert_eq!(purchaser.purchase("Item costing $100", "1234567890"), PurchaseResult.ok);
      let charger = purchaser.to_charger();

      assert_eq!(purchaser.purchase("Not found item", "1234567890"), PurchaseResult::ITEM_NOT_FOUND);
      let charger = purchaser.to_charger();
      assert_eq!(charger.get_calls(), 0);
    }

    #[test]
    fn card_charge_fails() {
      // 测试购买可能失败。

      let mut charger = Box::new(MockCreditCardCharger::new());
      charger.set_return_value(false);
      let mut purchaser = Purchaser::new(charger);
      assert_eq!(purchaser.purchase("Item costing $100", "1234567890"), PurchaseResult::CREDIT_CARD_FAILURE);
      let charger = purchaser.to_charger();

      assert_eq!(charger.get_calls(), 1);
      assert_eq!(charger.get_last_credit_card(), Some("1234567890"));
      assert_eq!(charger.get_last_amount, Some(100i32));
    }
  }
  ```

* {Java}

<!-- 
  ```java
  class Purchaser {
    private CreditCardCharger creditCardCharger;

    // Purchaser takes as input the credit card charger to use.
    public Purchaser(CreditCardCharger creditCardCharger) {
      this.creditCardCharger = creditCardCharger;
    }

    public PurchaseError Purchase(String itemName, String creditCard) {
      /* ... */
      // Use the injected credit card charger when needed.
      creditCardCharger.Charge(creditCard, /* amount */);
      /* ... */
    }
  };

  // Interface for concrete credit card chargers.
  interface CreditCardCharger {
    public boolean Charge(String creditCard, int amount);
  };

  class RealCreditCardCharger implements CreditCardCharger {
    @Override
    boolean Charge(String creditCard, int amount) {
      /* actually charge credit cards somehow */
    }
  };

  class MockCreditCardCharger implements CreditCardCharger {
    private boolean returnValue = true;
    private int calls = 0;
    private String lastCreditCard = '';
    private int lastAmount = 0;

    // Mock implementation of CreditCardCharger::Charge that returns
    // a configurable error value and records the arguments of its
    // previous call.
    @override
    public boolean Charge(String creditCard, int amount) {
      calls++;
      lastCreditCard = creditCard;
      lastAmount = amount;
      return returnValue;
    }

    // Set the value that will be returned when calling Charge
    public void setReturnValue(int v) {
      returnValue = v;
    }

    // Get the parameters of the last call to Charge.
    public int getCalls() { return calls; }
    public String getLastCreditCard() { return lastCreditCard; }
    public int getLastAmount() { return lastAmount; }
  };


  // Production code
  void main() {
    CreditCardCharger charger = new RealCreditCardCharger();
    Purchaser purchaser = new Purchaser(charger);
    // use purchaser in the program flow

    /* ... */
  }

  // Test code (assuming JUnit)

  public class PurchaserTest extends TestCase {
    protected MockCreditCardCharger charger;
    protected Purchaser purchaser;

    protected void setUp() {
      charger = new MockCreditCardCharger();
      purchaser = new Purchaser(charger);
    }

    public void testPurchaseSucceeds() {
      // Test that a purchase can succeed.
      // We expect that when a purchase is completed for an item costing
      // $100 that the CreditCardCharger is called with amount = 100.
      assertEquals(purchaser.Purchase("Item costing $100", "1234567890"), PurchaseResult.OK);
      assertEquals(charger.getCalls(), 1);
      assertEquals(charger.getLastCreditCard(), "1234567890");
      assertEquals(charger.getLastAmount(), 100);
    }

    public void testItemNotFoundError() {
      // Test that we do not actually try to charge a credit card if the item is not found.

      assertEquals(purchaser.Purchase("Not found item", "1234567890"), PurchaseResult.ITEM_NOT_FOUND);
      assertEquals(charger.getCalls(), 0);
    }

    public void testCardChargeFailure() {
      // Test that a purchase can fail.

      charger.returnValue = false;
      assertEquals(purchaser.Purchase("Item costing $100", "1234567890"), PurchaseResult.CREDIT_CARD_FAILURE);
      assertEquals(charger.getCalls(), 1);
      assertEquals(charger.getLastCreditCard(), "1234567890");
      assertEquals(charger.getLastAmount(), 100);
    }
  }
  ```
 -->
  ```java
  class Purchaser {
    private CreditCardCharger creditCardCharger;

    // Purchaser 将需要使用的信用卡扣费器作为输入。
    public Purchaser(CreditCardCharger creditCardCharger) {
      this.creditCardCharger = creditCardCharger;
    }

    public PurchaseError Purchase(String itemName, String creditCard) {
      /* ... */
      // 需要时使用注入的信用卡扣费器。
      creditCardCharger.Charge(creditCard, /* 数额 */);
      /* ... */
    }
  };

  // 具体信用卡扣费器的接口。
  interface CreditCardCharger {
    public boolean Charge(String creditCard, int amount);
  };

  class RealCreditCardCharger implements CreditCardCharger {
    @Override
    boolean Charge(String creditCard, int amount) {
      /* 以某种方式对信用卡实际扣费 */
    }
  };

  class MockCreditCardCharger implements CreditCardCharger {
    private boolean returnValue = true;
    private int calls = 0;
    private String lastCreditCard = '';
    private int lastAmount = 0;

    // CreditCardCharger::Charge 的模拟实现，返回一个可配置的错误值及其先前调用参数的记录。
    @override
    public boolean Charge(String creditCard, int amount) {
      calls++;
      lastCreditCard = creditCard;
      lastAmount = amount;
      return returnValue;
    }

    // 设置调用 Charge 时会返回的值
    public void setReturnValue(int v) {
      returnValue = v;
    }

    // 获取对 Charge 最后一次调用的参数。
    public int getCalls() { return calls; }
    public String getLastCreditCard() { return lastCreditCard; }
    public int getLastAmount() { return lastAmount; }
  };


  // 生产代码
  void main() {
    CreditCardCharger charger = new RealCreditCardCharger();
    Purchaser purchaser = new Purchaser(charger);
    // 在程序流程中使用 purchaser

    /* ... */
  }

  // 测试代码（假设使用 JUnit）

  public class PurchaserTest extends TestCase {
    protected MockCreditCardCharger charger;
    protected Purchaser purchaser;

    protected void setUp() {
      charger = new MockCreditCardCharger();
      purchaser = new Purchaser(charger);
    }

    public void testPurchaseSucceeds() {
      // 测试购买能够成功。
      // 我们希望当一件价值 100 美元的商品的购买完成时 CreditCardCharger 被调用，且 amount = 100。
      assertEquals(purchaser.Purchase("Item costing $100", "1234567890"), PurchaseResult.OK);
      assertEquals(charger.getCalls(), 1);
      assertEquals(charger.getLastCreditCard(), "1234567890");
      assertEquals(charger.getLastAmount(), 100);
    }

    public void testItemNotFoundError() {
      // 测试如果找不到该项目，我们实际上不会尝试从信用卡中扣费。

      assertEquals(purchaser.Purchase("Not found item", "1234567890"), PurchaseResult.ITEM_NOT_FOUND);
      assertEquals(charger.getCalls(), 0);
    }

    public void testCardChargeFailure() {
      // 测试购买可能失败。

      charger.returnValue = false;
      assertEquals(purchaser.Purchase("Item costing $100", "1234567890"), PurchaseResult.CREDIT_CARD_FAILURE);
      assertEquals(charger.getCalls(), 1);
      assertEquals(charger.getLastCreditCard(), "1234567890");
      assertEquals(charger.getLastAmount(), 100);
    }
  }
  ```

<!--
Mocking frameworks exist for many languages that handle setting return
values and inspecting call arguments. The above code demonstrates how
the functionality of those frameworks is implemented.
 -->
许多处理设置返回值和检查调用参数的语言都存在模拟框架。上述代码演示了这些框架的功能是如何实现的。

[builder-pattern]: https://en.wikipedia.org/wiki/Builder_pattern
[capabilities]: /concepts/components/v2/capabilities/README.md
[class-oop]: https://en.wikipedia.org/wiki/Class-based_programming
[component-binding]: /concepts/components/v2/lifecycle.md#binding
[component-collections]: /concepts/components/v2/realms.md#collections
[component-creation]: /concepts/components/v2/lifecycle.md#creating
[component-intro]: /concepts/components/v2/introduction.md
[component-manifests]: /concepts/components/v2/component_manifests.md
[dependency-injection]: https://en.wikipedia.org/wiki/Dependency_injection
[factory-method]: https://en.wikipedia.org/wiki/Factory_method_pattern
[fidl]: /concepts/fidl/overview.md
[late-binding]: https://en.wikipedia.org/wiki/Late_binding
[lazy-init]: https://en.wikipedia.org/wiki/Lazy_initialization
[lifecycle]: /concepts/components/v2/lifecycle.md
[oop]: https://en.wikipedia.org/wiki/Object-oriented_programming
[plain-data]: https://en.wikipedia.org/wiki/Passive_data_structure
[realm-open]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Realm.OpenExposedDir
[realm-create]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Realm.CreateChild
[realm-destroy]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Realm.DestroyChild
