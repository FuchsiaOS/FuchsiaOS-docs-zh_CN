# zbi

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

Usage: zbi [OUTPUT...] INPUT... [-- PATTERN...]
Diagnostic switches:
    --help, -h                     print this message
    --list, -t                     list input ZBI item headers; no --output
    --verbose, -v                  show contents (e.g. BOOTFS file names)
    --extract, -x                  extract BOOTFS files
    --extract-items, -X            extract items as pseudo-files (see below)
    --extract-raw, -R              extract original payloads, not ZBI format
Output file switches:
    --output=FILE, -o FILE         output file name
    --depfile=FILE, -d FILE        makefile dependency output file name
    --output-dir=DIR, -D FILE      extracted files go under DIR (default: .)
    --json-output=FILE, -j FILE    record entries to a JSON file
The `--output` FILE is always removed and created fresh after all input
files have been opened.  So it is safe to use the same file name as an input
file and the `--output` FILE, to append more items.
Input control switches apply to subsequent input arguments:
    --directory=DIR, -C DIR        change directory to DIR
    --files, -F                    read BOOTFS manifest files (default)
    --prefix=PREFIX, -p PREFIX     prepend PREFIX/ to target file names
    --replace, -r                  duplicate target file name OK (see below)
    --type=TYPE, -T TYPE           input files are TYPE items (see below)
    --compressed[=HOW], -c [HOW]   compress storage images (see below)
    --uncompressed, -u             do not compress storage images
    --recompress                   recompress input items already compressed
    --ignore-missing-files, -i     a manifest entry whose source file doesn't
                                   exist is ignored without error
Input arguments:
    --entry=TEXT, -e TEXT          like an input file containing only TEXT
    FILE                           input or manifest file
    DIRECTORY                      directory tree copied to BOOTFS PREFIX/
The `--directory` or `-C` switch affects subsequent input arguments but
it never affects output arguments, which are always relative to the original
current working directory (`zbi` doesn't actually do `chdir()` at all).
With `--files` or `-F` (the default state), files with ZBI_TYPE_CONTAINER
headers are incomplete boot files and other files are BOOTFS manifest files.
Each DIRECTORY is listed recursively and handled just like a manifest file
using the path relative to DIRECTORY as the target name (before any PREFIX).
Each `--prefix` or `-p` switch affects each file from a manifest or
directory in subsequent FILE, DIRECTORY, or TEXT arguments.
With `--type` or `-T`, input files are treated as TYPE instead of manifest
files, and directories are not permitted.  See below for the TYPE strings.
ZBI items from input ZBI files are normally emitted unchanged.  (However,
see below about BOOTFS items.)  With `--recompress`, input items of storage
types well be decompressed (if needed) on input and then freshly compressed
(or not) according to the preceding `--compressed=...` or `--uncompressed`.
Format control switches (last switch affects all output):
    --complete=ARCH, -B ARCH       verify result is a complete boot image
    --compressed[=HOW], -c [HOW]   compress BOOTFS images (see below)
    --uncompressed, -u             do not compress BOOTFS images
HOW defaults to `zstd` and can be one of (case-insensitive):
 * `none` (same as `--uncompressed`)
 * `LEVEL` (an integer) or `max` (default algorithm, currently `zstd`)
 * `lz4f` or `lz4f.LEVEL` (an integer) or `lz4f.max`
 * `zstd` or `zstd.LEVEL` (an integer) or `zstd.max` or `zstd.overclock`
The meaning of LEVEL depends on the algorithm.  The default is chosen for
good compression ratios with fast compression time.  `max` is for the best
compression ratios but much slower compression time (e.g. release builds).
If there are no PATTERN arguments and no files named to add to the BOOTFS
(via manifest file entries, nonempty directories, or `--entry` switches)
then any ZBI input items of BOOTFS type are passed through as they are,
except for possibly compressing raw `--type=bootfs` input items.
In all other cases there is only a single BOOTFS item (if any) written out.
So `-- \*` will force merging when no individual files are being added.
The BOOTFS image contains all files from BOOTFS items in ZBI input files,
manifest files, directories, and `--entry` switches.  The BOOTFS directory
table is always sorted.  By default it's an error to have duplicate target
file names in the input (even with the same source).  `--replace` or `-r`
allows it: the last entry in input order wins.
**TODO(mcgrathr):** not quite true yet
Each argument after -- is a shell filename PATTERN (`*` matches even `/`)
to filter the files that will be packed into BOOTFS, extracted, or listed.
For a PATTERN that starts with `!` or `^` matching names are excluded after
including matches for all positive PATTERN arguments.  Note that PATTERN
is compared to the final BOOTFS target file name with any PREFIX applied.
When extracting a single file, `--output` or `-o` can be used.
Otherwise multiple files are created with their BOOTFS file names
relative to PREFIX (default empty, so in the current directory).
Note that the last PREFIX on the command line affects extraction,
though each PREFIX also (first) affects BOOTFS files added due to arguments
that follow it.  So if any PREFIX appears before such input arguments when
extracting, the extracted file names will have a doubled PREFIX unless a
`--prefix=.` or other PREFIX value follows the input arguments.
With `--extract-items` or `-X`, instead of BOOTFS files the names are
synthesized as shown below, numbered in the order items appear in the input
starting with 001.  Output files are ZBI files that can be input later.
With `--extract-raw` or `-R`, each file is written with just the
uncompressed payload of the item and no ZBI headers.
TYPE can be hexadecimal or a name string (case-insensitive).
Extracted items use the file names shown below:
    --type               --extract-item             --extract-raw
    CONTAINER            001.zbi                    001.bin
    KERNEL_X64           001.zbi                    001.bin
    KERNEL_ARM64         001.zbi                    001.bin
    DISCARD              001.zbi                    001.bin
    RAMDISK              001.zbi                    001.bin
    BOOTFS               001.zbi                    001.bin
    BOOTFS_FACTORY       001.zbi                    001.bin
    CMDLINE              001.zbi                    001.txt
    CRASHLOG             001.zbi                    001.bin
    NVRAM                001.zbi                    001.bin
    PLATFORM_ID          001.zbi                    001.bin
    CPU_CONFIG           001.zbi                    001.bin
    CPU_TOPOLOGY         001.zbi                    001.bin
    MEM_CONFIG           001.zbi                    001.bin
    KERNEL_DRIVER        001.zbi                    001.bin
    ACPI_RSDP            001.zbi                    001.bin
    SMBIOS               001.zbi                    001.bin
    EFI_MEMORY_MAP       001.zbi                    001.bin
    EFI_SYSTEM_TABLE     001.zbi                    001.bin
    E820_TABLE           001.zbi                    001.bin
    FRAMEBUFFER          001.zbi                    001.bin
    DRV_MAC_ADDRESS      001.zbi                    001.bin
    DRV_PARTITION_MAP    001.zbi                    001.bin
    DRV_BOARD_PRIVATE    001.zbi                    001.bin
    DRV_BOARD_INFO       001.zbi                    001.bin
    IMAGE_ARGS           001.zbi                    001.txt
    BOOT_VERSION         001.zbi                    001.bin
    HW_REBOOT_REASON     001.zbi                    001.bin
    SERIAL_NUMBER        001.zbi                    001.txt
    BOOTLOADER_FILE      001.zbi                    001.bin
    DEVICETREE           001.zbi                    001.dtb
```

