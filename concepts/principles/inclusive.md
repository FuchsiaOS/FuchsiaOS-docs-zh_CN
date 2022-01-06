<!-- 
# Inclusive
 -->
# 包容

<!-- 
Fuchsia is an open source project that is inclusive by design,
from the architecture of the platform
to the open source community that we’re building.

Applying the principles of inclusion
through these dual lenses is a challenge we embrace.
We have not yet achieved all of our goals,
but we’re committed to doing the work to uphold this principle
with the help of our developer community.
 -->
Fuchsia 是为包容而设计的（inclusive by design）开源项目，从我们所建设的平台架构到开源社区。

通过这多个方面来实践包容性准则是我们所接受的挑战。我们尚未实现我们的所有目标，但是我们致力于在我们开发者社区的帮助下开展工作，以支持这项原则。

<!-- 
## Fuchsia architecture is inclusive by design
 -->
## Fuchsia 架构是为包容而设计的

<!-- 
### Developers can use their runtime and language of choice {#bring-your-own-runtime}
 -->
### 开发者可以使用他们所选择的运行时和语言 {#bring-your-own-runtime}

<!-- 
**[Fuchsia Interface Definition Language (FIDL)](/docs/concepts/fidl/overview.md)
allows diverse clients and services to interoperate**

Fuchsia is highly extensible:
developers can create components using the language and environment they prefer.
Both components and FIDL protocols are accessible to any runtime.
Software from different runtimes can integrate together to form a cohesive
experience. Fuchsia simplifies the development model,
making nearly all user space software a component,
from system services to end-user applications.

This principle is also known as Bring Your Own Runtime (BYOR).
 -->
**[Fuchsia 接口定义语言（Fuchsia Interface Definition Language, FIDL）](/concepts/fidl/overview.md)允许各种客户端和服务协同运作**

Fuchsia 是高度可扩展的：
开发人员可以使用他们所偏好的语言和环境来创建组件。组件和 FIDL 协议两者均可通过任何运行时访问。来自不同运行时的软件可以有机结合，形成一致体验。Fuchsia 简化了开发模型，将从系统服务到用户端应用的几乎所有用户空间软件变成了一个组件。

该原则也称为 Bring Your Own Runtime（BYOR，“带好您自己的运行时”）。

<!-- 
### Fuchsia is designed to support a wide range of hardware
 -->
### Fuchsia 为支持多种硬件而设计

<!-- 
**[Fuchsia Driver Framework](/docs/concepts/drivers/fdf.md)
allows for a diverse hardware ecosystem**

Fuchsia aims to have a binary-stable interface for drivers.
In this approach,
developers can write drivers once and
these drivers will continue to work as Fuchsia evolves.
There’s no need to modify or recompile drivers when there’s an
update to Fuchsia. This allows for a large hardware ecosystem that
is scalable and easier to maintain.
 -->
**[Fuchsia 驱动框架](/concepts/drivers/fdf.md]为丰富的硬件生态系统着想**

Fuchsia 旨在拥有面向驱动的二进制稳定接口。在该方案下，开发人员可以编写驱动一次，这些驱动将随着 Fuchsia 更新迭代而持续运作。当 Fuchsia 更新时，不需要修改或重新编译驱动。这是为一个可扩展、易维护的大型硬件生态系统所做的考虑。

<!-- 
### Anyone can build and test Fuchsia
 -->
### 任何人都能构建和测试 Fuchsia

