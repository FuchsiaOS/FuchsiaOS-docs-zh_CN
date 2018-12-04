<!--
Fuchsia Boot Sequence
=====================

This document describes the boot sequence for Fuchsia from the time the Zircon
layer hands control over to the Garnet layer.  This document is a work in
progress that will need to be extended as we bring up more of the system. -->

Fuchsia 启动顺序
=====================

本文档将描述 Fuchsia 将控制权从 Zircon 到 Garnet 的启动顺序。此文档是一项正在进行的工作，需要在我们开发中进行扩展。

<!--
# Layer 1: [appmgr](https://fuchsia.googlesource.com/garnet/+/master/bin/appmgr)

`appmgr`'s job is to host the environment tree and help create
processes in these environments.  Processes created by `appmgr`
have an `zx::channel` back to their environment, which lets them create other
processes in their environment and to create nested environments.

At startup, `appmgr` creates an empty root environment and creates
the initial apps listed in `/system/data/appmgr/initial.config` in
that environment. Typically, these applications create environments nested
directly in the root environment. The default configuration contains one initial
app: `bootstrap`. -->

# Layer 1: [appmgr](https://fuchsia.googlesource.com/garnet/+/master/bin/appmgr)

`appmgr`的工作是托管环境树(environment tree)并在这些环境中帮助创建流程。通过创建的进程 `appmgr`
有一个`zx::channel`回自己的环境，让他们在自己的环境中创建其他进程并创建嵌套环境。

在启动时, `appmgr` 创建一个空的根环境并在该环境中列出初始应用程序 `/system/data/appmgr/initial.config` 。通常，这些应用程序会创建直接嵌套在根环境中的环境。默认配置包含一个初始应用： `bootstrap`。

<!-- # Layer 2: [sysmgr](https://fuchsia.googlesource.com/garnet/+/master/bin/sysmgr/)

`sysmgr`'s job is to create the boot environment and create a number of
 initial components in the boot environment.

The services that `sysmgr` offers in the boot environment are not provided by
bootstrap itself. Instead, when `sysmgr` receives a request for a service for
the first time, `sysmgr` lazily creates the appropriate app to implement that
service and routes the request to that app. The table of which components
implement which services is contained in the
`/system/data/bootstrap/services.config` file. Subsequent requests for the same
service are routed to the already running app. If the app terminates,
`sysmgr` will start it again the next time it receives a request for a
service implemented by that app.

`sysmgr` also runs a number of components in the boot environment at
startup. The list of components to run at startup is contained in the
`/system/data/bootstrap/apps.config` file. -->

# Layer 2: [sysmgr](https://fuchsia.googlesource.com/garnet/+/master/bin/sysmgr/)

`sysmgr`的工作是创建引导环境并在引导环境中创建初始组件。

 `sysmgr`服务在根环境中不被引导本身提供。Instead, 当 `sysmgr` 第一次收到服务请求时, `sysmgr` 懒创建适当的应用程序来实现该服务并将请求routed到该应用程序。 哪个组件实现哪些服务包含在`/system/data/bootstrap/services.config` 文件中。对相同服务的后续请求将routed到已在运行的应用程序。如果应用程序终止，`sysmgr`则会在下次收到该应用程序实施的服务请求时再次启动它。

`sysmgr`在启动时还会在引导环境中运行许多组件。启动时运行的组件列表在`/system/data/bootstrap/apps.config` 文件中。  

<!--
# Layer 3: [basemgr](https://fuchsia.googlesource.com/peridot/+/master/bin/basemgr/)

`basemgr`'s job is to setup the interactive flow for user login and user
management.

It first gets access to the root view of the system, starts up Device Shell and
draws the Device Shell UI in the root view starting the interactive flow. It also
manages a user database that is exposed to Device Shell via the User Provider
FIDL API.

This API allows the Device Shell to add a new user, delete an existing user,
enumerate all existing users and login as an existing user or in incognito mode.

Adding a new user is done using an Account Manager service that can talk to an
identity provider to get an id token to access the user's
[Ledger](https://fuchsia.googlesource.com/peridot/+/master/bin/ledger/).

Logging-in as an existing user starts an instance of `user_runner` with that
user's id token and with a namespace that is mapped within and managed by
`basemgr`'s namespace.

Logging-in as a guest user (in incognito mode) starts an instance of
`user_runner` but without an id token and a temporary namespace. -->

# Layer 3: [basemgr](https://fuchsia.googlesource.com/peridot/+/master/bin/basemgr/)

`basemgr`的工作是建立用户登录和管理的交互流程。

它将先访问系统的根视图,启动 Device Shell，并在启动交互流的根视图中绘制 Device Shell UI。它还管理通过 User Provider FIDL AP 向 Device Shell公开的用户数据库。

此API允许设备Shell添加新用户，删除现有用户，查看所有现有用户，以现有用户登录或隐身登录。

添加新用户通过用户管理服务完成的，该服务可以与IDP(Identity Provider)通信以获取 ID token 来访问用户的[Ledger](https://fuchsia.googlesource.com/peridot/+/master/bin/ledger/)。

以现有用户身份登录将启动具有该用户的id令牌的`user_runner`实例，以及在命名空间内映射和管理`basemgr`的命名空间。

以访客身份登录（隐身模式）将启动没有id令牌和临时命名空间的`user_runner`实例 。
