# Evaluating and printing expressions in zxdb

Zxdb can evaluates simple C, C++, and Rust expressions. The most common place to evaluate an
expression in Zxdb is with the `print` command.  Expressions can also be used for most commands that
take a memory location as an argument such as `stack` or `mem-read`.

Evaluating an expression requires a stack frame, which in turn requires a process with a paused
thread. If the process is currently running, use the [pause](execution.md) command.

The most basic print command shows the current value of a variable in the current stack frame:

```none {:.devsite-disable-click-to-copy}
[zxdb] print i
34
```

More complex expressions are also supported:

```none {:.devsite-disable-click-to-copy}
[zxdb] print &foo->bar[baz]
(const MyStruct*) 0x59f4e1268f70
```

Expressions can be evaluated in the context of other stack frames without switching to them by
specifying the desired stack frame as a prefix (see [Interaction model](commands.md)):

```none {:.devsite-disable-click-to-copy}
[zxdb] frame 2 print argv[0]
"/bin/cowsay"
```

## Languages

Note: C++ and Rust are supported.

Expression evaluation takes the expression programming language from the current stack frame. If the
current frame's language is different, Zxdb defaults to C++.

The default language can be overridden with the `lauguage` setting, which can take the values `auto`
(use the current frame's language), `rust`, and `c++`:

```none {:.devsite-disable-click-to-copy}
[zxdb] set language rust
```

## Switches

The `print` command accepts these switches. To write an expression beginning with a hyphen, use
`--` to mark the end of switches. Following hyphens are treated as part of the expression:

```none {:.devsite-disable-click-to-copy}
[zxdb] print -- -i
```

  * *`--max-array=<number>`*: Specifies the maximum array size to print. By default this is 256.
    Specifying large values slows down expression evaluation and makes the output harder to read,
    but the default is sometimes insufficient. This also applies to strings.

  * *`--raw` or `-r`*: Bypass pretty-printers and show the raw type information.

  * *`--types` or `-t`*: Force type printing on. The type of every value printed is explicitly
    shown. Implies -v.

  * *`--verbose` or `-v`*: Don't omit type names. Show reference addresses and pointer types.

## Special variables

CPU registers can be referred to with the `$reg(register name)` syntax. For example, to display the
ARM register `v3`:

```none {:.devsite-disable-click-to-copy}
[zxdb] print $reg(v3)
0x573a420f128
```

CPU registers can also be used unescaped as long as no variable in the current scope has the same
name. Registers can also be used like any other variable in more complex expressions:

```none {:.devsite-disable-click-to-copy}
[zxdb] print rax + rbx
```

Vector registers can be treated as arrays according to the `vector-format` setting.

```none {:.devsite-disable-click-to-copy}
[zxdb] print ymm1
{3.141593, 1.0, 0, 0}

[zxdb] print ymm[0] * 2
6.28319
```

Sometimes an identifier may have a name that is not parseable in the current language. This is often
the case for compiler-generated symbols. Enclose such strings in "$(...)". Parentheses inside the
escaped contents can be literal as long as they are balanced, otherwise, escape them by preceeding
with a backslash. Include a literal backslash with two blackslashes:

  * `$(something with spaces)`
  * `{% verbatim %}$({{impl}}){% endverbatim %}`  {# note: the verbatim block is to avoid issues with the fuchsia.dev template engine #}
  * `$(some_closure(data))`
  * `$(line\)noise\\)`

## Setting variables

The `print` command can also mutate data, allowing variables to be set from expressions. For
example:

```none {:.devsite-disable-click-to-copy}
[zxdb] print done_flag = true
true
[zddb] print i = 56
56
```

## Other data display commands

  * Memory display commands are covered in the [memory](memory.md) section.
  * Register display commands are covered in the [assembly](assembly.md) section.

### The `locals` command

The `locals` command shows all local variables in the current stack frame. It accepts the same
switches as `print`:

```none {:.devsite-disable-click-to-copy}
[zxdb] locals
argc = 1
argv = (const char* const*) 0x59999ec02dc0
```

### The `display` command

When stepping through a function, it can be useful to automatically print one or more expressions
each time the program stops. The `display` command adds a given expression to this list:

```none {:.devsite-disable-click-to-copy}
[zxdb] display status
Added to display for every stop: status

[zxdb] next
🛑 main(…) • main.cc:48

    [code dump]

status = 5;
```
