<!---

# FIDL design principles

--->

# FIDL设计原则

<!---

This page summarizes the key design principles that FIDL has adopted over time.

--->

本页总结了一些FIDL随着时间推移采用的核心设计原则。

<!---

## Priority of constituencies

--->

## 最终用户优先

<!---

FIDL aims to respect the following priority of constituencies:

1. Users (using a Fuchsia product)
2. End-developers (using FIDL bindings)
3. Fuchsia contributors (using FIDL bindings)
4. API designers (authoring FIDL libraries)
5. [Fuchsia FIDL Team] members

This list was adapted from that of the [API Council Charter].

--->

FIDL是意在遵从于以下用户优先次序

1. 用户（使用Fuchsia产品）

2. 终端开发者（使用FIDL绑定）

3. Fuchsia贡献者（使用FIDL绑定）

4. API设计者（创作FIDL库）

5. [Fuchsia FIDL组]成员

该列表根据[API协会章程]改编

<!---

## ABI first, API second

--->

## 优先ABI，其次API

<!---

From [RFC-0050: Syntax revamp][rfc-0050-principles]:

> FIDL is primarily concerned with defining Application Binary Interface (ABI)
> concerns, and second with Application Programming Interface (API) concerns.

--->

来自：[RFC-0050: Syntax revamp][rfc-0050-principles]:

> FIDL主要优先关注应用二进制接口（ABI），其次关注的是应用程序接口（API）

<!---

## Binary wire format first {#binary-wire-format-first}

From [RFC-0050: Syntax revamp][rfc-0050-binary-wire-format-first]:

> While many formats can represent FIDL messages, the [FIDL Wire
> Format][wire-format] (or "FIDL Binary Wire Format") is the one which has
> preferential treatment, and is catered to first ... we choose to over rotate
> towards the binary ABI format when making syntax choices.

--->
## 二进制线型格式优先


来自：[RFC-0050: Syntax revamp][rfc-0050-binary-wire-format-first]:

>尽管有多种格式可以代表FIDL消息，我们选择[FIDL Wire Format][wire-format](或者"FIDL二进制线型格式")作为推荐使用格式并优先满足...在进行语法选择时，选择转换到二进制ABI的格式。

<!---

## Fewest features

From [RFC-0050: Syntax revamp][rfc-0050-fewest-features]:

> We strive to have the fewest features and rules, and aim to combine features
> to achieve use cases. In practice, when considering new features, we should
> first try to adapt or generalize other existing features rather than introduce
> new features.

--->

## 更少特性

来自：[RFC-0050: Syntax revamp][rfc-0050-fewest-features]:

> 我们力求采用最少的特性和规则，并计划以合并特性方式实现用例。在实际使用中考虑新特性时，我们应当首先试图调整或归纳其他已有的特性，而不是引入新特性。

<!---

## You only pay for what you use {#you-only-pay}

From [RFC-0027: You only pay for what you use][rfc-0027]:

> When adding functionality to FIDL, we should evaluate the costs that adding
> that functionality imposes on people who use FIDL but do not use the new
> functionality. We should then have a very high bar for accepting functionality
> that imposes costs on people who do not use the functionality.

For example, [RFC-0047: Tables][rfc-0047-motivation] followed this principle by
adding tables to the language rather than replacing structs:

> Tables are necessarily more complicated than structs, and so processing them
> will be slower and serializing them will use more space. As such, it's
> preferred to keep structs as is and introduce something new.

In contrast, [RFC-0061: Extensible unions][rfc-0061-pros-and-cons] reached the
opposite decision of replacing static unions with extensible unions, but only
after a careful analysis of the tradeoffs. Unlike with tables, the extra cost
imposed by extensible unions is marginal or nonexistent in most cases.
--->

## 你仅需为所得付出代价{#you-only-pay}
来自：[RFC-0027: You only pay for what you use][rfc-0027]:
>当增加功能到FIDL中，我们应当评估增加该功能是否会给使用此FIDL但不使用此新功能的人带来开销。那么，我们就应该设定一个很高的标准来接受该功能，使不使用这些功能的人不受到影响。

例如， [RFC-0047: Tables][rfc-0047-motivation] 遵循该原则追加列表到语言中，而不是替换结构。

