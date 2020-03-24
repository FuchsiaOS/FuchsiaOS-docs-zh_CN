Repository structure
====================

```
//docs
  README.md               # 欢迎消息，内容索引
  CODE_OF_CONDUCT.md      # 贡献者守则
  CONTRIBUTING.md         # lay out the ground rules for contributing, redirect
                          # development folder
  getting_in_touch.md     # how and why to get in touch with the Fuchsia team
  values/                 # various bits about project culture
  development/            # how to develop in the Fuchsia tree?
    README.md             # warn that it’s mainly about development of Fuchsia,
                          # not just targeting Fuchsia
    workflows/            # concrete usage patterns for: Jiri & Git, Gerrit, fx,
                          # GN/ninja, etc...
    best-practices/       # general articles about coding practices
    hardware/             # 怎么在各种设备上使用 Fuchsia
    languages/            # conventions, tooling for supported languages
      style.md            # 风格指南
      naming.md           # 命名规则
    sources.md            # explain the version control system: Jiri,
                          # fuchsia.googlesource.com, Git, Gerrit
    layers.md             # purpose and nature of the layers, auto-rolling
                          # system, embedded manifests
    third_party.md        # 第三方代码结构、政策和维护
    build_system.md       # 编译系统概述: GN/Ninja, Zircon
                          # specifics, what the main steps of the build are, how
                          # GN targets are structured, build package
  the-book/               # an academic description of the Fuchsia stack, with
                          # links to implementation
```
