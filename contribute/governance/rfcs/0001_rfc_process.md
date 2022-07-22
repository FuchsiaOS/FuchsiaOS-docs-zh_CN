{% set rfcid = "RFC-0001" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!--
##  summary
-->
## 总览
<!--
The Fuchsia RFC process is intended to provide a consistent and transparent path
for making project-wide, technical decisions. For example, the RFC process can
be used to evolve the project roadmap and the system architecture.
-->

Fuchsia RFC 工作流是为项目级别的技术决定提供一个一致且透明的工作流程。比如，RFC 流程可以用来演进项目路线图和系统架构。
<!--
## Motivation
-->
## 动机
<!--
Currently, the Fuchsia project does not have a formal process for making
project-wide, technical decisions. At our current scale, this informality
results in different people having different, sometimes inconsistent, viewpoints
on where the project is going and how the system is put together. By
establishing a consistent and transparent path for making project-wide,
technical decisions, all the stakeholders can be confident about the technical
direction of the project.
-->
现在，Fuchsia 项目在项目级别的技术决定上并没有一个正式的工作流程。以我们目前的规模来说，这样的非正式性导致了不同的人在项目方向和如何组合系统上有着不同的看法。 通过建立这样一个一致且透明的项目标准， 所有的利益相关者都能够在项目的技术方向充满信息。
<!--
## Design
-->
## 设计
<!--
This section describes the design of the RFC process.
-->
本节讲述 RFC 流程的设计。
<!--
### When to use the process {#criteria}
-->
### 使用场景 {#criteria}
<!--
The vast majority of changes to Fuchsia do not require an RFC. Instead, these
changes can be made using the [code review
process](/development/source_code/contribute_changes.md). However,
technical decisions that have broad impact across the project require broader
agreement and must be socialized with the project using the RFC process.
-->
绝大多数的 Fuchsia 更改并不会使用到 RFC。与之对应，这些更改可以使用：[代码审阅流程](development/source_code/contribute_changes.md)。但是，对整个项目有广泛影响的技术决策需要有更广范围的共识，则必须使用 RFC 流程使决策在项目范围内和大家交流。

<!--
The following kinds of changes must use the RFC process:
-->
下面列举了必须使用 RFC 流程的情况：
<!--
 * *Changing the project roadmap.* The project roadmap describes changes that
   have broad impact across the system, often touching a large fraction of the
   system or crossing boundaries between subsystems.
-->
 * **改变项目路线图。** 项目路线图描述了对整个系统有广泛影响的改变，一般会触及到一大部分的系统，或者跨越多个子系统的边界。

<!--
 * *Adding constraints on future development.* Some decisions, once made,
   constrain the future development of the system. We need to be careful when
   making such decisions because they can be difficult to revise later.
-->
  * **增加对未来发展的约束。** 一些决定一旦做出，就会限制系统的未来发展。我们在做这些决定的时候要慎重，因为之后可能会很难修改。
<!--
 * *Making project policy.* Project policies have broad impact across the
   system, often affecting contributors throughout the project. For example,
   changing the set of supported languages impacts everyone who needs to debug
   and understand the system, even if not everyone uses the new language.
-->
 * **制订项目政策。** 项目政策对系统有着广泛的影响，常常影响着项目贡献者。比如，修改支持的（编程）语言集，会影响需要调试和理解系统的人员，即使并不是所有的人都使用新语言。
<!--
 * *Changing the system architecture.* The system architecture describes how the
   system fits together as a whole. Changing the system architecture, by
   definition, crosses boundaries between subsystems and requires careful
   consultation with many stakeholders.
-->
 * **修改系统架构。** 系统架构描述了系统这个整体如何协作。 更改系统架构，顾名思义，会跨过子系统的边界，需要仔细向许多相关人员协商。

<!--
 * *Delegating decision-making authority.* There are often classes of decisions
   that the project needs to make frequently and that benefit from specialized
   expertise. Rather than making all these decisions through the RFC process,
   the project can delegate decision-making authority for those classes of
   decisions to another group or process. For example, we often need to make
   decisions about platform APIs, which add constraints on future development,
   but it would not be practical to use the RFC process for every change to the
   platform API.
-->
 * **决策委托。** 本项目经常会有一些需要特殊领域的专家参与的决定。这时我们把决策权委托给其他组织或者走别的流程，而不是使用 RFC 流程。比如，我们经常需要对平台的 API 接口做决定，这些接口限制着未来的开发工作，但是对所有的平台 API 接口的修改都使用 RFC 流程是不切实际的。
<!--
 * *Escalations.* Finally, contentious changes can benefit from the transparency
   and clarity of the RFC process. If there is a disagreement about technical
   direction that cannot be resolved by an individual technical leader, the
   decision can be escalated to the RFC process either by one of the disagreeing
   parties or by another contributor.
-->
 * **上报。** 最后，流程的透明性、清晰性有助于解决有争议的更改。如果在一个技术方向上存在单个技术领导者无法解决的分歧，这个决策可以被争议的任意一方或者其他贡献者上报至 RFC 流程。

<!--
The RFC process may also be used for other kinds of changes that would benefit
from its structured approach to decision making and its durable record of the
decision.
-->
RFC 流程同样也可以用于其他一些改变，从而受益于它结构化的决策方法和它持久的决策记录。

<!--
### Roles and responsibilities
-->
### 角色和职责
<!--
People interact with the RFC process in several roles:
-->
人们在与 RFC 流程的交互中有多种角色：
<!--
 * *RFC Authors.* An RFC Author is a person who writes an RFC. Everyone who
   contributes to Fuchsia can be an RFC Author. A given RFC can have one or more
   authors. The authors of a given RFC drive the process for that RFC.
-->
 * **RFC 作者。** RFC 作者是编写 RFC 的人。每个 Fuchsia 项目贡献者都可以是 RFC 的作者。一个 RFC 可以有一个或多个作者。RFC 的作者需要推进该 RFC 的进程。

<!--
 * *Stakeholder.* A stakeholder is a person who has a stake in whether the
   project accepts a given RFC. Stakeholders are typically Fuchsia contributors,
   but some RFCs might have stakeholders beyond the Fuchsia project. For
   example, stakeholders might be involved in other projects that use Fuchsia or
   are otherwise affected by changes to Fuchsia. Stakeholders do not always
   participate directly in discussions about RFCs. Instead, stakeholders are
   often *represented* by someone, often a technical lead or other person
   responsible for a group of stakeholders.
-->
  * **利益相关者。** 利益相关者是项目是否接受给定 RFC 有利益关系的人。利益相关者一般来说是 Fuchsia 的贡献者，但是也有一些 RFC 的利益相关者在 Fuchsia 项目之外。例如，利益相关者可能参与在其他使用  Fuchsia 的项目中，或者受到 Fuchsia 更改的影响。利益相关者也不一定都直接参与到 RFC 的讨论中。相反，利益相关者通常被别人“代表”，一般是技术领导或者其他一些代表利益相关群体的人。
<!--
 * *Eng Council.* The [Eng Council](../eng_council.md) facilitate discussion
   and make the final decision as to whether the project accepts an RFC.
-->
 * **工程委员会。** [工程委员会](../eng_council.md)促进讨论以及对是否接受一个 RFC 做最终的决定。
<!--
### How the process works
-->
### 流程运作原理
<!--
This section describes each step involved in the RFC process.
-->
本节介绍 RFC 流程中涉及的每一个步骤。
<!--
#### Step 1: Socialize {#socialize}
-->
#### 第一步：交流 {#socialize}
<!--
The first step in the RFC process is to socialize your idea with the project.
For example, you might have noticed a problem that you think is important to
solve. Are other people aware of this problem? Someone else might already be
working on the problem or might have some background or context about the
problem that would be useful to you. The earlier you discover this information,
the better.
-->
RFC 流程的第一步就是把您在项目中的想法和大家交流。比如，您可能发现了一个需要解决的重要问题。其他人注意到这个问题了吗？或许已经有人在着手解决这个问题了，再或者有一些其他的相关资料可以帮到您。总之越早发现这些情况，对项目越好。

<!--
Compared to the remaining steps in the process, this step is relatively
informal. This document does not contain a rigorous description of how to
socialize your idea. Socializing technical ideas is a skill unto itself.
However, a good place to start is to raise the topic in discussions with the
technical leads for areas related to the problem you are trying to solve. For
example, you might want to consult with people in the `OWNERS` files for the
areas of the codebase will need to be modified to execute your idea.
-->

相比于接下来的步骤，这一步相对非正式。这份文档不会严格要求您如何和其他人交流您的想法。交流技术想法本身就是一项技能。不过，一个好的起点，是向您准备解决的问题所在领域的技术领导讨论时提出该主题。譬如，您可能会想要跟在您需要修改的相关代码库的 `OWNERS` 文件中的人咨询，来执行您的想法。
<!--
If you are unsure how to socialize your idea, consider asking a technical leader
for advice. They will often have more experience socializing ideas and might be
able to point you in a good direction.
-->
如果您对如何和其他人交流您的想法有疑问，可以考虑向技术领导者寻求建议。通常他们在这方面更有经验，所以也能给您指明一条捷径。

<!--
> *Example.* This RFC was socialized by having a discussion in the Eng Forum,
> which is a regular meeting inside Google of various engineering leaders
> involved in the project. The RFC was also socialized with the creators of the
> FTP and CTP process, who have good background and context about these
> processes.
-->
> **例：** 这条 RFC 是经过议员论坛的交流后分发的， 
> 工程论坛是 Google 内部的由多个参与项目的工程领导参加的常规会议。
> 本条 RFC 也邀请了 FTP 和 CTP 流程的创建者交流，因为他们对这类流程有着丰富的经验。

<!--
#### Step 2: Draft {#draft}
-->
### 步骤二：起草 {#draft}
<!--
Once you have gathered all the background and context you can through
socialization, you are ready to start the formal part of the RFC process. The
next step is to write a first draft of the RFC document itself.
-->
当您准备好所有材料后，您已经准备好开始 RFC 流程的正式部分了。下一步就是编写 RFC 文档的初稿。
<!--
Mechanically, an RFC is a markdown file in the
`//contribute/governance/rfcs` directory.
To create and RFC, you create a CL that adds a file to that directory. You
should start by making a copy of the [RFC template](TEMPLATE.md). While not
strictly required, the template is designed to guide you towards writing a
high-quality RFC by prompting you to think through the problem you are trying to
solve in a semi-structured way.
-->
正常情况下，一份 RFC 是一个存放在 `/contribute/governance/rfcs` 路径下的 markdown 文件。创建一个 RFC，要先创建一个 CL 并放到这个目录下。建议从模版文件 [RFC template](TEMPLATE.md) 复制一份并在此基础上编写。模版并不是硬性要求，但是模版是设计来引导您写出一份高质量的 RFC 的。它帮助您以一种半结构化的方式认真思考您准备要解决的问题。

<!--
Do not worry about assigning a number to your RFC at this stage. Instead, use
`NNNN` as a placeholder. For example, the file name should be something like
`NNNN_my_idea.md`. The RFC will get a number shortly before landing.
-->
在这个阶段不必担心您的 RFC 的序号。反之，请使用 `NNNN` 做为占位符。比如，一个文件的名字应该是 `NNNN_my_idea.md` 这种形式。RFC 文档会在合并之前不久获得一个序号。
<!--
> *Suggestion.* Consider marking the CL containing your RFC as a
> "work-in-progress" until you are ready for feedback.
-->

> **建议。** 您可以在准备好接收反馈之前，先把包含 RFC 的 CL 标记为“进行中”。

<!--
#### Step 3: Iterate {#iterate}
-->
#### 第三步：迭代 {#iterate}
<!--
Once you have created a CL containing the first draft of your RFC, you are ready
to iterate on your idea with the appropriate stakeholders. Hopefully you will
have already discovered most the appropriate stakeholders as part of socializing
your idea, but you are very likely to discover additional stakeholders at this
stage.
-->
当您创建好包含您第一份 RFC 草稿的 CL 后，您就可以把您的想法和相关人员进行交流了。您有望发现大多数合适的利益相关者已经参与了您想法的交流，不过通常这一阶段您要发现更多的利益相关者。
<!--
Mechanically, you should invite stakeholders to provide feedback on your RFC by
adding them to the "Reviewers" or "CC" fields in the CL, as you would for a
normal code review. The stakeholders should provide you feedback by leaving
comments on your RFC in the code review tool.
-->
通常，您应该邀请利益相关者对您的 RFC 提供反馈，您可以通过在 CL 的“Reviewers“或者“CC“字段加上他们。利益相关者会通过代码审查工具在您的 RFC 中评论以提供反馈。
<!--
If the discussion is too complex for the code review tool, consider scheduling a
meeting with the relevant stakeholders to have a more efficient discussion.
After the meeting, you must post a summary of the meeting in a comment on the CL
so that people who were not at the meeting can understand what was discussed
during the meeting.
-->
如果讨论的内容对于代码审查工具来说过于复杂，可以考虑和利益相关者安排会议来进行更有效的讨论。会议结束后，您必须在 CL 中发布一段会议总结的评论，这样可以使没有参与会议的人能理解在会议中讨论的东西。

<!--
If the discussion becomes contentious, please escalate to one of the RFC
Editors. The Eng Council can help move the discussion forward, for example by
providing additional structure to the discussion or moving the discussion to
another forum. Regardless of how the discussion proceeds, the results of any
off-CL discussion must be captured in the CL, often by posting a summary of the
discussion as a CL comment.
-->
如果讨论中存在争议，请上报至 RFC 的编辑。工程委员会可以帮您推进讨论，比如，可以提供一些别的视角或者把讨论移动到其他论坛中。不论讨论如何推进，任何不在 CL 上的讨论都要记录在 CL 中，大多数情况下以评论的形式把讨论总结放上去。

<!--
If you wish to withdraw your RFC, you can mark the CL containing the RFC as
abandoned. You, or someone else, can always resurrect your RFC later if
circumstances change. If you are resurrecting an RFC created by someone else,
you should start the RFC process over from the beginning, but you can use the
withdrawn RFC as a starting point rather than `TEMPLATE.md`. Please confer with
the original authors to determine whether they wish to continue to have their
names associated with the new incarnation of the RFC.
-->
如果希望撤回 RFC，那么您可以将包含该 RFC 的 CL 标记为废弃状态。之后如果情况有变化，您或者其他人都可以恢复这个 RFC。如果您要恢复其他人创建的 RFC，那么您应当重头开始 RFC 流程，不过您可以将这个撤回的 RFC 作为起点，而不用从 `TEMPLATE.md` 开始。请与原作者协商，确认他们是否还想继续将其名称与您的新 RFC 相关联。

<!--
> *Suggestion.* If you are interested in RFCs, consider configuring the Gerrit
> Code Review tool to [send you an email > notification](https://gerrit-review.googlesource.com/Documentation/user-notify.html)
> when a CL modifies the `/contribute/governance/rfcs` directory.
-->

> **建议：** 如果您对 RFC 感兴趣，可以配置 Gerrit 代码审查工具，让它在有 CL 修改 `/contribute/governance/rfcs` 目录时[给您发送邮件 > 通知](https://gerrit-review.googlesource.com/Documentation/user-notify.html)。
<!--
#### Step 4: Approve {#approve}
-->
#### 第四步：批准 {#approve}
<!--

Once the iterations on the RFC are converging, you are ready to move to the
approval stage, in which the stakeholders sign-off on the RFC by setting the
Code-Review flag to either +1 or +2. Typically, stakeholders who need to approve
a CL (i.e., whose sign-off is required for the RFC to move forward) should
sign-off with a +2 whereas stakeholders whose approval is not required should
sign-off with a +1, but all stakeholders are welcome to sign-off with a +2 if
they wish to express their enthusiasm for the RFC.
-->
当 RFC 逐步推进至稳定，您已经准备好进入审批阶段，这个阶段中利益相关者就会给 RFC 代码审查标志为 +1 或者 +2 来表示许可。通常，需要批准 CL 的利益相关者（即 RFC 需要其许可才能向前推进）应以 +2 的形式表示许可，另一些利益相关者的批准不是必需的，则可以以 +1 的形式表示许可。不过，如果希望表达对该 RFC 的热情，那么也欢迎所有利益相关者使用 +2 许可。
<!--
Stakeholders who wish to object to an RFC can set the Code-Review flag to -1 or
 -2, depending on how strongly they feel that the RFC should not move forward.
When setting the Code-Review flag to -1 or -2, a stakeholder must state their
reason for objecting, ideally in a way that would let someone understand the
objection clearly without having to read the entire discussion that preceded
the objection.
-->
利益相关者如果希望表示反对一个 RFC，可以标记代码审查标志为 -1 或者 -2。这取决于他们对于该 RFC 不应推进的感受的强烈程度。当将一个 RFC 的代码审查标志设为 -1 或者 -2 的时候，利益相关者必须阐述说明反对的原因，最好能使人无需阅读完之前的所有讨论，就能清楚地理解反对意见。

<!--
A stakeholder setting the Code-Review flag to -1 or -2 does not necessarily
prevent the project from accepting the RFC. See the ["How decisions are made"
section](#how-decisions-are-made) below for more details about how the project
decides whether to accept an RFC.
-->
利益相关者将代码审查标志设为 -1 或者 -2 并不一定会阻止项目接收该 RFC。要获取关于 RFC 接受决定流程的更多细节，请参阅下面的[“决定如何做出”章节](#how-decisions-are-made) 。
<!--
After all the stakeholders have weighed in with their Code-Review flags, send an
email to eng-council@fuchsia.dev to prompt the Eng Council to decide whether to
accept your RFC.
-->
在所有利益相关者都给出了他们的代码审阅标志之后，请发送一封邮件到 eng-council@fuchsia.dev，提醒工程委员会决定是否接受您的 RFC。
<!--
#### Step 5: Submit {#submit}
-->
#### 第五步：提交 {#submit}
<!--
If the project decides to accept your RFC, a member of the Eng Council will
comment on your CL stating that the RFC is accepted and will assign the RFC a
number, typically the next available number in the series. If there are any -1
or -2 Code-Review flags, the Eng Council will explicitly clear each flag by
summarizing the objection and by describing why the RFC is moving forward
despite the objection.
-->
如果项目决定接受您的 RFC，工程委员会就会有一个人通过在您的 CL 中评论的形式声明这条 RFC 被接受了，并且会给 RFC 分配一个序号，通常是在序列中可用的下一个序号。如果 RFC 中有 -1 或者 -2 的代码审阅标志，工程委员会会为每个标志总结反对意见和叙述为什么这个 RFC 虽然有这些反对意见却还是继续推进，从而明确地将其清除。
<!--
If the project decides to reject your RFC, a member of the Eng Council will
comment on your CL stating that the RFC is rejected and providing a rationale
for the rejection.  Rejected RFCs are valuable engineering artifacts. The Eng
Council will work with the RFC Authors to land a version of the RFC that is
marked as rejected and incorporates the rationale.
-->
如果项目决定拒绝您的 RFC，工程委员会的委员会在您的 CL 中评论，声明该 RFC 被拒绝了，并且提供拒绝的依据。被拒绝的 RFC 也是宝贵的工程产物。工程委员会也会和 RFC 的作者一起生成一个将该 RFC 被标记为拒绝并包含理由的版本。
<!--
You should upload a new patchset of your RFC with the assigned number, both in
the title of the RFC and in the filename. If your RFC is approved and requires
implementation, please make sure you have an issue filed in the issue tracker
and put a link to the issue in the header of your RFC.
-->
您应该在 RFC 的题目和文件名中使用分配的序号，并通过 RFC 补丁集的方式上传。如果您的 RFC 已经被通过且需要具体实现，请确保在问题跟踪工具开一个问题，并且在 RFC 的头部里放上该问题的链接。
<!--
The Eng Council will then mark your CL Code-Review +2 and you can land your RFC!
-->
工程委员会会把您的 CL 的代码审查标志标记为 +2，之后您就可以合并您的 RFC 了。
<!--
*Congratulations! You have contributed a valuable engineering artifact to the
project!*
-->
**恭喜！您已经为项目提交了一份宝贵的工程产物。**
<!--
### How decisions are made {#how-decisions-are-made}
-->
### 决定如何做出 {#how-decisions-are-made}

<!--
The decision whether to accept an RFC is made by the Eng Council, acting in
[rough consensus](https://en.wikipedia.org/wiki/Rough_consensus) with each
other. If the decision involves an RFC that has Eng Council members as authors,
those members must recuse themselves from the decision.
-->
RFC 接受与否的决定是由工程委员会做出的，会内彼此达成[粗略共识](https://en.wikipedia.org/wiki/Rough_consensus)。如果要决定的 RFC 的作者中有工程委员会的成员，那这些成员需要在做决定时回避。

<!--
If the Eng Council cannot reach rough consensus, the RFC is not accepted.
In deciding whether to accept an RFC, the Eng Council will consider the
following factors:
-->
如果工程委员会不能达成粗略共识，该 RFC 不会被接受。在考虑是否接受 RFC 的时候，工程委员会将考虑如下几点：

<!--
 * Does the RFC advance the goals of the project?
 * Does the RFC uphold the values of the project?
 * Were all of the stakeholders appropriately represented in the discussion?
 * If any stakeholders objected, does the Eng Council understand the objections
   fully?
-->
 * 该 RFC 是否对推进了项目的目标？
 * 该 RFC 是否坚持了项目的价值观？
 * 是否所有利益相关者在讨论中都有合适的代表？
 * 如果有利益相关者反对，工程委员会是否充分了解反对意见？
<!--
Decisions made by the Eng Council can be escalated to the governing authority
for the project.
-->
工程委员会所做出的决定可以上报至项目管理机构。
<!--
## Documentation
-->
## 文档
<!--
This RFC serves as documentation for the RFC process.
-->
本 RFC 是 RFC 流程的文档。
<!--
## Drawbacks, Alternatives, and Unknowns
-->
## 缺点、替代方案和未知项
<!--
The primary cost of implementing this proposal is that introducing a formal
decision-making process might slow down the pace of decision-making. The process
might be heavier than necessary for some kinds of decisions.
-->
实施本提案的主要代价是，引入一个正式的决策流程会减缓决策的速度。该流程对于一些类型的决策而言可能显得多此一举。
<!--
Recording decisions in the source repository has the effect of making those
decisions more difficult to change. That effect might be positive in some
scenarios, but the effect might also be negative in other scenarios.
-->
在源仓库中记录这些决定会使得这些决定更加难以修改。这种效应在某些场景下是正面的，但某些场景下也可能是负面的。

<!--
The criteria in the ["when to use the process" section](#criteria) attempts to
mitigate this drawback by scoping the process to consequential situations but
such scoping is bound to have false positives and false negatives.
-->
[“何时使用该流程”章节](#criteria)中的标准试图把流程的使用范围限定在重大情况，来降低这一缺点的影响，但是这样的限定必然会有误判和漏判。

<!--
There are a large number of possible alternative strategies for solving the
underlying problem. For example, we could use a decision-making process that
centers around a synchronous meeting, but such a process will have difficulty
scaling to a global open-source project. We could also have selected a different
decision-making mechanism that balanced more towards consensus or more towards
authority.
-->
也有很多可行的替代方法来解决这些潜在问题。譬如，我们可以采用以同步会议为中心的决策流程，但是这种流程很难扩展到一个全球化的开源项目之中。我们也可以选择一种不同的决策机制，以更倾向于共识或更倾向于权威。

<!--
## Prior art and references
-->
## 现有技术和参考文献
<!--
There is a good deal of prior art about decision-making processes for
open-source projects. This proposal is strongly influenced by the following
existing processes:
-->
在开源项目中，已经有很多现有的决策流程。本提案受到以下现存流程的极大影响：

<!--
 * *IETF RFC process.* The IETF has run a successful, large-scale
   [decision-making process](https://ietf.org/standards/process/) for a long
   period of time. The process described in this document draws a number of
   ideas from the IETF process, including some of the terminology.
-->
 * **IETF RFC 流程。** IETF 项目长期使用了一个成功的大规模[决策流程](https://ietf.org/standards/process/)。本文档中描述的流程吸收了很多来自于 IETF 流程的想法，包括一些术语。

<!--
 * *Rust RFC process.* The Rust community runs an [RFC
   process](https://github.com/rust-lang/rfcs/blob/HEAD/text/0002-rfc-process.md),
   which has been effective at making decisions for somewhat similar software
   engineering project. The process described in this document is fairly
   directly modelled after the Rust RFC process.
-->
 * **Rust RFC 流程。** Rust 社区运营了 [RFC 流程](https://github.com/rust-lang/rfcs/blob/HEAD/text/0002-rfc-process.md)，有效地帮助了一些相似的软件工程项目做出了决策。本文档中描述的流程相当直接地模仿了 Rust RFC 流程。
<!--
 * *Blink Intent-to-implement process.* The Chromium project runs a
   [decision-making process](https://www.chromium.org/blink/launching-features)
   for behaviors that affect web pages. The process described in this document
   is informed by my (abarth) experience helping to design and run that process
   for a period of time.
-->
  * **Blink 意图实施（Intent-to-implement）流程。** Chromium 项目在影响到网页的行为上使用了一个[决策流程](https://www.chromium.org/blink/launching-features)。在本文档中描述的流程是根据我（abarth）帮助设计和运行 Blink 的流程一段时间的经验而成的。
<!--
 * *FIDL Tuning Proposal.* The Fuchsia project has had direct experience using a
   similar process [to make decisions about the FIDL
   language](/contribute/governance/deprecated-ftp-process.md). This
   proposal exists because of the success of that decision-making process.
-->

 * **FIDL 完善建议。** Fuchsia 项目在 [FIDL 语言决策](contribute/governance/deprecated-ftp-process.md)流程中直接使用过一个相似的流程。该决策流程很成功，从而使得这个提案依旧存在。
