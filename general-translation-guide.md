# 文档总体翻译指南

本指南包含了翻译时需要灵活差阅的既有参考资料，和翻译时的注意细节。

欢迎各位继续补充和提议。如有新的规范要求，请在 issue 中提出，由成员来一同评估。

如需参考翻译术语，请查阅[翻译术语表](glossary-translation.md)。

## Google 公司帮助文档链接汇总

- Google 帮助中心

  https://support.google.com/
  
- Google Cloud 文档
  
  https://cloud.google.com/docs?hl=zh-cn （中） 
  
  https://cloud.google.com/docs?hl=en-us （英）

- Android 开发者文档
  
  https://developer.android.com/docs?hl=zh-cn （中）
  
  https://developer.android.com/docs?hl=en-us （英）

- Firebase 文档
  
  https://firebase.google.com/docs?hl=zh-cn （中）
  
  https://firebase.google.com/docs?hl=en-us （英）

- Google 搜索中心 – Google 开发者
  
  https://developers.google.com/search?hl=zh-cn （中）
  
  https://developers.google.com/search?hl=en-us （英）

- Google 开发者（部分文档有中文）
  
  https://developers.google.com/

## 文档总体翻译规范

### 文件格式规范

1. 原文档中同时使用了 markdown 和 html 标记，在翻译时需要将英文文段使用 `<!-- -->` 注释掉，并**请注意将注释标记单独成行**，以便 GitHub 给出可读性更强的文件差异。具体格式如下：

   - 原文：

     ```markdown
     Most users control the build through [fx](/development/build/fx.md).
     The following documents provide details of build configuration and internal
     structure.
     ```

   - 译文：

     ```markdown
     <!-- 
     Most users control the build through [fx](/development/build/fx.md).
     The following documents provide details of build configuration and internal
     structure.
     -->
     大多用户通过 [fx](/development/build/fx.md) 来控制构建。下面的文档提供了构建配置和内部结构的细节。
     ```

   更多 markdown 和 html 语法请自行查阅，建议在翻译中针对不懂的位置进行查询即可，不必完全掌握。

1. 英文和中文非标点、数字和中文非标点间请用 1 个空格分割；英文或数字与中文标点间请不要空格。可以考虑使用[该软件](https://pypi.org/project/zhlint/)或类似程序进行自动化排版。

1. **请不要参照英文写法在译文段落内添加单个换行**。单个换行会使得渲染后的文档内出现多余的空格。

1. 为提高校对时文本的可读性，请尽量分段翻译，避免一次性注释过多段落并进行大段翻译。



### 翻译规范

1. 请至少在同一目录下保持措辞一致。

1. 为符合中文文档语言习惯，在行文中，对于第二人称单数代词请使用“您”；在非条目列举的句子中，若为祈使句，请在合适位置加入“请”。

1. 请尽量使用中文标点符号。请尽可能保留英文文档原文中的句尾标点符号，并将其替换为合适的中文标点符号。对于不能保留的，请按照中文习惯更换标点符号。

1. **中文不常使用复数名词**，所以请去掉中文翻译后附加的英文标注中的复数名
词。如：

    - 软件包（package）&nbsp;✓

    - 软件包（packages）✗

1. 请注意中文句法，补全句子的逻辑关系连词。如：“如果”后加“那么”。

1. **请避免对中文使用<del>*斜体*</del>。** 英文中常用斜体表示强调或作品名称，但在中文里斜体字形并不常见，且阅读时亦不美观。
  
   对于作品名称的情况，可以使用《书名号》标注。
  
   对于强调的情况，可以考虑使用“引号”标注或\*\***黑体**\*\*代替。
   
1. 请不要翻译文档中的**超链接**内容，即下文中的 `B` 处：

    ```markdown
    [A](B)
    [A][B]
    ```

1. 在翻译时，请不要忘记翻译**代码块内的可读英文注释**部分。

1. **满足以下情况之一的内容不译。**

   1. 任何形式的代码区域内，本身是代码的。
   1. 翻译过于困难且讨论无果的专有名词。

1. **对原文或译文标注的推荐做法。**
   
   在 Google 公司的帮助文档中已有译例的，不适用本条的标注方式，请按照译例进行翻译。

   - **译文作正文，标注原文。** 如果满足下列条件之一：

     1. 对于翻译结果不够确定的。
     1. 原文是描述有中文翻译（不论默认语言是否是英文）的软件界面、操作的。
     1. 原文是适配了中文的输出内容的。
     1. 原文是内联代码格式的、具有可读性的非代码内容的。
     1. 原文描述的是英文语境内容，强调了该处英文形式，且具有准确中文翻译的。
     
     对于**所在位置不是标题中的**，请使用下面的方式标注：

     - 如果原文不涉及全称和简称问题，则请在译文对应位置后使用全角括号标注英文原文。另外，如果原文在代码区内，则请在代码区外标注。
     - 如果原文同时具有全称和简称，或原文是未给出全称的专业术语简称，则请在译文对应位置后使用全角括号依次标注全称和简称，并在中间使用全角逗号分隔。另外，如果原文在代码区内，则请在代码区外标注。
      
     例如：

     - 原文：

        ```markdown
        [**Products**][products-source] and [**Boards**][boards-source]

        Execute-only memory (XOM)

        just-in-time (JIT) compilation
        ```

     - 译文：

        ```markdown
        [**产品**][products-source]（Product）和[**板型**][boards-source]（Board）

        只执行内存（Execute-only memory，XOM）

        即时 (JIT）编译
        ```

      具体地，第一例中只包含全称，因此正常使用全角括号标注；第二例中包含了英文简称，故在全角括号中依次标注全称和简称，并使用全角逗号分隔；第三例在 [Google 公司的帮助文档](https://developer.android.com/guide/platform#:~:text=%E9%A2%84%E5%85%88%20(AOT)%20%E5%92%8C-,%E5%8D%B3%E6%97%B6%20(JIT)%20%E7%BC%96%E8%AF%91,-%E4%BC%98%E5%8C%96%E7%9A%84)中已有译例，因此按照固定方式翻译和标注。

      对于**所在位置是标题中的**，请使用下面的方法标注：

      - 如果在正文中出现了该概念，则应当在正文进行标注，而不应在标题中进行标注。
      - 如果在正文中未出现该概念，则除非：（1）翻译具有较大歧义性，或（2）在其他文章中再次出现（即证明本身是重要概念），这些情况可以在标题中标注原文，否则不应在标题中标注原文（即不进行标注）。

    - **原文作正文，标注译文。** 如果满足下列条件之一：

      1. 原文是未适配中文的输出内容的。
      1. 原文描述的是英文语境内容，强调了该处英文形式，但没有准确中文翻译的。如：无准确翻译或不常用的俗语、谚语、俚语等。

      则请保留英文原文，并在原文之后使用全角括号标注译文。另外，如果原文在代码区内，则请在代码区外标注。

## 固定翻译句式

为保证文档风格统一性，以下句式按照固定句式翻译。

- For *A*, see *B*.

  要获取 *A*，请参阅 *B*。

- For *A* on *B*, (... , ) see *C*.
    
  要获取关于 *B* 的 *A*，（……，）请参阅 *C*。

