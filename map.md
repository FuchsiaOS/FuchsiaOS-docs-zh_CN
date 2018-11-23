Repository structure
====================

```
//docs
  README.md               # welcome message, table of contents
  CODE_OF_CONDUCT.md      # rules and expectations for contributors
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
    hardware/             # how to use Fuchsia on various devices
    languages/            # conventions, tooling for supported languages
      style.md            # style guide
      naming.md           # how to name stuff that’s not covered in the style
                          # guide
    sources.md            # explain the version control system: Jiri,
                          # fuchsia.googlesource.com, Git, Gerrit
    layers.md             # purpose and nature of the layers, auto-rolling
                          # system, embedded manifests
    third_party.md        # structure of third-party code, policies, maintenance
    build_system.md       # overview of the build system: GN/Ninja, Zircon
                          # specifics, what the main steps of the build are, how
                          # GN targets are structured, build package
  the-book/               # an academic description of the Fuchsia stack, with
                          # links to implementation
```
