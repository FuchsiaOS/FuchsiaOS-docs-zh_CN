{% set rfcid = "RFC-0002" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- ## Summary -->
## 概要

<!-- 
This document proposes the notion of an *API level* and an *ABI revision* to
the Fuchsia platform. End-developers build against a *target API level*, which
determines which declarations are visible to the application. The
*target API level* also becomes embedded in the compiled application as a
*target ABI revision*, which indicates the semantics the application expects
from the platform. A given release of the Fuchsia platform typically supports
multiple ABI revisions, which lets the platform run older applications while
still providing a path for evolving the platform.
 -->
本文档为 Fuchsia 平台提出了 API 版本和 ABI 版本的概念。
终端开发人员根据目标 API 版本构建应用程序，该版本确定哪些声明对应用程序可见。
目标 API 版本也被嵌入到已编译的应用程序中，作为一个目标 ABI 版本，
表示应用程序期望从平台获得哪些语义支持。一个给定的 Fuchsia 平台版本通常支持多个 ABI 版本，
这样既支持了平台运行较旧的应用程序，同时又为平台的发展提供了途径。

<!-- ## Motivation -->
## 动机

<!-- 
Currently, the Fuchsia platform evolves through a series of *soft transitions*.
To change part of the [Fuchsia System Interface], the platform first introduces
the new interface. Applications then migrate to the new interface. After all the
applications have migrated to the new interface, the platform then removes the
old interface.
 -->
目前，Fuchsia 平台通过一系列软过渡的方式逐步发展。
要改变一部分 [Fuchsia 系统接口]，平台首先会引入新接口。
然后应用程序可以逐步迁移到新接口。在所有应用程序都迁移到新接口之后，
平台会移除旧接口。

<!-- 
Using this approach, the platform can evolve only as fast as the slowest
application. In order to complete a soft transition, the platform needs to wait
for the last application to migrate off the old interface. As the number of
applications increases and the coupling between the platform and the
applications decreases, soft transitions take increasingly longer to execute.
Eventually, we will be unable to evolve the platform using soft transitions.
 -->
使用这种方法，平台的发展速度只能与最慢的应用程序一样快。为了完成软过渡，
平台需要等待最后一个应用程序从旧接口迁移到新接口。
当应用程序数量增加，平台和应用程序之间的耦合减少，
软过渡需要越来越长的时间来完成。
最终，我们将无法使用软过渡来发展平台。

<!-- 
This RFC addresses the following problem statement:
 -->
该 RFC 解决了以下问题陈述：

<!-- 
> How can the Fuchsia platform continue to evolve while being able to run a
growing number of older applications over a longer period of time?
 -->
> 在较长的一段时间内，Fuchsia 平台如何在支持越来越多的旧应用程序运行的同时, 保持平台的持续发展？

<!-- ## Why now? -->
## 为什么现在进行版本控制？

<!-- 
Several of our customers are requesting more stability from the platform. If we
offer that stability now, we will slow down our ability to evolve the platform.
In order to meet these current customer needs, the platform needs to be able to
offer longer compatibility windows without grinding the project to a halt.
 -->
我们的一些客户要求平台提供更高的稳定性。要是我们现在提供这种稳定性，
我们将放慢我们发展平台的能力。为了满足这些当前的客户需求，
平台需要能够提供更长的兼容性窗口才不会使项目停滞。

<!-- 
Additionally, the experience from Windows is that we would benefit from
embedding target ABI revisions in applications prior to being required to
provide binary compatibility with those applications for a long period of time.
Windows missed that opportunity and now tries to guess the target ABI revision
for binaries using heuristics, which creates significant developer pain.
 -->
此外，Windows 的经验使我们从中受益,
我们选择了在应用程序中嵌入目标 ABI 版本,
来回避长期为应用程序提供二进制兼容性。
Windows 错过了这个机会，现在只能通过启发式来猜测二进制文件的目标 ABI 版本，
这给开发人员带来了很大的痛苦。

<!-- ## Terminology -->
## 术语

<!-- 
A *release* of Fuchsia is a build of the Fuchsia operating system and associated
packages that is deployed to a user population. A release has a version number
that identifies the set of software artifacts contained in the release.
 -->
Fuchsia 版本是用于部署到用户群体的 Fuchsia 操作系统和相关的软件包的一个构建。
每个版本有一个版本号用来标识该版本中包含了哪些软件构件。

<!-- 
*Backwards compatibility* refers to the ability of a newer release of Fuchsia to
run binaries intended to run on older release of Fuchsia.
 -->
向后兼容性是指较新版本的 Fuchsia 能够运行原本用于在旧版本 Fuchsia 上运行的二进制文件。

<!-- 
The *Fuchsia IDK* is an artifact used by development environment integrators to
expose the Fuchsia platform to developers to build applications that
run on Fuchsia. The Fuchsia IDK is published by the Fuchsia project and defines
the contract between the Fuchsia platform and applications that run on Fuchsia.
The IDK tools define the contract between the Fuchsia IDK tools and the
development environment integrators' environments.
 -->
Fuchsia IDK 是一个供开发环境集成器使用的构件，
用于在开发人员构建 Fuchsia 应用程序时提供 Fuchsia 平台支持。
Fuchsia IDK 由 Fuchsia 项目发布，
并定义了 Fuchsia 平台与 Fuchsia 应用程序之间的协定。
IDK 工具定义了 Fuchsia IDK 工具和开发环境集成器的环境之间的协定。

<!-- 
A *soft transition* is a technique for breaking down a backwards-incompatible
change into a sequence of smaller changes to the platform and a set of known
binaries such that compatibility is maintained locally at each step.
 -->
软过渡是一种技术，可以将向后不兼容的改动分解为一系列对平台较小的改动和一组已知的二进制文件，
这样使得兼容性在每一步都得到维护。

<!-- ## Design -->
## 设计

<!-- 
The design described in this document is to version the [Fuchsia System
Interface], which lets the platform and the applications agree about the
semantics the application expects from the platform.
 -->
本文档中描述的设计旨在对 [Fuchsia 系统接口] 进行版本化，
这让平台和应用程序就应用程序期望从平台获得的语义达成一致。

<!--  
Specifically, if an application works on a given release of Fuchsia, then the
application should continue to work on future releases of Fuchsia unless Fuchsia
intentionally drops support for the application. This design does not address
the converse problem of creating a new application that works on older releases
of Fuchsia.
 -->
具体来说，如果一个应用程序在给定的 Fuchsia 版本上运行，那么应用程序应继续在 Fuchsia 的未来版本上工作，
除非 Fuchsia 故意放弃对该应用程序的支持。
这个设计不解决创建适用于 Fuchsia 旧版本的新应用程序问题。

<!-- ### Versioning -->
### 版本控制

<!-- 
The Fuchsia platform uses two version identifiers, an *API level* and an
*ABI revision*. Both these versions identify the *interface* provided by the
platform rather than the *implementation* of that interface. Releases of Fuchsia
use a different versioning scheme, which identifies the specific implementation
in that release.
 -->
Fuchsia 平台使用两个版本标识符，
一个 API 版本和一个 ABI 版本。
这两个版本都标识了由平台提供的接口而不是该接口的实现。
Fuchsia 版本使用了不同的版本控制方案，它在版本中标识了具体的实现。

<!-- 
A given API level implicates a specific ABI revision, but multiple API levels
might implicate the same ABI revision.
 -->
一个给定的 API 版本包含一个特定的 ABI 版本，但多个 API 版本可能包含相同的 ABI 版本。

<!-- #### API level -->
#### API 版本

<!-- 
A Fuchsia *API level* denotes a set of APIs available when building an
application. A given release of the [Fuchsia IDK] typically supports multiple
API levels. The APIs available at a given supported API level should be
consistent across IDK releases.
 -->
Fuchsia API 版本表示在构建应用程序时哪些 API 可用。
一个给定的 [Fuchsia IDK] 版本通常支持多个 API 版本。
一个给定的 API 版本中可用的 API，应该在多个 IDK 版本中保持一致。

<!-- 
> *Example.* Consider `pkg/fit`, which is a C++ library in the SDK. The `fit`
library declares a number of functions, each of which is an API exposed by the
library. The API defines that set of functions, which means two IDK releases
should expose the same set of functions in the `fit` library at the same API
level.
 -->
> 示例。例如 `pkg/fit`，它是 SDK 中的 C++ 库。
`fit` 库声明了许多函数，每一个函数都是由该库提供的一个 API。
API 版本定义了这组函数，这意味着两个 IDK 版本应该在同一个 API 版本中的 `fit` 库中提供相同的函数集。

<!-- 
Syntactically, a Fuchsia *API level* is an unsigned, 64-bit integer[^1]. 
As the platform evolves (see *Evolution* below), API levels are assigned in
increasing order and are intended to be understood by human beings, including end-developers.
 -->
从语法上讲，Fuchsia 的 API 版本是一个无符号的 64 位整数 [^1]。
随着平台的发展（参见下面的发展），API 版本按序递增，旨在为人类（包括终端开发人员）所理解。

<!-- #### ABI revision -->
#### ABI 版本

<!-- 
A Fuchsia *ABI revision* denotes the semantics of the [Fuchsia System Interface]
that an application expects the platform to provide. A given release of Fuchsia
typically supports multiple ABI revisions, but semantics for a given supported
ABI revision should be consistent (see *Evolution* below) across Fuchsia
releases.
 -->
Fuchsia ABI 版本表示应用程序期望平台提供的 [Fuchsia 系统接口] 语义信息。一个给定的 Fuchsia 版本
通常支持多个 ABI 版本，但一个给定的 ABI 版本的语义应该在多个 Fuchsia 版本中保持一致（参见下面的发展）。

<!-- 
> *Example.* Consider `zx_clock_get_monotonic`, which is a function exposed by
the vDSO as part of the [Fuchsia System Interface]. The ABI revision specifies
both whether this function exists and what happens when this function is called,
which means the semantics of `zx_clock_get_monotonic` should be consistent
across Fuchsia releases at the same ABI revision.
 -->
> 示例。 例如 `zx_clock_get_monotonic`，这是一个由
vDSO 提供的 [Fuchsia 系统接口]。ABI 版本指定该函数是否存在以及调用该函数时会发生什么，
这意味着 `zx_clock_get_monotonic` 在跨 Fuchsia 版本时，同一个 ABI 版本中的语义应该是一致的。 

<!--  
Syntactically, a Fuchsia *ABI revision* is an unsigned, 64-bit integer. An ABI
revision is an opaque identifier without internal structure. To create an
identifer for a new ABI revision, select a unsigned, 64-bit integer at random
among values that have never been used to identify a Fuchsia ABI revision
before.
 -->
在语法上，Fuchsia ABI 版本是一个无符号的 64 位整数。
ABI 版本是一个没有内部结构的不透明标识符。
要创建一个新 ABI 版本的标识符，随机选择一个从未用于标识 Fuchsia ABI 版本的无符号 64 位整数。

<!-- 
Identifiers for ABI revisions are chosen at random to prevent end-developers
from guessing a future ABI revision identifier and forming expectations about
the semantics of a future version of the [Fuchsia System Interface]. As a
result, ABI revisions are intended to be understood by machines and only rarely
interpreted by human beings.
 -->
ABI 版本的标识符是随机选择的，以防止终端开发人员猜测未来的 ABI 版本标识符并推测 [Fuchsia 系统接口] 未来版本的语义。
因此，ABI 版本旨在被机器理解，并且几乎不会被人类解读。

<!-- #### Evolution -->
#### 发展

<!-- 
The platform increases the API level whenever the platform adds or removes an
API from the [Fuchsia IDK] or when the ABI revision changes. In practice, the
project might batch changes by increasing the API level on some defined cadence
(e.g., once a day or once a week).
 -->
每当平台添加或删除一个 [Fuchsia IDK] 中的 API 或 ABI 版本更改时，平台都会递增 API 版本。
在实践中，通过一些设定的节奏（例如，一天一次或一周一次）来增加 API 版本，可能会导致项目批量更改。

<!-- 
The platform changes the ABI revision whenever the platform makes a
*backwards-incompatible* change to the semantics of the
[Fuchsia System Interface]. In practice, the project might batch
backwards-incompatible changes by changing the ABI revision on some defined
cadence (e.g., every six weeks or every six months).
 -->
每当平台对 [Fuchsia 系统接口] 的语义做出向后不兼容的改动时， 平台都会更改 ABI 版本。
在实践中，通过一些设定的节奏（例如，每六周或每六个月）来更改 ABI 版本，可能会导致项目批量向后不兼容地更改。

<!-- 
In the limit, every change in semantics is potentially backwards-incompatible,
but, in practice, operating systems do make changes to their semantics without
breaking applications. For example, many popular operating systems add system
calls without breaking their applications.
 -->
在极限情况下，语义的每一次变化都可能向后不兼容，但是，
在实践中，操作系统确实会更改其语义且不破坏应用程序。
例如，许多流行的操作系统都会添加系统调用而不破坏他们的应用程序。

<!-- 
> *Action item.* Create a document that details what changes to the Fuchsia
System Interface the platform considers to be backwards-compatible. The project
will likely need to refine that document over time as the project gains
implementation experience about what changes commonly do and do not break
applications in practice.
 -->
> 重要事项。创建一个文档，详细说明平台认为哪些对 [Fuchsia 系统接口] 的改动是向后兼容的。
随着时间的推移，当项目在实践中获得更多关于哪些改动会或者不会破坏应用程序的经验之后，
项目可能需要完善该文档。

<!-- ### Applications -->
### 应用程序

<!-- 
End-developers select a single *target API level* when building a component.
The target API level controls which declarations in the [Fuchsia IDK]
are available when building the component. For example, a FIDL message
introduced in API level 7 is not available when building a component that
targets API level 6 but is available when building a component that targets API
level 7 or 8 (assuming the message was not deprecated in API level 8).
 -->
终端开发人员在构建组件时选择一个目标 API 版本。
目标 API 版本控制 [Fuchsia IDK] 中的哪些声明在构建组件时可用。
例如，FIDL 消息 API 版本 7 中引入的组件，在构建以 API 版本 6 为目标的组件时不可用，
但在构建以 API 版本 7 或 8（假设 API 级别 8 中未弃用该消息）为目标的组件时可用。

<!-- 
As part of building a component, the tools in the SDK include the
*target ABI revision* associated with the target API level in the manifest of
the component. In this way, each component declares the semantics that the
developer expected the platform to provide when they built their component. A
given package can contain many components, each of which can select whichever
target ABI revision they prefer.
 -->
作为构建组件的一部分，SDK 中的工具包含了与组件清单中的目标 API 版本关联的目标 ABI 版本。
这样，每个组件都声明了开发人员在构建组件时希望平台提供的语义。
一个给定的包可以包含许多组件，每个组件都可以选择他们喜欢的任何一个目标 ABI 版本。

<!-- ### Platform -->
### 平台

<!-- 
The platform maintains a list of *supported ABI revisions*. The platform
provides binary compatibility for components that target a supported ABI
revision, which means the platform will attempt to provide those components the
platform semantics indicated by their target ABI revision.
 -->
平台维护了支持的 ABI 版本列表。
当组件将任意一个支持的 ABI 版本作为目标，平台将为其提供二进制兼容性，
这意味着平台将尝试为这些组件提供由其目标 ABI 版本所指示的平台语义。

<!-- 
> *Example.* Consider the transition from the `fuchsia.foo.Bar` protocol to the
`fuchsia.foo.Bar2` protocol. Suppose a component, `baz.cm`, has a target ABI
revision that indicates that the component expects the platform to provide the
`fuchsia.foo.Bar`. When running `baz.cm`, the platform will route requests for
`fuchsia.foo.Bar` to the appropriate implementation. However, when running
components with a target ABI revision after the transition to
`fuchsia.foo.Bar2`, the platform will no longer route requests for
`fuchsia.foo.Bar` to an implementation because components targeting that ABI
revision should be using `fuchsia.foo.Bar2` instead.
 -->
> 示例。考虑从 `fuchsia.foo.Bar` 协议过渡到 `fuchsia.foo.Bar2` 协议。
假设一个组件，`baz.cm`，有一个目标 ABI 版本指示着当前组件期望平台提供 `Fuchsia.foo.Bar`。
运行 `baz.cm` 时，平台将对 `fuchsia.foo.Bar` 的请求分发到适当的实现。
但是，将组件的 ABI 版本过渡到 `fuchsia.foo.Bar2` 之后， 组件在运行时，
平台将不再为 `fuchsia.foo.Bar` 分发请求到具体实现，
因为指定了过渡之后的 ABI 版本的组件，应该改用 `fuchsia.foo.Bar2`。

<!--
At some point, the platform might wish to remove support for a given ABI
revision. Such removals are often gated on a tail of important components that
still rely on the old ABI revision. Rather than maintaing the full semantics
implied by the older ABI revisions, the platform maintains a list of *legacy
components* along with a table of *quirks* necessary to run those specific
components. A quirk is a compatibility shim that lets a legacy component use an
otherwise unsupported interface. Using this mechanism, the platform can remove
general support for an older ABI revision while still being able to run certain
important components that target that older ABI revision.
 -->
在某些时候，平台可能希望移除对给定 ABI 版本的支持。
此类移除通常限制在在重要组件的尾部进行，这些组件仍然依赖旧的 ABI 版本。
相比于保持旧 ABI 版本暗指的完整语义，
平台选择了维护一个遗留组件列表以及运行这些组件必要的 quirks 表。
quirk 是一种兼容性填充程序，它允许遗留组件使用另外的不支持的接口。
使用这种机制，平台可以移除对旧 ABI 版本的一般性支持，
同时仍然能够运行某些指定旧 ABI 版本的重要组件。

<!-- 
> *Example.* Suppose the platform no longer supports any ABI revisions that
include `fuchsia.foo.Bar` but that `baz.cm` is an important component that has
not migrated to `fuchsia.foo.Bar2`. The project can treat `baz.cm` as a legacy
component with the `needs-fuchsia-foo-bar` quirk. Even though the platform does
not support the target ABI revision for `baz.cm`, the platform can continue to
run `baz.cm` by routing its request for `fuchsia.foo.Bar` to a compatibility
shim, perhaps implemented using `fuchsia.foo.Bar2`. The compatibility shim does
not need to support the full semantics implied by `fuchsia.foo.Bar`. Instead,
the compatibility shim need only work well enough to keep `baz.cm` (and the
other specific components with the `needs-fuchsia-foo-bar` quirk) working.
 -->
> 示例。假设平台不再支持任何包括 `fuchsia.foo.Bar` 的 ABI 版本，
但 `baz.cm` 是一个未迁移到 `fuchsia.foo.Bar2` 的重要组件。
项目可以将 `baz.cm` 视为具有 `needs-fuchsia-foo-bar` quirk 的遗产组件。
虽然平台不再支持 `baz.cm` 的目标 ABI 版本，
但平台可以通过将组件对 `fuchsia.foo.Bar` 的请求路由到兼容性填充程序来继续运行 `baz.cm`，
这个兼容性填充程序可能使用 `fuchsia.foo.Bar2` 来实现。
兼容性填充程序不需要支持 `fuchsia.foo.Bar` 所隐含的完整语义。
反而，兼容性填充程序只需要工作得足够好，
以保持 `baz.cm`（和其他具有 `needs-fuchsia-foo-bar` quirk 的特定组件）正常工作。

<!--
The platform cannot run components that neither target a supported ABI revision
nor are listed as legacy components because the platform does not know what
semantics those components expect.
 -->
平台无法运行既不指定支持的 ABI 版本也没有被列为遗留组件的组件，
因为平台不知道这些组件所期望的语义。

<!-- ### Lifecycle {#lifecycle} -->
### 生命周期 {#lifecycle}

<!-- 
Every element of the [Fuchsia System Interface][Fuchsia System Interface]
(e.g., a system call or a FIDL message) goes through the following lifecycle:
 -->
[Fuchsia 系统接口][Fuchsia 系统接口] 的每一个元素（例如，系统调用或 FIDL 消息）经历以下生命周期：

<!-- 
 1. The element is *introduced* into the platform. End-developers cannot use the
    API until Fuchsia releases an SDK with a new API level that includes that
    element. If the element can be introduced without breaking the ABI (e.g.,
    adding a system call), then the semantics of existing ABI revisions can be
    updated to include the newly introduced element. Otherwise, the element must
    be hidden from components that target older ABI revisions to avoid breaking
    them.
 -->
 1. 元素被引入到平台中。终端开发者不能使用 API，除非 Fuchsia 发布了包括该元素的新 API 版本的 SDK。
    如果可以在不破坏 ABI 的情况下引入元素（例如，添加系统调用）， 
    则现有 ABI 版本的语义可以被更新，以包括新引入的元素。
    否则，该元素必须对指定旧 ABI 版本的组件隐藏，以避免破坏他们。

<!-- 
 2. If possible, the element can be *extended* by introducing child elements.
    For example, a FIDL table can be extended by introducing new fields.
    Introducing a child element starts another instance of the element lifecycle
    for that child element, including requiring a new API level to make the API
    for that element visible to end-developers. An element can be extended only
    if adding child elements does not break the existing API or ABI.
 -->
2. 如果可能，可以通过引入子元素来扩展元素。
   例如，可以通过引入新字段来扩展 FIDL 表。
   引入子元素会为该子元素启动元素生命周期的另一个实例，
   包括需要新的 API 版本来使该元素的 API 对终端开发人员可见。 
   一个元素只能在添加子元素不会破坏现有的 API 或 ABI 的情况下被扩展。

<!-- 
 3. The element might be *deprecated*. Components that target older ABI
    revisions can still use the element when running on newer platform releases.
    However, end-developers that target a newer API
    level can no longer use the element.
 -->
3. 该元素可能已弃用。指定旧 ABI 版本的组件在较新的平台版本上运行时，仍然可以使用该元素。
   但是，针对较新 API 版本进行开发的终端开发人员不能再使用该元素。

<!-- 
 4. The element is a *legacy* once the platform no longer supports any ABI
    revisions between the *introduction* and *deprecation* of the element. At
    this point, the platform need only support the element insofar as the
    element is actually used by a specific legacy component by way of a quirk.
 -->
4. 当平台不再支持元素的引入和弃用之间的任何 ABI 版本，
   该元素就是一个遗留。 
   此时，只有在该元素实际上被某个遗留组件通过 quirk 的方式使用时，平台才需要支持元素。

<!-- 
 5. Once none of the legacy components use the element, the element can be
    *removed* from the platform entirely.
 -->
5. 一旦没有任何遗留组件使用该元素，该元素可以完全从平台上移除。

<!-- ### Dynamics {#dynamics} -->
### 动态{#dynamics}

<!-- 
This approach incentivizes developers to migrate away from deprecated interfaces
by coupling access to new APIs to performing those migrations. Specifically, to
gain access to a newly introduced API, the developer must change their target
API level, which requires them to migrate off any interfaces that were
deprecated in that API level.
 -->
通过耦合对新 API 的访问来执行这些迁移，这种方法激励开发人员从弃用的接口迁移。
具体来说，要获得对新引入的 API 的访问权限，开发人员必须更改他们的目标 API 版本，
这要求他们不再依赖旧 API 版本中任何已弃用的接口。

<!-- ## Implementation -->
## 实现

<!-- 
Implementing this design involves many layers of the Fuchsia system. This
document provides a sketch of the changes needed at each implicated layer, but
the detailed designs for those layers are left to subsequent documents.
 -->
实现此设计涉及 Fuchsia 系统的许多层。该文档提供了每个相关层所需更改的草图，
但是这些层的详细设计则留给后续文件。

### FIDL {#fidl}

<!-- 
FIDL should offer a way to annotate the range of API levels in which each
protocol element is available. The FIDL toolchain should be aware of the
target API level and generate code appropriate for that API level.
 -->
FIDL 应该提供一种方法来注释每个协议元素可用的 API 版本范围。
FIDL 工具链应该知道目标 API 版本并生成适合该 API 版本的代码。

<!-- 
When a protocol element (e.g., a field in a table or a message in protocol) is
deprecated at a given API level, we would ideally like components that target
that API level to be able to receive messages containing that protocol element
but would like to prevent those components from sending messages that contain
that protocol element.
 -->
当一个协议元素（例如，表中的字段或协议中的消息）在给定的 API 版本中已弃用，
理想情况下，我们希望指定该 API 版本的组件，能够接收包含该协议元素的消息，
但又想阻止那些组件发送包含该协议元素的消息。

<!-- ### System headers -->
### 系统头文件

<!-- 
The system headers should let the end-developer specify a target API level and
then adjust the set of APIs that are visible using those headers according to
the target API level. In addition, the system headers should define macros that
can be used to limit the visibility of declarations in other libraries to
certain API levels.
 -->
系统头文件应让终端开发人员指定目标 API 版本，然后根据目标 API 版本使用这些头文件，来调整可见的 API 集。
此外，系统头文件应定义一些宏，用于限制声明指向某些特定 API 版本的其他库的可见性。

### vDSO

<!-- 
The system should offer multiple vDSOs, each of which supports a list of ABI
revisions. When possible, the system should evolve by extending the vDSO in a
backwards-compatible way, but, when not possible, the system can mint a new vDSO
with a separate list of supported ABI revisions.
 -->
系统应提供多个 vDSO，每个 vDSO 都支持一个 ABI 版本列表。
如果可能，系统更新时应该通过向后兼容的方式来扩展 vDSO，但是，如果不可能，
系统可以创建一个拥有独立 ABI 版本支持列表的新 vDSO。

<!-- 
Extending the vDSO increases the attack surface for existing binaries because
those existing binaries can gain access to the vDSO extensions. When deciding
whether to extend an existing vDSO or whether to mint a new vDSO, the project
should consider the security implications as well as the compatibility
implications.
 -->
扩展 vDSO 会增加现有二进制文件的攻击面，因为那些现有的二进制文件可以访问 vDSO 扩展。
当决定是扩展现有的 vDSO 还是创建新的 vDSO 时，项目应考虑安全隐患以及兼容性影响。

<!-- 
The vDSO could offer a function that checks whether the vDSO supports a given
ABI revision, but the vDSO should not directly expose the list of supported ABI
revisions because exposing that list to applications would let applications break
when the list is extended.
 -->
vDSO 可以提供一个方法来检查 vDSO 是否支持给定的 ABI 修订版，
但 vDSO 不应直接公开支持的 ABI 版本列表，因为将该列表公开给应用程序，当列表扩展时会使应用程序被破坏。

<!-- ### Process framework -->
### 进程框架

<!-- 
When launching a process, the client should inform the process launcher which
ABI revision the process expects. The process launcher should use that
information to select an appropriate vDSO and process bootstrap message for the
newly launched process.
 -->
当启动一个进程时，客户端应该通知进程启动器哪一个 ABI 版本是进程所期望的。
进程启动器应该使用此信息来选择适当的 vDSO，并为新启动的进程处理引导消息。

<!-- 
> *Open problem.* What ABI revision should we use when creating processes that
do not have a component manifest? One possibility is to put the ABI revision in
the ELF data for the executable rather than (in addition to?) in the component
manifest. Another possibility is to add the ABI revision to the
`fuchsia.ldsvc.Loader` protocol, which is typically routed to the source of the
executable.
 -->
> 未解决的问题。创建没有组件清单的进程时，我们应该使用什么 ABI 版本？
一种可能性是将 ABI 版本放入可执行文件的 ELF 数据，而不是（或是额外附加？）组件清单内。
另一种可能性是将 ABI 版本添加到 `fuchsia.ldsvc.Loader` 协议，这通常被路由到可执行文件的源。

<!-- ### Component framework -->
### 组件框架

<!-- 
The tools that build component manifests should take the target API level as a
command-line parameter and embed the corresponding ABI revision in the component
manifests they create.
 -->
构建组件清单的工具应该将目标 API 版本作为命令行参数，并在创建的组件清单中嵌入相应的 ABI 版本。

<!--  
While not needed immediately, components will eventually want to modulate
capability routes according to ABI revision. For example, a component might wish
to stop offering a certain service to one of its child components. Removing the
service immediately could break compatibility with older versions of that child
component. Instead, the parent might want to offer the service only to children
that target an older ABI revision.
 -->
虽然不是立即需要，组件最终将希望根据 ABI 版本来调整路由功能。
例如，一个组件可能希望停止为其子组件之一提供某种服务。
立即移除服务可能会破坏与该子组件旧版本的兼容性。
相反，父级组件可能希望仅为指向较旧 ABI 版本的子组件提供服务。

<!-- 
Similarly, the platform might wish to route capabilities for specific legacy
components to specialized destinations that provide compatibility shims. For
example, we could define a routing *quirk* that gets applied for specific legacy
components that have that quirk in the quirk table.
 -->
类似地，平台可能希望将特定遗留组件的功能，路由到提供兼容性填充程序的专用目标。
例如，我们可以定义一个路由 quirk，用于在 quirk 表中具有该 quirk 的特定遗留组件。

### SDK

<!-- 
The SDK should specify the API levels supported by the SDK and the mapping
between those API levels and their ABI revision in some machine-readable format
(e.g., in its JSON metadata). The SDK integrations should be modified to let
end-developers specify a target API level and to supply the target API level as
a command line argument to all the tools that require that value.
 -->
SDK 应该以某种机器可读的格式（例如，在其 JSON 元数据中）指定 SDK 支持的 API 版本，
以及这些 API 版本与其 ABI 版本之间的映射。
应该修改 SDK 集成，让最终开发人员指定目标 API 版本，
并将目标 API 版本作为命令行参数提供给需要该值的所有工具。

<!-- ## Performance -->
## 性能

<!-- 
This proposal attempts to minimize the performance impact of platform versioning
by intervening primary during build and discovery. The compatibility shims used
to run legacy components could have a significant performance impact, but the
project can evaluate those performance implications on a case-by-case basis
when adding a component to the list of legacy components.
 -->
该提案试图通过在构建和发现期间干预主要版本来最小化平台版本控制的性能影响。
用于运行遗留组件的兼容性填充程序可能会对性能产生重大影响，
但项目可以在将组件添加到遗留组件列表时逐个评估这些性能影响。

<!-- ## Security considerations {#security-considerations} -->
## 安全注意事项 {#security-considerations}

<!-- 
This proposal should have a positive impact on security because the proposal
will make it easier to migrate the Fuchsia software ecosystem to newer APIs,
which presumably have better security properties than older APIs.
 -->
该提案应该对安全性产生积极影响，
因为该提案将使 Fuchsia 软件生态系统更容易迁移到新的 API，
这些 API 可能比旧 API 具有更好的安全属性。

<!-- 
Additionally, the ability to allocate new ABI revisions makes it possible to
avoid exposing new ABIs to existing applications, which can reduce the attack
surface exposed to those applications. When deciding whether to extend an
existing ABI or whether to allocate a new ABI revision, the project should
consider the security benefits of allocating a new ABI revision.
 -->
此外，分配新 ABI 版本的能力可以避免将新 ABI 暴露给现有应用程序，
这可以减少暴露给这些应用程序的攻击面。 
在决定是否扩展现有 ABI 或是否分配新 ABI 版本时，项目应考虑分配新 ABI 版本的安全优势。

<!-- 
This proposal does provide a mechanism for malicious applications to select
different, potentially older, code paths in the platform, for example by claiming
to target an older ABI revision. As the platform evolves, the project will need
to treat code that supports older ABI revisions with the same security diligence
that the project treats code that supports newer ABI revisions.
 -->
该提案确实为恶意应用程序提供了一种机制，
可以在平台中选择不同的、可能较旧的代码路径，
例如通过声称针对较旧的 ABI 修订版。 
随着平台的发展，与项目处理支持较新 ABI 版本的代码相比，
项目将需要在安全方面付出相同的努力来处理支持旧 ABI 版本的代码。

<!-- ## Privacy considerations -->
## 隐私注意事项

<!-- 
This proposal should have a positive impact on privacy because the proposal
will make it easier to migrate the Fuchsia software ecosystem to newer APIs,
which presumably have better privacy properties than older APIs.
 -->
该提案应该对隐私产生积极影响，
因为该提案将使 Fuchsia 软件生态系统更容易迁移到更新的 API，
这些 API 可能比旧 API 具有更好的隐私属性。

<!-- ## Testing -->
## 测试

<!-- 
This proposal somewhat increases the testing matrix because the platform behaves
different depending on the ABI revision of the running component. We will need
to factor this increase in the testing matrix into the design of the Fuchsia
Compatibility Test Suite (CTS). For example, the project might want to version
CTS according to the ABI revision to ensure that the platform does not regress
its support for older ABI revisions as it evolves.
 -->
该提议在一定程度上增加了测试矩阵，
因为平台的行为取决于正在运行的组件的 ABI 版本。 
我们需要将测试矩阵的这种增加纳入 Fuchsia 兼容性测试套件 (CTS) 的设计中。 
例如，项目可能希望根据 ABI 版本对 CTS 进行版本控制，
以确保平台不会随着其发展而倒退其对旧 ABI 修订版本的支持。

<!-- ## Documentation -->
## 文档

<!-- 
The documentation for the platform should be updated to annotate every API with
its current stage in the lifecycle as well as its lifecycle history (e.g., when
the API was introduced, deprecated, and/or removed). These annotations should be
derived from the same source-of-truth that control whether applications have
access to these API when targeting a specific API level. For example, the
`fidldoc` tool should understand the API level annotations in the FIDL source
files and generate the appropriate annotations in the generated documentation.
 -->
应该更新平台的文档以注释每个 API 及其在生命周期中的当前阶段和生命周期历史（例如，何时引入、弃用和/或删除 API）。
这些注释应该来自相同的事实来源，用于控制应用程序在针对特定 API 版本时是否可以访问这些 API。
例如，`fidldoc` 工具应该了解 FIDL 源文件中的 API 版本注释，并在生成的文档中生成适当的注释。

<!-- 
Whenever the platform creates a new ABI revision identifier, the project should
update the documentation to describe in what ways the new ABI revision is not
backwards compatible with the previous ABI revision and what action, if any,
end-developers should take when updating their applications.
 -->
每当平台创建新的 ABI 版本标识符时，
项目应更新文档以描述新的 ABI 版本在哪些方面与先前的 ABI 版本不向后兼容，
以及终端开发人员在更新其应用程序时应采取的措施（如果有）。

<!-- 
In addition, the project should have some conceptual documentation that explains
why the platform has API levels and how to upgrade from one API level to
another.
 -->
此外，该项目应该有一些概念文档来解释为什么平台有 API 版本以及如何从一个 API 版本升级到另一个 API 版本。

<!-- ## Drawbacks, Alternatives, and Unknowns -->
## 缺点、替代方案和未知数

<!-- ### What are the costs of implementing this proposal? -->
### 实施此提案的成本是多少？

<!-- 
The main cost of implementing this proposal is increased operational complexity
when evolving the platform. Adding a new API now requires coordination across
the project to release that API in a new API level. Similarly, deprecating an
ABI is more involved because deprecation happens in several steps.
 -->
实施此提议的主要成本是在发展平台时增加了操作复杂性。 
添加新 API 现在需要跨项目进行协调，以在新的 API 版本中发布该 API。 
同样，弃用 ABI 涉及更多，因为弃用发生在几个步骤中。

<!-- 
The system itself will also become more complicated because the behavior of the
system will be partially dependent on the ABI revision of each component.
 -->
系统本身也将变得更加复杂，因为系统的行为将部分依赖于每个组件的 ABI 版本。

<!-- ### What other strategies might solve the same problem? -->
### 还有什么其他策略可以解决同样的问题？

<!-- 
Another strategy, which is used by some other platforms, is to never remove
functionality. For example, the web platform evolves almost entirely additively.
In some ways, that approach is simpler because the system would not need a
mechanism to deprecate functionality.
 -->
其他一些平台使用的另一种策略是永远不要删除功能。 
例如，网络平台几乎完全是累加的。 
在某些方面，这种方法更简单，因为系统不需要一种机制来弃用功能。

<!-- 
Another approach might be to use different version identifiers for different
parts of the system rather than a single API level that applies to the entire
system. To a certain extent, Fuchsia uses this approach as well. For example,
the file systems each have their own version identifiers, which is used for the
contract between the on-disk representation and the in-memory code for the file
system. Using a single API level for the entire system implies a degree of
coordination about the evolution of contract between the platform and
applications.
 -->
另一种方法可能是对系统的不同部分使用不同的版本标识符，
而不是适用于整个系统的单个 API 版本。
在一定程度上，Fuchsia 也使用了这种方法。
例如，每个文件系统都有自己的版本标识符，用于文件系统的磁盘表示和内存代码之间的协定。 
对整个系统使用单一的 API 版本意味着平台和应用程序之间的协定演变的协调程度。

<!-- ## Prior Art and References {#prior-art-and-references} -->
## 现有技术和参考文献 {#prior-art-and-references}

<!-- 
There is a vast amount of prior art on this subject. The proposal in this
document builds directly on the experience of Android, Windows, and macOS/iOS.
 -->
关于这个主题有大量的现有技术。 
本文档中的提案直接建立在 Android、Windows 和 macOS/iOS 的经验之上。

### Android

<!-- 
Android has the concept of an API level. Every platform interface on Android is
annotated with the API level at which the interface was introduced. Android
applications also specify their target API level in their manifest using the
[`uses-sdk`] element. In principle, Android could use this API level mechanism
to deprecate and remove older interfaces.
 -->
Android 有 API 版本的概念。 
Android 上的每个平台接口都标有引入接口的 API 版本。 
Android 应用程序还使用 [`uses-sdk`] 元素在其清单中指定其目标 API 版本。 
原则上，Android 可以使用此 API 版本机制来弃用和删除旧接口。

### Windows

<!-- 
Windows makes heavy use of a concept similar to ABI revision, which appears as
the [`SupportedOS`] entry in application manifests. Windows uses a GUID to
identify the ABI revision that the application is targetting, which is similar
to the proposal in this document to use an opaque 64-bit integer.
 -->
Windows 大量使用了类似于 ABI 修订的概念，
它在应用程序清单中显示为 [`SupportedOS`] 条目。 
Windows 使用 GUID 来识别应用程序所针对的 ABI 版本，
这类似于本文档中使用不透明 64 位整数的建议。

<!--  
In Windows, the `SupportedOS` GUIDs are associated with specific releases of
Windows. For example, `e2011457-1546-43c5-a5fe-008deee3d3f0` identifies Windows
Vista. However, later versions of Windows (e.g., Windows 7) understand the
`e2011457-1546-43c5-a5fe-008deee3d3f0` GUID and provide compatibility with the
Windows Vista ABI. The proposal in this document decouples the ABI revision from
platform releases, which is more flexible.
 -->
在 Windows 中，`SupportedOS` GUID 与特定的 Windows 版本相关联。 
例如，`e2011457-1546-43c5-a5fe-008deee3d3f0` 标识 Windows Vista。 
但是，更高版本的 Windows（例如，Windows 7）理解 `e2011457-1546-43c5-a5fe-008deee3d3f0` GUID 并提供与 Windows Vista ABI 的兼容性。 
本文档中的提议将 ABI 版本与平台版本分离，这更加灵活。

### macOS, iOS

<!-- 
Both macOS and iOS use the [`API_AVAILABLE`] and `@available` annotations to
control whether a declaration is available when building an application.
System libraries (aka frameworks) also use "linked on or after" checks and
explicit quirk tables to support legacy applications that require older
semantics from the platform.
 -->
macOS 和 iOS 都使用 [`API_AVAILABLE`] 和 `@available` 注释来控制在构建应用程序时声明是否可用。 
系统库（又名框架）还使用 “链接时或链接后” 检查和显式 quirk 表来支持需要来自平台的旧语义的遗留应用程序。

<!-- 
Apple has used these mechanisms successfully to migrate applications for these
operating systems from older APIs to newer APIs.
 -->
Apple 已成功使用这些机制将这些操作系统的应用程序从旧 API 迁移到新 API。

<!-- 
[^1]: [RFC-0083: FIDL Versioning][rfc-0083] amends this, restricting
    API levels to 63 bits in order to reserve the high bit for other uses.
 -->
[^1]: [RFC-0083: FIDL Versioning][rfc-0083] 对此进行了修改，将 API 版本限制为 63 位，以便为其他用途保留高位。

[Fuchsia 系统接口]: /concepts/packages/system.md
[Fuchsia IDK]: /development/idk/README.md
[`uses-sdk`]: https://developer.android.com/guide/topics/manifest/uses-sdk-element
[`SupportedOS`]: https://docs.microsoft.com/en-us/windows/win32/win7appqual/compatibility---application-manifest#leveraging-feature-capabilities
[`API_AVAILABLE`]: https://developer.apple.com/documentation/swift/objective-c_and_c_code_customization/marking_api_availability_in_objective-c
[rfc-0083]: /contribute/governance/rfcs/0083_fidl_versioning.md
