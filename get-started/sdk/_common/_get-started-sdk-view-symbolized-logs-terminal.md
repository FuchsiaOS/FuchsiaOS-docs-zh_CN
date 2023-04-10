Examine the [symbolized logs][symbolize-logs] (that is, human readable stack
traces) of a crashed component.

The tasks include:

- Update the sample component to crash when it's started.
- Build and run the sample component, which generates and registers the debug
  symbols of the component.
- Verify that the crashed component's logs are in symbolized format.

Do the following:

1. Use a text editor to edit the `src/hello_world/hello_world.cc` file, for
   example:

   ```posix-terminal
   nano src/hello_world/hello_world.cc
   ```

1. Above the line `return 0;`, add the following line:

   ```
   abort();
   ```

   The `main()` method now should look like below:

   ```none {:.devsite-disable-click-to-copy}
   int main() {
     std::cout << "Hello again, World!\n";
     {{ '<strong>' }}abort();{{ '</strong>' }}
     return 0;
   }
   ```

   This update will cause the component to crash immediately after printing a
   message.

1. Save the file and exit the text editor.

1. Build and run the sample component:

   ```posix-terminal
   tools/bazel run //src/hello_world:pkg.component
   ```

   Building a component automatically generates and registers its debug symbols
   in the development environment.

1. Restart the `ffx` daemon:

   Note: Today, this workaround is required for newly registered symbols to be
   discovered in the environment. This issue is being tracked in
   [Issue 94614][ticket-94614]{:.external}.

   ```posix-terminal
   tools/ffx daemon stop
   ```

   A new instance of the `ffx `daemon starts the next time you run a `ffx`
   command.

1. Verify that the sample component's crash stack is symbolized in the kernel
   logs:

   ```posix-terminal
   tools/ffx log --kernel
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx log --kernel
   ...
   [174978.449][klog][klog][I] [[[ELF module #0x6 "libzircon.so" BuildID=5679a47f32c6fa7b 0x422808b26000]]]
   [174978.449][klog][klog][I] [[[ELF module #0x7 "libc.so" BuildID=1c3e8dded0fc94eb 0x428049099000]]]
   [174978.450][klog][klog][I]    #0    0x00004280490fd74b in abort() ../../zircon/third_party/ulib/musl/src/exit/abort.c:7 <libc.so>+0x6474b sp 0x11d191bcf70
   {{ '<strong>' }}[174978.450][klog][klog][I]    #1    0x000001d56b552047 in main() src/hello_world/hello_world.cc:9 <<VMO#32996646=blob-a4c56246>>+0x2047 sp 0x11d191bcf80{{ '</strong>' }}
   [174978.450][klog][klog][I]    #2    0x00004280490fcef2 in start_main(const start_params*) ../../zircon/third_party/ulib/musl/src/env/__libc_start_main.c:140 <libc.so>+0x63ef2 sp 0x11d191bcfa0
   [174978.450][klog][klog][I]    #3    0x00004280490fd145 in __libc_start_main(zx_handle_t, int (*)(int, char**, char**)) ../../zircon/third_party/ulib/musl/src/env/__libc_start_main.c:215 <libc.so>+0x64145 sp 0x11d191bcff0
   [174978.450][klog][klog][I]    #4    0x000001d56b552011 in _start(zx_handle_t) ../../zircon/system/ulib/c/Scrt1.cc:7 <<VMO#32996646=blob-a4c56246>>+0x2011 sp 0x42d5c7089fe0
   [174978.450][klog][klog][I]    #5    0x0000000000000000 is not covered by any module sp 0x42d5c7089ff0
   [174978.457][klog][klog][I] KERN: terminating process 'hello_world.cm' (32996655)
   ```

   The symbolized logs above show the exact filenames and line numbers
   (for example, `main() src/hello_world/hello_world.cc:9`) that might have
   caused the component to crash.

   Press `CTRL+C` to exit.

<!-- Reference links -->

[symbolize-logs]: /docs/development/sdk/ffx/symbolize-logs.md
[ticket-94614]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=94614
