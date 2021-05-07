# Assembly language in zxdb

## Disassembly

The `disassemble` command (`di` for short) disassembles from the current location. If available, the
instructions and call destinations are annotated with source line information:

```none {:.devsite-disable-click-to-copy}
[zxdb] di
miscsvc.cc:118
 ▶ 0x20bc1c7aa60a  mov     dword ptr [rbx + 0x10c], eax
miscsvc.cc:122
   0x20bc1c7aa610  movabs  rax, -0x5555555555555556
   0x20bc1c7aa61a  mov     qword ptr [rbx + 0xe8], rax
   0x20bc1c7aa621  mov     qword ptr [rbx + 0xe8], 0x0
   0x20bc1c7aa62c  mov     rdi, qword ptr [rbx + 0xb0]
   0x20bc1c7aa633  mov     rax, qword ptr [rbx + 0xe8]
   0x20bc1c7aa63a  mov     qword ptr [rbx + 0x20], rax
   0x20bc1c7aa63e  call    0x20d    ➔ std::__2::size<>()
```

The `di` command can also accept an address or symbol as a parameter. If given a function name,
it disassembles the entire function:

```none {:.devsite-disable-click-to-copy}
[zxdb] di main
miscsvc.cc:88
   0x20bc1c7aa000  push    rbp
   0x20bc1c7aa001  mov     rbp, rsp
   0x20bc1c7aa004  push    rbx
   0x20bc1c7aa005  and     rsp, -0x20
   0x20bc1c7aa009  sub     rsp, 0x140
   0x20bc1c7aa010  mov     rbx, rsp
   0x20bc1c7aa013  mov     rax, qword ptr fs:[0x10]
   ...
```

### Switches

The `disassemble` command accepts these switches:

  * `--num=<lines>` or `-n <lines>`: The number of lines or instructions to emit. Defaults to the
    instructions in the given function (if the location is a function name), or 16 otherwise.

  * `--raw` or `-r`: Output raw bytes in addition to the decoded instructions.

## Stepping in machine instructions

Machine instructions can be stepped using the following Zxdb commands:

  * `nexti` / `ni`: Step to the next instruction, stepping over function calls.

  * `stepi` / `si`: Step the next instruction, following function calls.

For example:

```none {:.devsite-disable-click-to-copy}
[zxdb] ni
🛑 main(int, const char**) • main.cc:102
main.cc:99
 ▶ 0x23f711346233  mov   edx, 0x20
   0x23f711346238  call  0x35a3a3  ➔ __asan_memcpy
   0x23f71134623d  mov   rdi, qword ptr [rbx + 0x258]
   0x23f711346244  call  0x1677    ➔ $anon::DecodeCommandLine

[zxdb] ni
🛑 main(int, const char**) • main.cc:102
main.cc:99
 ▶ 0x23f711346238  call  0x35a3a3 ➔ __asan_memcpy
   0x23f71134623d  mov   rdi, qword ptr [rbx + 0x258]
   0x23f711346244  call  0x1677   ➔ $anon::DecodeCommandLine
   0x23f711346249  mov   rdi, qword ptr [rbx + 0x260]
```

Zxdb maintains information about whether the last command was an assembly command or a source-code
and shows that information on stepping or breakpoint hits. To switch to assembly-language mode,
type `disassemble`, and to switch back to source-code mode, type `list`.

## Registers

The `regs` command shows the most common CPU registers.

```none {:.devsite-disable-click-to-copy}
[zxdb] regs
General Purpose Registers
      rax  0xfffffffffffffffa = -6
      rbx          0x50b7085b
      rcx                 0x0 = 0
      rdx      0x2023de8c87a0
      rsi  0x7fffffffffffffff
      rdi          0x50b7085b
      rbp      0x224bb1e0b950
      rsp      0x224bb1e0b928
      ...
```

There are other categories and options for CPU registers that can be shown by switches to the `regs`
command:

  * `--all` or `-a`: Enable all register categories (does not imply `-e`).

  * `--float` or `-f`: Prints the dedicated floating-point registers. In most cases you should use
    `--vector` instead because all 64-bit ARM code and most x64 code uses vector registers for
    floating point.

  * `--vector` or `-v`: Prints the vector registers. See below for more details.
  * `--debug` or `-d`: Prints the debug registers.

  * `--extended` or `-e`: Enables more verbose flag decoding. This enables more information
    that is not normally useful for everyday debugging. This includes information such as the
    system level flags within the `rflags` register for x64.

### Registers in expressions

Registers can be used in [expressions](printing.md) like variables. The canonical name of a register
is `$reg(register name)`.

```none {:.devsite-disable-click-to-copy}
[zxdb] print $reg(x3)
79
```

In addition, the raw register name can be used if there is no variable with the same name:

```none {:.devsite-disable-click-to-copy}
[zxdb] print x3
79
```

Registers can be assigned using the normal expression evaluation syntax:

```none {:.devsite-disable-click-to-copy}
[zxdb] print x3 = 0
0
```

### Vector registers

The `regs --vector` command displays vector registers in a table according to the current
`vector-format` setting. Use `get vector-format` to see the current value and documentation, and
`set vector-format <new-value>` to set a new vector format. Possible values are:

  * `i8` (signed) or `u8` (unsigned): Array of 8-bit integers.
  * `i16` (signed) or `u16` (unsigned): Array of 16-bit integers.
  * `i32` (signed) or `u32` (unsigned): Array of 32-bit integers.
  * `i64` (signed) or `u64` (unsigned): Array of 64-bit integers.
  * `i128` (signed) or `u128` (unsigned): Array of 128-bit integers.
  * `float`: Array of single-precision floating point.
  * `double`: Array of double-precision floating point. This is the default.

```none {:.devsite-disable-click-to-copy}
[zxdb] set vector-format double

[zxdb] regs -v
Vector Registers
  mxcsr 0x1fa0 = 8096

   Name [3] [2] [1]       [0]
   ymm0   0   0   0         0
   ymm1   0   0   0   3.14159
   ymm2   0   0   0         0
   ymm3   0   0   0         0
   ...
```

Vector registers can also be used like arrays in expressions. The `vector-format` setting controls
how each register is converted into an array value. For example, to show the low 32 bits interpreted
as a floating-point value of the x86 vector register `ymm1`:

```none {:.devsite-disable-click-to-copy}
[zxdb] set vector-format float

[zxdb] print ymm1[0]
3.14159
```

When converting to an array, the low bits are assigned to be index 0, increasing from there.
Note that the vector register table in `regs` are displayed with the low values on the right side.