> 列表必然比结构体复杂，因此处理它们会变慢，序列化它们会占用更多空间。正因如此，我们更倾向保持结构不变，并引入一些新的东西。

相比之下，[RFC-0061: Extensible unions][rfc-0061-pros-and-cons] 经过仔细的权衡分析，则采用不同策略，使用扩展联合体来取代静态联合体。在大多数情况下，对比列表而言。扩展联合体的额外开销是微不足道的或者不存在的。

<!---

## Solve real problems

We design FIDL to solve real problems and address real needs, not imagined ones.
We avoid designing a "solution in search of a problem".

For example, FIDL initially did not support empty structs because it was unclear
how to represent them in C/C++. In [RFC-0056: Empty structs][rfc-0056], we saw
users were employing workarounds and recognized the need for an official
solution. Only then did we add empty structs to the language.

--->

## 解决实际问题

我们设计FIDL就是为了解决实际问题和真实需求，而不是想象的问题。我们需避免“工具规律”（solution in search of a problem）。

例如，FIDL最初不支持空结构体是因为在C/C++中不能清楚体现。在[RFC-0056: Empty structs][rfc-0056]中，我们可以看见使用者采用变通方法，并意识到需要一个官方的解决方案。所以我们才在语言中追加了空结构体。

<!---

## Optimize based on data

Optimizing without data is useless at best and dangerous at worst. When
designing optimizations (e.g. performance, binary size), we follow the data.

For example, [RFC-0032: Efficient envelopes][rfc-0032] was initially accepted,
but later rejected. In hindsight, it should never have been accepted because
there was no data to back it up.

--->

## 基于数据优化

在没有实际数据的前提下进行优化，充其量仅是无用优化，最坏情况下则是件危险的事情。当设计优化时（例如：性能，库大小），我们应当遵从基于数据的原则。

例如，在[RFC-0032: Efficient envelopes][rfc-0032] 原本是被接受的，但是后来被拒绝。事后看来，是因为没有数据支持，它就不应该被接受。

<!---

## No breakage at a distance

We strive to avoid _breakage at a distance_. Changes in one place should not
cause surprising breakage in a faraway place. For example, it would be
surprising if adding a struct named `Foo` to a FIDL file broke compilation
because another FIDL file in a completely different part of the codebase already
had a type named `Foo`. This is why FIDL, like most programming languages, uses
namespaces to limit the scope of name collisions.

[RFC-0029: Increasing method ordinals][rfc-0029-breakage-at-a-distance]
discusses this problem as it relates to protocol composition. [RFC-0048:
Explicit union ordinals][rfc-0048-hashing-only-for-protocols] revisits the
topic, explaining why FIDL only uses hashing for protocols.

[RFC-0057: Default no handles][rfc-0057] introduced a distinction between [value
and resource types][lang-resource]. One motivation for this was providing the
`Clone` trait in Rust for types without handles without breakage at a distance:

> Although FIDL bindings _can_ conditionally enable code based on the presence
> of handles, doing so is undesirable because it breaks evolvability guarantees.
> For example, adding a field to a table is normally safe, but adding a handle
> field would become source-breaking &mdash; not only for that table, but for
> all types transitively containing it.

--->

## 避免连带影响（breakage at a distance)

我们应该努力避免*连带影响*。在一个地方的改变不应该造成另一个不相关的地方出现意外的损坏。例如，如果在一个FIDL文件中添加结构体 `Foo` 会破坏编译结果，令人惊讶的是原因居然是因为在一个完全不相关的地方的另一个FIDL文件中已经有相同名字 `Foo` 的结构体。这就是为什么FIDL像大多数的编程语言，使用命名空间来限制名称冲突的范围。

[RFC-0029: Increasing method ordinals][rfc-0029-breakage-at-a-distance]

因为与协议构成相关，所以讨论了这个问题

[RFC-0048:Explicit union ordinals][rfc-0048-hashing-only-for-protocols] 重谈该话题，解释了为什么FIDL仅对协议使用散列法。

[RFC-0057: Default no handles][rfc-0057]引入 [value and resource types][lang-resource]差异。这样做的一个动机是为了在Rust中提供`Clone` 特性用于没有句柄的类型，而不至于在其他地方发生破坏。

