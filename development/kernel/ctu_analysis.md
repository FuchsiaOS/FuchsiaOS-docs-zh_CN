# <!-- Cross Translation Unit Static Analysis in Zircon -->

# Zircon 交叉翻译单元静态分析

<!-- This document describes: -->

这篇文章下面要阐述：

- 在 Zircon 上如何使用 Clang 静态分析（CSA）来设置交叉翻译单元分析（CTU）
- 这部分工作已经由 Kareem Khazem 在实习期间完成了，剩下的工作需要 CTU 全面支持

* <!-- How to set up cross-translation-unit analysis (**CTU**) with the Clang Static Analyzer (**CSA**) in Zircon;-->
* <!-- The work done by Kareem Khazem during his internship; and -->
* <!-- The remaining work needed to get CTU fully supported on Zircon. -->



## <!-- Setting up and running CTU on Zircon -->

## 在 Zircon 上设置并运行 CTU

<!-- **Summary**: Download the source for Clang, and apply several non-mainline patches to it before compiling it. Run my wrapper script around the analysis tools. Download the `CodeChecker` tool; use it to digest the results of the analysis, and start a web server to view the results with a web interface. -->

**总结**：下载 Clang 的源码并在编译之前使用非主分支补丁。运行分析工具的已封装好的脚本，下载 `CodeChecker` 工具，使用它来理解分析的结果，并启动 web 服务来显示结果的网页接口。

## <!-- CTU-enabling patches -->

## CTU 开启补丁

<!-- There are two patchsets to be aware of: -->

以下有2个补丁需要注意的：

