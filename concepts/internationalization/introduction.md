<!-- 
# Internationalization
 -->
# 国际化

<!-- 
Modern user-facing operating systems need to support users across a variety of
regions, languages, and cultures. Localization and internationalization are
critical to this goal.
 -->
当代面向用户的操作系统需要支持跨不同地区、语种和文化的用户。本地化和国际化对于该目标至关重要。

<!-- 
**Localization** (L10N) is the process of translating user interfaces into local
languages and conventions. This includes text and image content, fonts, regional
variations for components like date, time, text and imagery direction, currency,
and number formats.
 -->
**本地化**（localization，L10N）是将用户界面翻译按照当地语言和习惯进行翻译的过程。这包括文本和图像内容、字体、组件的地域变体诸如日期、时间、文本和图像方向、货币和数字格式。

<!-- 
**Internationalization** (I18N) is the process of designing locale-independent
software that makes _localization_ in a wide range of languages, regions, and
cultures easy.
 -->
**国际化**（internationalization，I18N）是设计独立于原始语言的软件的过程，这样的软件易于广泛的语言、地区和文化_本地化_。

<!-- 
Typically, localization refers to the work done by language experts, designers,
tech writers, and other content producers. Internationalization refers to the
work done by software engineers.
 -->
通常情况下，本地化是由语言专家、设计师、科技作家和其他内容创作者完成的工作。国际化是由软件工程师完成的工作。

<!-- 
## Fuchsia's approach {#fuchsias-approach}
 -->
## Fuchsia 的方法 {#fuchsias-approach}

<!-- 
Because Fuchsia is [designed to be extensible][inclusive] and to flexibly
support a variety of runtimes and target products, the core operating system
does not strictly specify how to implement i18n and l10n.

However, Fuchsia does include some basic conventions, shared libraries, and
services for i18n/l10n, and uses these in the components and runners that are
typically distributed with Fuchsia. For more information, see the
[Internationalization][i18n-guide] section in the Development guides.
 -->
由于 Fuchsia 是[为扩展性设计][inclusive]的（designed to be extensible），并且灵活地支持各种运行时和目标产品，因此核心操作系统并不严格指定如何实现国际化和本地化。

不过，Fuchsia 确实包含一些针对 i18n/l10n 的基本约定、共享库和服务，并且在 Fuchsia 本身通常分发的组件和运行器中使用了它们。要获取更多信息，请参阅开发指南中的[国际化][i18n-guide]部分。

<!--xrefs-->

[i18n-guide]: /development/internationalization
[inclusive]: /concepts/principles/inclusive.md#fuchsia_architecture_is_inclusive_by_design
