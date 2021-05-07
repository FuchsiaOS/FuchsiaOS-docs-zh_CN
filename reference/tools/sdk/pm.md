# pm

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: pm [-k key] [-m manifest] [-o output dir] [-t tempdir] <command> [-help]
Package Commands:
    init     - initialize a package meta directory in the standard form
    build    - perform update and seal in order
    update   - update the merkle roots in meta/contents
    seal     - seal package metadata into a meta.far
    verify   - verify metadata
    archive  - construct a single .far representation of the package
Repository Commands:
    newrepo  - create a new local repostory
    publish  - publish a package to a local repository
    serve    - serve a local repository
    expand   - (deprecated) expand an archive
Tools:
    snapshot - capture metadata from multiple packages in a single file
    delta    - compare two snapshot files
For help with individual commands run "pm <command> --help"
  -k string
    	deprecated; do not use
  -m string
    	build manifest (or package directory) (default ".")
  -n string
    	name of the packages
  -o string
    	archive output directory (default ".")
  -t string
    	temporary directory (default "/tmp")
  -trace file
    	write runtime trace to file
  -version string
    	version of the packages (default "0")
```

