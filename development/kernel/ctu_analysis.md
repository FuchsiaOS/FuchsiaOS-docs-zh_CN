# Cross Translation Unit Static Analysis in Zircon

This document describes:

* How to set up cross-translation-unit analysis (**CTU**) with the Clang Static Analyzer (**CSA**) in Zircon;
* The work done by Kareem Khazem during his internship; and
* The remaining work needed to get CTU fully supported on Zircon.

## Setting up and running CTU on Zircon

**Summary**: Download the source for Clang, and apply several non-mainline patches to it before compiling it. Run my wrapper script around the analysis tools. Download the `CodeChecker` tool; use it to digest the results of the analysis, and start a web server to view the results with a web interface.

## CTU-enabling patches

There are two patchsets to be aware of:

* The [Samsung](https://github.com/haoNoQ/clang/tree/summary-ipa-draft) patchset, which is an enormous patch adding AST merging support to Clang. It consists mostly of additions to `lib/AST/ASTImporter.cpp`. There is also a (primitive, not working very well) set of tools for CTU analysis under `tools/xtu-build/*`. This patchset is based on an old revision of Clang; this fact, as well as its large size, makes it very difficult to rebase wholesale onto tip-of-tree (**ToT**) Clang.
* The [Ericsson](https://github.com/dkrupp/clang/blob/ctu-master/tools/xtu-build-new/xtu-analyze.py) patchset, which includes a subset of Samsung’s AST merging work and also adds several new tools (`tools/xtu-build-new/*` and `tools/scan-build-py/*`) that allow for CTU analysis. The xtu-build-new tools improve on, and are somewhat different to, Samsung’s xtu-build tools. This patchset is much newer than the Samsung one, and the authors are making an effort to keep it rebased on ToT.

We will be patching Clang with Ericsson’s patchset, since the AST merging work rebases cleanly and we also get the newer analysis tools. However, note that CTU support for Zircon is incomplete; in some cases, the Samsung patchset contains code that provides the required functionality (more details below).

### Steps to build CTU-capable CSA

1. Download and build Clang and LLVM as usual.
2. In a separate directory, clone Ericsson’s fork of Clang and switch to the ctu-master branch.
3. Download [this script](https://gist.github.com/karkhaz/d11efa611a1bde23490c2773dc0da60d) into Ericsson’s fork and run it. It should dump a series of patches into a patches directory. I purposely only dump the commits from the beginning of Ericsson’s changes until 1bb3636, which was the latest revision during my internship.
    * If you want more up-to-date changes from Ericsson, you can experiment with changing 1bb3636 to HEAD in the script. Make sure to skip commits that merge upstream commits into the ctu-master branch by specifying additional ranges in the script. git log --graph can be helpful to determine what the upstream commits vs. Ericsson’s commits are, I use

    ```
    git log --graph  --decorate --date=relative --format=format:'%C(green)%h%C(yellow) %s%C(reset)%w(0,6,6)%C(bold green)\n%C(cyan)%G? %C(bold red)%aN%C(reset) %cr%C(reset)%w(0,0,0)\n%-D\n%C(reset)' --all
    ```

4. Apply the generated patches to *upstream* Clang (not the Ericsson fork) one at a time.

   ```
   for p in $(ls $PATCH_DIR/*.patch | sort -n); do git am < $p; done
   ```

5. Apply Kareem Khazem’s patches that are listed [below](#zircon-patches) if they haven’t already landed
6. Re-build upstream Clang & LLVM.

## Running CTU analysis

**Summary:** Run my wrapper script. This builds Zircon normally, then builds it again but dumping serialised ASTs instead of object files, and then finally analyses each file using the dumped ASTs to achieve CTU.

### How CTU works

First, the story backwards:

Non-CTU static analysis analyzes the AST of each TU; any function calls to external functions are treated as opaque. Roughly, CTU analysis seeks to *substitute* the opaque function call node with the AST of that function’s implementation.

Thus, a CTU analysis will start analyzing an AST as usual, but when it encounters a function call node, it will try to *merge in* the AST for that function. This relies on the AST for the function already having been serialized to disk beforehand, so that the analyzer can re-load the AST into memory. It also relies on support for AST merging, which is what the Samsung patch to `ASTImporter.cpp` (and the Ericsson patch derived from it) is for.

In order to serialize the ASTs to disk, we need to emulate the real build process. The way to do this is to actually do a real build of Zircon while recording the compiler invocations; this allows us to ‘play back’ the invocations, but with the compiler flags modified to dump AST files rather than object files.

So to summarise, forwards this time:

* Build zircon using Clang, and wrap the build process in a program like [bear](https://github.com/rizsotto/Bear) in order to record compiler invocations and generate a JSON compilation database.
* Replay the same compilation steps, but dumping AST files instead of object files. This is what the [xtu-build.py](https://github.com/dkrupp/clang/blob/ctu-master/tools/xtu-build-new/xtu-build.py) tool does.
* Perform static analysis as usual, but deserialize the AST of every called function when needed. This is what the [xtu-analyze.py](https://github.com/dkrupp/clang/blob/ctu-master/tools/xtu-build-new/xtu-analyze.py) tool does at the top level, by invoking tools in the [scan-build-py/libscanbuild](https://github.com/dkrupp/clang/tree/ctu-master/tools/scan-build-py/libscanbuild) directory through the thin [scan-build replacement](https://github.com/dkrupp/clang/blob/ctu-master/tools/scan-build-py/bin/scan-build) written by the Ericsson team.

These steps are captured in the [Fuchsia wrapper](#fuchsia-wrapper-script) mentioned below. The result of all this is a directory full of reports, in [Apple plist](https://en.wikipedia.org/wiki/Property_list) format, which contain details of reported bugs.

### Ericsson’s wrapper scripts {#ericsson-wrapper-script}

There are two sets of tools for running cross-translation-unit analysis:

* The tools under `tools/xtu-build-new` are the top-level scripts. Since the underlying analyzer can fail (i.e. due to the CSA crashing), I’ve patched `xtu-analyze.py` (in Ericsson’s branch) so that it dumps the output of the analyzer (stdout/stderr, not the report) to a file. The output goes in ``$OUT_DIR/{passes,fails}`` depending on the return code of the analyzer, where `$OUT_DIR` is the directory passed to the `-o` argument of `xtu-analyze.py`. The particularly helpful part of those files is the *second line* that starts with `analyze: DEBUG: exec command in`, which is emitted by the `libscanbuild` tools (next bullet point). That command is the actual invocation to the CSA after the long, tedious process of modifying its command line. Therefore, it’s the command that you will need if you want to run the CSA on a troublesome file using gdb.
* The tools under `tools/scan-build-py` are a bird’s nest of tools to wrap around the actual invocation to Clang. They are responsible for modifying the command line. I’m not too familiar with them, and haven’t had to interfere with them in the past.

### Fuchsia wrapper script {#fuchsia-wrapper-script}

[This very small shell script](https://gist.github.com/karkhaz/c8ded50e564d73853731266fec729454) wraps the Ericsson `xtu-build-new` wrappers. To do a complete analysis of Zircon, make sure to clean first, and specify the correct path to your build of Clang. Then, in the zircon directory:

```
ninja -t clean && ninja && ./run.sh
```

In order to build only the kernel, specify a `TARGET` as an environment variable:

```
ninja -t clean && ninja clean && TARGET=./build-zircon-pc-x64/zircon.elf ./run.sh
```

The script also requires [clangify.py](https://gist.github.com/karkhaz/2ab5e8c7a8783318d44ceca715f20438) to be in the zircon directory with executable bit set. After the analysis has finished, there will be a `.result-xtu` directory, containing:

* A bunch of Apple plist files, which are the bug reports;
* A fails directory, containing the std{out,err} of analyzer invocations that returned non-zero;
* A passes directory, containing the std{out,err} of analyzer invocations that returned 0.

## Viewing analysis results

At the moment, the only way of parsing the plist reports and viewing them with a web interface is by using the [CodeChecker](https://github.com/Ericsson/codechecker) tool, which is developed at Ericsson and used for code comprehension and many other tasks. CodeChecker needs a large number of dependencies installed, and it’s best to install them with **pip** or **npm** or whatever rather than using **apt-get**. In short, after performing the analysis and dumping the plists into .result-xtu, you can invoke `CodeChecker plist` to parse the plists:

```
CodeChecker plist -d .result-xtu -n 2016-12-12T21:47_uniq_name -j 48
```

The argument to `-n` needs to be unique on each invocation of `CodeChecker plist`, as it represents a single parse run. CodeChecker complains otherwise. Then, run `CodeChecker server` to start a webserver on `localhost:8001`, which will display the reports of all previous parsing runs.

## Getting Help

The Samsung patchset was authored by [Aleksei Sidorin](mailto:a.sidorin@samsung.com) and his team. Aleksei is quite knowledgeable about `ASTImporter.cpp` and other AST merging aspects, and was very helpful. He and [Sean Callanan](mailto:scallanan@apple.com) were happy to review my AST Importer patches. Aleksei also [gave a relevant talk](https://www.youtube.com/watch?v=jbLkZ82mYE4) about summary-based interprocedural analysis at the 2016 LLVM Developers Meeting.

The Ericsson patchset was authored by [Gábor Horváth](mailto:xazax.hun@gmail.com) and his team. Gábor was very helpful with advice on how to run CTU analysis with the `xtu-build-new` tools.

I ([Kareem Khazem](mailto:karkhaz@karkhaz.com)) am also happy to help out where I can.

The LLVM irc channel can also be helpful.

## Zircon-specific analyses

Upstream Clang has been very receptive to receiving patches for Zircon-specific Clang checkers. The [MutexInInterruptContext](https://reviews.llvm.org/D27854) checker is one example (ported from an LLVM pass written by Farid Molazem Tabrizi), as are the [SpinLockChecker](https://reviews.llvm.org/D26340) and [MutexChecker](https://reviews.llvm.org/D26342). Potential reviewers for Clang checks are Devin Coughlin (from Apple), Artem Dergachev (on Aleksei Sidorin’s team at Samsung) and Anna Zaks (also at Apple).

These checkers are typically *opt-in*, meaning that you need to pass a flag to the analyzer to enable them: something like `-analyzer-checker=optin.zircon.MutexInInterruptContext`.

If those patches haven’t landed in Clang, you will need to apply them. To use them for analyzing Zircon with the [Ericsson wrapper scripts](#ericsson-wrapper-script), you should modify the [Fuchsia wrapper script](#fuchsia-wrapper-script) by adding the option `-e optin.zircon.MutexInInterruptContext` to the invocation of `xtu-analyze.py` at the end of the file. The patch for `MutexInInterruptContext` has a test suite, which can be used as an example of what the analysis is capable of.

# Progress on CTU support in Zircon

## Problems fixed in the AST importer

The upstream CSA crashes on the vast majority of Zircon files. This section describes some of the problems that Kareem Khazem encountered and their fixes.

### Unsupported AST Nodes {#zircon-patches}

The Clang Static Analyzer is unable to import a lot of Zircon code, due to not having implemented support for importing certain kinds of AST nodes. Patches to support these nodes are listed here:

AtomicType                    | Patch merged into upstream
------------------------------|--------------------------------
`CXXDependentScopeMemberExpr` | [`https://reviews.llvm.org/D26904`](https://reviews.llvm.org/D26904)
`UnresolvedLookupExpr`        | [`https://reviews.llvm.org/D27033`](https://reviews.llvm.org/D27033)
`DependentSizedArray`         | &nbsp;
`CXXUnresolvedConstructExpr`  | &nbsp;
`UsingDecl`                   | [`https://reviews.llvm.org/D27181`](https://reviews.llvm.org/D27181)
`UsingShadowDecl`             | [`https://reviews.llvm.org/D27181`](https://reviews.llvm.org/D27181)
`FunctionTemplateDecl`        | [`https://reviews.llvm.org/D26904`](https://reviews.llvm.org/D26904)

In general, when implementing support for new node types, one must implement a `VisitNode` function in `ASTImporter.cpp`, and also unit tests and functional tests; Kareem’s patches above contain examples. There are still quite a few unsupported AST nodes remaining; grep the analyzer output directory for `error: cannot import unsupported AST node`.

The Ericsson patchset contains only a subset of the `ASTImporter` code in the Samsung patchset. In some cases, the `Visit` function for an unsupported node can be taken straight from the Samsung patchset. However, the Samsung patchset does not include any tests, so it will still be necessary to write tests before the support for that node is upstreamed.

### Segfaults galore

A lot of the code in `ASTImporter.cpp` is buggy. Sometimes Aleksei has private patches for issues, like [this one](https://reviews.llvm.org/D26753), so it’s worth giving him (**a-sid**) a quick ping on IRC. My strategy for debugging is to look through the wrapper output for the *second* string starting with `analyze: DEBUG: exec command in` (followed by the actual command line of the analyzer), and running that command line through gdb. It often takes only a few hours to track down where a segfault is coming from.

## Bugs found before and after CTU

### Possible bug in VFS?

This is a double-free of `oldparent`, which is declared uninitialized on `system/ulib/fs/vfs.c:vfs_rename`. Two lines later, `vfs_walk` (same file) is called with `oldparent` as its second argument. It is possible to return from `vfs_walk` without assigning to `oldparent` by entering the for loop and hitting the `return r` statement on the first loop. If the value of `r` is greater than zero, then we go to the `else if` statement, which calls `vn_release` on `oldparent` (which is still uninitialized).

### Possible bug in thread?

This is a use-after-free. The path is:

* `kernel/kernel/thread.c:thread_detach_and_resume`
    * Call `thread_detach(t)`
        * Return `thread_join(t, NULL, 0)`
            * free `t` and return `NO_ERROR`
        * Return `NO_ERROR`
    * Check for error is 1false1
    * Call `thread_resume(t)`, which has been freed.
        * `thread_resume` then accesses `t`’s fields.

## CTU false positives

* The CSA cannot resolve the implementation of functions that are called through function pointers. This means that it cannot make any assumptions about what the return value of the function might be, nor any effects that the function might have on output parameters.
* There are several classes of function whose implementations are not accessible to the analyzer. Again, the analyzer cannot know that such functions touch their output arguments, so they will spuriously report that the following code reads from a garbage value:

  ```
  struct timeval tv;
  gettimeofday(&tv, NULL);
  printf("%d\n", tv.tv_usec);   // [SPURIOUS REPORT] Access to
                                // uninitialized variable ‘tv’
  ```

* Some kinds of functions that are vulnerable to this kind of imprecision include:
    * System calls (like `gettimeofday`)
    * Compiler builtins (like `memcpy`)
