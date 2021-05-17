<!-- # Fuchsia contributor roles -->

# Fuchsia 贡献者角色

<!-- ## Overview {:#overview} -->

## 概述 {:#overview}

<!-- This document defines the roles associated with contributing to
the Fuchsia project. -->

本文档定义了与 Fuchsia 项目相关的角色。

<!-- ## Principles {:#principles} -->

## 原则 {:#principles}

<!-- Roles within the Fuchsia project seek to embody the following principles: -->

Fuchsia 项目中的角色力求体现以下原则：

<!-- *   _Transparency._ We are transparent and open about roles and requirements.
*   _Inclusivity_. Fuchsia lets anyone contribute to the project, regardless of
    their employer.We believe contributions from a diverse, open-source
    community are critical to improving Fuchsia.
*   _Responsibility_. Roles and privileges can be revoked if a person no
    longer meets the requirements. -->

*   _透明性。_ 我们对角色和要求是透明和公开的；
*   _包容性。_ Fuchsia 项目允许任何人参与项目贡献，而不管他们的职业。我们认为，来自多元化、开放源代码社区的贡献对于改善 Fuchsia 至关重要；
*   _责任。_ 如果一个人不再符合要求，角色和特权可以被撤销。

<!-- ## Roles {:#roles} -->

## 角色 {:#roles}

<!-- The following are the contributor roles associated with the Fuchsia project: -->

以下是与 Fuchsia 项目相关的贡献者角色。

