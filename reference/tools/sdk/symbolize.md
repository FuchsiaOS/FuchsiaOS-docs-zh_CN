# symbolize

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage of symbolize:
  -build-id-dir value
    	path to .build-id directory
  -color value
    	use color in output, can be never, auto, always (default auto)
  -ids value
    	(deprecated) alias for -ids-txt
  -ids-rel
    	tells the symbolizer to always use ids.txt relative paths
  -ids-txt value
    	path to ids.txt
  -json-output string
    	outputs trigger information to the specified file
  -level value
    	output verbosity, can be fatal, error, warning, info, debug or trace (default info)
  -llvm-symbolizer string
    	path to llvm-symbolizer (default "llvm-symbolizer")
  -llvm-symbolizer-restart-interval uint
    	How many queries to make to the llvm-symbolizer tool before restarting it. 0 means never restart it. Use to control memory usage. See fxbug.dev/42018. (default 15)
  -symbol-cache string
    	path to directory to store cached debug binaries in
  -symbol-index string
    	path to the symbol-index file (default "/usr/local/google/home/kasiahayden/.fuchsia/debug/symbol-index")
  -symbol-server value
    	a GCS URL or bucket name that contains debug binaries indexed by build ID
  -symbol-server-timeout duration
    	Symbol server timeout for fetching an object from gs (default 5s)
```