<!-- 
**[Fuchsia's emulator (FEMU)](/docs/get-started/set_up_femu.md)
makes it easier for most development environments to run Fuchsia**

FEMU allows you to test Fuchsia components and applications
without needing a Fuchsia device. FEMU looks and behaves like a Fuchsia device,
with the exception that no paving is required.
FEMU simulates different processes and environments
that any developer can use to test and build Fuchsia.
 -->
**[Fuchsia 模拟器（Fuchsia's emulator，FEMU）](/get-started/set_up_femu.md)使得大多数开发环境运行 Fuchsia 变得很简单**

FEMU 允许您无需 Fuchsia 设备而测试 Fuchsia 组件和应用。FEMU 的外形和行为与 Fuchsia 设备相似，除了不需要铺设（pave）以外。FEMU 模拟了不同的进程和环境，任何开发者均可用来测试和构建 Fuchsia。

<!-- 
## Open source community
 -->
## 开源社区

<!-- 
### All developers are welcome to contribute
 -->
### 欢迎所有开发者做出贡献

<!-- 
**[Guidelines and resources](/CONTRIBUTING.md)
are available to help Fuchsia developers**

Google and the Fuchsia team are committed
to preserving and fostering a diverse, inclusive, and welcoming community.
As an open source effort, we welcome high-quality, well-tested contributions
from all. [Our code of conduct](/CODE_OF_CONDUCT.md)
is in place to ensure that community discussions are productive and kind.
 -->
**[方针和资源](/CONTRIBUTING.md)可用于帮助 Fuchsia 开发者**

Google 和 Fuchsia 团队致力于保护和培育一个多样、包容、热情友好的社区。由于开源性质，我们欢迎一切高质量的、经良好测试的贡献。[我们的行为规范](/CODE_OF_CONDUCT.md)正在实施中，以确保社区讨论有效而友善。

<!-- 
### Inclusive language is a core value
 -->
### 包容的语言是一项核心价值观

<!-- 
**[Respectful code practices](/docs/contribute/respectful_code.md)
reduce harm and bias**

Fuchsia's values include treating each other with dignity.
It’s important that everyone can contribute
without facing the harmful effects of bias and discrimination.
Our respectful code guidelines aim to eliminate terms
that perpetuate discrimination in our codebase, user
interfaces, and documentation.
 -->
**[心怀敬畏的代码习惯](/contribute/respectful_code.md)可以减少伤害和偏见**

Fuchsia 的价值观中包含对待他人要庄重。对于每个人来说，进行贡献而无需遭受偏见和歧视的有害后果非常重要。我们心怀敬畏的代码方针旨在消除我们代码库、用户界面、文档中助长歧视之风的词语。

<!-- 
### Communication channels are open
 -->
### 交流频道是开放的

<!-- 
**[Our bug tracking system](/docs/contribute/report-issue.md)
and [mailing lists](/docs/contribute/community/get-involved.md)
are public**
 -->
**[我们的故障跟踪系统](/contribute/report-issue.md)和[邮件列表](/contribute/community/get-involved.md)是公开的**

<!-- 
The open source community can stay informed about Fuchsia updates and progress
by joining our mailing lists.
Fuchsia invites developers to contribute and report issues though our
bug tracking system.
The Fuchsia project uses Gerrit's web-based UI to manage code and
documentation reviews.
 -->
通过加入邮件列表，开源社区可以了解 Fuchsia 的更新和进展。Fuchsia 诚邀广大开发人员通过我们的故障跟踪系统贡献代码、提报议题。Fuchsia 项目使用了 Gerrit 的基于网络的用户界面来管理代码和文档评论。

<!-- 
### Our roadmap is public
 -->
### 我们的路线规划图是公开的

<!-- 
**Fuchsia is a [work in progress](/docs/contribute/roadmap/index.md)**
 -->
**Fuchsia 是一件进展之中的产品**

<!-- 
As the project evolves,
Fuchsia is striving to be as open as possible about the state of
the code and roadmap. The [Fuchsia RFC process](/docs/contribute/governance/rfcs/README.md)
aims to provide a consistent and transparent path
for making project-wide, technical decisions.
 -->
随着项目不断进展，Fuchsia 的代码和路线规划会逐渐变得尽可能开放。[Fuchsia RFC 流程](/contribute/governance/rfcs/README.md)旨在提供一个连贯一致、直白透明的方式来做出全项目的、技术性的决策。