<!-- *   [Member](#member)
*   [Committer](#committer)
*   [Owner](#owner)
*   [Global Approver](#global-approver) -->

*   [成员](#member)
*   [提交者](#committer)
*   [所有者](#owner)
*   [终审员](#global-approver)

<!-- ### Member {:#member} -->

### 成员 {:#member}

<!-- Anyone who contributes to the project by providing patches to code or
documentation, and agrees to the Google Developers' [Contributor License Agreement](https://cla.developers.google.com/){:.external}. -->

任何通过提供代码补丁或文档来为项目做出贡献的人，并且同意了谷歌开发者的 [贡献者许可协议](https://cla.developers.google.com/){:.external}。

<!-- #### Responsibilities {:#responsibilities} -->

#### 责任 {:#responsibilities}

<!-- Members are responsible for acting in accordance with
the [Fuchsia Code of Conduct](/CODE_OF_CONDUCT.md). -->

委员们有责任遵守以下 [Fuchsia 行为准则](/CODE_OF_CONDUCT.md)。

<!-- #### Become a Member {:#become-a-member} -->

#### 成为一个成员 {:#become-a-member}

<!-- To become a Member you must do the following: -->

要成为一个成员，您需要做到以下几点：

<!-- *   Sign the Google Developers' [Contributor License Agreements](https://cla.developers.google.com/){:.external}.
*   Acknowledge the [Fuchsia Code of Conduct](/CODE_OF_CONDUCT.md). -->

*   签署谷歌开发者的 [贡献者许可协议](https://cla.developers.google.com/){:.external}。
*   了解并遵守 [Fuchsia 行为准则](/CODE_OF_CONDUCT.md)。

<!-- ### Committer {:#committer} -->

### 提交者 {:#committer}

<!-- A Committer is a person who has write access to the
[Fuchsia repository](https://fuchsia.googlesource.com/){:.external}. A Committer can submit
their own Gerrit changes or Gerrit changes from any other member. -->

提交者是一种对 [Fuchsia repository](https://fuchsia.googlesource.com/){:.external} 有写权限的角色。一个提交者可以提交他们自己的 Gerrit 更改或来自任何其他成员的 Gerrit 更改。

<!-- A Committer is not just someone who can make changes, but also someone who
demonstrated the ability to collaborate effectively with other Members of the
Fuchsia community. Example collaboration activities include but are not limited
to: -->

提交者不仅是能对项目做更改的人，而且是表明有能力与 Fuchsia 社区的其他成员有效合作的角色。Fuchsia 社区的其他成员合作的能力。协作活动包括但不限于：

<!-- *   Seeking out the most knowledgeable people to review their code changes.
*   Contributing high-quality, well-tested code.
*   Fixing bugs in code or tests. -->

*   寻找最有见识的人审查他们的代码更改；
*   贡献高质量、经过良好测试的代码；
*   修复代码或测试中的 bug。

<!-- Members can become Committers with different kinds of contributions. For
instance, those working on documentation or toolchain can meet the requirements
to become Committers by contributing high-quality documentation or configuration
changes, which would not meet the “traditional” bar for well-tested code. -->

成员可以通过不同方式的贡献成为提交者。举个例子，从事文档或工具链工作的人可以通过贡献高质量的文档或工具链配置来满足成为提交者的条件，但这并不符合 "传统 "的测试良好的代码。

<!-- In order to submit Gerrit changes, Committers need to either be [Owners](#owner)
of the affected files or receive approval from an Owner of the affected files. -->

为了提交 Gerrit 更改，提交者要么需要成为被影响文件的 [所有者](#owner)，要么得到被影响文件的所有者的批准。

<!-- #### Responsibilities {:#responsibilities} -->

#### 责任 {:#responsibilities}

<!-- Committers are responsible for the following: -->

提交者负责以下内容:

<!-- *   Ensuring that the code submitted to Fuchsia by Committers is tested
according to the [Testability Rubrics](/docs/concepts/testing/testability_rubric.md).
*   Ensuring that the code submitted to Fuchsia by Committers follows testing
best practices. -->

*   确保提交者提交到 Fuchsia 的代码满足 [可测试性指标](/docs/concepts/testing/testability_rubric.md) 的要求；
*   确保提交者提交到 Fuchsia 的代码是测试良好的。

<!-- #### Become a Committer {:#become-a-committer} -->

#### 成为提交者 {:#become-a-committer}

<!-- To become a Committer you must do the following: -->

要成为一个提交者，您需要做到以下几点：

<!-- *   Contribute 10 non-trivial patches to the project, demonstrating the ability
to write high-quality, well-tested code.
*   Be nominated by a current Committer.
*   Obtain reviews and approvals of those 10 non-trivial patches from at least
2 different Committers.
*   Ensure that your nomination is supported by 3 other Committers.
*   Ensure that your nomination is not blocked by any Committer. -->

*   在项目中贡献十个有意义的补丁，展示出编写高质量、经过良好测试的代码能力；
*   被当前的提交者提名；
*   从至少两个不同的提交者上，获得这十个有意义补丁的审核和批准；
*   确保您的提名得到三个提交者的支持；
*   确保您的提名没有被任何提交者反对。

<!-- Committer nominations are evaluated within seven business days of the initial
nomination request. -->

提交者提名将在初始提名请求后的七个工作日内进行评估。

<!-- ### Owner {:#owner} -->

### 所有者 {:#owner}

<!-- An Owner is responsible for files or directories within the Fuchsia project and
has comprehensive knowledge of the code in that subtree. Owners are listed in
`OWNERS` files. For directories or files that are outside of an Owner's
responsibility, that Owner has the same privileges as a Committer. -->

所有者对紫红色项目中的文件或目录负责，并对该子树中的代码有全面的了解。所有者们都在 `OWNERS` 文件下。对于不在所有者责任范围内的目录或文件，所有者的身份就和提交者相同。

<!-- #### Responsibilities {:#responsibilities} -->

#### 责任 {:#responsibilities}

<!-- In addition to the responsibilities of a Committer and Member, Owners
are responsible for the following: -->

在提交者和成员的责任基础上，作为所有者还需要做以下几点：

<!-- *   Nominating other Owners.
*   Approving or removing other Owners.
*   Provide high-quality reviews and design feedback.
*   Approve changes for code in their subtree. -->

*   提名其他所有者；
*   批准或删除其他所有者；
*   提供高质量的评论和代码设计反馈；
*   为提交者和成员审批代码更改。

<!-- #### Become an Owner {:#become-an-owner} -->

#### 成为一个所有者 {:#become-an-owner}

<!-- To become an Owner you must do the following: -->

要成为一个所有者，您需要做到以下几点：

<!-- *   Be a [Committer](#become-a-committer).
*   Submit a substantial number of non-trivial changes to the affected subtree.
*   Provide high-quality reviews and code design feedback.
*   Provide code reviews in a timely manner.
*   Self-nominate or be nominated by another Committer.
    *   To self-nominate, [submit a Gerrit change](/docs/development/source_code/contribute_changes.md)
    that adds yourself to the `OWNERS` file of your desired repository.
    Current Owners will evaluate your change and either accept or reject your
    request. -->

*   成为一个 [提交者](#become-a-committer)；
*   对受影响的子树提交重大的有意义的更改；
*   提供高质量的评论和代码设计反馈；
*   及时提供代码审查；
*   自提名或被其他提交者提名；
    *   对于自提名，[提交一个 Gerrit 更改](/docs/development/source_code/contribute_changes.md) 添加您的名字到您想拥有的仓库下的 `OWNERS` 文件。该仓库的所有者将评估您的更改并同意或拒绝您的请求。

<!-- ### Global Approver {:#global-approver} -->

### 终审员 {:#global-approver}

<!-- A Global Approver is an Owner in the [root `OWNERS` file](/OWNERS){:.external}.
A Global Approver often makes large-scale changes that affect the entire Fuchsia
codebase. For example, Global Approvers are people who tend to maintain
various languages, toolchains, and other build system components. -->

终审员是 [根目录下 `OWNERS` 文件](/OWNERS){:.external} 中的所有者。终审员通常会进行大规模更改，从而影响整个 Fuchsia 代码库。例如，终审员是倾向于维持各种语言，工具链和其他构建系统组件。

<!-- For the full set of Global Approver expectations as well as the list of current
Global Approvers, see [the root `OWNERS` file](/OWNERS){:.external}. -->

有关终审员对项目的全部政策以及当前的终审员列表，请查看 [根目录下 `OWNERS` 文件](/OWNERS){:.external}。

<!-- While Global Approvers are empowered to provide a [Code-Review +2](https://gerrit-review.googlesource.com/Documentation/config-labels.html){:.external}
to large-scale changes, Global Approvers are not expected to have comprehensive
knowledge of the entire Fuchsia codebase. -->

虽然终审被授权对于大规模的修改提供 [代码审查+2](https://gerrit-review.googlesource.com/Documentation/config-labels.html){:.external}，但终审并不期望对整个 Fuchsia 代码库有全面的了解。

<!-- #### Responsibilities {:#responsibilities} -->

#### 责任 {:#responsibilities}

<!-- In addition to the responsibilities of a Member, Committer, and Owner, Global
Approvers are responsible for the following: -->

在成员、提交者、所有者的责任基础上，作为所有者还需要做以下几点：

<!-- *   Approving large scale changes within the Fuchsia codebase with a +2
    in Gerrit.
*   Providing timely reviews for large scale changes. -->

*   在 Fuchsia 代码库中用 +2 的方式给 Gerrit 批准大规模的修改。
*   及时为重大代码更改提供审查。

<!-- #### Become a Global Approver {:#become-a-global-approver} -->

#### 成为终审 {:#become-a-global-approver}

<!-- To become a Global Approver you must do the following: -->

要成为一个成员，您需要做到以下几点：

<!-- *   Demonstrate considerable proficiency in making large-scale changes across
    the entire Fuchsia codebase.
*   Self-nominate or get nominated by another Committer.
    *  To self-nominate, do the following:
        * [Submit a Gerrit change](/docs/development/source_code/contribute_changes.md)
          that adds yourself to the [root `OWNERS` file](/OWNERS){:.external}.
          Current Owners will evaluate your change and either accept or reject your
          request.
        * Email all [existing Global Approvers](/OWNERS){:.external}
          with your associated Gerrit change and wait one business day for discussion
          and approval. If you are being nominated, existing Global Approvers will be
          emailed by the individual nominating you. -->

*   在整个 Fuchsia 代码库中，展示出在进行大规模变更方面的熟练程度；
*   自提名或被其他提交者提名；
    *   对于自提名，应该满足：
        *   [提交一个 Gerrit 更改](/docs/development/source_code/contribute_changes.md) 添加您的名字到 [根目录下的 `OWNERS` 文件](/OWNERS){:.external}。该仓库的所有者将评估您的更改并同意或拒绝您的请求。
        *   向所有 [现有的终审们](/OWNERS){:.external} 发送电子邮件，说明您的相关 Gerrit 变更，并等待一个工作日的讨论和批准。现有的全球批准人将通过电子邮件发送给您，以提名您。

<!-- ## Code review actions {:#code-review-actions} -->

## 代码审查动作 {:#code-review-actions}

<!-- The types of code review actions you can provide depend on your role within
the Fuchsia project. -->

您可以提供的代码检查操作的类型取决于您在 Fuchsia 项目中的角色。

<!-- ### Initiate a CQ Dry Run {:#initiate-a-cq-dry-run} -->

### 建立一个代码质量演示 {:#initiate-a-cq-dry-run}

<!-- A CQ Dry Run runs your change against the available tests in the Commit Queue.
Committers, Owners, and Global Approvers can initiate a CQ Dry Run. -->

代码质量演示（译文注：CQ Dry Run）是根据“提交队列”中的可用测试运行你的更改。提交者、所有者、和终审员都可以建立一个代码质量演示。

<!-- ### Score code reviews  {:#score-code-reviews} -->

### 评分代码审查 {:#score-code-reviews}

<!-- #### Code Review {:#code-review} -->

#### 代码审查 {:#code-review}

<!-- After you request a code review, reviewers can score your change. -->

在您请求代码审查后，审查人可以对您的更改进行评分。

<!-- Reviewers can label your change with a score of **-2, -1, 0, +1, or +2**.
For more information on review label definitions, see [Gerrit Code Review - Review Labels](https://gerrit-review.googlesource.com/Documentation/config-labels.html){:.external}. -->

审查人可以为您的更改打上 **-2, -1, 0, +1, 或 +2** 的分数。有关审查标签定义的更多信息，请查看 [Gerrit 代码审查 - 审查标签](https://gerrit-review.googlesource.com/Documentation/config-labels.html){:.external}

<!-- Committers, Owners, and Global Approvers can score code reviews but only a
Global Approver or repository Owner can provide a **+2**. -->

提交者、所有者、和终审员都可以评分代码审查，但是只有终审员或者仓库的所有者可以提供 **+2** 的评分。

<!-- ### Submit approved changes {:#submit-approved-changes} -->

### 提交批准的更改 {:#submit-approved-changes}

<!-- You need a **Code Review Label +2** to submit your change. A
**Code-Review Label +2** score can only be applied by a repository Owner or
Global Approver. -->

你需要**代码审查标签 +2** 来提交你的更改。一个**代码审查标签 +2** 分只能被仓库所有者或终审员使用。

<!-- When a change is  submitted, the change is moved to the Commit Queue (CQ).
The Commit Queue verifies, commits, and merges changes to the master branch. -->

当一个更改被提交（译文注：submit）后，更改会被移到提交队列中（CQ）。提交队列核实、提交（译文注：commit）、合并更改到主分支。

<!-- ## Role matrix {:#role-matrix} -->

## 角色表 {:#role-matrix}

<!-- This table summarizes the actions that each Fuchsia contributor role can
perform. -->

这个表格总结每个 Fuchsia 贡献者角色的作用。

<!-- <table>
  <tr>
   <td><strong>Role</strong>
   </td>
   <td><strong>Create Change</strong>
   </td>
   <td><strong>Code-Review another Committer’s change</strong>
   </td>
   <td><strong>Provide Code-Review +2</strong>
   </td>
   <td><strong>Provide CQ+1 (dry run of CQ)</strong>
   </td>
   <td><strong>Submit Approved Change to CQ</strong>
   </td>
   <td><strong>Add or remove Owners</strong>
   </td>
  </tr>
  <tr>
   <td>Member
   </td>
   <td>Yes
   </td>
   <td><strong>No</strong>
   </td>
   <td><strong>No</strong>
   </td>
   <td><strong>No</strong>
   </td>
   <td><strong>No</strong>
   </td>
   <td><strong>No</strong>
   </td>
  </tr>
  <tr>
   <td>Committer
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td><strong>No</strong>
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td><strong>No</strong>
   </td>
  </tr>
  <tr>
   <td>Owner (outside owned subtree)
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td><strong>No</strong>
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td><strong>No</strong>
   </td>
  </tr>
  <tr>
   <td>Owner (in own subtree)
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
  </tr>
  <tr>
   <td>Global Approver
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
   <td>Yes
   </td>
  </tr>
</table> -->

<table>
  <tr>
   <td><strong>角色</strong>
   </td>
   <td><strong>创建更改</strong>
   </td>
   <td><strong>审查其他提交者的更改</strong>
   </td>
   <td><strong>提供审查 +2 分</strong>
   </td>
   <td><strong>提供提交队列（试运提）</strong>
   </td>
   <td><strong>提交更改到提交队列</strong>
   </td>
   <td><strong>添加或删除所有者</strong>
   </td>
  </tr>
  <tr>
   <td>成员
   </td>
   <td>是
   </td>
   <td><strong>否</strong>
   </td>
   <td><strong>否</strong>
   </td>
   <td><strong>否</strong>
   </td>
   <td><strong>否</strong>
   </td>
   <td><strong>否</strong>
   </td>
  </tr>
  <tr>
   <td>提交者
   </td>
   <td>是
   </td>
   <td>是
   </td>
   <td><strong>否</strong>
   </td>
   <td>是
   </td>
   <td>是
   </td>
   <td><strong>否</strong>
   </td>
  </tr>
  <tr>
   <td>所有者（在所有者的子树以外）
   </td>
   <td>是
   </td>
   <td>是
   </td>
   <td><strong>否</strong>
   </td>
   <td>是
   </td>
   <td>是
   </td>
   <td><strong>否</strong>
   </td>
  </tr>
  <tr>
   <td>所有者 (在子树范围内)
   </td>
   <td>是
   </td>
   <td>是
   </td>
   <td>是
   </td>
   <td>是
   </td>
   <td>是
   </td>
   <td>是
   </td>
  </tr>
  <tr>
   <td>终审员
   </td>
   <td>是
   </td>
   <td>是
   </td>
   <td>是
   </td>
   <td>是
   </td>
   <td>是
   </td>
   <td>是
   </td>
  </tr>
</table>

<!-- ## Life of a change {:#life-of-a-change} -->

## 更改的流程 {:#life-of-a-change}

<!-- The following diagram depicts the high-level stages of what happens to a change
after its pushed to Gerrit. -->

下图描述了将更改推送到 Gerrit 之后发生什么变化的高级阶段。

<!-- ![alt_text](/docs/contribute/community/images/change-resolution.png "Change approval process") -->

![文本](/docs/contribute/community/images/change-resolution.png "更改批准流程")

<!-- ## Specialized roles {:#specialized-roles} -->

## 专门的角色 {:#specialized-roles}

<!-- Areas within the Fuchsia repository may have their own unique requirements,
defining their own sets of roles and responsibilities, in addition to the ones
detailed above. -->

Fuchsia 代码库中的区域可能有其独特的要求，除了上面详述的内容之外，还需要定义自己的角色和责任。

<!-- ### API Reviewer {:#api-reviewer} -->

### API 审查员 {:#api-reviewer}

<!-- An API Reviewer is accountable for the quality and long-term
health of the [Fuchsia API Surface](/docs/glossary.md#fuchsia-api-surface).
API Reviewers collectively form the API
Council. -->

API 审查员负责 [Fuchsia API Surface](/docs/glossary.md#fuchsia-api-surface) 的质量和长期运行状况。API 审核员共同组成 API 委员会。

<!-- Any change that modifies the Fuchsia API Surface must receive an **API-Review+1**.
from a member of API Council in addition to the usual **Code-Review+2**. -->

除了通常的 **代码审查 +2** 之外，修改 Fuchsia API Surface 的任何更改必须获得由API委员会成员提供的 **API 审查 +1**。

<!-- For more details about the responsibilities of an API Reviewer and how the API
Council operates, see the [API Council Charter](/docs/contribute/governance/api_council.md). -->

更多关于 API 审查员的责任和 API 委员会如何工作的详情，请查看 [API 委员会章程](/docs/contribute/governance/api_council.md)

<!-- #### API Reviewer membership {:#api-reviewer-membership} -->

#### API 审查员成员资格 {:#api-reviewer-membership}

<!-- To become an API Reviewer you must do the following: -->

要成为一个 API 审查员，您需要做到以下几点：

<!-- *   Be a [Committer](#committer).
*   Demonstrate good judgement about the quality and long-term health of APIs.
*   Be appointed by the functional area of the Fuchsia project, as per the [API Council Charter](/docs/contribute/governance/api_council.md#membership). -->

*   成为一个 [提交者](#committer)；
*   表现出对 API 的质量和长期运行状况的良好判断。
*   被 Fuchsia 项目的功能区委托，按照 [API 委员会章程](/docs/contribute/governance/api_council.md#membership)

<!-- ### Eng Council member {:#eng-council-member} -->

### 工程师委员会成员 {:#eng-council-member}

<!-- The Fuchsia Eng Council is a small group of senior technical leaders responsible
for providing a coherent technical vision for Fuchsia. The Eng Council largely
operates by delegation and ratification, communicating engineering standards,
values, and objectives throughout the community and then reviewing and ratifying
concrete engineering proposals from project contributors. -->

Fuchsia 工程师委员会是高级技术领导人的小组，负责为 Fuchsia 提供一致的技术远景。工程师委员会主要通过授权和批准来运作，在整个社区中交流工程标准，价值和目标，然后审查和批准项目贡献者提出的具体工程建议。

<!-- #### Eng Council membership {:#eng-council-membership} -->

#### 工程师委员会资格 {:#eng-council-membership}

<!-- There is no predetermined number of people on the Eng Council. However, in order
to provide a coherent technical vision, the council has a small number of
members. Eng Council members are appointed by the governing authority for the
project. -->

工程师委员会没有预定的人数。通常情况下，但是，为了提供一致的技术远景，委员会中有少量成员。工程师委员会成员受该项目的管理机构委托。

<!-- For more details about the responsibilities of an API Reviewer and how the API
Council operates, see the [Fuchsia Eng Council Charter](/docs/contribute/governance/eng_council.md). -->

更多关于工程师审查员的责任和 工程师委员会如何工作的详情，请查看 [Fuchsia 工程师委员会章程](/docs/contribute/governance/eng_council.md)

<!-- ## Revoking Privileges {:#revoking-privileges} -->

## 撤销特权 {:#revoking-privileges}

<!-- When contributors no longer meet requirements, their role and
corresponding privileges can be revoked. -->

当贡献者不再符合要求，他们的角色和相应的特权将会被撤销。

<!-- ### Scenarios {:#scenarios} -->

### 情景 {:#scenarios}

<!-- Example scenarios for having privileges revoked include, but are not limited to,
the following: -->

撤销特权的示例情景包括但不限于以下情况：

<!-- *   Not acting in accordance with the [Fuchsia Code of Conduct](/CODE_OF_CONDUCT.md).
*   Committers repeatedly ignoring testability best-practices in their code
    reviews.
*   Owners discouraging people from requesting code reviews.
*   Owners being unresponsive to review requests. -->

*   不按照 [Fuchsia 行为准测](/CODE_OF_CONDUCT.md)；
*   提交者在其代码审查中一再忽略可测试性；
*   所有者不鼓励人们请求进行代码审查；
*   所有者对审查请求没有回应。

<!-- ### Process {:#process} -->

### 流程 {:#process}

<!-- The process for revoking an individual’s role within the Fuchsia project
involves the following steps: -->

在 Fuchsia 项目中撤销个人角色的过程涉及以下步骤：

<!-- *   An Owner makes a recommendation to `community-managers@fuchsia.dev` to
    revoke someone’s role, specifying the rationale. Revoking an Owner role
    needs to be approved by an Owner in the same subtree
    or above.
    * Ownership is often revoked when an Owner is no longer actively
      contributing to their associated files or directories. -->

*   所有者向 `community-managers@fuchsia.dev` 提出建议，撤销某人的角色，并说明其理由。
    *   当所有者不再主动为其关联的文件或目录做出贡献时，所有权通常会被撤销。

<!-- Revoking a Committer role should be a rare action and requires approval by the
governance authority. Community managers should be involved in the process of
revoking the Committer role. -->

撤销提交者角色应该是一种罕见的操作，并且需要得到管理机构的批准。社区管理者应参与撤销提交者角色的过程。

<!-- ## Frequently asked questions {:#frequently-asked-questions} -->

## 常见问题 {:#frequently-asked-questions}

<!-- As a Fuchsia Member, you might have the following questions about requesting a
code review: -->

作为一个 Fuchsia 成员，你可能会遇到如下关于请求代码审查的问题：

<!-- *   Who can provide a **Code Review +1**?
    * All Committers, Owners, and Global Approvers. Code Review +1 means
    “Looks Good To Me” but a +1 alone doesn’t allow for submission.
    Someone else has to approve the change with a +2. For more information on
    review label definitions see, [Gerrit Code Review - Review Labels](https://gerrit-review.googlesource.com/Documentation/config-labels.html){:.external}.
*   Can specific portions of the Fuchsia source code have different requirements?
    * Yes. For example, API changes have special requirements as described in
     the [Fuchsia API Council Charter](/docs/contribute/governance/api_council.md#api_review).
*   Do I need **API-Review +1**?
    * Changes affecting the Fuchsia API surface require **API-Review +1**, and the
    code review tool will only show the API-Review flag when it is needed. -->

*   谁可以提供一个**代码审查 +1**？
    *   所有的提交者、所有者、和终审员。代码审查 +1 意味着“在我看来很好”，但一个 +1 不允许提交。别人必须以 +2 批准更改。有关审查标签定义的更多信息，请查看 [Gerrit 代码审查 - 审查标签](https://gerrit-review.googlesource.com/Documentation/config-labels.html){:.external}
*   Fuchsia 源代码的特定部分可以有不同的要求吗？
    *   是的。例如 API 修改有如上文所说的特定需要，请查看 [Fuchsia API Council Charter](/docs/contribute/governance/api_council.md#api_review)；
*   我需要 **API 审查 +1** 吗？
    *   修改影响到了 Fuchsia API Surface 是需要 **API 审查 +1**的，并且代码审查工具将只显示 API 审查的标记当需要的时候。