> 尽管FIDL绑定可以基于句柄的存在有条件地使能代码，但是这样做的后果不可预知的，因为这破坏了演化性保证。
>
> 例如，在列表中添加filed（字段）通常是安全的，但是添加句柄字段将破坏源—不仅对于列表，而且对包含该表的所有类型都是如此。

<!---

## Liberal syntax, idiomatic style

We do not rigidly adhere to a "one way to do it" philosophy. When we are
concerned that users will waste time deciding between trivial alternatives, we
introduce restrictions in `fidl-lint` or `fidl-format` rather than in `fidlc`.

<!-- TODO(fxbug.dev/74753): Say "the linter enforces an ordering" when it is true. -->
For example, FIDL accepts modifier keywords in any order, but we intend to
enforce a consistent ordering in the linter.

As another example, [RFC-0040: Identifier uniqueness][rfc-0040] fixed the
problem of identifiers colliding after case transformation by having `fidlc`
report an error if any two identifiers have the same canonical form. A simpler
alternative would have been to enforce FIDL naming conventions in the compiler.
However, this goes a step too far. There are valid reasons for using different
naming styles, for example in describing the Kernel API, where `snake_case`
methods are strongly preferred.

--->

## 自由语法，惯用风格

我们并不僵化坚持“用一种方式来做“的理念。我们担心用户会浪费时间来决定琐碎的选择，所以决定引入限制在 `fidl-lint` 或者 `fidl-format` 里，而不是 `fidlc`中。

<!-- TODO(fxbug.dev/74753): Say "the linter enforces an ordering" when it is true. -->

例如，尽管FIDL接受任何顺序的修饰语关键字，但我们试图强制在linter中执行一致的顺序。

正如另一个例子中，[RFC-0040: Identifier uniqueness][rfc-0040] 中修正了标识符冲突问题，即如果两个标识符拥有相同的规范形式，则经过大小写转换后`fidlc`会上报错误的异常。一个较为简单的选择则是在编译器中强制FIDL的命名惯例。尽管如此，这看起来还是太不合常理了。使用不同的命名风格还是存在合理的理由，例如在描述内核API时，我们强烈推荐`snake_case`命名法。

<!---

## Canonical representation

The FIDL wire format [is canonical][wire-format-dual-forms]: there is exactly
one encoding for a given message. As a corollary, every byte is accounted for:
there is no byte that can be changed without altering the message's meaning.

For example, the [specification][wire-format-padding] requires that all padding
bytes are zero. Similarly, [RFC-0047: Tables][rfc-0047-wire-format] disallows
storing extraneous empty envelopes to ensure a canonical representation.

A canonical representation makes FIDL simpler and more secure. For example,
allowing nonzero padding could result in FIDL messages leaking sensitive
information that happened to occupy those bytes in memory. Allowing multiple
representations for a given message also leads to rarely executed code paths
that can hide bugs, e.g. the "extra empty envelopes" code path. A canonical
representation also makes it easy to compare messages for equality without
knowing the schema: for [value types][lang-resource], it is a simple `memcmp`.

--->

## 准则性表示法

FIDL线型格式是[准则性的][wire-format-dual-forms]。对给定信息都有一个确切的编码对应。作为推论来说每一个字节都被计入：即没有一个字节可以在不改变信息含义的情况下被改变。

例如， [specification][wire-format-padding]规范定义中所有填充字节都为0。同样地，[RFC-0047: Tables][rfc-0047-wire-format] 不允许存储外来空包以此来确保准则性。

准则性表示法使FIDL更简单和安全。例如，允许填充非0可能因为这些信息正好占据了内存中的某些字节，造成FIDL泄露敏感信息。给定信息允许多种表现形式，同样也会造成很少运行的代码路径隐藏bug，例如，”外部空包“代码路径。准则性表示法同样在不知道模式的前提下能提供更简单的比较信息的平等性：对于[value types][lang-resource]，它是一个简单的 `memcmp`。

<!---

## No allocations required

From the [wire format specification][wire-format-dual-forms]:

> FIDL is designed such that **encoding** and **decoding** of messages can occur
> in place in memory.

This requirement significantly influences the design of the wire format: you
must be able to decode in place using only the stack. It is the reason the wire
format uses presence indicators and a depth-first traversal order rather than,
for example, and offset-based format that requires auxiliary data structures to
keep track of information while decoding.

