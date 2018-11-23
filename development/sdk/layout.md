# SDK layout

This document describes the standard layout of a Fuchsia SDK archive.

```
$root/
    meta/                          # metadata about the contents of this archive
        manifest.json              # describes the elements in the archive
        schemas/                   # JSON schemas for metadata files
    tools/                         # host tools
        do_something
        do_something-meta.json     # metadata about a particular tool
    pkg/                           # arch-independent package contents
        foo/
            meta.json              # metadata about this element
            include/               # headers
            docs/                  # documentation
        bar/
            meta.json
            include/
            src/                   # sources for a C++ library
            docs/
    dart/                          # Dart packages
        foo/
            meta.json
            lib/
    fidl/                          # FIDL libraries
        fuchsia.some.service/
            meta.json
            some_service.fidl
    arch                           # target-independent prebuilts
        x64/
            sysroot/
                include/
                lib/
                dist/
                debug/
            lib/
                libfoo.so          # ABI only, to link against
            dist/
                libfoo.so          # to include in Fuchsia packages
            debug/
                libfoo.so          # unstripped versions
        arm64/
            sysroot/
                include/
                lib/
                dist/
                debug/
            lib/
            dist/
            debug/
    target/                        # target-dependent prebuilts
        x64/
            fuchsia.zbi
        arm64/
            fuchsia.zbi
```