* <!-- The [Samsung](https://github.com/haoNoQ/clang/tree/summary-ipa-draft) patchset, which is an enormous patch adding AST merging support to Clang. It consists mostly of additions to `lib/AST/ASTImporter.cpp`. There is also a (primitive, not working very well) set of tools for CTU analysis under `tools/xtu-build/*`. This patchset is based on an old revision of Clang; this fact, as well as its large size, makes it very difficult to rebase wholesale onto tip-of-tree (**ToT**) Clang. -->
* [三星补丁](https://github.com/haoNoQ/clang/tree/summary-ipa-draft) 在 Clang 添加了 AST 支持的庞大补丁，对 `lib/AST/ASTImporter.cpp` 的补充，在 `tools/xtu-build/*` 下有一组工具（原始的，效果不佳）用于CTU分析工具。这个补丁是在 Clang 的旧版上。事实上，它的大小很大，要结合到最新的 Clang 分支是很困难的。
* <!-- The [Ericsson](https://github.com/dkrupp/clang/blob/ctu-master/tools/xtu-build-new/xtu-analyze.py) patchset, which includes a subset of Samsung’s AST merging work and also adds several new tools (`tools/xtu-build-new/*` and `tools/scan-build-py/*`) that allow for CTU analysis. The xtu-build-new tools improve on, and are somewhat different to, Samsung’s xtu-build tools. This patchset is much newer than the Samsung one, and the authors are making an effort to keep it rebased on ToT. -->
* [爱立信](https://github.com/dkrupp/clang/blob/ctu-master/tools/xtu-build-new/xtu-analyze.py) 的补丁，其中包括三星 AST 合并工作的一个子集，还添加了几个允许 CTU 分析的新工具（`tools/xtu-build-new/*` 和 `tools/scan-build-py/*`）。 xtu-build-new 工具改进了三星的 xtu-build 工具，但与三星的 xtu-build 工具有所不同。这个补丁集比三星的要新得多，作者正在努力使其重新基于 ToT（最新的分支）。

<!-- We will be patching Clang with Ericsson’s patchset, since the AST merging work rebases cleanly and we also get the newer analysis tools. However, note that CTU support for Zircon is incomplete; in some cases, the Samsung patchset contains code that provides the required functionality (more details below). -->

我们将使用 Ericsson 的补丁集修补 Clang，因为 AST 合并工作重新建立在干净的基础，并且我们还获得了更新的分析工具。但是，请注意 CTU 对 Zircon 的支持是不完整的；在某些情况下，三星补丁集包含提供所需功能的代码（更多详细信息见下文）。

### <!-- Steps to build CTU-capable CSA -->

## CTU-capable CSA 构建的步骤

<!-- Download and build Clang and LLVM as usual. -->

1. 像往常一样下载和构建 Clang 和 LLVM

    <!-- In a separate directory, clone Ericsson’s fork of Clang and switch to the ctu-master branch.-->

2. 在一个单独的目录中，clone 爱立信的 Clang 分支并切换到 ctu-master 分支

    <!-- Download [this script](https://gist.github.com/karkhaz/d11efa611a1bde23490c2773dc0da60d) into Ericsson’s fork and run it. It should dump a series of patches into a patches directory. I purposely only dump the commits from the beginning of Ericsson’s changes until 1bb3636, which was the latest revision during my internship. -->

3. 下载[这个脚本](https:gist.github.comkarkhazd11efa611a1bde23490c2773dc0da60d)到爱立信的分叉并运行它。它应该将一系列补丁转储到补丁目录中。我故意把从爱立信的修改开始到我实习期间的最新修改1bb3636为止的提交内容全部删除。

    <!-- If you want more up-to-date changes from Ericsson, you can experiment with changing 1bb3636 to HEAD in the script. Make sure to skip commits that merge upstream commits into the ctu-master branch by specifying additional ranges in the script. git log --graph can be helpful to determine what the upstream commits vs. Ericsson’s commits are, I use -->

    * 如果您想从 爱立信 获得更多最新更改，您可以尝试在脚本中将 1bb3636 更改为 HEAD。通过在脚本中指定其他范围，确保跳过将上游提交合并到 ctu-master 分支的提交。 git log --graph 可以帮助确定上游提交与爱立信的提交是什么，我使用

    ```
    git log --graph  --decorate --date=relative --format=format:'%C(green)%h%C(yellow) %s%C(reset)%w(0,6,6)%C(bold green)\n%C(cyan)%G? %C(bold red)%aN%C(reset) %cr%C(reset)%w(0,0,0)\n%-D\n%C(reset)' --all
    ```

    <!-- Apply the generated patches to *upstream* Clang (not the Ericsson fork) one at a time. -->

4. 一次一个地将生成的补丁应用到 *upstream* Clang（不是 爱立信 fork）。

   ```
   for p in $(ls $PATCH_DIR/*.patch | sort -n); do git am < $p; done
   ```

   <!-- Apply Kareem Khazem’s patches that are listed [below](#zircon-patches) if they haven’t already landed -->

5. 应用 [下面](#zircon-patches) 列出的 Kareem Khazem 的补丁，如果他们还没有登陆的话

    <!-- Re-build upstream Clang & LLVM. -->

6. 重构建最新的 Clang & LLVM

## <!-- Running CTU analysis -->

## 运行 CTU 分析

<!-- **Summary:** Run my wrapper script. This builds Zircon normally, then builds it again but dumping serialised ASTs instead of object files, and then finally analyses each file using the dumped ASTs to achieve CTU. --->

**总结：**运行我的包装脚本。这将正常构建 Zircon，然后再次构建它但转储序列化的 AST 而不是目标文件，然后最后使用转储的 AST 分析每个文件以实现 CTU

### <!-- How CTU works -->

### CTU如何工作的

<!-- First, the story backwards: -->

首先，事情的背景如下：

<!-- Non-CTU static analysis analyzes the AST of each TU; any function calls to external functions are treated as opaque. Roughly, CTU analysis seeks to *substitute* the opaque function call node with the AST of that function’s implementation.  -->

非CTU静态分析, 分析每个TU的AST；任何对外部函数的调用都被视为不透明的。粗略地说，CTU分析试图用该函数实现的AST来*替代*不透明的函数调用节点。



<!-- Thus, a CTU analysis will start analyzing an AST as usual, but when it encounters a function call node, it will try to *merge in* the AST for that function. This relies on the AST for the function already having been serialized to disk beforehand, so that the analyzer can re-load the AST into memory. It also relies on support for AST merging, which is what the Samsung patch to `ASTImporter.cpp` (and the Ericsson patch derived from it) is for.-->

因此，CTU分析将像往常一样开始分析一个AST，但是当它遇到一个函数调用节点时，它将尝试*合并*该函数的AST。这有赖于该函数的AST事先已经被序列化到磁盘上，这样分析器就可以将AST重新加载到内存中。这也依赖于对AST合并的支持，这也是三星对`ASTImporter.cpp`的补丁（以及由其衍生的爱立信分支）的作用。



<!-- In order to serialize the ASTs to disk, we need to emulate the real build process. The way to do this is to actually do a real build of Zircon while recording the compiler invocations; this allows us to ‘play back’ the invocations, but with the compiler flags modified to dump AST files rather than object files.-->

为了将AST序列化到磁盘，我们需要模拟真实的构建过程。这样做的方法是在记录编译器调用的同时，对Zircon进行一次真正的构建；这允许我们 "回放 "调用，但要修改编译器的标志，以转储AST文件而不是对象文件。



<!-- So to summarise, forwards this time: --> 

所以总结如下：

<!-- Build zircon using Clang, and wrap the build process in a program like [bear](https://github.com/rizsotto/Bear) in order to record compiler invocations and generate a JSON compilation database.-->

<!-- Replay the same compilation steps, but dumping AST files instead of object files. This is what the [xtu-build.py](https://github.com/dkrupp/clang/blob/ctu-master/tools/xtu-build-new/xtu-build.py) tool does.-->

<!-- Perform static analysis as usual, but deserialize the AST of every called function when needed. This is what the [xtu-analyze.py](https://github.com/dkrupp/clang/blob/ctu-master/tools/xtu-build-new/xtu-analyze.py) tool does at the top level, by invoking tools in the [scan-build-py/libscanbuild](https://github.com/dkrupp/clang/tree/ctu-master/tools/scan-build-py/libscanbuild) directory through the thin [scan-build replacement](https://github.com/dkrupp/clang/blob/ctu-master/tools/scan-build-py/bin/scan-build) written by the Ericsson team. -->



* 使用Clang构建zircon，并将构建过程包裹在 [bear](https://github.com/rizsotto/Bear) 这样的程序中，以便记录编译器的调用并生成JSON编译数据库。
* 重放同样的编译步骤，但转储AST文件而不是对象文件。这就是 [xtu-build.py](https://github.com/dkrupp/clang/blob/ctu-master/tools/xtu-build-new/xtu-build.py) 工具的作用。
* 像往常一样进行静态分析，但在需要时对每个调用的函数的AST进行反序列化。这就是 [xtu-analyze.py](https://github.com/dkrupp/clang/blob/ctu-master/tools/xtu-build-new/xtu-analyze.py) 工具在顶层所做的，通过爱立信团队编写的轻盈的 [scan-build replacement](https://github.com/dkrupp/clang/blob/ctu-master/tools/scan-build-py/bin/scan-build) 调用 [scan-build-py/libscanbuild](https://github.com/dkrupp/clang/tree/ctu-master/tools/scan-build-py/libscanbuild) 目录下的工具。



<!-- These steps are captured in the [Fuchsia wrapper](#fuchsia-wrapper-script) mentioned below. The result of all this is a directory full of reports, in [Apple plist](https://en.wikipedia.org/wiki/Property_list) format, which contain details of reported bugs.-->

这些步骤在下面提到的 [Fuchsia wrapper](#fuchsia-wrapper-script) 中有所体现。这一切的结果是一个充满报告的目录，以 [Apple plist](https://en.wikipedia.org/wiki/Property_list) 的格式，其中包含了报告的bug的细节。



### <!-- Ericsson’s wrapper scripts--> {#ericsson-wrapper-script}

### 爱立信封装脚本 {#ericsson-wrapper-script}

<!-- There are two sets of tools for running cross-translation-unit analysis:-->

有两套工具用于运行跨单元的分析: 

<!-- The tools under `tools/xtu-build-new` are the top-level scripts. Since the underlying analyzer can fail (i.e. due to the CSA crashing), I’ve patched `xtu-analyze.py` (in Ericsson’s branch) so that it dumps the output of the analyzer (stdout/stderr, not the report) to a file. The output goes in ``$OUT_DIR/{passes,fails}`` depending on the return code of the analyzer, where `$OUT_DIR` is the directory passed to the `-o` argument of `xtu-analyze.py`. The particularly helpful part of those files is the *second line* that starts with `analyze: DEBUG: exec command in`, which is emitted by the `libscanbuild` tools (next bullet point). That command is the actual invocation to the CSA after the long, tedious process of modifying its command line. Therefore, it’s the command that you will need if you want to run the CSA on a troublesome file using gdb.-->

<!-- The tools under `tools/scan-build-py` are a bird’s nest of tools to wrap around the actual invocation to Clang. They are responsible for modifying the command line. I’m not too familiar with them, and haven’t had to interfere with them in the past. -->

* `tools/xtu-build-new` 下的工具是顶层的脚本。由于底层分析器可能失败（即由于CSA崩溃），我对`xtu-analyze.py`（在Ericsson的分支中）进行了修补，使其将分析器的输出（stdout/stderr，而不是报告）转储到一个文件中。输到`$OUT_DIR/{passes,fails}`中，这取决于分析器的返回代码，其中`$OUT_DIR`是传递给`xtu-analyze.py`的`-o`参数的目录。这些文件中特别有用的部分是以`analyze.py`开头的*第二行*。`DEBUG: exec command in`，这是由`libscanbuild`工具发出的（下一个要点）。该命令是在修改CSA的命令行的漫长而乏味的过程之后，对CSA的实际调用。因此，如果你想用gdb在一个有问题的文件上运行CSA，你将需要这个命令。
* `tools/scan-build-py`下的工具是一个鸟巢式的工具，用来包裹对Clang的实际调用。它们负责修改命令行。我对它们不是很熟悉，而且过去也没有干涉过它们。



### <!-- Fuchsia wrapper script  -->

### Fuchsia 封装的脚本{#fuchsia-wrapper-script}

<!-- [This very small shell script](https://gist.github.com/karkhaz/c8ded50e564d73853731266fec729454) wraps the Ericsson `xtu-build-new` wrappers. To do a complete analysis of Zircon, make sure to clean first, and specify the correct path to your build of Clang. Then, in the zircon directory: -->

[这个非常小的shell脚本](https://gist.github.com/karkhaz/c8ded50e564d73853731266fec729454)包装了爱立信`xtu-build-new`包装器。要对Zircon进行完整的分析，请确保首先进行清理，并指定你构建的Clang的正确路径。然后，在zircon目录下

```
ninja -t clean && ninja && ./run.sh
```

<!-- In order to build only the kernel, specify a `TARGET` as an environment variable:-->

为了只构建内核，指定一个 `TARGET` 作为环境变量：

```
ninja -t clean && ninja clean && TARGET=./build-zircon-pc-x64/zircon.elf ./run.sh
```

<!-- The script also requires [clangify.py](https://gist.github.com/karkhaz/2ab5e8c7a8783318d44ceca715f20438) to be in the zircon directory with executable bit set. After the analysis has finished, there will be a `.result-xtu` directory, containing:-->

该脚本还要求[clangify.py](https://gist.github.com/karkhaz/2ab5e8c7a8783318d44ceca715f20438)在zircon目录下，并设置可执行位。分析结束后，会有一个`.result-xtu`目录，其中包含。



<!-- A bunch of Apple plist files, which are the bug reports;-->

<!-- A fails directory, containing the std{out,err} of analyzer invocations that returned non-zero;-->

<!--A passes directory, containing the std{out,err} of analyzer invocations that returned 0.-->

* 一堆Apple plist文件，这些是错误报告。
* 一个失败目录，包含了分析器调用的std{out,err}，返回非零的。
* 一个pass目录，包含分析器调用的std{out,err}，返回0的。

## <!--Viewing analysis results-->

### 查看分析结果

<!-- At the moment, the only way of parsing the plist reports and viewing them with a web interface is by using the [CodeChecker](https://github.com/Ericsson/codechecker) tool, which is developed at Ericsson and used for code comprehension and many other tasks. CodeChecker needs a large number of dependencies installed, and it’s best to install them with **pip** or **npm** or whatever rather than using **apt-get**. In short, after performing the analysis and dumping the plists into .result-xtu, you can invoke `CodeChecker plist` to parse the plists: -->

目前，解析plist报告并使用网络界面查看报告的唯一方法是使用[CodeChecker](https://github.com/Ericsson/codechecker)工具，该工具由爱立信开发，用于代码理解和其他许多任务。CodeChecker需要安装大量的依赖项，最好用**pip**或**npm**或其他方式安装，而不是用**apt-get**。简而言之，在进行分析并将plist转储到.result-xtu后，你可以调用`CodeChecker plist`来解析plist。

```
CodeChecker plist -d .result-xtu -n 2016-12-12T21:47_uniq_name -j 48
```

<!-- The argument to `-n` needs to be unique on each invocation of `CodeChecker plist`, as it represents a single parse run. CodeChecker complains otherwise. Then, run `CodeChecker server` to start a webserver on `localhost:8001`, which will display the reports of all previous parsing runs.-->

`-n`的参数在每次调用`CodeChecker plist`时需要是唯一的，因为它代表了一次解析运行。否则CodeChecker会报错。然后，运行`CodeChecker server`在`localhost:8001`上启动一个webserver，它将显示所有先前的解析运行报告。



## <!-- Getting Help-->

### 获取帮助

<!-- The Samsung patchset was authored by [Aleksei Sidorin](mailto:a.sidorin@samsung.com) and his team. Aleksei is quite knowledgeable about `ASTImporter.cpp` and other AST merging aspects, and was very helpful. He and [Sean Callanan](mailto:scallanan@apple.com) were happy to review my AST Importer patches. Aleksei also [gave a relevant talk](https://www.youtube.com/watch?v=jbLkZ82mYE4) about summary-based interprocedural analysis at the 2016 LLVM Developers Meeting.-->

三星的补丁集是由[Aleksei Sidorin](mailto:a.sidorin@samsung.com)和他的团队编写的。Aleksei对 `ASTImporter.cpp` 和其他AST合并方面的知识相当了解，而且非常有帮助。他和[Sean Callanan](mailto:scallanan@apple.com)都很乐意审查我的AST Importer补丁。Aleksei还在2016年的LLVM开发者会议上[做了一个相关的演讲](https://www.youtube.com/watch?v=jbLkZ82mYE4)，介绍了基于摘要的程序间分析。

<!--The Ericsson patchset was authored by [Gábor Horváth](mailto:xazax.hun@gmail.com) and his team. Gábor was very helpful with advice on how to run CTU analysis with the `xtu-build-new` tools.-->

爱立信的补丁集是由[Gábor Horváth](mailto:xazax.hun@gmail.com)和他的团队编写的。Gábor在如何使用`xtu-build-new`工具运行CTU分析方面提供了很多建议。

<!-- I ([Kareem Khazem](mailto:karkhaz@karkhaz.com)) am also happy to help out where I can.-->

我（[Kareem Khazem](mailto:karkhaz@karkhaz.com)）也很乐意在我力所能及的地方提供帮助。

<!-- The LLVM irc channel can also be helpful.-->

LLVM的irc频道也可以提供帮助。



## <!-- Zircon-specific analyses-->

### Zircon-specific 分析

<!--Upstream Clang has been very receptive to receiving patches for Zircon-specific Clang checkers. The [MutexInInterruptContext](https://reviews.llvm.org/D27854) checker is one example (ported from an LLVM pass written by Farid Molazem Tabrizi), as are the [SpinLockChecker](https://reviews.llvm.org/D26340) and [MutexChecker](https://reviews.llvm.org/D26342). Potential reviewers for Clang checks are Devin Coughlin (from Apple), Artem Dergachev (on Aleksei Sidorin’s team at Samsung) and Anna Zaks (also at Apple).-->

上游Clang一直非常乐于接受针对Zircon的Clang检查器的补丁。[MutexInInterruptContext](https://reviews.llvm.org/D27854)检查器是一个例子（移植自Farid Molazem Tabrizi编写的LLVM程序），还有[SpinLockChecker](https://reviews.llvm.org/D26340)和[MutexChecker](https://reviews.llvm.org/D26342)。Clang检查的潜在审查者是Devin Coughlin（来自苹果）、Artem Dergachev（在三星的Aleksei Sidorin团队）和Anna Zaks（也在苹果）。



<!--These checkers are typically *opt-in*, meaning that you need to pass a flag to the analyzer to enable them: something like `-analyzer-checker=optin.zircon.MutexInInterruptContext`.-->

这些检查器通常是*opt-in*，意味着你需要给分析器传递一个标志来启用它们：比如`-分析器-检查器=optin.zircon.MutexInInterruptContext`。

<!-- If those patches haven’t landed in Clang, you will need to apply them. To use them for analyzing Zircon with the [Ericsson wrapper scripts](#ericsson-wrapper-script), you should modify the [Fuchsia wrapper script](#fuchsia-wrapper-script) by adding the option `-e optin.zircon.MutexInInterruptContext` to the invocation of `xtu-analyze.py` at the end of the file. The patch for `MutexInInterruptContext` has a test suite, which can be used as an example of what the analysis is capable of.-->

如果这些补丁还没有在Clang中出现，你将需要应用这些补丁。要使用它们来分析[Ericsson wrapper scripts](#ericsson-wrapper-script)的Zircon，你应该修改[Fuchsia wrapper script](#fuchsia-wrapper-script)，在文件末尾的`xtu-analyze.py`调用中加入选项`-e optin.zircon.MutexInInterruptContext`。`MutexInInterruptContext`的补丁有一个测试套件，可以作为分析能力的一个例子。



# <!-- Progress on CTU support in Zircon-->

# CTU 支持在Zircon上的发展

## <!-- Problems fixed in the AST importer-->

## 在AST导入器的问题修复

<!--The upstream CSA crashes on the vast majority of Zircon files. This section describes some of the problems that Kareem Khazem encountered and their fixes.-->

上游CSA在绝大多数的Zircon文件上都会崩溃。本节介绍了Kareem Khazem遇到的一些问题及其解决方法。

### <!--Unsupported AST Nodes -->

### 不支持 AST 节点{#zircon-patches}

<!-- The Clang Static Analyzer is unable to import a lot of Zircon code, due to not having implemented support for importing certain kinds of AST nodes. Patches to support these nodes are listed here:-->

Clang静态分析器无法导入大量的Zircon代码，这是因为没有实现对导入某些类型的AST节点的支持。支持这些节点的补丁在此列出。

AtomicType                    | Patch merged into upstream
------------------------------|--------------------------------
`CXXDependentScopeMemberExpr` | [`https://reviews.llvm.org/D26904`](https://reviews.llvm.org/D26904)
`UnresolvedLookupExpr`        | [`https://reviews.llvm.org/D27033`](https://reviews.llvm.org/D27033)
`DependentSizedArray`         | &nbsp;
`CXXUnresolvedConstructExpr`  | &nbsp;
`UsingDecl`                   | [`https://reviews.llvm.org/D27181`](https://reviews.llvm.org/D27181)
`UsingShadowDecl`             | [`https://reviews.llvm.org/D27181`](https://reviews.llvm.org/D27181)
`FunctionTemplateDecl`        | [`https://reviews.llvm.org/D26904`](https://reviews.llvm.org/D26904)

<!-- In general, when implementing support for new node types, one must implement a `VisitNode` function in `ASTImporter.cpp`, and also unit tests and functional tests; Kareem’s patches above contain examples. There are still quite a few unsupported AST nodes remaining; grep the analyzer output directory for `error: cannot import unsupported AST node`.-->

一般来说，在实现对新节点类型的支持时，必须在`ASTImporter.cpp`中实现`VisitNode`函数，还要实现单元测试和功能测试；Kareem的上述补丁包含了一些例子。仍然有不少不支持的AST节点；在分析器输出目录中搜索 "error: cannot import unsupported AST node"。

<!-- The Ericsson patchset contains only a subset of the `ASTImporter` code in the Samsung patchset. In some cases, the `Visit` function for an unsupported node can be taken straight from the Samsung patchset. However, the Samsung patchset does not include any tests, so it will still be necessary to write tests before the support for that node is upstreamed.-->

爱立信补丁集只包含了三星补丁集中 "ASTImporter "代码的一个子集。在某些情况下，不支持的节点的`Visit'功能可以直接从三星补丁集中获取。然而，Samsung补丁集不包括任何测试，所以在对该节点的支持上行之前，仍然需要编写测试。



### Segfaults galore

<!--A lot of the code in `ASTImporter.cpp` is buggy. Sometimes Aleksei has private patches for issues, like [this one](https://reviews.llvm.org/D26753), so it’s worth giving him (**a-sid**) a quick ping on IRC. My strategy for debugging is to look through the wrapper output for the *second* string starting with `analyze: DEBUG: exec command in` (followed by the actual command line of the analyzer), and running that command line through gdb. It often takes only a few hours to track down where a segfault is coming from.-->

`ASTImporter.cpp`中的很多代码都有错误。有时Aleksei会对问题进行私人修补，比如[这个](https://reviews.llvm.org/D26753)，所以值得在IRC上给他(**a-sid**)一个快速的ping。我的调试策略是通过包装器的输出来寻找以 "analyze "开头的*第二个字符串*。`DEBUG: exec command in`（后面是分析器的实际命令行），并通过gdb运行该命令行。通常只需要几个小时就能追踪到一个segfault的来源。



## <!--Bugs found before and after CTU-->

## 在CTU之前和之后发现错误

### <!--Possible bug in VFS?-->

### 在 VFS 可能的错误

<!--This is a double-free of `oldparent`, which is declared uninitialized on `system/ulib/fs/vfs.c:vfs_rename`. Two lines later, `vfs_walk` (same file) is called with `oldparent` as its second argument. It is possible to return from `vfs_walk` without assigning to `oldparent` by entering the for loop and hitting the `return r` statement on the first loop. If the value of `r` is greater than zero, then we go to the `else if` statement, which calls `vn_release` on `oldparent` (which is still uninitialized).-->

这是对`oldparent`的双重释放，它在`system/ulib/fs/vfs.c:vfs_rename`中被声明为未初始化。两行之后，`vfs_walk`（同一文件）被调用，`oldparent`是其第二个参数。通过进入for循环并在第一个循环中点击 "return r "语句，可以从`vfs_walk`返回而不分配给`oldparent`。如果`r`的值大于0，那么我们就进入`else if`语句，在`oldparent`上调用`vn_release`（它仍然是未初始化的）。



### <!--Possible bug in thread?-->

### 在线程上可能出现的错误

<!--This is a use-after-free. The path is:-->

这是使用后释放，路径如下：

* `kernel/kernel/thread.c:thread_detach_and_resume`
    * Call `thread_detach(t)`
        * Return `thread_join(t, NULL, 0)`
            * free `t` and return `NO_ERROR`
        * Return `NO_ERROR`
    * Check for error is 1false1
    * Call `thread_resume(t)`, which has been freed.
        * `thread_resume` then accesses `t`’s fields.
* `kernel/kernel/thread.c:thread_detach_and_resume`
    * 调用 `thread_detach(t)`
      * 返回 `thread_join(t, NULL, 0)`
        * 释放 `t` 并返回 `NO_ERROR`
      * 返回 `NO_ERROR`
    * 检测错误是1 返回 false
    * 调用 `thread_resume(t)`, 这时已经释放了
      * `thread_resume` 之后反问`t` 的变量

## <!-- CTU false positives-->

### CTU 误报

<!--The CSA cannot resolve the implementation of functions that are called through function pointers. This means that it cannot make any assumptions about what the return value of the function might be, nor any effects that the function might have on output parameters.-->

* CSA不能解决通过函数指针调用的函数的实现问题。这意味着它不能对函数的返回值作出任何假设，也不能对函数可能对输出参数产生的任何影响作出假设。

<!--There are several classes of function whose implementations are not accessible to the analyzer. Again, the analyzer cannot know that such functions touch their output arguments, so they will spuriously report that the following code reads from a garbage value:-->

* 有几类函数的实现是分析器无法访问的。同样，分析器不能知道这类函数触及它们的输出参数，所以它们会虚假地报告说下面的代码是从一个垃圾值中读取的。

  ```
  struct timeval tv;
  gettimeofday(&tv, NULL);
  printf("%d\n", tv.tv_usec);   // [SPURIOUS REPORT] Access to
                                // uninitialized variable ‘tv’
  ```

  <!-- Some kinds of functions that are vulnerable to this kind of imprecision include:-->

* 容易受到这种不精确性影响的一些函数种类包括:

    <!--System calls (like `gettimeofday`)-->

    <!--Compiler builtins (like `memcpy`)-->

    * 系统调用（像 `gettimeofday`）
    * 编译构建（像`memcpy`）
