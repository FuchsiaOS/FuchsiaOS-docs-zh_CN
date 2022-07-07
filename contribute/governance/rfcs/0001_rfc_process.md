{% set rfcid = "RFC-0001" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!--##  Summary-->
## 总结

<!--The Fuchsia RFC process is intended to provide a consistent and transparent path-->
<!--for making project-wide, technical decisions. For example, the RFC process can-->
<!--be used to evolve the project roadmap and the system architecture.-->

Fuchsia RFC 工作流是为项目级别的技术决定提供一个固定且透明的工作流程。比如，RFC 工作流对项目和系统架构的迭代有帮助。

<!--## Motivation-->
## 目标

<!--Currently, the Fuchsia project does not have a formal process for making-->
<!--project-wide, technical decisions. At our current scale, this informality-->
<!--results in different people having different, sometimes inconsistent, viewpoints-->
<!--on where the project is going and how the system is put together. By-->
<!--establishing a consistent and transparent path for making project-wide,-->
<!--technical decisions, all the stakeholders can be confident about the technical-->
<!--direction of the project.-->

现在，Fuchsia 项目在项目级别的技术决定上并没有一个正式的工作流程。以我们目前来说，这样没有个标准将会给我们带来在项目的发展路线以及系统架构上的不同的看法的麻烦。通过建立这样一个固定且透明的项目标准，技术决策，以及所有的相关人员都能在项目发展路线上达成统一。

<!--## Design-->
## 原理 

<!--This section describes the design of the RFC process.-->
本节将讲述 RFC 的工作流程。

<!--### When to use the process {#criteria}-->
### 使用场景 {#criteria}

<!--The vast majority of changes to Fuchsia do not require an RFC. Instead, these-->
<!--changes can be made using the [code review-->
<!--process](development/source_code/contribute_changes.md). However,-->
<!--technical decisions that have broad impact across the project require broader-->
<!--agreement and must be socialized with the project using the RFC process.-->

大多数的 Fuchsia 修改并不会使用到 RFC 。但是，这些修改可以使用这个： [code review process](development/source_code/contribute_changes.md) 。有些时候一些技术方面的决策影响广泛，所以这就需要使用 RFC 流程来让更多的人授权


<!--The following kinds of changes must use the RFC process:-->
下面列举了必须使用 RFC 流程的情况：

<!-- * *Changing the project roadmap.* The project roadmap describes changes that-->
   <!--have broad impact across the system, often touching a large fraction of the-->
   <!--system or crossing boundaries between subsystems.-->

 * * 项目发展路线 * 项目发展路线的决策对系统有很大的影响，经常会牵涉到系统的一大块，或者是多个子系统。

<!-- * *Adding constraints on future development.* Some decisions, once made,-->
   <!--constrain the future development of the system. We need to be careful when-->
   <!--making such decisions because they can be difficult to revise later.-->

 * * 未来开发规划 * 一些技术的选择一旦敲定，就会严格限制着系统的未来。我们在做这些规划的时候一定要慎重，因为再去补救是很难的。

<!-- * *Making project policy.* Project policies have broad impact across the-->
   <!--system, often affecting contributors throughout the project. For example,-->
   <!--changing the set of supported languages impacts everyone who needs to debug-->
   <!--and understand the system, even if not everyone uses the new language.-->

 * * 制订项目准则 * 项目准则对系统有着广泛的影响，常常影响着项目贡献人员。比如，修改支持语言列表会影响到测试人员和这个系统的学习人员，因为并不是所有人使用新语言。

<!-- * *Changing the system architecture.* The system architecture describes how the-->
   <!--system fits together as a whole. Changing the system architecture, by-->
   <!--definition, crosses boundaries between subsystems and requires careful-->
   <!--consultation with many stakeholders.-->

 * * 修改系统架构 * 系统架构描述了系统这个整体如何协作。根据定义，更改架构，跨子系统的话需要仔细和大多数的相关人员咨询。


<!-- * *Delegating decision-making authority.* There are often classes of decisions-->
   <!--that the project needs to make frequently and that benefit from specialized-->
   <!--expertise. Rather than making all these decisions through the RFC process,-->
   <!--the project can delegate decision-making authority for those classes of-->
   <!--decisions to another group or process. For example, we often need to make-->
   <!--decisions about platform APIs, which add constraints on future development,-->
   <!--but it would not be practical to use the RFC process for every change to the-->
   <!--platform API.-->

 * * 决策委托 * 本项目经常会有一些需要特殊领域的专家参与的决定。这时我们使用决策委托给其他组或者走别的流程，而不是使用 RFC 流程。比如，我们经常修改平台的 API 接口，这些接口限制着未来的开发工作，但是它们也并不都是用到 RFC 流程的。

<!-- * *Escalations.* Finally, contentious changes can benefit from the transparency-->
   <!--and clarity of the RFC process. If there is a disagreement about technical-->
   <!--direction that cannot be resolved by an individual technical leader, the-->
   <!--decision can be escalated to the RFC process either by one of the disagreeing-->
   <!--parties or by another contributor.-->

 * * 决策分发 * RFC 流程的透明、清晰特性会对不断的更改帮助很大。如果对于一个技术方向有歧义，个人领导者拿不定主意，这个时候领导者或者该组织的任何一个贡献者就可以把它放到 RFC 流程上。

<!--The RFC process may also be used for other kinds of changes that would benefit-->
<!--from its structured approach to decision making and its durable record of the-->
<!--d-->ecision.

RFC 流程同样也可以放在其他一些因为它的构造而受益的更改或者难以决定的事情上，

<!--### Roles and responsibilities-->
### 定位和功能

<!--People interact with the RFC process in several roles:-->
使用者在 RFC 流程中的角色有如下几种：

<!-- * *RFC Authors.* An RFC Author is a person who writes an RFC. Everyone who-->
   <!--contributes to Fuchsia can be an RFC Author. A given RFC can have one or more-->
   <!--authors. The authors of a given RFC drive the process for that RFC.-->

 * * RFC 编写者 * RFC 编写者是提交 RFC 的人。每个 Fuchsia 项目贡献者都可以是编写者。一个 RFC 可以有一个或多个编写人员。这些编写人员推动了 RFC 的流程。

<!-- * *Stakeholder.* A stakeholder is a person who has a stake in whether the-->
   <!--project accepts a given RFC. Stakeholders are typically Fuchsia contributors,-->
   <!--but some RFCs might have stakeholders beyond the Fuchsia project. For-->
   <!--example, stakeholders might be involved in other projects that use Fuchsia or-->
   <!--are otherwise affected by changes to Fuchsia. Stakeholders do not always-->
   <!--participate directly in discussions about RFCs. Instead, stakeholders are-->
   <!--often *represented* by someone, often a technical lead or other person-->
   <!--responsible for a group of stakeholders.-->

 * * 参与人员 * 参与人员是在一个项目中是否接受指定 RFC 拥有投票权的人。参与人员当然也是本项目的贡献人员，当然也有一些 RFC 投票人员是没有参与 Fuchsia 项目。例如，参与人员可能是在其他项目中使用到了 Fuchsia 或者是受到了该项目的某一个修改的影响。

<!-- * *Eng Council.* The [Eng Council](../eng_council.md) facilitate discussion-->
   <!--and make the final decision as to whether the project accepts an RFC.-->

 * * 议员 *  [议员](../eng_council.md) 维持讨论以及做最后是否接受一个 RFC 流程的决定。

<!--### How the process works-->
### 工作原理

<!--This section describes each step involved in the RFC process.-->
本小节讲述 RFC 的工作步骤。

<!--#### Step 1: Socialize {#socialize}-->
#### 第一步：分享 {#socialize}

<!--The first step in the RFC process is to socialize your idea with the project.-->
<!--For example, you might have noticed a problem that you think is important to-->
<!--solve. Are other people aware of this problem? Someone else might already be-->
<!--working on the problem or might have some background or context about the-->
<!--problem that would be useful to you. The earlier you discover this information,-->
<!--the better.-->

RFC 流程的第一步就是把你在项目中的想法分享出去。比如，你发现了一个很重要需要解决的问题。其他人注意到这个问题了吗？或许已经有人在着手解决这个问题了，在或者有一些其他的相关资料可以帮到你。总之越早发现你的想法，对项目越好。

<!--Compared to the remaining steps in the process, this step is relatively-->
<!--informal. This document does not contain a rigorous description of how to-->
<!--socialize your idea. Socializing technical ideas is a skill unto itself.-->
<!--However, a good place to start is to raise the topic in discussions with the-->
<!--technical leads for areas related to the problem you are trying to solve. For-->
<!--example, you might want to consult with people in the `OWNERS` files for the-->
<!--areas of the codebase will need to be modified to execute your idea.-->


相比于接下来的步骤，这一步算是非正式了。本片文章并不会严格要求你如何分享你的想法。分享技术想法本身就是很重要的。不管怎么样，一个好的起点，是能引起你发现的问题相关的人员的注意和讨论的。譬如，你最好直接和文件 `拥有者` 咨询有关代码，以应对修改或者执行你的建议的需求。

<!--If you are unsure how to socialize your idea, consider asking a technical leader-->
<!--for advice. They will often have more experience socializing ideas and might be-->
<!--able to point you in a good direction.-->

如果你对如何分享你的想法有疑问，可以考虑向技术领导者寻求建议。通常他们在这方面更有经验，所以也能给你指明一条捷径。


<!--> *Example.* This RFC was socialized by having a discussion in the Eng Forum,-->
<!--> which is a regular meeting inside Google of various engineering leaders-->
<!--> involved in the project. The RFC was also socialized with the creators of the-->
<!--> FTP and CTP process, who have good background and context about these-->
<!--> processes.-->

> * 例子 * 这条 RFC 是经过议员论坛的讨论后分发的， 
> 讨论的形式是来自 Google 的、参与这个项目的多个工程师领导在例会上讨论得出的。
> 这条 RFC 同样分享给了 FTP 和 CTP 栈的创建者，因为他们在这方面的流程有丰富的经验。

<!--#### Step 2: Draft {#draft}-->
### 步骤二：起草 {#draft}

<!--Once you have gathered all the background and context you can through-->
<!--socialization, you are ready to start the formal part of the RFC process. The-->
<!--next step is to write a first draft of the RFC document itself.-->

当你准备好所有材料后，你就可以开始正式编写了。下一步就是编写 RFC 文档的初稿。

<!--Mechanically, an RFC is a markdown file in the-->
<!--`/contribute/governance/rfcs` directory.-->
<!--To create and RFC, you create a CL that adds a file to that directory. You-->
<!--should start by making a copy of the [RFC template](TEMPLATE.md). While not-->
<!--strictly required, the template is designed to guide you towards writing a-->
<!--high-quality RFC by prompting you to think through the problem you are trying to-->
<!--solve in a semi-structured way.-->

正常情况下， 一份 RFC 是一个存放来 `/contribute/governance/rfcs` 路径下的 markdown 文件。创建 RFC 的第一步，要先在目录下创建一个 CL 。建议从模版文件 [RFC template](TEMPLATE.md) 复制一份并在此基础上编写。模版并不是硬性要求，但是模版能帮助你更好、更高效的把你对于问题的想法和方案展现出来。


<!--Do not worry about assigning a number to your RFC at this stage. Instead, use-->
<!--`NNNN` as a placeholder. For example, the file name should be something like-->
<!--`NNNN_my_idea.md`. The RFC will get a number shortly before landing.-->

在这个阶段中并不需要过多关注你 RFC 的一些数字命名。你可以使用 `NNNN` 先做个占位符。比如，一个文件的名字可能会是这样的： `NNNN_my_idea.md` 。RFC 的文档都会在真正敲定之前给出一串确切的数字的。

<!--> *Suggestion.* Consider marking the CL containing your RFC as a-->
<!--> "work-in-progress" until you are ready for feedback.-->


> * 建议 * 可以在你得到答复之前把表示你的 RFC 的 CL 标志为“进行中”的

<!--#### Step 3: Iterate {#iterate}-->
#### 第三步：推进 {#iterate}

<!--Once you have created a CL containing the first draft of your RFC, you are ready-->
<!--to iterate on your idea with the appropriate stakeholders. Hopefully you will-->
<!--have already discovered most the appropriate stakeholders as part of socializing-->
<!--your idea, but you are very likely to discover additional stakeholders at this-->
<!--stage.-->

当你创建好表示你 RFC 的 CL 后，你就可以把你的想法和相关人员进行交流了。希望你已经找到了合适的相关人员，不过通常你在此过程中还会发现更多的相关人员。

<!--Mechanically, you should invite stakeholders to provide feedback on your RFC by-->
<!--adding them to the "Reviewers" or "CC" fields in the CL, as you would for a-->
<!--normal code review. The stakeholders should provide you feedback by leaving-->
<!--comments on your RFC in the code review tool.-->

通常，当你想尽快得到回复时，你可以通过在 CL 的 “Reviewers“ 或者 “CC“ 字段添加上相关人员来邀请他们对你的 RFC 进行审查反馈。相关人员会通过代码审查工具在你的 RFC 中以评论的形式提供反馈。

<!--If the discussion is too complex for the code review tool, consider scheduling a-->
<!--meeting with the relevant stakeholders to have a more efficient discussion.-->
<!--After the meeting, you must post a summary of the meeting in a comment on the CL-->
<!--so that people who were not at the meeting can understand what was discussed-->
<!--during the meeting.-->

如果讨论内容对于代码审查工具来说过于复杂，可以考虑和相关人员使用更有效率的会议形式。会议结束后，你在 CL 中发布一段会议总结，这样才可以使得没有参与会议的那部分人能理解你们在回忆中讨论的东西。

<!--If the discussion becomes contentious, please escalate to one of the RFC-->
<!--Editors. The Eng Council can help move the discussion forward, for example by-->
<!--providing additional structure to the discussion or moving the discussion to-->
<!--another forum. Regardless of how the discussion proceeds, the results of any-->
<!--off-CL discussion must be captured in the CL, often by posting a summary of the-->
<!--discussion as a CL comment.-->

如果讨论有争议，请把讨论内容发给 RFC 的编辑。议员可以帮你推进讨论，比如，可以提供一些别的视角或者把这次讨论移步到论坛中。不论讨论是以何种形式，最终都是要记录在 CL 中的，当然大多数情况下都是以评论的形式把讨论结果总结放上去。


<!--If you wish to withdraw your RFC, you can mark the CL containing the RFC as-->
<!--abandoned. You, or someone else, can always resurrect your RFC later if-->
<!--circumstances change. If you are resurrecting an RFC created by someone else,-->
<!--you should start the RFC process over from the beginning, but you can use the-->
<!--withdrawn RFC as a starting point rather than `TEMPLATE.md`. Please confer with-->
<!--the original authors to determine whether they wish to continue to have their-->
<!--names associated with the new incarnation of the RFC.-->

当你需要撤销 RFC 的时候，可以标识代表你 RFC 的 CL 为废弃状态。之后如果有需要，任何参与人员都可以重新恢复。如果你需要恢复其他人创建的 RFC ，这个时候就需要你从头开始了，唯一的好处就是你不用从模版开始了。请一定要和原作者确认下他们是否想要参与到你恢复的新的 RFC 中。

<!--> *Suggestion.* If you are interested in RFCs, consider configuring the Gerrit-->
<!--> Code Review tool to [send you an email > notification](https://gerrit-review.googlesource.com/Documentation/user-notify.html)-->
<!--> when a CL modifies the `/contribute/governance/rfcs` directory.-->


> * 建议 *  如果你对 RFC 感兴趣，可以配置下杰瑞特代码审查工具 (Gerrit Code Review) ，让它在你的 CL 修改 `/contribute/governance/rfcs` 目录时给你发送 [ 邮件 ](https://gerrit-review.googlesource.com/Documentation/user-notify.html)
。

<!--#### Step 4: Approve {#approve}-->
#### 第四步：通过 {#approve}

<!--Once the iterations on the RFC are converging, you are ready to move to the-->
<!--approval stage, in which the stakeholders sign-off on the RFC by setting the-->
<!--Code-Review flag to either +1 or +2. Typically, stakeholders who need to approve-->
<!--a CL (i.e., whose sign-off is required for the RFC to move forward) should-->
<!--sign-off with a +2 whereas stakeholders whose approval is not required should-->
<!--sign-off with a +1, but all stakeholders are welcome to sign-off with a +2 if-->
<!--they wish to express their enthusiasm for the RFC.-->

当你的 RFC 逐步推进时，最后就会到审核通过阶段，这个阶段中参与人员就会通过给你的 RFC 代码审查标志为 +1 或者 +2 。通常，参与人员需要通过一个 CL （比如，某个人的签收就需要 RFC 走个流程）时，就需要标为 +2 ， 如果不是必须的，就可以 +1 。大多数情况下，参与人员都会打 +2 来表示他们对与当前 RFC 的积极态度。

<!--Stakeholders who wish to object to an RFC can set the Code-Review flag to -1 or-->
<!---2, depending on how strongly they feel that the RFC should not move forward.-->
<!--When setting the Code-Review flag to -1 or -2, a stakeholder must state their-->
<!--reason for objecting, ideally in a way that would let someone understand the-->
<!--objection clearly without having to read the entire discussion that preceded-->
<!--the objection.-->

参与人员可以标为 -1 或者 -2 来表示反对，至于是哪一个，则取决于他们反对的程度。当一个 RFC 的代码审查标为 -1 或者 -2 的时候，打标人员必须阐述说明原因，理想的表述是不用阅读完就能理解他的反对理由。


A stakeholder setting the Code-Review flag to -1 or -2 does not necessarily
prevent the project from accepting the RFC. See the ["How decisions are made"
section](#how-decisions-are-made) below for more details about how the project
decides whether to accept an RFC.

参与人员的 -1 或者 -2 的标识并不能完全决定是否接受一个 RFC 。参考下面介绍 [ 结果如何判定 ](#how-decisions-are-made) 详细了解一个 RFC 的判决是如果决定的。

<!--After all the stakeholders have weighed in with their Code-Review flags, send an-->
<!--email to eng-council@fuchsia.dev to prompt the Eng Council to decide whether to-->
<!--accept your RFC.-->

当所有参与人员打标结束，就可以给 eng-council@fuchsia.dev 发送邮件，提醒议员可以判决是否接受你的 RFC 了。

<!--#### Step 5: Submit {#submit}-->
#### 第五步：提交 {#submit}

<!--If the project decides to accept your RFC, a member of the Eng Council will-->
<!--comment on your CL stating that the RFC is accepted and will assign the RFC a-->
<!--number, typically the next available number in the series. If there are any -1-->
<!--or -2 Code-Review flags, the Eng Council will explicitly clear each flag by-->
<!--summarizing the objection and by describing why the RFC is moving forward-->
<!--despite the objection.-->

如果项目决定接受你的 RFC ，议员团就会有一个人通过在你的 CL 中评论的形式声明这条 RFC 被接受了，并且会给 RFC 分配一串数字，通常是下一个可用的数字。如果 RFC 中有 -1 或者 -2 的反对，议员会把那些标识转换为有多少数反对票，并且解释描述为什么给予该 RFC 通过。

<!--If the project decides to reject your RFC, a member of the Eng Council will-->
<!--comment on your CL stating that the RFC is rejected and providing a rationale-->
<!--for the rejection.  Rejected RFCs are valuable engineering artifacts. The Eng-->
<!--Council will work with the RFC Authors to land a version of the RFC that is-->
<!--marked as rejected and incorporates the rationale.-->

如果你的 RFC 被拒绝了，议员团中也会有一个人出来在你的 CL 中阐述为什么会被拒绝，以及原因。被拒绝的 RFC 对工程师们来说也是很重要的。 议员也会和 RFC 的作者一起落实那些被拒绝的 RFC 的根本原因。

<!--You should upload a new patchset of your RFC with the assigned number, both in-->
<!--the title of the RFC and in the filename. If your RFC is approved and requires-->
<!--implementation, please make sure you have an issue filed in the issue tracker-->
<!--and put a link to the issue in the header of your RFC.-->

你应该为你的 RFC 想一串数字，并添加到 RFC 名称中和对应的文件名中。如果你通过的 RFC 需要部署，请确保在问题跟踪中留有记录，并且在你的 RFC 内容头部中放上该链接。

<!--The Eng Council will then mark your CL Code-Review +2 and you can land your RFC!-->

议员会把你的 CL 的代码审查标识为 +2，之后你就可以落实你的 RFC 了。

<!--*Congratulations! You have contributed a valuable engineering artifact to the-->
<!--project!*-->

* 恭喜你，你已经为该项目提交了一份宝贵建议。*

<!--### How decisions are made {#how-decisions-are-made}-->
### 结果如何判定 {#how-decisions-are-made}

<!--The decision whether to accept an RFC is made by the Eng Council, acting in-->
<!--[rough consensus](https://en.wikipedia.org/wiki/Rough_consensus) with each-->
<!--other. If the decision involves an RFC that has Eng Council members as authors,-->
<!--those members must recuse themselves from the decision.-->

议员决定一条 RFC 是否接受， 决策流程和 [初步统一](https://en.wikipedia.org/wiki/Rough_consensus) 里的流程一样。如果一项 RFC 的作者有议员，那所有涉及议员都不得参与决策。

<!--If the Eng Council cannot reach rough consensus, the RFC is not accepted.-->
<!--In deciding whether to accept an RFC, the Eng Council will consider the-->
<!--following factors:-->

如果议员不能达成一致，相关 RFC 就不会被接受。在考虑是否接受 RFC 的时候，议员需考虑如下几点：

 <!--* Does the RFC advance the goals of the project?-->
 * 该 RFC 是否对项目目标有推动作用？
 <!--* Does the RFC uphold the values of the project?-->
 * 该 RFC 是否对项目的价值没有影响？
 <!--* Were all of the stakeholders appropriately represented in the discussion?-->
 * 是否所有参与者都参与了讨论？
<!-- * If any stakeholders objected, does the Eng Council understand the objections-->
   <!--fully?-->
 * 如果有不同意见，议员是否充分了解反对意见？

<!--Decisions made by the Eng Council can be escalated to the governing authority-->
<!--for the project.-->
由议员所做出的决定应当分享给项目管理人员。

<!--## Documentation-->
## 文档

<!--This RFC serves as documentation for the RFC process.-->
下面这些 RFC 为 RFC 流程提供文档说明作用。

<!--## Drawbacks, Alternatives, and Unknowns-->
## 缺点，替代选择，以及未解

<!--The primary cost of implementing this proposal is that introducing a formal-->
<!--decision-making process might slow down the pace of decision-making. The process-->
<!--m-->ight be heavier than necessary for some kinds of decisions.

在实施这个提议的过程中，主要损失是引入一个正式的决策流程可能会降低效率。在某些场合会显得效率低下。

<!--Recording decisions in the source repository has the effect of making those-->
<!--decisions more difficult to change. That effect might be positive in some-->
<!--scenarios, but the effect might also be negative in other scenarios.-->

在代码仓库中记录这些决策会有难以更改的影响。这些影响在某些方面是好的，某些场景下是负面的。

<!--The criteria in the ["when to use the process" section](#criteria) attempts to-->
<!--mitigate this drawback by scoping the process to consequential situations but-->
<!--such scoping is bound to have false positives and false negatives.-->

在 [" 什么时候使用 " 章节 ](#criteria) 标准中，尝试把流程延伸到结果的环境中来弥补它的缺点，但是这样做又会带来错误的好与不好的影响。

<!--There are a large number of possible alternative strategies for solving the-->
<!--underlying problem. For example, we could use a decision-making process that-->
<!--centers around a synchronous meeting, but such a process will have difficulty-->
<!--scaling to a global open-source project. We could also have selected a different-->
<!--decision-making mechanism that balanced more towards consensus or more towards-->
<!--authority.-->

目前有很多可行的方法来规避那些潜在问题。譬如，我们可以把决策放到同步会议中，但是这样一个流程在一个全球化的开源项目中很难实现。我们可以选择一个不同的，可以平衡好意见统一和授权的决策机制，

<!--## Prior art and references-->
## 精巧之处和参考

<!--There is a good deal of prior art about decision-making processes for-->
<!--open-source projects. This proposal is strongly influenced by the following-->
<!--existing processes:-->

在开源项目中，决策流程也有其独到之处。对下面的流程也有很大的影线：

<!-- * *IETF RFC process.* The IETF has run a successful, large-scale-->
   <!--[decision-making process](https://ietf.org/standards/process/) for a long-->
   <!--period of time. The process described in this document draws a number of-->
   <!--ideas from the IETF process, including some of the terminology.-->

 * * IETF RFC 流程 * IETF 项目长期使用了一个大规模的、成功的 [ 决策流程 ](https://ietf.org/standards/process/)。这个文档中的流程为 IETF 项目提供了很多的想法，包括一些术语。

<!-- * *Rust RFC process.* The Rust community runs an [RFC-->
   <!--process](https://github.com/rust-lang/rfcs/blob/HEAD/text/0002-rfc-process.md),-->
   <!--which has been effective at making decisions for somewhat similar software-->
   <!--engineering project. The process described in this document is fairly-->
   <!--directly modelled after the Rust RFC process.-->

 * * Rust RFC 流程 * Rust 社区使用了 [ RFC 流程 ](https://github.com/rust-lang/rfcs/blob/HEAD/text/0002-rfc-process.md)，高效率的帮助一些相似的软件工程项目做出了决策。 这篇文章中的描述是在 Rust RFC 流程后做了相当直接的修改过的。

<!-- * *Blink Intent-to-implement process.* The Chromium project runs a-->
   <!--[decision-making process](https://www.chromium.org/blink/launching-features)-->
   <!--for behaviors that affect web pages. The process described in this document-->
   <!--is informed by my (abarth) experience helping to design and run that process-->
   <!--for a period of time.-->

  * * 闪烁 声明即实现流程  * Chromium 项目在影响到网页的行为上使用了这样一个[ 决策流程 ](https://www.chromium.org/blink/launching-features)。这个文档中描述了一个受我经验影响设计和实现的一个使用一段时间的流程。

<!-- * *FIDL Tuning Proposal.* The Fuchsia project has had direct experience using a-->
   <!--similar process [to make decisions about the FIDL-->
   <!--language](contribute/governance/deprecated-ftp-process.md). This-->
   <!--proposal exists because of the success of that decision-making process.-->


 * * FIDL 完善建议 * Fuchsia 项目在 [ FIDL 语言的决策流程 ](contribute/governance/deprecated-ftp-process.md) 中使用过一个相似的流程。这个流程因它的成功而依旧存在。

