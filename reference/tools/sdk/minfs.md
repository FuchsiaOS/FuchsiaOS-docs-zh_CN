# minfs

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

usage: minfs [ <option>* ] <file-or-device>[@<size>] <command> [ <arg>* ]
options: -d|--depfile          	Produce a depfile
         -r|--readonly         	Mount filesystem read-only
         -o|--offset   [bytes] 	Byte offset at which minfs partition starts
                                 (Default = 0)
         -l|--length   [bytes] 	Length in bytes of minfs partition
                                 (Default = Remaining Length)
         -h|--help             	Display this message
commands: create     Initialize filesystem.
          mkfs       Initialize filesystem.
          check      Check filesystem integrity.
          fsck       Check filesystem integrity.
          used-data-size Prints total bytes consumed by data.
          used-inodes Prints number of allocated inodes.
          used-size  Prints total bytes used by data and reserved for fs internal data structures.
          add        Add files to an fs image (additional arguments required).
          cp         Copy to/from fs.
          mkdir      Create directory.
          ls         List contents of directory.
          manifest   Add files to fs as specified in manifest (deprecated).
arguments (valid for create, one or more required for add):
	--manifest <path>
The format of the manifest must be as follows:
	'dst/path=src/path'
with one dst/src pair on each line.
Prefix all minfs paths with '::' (unless they are included in a manifest).
```