This principle is related to ["You only pay for what you use"](#you-only-pay),
in that it caters to very low-level uses of FIDL where `malloc` may not yet
exist, or is prohibitively expensive.

--->

## 不需要分配

从 [wire format specification][wire-format-dual-forms]中：

> FIDL被设计为可以在内存中就地完成信息的**encoding** （编码）和**decoding** （解码）

这个需求深远地影响了线型格式的设计：你仅使用堆就能完成就地解码。这也就是线型格式使用存在指示符和深度优先遍历顺序来代替的原因，例如当解码时，基于偏移的格式就需要附加数据结构来保持信息追踪。

当前原则与["You only pay for what you use"](#you-only-pay)相关，当`malloc` 不存在时，它也可以适合FIDL底层使用，而不需要太大开销。

<!---

## Transport generality

While [the binary wire format comes first](#binary-wire-format-first), this does
not mean FIDL should be tightly coupled to the Zircon channel transport. There
are other important use cases to consider, such as describing the Kernel API,
in-process messaging, and persistence.

[RFC-0050: Syntax revamp][rfc-0050-transport-generalization] describes the
future direction for transport generalization.

[RFC-0062: Method impossible][rfc-0062] was rejected in part because it coupled
FIDL too closely to the Zircon channel transport.

--->

## 传输通用性

尽管有 [库线型格式优先](#binary-wire-format-first)原则，但是考虑到其他重要的使用场景，例如描述内核API，进程内通信和持久化存储，这不表示FIDL应当与Zircon通道传输紧耦合。

[RFC-0050: Syntax revamp][rfc-0050-transport-generalization] 描述了传输通用化的长远方向。

[RFC-0062: Method impossible][rfc-0062] 部分被拒绝，因为它让FIDL和Zircon通道传输耦合太紧密。

<!-- link labels -->
[API Council Charter]: /docs/contribute/governance/api_council.md#values
[Fuchsia FIDL Team]: /src/fidl/OWNERS
[lang-resource]: /docs/reference/fidl/language/language.md#value-vs-resource
[wire-format]: /docs/reference/fidl/language/wire-format
[wire-format-dual-forms]: /docs/reference/fidl/language/wire-format#dual-forms
[wire-format-padding]: /docs/reference/fidl/language/wire-format#padding
[rfc-0027]: /docs/contribute/governance/rfcs/0027_you_only_pay_what_you_use.md
[rfc-0029-breakage-at-a-distance]: /docs/contribute/governance/rfcs/0029_increasing_method_ordinals.md#breakage-at-a-distance
[rfc-0029]: /docs/contribute/governance/rfcs/0029_increasing_method_ordinals.md
[rfc-0032]: /docs/contribute/governance/rfcs/0032_efficient_envelopes.md
[rfc-0040]: /docs/contribute/governance/rfcs/0040_identifier_uniqueness.md
[rfc-0047-motivation]: /docs/contribute/governance/rfcs/0047_tables.md#motivation
[rfc-0047-wire-format]: /docs/contribute/governance/rfcs/0047_tables.md#wire-format
[rfc-0048-hashing-only-for-protocols]: /docs/contribute/governance/rfcs/0048_explicit_union_ordinals.md#hashing-only-for-protocols
[rfc-0050-binary-wire-format-first]: /docs/contribute/governance/rfcs/0050_syntax_revamp.md#binary-wire-format-first
[rfc-0050-fewest-features]: /docs/contribute/governance/rfcs/0050_syntax_revamp.md#fewest-features
[rfc-0050-principles]: /docs/contribute/governance/rfcs/0050_syntax_revamp.md#principles
[rfc-0050-transport-generalization]: /docs/contribute/governance/rfcs/0050_syntax_revamp.md#transport-generalization
[rfc-0056]: /docs/contribute/governance/rfcs/0056_empty_structs.md
[rfc-0057]: /docs/contribute/governance/rfcs/0057_default_no_handles.md
[rfc-0061-pros-and-cons]: /docs/contribute/governance/rfcs/0061_extensible_unions.md#pros-and-cons
[rfc-0062]: /docs/contribute/governance/rfcs/0062_method_impossible.md
